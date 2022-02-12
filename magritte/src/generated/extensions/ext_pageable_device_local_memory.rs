//![VK_EXT_pageable_device_local_memory](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_pageable_device_local_memory.html) - device extension
//!# Description
//!Vulkan is frequently implemented on multi-user and multi-process operating
//!systems where the device-local memory can be shared by more than one
//!process.
//!On such systems the size of the device-local memory available to the
//!application may not be the full size of the memory heap at all times.
//!In order for these operating systems to support multiple applications the
//!device-local memory is virtualized and paging is used to move memory between
//!device-local and host-local memory heaps, transparent to the application.The current Vulkan
//! specification does not expose this behavior well and may
//!cause applications to make suboptimal memory choices when allocating memory.
//!For example, in a system with multiple applications running, the application
//!may think that device-local memory is full and revert to making
//!performance-sensitive allocations from host-local memory.
//!In reality the memory heap might not have been full, it just appeared to be
//!at the time memory consumption was queried, and a device-local allocation
//!would have succeeded.
//!A well designed operating system that implements pageable device-local
//!memory will try to have all memory allocations for the foreground
//!application paged into device-local memory, and paged out for other
//!applications as needed when not in use.When this extension is exposed by the Vulkan
//! implementation it indicates to
//!the application that the operating system implements pageable device-local
//!memory and the application should adjust its memory allocation strategy
//!accordingly.
//!The extension also exposes a new [`SetDeviceMemoryPriorityEXT`] function
//!to allow the application to dynamically adjust the priority of existing
//!memory allocations based on its current needs.
//!This will help the operating system page out lower priority memory
//!allocations before higher priority allocations when needed.
//!It will also help the operating system decide which memory allocations to
//!page back into device-local memory first.To take best advantage of pageable device-local memory
//! the application must
//!create the Vulkan device with the
//![`PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT::pageable_device_local_memory`]
//!feature enabled.
//!When enabled the Vulkan implementation will allow device-local memory
//!allocations to be paged in and out by the operating system, and **may** not
//!return VK_ERROR_OUT_OF_DEVICE_MEMORY even if device-local memory appears to
//!be full, but will instead page this, or other allocations, out to make room.
//!The Vulkan implementation will also ensure that host-local memory
//!allocations will never be promoted to device-local memory by the operating
//!system, or consume device-local memory.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_EXT_memory_priority`]`
//!# Contacts
//! - Piers Daniell [pdaniell-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_pageable_device_local_memory]
//!   @pdaniell-nv%0A<<Here describe the issue or question you have about the
//!   VK_EXT_pageable_device_local_memory extension>>)
//!# New functions & commands
//! - [`SetDeviceMemoryPriorityEXT`]
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT`]
//!# New constants
//! - [`EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_EXTENSION_NAME`]
//! - [`EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT`
//!# Version History
//! - Revision 1, 2021-08-24 (Piers Daniell)
//! - Initial revision
//!# Other info
//! * 2021-08-24
//!*
//! - Hans-Kristian Arntzen, Valve
//! - Axel Gneiting, id Software
//! - Billy Khan, id Software
//! - Daniel Koch, NVIDIA
//! - Chris Lentini, NVIDIA
//! - Joshua Schnarr, NVIDIA
//! - Stu Smith, AMD
//!# Related
//! - [`PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT`]
//! - [`SetDeviceMemoryPriorityEXT`]
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
#[doc(alias = "VK_EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_SPEC_VERSION")]
pub const EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_EXTENSION_NAME")]
pub const EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_EXT_pageable_device_local_memory");
