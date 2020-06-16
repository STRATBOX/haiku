# H A I K U

Rust microservices, jwt, mongodb and cloudrun

## Running the service as an image

This service bundles mongodb and other necessary infrastructure to run locally.

After making code updates, simply start service with the following commands:

```sh
$ docker-compose build
$ docker-compose up
```

### performance testing the service

Download and run [autocannon](https://github.com/mcollina/autocannon) with the appropriate parameters.

```sh
$ autocannon -c 100 -d 5 -p 10 http://0.0.0.0:3000
Running 5s test @ http://0.0.0.0:3000
100 connections with 10 pipelining factor

┌─────────┬──────┬──────┬────────┬────────┬─────────┬──────────┬───────────┐
│ Stat    │ 2.5% │ 50%  │ 97.5%  │ 99%    │ Avg     │ Stdev    │ Max       │
├─────────┼──────┼──────┼────────┼────────┼─────────┼──────────┼───────────┤
│ Latency │ 0 ms │ 0 ms │ 119 ms │ 146 ms │ 9.92 ms │ 31.65 ms │ 247.82 ms │
└─────────┴──────┴──────┴────────┴────────┴─────────┴──────────┴───────────┘
┌───────────┬─────────┬─────────┬─────────┬─────────┬─────────┬─────────┬─────────┐
│ Stat      │ 1%      │ 2.5%    │ 50%     │ 97.5%   │ Avg     │ Stdev   │ Min     │
├───────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┤
│ Req/Sec   │ 8163    │ 8163    │ 10391   │ 11183   │ 9898    │ 1129.36 │ 8160    │
├───────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┤
│ Bytes/Sec │ 1.78 MB │ 1.78 MB │ 2.27 MB │ 2.44 MB │ 2.16 MB │ 246 kB  │ 1.78 MB │
└───────────┴─────────┴─────────┴─────────┴─────────┴─────────┴─────────┴─────────┘

Req/Bytes counts sampled once per second.

49k requests in 5.07s, 10.8 MB read
```

## auto reloading while building locally

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