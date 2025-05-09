#!/bin/bash
set -eux -o pipefail

# Generate Neynar SDK
rm -rf ./generated/api/*
npx --yes @openapitools/openapi-generator-cli \
    generate \
    --git-user-id neynarxyz \
    --git-repo-id rust-sdk \
    -g rust \
    --additional-properties=packageVersion="$SDK_VERSION" \
    --config ./src/api/openapi-generator-config.json \
    -i ./src/OAS/src/v2/spec.yaml \
    -o ./generated/api \
    --inline-schema-options 'SKIP_SCHEMA_REUSE=true' \
    --type-mappings='"file=Vec<u8>"'

# Generate Hubble SDK
rm -rf ./generated/hub-api/*
npx --yes @openapitools/openapi-generator-cli \
    generate \
    --git-user-id neynarxyz \
    --git-repo-id rust-sdk \
    -g rust \
    --config ./src/hub-api/openapi-generator-config.json \
    -i ./src/OAS/src/hub-rest-api/spec.yaml \
    -o ./generated/hub-api \
    --additional-properties=packageVersion="$SDK_VERSION" \
    --inline-schema-options 'SKIP_SCHEMA_REUSE=true' \
    --type-mappings='"file=Vec<u8>"'

# Build the workspace
cargo fmt
cargo build
# test the workspace
cargo test
