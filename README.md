# Penguin Nurse

An open-source health tracking web application for recording consumables (foods, medications, supplements) and health metrics (blood glucose, weight, etc.). All data is stored locally in PostgreSQL — you retain full control.

Designed for technically inclined individuals who prefer local-first health tracking without third-party dependencies.

## Features

- Track **consumables** — foods, medications, supplements, and their nutritional/ingredient details
- Record **health metrics** — blood glucose, weight, and other measurements over time
- Local-first — all data stays on your own PostgreSQL database
- Flexible and extensible data model

## History

Penguin Nurse was presented at [Everything Open 2026](https://2026.everythingopen.au/schedule/presentation/23/) in Canberra, Australia. Video recordings were not available at time of publication.

## Getting Started

### Development Setup

This is a Rust-based full-stack application using Dioxus (frontend/WASM) and Axum (backend). The build environment is managed via Nix flakes and devenv.

1. Enter the development shell:
   ```bash
   nix develop
   ```
   or
   ```bash
   cd .devenv && source bin/activate
   ```

2. Start the Tailwind CSS compiler (in a separate terminal):
   ```bash
   npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
   ```

3. Start the development server:
   ```bash
   dx serve --platform web
   ```

   For desktop platforms:
   ```bash
   dx serve --platform desktop
   ```

### Production Build

```bash
nix build .#default
```

### Linting

Validate database records and check for data inconsistencies:

```bash
DATABASE_URL=postgres://user:password@localhost/penguin_nurse nix run .#lint
```

Or with Cargo:
```bash
export DATABASE_URL=postgres://user:password@localhost/penguin_nurse
./lint.sh
```

See [docs/LINT.md](docs/LINT.md) for details.

## Naming Conventions

### Verbs

- **Add** — add an item to a list.
- **Remove** — remove an item from a list.
- **Create** — create a new entity.
- **Archive** — non-permanently hide or archive an entity.
- **Delete** — permanently delete an entity.
- **Update** — update an entity or value.
- **View** — view an entity or value.
- **Change** — pending changes to an entity.
- **New** — pending entity to be created.

### Pattern

`[noun][verb][subverb]`

For example: `ConsumableUpdateBasic`, `ConsumableUpdateIngredients`.

## License

This project is licensed under the GNU Affero General Public License v3.0 (AGPL-3.0). See the [LICENSE](LICENSE) file for details.
