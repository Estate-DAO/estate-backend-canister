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

#[ic_cdk_macros::query(guard = "is_controller")]
fn get_controllers() -> Vec<Principal> {
    STATE.with(|state| state.borrow().controllers.clone().unwrap_or_default())
}

#[ic_cdk_macros::update(guard = "is_controller")]
pub fn add_controller(new_controller: Principal) -> Result<(), String> {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if !state
            .controllers
            .as_ref()
            .map(|f| f.contains(&new_controller))
            .unwrap_or(false)
        {
            state.controllers.as_mut().map(|f| f.push(new_controller));
            Ok(())
        } else {
            Err("Controller already exists.".to_string())
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
