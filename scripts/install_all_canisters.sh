set -euo pipefail

usage() {
  printf "Usage: \n[-s Skip test] \n[-h Display help] \n";
  exit 0;
}

skip_test=false

while getopts "sih" arg; do
  case $arg in
    s)
      skip_test=true
      ;;
    h)
      usage
      ;;
  esac
done

CANISTER_NAME=estate_backend

dfx canister create $CANISTER_NAME

gzip_canister() {
  gzip -f -1 ./target/wasm32-unknown-unknown/release/$1.wasm
}

scripts/candid_generator.sh

gzip_canister $CANISTER_NAME

dfx canister install $CANISTER_NAME