#![feature(lazy_cell, ptr_sub_ptr)]

pub use unity_macro::*;

pub mod cppvector;
pub mod engine;
pub mod il2cpp;
pub mod system;
pub mod app;

#[macro_export]
macro_rules! size_of {
    ($ty:tt) => {
        std::mem::size_of::<$ty>()
    };
}

extern crate memoffset;

use thiserror::Error;

pub type Il2CppResult<T> = Result<T, Il2CppError>;

#[derive(Debug, Error)]
pub enum Il2CppError {
    #[error("could not find the class `{0}`")]
    MissingClass(String),
    #[error("could not find the class for the Il2CppType")]
    MissingClassForType,
    #[error("could not find the method")]
    MissingMethod,
    #[error("could not instantiate the class `{0}`")]
    FailedInstantiation(String),
    #[error("could not instantiate the array")]
    FailedArrayInstantiation,
    #[error("could not invoke the method")]
    FailedMethodInvocation,
    #[error("could not get a ReflectionType for the type")]
    FailedReflectionQuerying,
}

pub mod prelude {
    pub use crate::{
        get_generic_class,
        il2cpp,
        Il2CppResult,
        Il2CppError,
        il2cpp::{
            class::{Il2CppClass, Il2CppClassData},
            method::{MethodInfo, OptionalMethod},
            object::{Il2CppArray, Il2CppObject},
        },
        system::Il2CppString,
    };
}
