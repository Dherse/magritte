pub use crate::common::extensions::khr_maintenance4::{
    KHR_MAINTENANCE_4_EXTENSION_NAME, KHR_MAINTENANCE_4_SPEC_VERSION,
};
use crate::vulkan1_3::{
    DeviceBufferMemoryRequirements, DeviceImageMemoryRequirements, PhysicalDeviceMaintenance4Features,
    PhysicalDeviceMaintenance4Properties,
};
#[doc(alias = "VkDeviceBufferMemoryRequirementsKHR")]
pub type DeviceBufferMemoryRequirementsKHR = DeviceBufferMemoryRequirements;
#[doc(alias = "VkDeviceImageMemoryRequirementsKHR")]
pub type DeviceImageMemoryRequirementsKHR = DeviceImageMemoryRequirements;
#[doc(alias = "VkPhysicalDeviceMaintenance4FeaturesKHR")]
pub type PhysicalDeviceMaintenance4FeaturesKHR = PhysicalDeviceMaintenance4Features;
#[doc(alias = "VkPhysicalDeviceMaintenance4PropertiesKHR")]
pub type PhysicalDeviceMaintenance4PropertiesKHR = PhysicalDeviceMaintenance4Properties;
