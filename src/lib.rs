mod tests;
mod structs;
mod r#impl;

use {
    structs::{Talkative}
};

#[no_mangle]
pub unsafe extern "C" fn add_GlobalContext(instance:*mut Talkative ,context:&str) {
    let talkative_instance = &mut *instance;
    talkative_instance.add_GlobalContext(context);
}
