//![VK_NV_clip_space_w_scaling](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_clip_space_w_scaling.html) - device extension
//!# Description
//!Virtual Reality (VR) applications often involve a post-processing step to
//!apply a “barrel” distortion to the rendered image to correct the
//!“pincushion” distortion introduced by the optics in a VR device.
//!The barrel distorted image has lower resolution along the edges compared to
//!the center.
//!Since the original image is rendered at high resolution, which is uniform
//!across the complete image, a lot of pixels towards the edges do not make it
//!to the final post-processed image.This extension provides a mechanism to render VR scenes at a
//! non-uniform
//!resolution, in particular a resolution that falls linearly from the center
//!towards the edges.
//!This is achieved by scaling the w coordinate of the vertices in the
//!clip space before perspective divide.
//!The clip space w coordinate of the vertices **can** be offset as of a
//!function of x and y coordinates as follows:w' = w +  Ax +  ByIn the intended use case for
//! viewport position scaling, an application
//!should use a set of four viewports, one for each of the four quadrants of a
//!Cartesian coordinate system.
//!Each viewport is set to the dimension of the image, but is scissored to the
//!quadrant it represents.
//!The application should specify A and B coefficients of the
//!w-scaling equation above, that have the same value, but different
//!signs, for each of the viewports.
//!The signs of A and B should match the signs of x and
//!y for the quadrant that they represent such that the value of w'
//!will always be greater than or equal to the original w value for the
//!entire image.
//!Since the offset to w, (Ax +  By), is always positive, and
//!increases with the absolute values of x and y, the effective
//!resolution will fall off linearly from the center of the image to its edges.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Eric Werness [ewerness-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_clip_space_w_scaling]
//!   @ewerness-nv%0A<<Here describe the issue or question you have about the
//!   VK_NV_clip_space_w_scaling extension>>)
//!# New functions & commands
//! - [`CmdSetViewportWScalingNV`]
//!# New structures
//! - [`ViewportWScalingNV`]
//! - Extending [`PipelineViewportStateCreateInfo`]:
//! - [`PipelineViewportWScalingStateCreateInfoNV`]
//!# New constants
//! - [`NV_CLIP_SPACE_W_SCALING_EXTENSION_NAME`]
//! - [`NV_CLIP_SPACE_W_SCALING_SPEC_VERSION`]
//! - Extending [`DynamicState`]:
//! - `VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV`
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV`
//!# Known issues & F.A.Q
//!1) Is the pipeline struct name too long?**RESOLVED**: It fits with the naming convention.2)
//! Separate W scaling section or fold into coordinate transformations?**RESOLVED**: Leaving it as
//! its own section for now.
//!# Version History
//! - Revision 1, 2017-02-15 (Eric Werness)
//! - Internal revisions
//!# Other info
//! * 2017-02-15
//!*
//! - Eric Werness, NVIDIA
//! - Kedarnath Thangudu, NVIDIA
//!# Related
//! - [`PipelineViewportWScalingStateCreateInfoNV`]
//! - [`ViewportWScalingNV`]
//! - [`CmdSetViewportWScalingNV`]
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
#[doc(alias = "VK_NV_CLIP_SPACE_W_SCALING_SPEC_VERSION")]
pub const NV_CLIP_SPACE_W_SCALING_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_CLIP_SPACE_W_SCALING_EXTENSION_NAME")]
pub const NV_CLIP_SPACE_W_SCALING_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_clip_space_w_scaling");
