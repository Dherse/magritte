//![VK_EXT_pci_bus_info](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_pci_bus_info.html) - device extension
//!# Description
//!This extension adds a new query to obtain PCI bus information about a
//!physical device.Not all physical devices have PCI bus information, either due to the device
//!not being connected to the system through a PCI interface or due to platform
//!specific restrictions and policies.
//!Thus this extension is only expected to be supported by physical devices
//!which can provide the information.As a consequence, applications should always check for the
//! presence of the
//!extension string for each individual physical device for which they intend
//!to issue the new query for and should not have any assumptions about the
//!availability of the extension on any given platform.
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Matthaeus G. Chajdas [anteru](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_pci_bus_info]
//!   @anteru%0A<<Here describe the issue or question you have about the VK_EXT_pci_bus_info
//!   extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDevicePciBusInfoPropertiesEXT`]
//!# New constants
//! - [`EXT_PCI_BUS_INFO_EXTENSION_NAME`]
//! - [`EXT_PCI_BUS_INFO_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT`
//!# Version History
//! - Revision 2, 2018-12-10 (Daniel Rakos)  - Changed all members of the new structure to have the
//!   uint32_t type
//! - Revision 1, 2018-10-11 (Daniel Rakos)  - Initial revision
//!# Other info
//! * 2018-12-10
//! * No known IP claims.
//! * - Matthaeus G. Chajdas, AMD  - Daniel Rakos, AMD
//!# Related
//! - [`PhysicalDevicePciBusInfoPropertiesEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
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
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`pci_domain`] is the PCI bus domain.
/// - [`pci_bus`] is the PCI bus identifier.
/// - [`pci_device`] is the PCI device identifier.
/// - [`pci_function`] is the PCI device function identifier.
/// # Description
/// If the [`PhysicalDevicePciBusInfoPropertiesEXT`] structure is included in the [`p_next`] chain
/// of the
/// [`PhysicalDeviceProperties2`] structure passed to
/// [`get_physical_device_properties2`], it is filled in with each
/// corresponding implementation-dependent property.These are properties of the PCI bus information
/// of a physical device.
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT`
/// # Related
/// - [`VK_EXT_pci_bus_info`]
/// - [`StructureType`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDevicePCIBusInfoPropertiesEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDevicePciBusInfoPropertiesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`pci_domain`] is the PCI bus domain.
    pub pci_domain: u32,
    ///[`pci_bus`] is the PCI bus identifier.
    pub pci_bus: u32,
    ///[`pci_device`] is the PCI device identifier.
    pub pci_device: u32,
    ///[`pci_function`] is the PCI device function identifier.
    pub pci_function: u32,
}
impl<'lt> Default for PhysicalDevicePciBusInfoPropertiesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            pci_domain: 0,
            pci_bus: 0,
            pci_device: 0,
            pci_function: 0,
        }
    }
}
impl<'lt> PhysicalDevicePciBusInfoPropertiesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::pci_domain`]
    pub fn pci_domain(&self) -> u32 {
        self.pci_domain
    }
    ///Gets the value of [`Self::pci_bus`]
    pub fn pci_bus(&self) -> u32 {
        self.pci_bus
    }
    ///Gets the value of [`Self::pci_device`]
    pub fn pci_device(&self) -> u32 {
        self.pci_device
    }
    ///Gets the value of [`Self::pci_function`]
    pub fn pci_function(&self) -> u32 {
        self.pci_function
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::pci_domain`]
    pub fn pci_domain_mut(&mut self) -> &mut u32 {
        &mut self.pci_domain
    }
    ///Gets a mutable reference to the value of [`Self::pci_bus`]
    pub fn pci_bus_mut(&mut self) -> &mut u32 {
        &mut self.pci_bus
    }
    ///Gets a mutable reference to the value of [`Self::pci_device`]
    pub fn pci_device_mut(&mut self) -> &mut u32 {
        &mut self.pci_device
    }
    ///Gets a mutable reference to the value of [`Self::pci_function`]
    pub fn pci_function_mut(&mut self) -> &mut u32 {
        &mut self.pci_function
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::pci_domain`]
    pub fn set_pci_domain(mut self, value: u32) -> Self {
        self.pci_domain = value;
        self
    }
    ///Sets the value of [`Self::pci_bus`]
    pub fn set_pci_bus(mut self, value: u32) -> Self {
        self.pci_bus = value;
        self
    }
    ///Sets the value of [`Self::pci_device`]
    pub fn set_pci_device(mut self, value: u32) -> Self {
        self.pci_device = value;
        self
    }
    ///Sets the value of [`Self::pci_function`]
    pub fn set_pci_function(mut self, value: u32) -> Self {
        self.pci_function = value;
        self
    }
}
