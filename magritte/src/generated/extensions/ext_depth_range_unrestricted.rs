//![VK_EXT_depth_range_unrestricted](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_depth_range_unrestricted.html) - device extension
//!# Description
//!This extension removes the [`Viewport`]`minDepth` and
//!`maxDepth` restrictions that the values must be between `0.0` and `1.0`,
//!inclusive.
//!It also removes the same restriction on
//![`PipelineDepthStencilStateCreateInfo`]`minDepthBounds` and
//!`maxDepthBounds`.
//!Finally it removes the restriction on the `depth` value in
//![`ClearDepthStencilValue`].
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Piers Daniell [pdaniell-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_depth_range_unrestricted]
//!   @pdaniell-nv%0A<<Here describe the issue or question you have about the
//!   VK_EXT_depth_range_unrestricted extension>>)
//!# New constants
//! - [`EXT_DEPTH_RANGE_UNRESTRICTED_EXTENSION_NAME`]
//! - [`EXT_DEPTH_RANGE_UNRESTRICTED_SPEC_VERSION`]
//!# Known issues & F.A.Q
//!1) How do [`Viewport`]`minDepth` and `maxDepth` values outside
//!of the `0.0` to `1.0` range interact with
//![Primitive Clipping](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#vertexpostproc-clipping)? **RESOLVED** : The behavior described in [Primitive
//!Clipping](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#vertexpostproc-clipping) still applies.
//!If depth clamping is disabled the depth values are still clipped to 0
//!≤ z<sub>c</sub> ≤ w<sub>c</sub> before the viewport transform.
//!If depth clamping is enabled the above equation is ignored and the depth
//!values are instead clamped to the [`Viewport`]`minDepth` and
//!`maxDepth` values, which in the case of this extension can be outside of
//!the `0.0` to `1.0` range.2) What happens if a resulting depth fragment is outside of the `0.0`
//! to
//!`1.0` range and the depth buffer is fixed-point rather than floating-point? **RESOLVED** : The
//! supported range of a fixed-point depth buffer is `0.0` to
//!`1.0` and depth fragments are clamped to this range.
//!# Version History
//! - Revision 1, 2017-06-22 (Piers Daniell)  - Internal revisions
//!# Other info
//! * 2017-06-22
//! * - Daniel Koch, NVIDIA  - Jeff Bolz, NVIDIA
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
#[doc(alias = "VK_EXT_DEPTH_RANGE_UNRESTRICTED_SPEC_VERSION")]
pub const EXT_DEPTH_RANGE_UNRESTRICTED_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DEPTH_RANGE_UNRESTRICTED_EXTENSION_NAME")]
pub const EXT_DEPTH_RANGE_UNRESTRICTED_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_depth_range_unrestricted");
