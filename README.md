<p align="center">
    <h2>Actix Real World (🚧 work in progress 🚧)</h2>
</p>

<p align="center">
    <b>The mother of all demo apps</b>
</p>

<p align="center">
  <a href="https://codecov.io/gh/0xBradock/actix-real-world" > 
 <img src="https://codecov.io/gh/0xBradock/actix-real-world/graph/badge.svg?token=79E40OWJ2E"/> 
 </a>
  <a href="https://github.com/0xBradock/actix-real-world/blob/master/openapi.yaml"><img src="https://img.shields.io/badge/Docs-OpenAPI%203.0-success" alt="OpenAPI Docs"></a>
  <a href="https://github.com/qdrant/qdrant/blob/master/LICENSE"><img src="https://img.shields.io/github/license/0xBradock/actix-real-world" alt="MIT"></a>
</p>


> This (will) complies with the [real world specification](https://github.com/gothinkster/realworld/tree/main/api)

This application will be built using the [OpenAPI](https://swagger.io/specification/) in [openapi.yaml](./openapi.yaml)

## Tooling

HTTP:

- [actix](https://actix.rs/): Web framework

Log:

- [tracing](https://docs.rs/tracing/latest/tracing/): Scoped, structural logging and diagnostics system
- [tracing-actix-web](https://docs.rs/tracing-actix-web/latest/tracing_actix_web/): Telemetry middleware for actix-web applications
- [secrecy](https://docs.rs/secrecy/0.8.0/secrecy/): Wrapper to handle secret values
- [sqlx](https://docs.rs/sqlx/latest/sqlx/): Async SQL toolkit

Security:

- [rustsec.org](https://rustsec.org/): Vulnerability database for the Rust ecosystem

Tests:

- [fake](https://docs.rs/fake/latest/fake/): Generate fake data
- [quickcheck](https://docs.rs/quickcheck/latest/quickcheck/): Random testing for property testing
- [quickcheck_macros](https://crates.io/crates/quickcheck_macros): Macros for `quickcheck`
- [rand](https://docs.rs/rand/latest/rand/): Random number generator
- [reqwest](https://docs.rs/reqwest/latest/reqwest/): High-level HTTP client
- [validator](https://crates.io/crates/validator): Simple struct validation
- [wiremock](https://github.com/lukemathwalker/wiremock-rs): Mocking HTTP

## Secondary Goals

The goal of this project is just to get familiar with the [Actix](https://actix.rs) web framework. I'll probably be writing about those topics.

- [ ] Docs
- [ ] Testing
- [ ] Error Handling
- [ ] Group requests
- [ ] Data handling (json and form)
- [ ] Middleware (integrated and custom)
- [ ] Authentication (user/pass, OAuth2, session management)
- [ ] Protocols HTTP, HTTP2, HTTPS, RPC, Server Sent Events (SSE) and WebSocket
- [ ] DB integration with [sqlx](https://docs.rs/sqlx/latest/sqlx/) and [diesel](https://diesel.rs/)
- [ ] Templating (probably with [htmx](https://htmx.org/))
- [ ] CORS
- [ ] Rate limitation
- [ ] Logging
- [ ] Tracing
- [ ] Security
- [ ] Multithreading

## Known Issues
