# Generator Proxy Contracts

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

3. Install Node libraries required:

```bash
cd scripts
npm install
```

4. Terra pheonix network MAINNET details -
   export CHAIN_ID="phoenix-1"
   export LCD_CLIENT_URL="https://phoenix-lcd.terra.dev"

5. Terra pheonix network TESTNET details -
   export CHAIN_ID="pisco-1"
   export LCD_CLIENT_URL="https://pisco-lcd.terra.dev"

6. Deploy:

```bash
export WALLET="<mnemonic seed>"
node --experimental-json-modules --loader ts-node/esm deploy.ts
```

### Compile

Make sure the current working directory is set to the root directory of this repository, then

```bash
cargo build
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.12.6
```

## License

[GPL v3.0](https://github.com/astroport-fi/generator-proxy-contracts/blob/main/LICENSE)
