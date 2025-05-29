// pub fn is_controller() -> Result<(), String> {
//     let caller = ic_cdk::caller();
//     if ic_cdk::api::is_controller(&caller) {
//         return Ok(());
//     } else {
//         Err("You are not Authorized to perform this action!".into())
//     }
// }

use candid::Principal;

use crate::STATE;

pub fn is_controller() -> Result<(), String> {
    let caller = ic_cdk::caller();
    if ic_cdk::api::is_controller(&caller) {
        return Ok(());
    }
    STATE.with(|state| {
        if state
            .borrow()
            .controllers
            .as_ref()
            .map(|f| f.contains(&ic_cdk::caller()))
            .unwrap_or(false)
        {
            Ok(())
        } else {
            Err("You are not authorized to perform this action.".to_string())
        }
    })
}

#[ic_cdk_macros::query]
fn get_controllers() -> Vec<Principal> {
    STATE.with(|state| state.borrow().controllers.clone().unwrap_or_default())
}

#[ic_cdk_macros::update(guard = "is_controller")]
pub fn add_controller(new_controller: Principal) -> Result<(), String> {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let controllers = state.controllers.get_or_insert_with(Vec::new);

        if controllers.contains(&new_controller) {
            Err("Controller already exists.".to_string())
        } else {
            controllers.push(new_controller);
            Ok(())
        }
    })
}

#[ic_cdk_macros::update(guard = "is_controller")]
pub fn remove_controller(controller_to_remove: Principal) -> Result<(), String> {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if let Some(index) = state
            .controllers
            .as_ref()
            .unwrap_or(&vec![])
            .iter()
            .position(|x| *x == controller_to_remove)
        {
            // state.controllers.remove(index);
            state.controllers.as_mut().map(|f| f.remove(index));
            Ok(())
        } else {
            Err("Controller not found.".to_string())
        }
    })
}
