use std::ffi::CStr;

use super::api;
use crate::cppvector::CppVector;

#[repr(C)]
pub struct Il2CppImage {
    pub name: *const u8,
    name_no_ext: *const u8,
    assembly: &'static Il2CppAssembly,
    // ...
}

impl Il2CppImage {
    pub fn get_name(&self) -> String {
        unsafe { String::from_utf8_lossy(CStr::from_ptr(self.name as _).to_bytes()).to_string() }
    }
}

/// Represents a C# assembly that has been converted by Il2Cpp
#[repr(C)]
pub struct Il2CppAssembly {
    pub image: &'static Il2CppImage,
    pub token: u32,
    referenced_assembly_start: i32,
    referenced_assembly_count: i32,
    // ...
}

pub fn get_assemblies() -> &'static CppVector<&'static Il2CppAssembly> {
    unsafe { api::assembly_getallassemblies() }
}
