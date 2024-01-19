# Soroban Eras Tour NFT

This project was built for my Meridian 2023 talk on building a simple NFT contract on Soroban.

## Getting Started

If you haven't setup your local environment to use Soroban, please go to Soroban's [Setup page](https://soroban.stellar.org/docs/getting-started/setup) and install Rust and Soroban

### Running the contract on Testnet

#### 1. Configure the CLI for Testnet ([official doc](https://soroban.stellar.org/docs/getting-started/setup#configuring-the-cli-for-testnet))

```
soroban config network add --global testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015"
```

#### 2. Configure an identity and fund the account using [Friendbot](https://developers.stellar.org/docs/fundamentals-and-concepts/testnet-and-pubnet#friendbot)

```
soroban config identity generate --global swift
```

Fund the account using Friendbot

```
curl "https://friendbot.stellar.org/?addr=$(soroban config identity address swift)"
```

#### 3. Run `cargo test` to make sure the tests for the contract is passing

```
cargo test
```

#### 4. Build a Soroban contract

```
soroban contract build
```

Once it is successfully built, its `.wasm` file should be outputted in the `target` directory. The `.wasm` file:

- contains the logic of the contract, as well as the contract's specification / interface types
- is the only artifact needed to deploy the contract, share the interface with others, or integration test against the contract

```
target/wasm32-unknown-unknown/release/eras_tour_nft.wasm
```

#### 5. Deploy to Testnet

```
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/eras_tour_nft.wasm \
  --source swift \
  --network testnet
```

this command should output a contract id that starts with `"C..."`

## Acknowledgments

- [ERC-721: Non-Fungible Token Standard](https://eips.ethereum.org/EIPS/eip-721)
- [Soroban's Advanced Tutorials: 'Tokens'](https://soroban.stellar.org/docs/tutorials/tokens): for generic token interface
- [millionlumenhomepage.art](https://github.com/candela-network/millionlumenhomepage.art): I think this is the first nft project built on Soroban. It was so helpful to see what others did first. It also includes frontend portion.
