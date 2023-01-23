//!# [VK_KHR_incremental_present](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_incremental_present.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_incremental_present/VK_KHR_incremental_present.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, Extent2D, Offset2D, StructureType},
};
use std::ffi::CStr;
///# [VkPresentRegionsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPresentRegionsKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_incremental_present/VkPresentRegionsKHR.md")]
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
///# [VkPresentRegionKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPresentRegionKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_incremental_present/VkPresentRegionKHR.md")]
#[doc(alias = "VkPresentRegionKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PresentRegionKHR {
    #[doc(alias = "rectangleCount")]
    rectangle_count: u32,
    #[doc(alias = "pRectangles")]
    rectangles: *const RectLayerKHR,
}
///# [VkRectLayerKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRectLayerKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_incremental_present/VkRectLayerKHR.md")]
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
