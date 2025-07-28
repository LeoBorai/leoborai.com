---
title: "Desarrollando un servidor HTTP en Rust"
description: "Introduccion a Axum para desarrollar un servidor HTTP en Rust"
categories: [rust, axum, http, rest]
icon: rust
langs: [rust]
date: 2023-09-12
preview_image_url: "https://images.unsplash.com/photo-1615834751896-b15e2330b289?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=1734&q=1"
published: true
---

## Por qué Rust?

En la mayoria de los lenguajes de programación nos vemos en la obligación de
hacer un intergambio entre eficiencia, ergonomía o seguridad.

Lenguajes como JavaScript/TypeScript, nos brindan ergonomía, tenemos el foco en
la lógica de negocio, las reglas que se deben de implementar, pero no tenemos
garantias en la fidelidad. Existen problemas de tipos, donde no se asegura que
un valor sera un `number` hasta el final de la función, tenemos casos similares
al `NullPointerException`, donde se asume que un objeto está presente e
intentamos acceder a un campo, el que termina en `Cannot read property 'X' of null`,
esto sin considerar el costo en recursos del runtime V8; Gecko; etc y el
[Garbage Collector][1].

Lenguajes como C y C++, son bastante eficientes en el contexto de ejecución, no
tienen un [Garbage Collector][1], el lenguaje nos permite acceder de forma
nativa a las librerias del sistema, también es posible escribir procesos
multi-thread tanto con [green threads][2] como con threads regulares.

El intercambio en estos lenguajes no se encuentra en la eficiencia, si no en la
seguridad, tanto en C como en C++ heredamos la tarea de administrar memoria de
forma manual, cada objeto dinámico como `arrays`, `struct`s ó `class`es, debe
ser explicitamente alojado en memoria, es consumido y luego desalojado.

Si esta tarea no se hace de forma correcta, podemos tener probelmas de memoria
como [_memory leaks_][3] ó [_null pointer exceptions_][4].

Otra complejidad en estos lenguajes es el manejo de dependencias, no existe un
servicio predominante encargado de este aspecto en C ó C++, por lo general
los programas en C y C++ usan librerias del sistema para funcionar. Por ejemplo,
si quieres hacer un pedido HTTP probablemente uses la librería de cURL para esto.

Podriamos hacer un review de cada lenguaje pero este seria una nota muy larga,
así que saltare a Rust.

Rust se enfoca en resolver los problemas anteriormente mencionados, el diseño
del lenguaje busca hacer _Low Level Programming with High Level Ergonomics_,
básicamente se quirere lograr un lenguaje sencillo de escribir pero tan
eficiente como C y C++, Rust logra esto a través de su diseño.

- Rust es rápido, compila a lenguaje de máquina y no usa un runtime, al menos de
que se agrege de forma explícita.
- Rust no tiene un Garbage Collector, de lo contrario Rust usa el método RAII
que significa _Resource Acquisition Is Initialization_, básicamente no eres
responsable de alojar ó desalojar memoria (al menos de que quieras hacerlo claro).
- Rust identifica código inseguro al momento de compilar, si por ejemplo
intentas acceder a un vector en un indice arbitrario, Rust colocará código que
verifique si el índice usado esta dentro del rango del vector.
- Rust cuenta con `cargo`, un Package Manager similar a `npm`, `pip` ó NuGet.

Estás son algunas de las caracteristicas mas relevantes, Rust también tiene un
toolchain para hacer `lint` y `format` de tu código, así como correr tests y
benchmarks sin necesidad de incluir librerias.

## Talk is Cheap, Show me The Code

Empecemos! Para continuar vas a necesitar instalar Rust en tu sistema. Rust puede
ser instalado usando [`rustup`][5], si estas usando un sistema de base Unix,
puedes usar el comando cURL que se encuntra en la página. Si estas en Windows
vas a necesitar los Build Tools que se adquieren a través de Visual Studio, hace
un tiempo escribi una guía de como hacerlo acá.

### Qué vamos a desarrollar?

Para esta nota vamos a desarrollar la solución [_The Local Library website_][7] que
está disponible en MDN (Mozilla Developer Network). Si vienes de JavaScript vas
a tener la ventaja de entendor la contraparte del código en Rust y podrás
comparar.

> Fué en Mozilla que nació Rust, el autor principal es  Graydon Hoare. [Lee más acá][8].

### Creando el Proyecto

Una vez instalado Rust en tu sistema deberas de poder correr el siguiente
comando:

```bash
cargo --version
```

Deberias de ver un output similar a este:

```bash
cargo 1.72.0 (103a7ff2e 2023-08-15)
```

Luego crearemos el proyecto en nuestro directorio favorito usando el comando
`cargo new local-library-website`.

Este comando creara un directorio `local-library-website`, que poseerá la
siguiente estructura

```
local-library-website
├── src
│   └── main.rs
└── Cargo.toml
```

El archivo `main.rs` es considerado el punto de entrada ("entrypoint"), de la
aplicación. El `entrypoint` es la función que se llamara al ejecutar la
aplicación.

Si abrimos el archivo en nuestro editor favorito veremos el siguiente contenido:

```rust
fn main() {
    println!("Hello, world!");
}
```

Aca la función `main` es el punto de entrada de nuestra aplicación, al ejecutar
`cargo run` en nuestro terminal, es la función `main` la que se ejecutará,
imprimiendo en el terminal el siguiente texto:

```bash
Hello, world!
```

### Implementando el servidor

Para implementar nuestro servidor usaremos el crate `axum` desarrollado por
contribuidores y mantenedores de Tokio. Tokio es ⎯ para el momento en que
escribo esta nota ⎯ el ejecutor asincrónico más popular en el ecosistema de
Rust

> Debido a que Rust es un lenguaje de bajo nivel, el mismo no hace suposiciones
> sobre el contexto de ejecución, esto le permite definir como se ejecutarán las
> instrucciónes en cada dispositivo según las especificaciones (registros e
> instrucciones del CPU). Por esto debemos definir el ejecutor asincrónico a
> usar explícitamente.

Instalaremos `axum` y `tokio` usando `cargo add`, a la vez habilitaremos la
feature `full` para `tokio`.

```bash
cargo add axum tokio --features tokio/full
```

> La feature `full` en `tokio` incluye mucho código que no vamos a utilizar para
> esta nota. En un proyecto para producción, te recomiendo habilitar solamente
> las features necesarias, esto incrementará la velocidad de compilación y
> reducirá el tamaño del ejecutable.

Nuestro archivo `Cargo.toml` se actualizó con el siguiente continido:

```toml
[package]
name = "local-library-website"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
axum = "0.6.20"
tokio = { version = "1.32.0", features = ["full"] }
```

Una vez agregadas las dependencias, actualizaremos nuestro `main.rs` para
implementar nuestro servidor.

Primero debemos incluir las `struct`s y las funciones necesarias. Agrega el
siguiente código al inicio del archivo.

```diff
+ use std::net::SocketAddr;

+ use axum::{Router, Server};
```

- `Router` define rutas de nuestro servidor HTTP
- `SocketAddr` representa direcciónes IP incluyendo el puerto
- `Server` se encarga de escuchar en una dirección (`SocketAddr`), los paquetes de entrada y los dirige a las rutas correspondientes usando `Router`

A continuación usaremos el macro `tokio::main` para permitirnos usar el ejecutor
asincróno de Tokio. Esto transformará nuestra función `main` en una función
asínncrona permitiendonos usar `async` y `await`.

```diff
+ #[tokio::main]
+ async fn main() {
- fn main() {
    // -- snip --
}
```

> Los macros en Rust son la implementación del lenguaje que nos permite llevar
> a cabo "[meta-programación][9]". La meta-programación nos permite escribir
> código que se expandera (ó generará) otro código basado en los parámetros
> otorgados.
>
> Este código se ejecutará en el tiempo de compilación.

Ya que nuestra función `main` es asincróna, podremos introducir nuestro servidor.
Definimos la dirección a la que se subscribirá para recibir pedidos usando
`SocketAddr`:

```rust
let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
```

- `SocketAddr::from` nos permite crear una instancia de `SocketAddr` a partir de un valor
- La tupla `([127, 0, 0, 1], 3000)` representa la dirección en 2 partes:
  - `[127, 0, 0, 1]` representa `127.0.0.1` en una IPv4, esta dirección es equivalente a `localhost`
  - `3000` será el puerto al que se subscribirá

Luego crearemos nuestro router. Este es responsable de definir las funciones a
ejecutar basado en el método HTTP (`GET`, `POST`, `PUT`, `DELETE`, entre otros),
y el `path` del URI en cuestión. Por ahora no configuraremos ninguna ruta, sin
embargo, este recurso es necesario para crear nuestro servidor en Axum.

```rust
let router = Router::new();
```

Peor último, crearemos una instancia de `Server` y asignaremos el `Router` que
acabamos de crear.

```rust
Server::bind(&addr)
    .serve(router.into_make_service())
    .await
    .expect("Failed to initialize the Local Library Website Server");
```

- `Server::bind(&addr)` toma como parámetro el `SocketAddr` creado anteriormente
- `.serve(router.into_make_service())` registra las rutas a usar
- El proceso de registrar rutas y subscibirse a una dirección es asincróno, `.await` debe ser specificado para que la función asíncrona sea ejecutada
- Debido a que `.serve(...)` devuelve una instancia de `Result<T, E>`, debemos consumir su resultado. Para simplicidad usaremos `.expect(...)` el cual provoca un Panic (Error Inrecuperable)

> En el contexto profesional debemos evitar hacer llamados a funciones que
> puedan causar `panic`. Un `panic` es considerado un error no manejado, o peor,
> un error mal manejado. Rust provee los tipos `Result` y `Option` que nos
> permiten manejar errores de forma limpia así como la posible ausencia de un
> valor. Los valores de estos tipos siempre debon ser consumidos de forma
> apropiada, determinando las intrucciones para cada caso.

Nuestro archivo `main.rs` debe tener el siguiente aspecto:

```rust
use std::net::SocketAddr;

use axum::{Router, Server};

#[tokio::main]
async fn main() {
    let router = Router::new();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .expect("Failed to initialize the Local Library Website Server");
}
```

Con este código en lugar, podemos proceder a ejecutar nuestro servidor,
notaremos que nuestra aplicación queda corriendo, el proceso no termina.

```bash
➜  local-library-website git:(main) ✗ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/local-library-website`
```

Esto quiere decir que nuestro servidor esta subscripto a la dirección
`127.0.0.1:3000` y está esperando peticiones!

> Puedes revisar el código que hemos escrito hasta ahora [acá][10].

### Presentando la primera ruta

Con nuestro servidor funcionando, ya estamos listos para crear nuestra primera
ruta. A continuación agregaremos una ruta de prueba que nos permitirá entender
la forma en que Axum consume la información de una petición.

Implementaremos una ruta de método `GET` que recibirá parámetros de búsqueda
(URL Search Params), el parametro en cuestión sera `name`.

El valor de `name` se usará para crear una oración `Hello, {name}!`, la cual
se enviará como respuesta.

Para brindar la capacidad de deserializar la información que llega a nuestro
servidor usaremos `serde`. Serde es la libreria de-facto usada en la comunidad
para serializar y deserializar información en una gran cantidad de formatos,
icluyendo BSON, JSON, YAML y TOML.

Habilitaremos la feature `derive` que nos permitirá serializar/deserializar
structs con tan solo "decorar" el struct usando este derive macro.

Instalemos `serde` habilitando el feature deseado:

```bash
cargo add serde --features serde/derive
```

Así se vera nuestro `Cargo.toml` luego de ejecutar este comando:

```diff
[package]
name = "local-library-website"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
axum = "0.6.20"
+ serde = { version = "1.0.188", features = ["derive"] }
tokio = { version = "1.32.0", features = ["full"] }
```

A continuación importaremos `serde::Deserialize` en nuestro archivo `main.rs`:

```diff
use axum::{Router, Server};
+ use serde::Deserialize;
```

Luego definimos los parametros que deseamos recibir en un `struct`, que a su vez
decoraremos con el `derive` macro `Debug` y `Deserialize`.

La anotación `#[serde(default)]` usara `String::default` en caso de que el
parámetro no esté presente en la petición.

```diff
+ #[derive(Debug, Deserialize)]
+ struct Params {
+     #[serde(default)]
+     name: String,
+ }
```

Con nuestros parámetros definidos, crearemos nuestro `handler`, será la función
a ejecutar cuando se haga una petición a la ruta `/` usando el método `GET`.

Importaremos `extract::Query` para extraer los parámetros de búsqueda y también
`router::get` para defitnir la ruta de método `GET`, ambos de `axum`.

```diff
+ use axum::extract::Query;
+ use axum::routing::get;
use axum::{Router, Server};
use serde::Deserialize;
```

Luego esciribiremos nuestra función `greeting`, para usar como `handler`:

```rust
async fn greeting(Query(params): Query<Params>) -> String {
    format!("Hello, {}!", params.name)
}
```

Por último agregamos la ruta a nuestro `Router`

```diff
- let router = Router::new();
+ let router = Router::new().route("/", get(greeting));
```

Este seria el estado final:

```rust
use std::net::SocketAddr;

use axum::extract::Query;
use axum::routing::get;
use axum::{Router, Server};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Params {
    #[serde(default)]
    name: String,
}

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(greeting));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .expect("Failed to initialize the Local Library Website Server");
}

async fn greeting(Query(params): Query<Params>) -> String {
    format!("Hello, {}!", params.name)
}
```

### Probando nuestro servidor manualmente

Ejecutemos nuestro servidor, tal como hicimos anteriormente:

```bash
cargo run
```

Luego abre tu navegador favorito y visita:

```
http://127.0.0.1:3000/?name=Esteban
```

> Por supuesto, coloca tu nombre en vez de `Esteban`

Voilà! Escribiste tu primera ruta en un servidor HTTP usando Rust. Prueba otros
nombres! Intenta cambiar el mensaje si el nombre viene vacío! Atrévete a
experimentar sin miedo a dañar el código!

[Visita los cambios que hicimos en esta seccíon en mi GitHub!][11]

## Conclusión

Esta fué la primera entrega de mi "playlist" `Desarrollando un servidor HTTP en Rust`,
la idea es llegar a aquellos desarrolladores (ó futuros desarrolladores), que
aún no manejan el inglés para que puedan probar el lenguaje de programación Rust.

[1]: https://learn.microsoft.com/en-us/dotnet/standard/garbage-collection/fundamentals
[2]: https://en.wikipedia.org/wiki/Green_thread
[3]: https://en.wikipedia.org/wiki/Memory_leak
[4]: https://en.wikipedia.org/wiki/Null_pointer
[5]: https://rustup.rs
[6]: https://leoborai.com/en/notes/installing-the-rust-programming-language-on-windows
[7]: https://developer.mozilla.org/en-US/docs/Learn/Server-side/Express_Nodejs/Tutorial_local_library_website
[8]: https://www.technologyreview.com/2023/02/14/1067869/rust-worlds-fastest-growing-programming-language/
[9]: https://subscription.packtpub.com/book/web-development/9781800560819/2/ch02lvl1sec07/metaprogramming-with-macros
[10]: https://github.com/LeoBorai/local-library-website/commit/2f5920f2af75dc6e99f85e8824fbe0b251bc58c3
[11]: https://github.com/LeoBorai/local-library-website/commit/2dc8bea553f9887b9525c2c5bad52de573c1da29
