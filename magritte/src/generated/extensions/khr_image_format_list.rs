//![VK_KHR_image_format_list](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_image_format_list.html) - device extension
//!# Description
//!On some implementations, setting the
//!`VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT` on image creation can cause access
//!to that image to perform worse than an equivalent image created without
//!`VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT` because the implementation does not
//!know what view formats will be paired with the image.This extension allows an application to
//! provide the list of all formats that
//! **can**  be used with an image when it is created.
//!The implementation may then be able to create a more efficient image that
//!supports the subset of formats required by the application without having to
//!support all formats in the format compatibility class of the image format.
# ! [doc = concat ! ("# " , "Revision")]
//!1
# ! [doc = concat ! ("# " , "Dependencies")]
//! - Requires Vulkan 1.0
# ! [doc = concat ! ("# " , "Deprecation State")]
//! - *Promoted* to [Vulkan 1.2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.2-promotions)
# ! [doc = concat ! ("# " , "Contacts")]
//! - Jason Ekstrand [jekstrand](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_image_format_list]
//!   @jekstrand%0A<<Here describe the issue or question you have about the VK_KHR_image_format_list
//!   extension>>)
# ! [doc = concat ! ("# " , "New structures")]
//! - Extending [`ImageCreateInfo`], [`SwapchainCreateInfoKHR`], [`PhysicalDeviceImageFormatInfo2`]:
//!   - [`ImageFormatListCreateInfoKHR`]
# ! [doc = concat ! ("# " , "New constants")]
//! - [`KHR_IMAGE_FORMAT_LIST_EXTENSION_NAME`]
//! - [`KHR_IMAGE_FORMAT_LIST_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO_KHR`
# ! [doc = concat ! ("# " , "Version history")]
//! - Revision 1, 2017-03-20 (Jason Ekstrand)  - Initial revision
//!# Other info
//! * 2017-03-20
//! * - Promoted to Vulkan 1.2 Core
//! * No known IP claims.
//! * - Jason Ekstrand, Intel  - Jan-Harald Fredriksen, ARM  - Jeff Bolz, NVIDIA  - Jeff Leger,
//!   Qualcomm  - Neil Henning, Codeplay
//!# Related
//! - [`ImageFormatListCreateInfoKHR`]
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
#[doc(alias = "VK_KHR_IMAGE_FORMAT_LIST_SPEC_VERSION")]
pub const KHR_IMAGE_FORMAT_LIST_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_IMAGE_FORMAT_LIST_EXTENSION_NAME")]
pub const KHR_IMAGE_FORMAT_LIST_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_image_format_list");
