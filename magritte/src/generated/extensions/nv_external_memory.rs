//![VK_NV_external_memory](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_external_memory.html) - device extension
//!# Description
//!Applications may wish to export memory to other Vulkan instances or other
//!APIs, or import memory from other Vulkan instances or other APIs to enable
//!Vulkan workloads to be split up across application module, process, or API
//!boundaries.
//!This extension enables applications to create exportable Vulkan memory
//!objects such that the underlying resources can be referenced outside the
//!Vulkan instance that created them.
//!# Revision
//!1
//!# Dependencies
//! - *Deprecated* by
//!`[`VK_KHR_external_memory`]`
//!extension
//! - Which in turn was *promoted* to
//![Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_NV_external_memory_capabilities`]`
//!# Contacts
//! - James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_external_memory]
//!   @cubanismo%0A<<Here describe the issue or question you have about the VK_NV_external_memory
//!   extension>>)
//!# New structures
//! - Extending [`ImageCreateInfo`]:
//! - [`ExternalMemoryImageCreateInfoNV`]
//!
//! - Extending [`MemoryAllocateInfo`]:
//! - [`ExportMemoryAllocateInfoNV`]
//!# New constants
//! - [`NV_EXTERNAL_MEMORY_EXTENSION_NAME`]
//! - [`NV_EXTERNAL_MEMORY_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_NV`
//! - `VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV`
//!# Known issues & F.A.Q
//!1) If memory objects are shared between processes and APIs, is this
//!considered aliasing according to the rules outlined in the
//![Memory Aliasing](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-memory-aliasing) section?**RESOLVED**: Yes, but strict exceptions to the rules are added to allow some
//!forms of aliasing in these cases.
//!Further, other extensions may build upon these new aliasing rules to define
//!specific support usage within Vulkan for imported native memory objects, or
//!memory objects from other APIs.2) Are new image layouts or metadata required to specify image
//! layouts and
//!layout transitions compatible with non-Vulkan APIs, or with other instances
//!of the same Vulkan driver?**RESOLVED**: No.
//!Separate instances of the same Vulkan driver running on the same GPU should
//!have identical internal layout semantics, so applictions have the tools they
//!need to ensure views of images are consistent between the two instances.
//!Other APIs will fall into two categories: Those that are Vulkan compatible
//!(a term to be defined by subsequent interopability extensions), or Vulkan
//!incompatible.
//!When sharing images with Vulkan incompatible APIs, the Vulkan image must be
//!transitioned to the `VK_IMAGE_LAYOUT_GENERAL` layout before handing it
//!off to the external API.Note this does not attempt to address cross-device transitions, nor
//!transitions to engines on the same device which are not visible within the
//!Vulkan API.
//!Both of these are beyond the scope of this extension.
//!# Version History
//! - Revision 1, 2016-08-19 (James Jones)
//! - Initial draft
//!# Other info
//! * 2016-08-19
//! * No known IP claims.
//!*
//! - James Jones, NVIDIA
//! - Carsten Rohde, NVIDIA
//!# Related
//! - [`ExportMemoryAllocateInfoNV`]
//! - [`ExternalMemoryImageCreateInfoNV`]
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
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_SPEC_VERSION")]
pub const NV_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_EXTENSION_NAME")]
pub const NV_EXTERNAL_MEMORY_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_external_memory");
