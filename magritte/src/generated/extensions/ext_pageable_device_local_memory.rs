use crate::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_SPEC_VERSION")]
pub const EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_EXTENSION_NAME")]
pub const EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_EXT_pageable_device_local_memory");
///[VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT.html) - Structure describing whether the implementation supports pageable device-local memory
///# C Specifications
///The [`PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_pageable_device_local_memory
///typedef struct VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           pageableDeviceLocalMemory;
///} VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`pageable_device_local_memory`] indicates that the implementation supports pageable
///   device-local memory and **may** transparently move device-local memory allocations to
///   host-local memory to better share device-local memory with other applications.
///If the [`PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT`]**can** also be used in the [`p_next`]
/// chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT`
///# Related
/// - [`VK_EXT_pageable_device_local_memory`]
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`pageable_device_local_memory`]
    ///indicates that the implementation supports pageable device-local memory
    ///and **may** transparently move device-local memory allocations to
    ///host-local memory to better share device-local memory with other
    ///applications.
    pageable_device_local_memory: Bool32,
}
