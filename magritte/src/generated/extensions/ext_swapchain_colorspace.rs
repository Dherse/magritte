//![VK_EXT_swapchain_colorspace](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_swapchain_colorspace.html) - instance extension
//!# Description
//!To be done.
# ! [doc = concat ! ("# " , "Revision")]
//!4
# ! [doc = concat ! ("# " , "Dependencies")]
//! - Requires Vulkan 1.0
//! - Requires `[`khr_surface`]`
# ! [doc = concat ! ("# " , "Contacts")]
//! - Courtney Goeltzenleuchter [courtney-g](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_swapchain_colorspace]
//!   @courtney-g%0A<<Here describe the issue or question you have about the
//!   VK_EXT_swapchain_colorspace extension>>)
# ! [doc = concat ! ("# " , "New constants")]
//! - [`EXT_SWAPCHAIN_COLOR_SPACE_EXTENSION_NAME`]
//! - [`EXT_SWAPCHAIN_COLOR_SPACE_SPEC_VERSION`]
//! - Extending [`ColorSpaceKHR`]:  - `VK_COLOR_SPACE_ADOBERGB_LINEAR_EXT`  -
//!   `VK_COLOR_SPACE_ADOBERGB_NONLINEAR_EXT`  - `VK_COLOR_SPACE_BT2020_LINEAR_EXT`  -
//!   `VK_COLOR_SPACE_BT709_LINEAR_EXT`  - `VK_COLOR_SPACE_BT709_NONLINEAR_EXT`  -
//!   `VK_COLOR_SPACE_DCI_P3_LINEAR_EXT`  - `VK_COLOR_SPACE_DCI_P3_NONLINEAR_EXT`  -
//!   `VK_COLOR_SPACE_DISPLAY_P3_LINEAR_EXT`  - `VK_COLOR_SPACE_DISPLAY_P3_NONLINEAR_EXT`  -
//!   `VK_COLOR_SPACE_DOLBYVISION_EXT`  - `VK_COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT`  -
//!   `VK_COLOR_SPACE_EXTENDED_SRGB_NONLINEAR_EXT`  - `VK_COLOR_SPACE_HDR10_HLG_EXT`  -
//!   `VK_COLOR_SPACE_HDR10_ST2084_EXT`  - `VK_COLOR_SPACE_PASS_THROUGH_EXT`
# ! [doc = concat ! ("# " , "Known issues & F.A.Q.")]
//!1) Does the spec need to specify which kinds of image formats support the
//!color spaces? **RESOLVED** : Pixel format is independent of color space (though some color
//!spaces really want / need floating point color components to be useful).
//!Therefore, do not plan on documenting what formats support which
//!colorspaces.
//!An application  **can**  call [`get_physical_device_surface_formats_khr`] to query
//!what a particular implementation supports.2) How does application determine if HW supports
//! appropriate transfer
//!function for a colorspace? **RESOLVED** : Extension indicates that implementation  **must**  not
//! do the OETF
//!encoding if it is not sRGB.
//!That responsibility falls to the application shaders.
//!Any other native OETF / EOTF functions supported by an implementation can be
//!described by separate extension.
# ! [doc = concat ! ("# " , "Version history")]
//! - Revision 1, 2016-12-27 (Courtney Goeltzenleuchter)  - Initial version
//! - Revision 2, 2017-01-19 (Courtney Goeltzenleuchter)  - Add pass through and multiple options
//!   for BT2020.  - Clean up some issues with equations not displaying properly.
//! - Revision 3, 2017-06-23 (Courtney Goeltzenleuchter)  - Add extended sRGB non-linear enum.
//! - Revision 4, 2019-04-26 (Graeme Leese)  - Clarify colorspace transfer function usage.  - Refer
//!   to normative definitions in the Data Format Specification.  - Clarify DCI-P3 and Display P3
//!   usage.
//!# Other info
//! * 2019-04-26
//! * No known IP claims.
//! * - Courtney Goeltzenleuchter, Google
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
#[doc(alias = "VK_EXT_SWAPCHAIN_COLOR_SPACE_SPEC_VERSION")]
pub const EXT_SWAPCHAIN_COLOR_SPACE_SPEC_VERSION: u32 = 4;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_SWAPCHAIN_COLOR_SPACE_EXTENSION_NAME")]
pub const EXT_SWAPCHAIN_COLOR_SPACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_swapchain_colorspace");
