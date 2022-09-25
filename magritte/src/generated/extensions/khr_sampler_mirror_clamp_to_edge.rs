//![VK_KHR_sampler_mirror_clamp_to_edge](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_sampler_mirror_clamp_to_edge.html) - device extension
//!# Description
//![`VK_KHR_sampler_mirror_clamp_to_edge`] extends the set of sampler address
//!modes to include an additional mode
//!(`VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE`) that effectively uses a
//!texture map twice as large as the original image in which the additional
//!half of the new image is a mirror image of the original image.This new mode relaxes the need to
//! generate images whose opposite edges match
//!by using the original image to generate a matching “mirror image”.
//!This mode allows the texture to be mirrored only once in the negative s, t,
//!and r directions.
# ! [doc = concat ! ("# " , "Revision")]
//!3
# ! [doc = concat ! ("# " , "Dependencies")]
//! - Requires Vulkan 1.0
# ! [doc = concat ! ("# " , "Deprecation State")]
//! - *Promoted* to [Vulkan 1.2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.2-promotions)
# ! [doc = concat ! ("# " , "Contacts")]
//! - Tobias Hector [tobski](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_sampler_mirror_clamp_to_edge]
//!   @tobski%0A<<Here describe the issue or question you have about the
//!   VK_KHR_sampler_mirror_clamp_to_edge extension>>)
# ! [doc = concat ! ("# " , "New constants")]
//! - [`KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_EXTENSION_NAME`]
//! - [`KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_SPEC_VERSION`]
//! - Extending [`SamplerAddressMode`]:  - `VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE`  -
//!   `VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE_KHR`
# ! [doc = concat ! ("# " , "Known issues & F.A.Q.")]
//!1) Why are both KHR and core versions of the
//!`VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE` token present? **RESOLVED** : This functionality
//! was intended to be required in Vulkan 1.0.
//!We realized shortly before public release that not all implementations could
//!support it, and moved the functionality into an optional extension, but did
//!not apply the KHR extension suffix.
//!Adding a KHR-suffixed alias of the non-suffixed enum has been done to comply
//!with our own naming rules.In a related change, before spec revision 1.1.121 this extension was
//!hardwiring into the spec Makefile so it was always included with the
//!Specification, even in the core-only versions.
//!This has now been reverted, and it is treated as any other extension.
# ! [doc = concat ! ("# " , "Version history")]
//! - Revision 1, 2016-02-16 (Tobias Hector)  - Initial draft
//! - Revision 2, 2019-08-14 (Jon Leech)  - Add KHR-suffixed alias of non-suffixed enum.
//! - Revision 3, 2019-08-17 (Jon Leech)  - Add an issue explaining the reason for the extension API
//!   not being suffixed with KHR.
//!# Other info
//! * 2019-08-17
//! * - Promoted to Vulkan 1.2 Core
//! * - Tobias Hector, Imagination Technologies  - Jon Leech, Khronos
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
#[doc(alias = "VK_KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_SPEC_VERSION")]
pub const KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_SPEC_VERSION: u32 = 3;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_EXTENSION_NAME")]
pub const KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_KHR_sampler_mirror_clamp_to_edge");
