#!/bin/sh
set -e

# npm update
rm -rf node_modules
hash="$(prefetch-npm-deps package-lock.json)"
echo "updated npm dependency hash: $hash" >&2
echo "$hash" >npm-deps-hash
