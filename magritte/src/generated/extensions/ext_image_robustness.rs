//![VK_EXT_image_robustness](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_image_robustness.html) - device extension
//!# Description
//!This extension adds stricter requirements for how out of bounds reads from
//!images are handled.
//!Rather than returning undefined values, most out of bounds reads return R,
//!G, and B values of zero and alpha values of either zero or one.
//!Components not present in the image format may be set to zero or to values
//!based on the format as described in [Conversion to RGBA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-conversion-to-rgba).
//!# Revision
//!1
//!# Dependencies
//! - *Promoted* to [Vulkan 1.3](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.3-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Graeme Leese [gnl21](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_image_robustness]
//!   @gnl21%0A<<Here describe the issue or question you have about the VK_EXT_image_robustness
//!   extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceImageRobustnessFeaturesEXT`]
//!# New constants
//! - [`EXT_IMAGE_ROBUSTNESS_EXTENSION_NAME`]
//! - [`EXT_IMAGE_ROBUSTNESS_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES_EXT`
//!# Known issues & F.A.Q
//!0. How does this extension differ from VK_EXT_robustness2?
//!The guarantees provided by this extension are a subset of those provided by
//!the robustImageAccess2 feature of VK_EXT_robustness2.
//!Where this extension allows return values of (0, 0, 0, 0) or (0, 0, 0, 1),
//!robustImageAccess2 requires that a particular value dependent on the image
//!format be returned.
//!This extension provides no guarantees about the values returned for an
//!access to an invalid Lod.
//!# Version History
//! - Revision 1, 2020-04-27 (Graeme Leese)  - Initial draft
//!# Other info
//! * 2020-04-27
//! * - Promoted to Vulkan 1.3 Core
//! * No known IP claims.
//! * - Graeme Leese, Broadcom  - Jan-Harald Fredriksen, ARM  - Jeff Bolz, NVIDIA  - Spencer Fricke,
//!   Samsung  - Courtney Goeltzenleuchter, Google  - Slawomir Cygan, Intel
//!# Related
//! - [`PhysicalDeviceImageRobustnessFeaturesEXT`]
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
#[doc(alias = "VK_EXT_IMAGE_ROBUSTNESS_SPEC_VERSION")]
pub const EXT_IMAGE_ROBUSTNESS_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_IMAGE_ROBUSTNESS_EXTENSION_NAME")]
pub const EXT_IMAGE_ROBUSTNESS_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_image_robustness");
