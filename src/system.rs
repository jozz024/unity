use std::{marker::PhantomData, ops::{Deref, DerefMut}};

use crate::prelude::{Il2CppArray, Il2CppClass, Il2CppClassData, Il2CppObject, MethodInfo};

/// A type alias for `Il2CppObject<SystemString>`.
/// 
/// Represents a C# string used by Il2Cpp.
pub type Il2CppString = Il2CppObject<SystemString>;

#[repr(C)]
#[crate::class("System", "Type")]
pub struct SystemType { }

#[repr(C)]
#[crate::class("System", "Byte")]
pub struct SystemByte { }


#[crate::from_offset("System", "RuntimeType", "MakeGenericType")]
pub fn runtime_type_make_generic_type(gt: *const u8, ty: *const u8);

/// Represents a C# String used by Il2Cpp.
/// 
/// It is rarely needed to manipulate this directly.  
/// Prefer using [`Il2CppString`] instead.
#[repr(C)]
#[derive(Clone)]
pub struct SystemString {
    len: i32,
    string: [u16; 0],
}

impl Il2CppClassData for Il2CppString {
    const NAMESPACE: &'static str = "System";
    const CLASS: &'static str = "String";

    fn class() -> &'static Il2CppClass {
        static CLASS_TYPE: std::sync::LazyLock<&'static mut Il2CppClass> = std::sync::LazyLock::new(|| {
            Il2CppClass::from_name("System", "String")
                .expect(&format!("Failed to find class {}.{}", "System", "String"))
        });

        &CLASS_TYPE
    }

    fn class_mut() -> &'static mut Il2CppClass {
        Self::class().clone()
    }
}

#[crate::from_offset("System", "String", "Equals")]
pub fn system_string_equals(a: &Il2CppString, b: &Il2CppString) -> bool;

impl Il2CppString {
    /// Create a new instance of a SystemString using the provided value.
    /// 
    /// Internally turned into a `CString`, so make sure the provided value is a valid UTF-8 string.
    /// 
    /// Example:
    ///
    /// ```
    /// let string = Il2CppString::new("A new string");
    /// ```j
    pub fn new<'a>(string: impl AsRef<str>) -> &'a Il2CppString {
        let cock = std::ffi::CString::new(string.as_ref()).unwrap();
        unsafe { string_new(cock.as_bytes_with_nul().as_ptr()) }
    }

    pub fn new_static(string: impl AsRef<str>) -> &'static mut Il2CppString {
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

impl<T: AsRef<str>> From<T> for &'_ Il2CppString {
    fn from(value: T) -> Self {
        Il2CppString::new(value)
    }
}

impl PartialEq for Il2CppString {
    fn eq(&self, other: &Self) -> bool {
        unsafe { system_string_equals(self, other) }
    }
}

#[lazysimd::from_pattern("ff 03 01 d1 fd 7b 02 a9 fd 83 00 91 f4 4f 03 a9 f3 03 00 aa ?? ?? ?? ?? 01 7c 40 92 e8 23 00 91 e0 03 13 aa f4 23 00 91 ?? ?? ?? ?? e8 23 40 39 0b fd 41 d3 e9 0f 40 f9")]
fn string_new<'a>(c_str: *const u8) -> &'a mut Il2CppString;

/// The Il2Cpp equivalent of a C# List, similar to a Rust Vec.
/// 
/// Internally backed by a [`Il2CppArray`](crate::il2cpp::object::Il2CppArray), this class keeps track of how many entries are in the array.  
/// This means you do not want to directly edit the array unless you also increase the size field.
#[repr(C)]
#[crate::class("System.Collections.Generic", "List`1")]
pub struct List<T: 'static> {
    pub items: &'static mut Il2CppArray<&'static mut T>,
    pub size: u32,
    version: u32,
    sync_root: *const u8,
}

impl<T: 'static> Deref for ListFields<T> {
    type Target = [&'static mut T];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.items.m_items.as_ptr(), self.size as usize) }
    }
}

impl<T: 'static> DerefMut for ListFields<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.items.m_items.as_mut_ptr(), self.size as usize) }

    }
}

impl<T> List<T> {
    pub fn resize(&mut self, length: usize) {
        if self.items.len() != length {
            let new_array = crate::il2cpp::object::Il2CppArray::new_specific(self.items.get_class(), length as _).unwrap();
            new_array[..self.items.len()].swap_with_slice(self.items);
            self.items = new_array;
        }
    }

    pub fn add(&mut self, element: &'static mut T) {
        let method = self.get_class()
            .get_methods()
            .iter()
            .find(|method| method.get_name() == Some(String::from("Add")))
            .unwrap();
        
        let add = unsafe {
            std::mem::transmute::<_, extern "C" fn(&mut Self, &'static mut T, &MethodInfo)>(
                method.method_ptr,
            )
        };

        add(self, element, method);
    }

    pub fn len(&self) -> usize {
        self.size as _
    }

    pub fn capacity(&self) -> usize {
        self.items.len() as _
    }

    pub fn clear(&mut self) {
        self.get_class().get_virtual_method("Clear").map(|method| {
            let clear = unsafe { std::mem::transmute::<_, extern "C" fn(&List<T>, &MethodInfo)>(method.method_info.method_ptr) };
            clear(&self, method.method_info);
        }).unwrap();
    }
}

pub trait ListVirtual<T>: Il2CppClassData {
    fn add(&mut self, element: &'static mut T) {
        let method = Self::class().get_virtual_method("Add").unwrap();
        
        let add = unsafe {
            std::mem::transmute::<_, extern "C" fn(&mut Self, &'static mut T, &MethodInfo)>(
                method.method_info.method_ptr,
            )
        };

        add(self, element, method.method_info);
    }
}

#[crate::class("System.Collections.Generic", "Dictionary`1")]
pub struct Dictionary<TKey, TValue> {
    lol: PhantomData<(TKey, TValue)>
}

impl<TKey, TValue> Dictionary<TKey, TValue> {
    pub fn add(&self, key: TKey, value: TValue) {
        let method = self.get_class()
            .get_virtual_method("Add")
            .unwrap();
        
        let add = unsafe {
            std::mem::transmute::<_, extern "C" fn(&Self, TKey, TValue, &MethodInfo)>(
                method.method_info.method_ptr,
            )
        };

        add(self, key, value, method.method_info);
    }

    pub fn try_get_value<'a>(&self, key: TKey, value: &'a mut TValue) -> bool {
        let method = self.get_class()
            .get_virtual_method("TryGetValue")
            .unwrap();
        
        let try_get_value = unsafe {
            std::mem::transmute::<_, extern "C" fn(&Self, TKey, &mut TValue, &MethodInfo) -> bool>(
                method.method_info.method_ptr,
            )
        };

        try_get_value(self, key, value, method.method_info)
    }
}
