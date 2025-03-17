# set -euo pipefail
dfx stop && \
    dfx start --clean --background --host 127.0.0.1:4943 && \
    dfx canister create estate_backend

scripts/candid_generator.sh
cp ./src/backend/can.did ../estate-fe-pr-review/ssr/did/backend.did

dfx deploy

# add anonymous as the controller 
dfx canister update-settings --add-controller  "2vxsx-fae" estate_backend
