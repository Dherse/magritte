use crate::{
    cstr,
    vulkan1_1::{PhysicalDeviceMultiviewFeatures, PhysicalDeviceMultiviewProperties, RenderPassMultiviewCreateInfo},
};
use std::ffi::CStr;
///See [`PhysicalDeviceMultiviewFeatures`]
#[doc(alias = "VkPhysicalDeviceMultiviewFeaturesKHR")]
pub type PhysicalDeviceMultiviewFeaturesKHR = PhysicalDeviceMultiviewFeatures;
///See [`PhysicalDeviceMultiviewProperties`]
#[doc(alias = "VkPhysicalDeviceMultiviewPropertiesKHR")]
pub type PhysicalDeviceMultiviewPropertiesKHR = PhysicalDeviceMultiviewProperties;
///See [`RenderPassMultiviewCreateInfo`]
#[doc(alias = "VkRenderPassMultiviewCreateInfoKHR")]
pub type RenderPassMultiviewCreateInfoKHR = RenderPassMultiviewCreateInfo;
#[doc(alias = "VK_KHR_MULTIVIEW_SPEC_VERSION")]
pub const KHR_MULTIVIEW_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_MULTIVIEW_EXTENSION_NAME")]
pub const KHR_MULTIVIEW_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_multiview");
