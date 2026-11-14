---
title: "Rust and its General Purpose Nature"
description: "In this note I will share how Rust is suitable for more than Systems Programming by walking you through the Rust ecosystems and the different Software Development categories where Rust is supported."
categories: [rust, http, web, ai]
icon: rust
date: 2023-08-27
preview_image_url: "https://images.unsplash.com/photo-1675654567731-6202cd552e3b?ixlib=rb-4.0.3&q=85&fm=jpg&crop=entropy&cs=srgb&dl=ava-sol-zc6GiHMiQTE-unsplash.jpg"
published: true
---

Rust has been my programming language of choice since 2019. I moved into Rust because I wanted to dive deeply into Systems Programming, but quickly I learned that Rust is not only for Systems Programming and that it has even more to offer.

In this note I will share how Rust is suitable for more than Systems Programming by walking you through the Rust ecosystems and the different Software Development categories where Rust is supported.

Even thought Rust claims to be a Systems Programming Language it is also suitable for Web Development thanks to its capability to compile to different targets including 32bits WASM using the `wasm32-unknown-unknown` compile target.

## Web UI Frameworks

Today is not uncommon to see Web UI frameworks written in Rust, some of these includes:

- [Yew.rs](https://yew.rs)
- [Leptos](https://leptos.dev)
- [Percy](https://chinedufn.github.io/percy/)

Along with the capabality to build for WASM, this is also possible thanks to bindings that allows us to use FFI between Rust and JavaScript.

The Rust Community did a great work providing crates such as [js_sys](https://docs.rs/js-sys/latest/js_sys/), [wasm_bindgen](https://docs.rs/wasm-bindgen/latest/wasm_bindgen/) and [web_sys](https://rustwasm.github.io/wasm-bindgen/api/web_sys/), which provides you with Rust code that allows you to invoke JavaScript functions.

## HTTP Server Frameworks

Rust is also suitable for HTTP Server development, theres production ready crates for this field, including:

- [Actix](https://actix.rs)
- [Axum](https://github.com/tokio-rs/axum)
- [Rocket](https://rocket.rs)
- [Tower](https://github.com/tower-rs/tower)

## Database Drivers

You will also find database connectors for widely known databases such as PostgreSQL, MySQL, SQLite and MongoDB. Some of these perform FFI on C libraries but others like [rust-postgres](https://github.com/sfackler/rust-postgres) are written in Rust.

Its important to mention that the community is continuosly evolving in this direction, Rust developers prefer to use Rust written crates than other crates that wraps libraries written in other languages.

## Query Builders and ORMs

You don't have to write your SQL manually — unless you want, but its not recommended — the Rust ecosystem counts with libraries to help you build injection free queries as well as ORMs for more complex taxonomies. You will find [SQLx](https://github.com/launchbadge/sqlx), a SQL query builder which allows you to validate SQL queries in development time with the help of macros and is compatible with PostgreSQL, MySQL and SQLite.

If you are looking for a ORM you will also find [Diesel.rs](https://diesel.rs/) and [SeaORM](https://www.sea-ql.org/SeaORM/), both have evolved and supports migrations and code generation to help you map database structs with your source.

## Game Development

Given that Rust [has a very small runtime](https://github.com/rust-lang/rust/blob/master/library/std/src/rt.rs) and thanks to the wide list of [supported compile targets](https://doc.rust-lang.org/rustc/platform-support.html), companies such as [Embark Studios](https://www.embark-studios.com/) embrace Rust for Game Development, their most popular project (at the moment of this writing) is [The Finals](https://www.reachthefinals.com/), Embark Studios is a big Rust supporter, you can read their background and what they think about Rust [here](https://github.com/EmbarkStudios/rust-ecosystem/tree/0916b1e271e708286b1aba5758aa15b45467a2b3#background).

> When we started Embark, we chose Rust as our primary language for the long term future we are building. We love the safety and robustness of the language, the ability to write high performance, safe, and (mostly) bug free code and then fearlessly refactor and change it without common lifetime/ownership, memory safety or race condition problems.
>
> — Johan Andersson ([`@repi`](http://twitter.com/repi)), CTO, Embark

Rust is considered a low-level programming language with high-level ergonomics due to its resource management model, this encourages Game Development companies because it allows them to deliver resource efficient solutions.

The Rust Community also counts with other indie and relevant game projects, [Veloren](https://veloren.net/) a multiplayer voxel RPG is not only writen in Rust, but it is also a fully [open source project](https://github.com/veloren/veloren).

You can find released games writen in Rust by the community at [arewegameyet.rs](https://arewegameyet.rs/games/released/).

## Data Science

Even when the Data Science landscape in Rust stills very young, it is promising and relevant in the data science community. As of 2023 Python is by far the language with the most mature ecosystem in this field, with libraries like MatPlotLib, Pandas, Keras, PyTorch and TensorFlow.
Rust is not being left behind in this area, thanks to solutions like [evcxr](https://github.com/evcxr/evcxr) Rust can be used in a Jupyter Notebook, check out the [Evcxr Jupyter Kernel](https://github.com/evcxr/evcxr/blob/main/evcxr_jupyter/README.md).

[Pola-rs](https://www.pola.rs/) is another huge member in the Data Science community brought by Rust developers, Pola-rs is a fast and efficient Data Frame library which allows you to read CSV files — along with other data sources — and perform queries against them. This library along with Jupyter Notebooks support opens the path for Rust in this ecosystem.
As part of an illustration, check out this snippet borrowed from Pola-rs landing page:

```rust
use polars::prelude::*;

fn example() -> Result<DataFrame, PolarsError> {
    LazyCsvReader::new("foo.csv")
        .has_header(true)
        .finish()?
        .filter(col("bar").gt(lit(100)))
        .groupby(vec![col("ham")])
        .agg(vec![col("spam").sum(), col("ham").sort(false).first()])
        .collect()
}
```

Check out more about the progress of Rust in this area on [arewelearningyet.rs](https://www.arewelearningyet.com/).

## Conclusion

Thank to Rust versatility and continuously evolving community, Rust is turning into an option for different Software Development industry categories. Learning Rust today will allow you to participate in different projects as you learn the fundamentals and key concepts of different categories.

It worth mentioning that I have skipped some of these categories in this note, Rust counts with crates for GUI Development, Mobile Development and Embedded Systems as well but it would take a longer note.
