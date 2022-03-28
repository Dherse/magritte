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
///   estimate of how much memory the process  **can**  allocate from that heap before allocations
///   **may**  fail or cause performance degradation. The budget includes any currently allocated
///   device memory.
/// - [`heap_usage`] is an array of [`MAX_MEMORY_HEAPS`][`DeviceSize`] values in which memory usages
///   are returned, with one element for each memory heap. A heap’s usage is an estimate of how much
///   memory the process is currently using in that heap.
///# Description
///The values returned in this structure are not invariant.
///The [`heap_budget`] and [`heap_usage`] values  **must**  be zero for array
///elements greater than or equal to
///[`PhysicalDeviceMemoryProperties::memory_heap_count`].
///The [`heap_budget`] value  **must**  be non-zero for array elements less than
///[`PhysicalDeviceMemoryProperties::memory_heap_count`].
///The [`heap_budget`] value  **must**  be less than or equal to
///[`MemoryHeap::size`] for each heap.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT`
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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceMemoryBudgetPropertiesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`heap_budget`] is an array of [`MAX_MEMORY_HEAPS`][`DeviceSize`] values in which memory
    /// budgets are returned, with one element for each memory heap.
    ///A heap’s budget is a rough estimate of how much memory the process  **can**
    ///allocate from that heap before allocations  **may**  fail or cause
    ///performance degradation.
    ///The budget includes any currently allocated device memory.
    heap_budget: [DeviceSize; MAX_MEMORY_HEAPS],
    ///[`heap_usage`] is an array of [`MAX_MEMORY_HEAPS`][`DeviceSize`] values in which memory
    /// usages are returned, with one element for each memory heap.
    ///A heap’s usage is an estimate of how much memory the process is
    ///currently using in that heap.
    heap_usage: [DeviceSize; MAX_MEMORY_HEAPS],
}
impl<'lt> Default for PhysicalDeviceMemoryBudgetPropertiesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            heap_budget: [Default::default(); MAX_MEMORY_HEAPS],
            heap_usage: [Default::default(); MAX_MEMORY_HEAPS],
        }
    }
}
impl<'lt> PhysicalDeviceMemoryBudgetPropertiesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
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
    ///Gets the value of [`Self::heap_budget`]
    pub fn heap_budget(&self) -> &[DeviceSize; MAX_MEMORY_HEAPS] {
        &getter
    }
    ///Gets the value of [`Self::heap_usage`]
    pub fn heap_usage(&self) -> &[DeviceSize; MAX_MEMORY_HEAPS] {
        &getter
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
    ///Gets a mutable reference to the value of [`Self::heap_budget`]
    pub fn heap_budget_mut(&mut self) -> &mut [DeviceSize; MAX_MEMORY_HEAPS] {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::heap_usage`]
    pub fn heap_usage_mut(&mut self) -> &mut [DeviceSize; MAX_MEMORY_HEAPS] {
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
    ///Sets the raw value of [`Self::heap_budget`]
    pub fn set_heap_budget(
        &mut self,
        value: [crate::vulkan1_0::DeviceSize; crate::core::MAX_MEMORY_HEAPS],
    ) -> &mut Self {
        self.heap_budget = value;
        self
    }
    ///Sets the raw value of [`Self::heap_usage`]
    pub fn set_heap_usage(
        &mut self,
        value: [crate::vulkan1_0::DeviceSize; crate::core::MAX_MEMORY_HEAPS],
    ) -> &mut Self {
        self.heap_usage = value;
        self
    }
}
