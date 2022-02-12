//![VK_AMD_rasterization_order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_AMD_rasterization_order.html) - device extension
//!# Description
//!This extension introduces the possibility for the application to control the
//!order of primitive rasterization.
//!In unextended Vulkan, the following stages are guaranteed to execute in *API
//!order*:
//! - depth bounds test
//! - stencil test, stencil op, and stencil write
//! - depth test and depth write
//! - occlusion queries
//! - blending, logic op, and color writeThis extension enables applications to opt into a relaxed,
//!   implementation
//!defined primitive rasterization order that may allow better parallel
//!processing of primitives and thus enabling higher primitive throughput.
//!It is applicable in cases where the primitive rasterization order is known
//!to not affect the output of the rendering or any differences caused by a
//!different rasterization order are not a concern from the point of view of
//!the application’s purpose.A few examples of cases when using the relaxed primitive rasterization
//! order
//!would not have an effect on the final rendering:
//! - If the primitives rendered are known to not overlap in framebuffer
//!space.
//! - If depth testing is used with a comparison operator of
//!`VK_COMPARE_OP_LESS`, `VK_COMPARE_OP_LESS_OR_EQUAL`,
//!`VK_COMPARE_OP_GREATER`, or `VK_COMPARE_OP_GREATER_OR_EQUAL`,
//!and the primitives rendered are known to not overlap in clip space.
//! - If depth testing is not used and blending is enabled for all attachments
//!with a commutative blend operator.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Daniel Rakos [drakos-amd](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_AMD_rasterization_order]
//!   @drakos-amd%0A<<Here describe the issue or question you have about the
//!   VK_AMD_rasterization_order extension>>)
//!# New structures
//! - Extending [`PipelineRasterizationStateCreateInfo`]:
//! - [`PipelineRasterizationStateRasterizationOrderAMD`]
//!# New enums
//! - [`RasterizationOrderAMD`]
//!# New constants
//! - [`AMD_RASTERIZATION_ORDER_EXTENSION_NAME`]
//! - [`AMD_RASTERIZATION_ORDER_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD`
//!# Known issues & F.A.Q
//!1) How is this extension useful to application developers?**RESOLVED**: Allows them to increase
//! primitive throughput for cases when
//!strict API order rasterization is not important due to the nature of the
//!content, the configuration used, or the requirements towards the output of
//!the rendering.2) How does this extension interact with content optimizations aiming to
//!reduce overdraw by appropriately ordering the input primitives?**RESOLVED**: While the relaxed
//! rasterization order might somewhat limit the
//!effectiveness of such content optimizations, most of the benefits of it are
//!expected to be retained even when the relaxed rasterization order is used,
//!so applications **should** still apply these optimizations even if they intend
//!to use the extension.3) Are there any guarantees about the primitive rasterization order when
//!using the new relaxed mode?**RESOLVED**: No.
//!In this case the rasterization order is completely implementation-dependent,
//!but in practice it is expected to partially still follow the order of
//!incoming primitives.4) Does the new relaxed rasterization order have any adverse effect on
//!repeatability and other invariance rules of the API?**RESOLVED**: Yes, in the sense that it
//! extends the list of exceptions when
//!the repeatability requirement does not apply.
//!# Version History
//! - Revision 1, 2016-04-25 (Daniel Rakos)
//! - Initial draft.
//!# Other info
//! * 2016-04-25
//! * No known IP claims.
//!*
//! - Matthaeus G. Chajdas, AMD
//! - Jaakko Konttinen, AMD
//! - Daniel Rakos, AMD
//! - Graham Sellers, AMD
//! - Dominik Witczak, AMD
//!# Related
//! - [`PipelineRasterizationStateRasterizationOrderAMD`]
//! - [`RasterizationOrderAMD`]
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
