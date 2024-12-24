#!/usr/bin/env bash

function generate_did() {
  local canister=$1
  canister_root="src/backend"


  candid-extractor "target/wasm32-unknown-unknown/release/$canister.wasm" > "$canister_root/can.did"
}

# The list of canisters of your project
CANISTERS="estate_backend"

for canister in $(echo $CANISTERS | sed "s/,/ /g")
do
    dfx build $canister
    generate_did "$canister"
done