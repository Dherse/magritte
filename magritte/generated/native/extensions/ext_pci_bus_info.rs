use crate::native::vulkan1_0::{BaseOutStructure, StructureType};
#[doc(alias = "VkPhysicalDevicePCIBusInfoPropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePciBusInfoPropertiesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "pciDomain")]
    pub pci_domain: u32,
    #[doc(alias = "pciBus")]
    pub pci_bus: u32,
    #[doc(alias = "pciDevice")]
    pub pci_device: u32,
    #[doc(alias = "pciFunction")]
    pub pci_function: u32,
}
impl Default for PhysicalDevicePciBusInfoPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDevicePciBusInfoPropertiesExt,
            p_next: unsafe { std::mem::zeroed() },
            pci_domain: unsafe { std::mem::zeroed() },
            pci_bus: unsafe { std::mem::zeroed() },
            pci_device: unsafe { std::mem::zeroed() },
            pci_function: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_pci_bus_info::{EXT_PCI_BUS_INFO_EXTENSION_NAME, EXT_PCI_BUS_INFO_SPEC_VERSION};
