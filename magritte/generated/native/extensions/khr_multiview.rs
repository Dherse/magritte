use crate::native::vulkan1_1::{
    PhysicalDeviceMultiviewFeatures, PhysicalDeviceMultiviewProperties, RenderPassMultiviewCreateInfo,
};
///See [`PhysicalDeviceMultiviewFeatures`]
#[doc(alias = "VkPhysicalDeviceMultiviewFeaturesKHR")]
pub type PhysicalDeviceMultiviewFeaturesKHR = PhysicalDeviceMultiviewFeatures;
///See [`PhysicalDeviceMultiviewProperties`]
#[doc(alias = "VkPhysicalDeviceMultiviewPropertiesKHR")]
pub type PhysicalDeviceMultiviewPropertiesKHR = PhysicalDeviceMultiviewProperties;
///See [`RenderPassMultiviewCreateInfo`]
#[doc(alias = "VkRenderPassMultiviewCreateInfoKHR")]
pub type RenderPassMultiviewCreateInfoKHR = RenderPassMultiviewCreateInfo;
pub use crate::common::extensions::khr_multiview::{KHR_MULTIVIEW_EXTENSION_NAME, KHR_MULTIVIEW_SPEC_VERSION};
