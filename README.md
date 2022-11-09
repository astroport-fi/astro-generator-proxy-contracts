# Astroport Generator Proxy Contracts

[![codecov](https://codecov.io/gh/astroport-fi/astro-generator-proxy-contracts/branch/release/graph/badge.svg?token=ZCO1D3AGSM)](https://codecov.io/gh/astroport-fi/astro-generator-proxy-contracts)

This repo contains the proxy contracts for 3rd party LP staking incentives.
These are needed for allowing dual incentives on the Astro LP Tokens via the generator contract.

## Contracts

| Name                           | Description                      |
| ------------------------------ | -------------------------------- |
| [`proxy_to_vkr`](contracts/proxy_to_vkr) | Generator Proxy to Valkyrie Protocol |

## Building Contracts

You will need Rust 1.64.0+ with wasm32-unknown-unknown target installed.

### You can compile each contract:
Go to contract directory and run 
    
```
cargo wasm
cp ../../target/wasm32-unknown-unknown/release/astroport_token.wasm .
ls -l astroport_token.wasm
sha256sum astroport_token.wasm
```

### You can run tests for all contracts
Run the following from the repository root

```
cargo test
```

### For a production-ready (compressed) build:
Run the following from the repository root

```
./scripts/build_release.sh
```

The optimized contracts are generated in the artifacts/ directory.

## Branches

We use [main](https://github.com/astroport-fi/astro-generator-proxy-contracts/tree/main) branch for new feature development and [release](https://github.com/astroport-fi/astro-generator-proxy-contracts/tree/release) one for collecting features which are ready for deployment. You can find the version and commit for actually deployed contracts [here](https://github.com/astroport-fi/astroport-changelog).

## Docs

Docs can be generated using `cargo doc --no-deps`

