use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
#[doc(alias = "VkPhysicalDevicePortabilitySubsetFeaturesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePortabilitySubsetFeaturesKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "constantAlphaColorBlendFactors")]
    constant_alpha_color_blend_factors: Bool32,
    events: Bool32,
    #[doc(alias = "imageViewFormatReinterpretation")]
    image_view_format_reinterpretation: Bool32,
    #[doc(alias = "imageViewFormatSwizzle")]
    image_view_format_swizzle: Bool32,
    #[doc(alias = "imageView2DOn3DImage")]
    image_view2_d_on3_d_image: Bool32,
    #[doc(alias = "multisampleArrayImage")]
    multisample_array_image: Bool32,
    #[doc(alias = "mutableComparisonSamplers")]
    mutable_comparison_samplers: Bool32,
    #[doc(alias = "pointPolygons")]
    point_polygons: Bool32,
    #[doc(alias = "samplerMipLodBias")]
    sampler_mip_lod_bias: Bool32,
    #[doc(alias = "separateStencilMaskRef")]
    separate_stencil_mask_ref: Bool32,
    #[doc(alias = "shaderSampleRateInterpolationFunctions")]
    shader_sample_rate_interpolation_functions: Bool32,
    #[doc(alias = "tessellationIsolines")]
    tessellation_isolines: Bool32,
    #[doc(alias = "tessellationPointMode")]
    tessellation_point_mode: Bool32,
    #[doc(alias = "triangleFans")]
    triangle_fans: Bool32,
    #[doc(alias = "vertexAttributeAccessBeyondStride")]
    vertex_attribute_access_beyond_stride: Bool32,
}
#[doc(alias = "VkPhysicalDevicePortabilitySubsetPropertiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePortabilitySubsetPropertiesKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "minVertexInputBindingStrideAlignment")]
    min_vertex_input_binding_stride_alignment: u32,
}
#[doc(alias = "VK_KHR_PORTABILITY_SUBSET_SPEC_VERSION")]
pub const KHR_PORTABILITY_SUBSET_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_PORTABILITY_SUBSET_EXTENSION_NAME")]
pub const KHR_PORTABILITY_SUBSET_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_portability_subset");
