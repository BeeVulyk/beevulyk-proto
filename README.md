# beevulyk-proto

Canonical gRPC contracts for the **BeeVulyk** platform.

This crate holds every `.proto` file the platform speaks, plus the generated
Rust bindings produced by `tonic-build` at compile time. Each proto package is
versioned independently (e.g. `identity.users.v1`) and exposed as a Rust module
mirroring the package path.

## Layout

```
proto/
  <bounded-context>/<aggregate>/v<n>/<file>.proto
src/
  lib.rs                     # one Rust module per proto package
build.rs                     # tonic-build invocation
```

## Consumption

Services depend on this crate as a **cargo git dependency pinned to a release
tag** — never as a path dependency, which builds locally and fails in CI where
the service repo is checked out standalone. Reference the generated types via
the matching Rust module path, e.g.:

```rust
use beevulyk_proto::identity::users::v1::{
    users_service_server::UsersService,
    RegisterUserRequest,
    RegisterUserResponse,
};
```

## Conventions

- **No language-specific options** in `.proto` files.
- RPC responses use `oneof result { Success, Error }`.
- Error models are a typed enum plus an optional human-readable message.
- Timestamps are `int64` milliseconds since epoch.
- Enums (not strings) for any fixed value set.

## Current packages

| Package                    | Purpose                                                  |
|----------------------------|----------------------------------------------------------|
| `identity.users.v1`        | User registration and identity bounded ctx.              |
| `identity.profiles.v1`     | Seller profiles bounded ctx.                             |
| `marketplace.listings.v1`  | Catalogue listings bounded ctx (CRUD + lifecycle).       |
| `marketplace.orders.v1`    | Orders bounded ctx (creation + party-scoped reads).      |
| `notifications.delivery.v1`| Notification delivery bounded ctx (transactional email + device tokens). |
| `beekeeping.reference.v1`  | Closed dictionaries shared within the beekeeping domain. |
| `marketplace.reference.v1` | Closed dictionaries shared within the marketplace domain.|
| `common.geo.v1`            | Ukrainian administrative geography; belongs to no domain.|
| `common.money.v1`          | Currency; belongs to no domain.                          |

Reference (`*.reference.*`) and `common.*` packages declare no service and may
be imported by anyone. A service package must never be imported by another
domain's service package.
