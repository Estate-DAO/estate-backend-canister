### How to do the swap for local-sns?

[see video](./video/register_identity_for_swap.webm)

https://github.com/Estate-DAO/estate-backend-canister/raw/refs/heads/estate_backend_integration/scripts/local-sns-testing/video/register_identity_for_swap.webm

So a local-sns with swap completed is setup.


### Commands to run to get local sns working (with swap completed)

```bash
 export SNS_TESTING_INSTANCE=$(
   docker run -p 8000:8000 -p 8080:8080 -d estate-be-localsns:250117 dfx start --clean
)

while ! docker logs $SNS_TESTING_INSTANCE 2>&1 | grep -m 1 'Replica API running'
    do
        echo "Awaiting local replica ..."
        sleep 3
    done

echo $SNS_TESTING_INSTANCE


docker exec -it $SNS_TESTING_INSTANCE bash 

cd dapp 
bash scripts/local-sns-testing/copy_selectively.sh 

# if possible, copy all the canister did files and wasm files from local
bash setup_locally.sh

### deploy test container + create sns 

./deploy_test_canister.sh  # from Bash
./let_nns_control_dapp.sh  # from Bash
./propose_sns.sh  # from Bash

```