//![VK_EXT_blend_operation_advanced](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_blend_operation_advanced.html) - device extension
//!# Description
//!This extension adds a number of “advanced” blending operations that **can**
//!be used to perform new color blending operations, many of which are more
//!complex than the standard blend modes provided by unextended Vulkan.
//!This extension requires different styles of usage, depending on the level of
//!hardware support and the enabled features:
//! - If
//![`PhysicalDeviceBlendOperationAdvancedFeaturesEXT::advanced_blend_coherent_operations`]
//!is [`FALSE`], the new blending operations are supported, but a
//!memory dependency **must** separate each advanced blend operation on a
//!given sample.
//!`VK_ACCESS_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT` is used to
//!synchronize reads using advanced blend operations.
//! - If
//![`PhysicalDeviceBlendOperationAdvancedFeaturesEXT::advanced_blend_coherent_operations`]
//!is [`TRUE`], advanced blend operations obey primitive order just
//!like basic blend operations.
//!In unextended Vulkan, the set of blending operations is limited, and **can** be
//!expressed very simply.
//!The `VK_BLEND_OP_MIN` and `VK_BLEND_OP_MAX` blend operations simply
//!compute component-wise minimums or maximums of source and destination color
//!components.
//!The `VK_BLEND_OP_ADD`, `VK_BLEND_OP_SUBTRACT`, and
//!`VK_BLEND_OP_REVERSE_SUBTRACT` modes multiply the source and destination
//!colors by source and destination factors and either add the two products
//!together or subtract one from the other.
//!This limited set of operations supports many common blending operations but
//!precludes the use of more sophisticated transparency and blending operations
//!commonly available in many dedicated imaging APIs.This extension provides a number of new
//! “advanced” blending operations.
//!Unlike traditional blending operations using `VK_BLEND_OP_ADD`, these
//!blending equations do not use source and destination factors specified by
//![`BlendFactor`].
//!Instead, each blend operation specifies a complete equation based on the
//!source and destination colors.
//!These new blend operations are used for both RGB and alpha components; they
//!**must** not be used to perform separate RGB and alpha blending (via different
//!values of color and alpha [`BlendOp`]).These blending operations are performed using
//! premultiplied colors, where
//!RGB colors **can** be considered premultiplied or non-premultiplied by alpha,
//!according to the `srcPremultiplied` and `dstPremultiplied` members
//!of [`PipelineColorBlendAdvancedStateCreateInfoEXT`].
//!If a color is considered non-premultiplied, the (R,G,B) color components are
//!multiplied by the alpha component prior to blending.
//!For non-premultiplied color components in the range [0,1], the
//!corresponding premultiplied color component would have values in the range
//![0 × A, 1 × A].Many of these advanced blending equations are formulated where the result of
//!blending source and destination colors with partial coverage have three
//!separate contributions: from the portions covered by both the source and the
//!destination, from the portion covered only by the source, and from the
//!portion covered only by the destination.
//!The blend parameter
//![`PipelineColorBlendAdvancedStateCreateInfoEXT::blend_overlap`]**can** be used to specify a
//! correlation between source and destination pixel
//!coverage.
//!If set to `VK_BLEND_OVERLAP_CONJOINT_EXT`, the source and destination
//!are considered to have maximal overlap, as would be the case if drawing two
//!objects on top of each other.
//!If set to `VK_BLEND_OVERLAP_DISJOINT_EXT`, the source and destination
//!are considered to have minimal overlap, as would be the case when rendering
//!a complex polygon tessellated into individual non-intersecting triangles.
//!If set to `VK_BLEND_OVERLAP_UNCORRELATED_EXT`, the source and
//!destination coverage are assumed to have no spatial correlation within the
//!pixel.In addition to the coherency issues on implementations not supporting
//!`advancedBlendCoherentOperations`, this extension has several
//!limitations worth noting.
//!First, the new blend operations have a limit on the number of color
//!attachments they **can** be used with, as indicated by
//![`PhysicalDeviceBlendOperationAdvancedPropertiesEXT::advanced_blend_max_color_attachments`].
//!Additionally, blending precision **may** be limited to 16-bit floating-point,
//!which **may** result in a loss of precision and dynamic range for framebuffer
//!formats with 32-bit floating-point components, and in a loss of precision
//!for formats with 12- and 16-bit signed or unsigned normalized integer
//!components.
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_blend_operation_advanced]
//!   @jeffbolznv%0A<<Here describe the issue or question you have about the
//!   VK_EXT_blend_operation_advanced extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceBlendOperationAdvancedFeaturesEXT`]
//!
//! - Extending [`PhysicalDeviceProperties2`]:
//! - [`PhysicalDeviceBlendOperationAdvancedPropertiesEXT`]
//!
//! - Extending [`PipelineColorBlendStateCreateInfo`]:
//! - [`PipelineColorBlendAdvancedStateCreateInfoEXT`]
//!# New enums
//! - [`BlendOverlapEXT`]
//!# New constants
//! - [`EXT_BLEND_OPERATION_ADVANCED_EXTENSION_NAME`]
//! - [`EXT_BLEND_OPERATION_ADVANCED_SPEC_VERSION`]
//! - Extending [`AccessFlagBits`]:
//! - `VK_ACCESS_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT`
//!
//! - Extending [`BlendOp`]:
//! - `VK_BLEND_OP_BLUE_EXT`
//! - `VK_BLEND_OP_COLORBURN_EXT`
//! - `VK_BLEND_OP_COLORDODGE_EXT`
//! - `VK_BLEND_OP_CONTRAST_EXT`
//! - `VK_BLEND_OP_DARKEN_EXT`
//! - `VK_BLEND_OP_DIFFERENCE_EXT`
//! - `VK_BLEND_OP_DST_ATOP_EXT`
//! - `VK_BLEND_OP_DST_EXT`
//! - `VK_BLEND_OP_DST_IN_EXT`
//! - `VK_BLEND_OP_DST_OUT_EXT`
//! - `VK_BLEND_OP_DST_OVER_EXT`
//! - `VK_BLEND_OP_EXCLUSION_EXT`
//! - `VK_BLEND_OP_GREEN_EXT`
//! - `VK_BLEND_OP_HARDLIGHT_EXT`
//! - `VK_BLEND_OP_HARDMIX_EXT`
//! - `VK_BLEND_OP_HSL_COLOR_EXT`
//! - `VK_BLEND_OP_HSL_HUE_EXT`
//! - `VK_BLEND_OP_HSL_LUMINOSITY_EXT`
//! - `VK_BLEND_OP_HSL_SATURATION_EXT`
//! - `VK_BLEND_OP_INVERT_EXT`
//! - `VK_BLEND_OP_INVERT_OVG_EXT`
//! - `VK_BLEND_OP_INVERT_RGB_EXT`
//! - `VK_BLEND_OP_LIGHTEN_EXT`
//! - `VK_BLEND_OP_LINEARBURN_EXT`
//! - `VK_BLEND_OP_LINEARDODGE_EXT`
//! - `VK_BLEND_OP_LINEARLIGHT_EXT`
//! - `VK_BLEND_OP_MINUS_CLAMPED_EXT`
//! - `VK_BLEND_OP_MINUS_EXT`
//! - `VK_BLEND_OP_MULTIPLY_EXT`
//! - `VK_BLEND_OP_OVERLAY_EXT`
//! - `VK_BLEND_OP_PINLIGHT_EXT`
//! - `VK_BLEND_OP_PLUS_CLAMPED_ALPHA_EXT`
//! - `VK_BLEND_OP_PLUS_CLAMPED_EXT`
//! - `VK_BLEND_OP_PLUS_DARKER_EXT`
//! - `VK_BLEND_OP_PLUS_EXT`
//! - `VK_BLEND_OP_RED_EXT`
//! - `VK_BLEND_OP_SCREEN_EXT`
//! - `VK_BLEND_OP_SOFTLIGHT_EXT`
//! - `VK_BLEND_OP_SRC_ATOP_EXT`
//! - `VK_BLEND_OP_SRC_EXT`
//! - `VK_BLEND_OP_SRC_IN_EXT`
//! - `VK_BLEND_OP_SRC_OUT_EXT`
//! - `VK_BLEND_OP_SRC_OVER_EXT`
//! - `VK_BLEND_OP_VIVIDLIGHT_EXT`
//! - `VK_BLEND_OP_XOR_EXT`
//! - `VK_BLEND_OP_ZERO_EXT`
//!
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT`
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT`
//! - `VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT`
//!# Known issues & F.A.Q
//!None.
//!# Version History
//! - Revision 1, 2017-06-12 (Jeff Bolz)
//! - Internal revisions
//!
//! - Revision 2, 2017-06-12 (Jeff Bolz)
//! - Internal revisions
//!# Other info
//! * 2017-06-12
//!*
//! - Jeff Bolz, NVIDIA
//!# Related
//! - [`BlendOverlapEXT`]
//! - [`PhysicalDeviceBlendOperationAdvancedFeaturesEXT`]
//! - [`PhysicalDeviceBlendOperationAdvancedPropertiesEXT`]
//! - [`PipelineColorBlendAdvancedStateCreateInfoEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_BLEND_OPERATION_ADVANCED_SPEC_VERSION")]
pub const EXT_BLEND_OPERATION_ADVANCED_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_BLEND_OPERATION_ADVANCED_EXTENSION_NAME")]
pub const EXT_BLEND_OPERATION_ADVANCED_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_blend_operation_advanced");
///[VkBlendOverlapEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBlendOverlapEXT.html) - Enumerant specifying the blend overlap parameter
///# C Specifications
///The weighting functions p<sub>0</sub>, p<sub>1</sub>, and p<sub>2</sub> are defined
///in table [Advanced Blend Overlap
///Modes](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blend-advanced-overlap-modes).
///In these functions, the A components of the source and destination colors
///are taken to indicate the portion of the pixel covered by the fragment
///(source) and the fragments previously accumulated in the pixel
///(destination).
///The functions p<sub>0</sub>, p<sub>1</sub>, and p<sub>2</sub> approximate the
///relative portion of the pixel covered by the intersection of the source and
///destination, covered only by the source, and covered only by the
///destination, respectively.Possible values of
///[`PipelineColorBlendAdvancedStateCreateInfoEXT::blend_overlap`],
///specifying the blend overlap functions, are:
///```c
///// Provided by VK_EXT_blend_operation_advanced
///typedef enum VkBlendOverlapEXT {
///    VK_BLEND_OVERLAP_UNCORRELATED_EXT = 0,
///    VK_BLEND_OVERLAP_DISJOINT_EXT = 1,
///    VK_BLEND_OVERLAP_CONJOINT_EXT = 2,
///} VkBlendOverlapEXT;
///```
///# Description
/// - [`BLEND_OVERLAP_UNCORRELATED`] specifies that there is no
///correlation between the source and destination coverage.
/// - [`BLEND_OVERLAP_CONJOINT`] specifies that the source and
///destination coverage are considered to have maximal overlap.
/// - [`BLEND_OVERLAP_DISJOINT`] specifies that the source and
///destination coverage are considered to have minimal overlap.
///# Related
/// - [`VK_EXT_blend_operation_advanced`]
/// - [`PipelineColorBlendAdvancedStateCreateInfoEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkBlendOverlapEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct BlendOverlapEXT(i32);
impl const Default for BlendOverlapEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for BlendOverlapEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("BlendOverlapEXT")
            .field(match *self {
                Self::BLEND_OVERLAP_UNCORRELATED => &"BLEND_OVERLAP_UNCORRELATED",
                Self::BLEND_OVERLAP_DISJOINT => &"BLEND_OVERLAP_DISJOINT",
                Self::BLEND_OVERLAP_CONJOINT => &"BLEND_OVERLAP_CONJOINT",
                other => unreachable!("invalid value for `BlendOverlapEXT`: {:?}", other),
            })
            .finish()
    }
}
impl BlendOverlapEXT {
    ///[`BLEND_OVERLAP_UNCORRELATED`] specifies that there is no
    ///correlation between the source and destination coverage.
    pub const BLEND_OVERLAP_UNCORRELATED: Self = Self(0);
    ///[`BLEND_OVERLAP_DISJOINT`] specifies that the source and
    ///destination coverage are considered to have minimal overlap.
    pub const BLEND_OVERLAP_DISJOINT: Self = Self(1);
    ///[`BLEND_OVERLAP_CONJOINT`] specifies that the source and
    ///destination coverage are considered to have maximal overlap.
    pub const BLEND_OVERLAP_CONJOINT: Self = Self(2);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
}
