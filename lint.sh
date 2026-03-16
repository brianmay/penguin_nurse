#!/usr/bin/env bash
# Penguin Nurse Data Linter
# This script runs the database lint checker to find data validation errors

set -euo pipefail

# Check if DATABASE_URL is set
if [ -z "${DATABASE_URL:-}" ]; then
    echo "Error: DATABASE_URL environment variable is not set"
    echo "Please set it before running the linter:"
    echo "  export DATABASE_URL=postgres://user:password@localhost/database"
    exit 1
fi

# Build and run the lint binary
echo "Building lint tool..."
cargo build --bin lint --features cli-only --quiet

echo "Running lint checks..."
cargo run --bin lint --features cli-only --quiet
