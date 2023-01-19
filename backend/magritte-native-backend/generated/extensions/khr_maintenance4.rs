//!# [VK_KHR_maintenance4](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_maintenance4.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_maintenance4/VK_KHR_maintenance4.md")]
use crate::{
    cstr,
    vulkan1_3::{
        DeviceBufferMemoryRequirements, DeviceImageMemoryRequirements, FNGetDeviceBufferMemoryRequirements,
        FNGetDeviceImageMemoryRequirements, FNGetDeviceImageSparseMemoryRequirements,
        PhysicalDeviceMaintenance4Features, PhysicalDeviceMaintenance4Properties,
    },
};
use std::ffi::CStr;
///See [`DeviceBufferMemoryRequirements`]
#[doc(alias = "VkDeviceBufferMemoryRequirementsKHR")]
pub type DeviceBufferMemoryRequirementsKHR = DeviceBufferMemoryRequirements;
///See [`DeviceImageMemoryRequirements`]
#[doc(alias = "VkDeviceImageMemoryRequirementsKHR")]
pub type DeviceImageMemoryRequirementsKHR = DeviceImageMemoryRequirements;
///See [`PhysicalDeviceMaintenance4Features`]
#[doc(alias = "VkPhysicalDeviceMaintenance4FeaturesKHR")]
pub type PhysicalDeviceMaintenance4FeaturesKHR = PhysicalDeviceMaintenance4Features;
///See [`PhysicalDeviceMaintenance4Properties`]
#[doc(alias = "VkPhysicalDeviceMaintenance4PropertiesKHR")]
pub type PhysicalDeviceMaintenance4PropertiesKHR = PhysicalDeviceMaintenance4Properties;
#[doc(alias = "VK_KHR_MAINTENANCE_4_SPEC_VERSION")]
pub const KHR_MAINTENANCE_4_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_KHR_MAINTENANCE_4_EXTENSION_NAME")]
pub const KHR_MAINTENANCE_4_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_maintenance4");
///See [`get_device_buffer_memory_requirements`]
#[doc(alias = "vkGetDeviceBufferMemoryRequirementsKHR")]
pub type FNGetDeviceBufferMemoryRequirementsKhr = FNGetDeviceBufferMemoryRequirements;
///See [`get_device_image_memory_requirements`]
#[doc(alias = "vkGetDeviceImageMemoryRequirementsKHR")]
pub type FNGetDeviceImageMemoryRequirementsKhr = FNGetDeviceImageMemoryRequirements;
///See [`get_device_image_sparse_memory_requirements`]
#[doc(alias = "vkGetDeviceImageSparseMemoryRequirementsKHR")]
pub type FNGetDeviceImageSparseMemoryRequirementsKhr = FNGetDeviceImageSparseMemoryRequirements;
