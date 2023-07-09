pub use crate::common::extensions::ext_private_data::{EXT_PRIVATE_DATA_EXTENSION_NAME, EXT_PRIVATE_DATA_SPEC_VERSION};
use crate::vulkan1_3::{
    DevicePrivateDataCreateInfo, PhysicalDevicePrivateDataFeatures, PrivateDataSlot, PrivateDataSlotCreateFlags,
    PrivateDataSlotCreateInfo,
};
#[doc(alias = "VkPrivateDataSlotCreateFlagsEXT")]
pub type PrivateDataSlotCreateFlagsEXT = PrivateDataSlotCreateFlags;
#[doc(alias = "VkPrivateDataSlotEXT")]
pub type PrivateDataSlotEXT = PrivateDataSlot;
#[doc(alias = "VkDevicePrivateDataCreateInfoEXT")]
pub type DevicePrivateDataCreateInfoEXT = DevicePrivateDataCreateInfo;
#[doc(alias = "VkPrivateDataSlotCreateInfoEXT")]
pub type PrivateDataSlotCreateInfoEXT = PrivateDataSlotCreateInfo;
#[doc(alias = "VkPhysicalDevicePrivateDataFeaturesEXT")]
pub type PhysicalDevicePrivateDataFeaturesEXT = PhysicalDevicePrivateDataFeatures;
