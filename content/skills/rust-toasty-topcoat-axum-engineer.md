---
name: rust-toasty-topcoat-axum-engineer
description: Engineer Rust web apps built with Axum (JSON API), Topcoat (server-rendered UI) and Toasty (SQL persistence) in a layered architecture. Use when adding or changing endpoints, domain logic, repository queries, Toasty models, Topcoat views or components, binaries, or tests.
---

# rust-toasty-topcoat-axum-engineer

Practices for a single Rust crate that serves a JSON API (Axum) and server-rendered HTML (Topcoat) from one binary, with persistence through Toasty. Examples use a generic `Item` entity; substitute the real domain names.

## 1. Architecture

Three layers. Dependencies point downward only.

```
 api (Axum JSON)      web (Topcoat HTML)
        \                  /
         v                v
        domain::<module>::service      <- validation + business rules
                  |
                  v
        domain::<module>::repository   <- Toasty rows, queries, mapping
                  |
                  v
                 SQL DB
```

| Path                                                | Role                                                                                                                  |
| --------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------- |
| `src/lib.rs`                                        | Exposes `api`, `domain`, `web`.                                                                                       |
| `src/domain/error.rs`                               | Single `Error` enum (`NotFound`, `Conflict`, `Validation`, `Internal`) and `Result<T>`, shared by all domain modules. |
| `src/domain/<module>/` (e.g. `users/`)              | One module per domain concept, each with its own `model.rs`, `repository.rs` and `service.rs` (see section 3).        |
| `src/domain/<module>/model.rs`                      | Pure domain types and validated newtypes. No Toasty, no HTTP.                                                         |
| `src/domain/<module>/repository.rs`                 | Repository: private `*Row` Toasty models and all DB access for that module. Rows never leave this file.               |
| `src/domain/<module>/service.rs`                    | Service: validates and normalizes input, derives data, delegates to the module's repository.                          |
| `src/api.rs`, `src/api/v0/...`                      | Axum router, shared state, error-to-HTTP mapping.                                                                     |
| `src/web.rs`, `src/web/views`, `src/web/components` | Topcoat router, pages, Atomic Design components.                                                                      |
| `src/bin/<name>/main.rs`                            | Executables: CLI parsing, tracing, wiring.                                                                            |
| `tests/`                                            | End-to-end API tests.                                                                                                 |
| `build.rs`                                          | Generates Tailwind CSS via Topcoat into `OUT_DIR`.                                                                    |

## 2. How everything connects

1. The server binary loads `.env` (`dotenvy`), parses a Clap `Cli` (precedence: flag > env var > `.env` > default), and initializes `tracing_subscriber`.
2. A connect function builds one `toasty::Db` with `toasty::models!(...)` listing every module's rows and calls `push_schema()`, tolerating "already exists". Schema is created on startup.
3. For each domain module, `XService::new(XRepository::new(db.clone()))` is built, and the services are bundled in the shared state (wrapped in `Arc`) handed to the API router.
4. The API `router(state)` builds the Axum `Router`: health route, versioned nest (`/api/v0`), a `TraceLayer`, `.with_state(state)`, and a **`fallback_service`** that mounts the Topcoat router through `topcoat::router::tower::TowerService`. Any path the API does not match falls through to the HTML UI.
5. `web::router(service)` uses `module_router!().discover().with(service).build()`. Routes, pages and layouts self-register through attribute macros; the service is provided as Topcoat context.
6. Handlers get the service: API via `State(service): State<AppState>`, web via `context::<Arc<Service>>(cx)`.
7. `build.rs` renders Tailwind CSS; `web.rs` embeds it with `include_str!(concat!(env!("OUT_DIR"), "/tailwind.css"))` and serves it from a stylesheet route. Always pair light classes with `dark:` variants.

## 3. Domain layer

- **Organize by module, one trio per module.** `domain` holds one module per concept (e.g. `users`, `orders`). Each module contains exactly its own `model`, `repository` and `service`:

```
src/domain.rs            // pub mod error; pub mod users; pub mod orders;
src/domain/error.rs
src/domain/users.rs      // pub mod model; pub mod repository; pub mod service;
src/domain/users/model.rs
src/domain/users/repository.rs
src/domain/users/service.rs
src/domain/orders.rs
src/domain/orders/model.rs
src/domain/orders/repository.rs
src/domain/orders/service.rs
```

- `users::model`: `User` and its newtypes. `users::repository`: `UserRepository` with the private `UserRow`. `users::service`: `UserService`, which owns a `UserRepository`.
- A module never reaches into another module's repository or rows. Cross-module needs go service-to-service (inject the other `Service`) or via the other module's public model types.
- Shared concerns (error type, cross-cutting newtypes) live at the `domain` root, not inside a feature module.
- Handlers and views receive services through shared state/context, e.g. an app state struct bundling `UserService`, `OrderService`, ...; each `Repository::new(db)` shares the same cloned `toasty::Db`.
- Register every row type of every module in the single `toasty::models!(...)` call used when connecting.
- **Models are plain structs** (`Clone, Debug, PartialEq`). Typical types: `ulid::Ulid` ids, `rust_decimal::Decimal` for money, `chrono::DateTime<Utc>` for times, `url::Url` for URLs.
- **Validated newtypes** (e.g. a `Slug`) implement `FromStr`, check their invariant once, and expose `as_str()`. Parse into them; do not pass raw strings where a newtype is expected.
- **Errors**: always return `domain::error::Result<T>`. `Validation` for bad input, `NotFound` for missing rows, `Conflict` for uniqueness violations, `Internal` for everything unexpected. `From<toasty::Error>` maps record-not-found and unique-constraint failures; add `From` impls for other source errors and use `?`.
- **Service rules**: trim strings, reject empty values, parse into typed values, enforce business rules. Services never touch Toasty types. Derived or aggregate logic lives in `service/` submodules.
- No validation in handlers or the repository. The repository trusts typed input but maps corrupt stored data to `Error::Internal`.

## 4. Persistence layer (Toasty, SQLite)

- Each table has a private row struct:

```rust
#[derive(Debug, toasty::Model)]
#[table = "items"]
struct ItemRow {
    #[key]
    id: String,
    #[unique]
    name: String,
    #[index]
    owner_id: String,
}
```

Relations use `#[belongs_to(key = ..., references = ...)]` with `toasty::Deferred<T>`; composite keys use several `#[key]` fields.

- **Storage conventions**: ids stored as `String` (`Ulid::to_string()`), decimals as text (no float rounding), timestamps as `i64` unix seconds. Convert at the boundary with explicit `map_err(|err| Error::Internal(...))`, naming the bad value in the message.
- **Every new row type must be added** to `toasty::models!(...)` in the connect function, otherwise its table is never created.
- Idioms:
  - `let mut db = self.db.clone();` at the top of each method (`Db` is cheap to clone).
  - Insert: `toasty::create!(ItemRow { id: ..., name: ... }).exec(&mut db).await?`
  - Fetch: `ItemRow::get_by_id(&mut db, id).await?`, `ItemRow::filter_by_name(name).exec(&mut db).await?`, `ItemRow::all().exec(&mut db).await?`.
- Self-referencing hierarchies: a root points at itself; walk chains with a max-depth guard against cycles.
- Unique human-readable keys: derive a candidate, check for collisions, append `-2`, `-3`, ... until free.
- Multi-table reads: load the tables needed and join in memory with `HashMap`s unless asked to optimize. Sort deterministically (by id, or by `(timestamp, id)`).
- Provide an in-memory constructor (`sqlite::memory:`, pool size 1) for tests.

## 5. API layer (Axum 0.8)

- File layout mirrors URL structure: `api/v0/items.rs` (router + shared `*Resource`), `items/create.rs`, `items/id.rs` (router for `/{id}`), `items/id/update.rs`, `items/id/delete.rs`. One file per operation; directory nesting equals path nesting.
- Each module exposes `pub fn router() -> Router<AppState>`; parents `.nest(...)` children. Path params use Axum 0.8 syntax `/{id}` and extract `Path<Ulid>`.
- Handler shape:

```rust
/// `POST /api/v0/items`
pub async fn create(
    State(service): State<AppState>,
    Json(req): Json<CreateItemReq>,
) -> ApiResult<(StatusCode, Json<CreateItemRes>)> {
    let item = service.create_item(&req.name).await?;
    Ok((StatusCode::CREATED, Json(item.into())))
}
```

- Naming: `XxxReq` (`Deserialize`), `XxxRes` / `XxxResource` (`Serialize`), converted with `impl From<DomainModel> for XxxRes`. Never serialize domain models directly.
- Doc-comment each handler with its verb and path.
- Return `ApiResult<T>`. `ApiError` wraps the domain `Error` and renders `{"error": "..."}`: NotFound 404, Conflict 409, Validation 422, Internal 500 (logged with `tracing::error!`, generic message to the client). Change status mappings in the error module, not in handlers.
- New endpoints go under the current version prefix; wire new modules in the version router.

## 6. Web layer (Topcoat)

- **Views** (`web/views/*`): one module per page, annotated `#[page("/route")]`, signature `async fn name(cx: &Cx) -> Result<impl View>`. Get the service with `context::<Arc<Service>>(cx)`. Path params: `path_param!(slug: String);` then `slug(cx).ok_or_not_found()?`. Register new view modules in `views.rs`.
- **Layout**: a `#[layout("/")]` template with a `Slot` wraps every page in the HTML document.
- **Components** follow Atomic Design; each level only uses levels below it:
  - `atoms`: smallest elements (badge, image, header)
  - `molecules`: small groups of atoms
  - `organisms`: distinct page sections (card, grid)
  - `templates`: page-level layouts

  Register new components in the level's `mod` file.

- Component shape: `#[component] pub async fn name(cx: &Cx, props...) -> Result<impl View>`; `#[default]` for optional props, `Child<'_>` for children. Call with `name(prop: value)`. Model variants as enums matched inside `view!`.
- `view!` syntax: HTML tags, `(expr)` interpolation, quoted string literals (`"text"`), inline `if` / `if let` / `match`.
- Pure formatting helpers go in `web/utils/`, not in components.
- Styling: Tailwind utility classes, always with `dark:` variants.
- Errors in views: log with `tracing::error!(%err, ...)` and convert with `topcoat::Error::msg(err)`; `ok_or_not_found()` for missing params. Do not `.unwrap()` service calls.

## 7. Testing

- Use the project's task runner (Justfile) for test, format/lint, run and seed commands; tests run with `cargo nextest run --workspace`, formatting with `cargo clippy --fix` then `cargo fmt`.
- **Repository unit tests** live in `#[cfg(test)] mod tests` at the bottom of each module's `repository.rs`; use `#[tokio::test]` and the in-memory database. Assert on `Error` variants with `matches!`.
- **API tests** in `tests/` drive the Axum `Router` with `tower::ServiceExt::oneshot` against an in-memory DB: no sockets, no binary. Keep a `TestApp` helper (`request`, `get`, creation helpers) and extend it instead of duplicating request code. Each test builds a fresh app.
- Cover conflict (409), not-found (404) and validation (422) paths, not only the happy path.
- Unit-test newtype parsing and normalization next to the type.

## 8. Best practices

1. **Respect the layers.** Handlers and views call services only. A service calls its own module's repository (and other modules' services, never their repositories). Row structs, Toasty calls and DB error strings stay inside the module's `repository.rs`.
2. **Validate in the service, once.** Trim, check emptiness, parse into typed values, then hand typed data down.
3. **Make invalid states unrepresentable**: newtypes, enums for variants, `Option` for nullable columns.
4. **One error type.** Extend `domain::error::Error` and its mappers instead of inventing per-layer errors. Never leak internal messages to API clients.
5. **Stable ordering** on every list result so tests and pages are deterministic.
6. **Small files, one operation or component each**, with a doc comment stating route or purpose. Keep module docs (`//!`) current.
7. **Adding a table**: row struct -> add to `toasty::models!` -> repository methods -> model mapping -> service method -> API/web usage -> tests.
8. **Adding an endpoint**: `Req`/`Res` types + `From` -> handler file -> register in parent `router()` -> API test.
9. **Adding a page**: view module with `#[page]` -> register in `views.rs` -> compose from existing components; create new ones at the right atomic level.
10. **Decimals for money, never floats.** Timestamps in UTC.
11. **Log with `tracing`** (structured fields, `%err`); log internal failures once, at the boundary.
12. **Config** goes through the `Cli` struct (flag, env var, default); document new variables in `.env.example`.
13. Use Conventional Commits (`feat:`, `fix:`, `chore:`), one concern per commit.
14. Before finishing: format and lint, run the tests, and start the server if the change touches UI or routing.
