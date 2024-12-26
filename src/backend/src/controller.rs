pub fn is_controller() -> Result<(), String> {
    let caller = ic_cdk::caller();
    if ic_cdk::api::is_controller(&caller) {
        return Ok(());
    } else {
        Err("You are not Authorized to perform this action!".into())
    }
}
