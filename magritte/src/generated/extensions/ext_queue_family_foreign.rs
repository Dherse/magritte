//![VK_EXT_queue_family_foreign](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_queue_family_foreign.html) - device extension
//!# Description
//!This extension defines a special queue family,
//![`QUEUE_FAMILY_FOREIGN_EXT`], which can be used to transfer ownership
//!of resources backed by external memory to foreign, external queues.
//!This is similar to [`QUEUE_FAMILY_EXTERNAL_KHR`], defined in
//!`[`khr_external_memory`]`.
//!The key differences between the two are:
//! - The queues represented by [`QUEUE_FAMILY_EXTERNAL_KHR`] must share the same physical device
//!   and the same driver version as the current [`Instance`]. [`QUEUE_FAMILY_FOREIGN_EXT`] has no
//!   such restrictions. It can represent devices and drivers from other vendors, and can even
//!   represent non-Vulkan-capable devices.
//! - All resources backed by external memory support [`QUEUE_FAMILY_EXTERNAL_KHR`]. Support for
//!   [`QUEUE_FAMILY_FOREIGN_EXT`] is more restrictive.
//! - Applications should expect transitions to/from [`QUEUE_FAMILY_FOREIGN_EXT`] to be more
//!   expensive than transitions to/from [`QUEUE_FAMILY_EXTERNAL_KHR`].
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_external_memory`]`
//!# Contacts
//! - Chad Versace [chadversary](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_queue_family_foreign]
//!   @chadversary%0A<<Here describe the issue or question you have about the
//!   VK_EXT_queue_family_foreign extension>>)
//!# New constants
//! - [`EXT_QUEUE_FAMILY_FOREIGN_EXTENSION_NAME`]
//! - [`EXT_QUEUE_FAMILY_FOREIGN_SPEC_VERSION`]
//! - [`QUEUE_FAMILY_FOREIGN_EXT`]
//!# Version History
//! - Revision 1, 2017-11-01 (Chad Versace)  - Squashed internal revisions
//!# Other info
//! * 2017-11-01
//! * No known IP claims.
//! * - Chad Versace, Google  - James Jones, NVIDIA  - Jason Ekstrand, Intel  - Jesse Hall, Google
//!   - Daniel Rakos, AMD  - Ray Smith, ARM
//!# Related
//! - [`QUEUE_FAMILY_FOREIGN_EXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use std::ffi::CStr;
///[VK_QUEUE_FAMILY_FOREIGN_EXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_QUEUE_FAMILY_FOREIGN_EXT.html) - Foreign queue family index sentinel
///# C Specifications
///The special queue family index [`QUEUE_FAMILY_FOREIGN_EXT`] represents
///any queue external to the resource’s current Vulkan instance, regardless of
///the queue’s underlying physical device or driver version.
///This includes, for example, queues for fixed-function image processing
///devices, media codec devices, and display devices, as well as all queues
///that use the same underlying
///device group or
///physical device, and the same driver version as the resource’s
///[`Device`].
///```c
/// #define VK_QUEUE_FAMILY_FOREIGN_EXT       (~2U)
/// ```
///# Related
/// - [`ext_queue_family_foreign`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_QUEUE_FAMILY_FOREIGN_EXT")]
pub const QUEUE_FAMILY_FOREIGN_EXT: u32 = !2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_QUEUE_FAMILY_FOREIGN_SPEC_VERSION")]
pub const EXT_QUEUE_FAMILY_FOREIGN_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_QUEUE_FAMILY_FOREIGN_EXTENSION_NAME")]
pub const EXT_QUEUE_FAMILY_FOREIGN_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_queue_family_foreign");
