use crate::prelude::*;

use super::{Sprite, Material};

#[crate::class("UnityEngine.UI", "Image")]
pub struct Image { 
    parent: [u8; 0xc8],
    sprite: &'static Sprite,
    override_sprite: &'static Sprite,
    parent2: [u8; 0x28],
}

impl Image {
    pub fn get_default_graphic_material() -> &'static Material {
        unsafe { image_get_default_graphic_material(None) }
    }
}

#[crate::from_offset("UnityEngine.UI", "Image", "set_sprite")]
fn image_set_sprite<I: IsImage + ?Sized>(this: &mut I, value: &Sprite, method_info: OptionalMethod);

#[crate::from_offset("UnityEngine.UI", "Image", "get_defaultGraphicMaterial")]
fn image_get_default_graphic_material(method_info: OptionalMethod) -> &'static Material;

/// Marker trait for anything that is or inherits from Image
pub trait IsImage {
    fn set_sprite(&mut self, sprite: &'static Sprite) {
        unsafe { image_set_sprite(self, sprite, None) };
    }
}


impl IsImage for Image { }