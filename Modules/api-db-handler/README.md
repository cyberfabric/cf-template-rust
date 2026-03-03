# api-db-handler

Pokemon management module with REST API, database storage, and inter-module communication via `ClientHub`.

## Module Structure

```
api-db-handler/
├── sdk/                              # Standalone SDK crate for external consumers
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs                    # Public exports
│       ├── models.rs                 # Pokemon struct (transport-agnostic)
│       ├── errors.rs                 # PokemonError (safe to expose externally)
│       ├── client.rs                 # PokemonClientV1 trait + PokemonStreamingClientV1
│       └── odata/
│           └── pokemon.rs            # PokemonQuery, PokemonSchema, PokemonFilterField
└── src/
    ├── lib.rs                        # Crate root and public re-exports
    ├── config.rs                     # PokemonConfig (page sizes)
    ├── errors.rs                     # API error definitions
    ├── module.rs                     # PokemonModule — ModKit registration point
    │
    ├── api/
    │   └── rest/
    │       ├── dto.rs                # PokemonDto — REST wire format with serde/utoipa
    │       ├── error.rs              # DomainError → RFC 9457 Problem mapping
    │       ├── handlers/
    │       │   └── pokemon.rs        # get_pokemon, list_pokemon handler functions
    │       └── routes/
    │           └── pokemon.rs        # OperationBuilder route + OpenAPI registration
    │
    ├── domain/
    │   ├── error.rs                  # DomainError + conversions to PokemonError
    │   ├── repos/
    │   │   └── pokemon_repo.rs       # PokemonRepository trait (get, list_page)
    │   ├── service/
    │   │   ├── mod.rs                # AppServices container, ServiceConfig, DbProvider
    │   │   └── pokemon.rs            # PokemonService — business logic
    │   └── local_client/
    │       ├── client.rs             # PokemonLocalClient — implements PokemonClientV1
    │       └── streaming.rs          # LocalPokemonStreamingClient — streaming adapter
    │
    └── infra/
        └── storage/
            ├── db.rs                 # db_err helper
            ├── entity/
            │   └── pokemon.rs        # SeaORM entity (DeriveEntityModel, Scopable)
            ├── mapper.rs             # entity::Model → api_db_handler_sdk::Pokemon conversion
            ├── odata_mapper.rs       # PokemonFilterField → SeaORM Column mapping
            ├── pokemon_sea_repo.rs   # OrmPokemonRepository — implements PokemonRepository
            └── migrations/
                └── m20260111_000001_initial.rs  # CREATE TABLE pokemon
```

### Layer responsibilities

| Layer              | Package path                                | Rule                                                                     |
|--------------------|---------------------------------------------|--------------------------------------------------------------------------|
| **SDK**            | `api-db-handler-sdk` (`api_db_handler_sdk`) | Public contract. No server code, no DB. Safe to expose to other modules. |
| **API**            | `crate::api`                                | HTTP concerns only. Translates HTTP ↔ domain. No business logic.         |
| **Domain**         | `crate::domain`                             | Business logic and rules. Must not import `api::*` or `infra::*`.        |
| **Infrastructure** | `crate::infra`                              | Database persistence. Implements domain repository traits.               |

---

## Data Flow

### GET /pokemon/v1/pokemon/{id}

```
HTTP Request
    │
    ▼
axum::Router  (routes/pokemon.rs)
    │  Extracts: Path(id), Extension(SecurityContext), OData($select)
    │
    ▼
handlers::get_pokemon  (handlers/pokemon.rs)
    │  Calls: svc.pokemon.get_pokemon(&ctx, id)
    │
    ▼
PokemonService::get_pokemon  (domain/service/pokemon.rs)
    │  Acquires: db.conn()  →  SecureConn
    │  Builds:   AccessScope::for_tenant(ctx.subject_tenant_id())
    │  Calls:    repo.get(&conn, &scope, id)
    │
    ▼
OrmPokemonRepository::get  (infra/storage/pokemon_sea_repo.rs)
    │  Executes: PokemonEntity::find()
    │              .filter(Column::Id = id)
    │              .secure().scope_with(scope)
    │              .one(conn)
    │
    ▼
mapper::From<entity::Model>  (infra/storage/mapper.rs)
    │  Converts: SeaORM Model → api_db_handler_sdk::Pokemon
    │
    ▼
handlers::get_pokemon  (back in handler)
    │  Maps:    Pokemon → PokemonDto
    │  Applies: $select field projection (apply_select)
    │
    ▼
JSON Response  (axum::Json<serde_json::Value>)
```

### GET /pokemon/v1/pokemon (paginated list)

```
HTTP Request  (?$filter=name eq 'Pikachu'&$orderby=id desc&limit=20&cursor=...)
    │
    ▼
axum::Router  (routes/pokemon.rs)
    │  Extracts: Extension(SecurityContext), OData(query)
    │
    ▼
handlers::list_pokemon  (handlers/pokemon.rs)
    │  Calls: svc.pokemon.list_pokemon_page(&ctx, &query)
    │
    ▼
PokemonService::list_pokemon_page  (domain/service/pokemon.rs)
    │  Acquires: db.conn()  →  SecureConn
    │  Builds:   AccessScope::for_tenant(ctx.subject_tenant_id())
    │  Calls:    repo.list_page(&conn, &scope, query)
    │
    ▼
OrmPokemonRepository::list_page  (infra/storage/pokemon_sea_repo.rs)
    │  Calls: paginate_odata::<PokemonFilterField, PokemonODataMapper, ...>(
    │             base_query, conn, query, ("id", Desc), limit_cfg, Into::into
    │         )
    │
    ├── odata_mapper::PokemonODataMapper   ($filter → SeaORM Condition)
    │     PokemonFilterField::Id       → Column::Id
    │     PokemonFilterField::Name     → Column::Name
    │     PokemonFilterField::CreatedAt → Column::CreatedAt
    │
    └── mapper::From<entity::Model>   (per row: SeaORM Model → api_db_handler_sdk::Pokemon)
    │
    ▼
handlers::list_pokemon  (back in handler)
    │  Maps:    Page<Pokemon> → Page<PokemonDto>
    │  Applies: $select field projection (page_to_projected_json)
    │
    ▼
JSON Response  (axum::Json<serde_json::Value>)
```

### Inter-module communication (ClientHub)

Other modules can consume this module without HTTP by obtaining the client from `ClientHub`:

```
module.rs: PokemonModule::init()
    └── registers Arc<PokemonLocalClient> as dyn PokemonClientV1

Consumer module:
    let client = hub.get::<dyn PokemonClientV1>()?;
    let pokemon = client.get_pokemon(ctx, id).await?;
    // or stream all results:
    let stream = client.pokemon().stream(ctx, query_builder);
```

```
PokemonClientV1::get_pokemon / list_pokemon  (sdk/src/client.rs)
    │
    ▼
PokemonLocalClient  (domain/local_client/client.rs)
    │  Converts: DomainError → PokemonError
    │
    ▼
PokemonService  (same domain service used by REST handlers)
```
