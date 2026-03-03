# background-worker

Periodically fetches a random Pokemon from the [PokeAPI](https://pokeapi.co/) and exposes the
fetch capability to other modules via `ClientHub`.

## Module structure

```
background-worker/
├── sdk/                        # Public API crate (background-worker-sdk)
│   └── src/
│       ├── client.rs           # PokemonClientV1 trait
│       ├── errors.rs           # PokemonError (public)
│       └── models.rs           # Pokemon (public)
└── src/
    ├── domain/
    │   ├── error.rs            # DomainError (internal)
    │   ├── ports.rs            # PokemonRepository trait (port)
    │   ├── service.rs          # PokemonService (domain logic)
    │   └── local_client.rs     # PokemonLocalClient (SDK adapter)
    ├── infra/
    │   ├── mod.rs              # PokemonHttpRepository (HTTP impl of PokemonRepository)
    │   └── model.rs            # PokemonResponse (raw API shape)
    └── module.rs               # BackgroundWorkerModule (modkit wiring)
```

### Layer responsibilities

| Layer      | What it does                                                                                                                                                                          |
|------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| **sdk**    | Defines the public contract (`PokemonClientV1` trait, `Pokemon` model, `PokemonError`). Other modules depend only on this crate.                                                      |
| **domain** | Pure business logic. `PokemonService` drives the use-case through the `PokemonRepository` port. Errors stay internal (`DomainError`).                                                 |
| **infra**  | `PokemonHttpRepository` implements `PokemonRepository` by calling the PokeAPI over HTTP. Maps the raw `PokemonResponse` to the SDK `Pokemon` type.                                    |
| **module** | Wires everything together. `init()` constructs the object graph and registers `PokemonLocalClient` into `ClientHub`. `start()` runs the background polling loop via `PokemonService`. |

### Data flow

```
ClientHub consumer
  └─▶ PokemonClientV1 (sdk trait)
        └─▶ PokemonLocalClient (domain/local_client.rs)
              └─▶ PokemonService (domain/service.rs)
                    └─▶ PokemonRepository port (domain/ports.rs)
                          └─▶ PokemonHttpRepository (infra/mod.rs)
                                └─▶ PokeAPI (HTTPS)
```

## Background polling

In addition to on-demand access via `ClientHub`, the module spawns a background task (every 5 s)
that calls `PokemonService::fetch_random_pokemon()` and logs the result. The loop is cancelled
gracefully via a `CancellationToken` when the application shuts down.

## Dependencies

- `cf-modkit` — module framework (`Module`, `RunnableCapability`, `ModuleCtx`, `ClientHub`)
- `cf-modkit-http` — `HttpClient` wrapper
- `background-worker-sdk` — public SDK (path dependency)
- `anyhow` — error handling in modkit boundaries
- `thiserror` — `DomainError` derive
- `async-trait` — object-safe async traits
- `tokio` / `tokio-util` — async runtime and cancellation
- `tracing` — structured logging
- `serde` — JSON deserialization of PokeAPI responses
