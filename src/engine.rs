use crate::prelude::*;

pub mod ui;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vector2<T> {
    x: T,
    y: T,
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vector3<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

#[crate::class("UnityEngine", "Material")]
pub struct Material { }

#[repr(i32)]
pub enum FilterMode {
    Point,
    Bilinear,
    Trilinear,
}

#[crate::class("UnityEngine", "Texture2D")]
pub struct Texture2D { }

impl Texture2D {
    pub fn new(width: i32, height: i32) -> &'static mut Self {
        let new_texture = Texture2D::instantiate().unwrap();
        unsafe { texture2d_ctor(new_texture, width, height, None) };
        new_texture
    }

    pub fn set_filter_mode(&mut self, mode: FilterMode) {
        unsafe { texture2d_set_filter_mode(&self, mode, None) }
    }
}

#[crate::from_offset("UnityEngine", "Texture2D", ".ctor")]
fn texture2d_ctor(this: &Texture2D, width: i32, height: i32, method_info: OptionalMethod);


#[crate::from_offset("UnityEngine", "Texture", "set_filterMode")]
fn texture2d_set_filter_mode(this: &Texture2D, filter_mode: FilterMode, method_info: OptionalMethod);

#[crate::class("UnityEngine", "Sprite")]
pub struct Sprite { }

#[repr(i32)]
pub enum SpriteMeshType {
    FullRect,
    Tight
}

impl Sprite {
    pub fn create2(texture: &Texture2D, rect: Rect, pivot: Vector2<f32>, pixels_to_unit: f32, extrude: u32, mesh_type: SpriteMeshType) -> &'static mut Self {
        unsafe { sprite_create2(texture, rect, pivot, pixels_to_unit, extrude, mesh_type, None) }
    }
}

#[skyline::from_offset(0x2f989c0)]
fn sprite_create2(texture: &Texture2D, rect: Rect, pivot: Vector2<f32>, pixels_to_unit: f32, extrude: u32, mesh_type: SpriteMeshType, method_info: OptionalMethod) -> &'static mut Sprite;


#[crate::class("UnityEngine", "ImageConversion")]
pub struct ImageConversion { }

impl ImageConversion {
    pub fn load_image(texture: &mut Texture2D, data: &Il2CppArray<u8>) -> bool {
		unsafe { imageconversion_load_image(texture, data, None) }
    }
}

#[crate::from_offset("UnityEngine", "ImageConversion", "LoadImage")]
fn imageconversion_load_image(tex: &Texture2D, data: &Il2CppArray<u8>, method_info: OptionalMethod) -> bool;

#[repr(C)]
pub struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }
}