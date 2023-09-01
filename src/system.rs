use std::sync::LazyLock;

use crate::il2cpp::object::{Il2CppArray, Il2CppObject};

pub type Il2CppString = Il2CppObject<SystemString>;

#[repr(C)]
#[crate::class("System", "Type")]
pub struct SystemType;
#[repr(C)]
#[crate::class("System", "Byte")]
pub struct SystemByte;

#[crate::from_offset("System", "RuntimeType", "MakeGenericType")]
pub fn runtime_type_make_generic_type(gt: *const u8, typse: *const u8);

#[repr(C)]
#[derive(Clone)]
pub struct SystemString {
    len: i32,
    pub string: [u16; 0],
}

impl Il2CppString {
    pub fn new(string: impl AsRef<str>) -> &'static Il2CppString {
        let cock = std::ffi::CString::new(string.as_ref()).unwrap();
        unsafe { string_new(cock.as_bytes_with_nul().as_ptr()) }
    }

    pub fn get_string(&self) -> Result<String, std::string::FromUtf16Error> {
        if self.len == 0 {
            Ok(String::new())
        } else {
            unsafe { String::from_utf16(std::slice::from_raw_parts(self.string.as_ptr(), self.len as _)) }
        }
    }
}

impl<T: AsRef<str>> From<T> for &'static Il2CppString {
    fn from(value: T) -> Self {
        Il2CppString::new(value)
    }
}

#[lazysimd::from_pattern("ff 03 01 d1 fd 7b 02 a9 fd 83 00 91 f4 4f 03 a9 f3 03 00 aa ?? ?? ?? ?? 01 7c 40 92 e8 23 00 91 e0 03 13 aa f4 23 00 91 ?? ?? ?? ?? e8 23 40 39 0b fd 41 d3 e9 0f 40 f9")]
fn string_new(c_str: *const u8) -> &'static Il2CppString;

#[repr(C)]
#[crate::class("System.Collections.Generic", "List`1")]
pub struct List<T: 'static> {
    pub items: &'static mut Il2CppArray<&'static mut T>,
    pub size: u32,
    version: u32,
    sync_root: *const u8,
}

impl<T> Il2CppObject<List<T>> {
    pub fn add(&mut self, element: &'static mut T) {
        let cur_size = self.size as usize;

        if cur_size == self.items.max_length {
            self.resize(cur_size * 2);
        }

        self.items[cur_size as usize] = element;
        self.size += 1;
    }

    // Untested
    pub fn resize(&mut self, length: usize) {
        if self.items.len() != length {
            let new_array = crate::il2cpp::object::Il2CppArray::new_specific(self.items.get_class(), length as _).unwrap();
            new_array[..self.items.len()].swap_with_slice(self.items);
            self.items = new_array;
        }
    }
}

#[repr(C)]
pub struct StructBase {
    pub index: i32,
    hash: i32,
    pub key: &'static Il2CppString,
}

impl std::fmt::Debug for StructBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StructBase")
            .field("index", &self.index)
            .field("hash", &self.hash)
            .field("key", &self.key.get_string().unwrap())
            .finish()
    }
}

impl Default for StructBase {
    fn default() -> Self {
        Self {
            index: Default::default(),
            hash: Default::default(),
            key: "".into(),
        }
    }
}
