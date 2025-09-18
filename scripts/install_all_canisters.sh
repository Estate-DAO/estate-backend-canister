# set -euo pipefail
dfx stop && \
    dfx start --clean --background --host 127.0.0.1:4943 && \
    dfx canister create estate_backend

scripts/candid_generator.sh
cp ./src/backend/can.did ../estate-dao-platform-leptos-ssr/ssr/did/backend.did

dfx deploy

# add anonymous as the controller 
dfx canister update-settings --add-controller  "2vxsx-fae" estate_backend

# agent principal from estate-fe repo
dfx canister update-settings --add-controller  "4x6v7-w2npu-2jo7b-zzegd-ru6zk-zat64-mqkl5-nqea7-jy2xj-odvyc-xqe" estate_backend


bash ./scripts/seed_data.sh