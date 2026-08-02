---
name: bevy-ecs-expert
description: "Write idiomatic, high-performance Bevy 0.19 ECS code in Rust: components, required components, BSN scenes, queries, systems, resources-as-components, observers and messages, relationships, scheduling, and parallelism. Use this skill whenever the user mentions Bevy, `bevy_ecs`, ECS architecture in Rust, or is writing game logic with entities/components/systems — and especially when porting code from Bevy 0.14–0.18, since nearly every ECS API was renamed across those releases."
---

# Bevy ECS Expert

## Overview

Guidance for building game logic on Bevy's data-oriented ECS. Covers how to model data as components, express behavior as systems, query efficiently, and let the scheduler extract parallelism for you.

## Version scope

**This skill targets Bevy 0.19** (released 18 June 2026). Bevy is pre-1.0 and breaks its ECS API nearly every release, so version discipline matters more here than in most Rust crates. Two habits worth keeping:

- **Pin the version.** `bevy = "0.19"` in `Cargo.toml`. Never mix advice across releases.
- **Assume older code is stale.** Most Bevy code in the wild (and most model training data) targets 0.14–0.16. Before copying a snippet, check it against the [renames cheat sheet](#renames-cheat-sheet) at the end of this file.

If the user is on a different version, say so plainly and point them at the [migration guides](https://bevy.org/learn/migration-guides/) rather than guessing which APIs existed when.

Three things changed in 0.19 that reshape how idiomatic code looks:

1. **BSN** — a new scene system (`bsn!` macro) that largely replaces bundle-spawning functions.
2. **Resources are components** — stored on singleton entities, unifying the two data models.
3. **Contiguous query iteration** — table slices exposed for SIMD/autovectorization.

## When to Use This Skill

- Building or reviewing game logic with the Bevy engine.
- Designing systems intended to run in parallel.
- Optimizing frame time: query shape, change detection, cache behavior.
- Refactoring object-oriented or inheritance-shaped code into ECS.
- Migrating a project from an older Bevy release.

---

## 1. Components

Components are plain data. Derive `Component`; add `Reflect` when the type needs to participate in reflection (inspectors, serialization, the Bevy Remote Protocol).

```rust
use bevy::prelude::*;

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Player;
```

Register reflected types once, in a plugin: `app.register_type::<Velocity>();`

Derive `Clone + Default` on gameplay components even when it isn't strictly needed — BSN gives those types a pass-through `Template` for free, so they can be used in `bsn!` without extra work.

### Required components

`#[require(...)]` declares dependencies that are inserted automatically (recursively, depth-first) unless the spawner supplies its own value. This replaced bundles as the primary mechanism for "these components belong together."

```rust
#[derive(Component, Default, Clone)]
#[require(
    Velocity,                    // Default::default()
    Health = Health(100.0),      // arbitrary expression
    Transform,
)]
struct Enemy;
```

The `Foo = expr` form is current. The older `Foo(constructor_fn)` call syntax was replaced — `B(1)` is now read as a tuple-struct literal, not a function call.

### Component hooks and immutability

```rust
#[derive(Component)]
#[component(immutable)]                    // no ResMut/&mut access; forces insert-to-change
struct EntityId(u64);

#[derive(Component)]
#[component(on_add = index_mine, on_remove = unindex_mine)]
struct Mine { pos: Vec2 }
```

Hook names are `on_add`, `on_insert`, `on_discard`, `on_remove`. (`on_replace` was renamed to `on_discard` in 0.19, matching the `Replace` → `Discard` lifecycle event rename.)

Prefer observers to hooks for game logic; hooks are for maintaining invariants the type itself owns, such as keeping an index in sync.

---

## 2. Spawning: prefer BSN

Bevy 0.19 introduces BSN (Bevy Scene Notation). `bsn!` produces an `impl Scene`, which is composable, patchable, and resolves its own asset dependencies — so scene functions no longer need every `AssetServer`/`Assets<T>` threaded through them.

```rust
fn player() -> impl Scene {
    bsn! {
        Player
        Health(100.0)
        Sprite { image: "player.png" }   // asset path resolved by the template
        Children [
            Sword,
            Shield,
        ]
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_scene(player());
}
```

Only the fields you name are written; everything else takes its `Default`. Layering one scene over another patches it, which is what makes widget-style composition work.

**`SceneComponent` solves the "is the whole scene there?" problem.** Deriving it ties a component to a scene and guarantees that if the component is present, its scene was spawned with it:

```rust
#[derive(SceneComponent, Default, Clone)]
struct Player { score: usize }

impl Player {
    fn scene() -> impl Scene {
        bsn! {
            Children [ LeftHand, RightHand ]
        }
    }
}

// Spawn via `@`. Spawning it directly with `world.spawn(Player::default())` logs an error.
world.spawn_scene(bsn! { @Player { score: 10 } });
```

Tuple spawning still works and is fine for simple, dependency-free entities:

```rust
commands.spawn((Player, Velocity(Vec2::new(10.0, 0.0)), Transform::default()));
```

Caveats worth stating to users: there is no first-party `.bsn` asset loader yet in 0.19 (code-driven only), and glTF still loads through the old path — `commands.spawn(WorldAssetRoot(asset_server.load("scene.gltf#Scene0")))`.

---

## 3. Systems

Systems are ordinary functions whose parameters implement `SystemParam`. The scheduler infers data access from the signature and runs non-conflicting systems in parallel automatically.

```rust
fn movement(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.0.extend(0.0) * time.delta_secs();
    }
}
```

Note `delta_secs()` — `delta_seconds()` has been gone since 0.16.

### Fallible systems

Systems may return `Result`, letting you use `?` instead of `unwrap()`:

```rust
fn aim_camera(
    mut camera: Single<&mut Transform, With<Camera>>,
    player: Single<&Transform, (With<Player>, Without<Camera>)>,
) -> Result {
    camera.look_at(player.translation, Vec3::Y);
    Ok(())
}
```

Errors reach the app's error handler, which panics by default; override with `FallbackErrorHandler` (renamed from `DefaultErrorHandler` in 0.19) or per-system via `.pipe(...)`.

Fallible system params control whether the system runs at all: `Single<D, F>` requires exactly one match and silently skips otherwise, `Populated<D, F>` requires at least one, `Option<Single<D, F>>` allows zero or one. Reach for these instead of `query.single()?` when "no match" is a normal frame state — it keeps the skip cheap and intentional.

---

## 4. Queries

Filters shrink the archetypes touched, which is where most easy performance lives.

```rust
fn enemy_ai(query: Query<&Transform, (With<Enemy>, Without<Dead>)>) {
    for transform in &query {
        // only living enemies
    }
}
```

- `With<T>` / `Without<T>` — archetypal, effectively free.
- `Changed<T>` / `Added<T>` — per-entity checks; excellent for avoiding recomputation, but they disable contiguous iteration.
- `Or<(With<A>, With<B>)>` — combine filters.
- `Ref<T>` — read the value plus its change ticks. In 0.19 `Ref` is `Copy`, so `r.clone()` clones the `Ref`, not the inner `T`; use `r.into_inner().clone()` if you wanted the value.

Parallel iteration for heavy per-entity work:

```rust
fn simulate(mut query: Query<(&mut Transform, &Velocity)>) {
    query.par_iter_mut().for_each(|(mut transform, velocity)| {
        transform.translation += velocity.0.extend(0.0);
    });
}
```

Two systems can't hold conflicting access to the same component; if one system needs both, use `ParamSet` or disjoint filters (see [Troubleshooting](#troubleshooting)).

**Generic code note:** 0.19 added nested query access (data from multiple entities per item), so iteration methods now require an `IterQueryData` bound. Concrete queries are unaffected; generic helpers may need `fn f<D: IterQueryData>(...)`.

---

## 5. Resources

Resources are still declared and accessed the familiar way:

```rust
#[derive(Resource, Default)]
struct Score(u32);

fn award_points(mut score: ResMut<Score>) {
    score.0 += 10;
}
```

Under the hood in 0.19 they're components on singleton entities, which has real consequences:

- **`Resource` is a subtrait of `Component`.** A type can no longer derive both. If you had `#[derive(Component, Resource)]`, split it into two types — otherwise inserting the component will despawn the resource entity, and `Query<&T>` will surface the resource.
- **Broad queries can now conflict with resource access.** `Query<EntityMut>` alongside `Res<Foo>` is a conflict; filter with `Without<IsResource>` (a marker on every resource entity).
- **Resources can be immutable.** Generic code taking `ResMut<R>` needs `R: Resource<Mutability = Mutable>`.
- **Upside:** resources now support hooks, observers, and relationships, and can be queried alongside components.

Prefer `Res` over `ResMut` wherever possible — a single `ResMut` serializes every system that reads that resource.

---

## 6. Scheduling

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .add_systems(Startup, setup)
        .add_systems(Update, (input, movement, collision).chain())
        .add_systems(FixedUpdate, physics_step)
        .run();
}
```

- Use `.chain()` only when order genuinely matters; every chain link removes parallelism.
- Prefer `.before()` / `.after()` on named system sets over ad-hoc chains as the app grows.
- Name system sets with a `Systems` suffix (`CombatSystems`, not `CombatSet`) — this became the upstream convention in 0.17 and ecosystem crates follow it.
- Put simulation in `FixedUpdate`, rendering-adjacent and input work in `Update`.
- 0.19 replaced `ExecutorKind` with executor instances: `schedule.set_executor(SingleThreadedExecutor::new())`.

Scene functions can be turned straight into spawn systems: `.add_systems(Startup, level.spawn())`.

---

## 7. Events, messages, observers

Since 0.17 these are two distinct concepts, and conflating them is the most common source of stale-API confusion.

**Messages** are buffered and polled — the old `EventWriter`/`EventReader` model:

```rust
#[derive(Message)]
struct LevelUp { entity: Entity }

fn emit(mut writer: MessageWriter<LevelUp>) { /* writer.write(...) */ }
fn consume(mut reader: MessageReader<LevelUp>) {
    for msg in reader.read() { /* ... */ }
}
```

**Events** are triggered and observed, running immediately:

```rust
#[derive(EntityEvent)]
struct Damage { entity: Entity, amount: f32 }

fn setup(app: &mut App) {
    app.add_observer(|damage: On<Damage>, mut health: Query<&mut Health>| {
        if let Ok(mut hp) = health.get_mut(damage.entity) {
            hp.0 -= damage.amount;
        }
    });
}

// commands.trigger(Damage { entity, amount: 10.0 });
```

`#[derive(Event)]` is global/untargeted; `#[derive(EntityEvent)]` targets the `entity` field and also runs global observers. Add `#[entity_event(propagate)]` to bubble up the `ChildOf` hierarchy.

Lifecycle events are observed the same way: `On<Add, Enemy>`, `On<Insert, T>`, `On<Discard, T>`, `On<Remove, T>`, `On<Despawn, T>`.

New in 0.19, observers accept run conditions:

```rust
app.add_observer(on_damage.run_if(|paused: Res<Paused>| !paused.0));
```

**Choosing between them:** messages for decoupled, throughput-oriented streams that tolerate a frame of latency; events for immediate, targeted reactions where the handler needs to run before anything else observes the world.

---

## 8. Relationships and hierarchy

```rust
commands.spawn((Player, children![Sword, Shield]));
```

`ChildOf` is the relationship, `Children` the relationship target. Define custom ones:

```rust
#[derive(Component)]
#[relationship(relationship_target = Inventory)]
struct StoredIn(Entity);

#[derive(Component)]
#[relationship_target(relationship = StoredIn)]
struct Inventory(Vec<Entity>);
```

Bevy rejects self-referential relationships by default (a self-`ChildOf` would loop forever during traversal). For purely semantic relations, opt in with `#[relationship(relationship_target = ..., allow_self_referential)]` — new in 0.19.

---

## 9. Performance

**Contiguous iteration (0.19).** Table storage is already a flat array; `contiguous_iter_mut()` hands you the slice so LLVM can vectorize. On a 10k-entity bulk update this was roughly 3x faster with AVX2 than normal iteration.

```rust
fn decay(mut query: Query<(&mut Health, &Decay)>) {
    for (mut health, decay) in query.contiguous_iter_mut().unwrap() {
        for (h, d) in health.iter_mut().zip(decay) {
            h.0 *= d.0;
        }
    }
}
```

It only returns `Ok` for dense queries: all fetched components must use table storage, and `Changed`/`Added` filters disqualify it (`With`/`Without` are fine). Since that's a static property of the query type, unwrapping is safe outside generic code. `bypass_change_detection()` drops the change-tick overhead for another increment of speed.

**Delayed commands (0.19)** replace hand-rolled timers:

```rust
commands.delayed().secs(1.0).spawn(Explosion);
```

There's no built-in cancellation — embed the originating `Entity` if the action should be dropped when that entity despawns.

**General guidance.** Keep components small and single-purpose so archetypes stay narrow; use `Changed<T>` to skip recomputation; use marker components to split archetypes rather than branching on an enum field inside a hot loop; measure before restructuring.

---

## Best Practices

- ✅ Model data as small, focused components; put behavior in systems.
- ✅ Use `#[require(...)]` (or a `SceneComponent`) to encode "these always go together" instead of relying on spawn discipline.
- ✅ Take the narrowest access that works: `Res` over `ResMut`, `&T` over `&mut T`, filtered queries over broad ones.
- ✅ Return `Result` from systems and use `?`; reserve `unwrap()` for genuine invariants.
- ✅ Let the scheduler parallelize — order only what must be ordered.
- ❌ Don't use `RefCell`, `Mutex`, or other interior mutability inside components to dodge the borrow checker; that hides access from the scheduler and defeats parallelism.
- ❌ Don't declare a type as both `Component` and `Resource` (impossible in 0.19, and it was a footgun before).
- ❌ Don't spawn a `SceneComponent` directly — use `spawn_scene` with `@`.
- ❌ Don't copy Bevy snippets from blog posts or Stack Overflow without version-checking them.

---

## Troubleshooting

**"Query in system conflicts with a previous system parameter" (error B0001).**
This is not a parallelism problem between two systems — the scheduler never runs conflicting systems concurrently. It means *one* system asks for aliasing access, e.g. `Query<&mut Transform, With<Player>>` and `Query<&Transform>` in the same signature. Fix by making the queries disjoint with `Without<Player>`, or wrap them in a `ParamSet` when they legitimately overlap:

```rust
fn sync(mut set: ParamSet<(Query<&mut Transform, With<Player>>, Query<&Transform>)>) {
    // access one at a time via set.p0() / set.p1()
}
```

Adding `.chain()` does *not* fix this, and is the most common wrong answer.

**System silently never runs.** Usually a fallible param: `Single` found zero or multiple matches, or a `Res<T>` doesn't exist yet. Switch to `Option<Single<...>>` or `Query`, or check that the resource is initialized before the schedule runs.

**Required components not applied.** Required components are inserted on *insert*, so a component added before the requirement was registered won't retroactively gain it. Register runtime requirements (`world.register_required_components::<A, B>()`) during plugin build, before spawning.

**A resource shows up in an entity query.** You have a type deriving `Resource` being inserted as a component, or a broad query (`Query<EntityMut>`, `Query<Entity>`, `Query<Option<&T>>`) sweeping resource entities. Add `Without<IsResource>`.

**Change detection fires every frame.** `ResMut`/`&mut` marks changed on deref, not on actual mutation. Compare before assigning, or use `bypass_change_detection()`.

---

## Renames cheat sheet

Most common breakages when reading pre-0.19 code:

| Old | Current (0.19) | Since |
|---|---|---|
| `time.delta_seconds()` | `time.delta_secs()` | 0.16 |
| `EventWriter` / `EventReader` / `Events<T>` | `MessageWriter` / `MessageReader` / `Messages<T>` | 0.17 |
| `Event` (buffered) | `Message`; `Event` now means observable | 0.17 |
| `Trigger<E>` | `On<E>` | 0.17 |
| `world.trigger_targets(E, entity)` | `#[derive(EntityEvent)]` + `world.trigger(E { entity })` | 0.17 |
| `Parent` | `ChildOf` | 0.16 |
| `#[require(Foo(ctor_fn))]` | `#[require(Foo = ctor_fn())]` | 0.17 |
| `Replace` event / `on_replace` | `Discard` / `on_discard` | 0.19 |
| `DefaultErrorHandler` | `FallbackErrorHandler` | 0.19 |
| `ExecutorKind` + `set_executor_kind` | executor instances + `set_executor` | 0.19 |
| `SceneRoot` / `DynamicScene` / `bevy_scene` (old) | `WorldAssetRoot` / `DynamicWorld` / `bevy_world_serialization` | 0.19 |
| `MySet` naming for system sets | `MySystems` | 0.17 |
| Bundles as the unit of construction | required components, then BSN scenes | 0.15, 0.19 |

Also check `Cargo.toml`: as of 0.19 the `audio` and `ui` features are no longer implied by `2d`/`3d`, so a `default-features = false` setup may silently lose them.

## References

- Release notes: <https://bevy.org/news/bevy-0-19/>
- Migration guide 0.18 → 0.19: <https://bevy.org/learn/migration-guides/0-18-to-0-19>
- API docs: <https://docs.rs/bevy/0.19.0/bevy/>
- Official examples (authoritative for current idiom): <https://github.com/bevyengine/bevy/tree/v0.19.0/examples>
