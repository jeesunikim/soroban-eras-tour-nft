#!/bin/bash

set -e

NETWORK="$1"

# If soroban-cli is called inside the soroban-preview docker container,
# it can call the stellar standalone container just using its name "stellar"
if [[ "$IS_USING_DOCKER" == "true" ]]; then
  SOROBAN_RPC_HOST="http://stellar:8000"
else
  SOROBAN_RPC_HOST="http://localhost:8000"
fi

case "$1" in
standalone)
  echo "Using standalone network"
  SOROBAN_NETWORK_PASSPHRASE="Standalone Network ; February 2017"
  FRIENDBOT_URL="$SOROBAN_RPC_HOST/friendbot"
  SOROBAN_RPC_URL="$SOROBAN_RPC_HOST/soroban/rpc"
  ;;
futurenet)
  echo "Using Futurenet network"
  SOROBAN_NETWORK_PASSPHRASE="Test SDF Future Network ; October 2022"
  FRIENDBOT_URL="https://friendbot-futurenet.stellar.org/"
  SOROBAN_RPC_URL="https://rpc-futurenet.stellar.org"
  ;;
testnet)
  echo "Using Testnet network"
  SOROBAN_NETWORK_PASSPHRASE="Test SDF Network ; September 2015"
  FRIENDBOT_URL="https://friendbot.stellar.org/"
  SOROBAN_RPC_URL="https://soroban-testnet.stellar.org"
  ;;
*)
  echo "Usage: $0 standalone|futurenet|testnet"
  exit 1
  ;;
esac


echo Add the $NETWORK network to cli client
soroban config network add \
  --rpc-url "$SOROBAN_RPC_URL" \
  --network-passphrase "$SOROBAN_NETWORK_PASSPHRASE" "$NETWORK"

if !(soroban config identity ls | grep swift 2>&1 >/dev/null); then
  echo Create the swift identity
  soroban config identity generate swift
fi
SWIFT_SECRET="$(soroban config identity show swift)"
SWIFT_ADDRESS="$(soroban config identity address swift)"

# TODO: Remove this once we can use `soroban config identity` from webpack.
mkdir -p .soroban
echo "$SWIFT_SECRET" > .soroban/SWIFT_secret
echo "$SWIFT_ADDRESS" > .soroban/SWIFT_address

# This will fail if the account already exists, but it'll still be fine.
echo Fund swift account from friendbot
curl --silent -X POST "$FRIENDBOT_URL?addr=$SWIFT_ADDRESS" >/dev/null

ARGS="--network $NETWORK --source swift"

echo Build the eras tour nft contract
make build

echo Deploy the eras tour nft contract
ERAS_TOUR_NFT_ID="$(
  soroban contract deploy $ARGS \
    --wasm target/wasm32-unknown-unknown/release/eras_tour_nft.wasm
)"
echo "Contract deployed successfully with ID: $ERAS_TOUR_NFT_ID"
echo "$ERAS_TOUR_NFT_ID" > .soroban/eras-tour-nft-id

echo "Initialize the eras tour nft contract"
deadline="$(($(date +"%s") + 86400))"
soroban contract invoke \
  $ARGS \
  --id "$ERAS_TOUR_NFT_ID" \
  -- \
  initialize \
  --admin "$SWIFT_ADDRESS" \
  --name "eras tour" \
  --symbol "eras"

echo "Done"