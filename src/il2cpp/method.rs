use std::ffi::CStr;

use crate::{Il2CppError, Il2CppResult};

use super::{class::Il2CppClass, Il2CppReflectionType, Il2CppType};

pub type OptionalMethod = Option<&'static MethodInfo>;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MethodInfo {
    pub method_ptr: *mut u8,
    pub invoker_method: *const u8,
    pub name: *const u8,
    pub class: Option<&'static Il2CppClass>,
    pub return_type: *const u8,
    pub parameters: *const ParameterInfo,
    pub info_or_definition: *const u8,
    pub generic_method_or_container: *const u8,
    pub token: u32,
    pub flags: u16,
    pub iflags: u16,
    pub slot: u16,
    pub parameters_count: u8,
    pub bitflags: u8,
}

unsafe impl Send for MethodInfo {}
unsafe impl Sync for MethodInfo {}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ParameterInfo {
    pub name: *const u8,
    pub position: i32,
    pub token: u32,
    pub parameter_type: &'static Il2CppType,
}

impl MethodInfo {
    pub fn new() -> Self {
        Self {
            method_ptr: 0 as _,
            invoker_method: 0 as _,
            name: 0 as _,
            class: None,
            return_type: 0 as _,
            parameters: 0 as _,
            info_or_definition: 0 as _,
            generic_method_or_container: 0 as _,
            bitflags: 0,
            flags: 0,
            iflags: 0,
            parameters_count: 0,
            slot: 0,
            token: 0,
        }
    }

    pub fn new_from(base: Self) -> Self {
        Self {
            invoker_method: 0 as _,
            bitflags: 0,
            flags: 0,
            iflags: 0,
            slot: 0,
            token: 0,
            ..base
        }
    }
}

impl MethodInfo {
    pub fn get_name(&self) -> Option<String> {
        if self.name.is_null() {
            None
        } else {
            Some(unsafe { String::from_utf8_lossy(CStr::from_ptr(self.name as _).to_bytes()).to_string() })
        }
    }

    pub fn get_parameters(&self) -> &[ParameterInfo] {
        unsafe { std::slice::from_raw_parts(self.parameters, self.parameters_count as _) }
    }

    pub fn invoke(&self, obj: *const u8, params: *const u8) -> Il2CppResult<&'static mut Il2CppReflectionType<()>> {
        let runtime_invoke = unsafe {
            std::mem::transmute::<_, extern "C" fn(*const u8, &MethodInfo, *const u8, *const u8) -> Option<&'static mut Il2CppReflectionType<()>>>(
                self.invoker_method,
            )
        };
        runtime_invoke(self.method_ptr, self, obj, params).ok_or(Il2CppError::FailedMethodInvocation)
    }
}

impl ParameterInfo {
    pub fn get_name(&self) -> Option<String> {
        if self.name.is_null() {
            None
        } else {
            Some(unsafe { String::from_utf8_lossy(CStr::from_ptr(self.name as _).to_bytes()).to_string() })
        }
    }
}
