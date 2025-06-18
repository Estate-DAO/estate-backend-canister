use ciborium::de;
use ic_cdk::api::call::ArgDecoderConfig;
use ic_cdk::storage;
use ic_cdk_macros::post_upgrade;
use ic_stable_structures::reader::Reader;
use std::borrow::BorrowMut;

use crate::memory;

use crate::CanisterState;
use crate::STATE as CANISTER_DATA;

// #[post_upgrade]
// fn post_upgrade() {
//     restore_data_from_stable_memory();
//     // save_upgrade_args_to_memory();
// }

// fn restore_data_from_stable_memory() {
//     let heap_data = memory::get_upgrades_memory();
//     let mut upgrade_reader = Reader::new(&heap_data, 0);

//     let mut heap_data_len_bytes = [0; 4];
//     upgrade_reader.read(&mut heap_data_len_bytes).unwrap();
//     let heap_data_len = u32::from_le_bytes(heap_data_len_bytes) as usize;

//     let mut canister_data_bytes = vec![0; heap_data_len];
//     upgrade_reader.read(&mut canister_data_bytes).unwrap();

//     let canister_data =
//         de::from_reader(&*canister_data_bytes).expect("Failed to deserialize heap data");

//     CANISTER_DATA.with_borrow_mut(|cdata| {
//         *cdata = canister_data;
//     });
// }
 

 

#[post_upgrade]
fn post_upgrade() {
    let state: Result<(CanisterState,), _> = storage::stable_restore();
    match state {
        Ok((restored_state,)) => {
            CANISTER_DATA.with(|state| {
                *state.borrow_mut() = restored_state;
            });
        }
        Err(err) => {
            ic_cdk::trap(&format!("Failed to restore stable state: {}", err));
        }
    }
}
 