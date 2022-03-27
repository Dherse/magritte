use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_MEMORY_PRIORITY_SPEC_VERSION")]
pub const EXT_MEMORY_PRIORITY_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_MEMORY_PRIORITY_EXTENSION_NAME")]
pub const EXT_MEMORY_PRIORITY_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_memory_priority");
///[VkPhysicalDeviceMemoryPriorityFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMemoryPriorityFeaturesEXT.html) - Structure describing memory priority features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceMemoryPriorityFeaturesEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_memory_priority
///typedef struct VkPhysicalDeviceMemoryPriorityFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           memoryPriority;
///} VkPhysicalDeviceMemoryPriorityFeaturesEXT;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`memory_priority`] indicates that the implementation supports memory priorities specified at
///   memory allocation time via [`MemoryPriorityAllocateInfoEXT`].
///If the [`PhysicalDeviceMemoryPriorityFeaturesEXT`] structure is included in the [`p_next`] chain
/// of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceMemoryPriorityFeaturesEXT`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT`
///# Related
/// - [`VK_EXT_memory_priority`]
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
pub struct PhysicalDeviceMemoryPriorityFeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`memory_priority`] indicates that the
    ///implementation supports memory priorities specified at memory allocation
    ///time via [`MemoryPriorityAllocateInfoEXT`].
    memory_priority: Bool32,
}
///[VkMemoryPriorityAllocateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryPriorityAllocateInfoEXT.html) - Specify a memory allocation priority
///# C Specifications
///If the [`p_next`] chain includes a [`MemoryPriorityAllocateInfoEXT`]
///structure, then that structure includes a priority for the memory.The
/// [`MemoryPriorityAllocateInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_memory_priority
///typedef struct VkMemoryPriorityAllocateInfoEXT {
///    VkStructureType    sType;
///    const void*        pNext;
///    float              priority;
///} VkMemoryPriorityAllocateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`priority`] is a floating-point value between `0` and `1`, indicating the priority of the
///   allocation relative to other memory allocations. Larger values are higher priority. The
///   granularity of the priorities is implementation-dependent.
///# Description
///Memory allocations with higher priority **may** be more likely to stay in
///device-local memory when the system is under memory pressure.If this structure is not included,
/// it is as if the [`priority`] value were
///`0.5`.Valid Usage
/// - [`priority`]**must** be between `0` and `1`, inclusive
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_MEMORY_PRIORITY_ALLOCATE_INFO_EXT`
///# Related
/// - [`VK_EXT_memory_priority`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct MemoryPriorityAllocateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`priority`] is a floating-point value between `0` and `1`, indicating
    ///the priority of the allocation relative to other memory allocations.
    ///Larger values are higher priority.
    ///The granularity of the priorities is implementation-dependent.
    priority: f32,
}
