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

Services depend on this crate as a **git submodule** at
`shared-contracts/beevulyk-proto`, then reference the generated types via the
matching Rust module path, e.g.:

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

| Package              | Purpose                                       |
|----------------------|-----------------------------------------------|
| `identity.users.v1`  | User registration and identity bounded ctx.   |
