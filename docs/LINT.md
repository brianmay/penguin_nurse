# Penguin Nurse Data Linter

A CLI tool to validate and display all data validation errors in the Penguin Nurse database.

## What it checks

The linter validates:

### Consumables
- **Consumption type mismatches**: Ensures that nested consumables (ingredients) have the same consumption type as their parent consumable

### Consumptions
- **Short duration warnings**: Flags consumptions with suspiciously short durations (< 2 seconds)
- **Liquid volume mismatches**: Checks that the sum of liquid ml from ingredients matches the consumption's total liquid ml
- **Consumption type conflicts**: Ensures that consumed items have matching consumption types

## Usage

### Using Nix Flake

If you're using Nix, you can build and run the linter directly from the flake:

```bash
# Build the lint binary
nix build .#lint

# Run directly with nix run
DATABASE_URL=postgres://user:password@localhost/penguin_nurse nix run .#lint

# Or install it to your profile
nix profile install .#lint
DATABASE_URL=postgres://user:password@localhost/penguin_nurse penguin-nurse-lint
```

### Quick Start

The easiest way to run the linter is using the provided script:

```bash
# Set your database URL
export DATABASE_URL=postgres://user:password@localhost/penguin_nurse

# Run the linter
./lint.sh
```

### Manual Usage

You can also run the linter directly with cargo:

```bash
# Build the binary
cargo build --bin lint --features cli-only

# Run with your database URL
DATABASE_URL=postgres://user:password@localhost/penguin_nurse cargo run --bin lint --features cli-only
```

### Release Build

For faster execution, build in release mode:

```bash
cargo build --bin lint --features cli-only --release
DATABASE_URL=postgres://user:password@localhost/penguin_nurse ./target/release/lint
```

## Output

The linter will display:
- Number of errors found in consumables
- Number of errors found in consumptions
- Detailed error messages for each problematic record
- A summary of total errors found

Example output:
```
=== Penguin Nurse Data Lint ===

Checking consumables...
  ✓ No consumable errors found

Checking consumptions...

  Consumption: Digest 2025-03-15 14:30 (ID: 123)
    ✗ Duration PT1S is suspiciously short
    ✗ Liquid ml total from ingredients 250ml does not match consumption liquid ml 300ml

=== Summary ===
✗ Found 2 total error(s)
```

## Implementation Details

The linter uses the same validation logic as the web application (`consumable_errors` and `consumption_errors` functions) to ensure consistency. It:

1. Connects to the PostgreSQL database using Diesel async
2. Queries all consumables and their nested ingredients
3. Queries all consumptions and their consumed items
4. Runs validation checks on each record
5. Reports any errors found

The validation logic is extracted into a separate `validation` module that is shared between the web application and the CLI tool.

## Database Connection

The linter requires:
- PostgreSQL database with the Penguin Nurse schema
- `DATABASE_URL` environment variable set to a valid connection string
- Network access to the database server

## Exit Codes

- `0`: Success, no errors found
- `1`: Runtime error (database connection failed, etc.)

Note: The linter does not use a non-zero exit code when validation errors are found, only for runtime errors.
