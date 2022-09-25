//![VK_KHR_external_fence](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_fence.html) - device extension
//!# Description
//!An application using external memory may wish to synchronize access to that
//!memory using fences.
//!This extension enables an application to create fences from which non-Vulkan
//!handles that reference the underlying synchronization primitive can be
//!exported.
# ! [doc = concat ! ("# " , "Revision")]
//!1
# ! [doc = concat ! ("# " , "Dependencies")]
//! - Requires Vulkan 1.0
//! - Requires `[`khr_external_fence_capabilities`]`
# ! [doc = concat ! ("# " , "Deprecation State")]
//! - *Promoted* to [Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions)
# ! [doc = concat ! ("# " , "Contacts")]
//! - Jesse Hall [critsec](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_external_fence]
//!   @critsec%0A<<Here describe the issue or question you have about the VK_KHR_external_fence
//!   extension>>)
# ! [doc = concat ! ("# " , "New structures")]
//! - Extending [`FenceCreateInfo`]:  - [`ExportFenceCreateInfoKHR`]
# ! [doc = concat ! ("# " , "New enums")]
//! - [`FenceImportFlagBitsKHR`]
# ! [doc = concat ! ("# " , "New bitmasks")]
//! - [`FenceImportFlagsKHR`]
# ! [doc = concat ! ("# " , "New constants")]
//! - [`KHR_EXTERNAL_FENCE_EXTENSION_NAME`]
//! - [`KHR_EXTERNAL_FENCE_SPEC_VERSION`]
//! - Extending [`FenceImportFlagBits`]:  - `VK_FENCE_IMPORT_TEMPORARY_BIT_KHR`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO_KHR`
# ! [doc = concat ! ("# " , "Known issues & F.A.Q.")]
//!This extension borrows concepts, semantics, and language from
//!`[`khr_external_semaphore`]`.
//!That extension’s issues apply equally to this extension.
# ! [doc = concat ! ("# " , "Version history")]
//! - Revision 1, 2017-05-08 (Jesse Hall)  - Initial revision
//!# Other info
//! * 2017-05-08
//! * No known IP claims.
//! * - Promoted to Vulkan 1.1 Core
//! * - Jesse Hall, Google  - James Jones, NVIDIA  - Jeff Juliano, NVIDIA  - Cass Everitt, Oculus  -
//!   Contributors to `[`khr_external_semaphore`]`
//!# Related
//! - [`ExportFenceCreateInfoKHR`]
//! - [`FenceImportFlagBitsKHR`]
//! - [`FenceImportFlagsKHR`]
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
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_SPEC_VERSION")]
pub const KHR_EXTERNAL_FENCE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_EXTENSION_NAME")]
pub const KHR_EXTERNAL_FENCE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_external_fence");
