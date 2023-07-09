use crate::native::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDevicePortabilitySubsetFeaturesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePortabilitySubsetFeaturesKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "constantAlphaColorBlendFactors")]
    pub constant_alpha_color_blend_factors: Bool32,
    pub events: Bool32,
    #[doc(alias = "imageViewFormatReinterpretation")]
    pub image_view_format_reinterpretation: Bool32,
    #[doc(alias = "imageViewFormatSwizzle")]
    pub image_view_format_swizzle: Bool32,
    #[doc(alias = "imageView2DOn3DImage")]
    pub image_view2_d_on3_d_image: Bool32,
    #[doc(alias = "multisampleArrayImage")]
    pub multisample_array_image: Bool32,
    #[doc(alias = "mutableComparisonSamplers")]
    pub mutable_comparison_samplers: Bool32,
    #[doc(alias = "pointPolygons")]
    pub point_polygons: Bool32,
    #[doc(alias = "samplerMipLodBias")]
    pub sampler_mip_lod_bias: Bool32,
    #[doc(alias = "separateStencilMaskRef")]
    pub separate_stencil_mask_ref: Bool32,
    #[doc(alias = "shaderSampleRateInterpolationFunctions")]
    pub shader_sample_rate_interpolation_functions: Bool32,
    #[doc(alias = "tessellationIsolines")]
    pub tessellation_isolines: Bool32,
    #[doc(alias = "tessellationPointMode")]
    pub tessellation_point_mode: Bool32,
    #[doc(alias = "triangleFans")]
    pub triangle_fans: Bool32,
    #[doc(alias = "vertexAttributeAccessBeyondStride")]
    pub vertex_attribute_access_beyond_stride: Bool32,
}
impl Default for PhysicalDevicePortabilitySubsetFeaturesKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDevicePortabilitySubsetFeaturesKhr,
            p_next: unsafe { std::mem::zeroed() },
            constant_alpha_color_blend_factors: unsafe { std::mem::zeroed() },
            events: unsafe { std::mem::zeroed() },
            image_view_format_reinterpretation: unsafe { std::mem::zeroed() },
            image_view_format_swizzle: unsafe { std::mem::zeroed() },
            image_view2_d_on3_d_image: unsafe { std::mem::zeroed() },
            multisample_array_image: unsafe { std::mem::zeroed() },
            mutable_comparison_samplers: unsafe { std::mem::zeroed() },
            point_polygons: unsafe { std::mem::zeroed() },
            sampler_mip_lod_bias: unsafe { std::mem::zeroed() },
            separate_stencil_mask_ref: unsafe { std::mem::zeroed() },
            shader_sample_rate_interpolation_functions: unsafe { std::mem::zeroed() },
            tessellation_isolines: unsafe { std::mem::zeroed() },
            tessellation_point_mode: unsafe { std::mem::zeroed() },
            triangle_fans: unsafe { std::mem::zeroed() },
            vertex_attribute_access_beyond_stride: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDevicePortabilitySubsetPropertiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePortabilitySubsetPropertiesKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "minVertexInputBindingStrideAlignment")]
    pub min_vertex_input_binding_stride_alignment: u32,
}
impl Default for PhysicalDevicePortabilitySubsetPropertiesKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDevicePortabilitySubsetPropertiesKhr,
            p_next: unsafe { std::mem::zeroed() },
            min_vertex_input_binding_stride_alignment: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::khr_portability_subset::{
    KHR_PORTABILITY_SUBSET_EXTENSION_NAME, KHR_PORTABILITY_SUBSET_SPEC_VERSION,
};
