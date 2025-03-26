### How to do the swap for local-sns?

[see video](./video/register_identity_for_swap.webm)

https://github.com/Estate-DAO/estate-backend-canister/raw/refs/heads/estate_backend_integration/scripts/local-sns-testing/video/register_identity_for_swap.webm

So a local-sns with swap completed is setup.

### Note: 

this is the current docker image. 
`estate-be-localsns:250117`

Ensure that you build the latest docker image from 'before' the code changes so that upgrade dapp can be tested.

it is recommended to use a date as a tag so that the previous build remains locally avaiable.

```bash
docker build -t estate-be-localsns:250325 -f scripts/local-sns-testing/Dockerfile-local-sns-test .


```

## Commands to run to get local sns working (with swap completed)

#### Start the local sns inside docker. 

This is to ensure that the local file system is isolated.

```bash

export LAST_DOCKER_BUILD_TAG="250117"

export SNS_TESTING_INSTANCE=$(
   docker run -p 8000:8000 -p 8080:8080 -d estate-be-localsns:$LAST_DOCKER_BUILD_TAG dfx start --clean
)

while ! docker logs $SNS_TESTING_INSTANCE 2>&1 | grep -m 1 'Replica API running'
    do
        echo "Awaiting local replica ..."
        sleep 3
    done

echo $SNS_TESTING_INSTANCE


docker exec -it $SNS_TESTING_INSTANCE bash 

cd /dapp 
bash scripts/local-sns-testing/copy_selectively.sh 

# set our custom example_sns_init so that we can have 
# minimum_participants = 2
# maximum_direct_participation_icp = 3
# this makes it easier to participate_sns_swap.sh
cp example_sns_init.yaml example_sns_init.yaml.bak 

# sed -i 's/^\([[:space:]]*minimum_participants: \)57$/\12/' example_sns_init.yaml
sed -i -E 's/^([[:space:]]*minimum_participants: )[0-9]+$/\12/; s/^([[:space:]]*maximum_direct_participation_icp: )[0-9_]+/\13000/' example_sns_init.yaml

cat example_sns_init.yaml | grep minimum_participants
cat example_sns_init.yaml | grep maximum_direct_participation_icp

# if possible, copy all the canister did files and wasm files from local
bash setup_locally.sh


#  THERE ARE FOUR STEPS NOW.

# 1. ./deploy_test_canister.sh  # from Bash
# 2. ./let_nns_control_dapp.sh  # from Bash
# 3. ./propose_sns.sh  # from Bash

# 4. copy code and raise proposal for upgrade

# WAIT FOR 5 MIN FOR IT TO APPEAR IN LOCAL NNS DASHBOARD
# since there are only min_participants = 2, do it from dashboard to get swap tokens. 
# they will be useful to vote on proposals later.
# ./participate_sns_swap.sh


```

#### step 1 - (works) - build (estate_backend) canister and install on local network

```bash 

# enable debugging
set -x

# cd 
cd /dapp 

. ./constants.sh normal

export DFX_BINARY="dfx"
export NETWORK="local"

export NAME="estate_backend"
# export WASM=".dfx/local/canisters/$NAME/${NAME}"
export WASM=""
export ARG="()"


# the command below does the following:
# adds ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓  to dfx.json
#  "estate_backend": {
#       "candid": "src/backend/can.did",
#       "package": "estate_backend",
#       "type": "rust"
#   }
# ↑ ↑ ↑ ↑ ↑ ↑ ↑ ↑ ↑ ↑ ↑ ↑ ↑ ↑ 

yq -i '.canisters.estate_backend = {"candid": "src/backend/can.did", "package": "estate_backend", "type": "rust"}' dfx.json
# verify that estate_backend exists in dfx.json
yq '.canisters.estate_backend' dfx.json

export MY_CANISTER_ID="ajuq4-ruaaa-aaaaa-qaaga-cai"

"${DFX_BINARY}" \
    canister create "${NAME}" \
    --specified-id "${MY_CANISTER_ID}" \
    --network "${NETWORK}" \
    --no-wallet


# might have to remove cargo.lock and regenerate it 
rm Cargo.lock 
cargo generate-lockfile


if [[ -z "${WASM}" ]]
then
  WASM=".dfx/${NETWORK}/canisters/${NAME}/${NAME}"
  rm -f "${WASM}-s.wasm.gz"
  "${DFX_BINARY}" build --network "${NETWORK}" "${NAME}"
  ic-wasm "${WASM}.wasm" -o "${WASM}-s.wasm" shrink
  gzip "${WASM}-s.wasm"
  export WASM="${WASM}-s.wasm.gz"
fi

"${DFX_BINARY}" canister install "${NAME}" --network "${NETWORK}" --argument "${ARG}" --argument-type idl --wasm "${WASM}"

# if this returns error DestinationInvalid , run 
# cd /dapp
# rm -rf .dfx 

set +x
```



#### step 2 - let nns control dapp (estate_backend)

```bash

set -x 

cd /dapp
. ./constants.sh normal

export CURRENT_DX_IDENT="$(dfx identity whoami)"

dfx identity use "${DX_IDENT}"


# which identity has which principal 
# dfx canister update-settings --network local --wallet "$(dfx identity get-wallet)" --all --add-controller "$(dfx identity get-principal)"

SNS_CONFIGURATION_FILE_PATH=""
# SNS_CONFIGURATION_FILE_PATH=sns_init.yaml

PRINCIPAL_ID="$(dfx identity get-principal)"
CANISTER_ID="$(dfx canister id estate_backend)"


if [[ -z $SNS_CONFIGURATION_FILE_PATH ]]
then
    SNS_CONFIGURATION_FILE_PATH=sns_init.yaml

    # Write sns_init.yaml, but only if it doesn't exist already (do not clobber).
    if [[ ! -e "$SNS_CONFIGURATION_FILE_PATH" ]]
    then
        PRINCIPAL_ID="$(dfx identity get-principal)"
        CANISTER_ID="$(dfx canister id estate_backend)"
        cat example_sns_init.yaml \
            | sed "s/YOUR_PRINCIPAL_ID/${PRINCIPAL_ID}/" \
            | sed "s/YOUR_CANISTER_ID/${CANISTER_ID}/" \
            | sed 's/  # propose_sns[.]sh .*//' \
            > "$SNS_CONFIGURATION_FILE_PATH"
    fi
fi


sns prepare-canisters add-nns-root $CANISTER_ID

set +x
```


### step 3 -  propose sns 

```bash
set -x 

cd /dapp 

. ./constants.sh normal

export SNS_CONFIGURATION_FILE_PATH="sns_init.yaml"

./propose_sns.sh

set +x
```


### step 4 - login in NNS Frontend app and vote on proposal 

Do this step manually.


### debugging 

```bash 
cd /dapp 
. ./constants.sh normal


export SNS_ROOT_CANISTER_ID=$(jq -r '.root_canister_id' sns_canister_ids.json)
export DFX_CALL="dfx canister --network local call"

${DFX_CALL} $SNS_ROOT_CANISTER_ID list_sns_canisters '(record {})'

# ${DFX_CALL} $SNS_GOVERNANCE_CANISTER_ID get_proposal '(record {proposal_id = opt record {id=1:nat64}})'

```


### step 5. -  copy the src folder from local file system to docker.

docker build was done with previous commit, so the local build in your system should have old code.

```bash
docker cp ./src/backend/src/ $SNS_TESTING_INSTANCE:/dapp/src/backend/
```

### step 6. - upgrade_dapp - estate_backend

```bash 

set -x 

cd /dapp 

. ./constants.sh normal
REPODIR="$(pwd)"

# export NAME="${1:-estate_backend}"
# export WASM="${2:-}"
# export ARG="${3:-()}"
export NAME="estate_backend"
export WASM=""
export ARG="()"


. ./constants.sh normal

export DEVELOPER_NEURON_ID="$(dfx canister \
  --network "${NETWORK}" \
  call "${SNS_GOVERNANCE_CANISTER_ID}" \
  list_neurons "(record {of_principal = opt principal\"${DX_PRINCIPAL}\"; limit = 1})" \
    | idl2json \
    | jq -r ".neurons[0].id[0].id" \
    | python3 -c "import sys; ints=sys.stdin.readlines(); sys.stdout.write(bytearray(eval(''.join(ints))).hex())")"


if [ -f "${ARG}" ]
then
  ARGFLAG="--canister-upgrade-arg-path"
else
  ARGFLAG="--canister-upgrade-arg"
fi

if [[ -z "${WASM}" ]]
then
  WASM=".dfx/${DX_NETWORK}/canisters/${NAME}/${NAME}"
  rm -f "${WASM}-s.wasm.gz"
  dfx build --network "${NETWORK}" "${NAME}"
  ic-wasm "${WASM}.wasm" -o "${WASM}-s.wasm" shrink
  gzip "${WASM}-s.wasm"
  export WASM="${WASM}-s.wasm.gz"
fi

export CID="$( dfx canister --network "${NETWORK}" id "${NAME}")"

# identity 
dfx identity export default > actions_identity.pem
export PEM_FILE="actions_identity.pem"


 quill sns \
   --canister-ids-file "${REPODIR}/sns_canister_ids.json" \
   --pem-file "${PEM_FILE}" \
   make-upgrade-canister-proposal \
   --target-canister-id "${CID}" \
   --mode upgrade \
   --wasm-path "${WASM}" \
   "${ARGFLAG}" "${ARG}" \
   "${DEVELOPER_NEURON_ID}" > msg.json

quill send \
  --insecure-local-dev-mode \
  --yes msg.json | grep -v "new_canister_wasm"


```

#### Step 7. go and vote on proposal in NNS Frontend Dapp


