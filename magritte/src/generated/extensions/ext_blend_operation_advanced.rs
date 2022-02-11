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
