use crate::native::vulkan1_3::{
    DevicePrivateDataCreateInfo, FNCreatePrivateDataSlot, FNDestroyPrivateDataSlot, FNGetPrivateData, FNSetPrivateData,
    PhysicalDevicePrivateDataFeatures, PrivateDataSlot, PrivateDataSlotCreateFlags, PrivateDataSlotCreateInfo,
};
///See [`PrivateDataSlotCreateFlags`]
#[doc(alias = "VkPrivateDataSlotCreateFlagsEXT")]
pub type PrivateDataSlotCreateFlagsEXT = PrivateDataSlotCreateFlags;
///See [`PrivateDataSlot`]
#[doc(alias = "VkPrivateDataSlotEXT")]
pub type PrivateDataSlotEXT = PrivateDataSlot;
///See [`DevicePrivateDataCreateInfo`]
#[doc(alias = "VkDevicePrivateDataCreateInfoEXT")]
pub type DevicePrivateDataCreateInfoEXT = DevicePrivateDataCreateInfo;
///See [`PrivateDataSlotCreateInfo`]
#[doc(alias = "VkPrivateDataSlotCreateInfoEXT")]
pub type PrivateDataSlotCreateInfoEXT = PrivateDataSlotCreateInfo;
///See [`PhysicalDevicePrivateDataFeatures`]
#[doc(alias = "VkPhysicalDevicePrivateDataFeaturesEXT")]
pub type PhysicalDevicePrivateDataFeaturesEXT = PhysicalDevicePrivateDataFeatures;
pub use crate::common::extensions::ext_private_data::{EXT_PRIVATE_DATA_EXTENSION_NAME, EXT_PRIVATE_DATA_SPEC_VERSION};
///See [`create_private_data_slot`]
#[doc(alias = "vkCreatePrivateDataSlotEXT")]
pub type FNCreatePrivateDataSlotExt = FNCreatePrivateDataSlot;
///See [`destroy_private_data_slot`]
#[doc(alias = "vkDestroyPrivateDataSlotEXT")]
pub type FNDestroyPrivateDataSlotExt = FNDestroyPrivateDataSlot;
///See [`set_private_data`]
#[doc(alias = "vkSetPrivateDataEXT")]
pub type FNSetPrivateDataExt = FNSetPrivateData;
///See [`get_private_data`]
#[doc(alias = "vkGetPrivateDataEXT")]
pub type FNGetPrivateDataExt = FNGetPrivateData;
