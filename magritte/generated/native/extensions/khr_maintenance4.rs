use crate::native::vulkan1_3::{
    DeviceBufferMemoryRequirements, DeviceImageMemoryRequirements, FNGetDeviceBufferMemoryRequirements,
    FNGetDeviceImageMemoryRequirements, FNGetDeviceImageSparseMemoryRequirements, PhysicalDeviceMaintenance4Features,
    PhysicalDeviceMaintenance4Properties,
};
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
pub use crate::common::extensions::khr_maintenance4::{
    KHR_MAINTENANCE_4_EXTENSION_NAME, KHR_MAINTENANCE_4_SPEC_VERSION,
};
///See [`get_device_buffer_memory_requirements`]
#[doc(alias = "vkGetDeviceBufferMemoryRequirementsKHR")]
pub type FNGetDeviceBufferMemoryRequirementsKhr = FNGetDeviceBufferMemoryRequirements;
///See [`get_device_image_memory_requirements`]
#[doc(alias = "vkGetDeviceImageMemoryRequirementsKHR")]
pub type FNGetDeviceImageMemoryRequirementsKhr = FNGetDeviceImageMemoryRequirements;
///See [`get_device_image_sparse_memory_requirements`]
#[doc(alias = "vkGetDeviceImageSparseMemoryRequirementsKHR")]
pub type FNGetDeviceImageSparseMemoryRequirementsKhr = FNGetDeviceImageSparseMemoryRequirements;
