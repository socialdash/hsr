# HSR

Build fast HTTP apis fast, with Rust, and [OpenAPI](https://swagger.io/docs/specification/about/)

 * Define your API as an [OpenAPI 3.0](https://github.com/OAI/OpenAPI-Specification) spec
 * Code-gen a server/client interface
 * Implement your interface (simple, safe, strongly typed!)
 * Run! 'If it compiles, it works!'


## Quickstart

Take a look at the [quickstart example](examples/quickstart). It contains the
minimum boilerplate needed to get started.

## Less Quick Start

Take a look at the [petstore example](examples/petstore) for a more in-depth example.

## Roadmap

- [x] HTTP server
- [x] HTTP client
- [x] Type-safe path/query/json handling
- [x] Supports all HTTP verbs
- [x] High performance
- [x] Based on `async/await` and `actix-web 2.0`
- [x] Use async in trait (partial - uses the amazing [`async-trait`](https://github.com/dtolnay/async-trait))
- [x] Works on stable Rust
- [x] Benchmarks
- [ ] HTTPS
- [ ] Better tests
- [ ] Return content-types other than JSON
- [ ] AnyOf/OneOf schema support
- [ ] Support all features necessary to get `petstore-expanded.yaml` working
- [ ] Support headers
- [ ] Advanced server configuration (with middleware etc)
- [ ] Even less boilerplate
- [ ] support JSON (not just YAML) schema
- [ ] Tutorial
- [ ] Auto-generate a client binary

## FAQ

**What's the difference between this and [swagger-rs](https://github.com/Metaswitch/swagger-rs)?**

* `swagger-rs` is a more mature projuect with institutional backing.
* It uses `futures v0.1` throughout - `hsr` attempts to use `futures v0.3` to allow `async/await` syntax
  -  Consequently - it works on stable, `hsr` requires nightly
* Requires the codegenerator, written in Java - `hsr` is 100% Rust
  - That means that the `swagger-rs` code generator is likely much more correct...
  - ..but the `hsr` build process is much more seamlessly integrated into typical Rust workflow
* Error-handling story somewhat different
* Surely lots of other differences

**What do you mean, 'fast'?**

It uses [Actix-Web](https://github.com/actix/actix-web) under the hood, rated as one of the
fastest web frameworks by [techempower](https://www.techempower.com/benchmarks/#section=data-r18&hw=ph&test=fortune).

As a simple and not-very-scientific benchmark, on a Dell XPS 13 9370/i7-8550U CPU @ 1.80GHz
(4 cores, 8 threads), [`wrk2`](https://github.com/giltene/wrk2) measured around:

* 170,000 requests/second for an empty GET request
* 150,000 requests/second for a POST request with a JSON roundtrip

See [bench example](/examples/bench) for details.

**Why the name?**

I like fast trains.

## License

MIT
