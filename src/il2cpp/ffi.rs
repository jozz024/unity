use super::api::*;

// Expose these methods for plugins to use, since Switch il2cpp games do not come with il2cpp symbols

#[no_mangle]
pub extern "C" fn il2cpp_init(domain_name: *const i8) -> i32 {
    unsafe { init(domain_name) }
}
