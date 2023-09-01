use std::ops::{Deref, DerefMut};

use crate::{Il2CppResult, Il2CppError};

use super::{api, class::Il2CppClass};

pub type Il2CppArray<T> = Il2CppObject<Array<T>>;

#[repr(C)]
pub struct Il2CppObject<T> {
    pub klass: &'static mut Il2CppClass,
    monitor: *const u8,
    pub fields: T,
}

impl<T> Deref for Il2CppObject<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.fields
    }
}

impl<T> DerefMut for Il2CppObject<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.fields
    }
}

impl<T> AsRef<T> for Il2CppObject<T> {
    fn as_ref(&self) -> &T {
        &self.fields
    }
}

impl<T> AsMut<T> for Il2CppObject<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.fields
    }
}

impl<T> Il2CppObject<T> {
    pub fn get_class(&self) -> &Il2CppClass {
        self.klass
    }

    pub fn get_class_mut(&mut self) -> &mut Il2CppClass {
        self.klass
    }

    pub fn from_class(class: &Il2CppClass) -> Il2CppResult<&'static mut Self> {
        unsafe { api::object_new(class) }.ok_or(Il2CppError::FailedInstantiation(class.get_name()))
    }

    pub fn from_token(token: u64) -> Il2CppResult<&'static mut Self> {
        unsafe {
            let fake_class = &mut *(token as *mut Il2CppClass);
            Self::from_class(fake_class)
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Il2CppArrayBounds {
    length: usize,
    lower_bounds: u32,
}

#[repr(C)]
pub struct Array<T> {
    bounds: &'static Il2CppArrayBounds,
    pub max_length: usize,
    pub m_items: [T; 0],
}

impl<T> Deref for Array<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.m_items.as_ptr(), self.max_length) }
    }
}

impl<T> DerefMut for Array<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.m_items.as_mut_ptr(), self.max_length) }
    }
}

impl<T> Il2CppArray<T> {
    pub fn new(class: &Il2CppClass, capacity: usize) -> Il2CppResult<&'static mut Self> {
        array_new(class, capacity)
    }

    pub fn new_from(class: &Il2CppClass, mut slice: impl AsMut<[T]>) -> Il2CppResult<&'static mut Self> {
        let new_array = array_new(class, slice.as_mut().len())?;
        new_array.swap_with_slice(slice.as_mut());
        Ok(new_array)
    }

    pub fn new_specific(class: &Il2CppClass, capacity: usize) -> Il2CppResult<&'static mut Self> {
        array_new_specific(class, capacity)
    }

    pub fn new_from_token(token: u64, capacity: usize) -> Il2CppResult<&'static mut Self> {
        let temp = token;
        let fake_class = unsafe { &*(temp as *mut Il2CppClass) };
        array_new_specific(fake_class, capacity)
    }

    /// Takes a mutable slice and allocates a new Il2CppArray filled with its content.
    ///
    /// This is partially needed because we do not implement Clone on Il2CppObject.
    pub fn new_specific_from(class: &Il2CppClass, mut slice: impl AsMut<[T]>) -> Il2CppResult<&'static mut Self> {
        let new_array = array_new_specific(class, slice.as_mut().len())?;
        new_array.swap_with_slice(slice.as_mut());
        Ok(new_array)
    }

    /// Create a new Vec filled with the content from the Il2CppArray.
    ///
    /// This is partially needed because we do not implement Clone on Il2CppObject.
    pub fn to_vec(&mut self) -> Vec<T> {
        let mut vec = Vec::with_capacity(self.len());
        unsafe { vec.set_len(self.len()) }
        // TODO: Try to improve this with a memcpy, since we have the same size
        self.swap_with_slice(vec.as_mut());
        vec
    }
}

fn array_new_specific<T>(array_typeinfo: &Il2CppClass, length: usize) -> Il2CppResult<&'static mut Il2CppArray<T>> {
    unsafe { api::array_new_specific(array_typeinfo, length) }.ok_or(Il2CppError::FailedArrayInstantiation)
}

fn array_new<T>(element_typeinfo: &Il2CppClass, length: usize) -> Il2CppResult<&'static mut Il2CppArray<T>> {
    unsafe { api::array_new(element_typeinfo, length) }.ok_or(Il2CppError::FailedArrayInstantiation)
}
