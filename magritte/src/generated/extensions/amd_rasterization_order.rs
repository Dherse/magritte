#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_RASTERIZATION_ORDER_SPEC_VERSION")]
pub const AMD_RASTERIZATION_ORDER_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_RASTERIZATION_ORDER_EXTENSION_NAME")]
pub const AMD_RASTERIZATION_ORDER_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_AMD_rasterization_order");
///[VkRasterizationOrderAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRasterizationOrderAMD.html) - Specify rasterization order for a graphics pipeline
///# C Specifications
///Possible values of
///[`PipelineRasterizationStateRasterizationOrderAMD::rasterization_order`],
///specifying the primitive rasterization order, are:
///```c
///// Provided by VK_AMD_rasterization_order
///typedef enum VkRasterizationOrderAMD {
///    VK_RASTERIZATION_ORDER_STRICT_AMD = 0,
///    VK_RASTERIZATION_ORDER_RELAXED_AMD = 1,
///} VkRasterizationOrderAMD;
///```
///# Description
/// - [`RASTERIZATION_ORDER_STRICT`] specifies that operations for
///each primitive in a subpass **must** occur in [primitive order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-primitive-order).
/// - [`RASTERIZATION_ORDER_RELAXED`] specifies that operations for
///each primitive in a subpass **may** not occur in [primitive order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-primitive-order).
///# Related
/// - [`VK_AMD_rasterization_order`]
/// - [`PipelineRasterizationStateRasterizationOrderAMD`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkRasterizationOrderAMD")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct RasterizationOrderAMD(i32);
impl const Default for RasterizationOrderAMD {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for RasterizationOrderAMD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("RasterizationOrderAMD")
            .field(match *self {
                Self::RASTERIZATION_ORDER_STRICT => &"RASTERIZATION_ORDER_STRICT",
                Self::RASTERIZATION_ORDER_RELAXED => &"RASTERIZATION_ORDER_RELAXED",
                other => unreachable!("invalid value for `RasterizationOrderAMD`: {:?}", other),
            })
            .finish()
    }
}
impl RasterizationOrderAMD {
    ///[`RASTERIZATION_ORDER_STRICT`] specifies that operations for
    ///each primitive in a subpass **must** occur in [primitive order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-primitive-order).
    pub const RASTERIZATION_ORDER_STRICT: Self = Self(0);
    ///[`RASTERIZATION_ORDER_RELAXED`] specifies that operations for
    ///each primitive in a subpass **may** not occur in [primitive order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-primitive-order).
    pub const RASTERIZATION_ORDER_RELAXED: Self = Self(1);
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
