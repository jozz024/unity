pub mod universal {
    #[crate::class("UnityEngine.Rendering.Universal", "UniversalRenderPipelineAsset")]
    pub struct UniversalRenderPipelineAsset {
        pub zuper: *const u8,
        pub default_shader: *const u8,
        pub renderers: *const u8,
        pub asset_version: i32,
        pub asset_previous_version: i32,
        pub renderer_type: i32,
        pub renderer_data: *const u8,
        pub renderer_data_list: *const u8,
        pub default_renderer_index: i32,
        pub require_depth_texture: bool,
        pub require_opaque_texture: bool,
        pub opaque_downsampling: i32,
        pub supports_terrain_holes: bool,
        pub supports_hdr: bool,
        pub msaa: i32,
        pub render_scale: f32,
    }
}