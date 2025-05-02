#!/bin/bash
set -eux -o pipefail

# Generate Neynar SDK
rm -rf ./generated/api/*
npx --yes @openapitools/openapi-generator-cli \
    generate \
    --git-user-id neynarxyz \
    --git-repo-id rust-sdk \
    -g rust \
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
    --inline-schema-options 'SKIP_SCHEMA_REUSE=true' \
    --type-mappings='"file=Vec<u8>"'

# Fix generated code if needed
sed -i '' 's/models::models/models/g' ./generated/api/src/apis/feed_api.rs