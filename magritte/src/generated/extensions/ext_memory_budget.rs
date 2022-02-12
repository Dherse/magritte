//![VK_EXT_memory_budget](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_memory_budget.html) - device extension
//!# Description
//!While running a Vulkan application, other processes on the machine might
//!also be attempting to use the same device memory, which can pose problems.
//!This extension adds support for querying the amount of memory used and the
//!total memory budget for a memory heap.
//!The values returned by this query are implementation-dependent and can
//!depend on a variety of factors including operating system and system load.The
//! [`PhysicalDeviceMemoryBudgetPropertiesEXT::heap_budget`] values
//!can be used as a guideline for how much total memory from each heap the
//!**current process** can use at any given time, before allocations may start
//!failing or causing performance degradation.
//!The values may change based on other activity in the system that is outside
//!the scope and control of the Vulkan implementation.The
//! [`PhysicalDeviceMemoryBudgetPropertiesEXT::heap_usage`] will
//!display the **current process** estimated heap usage.With this information, the idea is for an
//! application at some interval (once
//!per frame, per few seconds, etc) to query `heapBudget` and
//!`heapUsage`.
//!From here the application can notice if it is over budget and decide how it
//!wants to handle the memory situation (free it, move to host memory, changing
//!mipmap levels, etc).
//!This extension is designed to be used in concert with
//!`[`VK_EXT_memory_priority`]` to help with this part of memory management.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_memory_budget]
//!   @jeffbolznv%0A<<Here describe the issue or question you have about the VK_EXT_memory_budget
//!   extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceMemoryProperties2`]:
//! - [`PhysicalDeviceMemoryBudgetPropertiesEXT`]
//!# New constants
//! - [`EXT_MEMORY_BUDGET_EXTENSION_NAME`]
//! - [`EXT_MEMORY_BUDGET_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT`
//!# Version History
//! - Revision 1, 2018-10-08 (Jeff Bolz)
//! - Initial revision
//!# Other info
//! * 2018-10-08
//!*
//! - Jeff Bolz, NVIDIA
//! - Jeff Juliano, NVIDIA
//!# Related
//! - [`PhysicalDeviceMemoryBudgetPropertiesEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_MEMORY_BUDGET_SPEC_VERSION")]
pub const EXT_MEMORY_BUDGET_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_MEMORY_BUDGET_EXTENSION_NAME")]
pub const EXT_MEMORY_BUDGET_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_memory_budget");
