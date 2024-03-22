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

    // This method actually belongs to UnityEngine.UI.Graphic. It's a property stter
    pub fn set_color(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        self.get_class()
            .get_virtual_method("set_color")
            .map(|method| {
                let set_color = unsafe {
                    std::mem::transmute::<_, extern "C" fn(f32, f32, f32, f32, &Image, &MethodInfo)>(method.method_info.method_ptr)
                };
                set_color(red, green, blue, alpha, self, method.method_info);
                // #B00B69
            })
            .unwrap();
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
