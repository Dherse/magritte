#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_CONSERVATIVE_RASTERIZATION_SPEC_VERSION")]
pub const EXT_CONSERVATIVE_RASTERIZATION_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_CONSERVATIVE_RASTERIZATION_EXTENSION_NAME")]
pub const EXT_CONSERVATIVE_RASTERIZATION_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_EXT_conservative_rasterization");
///[VkConservativeRasterizationModeEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkConservativeRasterizationModeEXT.html) - Specify the conservative rasterization mode
///# C Specifications
///Possible values of
///[`PipelineRasterizationConservativeStateCreateInfoEXT::conservative_rasterization_mode`],
///specifying the conservative rasterization mode are:
///```c
///// Provided by VK_EXT_conservative_rasterization
///typedef enum VkConservativeRasterizationModeEXT {
///    VK_CONSERVATIVE_RASTERIZATION_MODE_DISABLED_EXT = 0,
///    VK_CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE_EXT = 1,
///    VK_CONSERVATIVE_RASTERIZATION_MODE_UNDERESTIMATE_EXT = 2,
///} VkConservativeRasterizationModeEXT;
///```
///# Description
/// - [`CONSERVATIVE_RASTERIZATION_MODE_DISABLED`] specifies that
///conservative rasterization is disabled and rasterization proceeds as
///normal.
/// - [`CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE`] specifies that
///conservative rasterization is enabled in overestimation mode.
/// - [`CONSERVATIVE_RASTERIZATION_MODE_UNDERESTIMATE`] specifies
///that conservative rasterization is enabled in underestimation mode.
///# Related
/// - [`VK_EXT_conservative_rasterization`]
/// - [`PipelineRasterizationConservativeStateCreateInfoEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkConservativeRasterizationModeEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ConservativeRasterizationModeEXT(i32);
impl const Default for ConservativeRasterizationModeEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for ConservativeRasterizationModeEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("ConservativeRasterizationModeEXT")
            .field(match *self {
                Self::CONSERVATIVE_RASTERIZATION_MODE_DISABLED => &"CONSERVATIVE_RASTERIZATION_MODE_DISABLED",
                Self::CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE => &"CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE",
                Self::CONSERVATIVE_RASTERIZATION_MODE_UNDERESTIMATE => &"CONSERVATIVE_RASTERIZATION_MODE_UNDERESTIMATE",
                other => unreachable!("invalid value for `ConservativeRasterizationModeEXT`: {:?}", other),
            })
            .finish()
    }
}
impl ConservativeRasterizationModeEXT {
    ///[`CONSERVATIVE_RASTERIZATION_MODE_DISABLED`] specifies that
    ///conservative rasterization is disabled and rasterization proceeds as
    ///normal.
    pub const CONSERVATIVE_RASTERIZATION_MODE_DISABLED: Self = Self(0);
    ///[`CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE`] specifies that
    ///conservative rasterization is enabled in overestimation mode.
    pub const CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE: Self = Self(1);
    ///[`CONSERVATIVE_RASTERIZATION_MODE_UNDERESTIMATE`] specifies
    ///that conservative rasterization is enabled in underestimation mode.
    pub const CONSERVATIVE_RASTERIZATION_MODE_UNDERESTIMATE: Self = Self(2);
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
