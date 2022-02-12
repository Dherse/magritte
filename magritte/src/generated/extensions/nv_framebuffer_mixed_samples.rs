//![VK_NV_framebuffer_mixed_samples](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_framebuffer_mixed_samples.html) - device extension
//!# Description
//!This extension allows multisample rendering with a raster and depth/stencil
//!sample count that is larger than the color sample count.
//!Rasterization and the results of the depth and stencil tests together
//!determine the portion of a pixel that is “covered”.
//!It can be useful to evaluate coverage at a higher frequency than color
//!samples are stored.
//!This coverage is then “reduced” to a collection of covered color samples,
//!each having an opacity value corresponding to the fraction of the color
//!sample covered.
//!The opacity can optionally be blended into individual color samples.Rendering with fewer color
//! samples than depth/stencil samples greatly
//!reduces the amount of memory and bandwidth consumed by the color buffer.
//!However, converting the coverage values into opacity introduces artifacts
//!where triangles share edges and **may** not be suitable for normal triangle
//!mesh rendering.One expected use case for this functionality is Stencil-then-Cover path
//!rendering (similar to the OpenGL GL_NV_path_rendering extension).
//!The stencil step determines the coverage (in the stencil buffer) for an
//!entire path at the higher sample frequency, and then the cover step draws
//!the path into the lower frequency color buffer using the coverage
//!information to antialias path edges.
//!With this two-step process, internal edges are fully covered when
//!antialiasing is applied and there is no corruption on these edges.The key features of this
//! extension are:
//! - It allows render pass and framebuffer objects to be created where the
//!number of samples in the depth/stencil attachment in a subpass is a
//!multiple of the number of samples in the color attachments in the
//!subpass.
//! - A coverage reduction step is added to Fragment Operations which converts
//!a set of covered raster/depth/stencil samples to a set of color samples
//!that perform blending and color writes.
//!The coverage reduction step also includes an optional coverage
//!modulation step, multiplying color values by a fractional opacity
//!corresponding to the number of associated raster/depth/stencil samples
//!covered.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_framebuffer_mixed_samples]
//!   @jeffbolznv%0A<<Here describe the issue or question you have about the
//!   VK_NV_framebuffer_mixed_samples extension>>)
//!# New structures
//! - Extending [`PipelineMultisampleStateCreateInfo`]:
//! - [`PipelineCoverageModulationStateCreateInfoNV`]
//!# New enums
//! - [`CoverageModulationModeNV`]
//!# New bitmasks
//! - [`PipelineCoverageModulationStateCreateFlagsNV`]
//!# New constants
//! - [`NV_FRAMEBUFFER_MIXED_SAMPLES_EXTENSION_NAME`]
//! - [`NV_FRAMEBUFFER_MIXED_SAMPLES_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV`
//!# Version History
//! - Revision 1, 2017-06-04 (Jeff Bolz)
//! - Internal revisions
//!# Other info
//! * 2017-06-04
//!*
//! - Jeff Bolz, NVIDIA
//!# Related
//! - [`CoverageModulationModeNV`]
//! - [`PipelineCoverageModulationStateCreateFlagsNV`]
//! - [`PipelineCoverageModulationStateCreateInfoNV`]
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
#[doc(alias = "VK_NV_FRAMEBUFFER_MIXED_SAMPLES_SPEC_VERSION")]
pub const NV_FRAMEBUFFER_MIXED_SAMPLES_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_FRAMEBUFFER_MIXED_SAMPLES_EXTENSION_NAME")]
pub const NV_FRAMEBUFFER_MIXED_SAMPLES_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_framebuffer_mixed_samples");
///[VkCoverageModulationModeNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCoverageModulationModeNV.html) - Specify the coverage modulation mode
///# C Specifications
///Possible values of
///[`PipelineCoverageModulationStateCreateInfoNV::coverage_modulation_mode`],
///specifying which color components are modulated, are:
///```c
///// Provided by VK_NV_framebuffer_mixed_samples
///typedef enum VkCoverageModulationModeNV {
///    VK_COVERAGE_MODULATION_MODE_NONE_NV = 0,
///    VK_COVERAGE_MODULATION_MODE_RGB_NV = 1,
///    VK_COVERAGE_MODULATION_MODE_ALPHA_NV = 2,
///    VK_COVERAGE_MODULATION_MODE_RGBA_NV = 3,
///} VkCoverageModulationModeNV;
///```
///# Description
/// - [`COVERAGE_MODULATION_MODE_NONE`] specifies that no components
///are multiplied by the modulation factor.
/// - [`COVERAGE_MODULATION_MODE_RGB`] specifies that the red, green,
///and blue components are multiplied by the modulation factor.
/// - [`COVERAGE_MODULATION_MODE_ALPHA`] specifies that the alpha
///component is multiplied by the modulation factor.
/// - [`COVERAGE_MODULATION_MODE_RGBA`] specifies that all components
///are multiplied by the modulation factor.
///# Related
/// - [`VK_NV_framebuffer_mixed_samples`]
/// - [`PipelineCoverageModulationStateCreateInfoNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkCoverageModulationModeNV")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct CoverageModulationModeNV(i32);
impl const Default for CoverageModulationModeNV {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for CoverageModulationModeNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("CoverageModulationModeNV")
            .field(match *self {
                Self::COVERAGE_MODULATION_MODE_NONE => &"COVERAGE_MODULATION_MODE_NONE",
                Self::COVERAGE_MODULATION_MODE_RGB => &"COVERAGE_MODULATION_MODE_RGB",
                Self::COVERAGE_MODULATION_MODE_ALPHA => &"COVERAGE_MODULATION_MODE_ALPHA",
                Self::COVERAGE_MODULATION_MODE_RGBA => &"COVERAGE_MODULATION_MODE_RGBA",
                other => unreachable!("invalid value for `CoverageModulationModeNV`: {:?}", other),
            })
            .finish()
    }
}
impl CoverageModulationModeNV {
    ///[`COVERAGE_MODULATION_MODE_NONE`] specifies that no components
    ///are multiplied by the modulation factor.
    pub const COVERAGE_MODULATION_MODE_NONE: Self = Self(0);
    ///[`COVERAGE_MODULATION_MODE_RGB`] specifies that the red, green,
    ///and blue components are multiplied by the modulation factor.
    pub const COVERAGE_MODULATION_MODE_RGB: Self = Self(1);
    ///[`COVERAGE_MODULATION_MODE_ALPHA`] specifies that the alpha
    ///component is multiplied by the modulation factor.
    pub const COVERAGE_MODULATION_MODE_ALPHA: Self = Self(2);
    ///[`COVERAGE_MODULATION_MODE_RGBA`] specifies that all components
    ///are multiplied by the modulation factor.
    pub const COVERAGE_MODULATION_MODE_RGBA: Self = Self(3);
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
