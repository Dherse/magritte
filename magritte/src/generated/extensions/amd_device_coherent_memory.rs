//![VK_AMD_device_coherent_memory](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_AMD_device_coherent_memory.html) - device extension
//!# Description
//!This extension adds the device coherent and device uncached memory types.
//!Any device accesses to device coherent memory are automatically made visible
//!to any other device access.
//!Device uncached memory indicates to applications that caches are disabled
//!for a particular memory type, which guarantees device coherence.Device coherent and uncached
//! memory are expected to have lower performance
//!for general access than non-device coherent memory, but can be useful in
//!certain scenarios; particularly so for debugging.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Tobias Hector [tobski](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_AMD_device_coherent_memory]
//!   @tobski%0A<<Here describe the issue or question you have about the
//!   VK_AMD_device_coherent_memory extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceCoherentMemoryFeaturesAMD`]
//!# New constants
//! - [`AMD_DEVICE_COHERENT_MEMORY_EXTENSION_NAME`]
//! - [`AMD_DEVICE_COHERENT_MEMORY_SPEC_VERSION`]
//! - Extending [`MemoryPropertyFlagBits`]:  - `VK_MEMORY_PROPERTY_DEVICE_COHERENT_BIT_AMD`  -
//!   `VK_MEMORY_PROPERTY_DEVICE_UNCACHED_BIT_AMD`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD`
//!# Version history
//! - Revision 1, 2019-02-04 (Tobias Hector)  - Initial revision
//!# Other information
//! * 2019-02-04
//! * - Ping Fu, AMD  - Timothy Lottes, AMD  - Tobias Hector, AMD
//!# Related
//! - [`PhysicalDeviceCoherentMemoryFeaturesAMD`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_DEVICE_COHERENT_MEMORY_SPEC_VERSION")]
pub const AMD_DEVICE_COHERENT_MEMORY_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_DEVICE_COHERENT_MEMORY_EXTENSION_NAME")]
pub const AMD_DEVICE_COHERENT_MEMORY_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_AMD_device_coherent_memory");
///[VkPhysicalDeviceCoherentMemoryFeaturesAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCoherentMemoryFeaturesAMD.html) - Structure describing whether device coherent memory can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceCoherentMemoryFeaturesAMD`] structure is defined as:
///```c
///// Provided by VK_AMD_device_coherent_memory
///typedef struct VkPhysicalDeviceCoherentMemoryFeaturesAMD {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           deviceCoherentMemory;
///} VkPhysicalDeviceCoherentMemoryFeaturesAMD;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`device_coherent_memory`] indicates that the implementation supports
///   [`MemoryPropertyFlagBits`].
///If the [`PhysicalDeviceCoherentMemoryFeaturesAMD`] structure is included in the [`p_next`] chain
/// of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceCoherentMemoryFeaturesAMD`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD`
///# Related
/// - [`amd_device_coherent_memory`]
/// - [`Bool32`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceCoherentMemoryFeaturesAMD")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceCoherentMemoryFeaturesAMD<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`device_coherent_memory`] indicates
    ///that the implementation supports [`MemoryPropertyFlagBits`].
    pub device_coherent_memory: Bool32,
}
impl<'lt> Default for PhysicalDeviceCoherentMemoryFeaturesAMD<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD,
            p_next: std::ptr::null_mut(),
            device_coherent_memory: 0,
        }
    }
}
impl<'lt> PhysicalDeviceCoherentMemoryFeaturesAMD<'lt> {
    ///Creates a static version of this structure
    pub fn make_static(mut self) -> PhysicalDeviceCoherentMemoryFeaturesAMD<'static> {
        unsafe {
            self.p_next = std::ptr::null_mut() as _;
            std::mem::transmute(self)
        }
    }
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::device_coherent_memory`]
    pub fn device_coherent_memory_raw(&self) -> Bool32 {
        self.device_coherent_memory
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::device_coherent_memory`]
    pub fn set_device_coherent_memory_raw(&mut self, value: Bool32) -> &mut Self {
        self.device_coherent_memory = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::device_coherent_memory`]
    pub fn with_device_coherent_memory_raw(mut self, value: Bool32) -> Self {
        self.device_coherent_memory = value;
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
    ///Gets the value of [`Self::device_coherent_memory`]
    pub fn device_coherent_memory(&self) -> bool {
        unsafe { std::mem::transmute(self.device_coherent_memory as u8) }
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
    ///Gets a mutable reference to the value of [`Self::device_coherent_memory`]
    pub fn device_coherent_memory_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.device_coherent_memory as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.device_coherent_memory as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::device_coherent_memory`]
    pub fn set_device_coherent_memory(&mut self, value: bool) -> &mut Self {
        self.device_coherent_memory = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::device_coherent_memory`]
    pub fn with_device_coherent_memory(mut self, value: bool) -> Self {
        self.device_coherent_memory = value as u8 as u32;
        self
    }
}
