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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDevicePciBusInfoPropertiesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`pci_domain`] is the PCI bus domain.
    pci_domain: u32,
    ///[`pci_bus`] is the PCI bus identifier.
    pci_bus: u32,
    ///[`pci_device`] is the PCI device identifier.
    pci_device: u32,
    ///[`pci_function`] is the PCI device function identifier.
    pci_function: u32,
}
impl<'lt> Default for PhysicalDevicePciBusInfoPropertiesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
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
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::pci_domain`]
    pub fn pci_domain_raw(&self) -> u32 {
        self.pci_domain
    }
    ///Gets the raw value of [`Self::pci_bus`]
    pub fn pci_bus_raw(&self) -> u32 {
        self.pci_bus
    }
    ///Gets the raw value of [`Self::pci_device`]
    pub fn pci_device_raw(&self) -> u32 {
        self.pci_device
    }
    ///Gets the raw value of [`Self::pci_function`]
    pub fn pci_function_raw(&self) -> u32 {
        self.pci_function
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::pci_domain`]
    pub fn set_pci_domain_raw(&mut self, value: u32) -> &mut Self {
        self.pci_domain = value;
        self
    }
    ///Sets the raw value of [`Self::pci_bus`]
    pub fn set_pci_bus_raw(&mut self, value: u32) -> &mut Self {
        self.pci_bus = value;
        self
    }
    ///Sets the raw value of [`Self::pci_device`]
    pub fn set_pci_device_raw(&mut self, value: u32) -> &mut Self {
        self.pci_device = value;
        self
    }
    ///Sets the raw value of [`Self::pci_function`]
    pub fn set_pci_function_raw(&mut self, value: u32) -> &mut Self {
        self.pci_function = value;
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
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::pci_bus`]
    pub fn pci_bus_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::pci_device`]
    pub fn pci_device_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::pci_function`]
    pub fn pci_function_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::pci_domain`]
    pub fn set_pci_domain(&mut self, value: u32) -> &mut Self {
        self.pci_domain = value;
        self
    }
    ///Sets the raw value of [`Self::pci_bus`]
    pub fn set_pci_bus(&mut self, value: u32) -> &mut Self {
        self.pci_bus = value;
        self
    }
    ///Sets the raw value of [`Self::pci_device`]
    pub fn set_pci_device(&mut self, value: u32) -> &mut Self {
        self.pci_device = value;
        self
    }
    ///Sets the raw value of [`Self::pci_function`]
    pub fn set_pci_function(&mut self, value: u32) -> &mut Self {
        self.pci_function = value;
        self
    }
}
