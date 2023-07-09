pub use crate::common::extensions::nv_shading_rate_image::CoarseSampleLocationNV;
use crate::native::vulkan1_0::{
    BaseInStructure, BaseOutStructure, Bool32, CommandBuffer, Extent2D, ImageLayout, ImageView, StructureType,
};
#[doc(alias = "VkShadingRatePaletteNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ShadingRatePaletteNV {
    #[doc(alias = "shadingRatePaletteEntryCount")]
    pub shading_rate_palette_entry_count: u32,
    #[doc(alias = "pShadingRatePaletteEntries")]
    pub shading_rate_palette_entries: *const ShadingRatePaletteEntryNV,
}
impl Default for ShadingRatePaletteNV {
    fn default() -> Self {
        Self {
            shading_rate_palette_entry_count: unsafe { std::mem::zeroed() },
            shading_rate_palette_entries: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineViewportShadingRateImageStateCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineViewportShadingRateImageStateCreateInfoNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "shadingRateImageEnable")]
    pub shading_rate_image_enable: Bool32,
    #[doc(alias = "viewportCount")]
    pub viewport_count: u32,
    #[doc(alias = "pShadingRatePalettes")]
    pub shading_rate_palettes: *const ShadingRatePaletteNV,
}
impl Default for PipelineViewportShadingRateImageStateCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineViewportShadingRateImageStateCreateInfoNv,
            p_next: unsafe { std::mem::zeroed() },
            shading_rate_image_enable: unsafe { std::mem::zeroed() },
            viewport_count: unsafe { std::mem::zeroed() },
            shading_rate_palettes: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceShadingRateImageFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShadingRateImageFeaturesNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "shadingRateImage")]
    pub shading_rate_image: Bool32,
    #[doc(alias = "shadingRateCoarseSampleOrder")]
    pub shading_rate_coarse_sample_order: Bool32,
}
impl Default for PhysicalDeviceShadingRateImageFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceShadingRateImageFeaturesNv,
            p_next: unsafe { std::mem::zeroed() },
            shading_rate_image: unsafe { std::mem::zeroed() },
            shading_rate_coarse_sample_order: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceShadingRateImagePropertiesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShadingRateImagePropertiesNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "shadingRateTexelSize")]
    pub shading_rate_texel_size: Extent2D,
    #[doc(alias = "shadingRatePaletteSize")]
    pub shading_rate_palette_size: u32,
    #[doc(alias = "shadingRateMaxCoarseSamples")]
    pub shading_rate_max_coarse_samples: u32,
}
impl Default for PhysicalDeviceShadingRateImagePropertiesNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceShadingRateImagePropertiesNv,
            p_next: unsafe { std::mem::zeroed() },
            shading_rate_texel_size: unsafe { std::mem::zeroed() },
            shading_rate_palette_size: unsafe { std::mem::zeroed() },
            shading_rate_max_coarse_samples: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkCoarseSampleOrderCustomNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CoarseSampleOrderCustomNV {
    #[doc(alias = "shadingRate")]
    pub shading_rate: ShadingRatePaletteEntryNV,
    #[doc(alias = "sampleCount")]
    pub sample_count: u32,
    #[doc(alias = "sampleLocationCount")]
    pub sample_location_count: u32,
    #[doc(alias = "pSampleLocations")]
    pub sample_locations: *const CoarseSampleLocationNV,
}
impl Default for CoarseSampleOrderCustomNV {
    fn default() -> Self {
        Self {
            shading_rate: unsafe { std::mem::zeroed() },
            sample_count: unsafe { std::mem::zeroed() },
            sample_location_count: unsafe { std::mem::zeroed() },
            sample_locations: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineViewportCoarseSampleOrderStateCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineViewportCoarseSampleOrderStateCreateInfoNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "sampleOrderType")]
    pub sample_order_type: CoarseSampleOrderTypeNV,
    #[doc(alias = "customSampleOrderCount")]
    pub custom_sample_order_count: u32,
    #[doc(alias = "pCustomSampleOrders")]
    pub custom_sample_orders: *const CoarseSampleOrderCustomNV,
}
impl Default for PipelineViewportCoarseSampleOrderStateCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineViewportCoarseSampleOrderStateCreateInfoNv,
            p_next: unsafe { std::mem::zeroed() },
            sample_order_type: unsafe { std::mem::zeroed() },
            custom_sample_order_count: unsafe { std::mem::zeroed() },
            custom_sample_orders: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::nv_shading_rate_image::{
    CoarseSampleOrderTypeNV, ShadingRatePaletteEntryNV, NV_SHADING_RATE_IMAGE_EXTENSION_NAME,
    NV_SHADING_RATE_IMAGE_SPEC_VERSION,
};
#[doc(alias = "vkCmdBindShadingRateImageNV")]
pub type FNCmdBindShadingRateImageNv =
    unsafe extern "system" fn(command_buffer: CommandBuffer, image_view: ImageView, image_layout: ImageLayout);
#[doc(alias = "vkCmdSetViewportShadingRatePaletteNV")]
pub type FNCmdSetViewportShadingRatePaletteNv = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_viewport: u32,
    viewport_count: u32,
    p_shading_rate_palettes: *const ShadingRatePaletteNV,
);
#[doc(alias = "vkCmdSetCoarseSampleOrderNV")]
pub type FNCmdSetCoarseSampleOrderNv = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    sample_order_type: CoarseSampleOrderTypeNV,
    custom_sample_order_count: u32,
    p_custom_sample_orders: *const CoarseSampleOrderCustomNV,
);
