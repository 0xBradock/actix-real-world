# Scryfall

> MTG card collection manager

The goal of this project is just to get familiar with the [Actix](https://actix.rs) web framework.
The list of the things I want to explore on Actix:

- [ ] Docs
- [ ] Testing
- [ ] Error Handling
- [ ] Group requests
- [ ] Data handling (json and form)
- [ ] Middleware (integrated and custom)
- [ ] Authentication (user/pass, OAuth2, session management)
- [ ] Protocols HTTP, HTTP2, RPC, Server Sent Events (SSE) and WebSocket
- [ ] DB integration with [sqlx](https://docs.rs/sqlx/latest/sqlx/) and [diesel](https://diesel.rs/)
- [ ] Templating (probably with [htmx](https://htmx.org/))
- [ ] CORS
- [ ] Rate limitation
- [ ] Logging
- [ ] Tracing
- [ ] Security
- [ ] Multithreading

This is created using:

- [Actix](https://actix.rs)
- [scryfall](https://scryfall.com/docs/api/bulk-data)

## Endpoints

Public:

- `GET(/)`: Return all cards
- `GET(/{name})`: Return cards
- `POST(/signup)`: Create an account
- `POST(/signin)`: Log into account

Protected:

- `GET(/collection)`: Return list of cards in collection
- `POST(/collection)`: Add card to collection
- `DELETE(/collection)`: Remove card from collection
- `GET(/user)`: Get account information
- `POST(/user)`: Create an account
- `DELETE(/user)`: Delete account