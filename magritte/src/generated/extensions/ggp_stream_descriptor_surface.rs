//![VK_GGP_stream_descriptor_surface](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_GGP_stream_descriptor_surface.html) - instance extension
//!# Description
//!The [`VK_GGP_stream_descriptor_surface`] extension is an instance extension.
//!It provides a mechanism to create a [`SurfaceKHR`] object (defined by
//!the `[`VK_KHR_surface`]` extension) that refers to a Google Games
//!Platform [`GgpStreamDescriptor`].
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_surface`]`
//!# Contacts
//! - Jean-Francois Roy [jfroy](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_GGP_stream_descriptor_surface]
//!   @jfroy%0A<<Here describe the issue or question you have about the
//!   VK_GGP_stream_descriptor_surface extension>>)
//!# New functions & commands
//! - [`CreateStreamDescriptorSurfaceGGP`]
//!# New structures
//! - [`StreamDescriptorSurfaceCreateInfoGGP`]
//!# New bitmasks
//! - [`StreamDescriptorSurfaceCreateFlagsGGP`]
//!# New constants
//! - [`GGP_STREAM_DESCRIPTOR_SURFACE_EXTENSION_NAME`]
//! - [`GGP_STREAM_DESCRIPTOR_SURFACE_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP`
//!# Version History
//! - Revision 1, 2018-11-26 (Jean-Francois Roy)
//! - Initial revision.
//!# Other info
//! * 2019-01-28
//! * No known IP claims.
//!*
//! - Jean-Francois Roy, Google
//! - Brad Grantham, Google
//! - Connor Smith, Google
//! - Cort Stratton, Google
//! - Hai Nguyen, Google
//! - Ian Elliott, Google
//! - Jesse Hall, Google
//! - Jim Ray, Google
//! - Katherine Wu, Google
//! - Kaye Mason, Google
//! - Kuangye Guo, Google
//! - Mark Segal, Google
//! - Nicholas Vining, Google
//! - Paul Lalonde, Google
//! - Richard O’Grady, Google
//!# Related
//! - [`StreamDescriptorSurfaceCreateFlagsGGP`]
//! - [`StreamDescriptorSurfaceCreateInfoGGP`]
//! - [`CreateStreamDescriptorSurfaceGGP`]
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
#[doc(alias = "VK_GGP_STREAM_DESCRIPTOR_SURFACE_SPEC_VERSION")]
pub const GGP_STREAM_DESCRIPTOR_SURFACE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_GGP_STREAM_DESCRIPTOR_SURFACE_EXTENSION_NAME")]
pub const GGP_STREAM_DESCRIPTOR_SURFACE_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_GGP_stream_descriptor_surface");
