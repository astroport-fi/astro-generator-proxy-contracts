# Generator Proxy contracts

This repo contains the proxy contracts for 3rd party LP staking incentives.
These are needed for allowing dual incentives on the Astro LP Tokens via the generator contract.

## Development

### Dependencies

- Rust v1.44.1+
- `wasm32-unknown-unknown` target
- Docker
- [LocalTerra](https://github.com/terra-project/LocalTerra)
- Node.js v16

### Envrionment Setup

1. Install `rustup` via https://rustup.rs/

2. Add `wasm32-unknown-unknown` target

```sh
rustup default stable
rustup target add wasm32-unknown-unknown
```

### Compile

Make sure the current working directory is set to the root directory of this repository, then

```bash
cargo build
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.11.3
```

## License

[Apache License 2.0](https://github.com/astroport-fi/generator-proxy-contracts/blob/main/LICENSE)
