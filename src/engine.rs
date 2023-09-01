#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

#[repr(C)]
pub struct Vector3<T> {
    x: T,
    y: T,
    z: T,
}