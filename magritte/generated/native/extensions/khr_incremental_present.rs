pub use crate::common::extensions::khr_incremental_present::RectLayerKHR;
use crate::native::vulkan1_0::{BaseInStructure, StructureType};
#[doc(alias = "VkPresentRegionsKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PresentRegionsKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "swapchainCount")]
    pub swapchain_count: u32,
    #[doc(alias = "pRegions")]
    pub regions: *const PresentRegionKHR,
}
impl Default for PresentRegionsKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::PresentRegionsKhr,
            p_next: unsafe { std::mem::zeroed() },
            swapchain_count: unsafe { std::mem::zeroed() },
            regions: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPresentRegionKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PresentRegionKHR {
    #[doc(alias = "rectangleCount")]
    pub rectangle_count: u32,
    #[doc(alias = "pRectangles")]
    pub rectangles: *const RectLayerKHR,
}
impl Default for PresentRegionKHR {
    fn default() -> Self {
        Self {
            rectangle_count: unsafe { std::mem::zeroed() },
            rectangles: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::khr_incremental_present::{
    KHR_INCREMENTAL_PRESENT_EXTENSION_NAME, KHR_INCREMENTAL_PRESENT_SPEC_VERSION,
};
