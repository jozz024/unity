pub mod universal {
    #[crate::class("UnityEngine.Rendering.Universal", "UniversalRenderPipelineAsset")]
    pub struct UniversalRenderPipelineAsset {
        zuper: *const u8,
        default_shader: *const u8,
        renderers: *const u8,
        asset_version: i32,
        asset_previous_version: i32,
        renderer_type: i32,
        renderer_data: *const u8,
        renderer_data_list: *const u8,
        default_renderer_index: i32,
        require_depth_texture: bool,
        require_opaque_texture: bool,
        opaque_downsampling: i32,
        supports_terrain_holes: bool,
        supports_hdr: bool,
        msaa: i32,
        render_scale: f32,
    }
}