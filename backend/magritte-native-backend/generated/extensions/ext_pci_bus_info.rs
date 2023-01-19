//!# [VK_EXT_pci_bus_info](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_pci_bus_info.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_pci_bus_info/VK_EXT_pci_bus_info.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, StructureType},
};
use std::ffi::CStr;
///# [VkPhysicalDevicePCIBusInfoPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePCIBusInfoPropertiesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_pci_bus_info/VkPhysicalDevicePCIBusInfoPropertiesEXT.md")]
#[doc(alias = "VkPhysicalDevicePCIBusInfoPropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePciBusInfoPropertiesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "pciDomain")]
    pci_domain: u32,
    #[doc(alias = "pciBus")]
    pci_bus: u32,
    #[doc(alias = "pciDevice")]
    pci_device: u32,
    #[doc(alias = "pciFunction")]
    pci_function: u32,
}
#[doc(alias = "VK_EXT_PCI_BUS_INFO_SPEC_VERSION")]
pub const EXT_PCI_BUS_INFO_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_EXT_PCI_BUS_INFO_EXTENSION_NAME")]
pub const EXT_PCI_BUS_INFO_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_pci_bus_info");
