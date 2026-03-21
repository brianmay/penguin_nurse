# AGENTS.md — Developer & Agent Reference

## Project Overview

`penguin_nurse` is a full-stack Rust web application built with **Dioxus 0.7** (React-like
UI framework), **Axum 0.8** (HTTP server), **Diesel 2.2** (async PostgreSQL ORM), and
**axum-login** (OIDC-based authentication). The frontend compiles to WASM; the backend runs
on the server. CSS is Tailwind v4 + DaisyUI v5; a small barcode-scanner JS bundle is built
with Rollup. The build environment is managed via **Nix flakes** and **devenv**.

---

## Build & Development Commands

All commands assume you are inside the Nix devenv shell (`devenv shell` or `nix develop`).

### Start development server
```bash
dx serve --platform web
```

### Compile Tailwind CSS (watch mode)
```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

### Build JS bundle (barcode polyfill)
```bash
./node_modules/.bin/rollup --config rollup.config.mjs
```

### Build the data-linter CLI binary
```bash
cargo build --bin lint --features cli-only
# or in release mode:
cargo build --bin lint --features cli-only --release
```

### Production build (Nix — fully reproducible)
```bash
nix build .#default   # fullstack web app
nix build .#lint      # lint binary only
```

---

## Lint & Type-Check Commands

```bash
cargo check                         # fast type check (all features)
cargo check --features cli-only     # check the lint binary specifically
cargo clippy                        # lints; fix warnings before committing
cargo fmt                           # auto-format (rustfmt, Rust 2024 edition)
cargo fmt --check                   # verify formatting without writing changes
```

There is no custom `rustfmt.toml`; default `rustfmt` settings apply.

---

## Testing

**There are no unit or integration tests in Rust source.** The `[dev-dependencies]`
section in `Cargo.toml` is intentionally empty.

The only automated test is a **NixOS integration/smoke test** that:
1. Starts the full `penguin-nurse.service` in a NixOS VM.
2. Waits for port 4000 to open.
3. Hits `GET http://localhost:4000/_health` and asserts success.

### Run the integration test
```bash
nix flake check --impure
```

This is also what CI runs (see `.github/workflows/ci.yml`). There is no way to run a
"single test" in the traditional sense — the full flake check is the test suite.

### Data linter (manual validation)
```bash
export DATABASE_URL=postgres://user:password@localhost/penguin_nurse
./lint.sh
# or:
DATABASE_URL=postgres://... nix run .#lint
```

---

## Code Style

### Rust Edition
All code targets **Rust 2024 edition** (`edition = "2024"` in `Cargo.toml`).

### Formatting
- Use `cargo fmt` before committing. No custom `rustfmt.toml`.
- Add `#[rustfmt::skip]` sparingly for macros/enums where formatting would be harmful
  (e.g., the `Route` enum in `src/main.rs`).

### Non-snake-case allowance
UI-heavy files (components, forms) include `#![allow(non_snake_case)]` at the top
because Dioxus components must be `PascalCase` functions.

### Import ordering
Group imports as follows, separated by blank lines:
1. `std::*`
2. External crates (alphabetical)
3. `crate::*` (internal)

Example:
```rust
use std::{num::ParseIntError, str::FromStr};

use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    forms::fields::InputString,
    models::consumables::Consumable,
};
```

---

## Naming Conventions

### Domain naming pattern: `[Noun][Verb][SubVerb]`
This applies to Dioxus components, operation enums, dialog variants, etc.

| Example | Meaning |
|---------|---------|
| `ConsumableUpdateBasic` | Update basic fields of a Consumable |
| `ConsumableUpdateIngredients` | Update ingredients of a Consumable |
| `ConsumableDelete` | Permanently delete a Consumable |
| `ConsumableCreate` | Create a new Consumable |

### Verb vocabulary (use these precisely)
- **Add** — add an item to a list
- **Remove** — remove an item from a list
- **Create** — create a new entity (persisted)
- **Archive** — non-permanently hide/archive an entity
- **Delete** — permanently delete an entity
- **Update** — update an entity or value
- **View** — view an entity or value
- **Change** — pending (unsaved) changes to an entity
- **New** — pending (unsaved) entity to be created

### Item-level conventions
| Kind | Convention | Example |
|------|-----------|---------|
| Structs / Enums | `PascalCase` | `Consumable`, `MaybeSet`, `ConsumableUnit` |
| Dioxus components | `PascalCase` | `ConsumableUpdate`, `InputString` |
| Functions / methods | `snake_case` | `create_consumable`, `get_user_id` |
| Variables | `snake_case` | `consumable_id`, `active_dialog` |
| Server functions | `snake_case` (with `#[server]`) | `delete_nested_consumable` |
| Constants | `SCREAMING_SNAKE_CASE` | `ORGANIC_SVG`, `TAILWIND_CSS` |
| Files | `snake_case.rs` | `consumables.rs`, `health_metrics.rs` |
| Validation functions | `validate_<field>` | `validate_name`, `validate_blood_glucose` |

---

## Types & Data Models

### Three-variant model pattern
Each domain entity has three model structs:
- **`Foo`** — the read/display model returned from the database
- **`NewFoo`** — the creation model (all required fields)
- **`ChangeFoo`** — the update/patch model using `MaybeSet<T>` fields

### `MaybeSet<T>`
A custom enum in `src/models/common.rs` used for partial updates (patch semantics).
Use it instead of `Option<T>` when a field can be explicitly set to `None` vs. left
unchanged.

### Newtype IDs
All entity IDs are newtype structs over `i64`:
```rust
pub struct ConsumableId(pub i64);
```
This prevents accidental ID cross-assignment at compile time.

### Derive macros
All model structs derive: `Serialize, Deserialize, Debug, Clone, PartialEq, Eq`.
Add `Hash` when the type will be used in a `HashMap`/`HashSet`.

### `FieldValue` and `FieldLabel` traits
- `FieldValue` — bidirectional `String` ↔ typed conversion for form fields (`src/forms/values.rs`)
- `FieldLabel` — renders an enum variant as a human-readable label for dropdowns
- `AllValues` (from `derive_enum_all_values`) — enumerates all variants of an enum

---

## Error Handling

1. **Use `thiserror`** for all error types. Derive `#[derive(Error, Debug)]`.

2. **`AppError`** (`src/functions/common.rs`) is the top-level server error enum.
   It `#[from]`-converts database and bb8 pool errors.

3. **Server function error chain**: always end with:
   ```rust
   .map_err(AppError::from)
   .map_err(ServerFnError::from)
   ```

4. **`ValidationError(String)`** — simple newtype for form field validation errors.

5. **`EditError`** — wraps either a `ServerFnError` or a `ValidationError`, used as
   the error type in form save operations.

6. **`Saving` enum** — models async form submission state:
   `No` | `Yes` | `Finished(Result<(), EditError>)`.

7. Use `?` freely. Use `.optional()` for database queries that may return zero rows.

8. **No panics** in normal flow. Only `expect()` for mandatory server extensions
   (startup-time assertions).

---

## Architecture Patterns

### Server functions (`src/functions/`)
Every `#[server]` function follows this exact pattern:
```rust
#[server]
pub async fn do_thing(arg: Arg) -> Result<Output, ServerFnError> {
    let user_id = get_user_id().await?;                    // 1. authenticate
    let mut conn = get_database_connection().await?;       // 2. get DB connection
    // 3. delegate to src/server/database/models/
    db::do_thing(&mut conn, user_id, arg)
        .await
        .map(|x| x.into())                                 // 4. convert DB model → frontend model
        .map_err(AppError::from)                           // 5. map errors
        .map_err(ServerFnError::from)
}
```

### Dioxus components (`src/components/`, `src/views/`)
- State: `use_signal`, `use_memo`, `use_resource`, `use_callback`
- Async side-effects: `spawn(async move { ... })`
- Dialog state: an `ActiveDialog` signal holding an enum of which dialog is open
- Template: `rsx!{ ... }` macro with Tailwind + DaisyUI class strings
- Components with validation use a `Validate` struct from `src/forms/` holding
  a `Memo<Result<T, ValidationError>>` per field

### Feature flags
- `#[cfg(feature = "server")]` — gates all server-only code (Diesel, Axum, auth).
  Model files use this to conditionally add Diesel derives.
- `cli-only` — feature used to build the `lint` binary without the full web stack.

### `tap::Pipe`
The `tap` crate's `.pipe(fn)` is used for readable method-chaining of results and
values. Prefer `.pipe(Ok)` over wrapping in `Ok(...)` at end of expression chains.
