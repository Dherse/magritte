use crate::{
    cstr,
    vulkan1_0::{
        BaseInStructure, BaseOutStructure, Bool32, CommandBuffer, Extent2D, ImageLayout, ImageView, StructureType,
    },
};
use std::ffi::CStr;
#[doc(alias = "VkShadingRatePaletteNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ShadingRatePaletteNV {
    #[doc(alias = "shadingRatePaletteEntryCount")]
    shading_rate_palette_entry_count: u32,
    #[doc(alias = "pShadingRatePaletteEntries")]
    shading_rate_palette_entries: *const ShadingRatePaletteEntryNV,
}
#[doc(alias = "VkPipelineViewportShadingRateImageStateCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineViewportShadingRateImageStateCreateInfoNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "shadingRateImageEnable")]
    shading_rate_image_enable: Bool32,
    #[doc(alias = "viewportCount")]
    viewport_count: u32,
    #[doc(alias = "pShadingRatePalettes")]
    shading_rate_palettes: *const ShadingRatePaletteNV,
}
#[doc(alias = "VkPhysicalDeviceShadingRateImageFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShadingRateImageFeaturesNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "shadingRateImage")]
    shading_rate_image: Bool32,
    #[doc(alias = "shadingRateCoarseSampleOrder")]
    shading_rate_coarse_sample_order: Bool32,
}
#[doc(alias = "VkPhysicalDeviceShadingRateImagePropertiesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShadingRateImagePropertiesNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "shadingRateTexelSize")]
    shading_rate_texel_size: Extent2D,
    #[doc(alias = "shadingRatePaletteSize")]
    shading_rate_palette_size: u32,
    #[doc(alias = "shadingRateMaxCoarseSamples")]
    shading_rate_max_coarse_samples: u32,
}
#[doc(alias = "VkCoarseSampleLocationNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CoarseSampleLocationNV {
    #[doc(alias = "pixelX")]
    pixel_x: u32,
    #[doc(alias = "pixelY")]
    pixel_y: u32,
    sample: u32,
}
#[doc(alias = "VkCoarseSampleOrderCustomNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CoarseSampleOrderCustomNV {
    #[doc(alias = "shadingRate")]
    shading_rate: ShadingRatePaletteEntryNV,
    #[doc(alias = "sampleCount")]
    sample_count: u32,
    #[doc(alias = "sampleLocationCount")]
    sample_location_count: u32,
    #[doc(alias = "pSampleLocations")]
    sample_locations: *const CoarseSampleLocationNV,
}
#[doc(alias = "VkPipelineViewportCoarseSampleOrderStateCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineViewportCoarseSampleOrderStateCreateInfoNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "sampleOrderType")]
    sample_order_type: CoarseSampleOrderTypeNV,
    #[doc(alias = "customSampleOrderCount")]
    custom_sample_order_count: u32,
    #[doc(alias = "pCustomSampleOrders")]
    custom_sample_orders: *const CoarseSampleOrderCustomNV,
}
#[doc(alias = "VK_NV_SHADING_RATE_IMAGE_SPEC_VERSION")]
pub const NV_SHADING_RATE_IMAGE_SPEC_VERSION: u32 = 3;
#[doc(alias = "VK_NV_SHADING_RATE_IMAGE_EXTENSION_NAME")]
pub const NV_SHADING_RATE_IMAGE_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_shading_rate_image");
#[doc(alias = "VkShadingRatePaletteEntryNV")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ShadingRatePaletteEntryNV(i32);
impl ShadingRatePaletteEntryNV {
    #[doc(alias = "VK_SHADING_RATE_PALETTE_ENTRY_NO_INVOCATIONS_NV")]
    pub const NO_INVOCATIONS: Self = Self(0);
    #[doc(alias = "VK_SHADING_RATE_PALETTE_ENTRY_16_INVOCATIONS_PER_PIXEL_NV")]
    pub const N16_INVOCATIONS_PER_PIXEL: Self = Self(1);
    #[doc(alias = "VK_SHADING_RATE_PALETTE_ENTRY_8_INVOCATIONS_PER_PIXEL_NV")]
    pub const N8_INVOCATIONS_PER_PIXEL: Self = Self(2);
    #[doc(alias = "VK_SHADING_RATE_PALETTE_ENTRY_4_INVOCATIONS_PER_PIXEL_NV")]
    pub const N4_INVOCATIONS_PER_PIXEL: Self = Self(3);
    #[doc(alias = "VK_SHADING_RATE_PALETTE_ENTRY_2_INVOCATIONS_PER_PIXEL_NV")]
    pub const N2_INVOCATIONS_PER_PIXEL: Self = Self(4);
    #[doc(alias = "VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_PIXEL_NV")]
    pub const N1_INVOCATION_PER_PIXEL: Self = Self(5);
    #[doc(alias = "VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X1_PIXELS_NV")]
    pub const N1_INVOCATION_PER2X1_PIXELS: Self = Self(6);
    #[doc(alias = "VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_1X2_PIXELS_NV")]
    pub const N1_INVOCATION_PER1X2_PIXELS: Self = Self(7);
    #[doc(alias = "VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X2_PIXELS_NV")]
    pub const N1_INVOCATION_PER2X2_PIXELS: Self = Self(8);
    #[doc(alias = "VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X2_PIXELS_NV")]
    pub const N1_INVOCATION_PER4X2_PIXELS: Self = Self(9);
    #[doc(alias = "VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X4_PIXELS_NV")]
    pub const N1_INVOCATION_PER2X4_PIXELS: Self = Self(10);
    #[doc(alias = "VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X4_PIXELS_NV")]
    pub const N1_INVOCATION_PER4X4_PIXELS: Self = Self(11);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::NO_INVOCATIONS.bits() => Some(Self(x)),
            x if x == Self::N16_INVOCATIONS_PER_PIXEL.bits() => Some(Self(x)),
            x if x == Self::N8_INVOCATIONS_PER_PIXEL.bits() => Some(Self(x)),
            x if x == Self::N4_INVOCATIONS_PER_PIXEL.bits() => Some(Self(x)),
            x if x == Self::N2_INVOCATIONS_PER_PIXEL.bits() => Some(Self(x)),
            x if x == Self::N1_INVOCATION_PER_PIXEL.bits() => Some(Self(x)),
            x if x == Self::N1_INVOCATION_PER2X1_PIXELS.bits() => Some(Self(x)),
            x if x == Self::N1_INVOCATION_PER1X2_PIXELS.bits() => Some(Self(x)),
            x if x == Self::N1_INVOCATION_PER2X2_PIXELS.bits() => Some(Self(x)),
            x if x == Self::N1_INVOCATION_PER4X2_PIXELS.bits() => Some(Self(x)),
            x if x == Self::N1_INVOCATION_PER2X4_PIXELS.bits() => Some(Self(x)),
            x if x == Self::N1_INVOCATION_PER4X4_PIXELS.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
#[doc(alias = "VkCoarseSampleOrderTypeNV")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct CoarseSampleOrderTypeNV(i32);
impl CoarseSampleOrderTypeNV {
    #[doc(alias = "VK_COARSE_SAMPLE_ORDER_TYPE_DEFAULT_NV")]
    pub const DEFAULT: Self = Self(0);
    #[doc(alias = "VK_COARSE_SAMPLE_ORDER_TYPE_CUSTOM_NV")]
    pub const CUSTOM: Self = Self(1);
    #[doc(alias = "VK_COARSE_SAMPLE_ORDER_TYPE_PIXEL_MAJOR_NV")]
    pub const PIXEL_MAJOR: Self = Self(2);
    #[doc(alias = "VK_COARSE_SAMPLE_ORDER_TYPE_SAMPLE_MAJOR_NV")]
    pub const SAMPLE_MAJOR: Self = Self(3);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::DEFAULT.bits() => Some(Self(x)),
            x if x == Self::CUSTOM.bits() => Some(Self(x)),
            x if x == Self::PIXEL_MAJOR.bits() => Some(Self(x)),
            x if x == Self::SAMPLE_MAJOR.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
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
