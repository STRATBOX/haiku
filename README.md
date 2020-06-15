# H A I K U

Rust microservices, jwt, mongodb and cloudrun

## Prerequisites

The easiest way to get Cargo is to install the current stable release of **Rust** by using `rustup`.

On Linux and macOS systems, this is done as follows:

```sh
$ curl https://sh.rustup.rs -sSf | sh
```
It will download a script, and start the installation. If everything goes well, you’ll see this appear:

```sh
Rust is installed now. Great!
```

## auto reloading

For this, we need to install `cargo-watch` and `systemfd`. Both are written in Rust and available on [crates.io](http://crates.io), so we can install them with cargo.

```sh
$ cargo install systemfd cargo-watch
```
We also need to add `listenfd` to our dependencies.
```toml
[dependencies]
listenfd = "0.3"
```

Then we need to make some changes to `src/main.rs` so that we can use the listener that is provided for us by `systemfd`, but also have a fallback for cases when we don’t need it. Like when we are deploying our code.

```rust
// src/main.rs
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use listenfd::ListenFd;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(||
        App::new()
            .service(index)
    );

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind("127.0.0.1:3000")?,
    };

    server.run().await
}
```

### links
* [Mongodb client](https://github.com/mongodb/mongo-rust-driver)
* [Apache pulsar client](https://github.com/wyyerd/pulsar-rs)
* [Build an API in Rust with JWT Authentication](https://auth0.com/blog/build-an-api-in-rust-with-jwt-authentication-using-actix-web/)
* [How to create a REST API in rust](https://cloudmaker.dev/how-to-create-a-rest-api-in-rust/)
* [Rust fullstack](https://github.com/steadylearner/Rust-Full-Stack)
* [Building a microservice with rust](https://medium.com/@ilegra/building-a-microservice-with-rust-ef9641cf2331)
* [The Rust Programming Language - 2018 Edition](https://doc.rust-lang.org/book/2018-edition/index.html)