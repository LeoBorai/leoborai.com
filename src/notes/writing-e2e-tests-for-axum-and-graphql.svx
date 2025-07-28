---
title: "Writing E2E Tests for Axum & GraphQL"
description: "A tutorial on how to write E2E Tests for your Axum/GraphQL server"
categories: [rust, axum, graphql, testing]
icon: rust
langs: [rust]
date: 2023-06-25
preview_image_url: "https://images.unsplash.com/photo-1596496181848-3091d4878b24?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=2070&q=80"
published: true
---

As your application grows in features you want to make sure different flows
work as expected. For isolated pieces of logic it's very practical to use the
built-in approach for tests in `cargo`.

In other cases you may encounter a more complex scenario, with different layers,
from data input to database queries. You will expect an specific output given
certain input.

## The Project

Consider one of my very rencent projects [Gabble][1], a chat solution you can
host yourself.

Gabble has different layers of logic, every operation will be processed in each
layer in the following order:

1. GraphQL
2. Services (Domain Logic)
3. Database (PostgreSQL)

> Gabble stills in a very early development, but you can join us and help building it!
> Visit our Discord server <a href="https://discord.gg/yde6mcgs2C" target="_blank">Whizzes</a>,
> a place to learn Rust and Svelte with English/Spanish speakers by building in the Open Source.

Its important to make sure each layer works as expected, when changes are made
on certain layers we dont want to have regression bugs from unexpected behavior.

In order to do this we have to write tests that mimics a user from the platform.

### The Stack

Gabble is written in Rust, using the Axum as the HTTP Server Framework,
Async-GraphQL as the GraphQL Server library, Tokio as the Async Executor/Runtime
and SeaORM as the database ORM solution for PostgreSQL. The database runs on a
Docker container.

## Using the Cargo Workspace layout with a dedicated crate for E2E tests

Gabble project uses the Workspace Crate Layout to split logic in different
crates and split respossibilities, the crates are the following:

- **Core**: Business Logic (Based on Domain Driven Design fundamentals)
- **CLI**: Developer Commands
- **Database**: Database logic implementations
- **Server**: GraphQL Server logic
- **Test**: Here is where our E2E tests live!

> You can
> <a
>   target="_blank"
>   href="https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html"
> >
>   read more on Cargo Workspaces here
> </a>!

## Writing a test compliant GraphQL Server with Async-GraphQL

With Async-GraphQL you dont need to spin up the server in order to test against
it instead you could have a server decoupled version of your GraphQL schema to
run your tests against it.

If you have any logic tied to this schema like database operations, business
logic or [value objects][2], you will encounter the GraphQL `Context` feature
useful to inject such logic.

Take a look to how Gabble uses the Async-GraphQL `Schema` struct to achieve this:

### Define your `Schema` instance

The same `Schema` used on production must be the same `Schema` used for testing,
this is the only way to guarantee coverage in your code.

```rust
use std::sync::Arc;

use async_graphql::{EmptySubscription, Schema};

use crate::context::Context;

pub use super::modules::{MutationRoot, QueryRoot};

pub type GraphQLSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn build_schema_with_context(context: &Arc<Context>) -> GraphQLSchema {
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(Arc::clone(context))
    .finish()
}
```

[Source Code](https://github.com/whizzes/gabble/blob/e7b5a2f14aece65bc03c348b720e161e0e240340/src/server/src/graphql/schema.rs#L1-L19)

Note how the `Context` is injected to the `Schema` instance, let's check on
the `Contex`'s source code and see what it is about.

```rust
use std::sync::Arc;

use crate::config::Config;
use crate::services::{Services, SharedServices};

#[derive(Clone)]
pub struct Context {
    pub services: SharedServices,
}

pub type SharedContext = Arc<Context>;

impl Context {
    pub async fn new(config: &Config) -> Self {
        let services = Services::shared(config).await;

        Self { services }
    }

    pub async fn shared(config: &Config) -> SharedContext {
        let context = Self::new(config).await;

        Arc::new(context)
    }
}
```

[Source Code](https://github.com/whizzes/gabble/blob/e7b5a2f14aece65bc03c348b720e161e0e240340/src/server/src/context.rs)

So the `Context` holds the `Services` which contain stateless logic, all the
persistance is achieved via database, to be precise, a PostgreSQL instance.

The `Config` determines parameters to run the application, these parameters
should not change actual behavior, but instead provide details such as
database URL, JWT signing secret, connections to use, and any other relevant
exernal resources configuration values.

Just to picture them in our your minds, let's have a look on the `Config` struct.

```rust
use std::env;

pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub server_port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        let database_url = env::var("DATABASE_URL").unwrap();
        let jwt_secret = env::var("JWT_SECRET").unwrap();
        let server_port = env::var("PORT").unwrap().parse::<u16>().unwrap();

        Self {
            database_url,
            jwt_secret,
            server_port,
        }
    }
}
```

[Source Code](https://github.com/whizzes/gabble/blob/e7b5a2f14aece65bc03c348b720e161e0e240340/src/server/src/config.rs)

With this setup in place we should be able to jump into tests implementation.

### Writing the E2E Tests

Now that we can use our `Schema` with arbitrary configurations on its backend,
lets write some tests.

Checkout the [`src/test/src/lib.rs`][1] file, we can see a struct called
`TestUtil`, this struct brings methods for repetitive tasks such as:

- Running database migrations
- Wipping the database
- Creating an authentication token

```rust
use std::sync::Arc;

use anyhow::Result;
use pxid::Pxid;

use database::Database;
use libserver::config::Config;
use libserver::context::Context;
use libserver::graphql::schema::{build_schema_with_context, GraphQLSchema};
use libserver::services::auth::Token;

pub const TEST_ADMIN_EMAIL: &str = "admin@whizzes.io";
pub const TEST_ADMIN_PASSWORD: &str = "R00tP@ssw0rd";
pub const TEST_JWT_SECRET: &str = "test-jwt-value";

#[cfg(test)]
mod modules;

pub struct TestUtil {
    pub context: Arc<Context>,
    pub db: Database,
    pub schema: GraphQLSchema,
}

impl TestUtil {
    pub async fn new() -> Result<Self> {
        dotenv::dotenv().ok();

        let config = Config::new();
        let context = Context::new(&config).await;
        let context = Arc::new(context);
        let schema = build_schema_with_context(&context);
        let db = Database::new(config.database_url.as_str())
            .await
            .expect("Failed to connect to DB");

        Ok(Self {
            context,
            db,
            schema,
        })
    }

    /// Creates a new instance of [`TestUtil`] and clears the database.
    /// This is equivalent to calling
    ///
    /// ```ignore
    /// use crate::TestUtil;
    ///
    /// let test_util = TestUtil::new().await;
    ///
    /// test_util.clear_db().await;
    /// ```
    ///
    pub async fn new_cleared() -> Self {
        let test_util = Self::new().await.expect("Failed to create TestUtil");

        test_util.clear_db().await;

        test_util
    }

    pub async fn clear_db(&self) {
        self.db.refresh().await.expect("Failed to refresh database");
    }

    pub fn parts(&self) -> (Arc<Context>, GraphQLSchema) {
        (Arc::clone(&self.context), self.schema.clone())
    }

    pub async fn token_create(&self, uid: Pxid) -> Token {
        self.context.services.auth.sign_token(uid).unwrap()
    }
}
```

[Source Code][3]

Feel free to provide handy methods on this struct so you can leverage "chores"
to these methods and focus on your tests.

Then let's go and implement our tests! Create a module for one of your tests,
I will use the `user_register` mutation from Gabble!

Bring into context the required dependencies:

```rust
use async_graphql::{Request, Variables};
use serde_json::json;

use crate::TestUtil;
```

Then provide a descriptive name to the test and _decorate it_ with the test
macro of your async executor of choice.

Use `TestUtil::new_cleared` to clean up the database:

```rust
#[tokio::test]
async fn creates_a_new_user() {
    let test_util = TestUtil::new_cleared().await;
    let (_, schema) = test_util.parts();
```

Then write the GraphQL mutation to test against:

```rust
    let mutation: &str = "
        mutation RegisterUser($payload: UserRegisterInput!) {
            userRegister(input: $payload) {
                user {
                    id
                    name
                    surname
                    username
                    email
                    createdAt
                    updatedAt
                }
                error {
                    code
                    message
                }
            }
        }
    ";
```

Now let's execute the request! To do this we build a `Request` instance
providing the mutation and provide any desired variables.

```rust
 let result = schema
        .execute(
            Request::new(mutation).variables(Variables::from_json(json!({
                "payload": {
                  "name": "John",
                  "surname": "Appleseed",
                  "username": "john_appleseed",
                  "password": "Root$1234",
                  "email": "john_appleseed@whizzes.io",
            }
              }))),
        )
        .await;
```

Finally test your assertions on the response, keep in mind that the returned
value is actually a JSON instance so you must access its keys using the
`Index` operator and `&'static str` instances!

```rust
    let data = result.data.into_json().unwrap();
    let user_register_user = &data["userRegister"]["user"];

    assert_eq!(user_register_user["name"], "John");
    assert_eq!(user_register_user["surname"], "Appleseed");
    assert_eq!(user_register_user["email"], "john_appleseed@whizzes.io");
    assert_eq!(user_register_user["username"], "john_appleseed");
    assert!(user_register_user["createdAt"].is_string());
    assert!(user_register_user["updatedAt"].is_string());
```

[Source Code](https://github.com/whizzes/gabble/blob/e7b5a2f14aece65bc03c348b720e161e0e240340/src/test/src/modules/user/user_register.rs)

Finally execute your tests using the integrated `cargo` test runner via:

```bash
cargo test creates_a_new_user
```

Don't forget to have your database running in the specified URL and any other
required resources in your project are running and configured as expected!

## Conclusion

In this note we can find insights and ideas on how we can setup our tests to
run against the actual environment! Make sure you test your GraphQL operations
and keep tuned for more notes.

Happy Hacking!

[1]: https://github.com/whizzes/gabble
[2]: https://domaincentric.net/blog/ddd-building-blocks-in-php-value-object
[3]: https://github.com/whizzes/gabble/blob/e7b5a2f14aece65bc03c348b720e161e0e240340/src/test/src/lib.rs
