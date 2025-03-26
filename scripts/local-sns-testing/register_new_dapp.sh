
. ./constants.sh normal

quill sns --canister-ids-file ./sns_canister_ids.json --pem-file $PEM_FILE make-proposal $PROPOSAL_NEURON_ID --proposal '(
    record {
        title = "Register new dapp canisters";
        url = "https://sns-examples.com/proposal/42";
        summary = "Proposal to register two new dapp canisters, with ID ltyfs-qiaaa-aaaak-aan3a-cai and ltyfs-qiaaa-aaaak-aan3a-cai to the SNS.";
        action = opt variant {
            RegisterDappCanisters = record {

                canister_ids = vec {principal "ltyfs-qiaaa-aaaak-aan3a-cai", principal "ltyfs-qiaaa-aaaak-aan3a-cai"};

            };
        };
    }
)' > message.json

quill send message.json