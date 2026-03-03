# rest-gateway

Minimal REST host module that owns the Axum router and serves health endpoints.

## Overview

The `rest-gateway` crate provides:

- HTTP server lifecycle (bind, serve, graceful shutdown) via the `ModKit` `rest_host` capability
- `/health` — JSON health response with `status`, `timestamp`, and `version`
- `/healthz` — plain-text `ok` for lightweight probes
- Standard middleware stack: request ID propagation, distributed tracing, per-request timeout

## Middleware stack

Execution order (outermost → innermost):

```
SetRequestId       — generates x-request-id if absent
PropagateRequestId — copies x-request-id to the response
TraceLayer         — opens an http_request span
push_req_id        — records x-request-id into the span
TimeoutLayer       — returns 408 after timeout_secs
Router / handlers
```

## Configuration

```yaml
modules:
  rest-host:
    config:
      bind_addr: "0.0.0.0:8080"   # default: 127.0.0.1:8080
      timeout_secs: 30             # default: 30
```

## DDD layer mapping

| Layer          | File(s)                            |
|----------------|------------------------------------|
| Interface      | `src/web.rs`                       |
| Application    | `src/module.rs`                    |
| Infrastructure | `src/config.rs`, `src/middleware/` |

## License

Licensed under Apache-2.0.
