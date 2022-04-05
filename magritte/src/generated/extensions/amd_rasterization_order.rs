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
//! - blending, logic op, and color write
//!This extension enables applications to opt into a relaxed, implementation
//!defined primitive rasterization order that may allow better parallel
//!processing of primitives and thus enabling higher primitive throughput.
//!It is applicable in cases where the primitive rasterization order is known
//!to not affect the output of the rendering or any differences caused by a
//!different rasterization order are not a concern from the point of view of
//!the application’s purpose.A few examples of cases when using the relaxed primitive rasterization
//! order
//!would not have an effect on the final rendering:
//! - If the primitives rendered are known to not overlap in framebuffer space.
//! - If depth testing is used with a comparison operator of `VK_COMPARE_OP_LESS`,
//!   `VK_COMPARE_OP_LESS_OR_EQUAL`, `VK_COMPARE_OP_GREATER`, or `VK_COMPARE_OP_GREATER_OR_EQUAL`,
//!   and the primitives rendered are known to not overlap in clip space.
//! - If depth testing is not used and blending is enabled for all attachments with a commutative
//!   blend operator.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Daniel Rakos [drakos-amd](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_AMD_rasterization_order]
//!   @drakos-amd%0A<<Here describe the issue or question you have about the
//!   VK_AMD_rasterization_order extension>>)
//!# New structures
//! - Extending [`PipelineRasterizationStateCreateInfo`]:  -
//!   [`PipelineRasterizationStateRasterizationOrderAMD`]
//!# New enums
//! - [`RasterizationOrderAMD`]
//!# New constants
//! - [`AMD_RASTERIZATION_ORDER_EXTENSION_NAME`]
//! - [`AMD_RASTERIZATION_ORDER_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD`
//!# Known issues & F.A.Q
//!1) How is this extension useful to application developers? **RESOLVED** : Allows them to
//! increase primitive throughput for cases when
//!strict API order rasterization is not important due to the nature of the
//!content, the configuration used, or the requirements towards the output of
//!the rendering.2) How does this extension interact with content optimizations aiming to
//!reduce overdraw by appropriately ordering the input primitives? **RESOLVED** : While the relaxed
//! rasterization order might somewhat limit the
//!effectiveness of such content optimizations, most of the benefits of it are
//!expected to be retained even when the relaxed rasterization order is used,
//!so applications  **should**  still apply these optimizations even if they intend
//!to use the extension.3) Are there any guarantees about the primitive rasterization order when
//!using the new relaxed mode? **RESOLVED** : No.
//!In this case the rasterization order is completely implementation-dependent,
//!but in practice it is expected to partially still follow the order of
//!incoming primitives.4) Does the new relaxed rasterization order have any adverse effect on
//!repeatability and other invariance rules of the API? **RESOLVED** : Yes, in the sense that it
//! extends the list of exceptions when
//!the repeatability requirement does not apply.
//!# Version History
//! - Revision 1, 2016-04-25 (Daniel Rakos)  - Initial draft.
//!# Other info
//! * 2016-04-25
//! * No known IP claims.
//! * - Matthaeus G. Chajdas, AMD  - Jaakko Konttinen, AMD  - Daniel Rakos, AMD  - Graham Sellers,
//!   AMD  - Dominik Witczak, AMD
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
use crate::vulkan1_0::{BaseInStructure, StructureType};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{ffi::CStr, marker::PhantomData};
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
/// - [`RasterizationOrderStrictAmd`] specifies that operations for each primitive in a subpass  **must**  occur in [primitive order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-primitive-order).
/// - [`RasterizationOrderRelaxedAmd`] specifies that operations for each primitive in a subpass  **may**  not occur in [primitive order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-primitive-order).
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(i32)]
pub enum RasterizationOrderAMD {
    ///[`RasterizationOrderStrictAmd`] specifies that operations for
    ///each primitive in a subpass  **must**  occur in [primitive order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-primitive-order).
    RasterizationOrderStrictAmd = 0,
    ///[`RasterizationOrderRelaxedAmd`] specifies that operations for
    ///each primitive in a subpass  **may**  not occur in [primitive order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-primitive-order).
    RasterizationOrderRelaxedAmd = 1,
}
impl const Default for RasterizationOrderAMD {
    fn default() -> Self {
        Self::RasterizationOrderStrictAmd
    }
}
impl RasterizationOrderAMD {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        *self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
    }
}
///[VkPipelineRasterizationStateRasterizationOrderAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationStateRasterizationOrderAMD.html) - Structure defining rasterization order for a graphics pipeline
///# C Specifications
///The rasterization order to use for a graphics pipeline is specified by
///adding a [`PipelineRasterizationStateRasterizationOrderAMD`] structure
///to the [`p_next`] chain of a [`PipelineRasterizationStateCreateInfo`]
///structure.The [`PipelineRasterizationStateRasterizationOrderAMD`] structure is
///defined as:
///```c
///// Provided by VK_AMD_rasterization_order
///typedef struct VkPipelineRasterizationStateRasterizationOrderAMD {
///    VkStructureType            sType;
///    const void*                pNext;
///    VkRasterizationOrderAMD    rasterizationOrder;
///} VkPipelineRasterizationStateRasterizationOrderAMD;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`rasterization_order`] is a [`RasterizationOrderAMD`] value specifying the primitive
///   rasterization order to use.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD`
/// - [`rasterization_order`] **must**  be a valid [`RasterizationOrderAMD`] value
///If the `[`VK_AMD_rasterization_order`]` device extension is not enabled
///or the application does not request a particular rasterization order through
///specifying a [`PipelineRasterizationStateRasterizationOrderAMD`]
///structure then the rasterization order used by the graphics pipeline
///defaults to `VK_RASTERIZATION_ORDER_STRICT_AMD`.
///# Related
/// - [`VK_AMD_rasterization_order`]
/// - [`RasterizationOrderAMD`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPipelineRasterizationStateRasterizationOrderAMD")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PipelineRasterizationStateRasterizationOrderAMD<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`rasterization_order`] is a [`RasterizationOrderAMD`] value
    ///specifying the primitive rasterization order to use.
    pub rasterization_order: RasterizationOrderAMD,
}
impl<'lt> Default for PipelineRasterizationStateRasterizationOrderAMD<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PipelineRasterizationStateRasterizationOrderAmd,
            p_next: std::ptr::null(),
            rasterization_order: Default::default(),
        }
    }
}
impl<'lt> PipelineRasterizationStateRasterizationOrderAMD<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::rasterization_order`]
    pub fn rasterization_order(&self) -> RasterizationOrderAMD {
        self.rasterization_order
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::rasterization_order`]
    pub fn rasterization_order_mut(&mut self) -> &mut RasterizationOrderAMD {
        &mut self.rasterization_order
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::rasterization_order`]
    pub fn set_rasterization_order(
        mut self,
        value: crate::extensions::amd_rasterization_order::RasterizationOrderAMD,
    ) -> Self {
        self.rasterization_order = value;
        self
    }
}
