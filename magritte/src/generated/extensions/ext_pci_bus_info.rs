use crate::vulkan1_0::{BaseOutStructure, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_PCI_BUS_INFO_SPEC_VERSION")]
pub const EXT_PCI_BUS_INFO_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_PCI_BUS_INFO_EXTENSION_NAME")]
pub const EXT_PCI_BUS_INFO_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_pci_bus_info");
///[VkPhysicalDevicePCIBusInfoPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePCIBusInfoPropertiesEXT.html) - Structure containing PCI bus information of a physical device
///# C Specifications
///The [`PhysicalDevicePciBusInfoPropertiesEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_pci_bus_info
///typedef struct VkPhysicalDevicePCIBusInfoPropertiesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           pciDomain;
///    uint32_t           pciBus;
///    uint32_t           pciDevice;
///    uint32_t           pciFunction;
///} VkPhysicalDevicePCIBusInfoPropertiesEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`pci_domain`] is the PCI bus domain.
/// - [`pci_bus`] is the PCI bus identifier.
/// - [`pci_device`] is the PCI device identifier.
/// - [`pci_function`] is the PCI device function identifier.
///# Description
///If the [`PhysicalDevicePciBusInfoPropertiesEXT`] structure is included in the [`p_next`] chain
/// of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.These are properties of the PCI bus information
/// of a physical device.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT`
///# Related
/// - [`VK_EXT_pci_bus_info`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDevicePciBusInfoPropertiesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`pci_domain`] is the PCI bus domain.
    pci_domain: u32,
    ///[`pci_bus`] is the PCI bus identifier.
    pci_bus: u32,
    ///[`pci_device`] is the PCI device identifier.
    pci_device: u32,
    ///[`pci_function`] is the PCI device function identifier.
    pci_function: u32,
}
