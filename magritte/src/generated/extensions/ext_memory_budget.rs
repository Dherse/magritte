use crate::{
    core::MAX_MEMORY_HEAPS,
    vulkan1_0::{BaseOutStructure, DeviceSize, StructureType},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_MEMORY_BUDGET_SPEC_VERSION")]
pub const EXT_MEMORY_BUDGET_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_MEMORY_BUDGET_EXTENSION_NAME")]
pub const EXT_MEMORY_BUDGET_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_memory_budget");
///[VkPhysicalDeviceMemoryBudgetPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMemoryBudgetPropertiesEXT.html) - Structure specifying physical device memory budget and usage
///# C Specifications
///If the [`PhysicalDeviceMemoryBudgetPropertiesEXT`] structure is included
///in the [`p_next`] chain of [`PhysicalDeviceMemoryProperties2`], it is
///filled with the current memory budgets and usages.The
/// [`PhysicalDeviceMemoryBudgetPropertiesEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_memory_budget
///typedef struct VkPhysicalDeviceMemoryBudgetPropertiesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkDeviceSize       heapBudget[VK_MAX_MEMORY_HEAPS];
///    VkDeviceSize       heapUsage[VK_MAX_MEMORY_HEAPS];
///} VkPhysicalDeviceMemoryBudgetPropertiesEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`heap_budget`] is an array of [`MAX_MEMORY_HEAPS`][`DeviceSize`] values in which memory
///   budgets are returned, with one element for each memory heap. A heap’s budget is a rough
///   estimate of how much memory the process **can** allocate from that heap before allocations
///   **may** fail or cause performance degradation. The budget includes any currently allocated
///   device memory.
/// - [`heap_usage`] is an array of [`MAX_MEMORY_HEAPS`][`DeviceSize`] values in which memory usages
///   are returned, with one element for each memory heap. A heap’s usage is an estimate of how much
///   memory the process is currently using in that heap.
///# Description
///The values returned in this structure are not invariant.
///The [`heap_budget`] and [`heap_usage`] values **must** be zero for array
///elements greater than or equal to
///[`PhysicalDeviceMemoryProperties::memory_heap_count`].
///The [`heap_budget`] value **must** be non-zero for array elements less than
///[`PhysicalDeviceMemoryProperties::memory_heap_count`].
///The [`heap_budget`] value **must** be less than or equal to
///[`MemoryHeap::size`] for each heap.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT`
///# Related
/// - [`VK_EXT_memory_budget`]
/// - [`DeviceSize`]
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
pub struct PhysicalDeviceMemoryBudgetPropertiesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`heap_budget`] is an array of [`MAX_MEMORY_HEAPS`][`DeviceSize`] values in which memory
    /// budgets are returned, with one element for each memory heap.
    ///A heap’s budget is a rough estimate of how much memory the process **can**
    ///allocate from that heap before allocations **may** fail or cause
    ///performance degradation.
    ///The budget includes any currently allocated device memory.
    heap_budget: [DeviceSize; MAX_MEMORY_HEAPS],
    ///[`heap_usage`] is an array of [`MAX_MEMORY_HEAPS`][`DeviceSize`] values in which memory
    /// usages are returned, with one element for each memory heap.
    ///A heap’s usage is an estimate of how much memory the process is
    ///currently using in that heap.
    heap_usage: [DeviceSize; MAX_MEMORY_HEAPS],
}
