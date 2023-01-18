use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, Extent2D, Offset2D, StructureType},
};
use std::ffi::CStr;
#[doc(alias = "VkPresentRegionsKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PresentRegionsKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "swapchainCount")]
    swapchain_count: u32,
    #[doc(alias = "pRegions")]
    regions: *const PresentRegionKHR,
}
#[doc(alias = "VkPresentRegionKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PresentRegionKHR {
    #[doc(alias = "rectangleCount")]
    rectangle_count: u32,
    #[doc(alias = "pRectangles")]
    rectangles: *const RectLayerKHR,
}
#[doc(alias = "VkRectLayerKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RectLayerKHR {
    offset: Offset2D,
    extent: Extent2D,
    layer: u32,
}
#[doc(alias = "VK_KHR_INCREMENTAL_PRESENT_SPEC_VERSION")]
pub const KHR_INCREMENTAL_PRESENT_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_KHR_INCREMENTAL_PRESENT_EXTENSION_NAME")]
pub const KHR_INCREMENTAL_PRESENT_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_incremental_present");
