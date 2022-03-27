use crate::vulkan1_0::{BaseInStructure, StructureType};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_VIEWPORT_SWIZZLE_SPEC_VERSION")]
pub const NV_VIEWPORT_SWIZZLE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_VIEWPORT_SWIZZLE_EXTENSION_NAME")]
pub const NV_VIEWPORT_SWIZZLE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_viewport_swizzle");
///[VkViewportCoordinateSwizzleNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkViewportCoordinateSwizzleNV.html) - Specify how a viewport coordinate is swizzled
///# C Specifications
///Possible values of the [`ViewportSwizzleNV::x`], `y`, `z`,
///and `w` members, specifying swizzling of the corresponding components of
///primitives, are:
///```c
///// Provided by VK_NV_viewport_swizzle
///typedef enum VkViewportCoordinateSwizzleNV {
///    VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_X_NV = 0,
///    VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_X_NV = 1,
///    VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Y_NV = 2,
///    VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Y_NV = 3,
///    VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Z_NV = 4,
///    VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Z_NV = 5,
///    VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_W_NV = 6,
///    VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_W_NV = 7,
///} VkViewportCoordinateSwizzleNV;
///```
///# Description
///These values are described in detail in [Viewport Swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#vertexpostproc-viewport-swizzle).
///# Related
/// - [`VK_NV_viewport_swizzle`]
/// - [`ViewportSwizzleNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkViewportCoordinateSwizzleNV")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum ViewportCoordinateSwizzleNV {
    ///No documentation found
    ViewportCoordinateSwizzlePositiveXNv = 0,
    ///No documentation found
    ViewportCoordinateSwizzleNegativeXNv = 1,
    ///No documentation found
    ViewportCoordinateSwizzlePositiveYNv = 2,
    ///No documentation found
    ViewportCoordinateSwizzleNegativeYNv = 3,
    ///No documentation found
    ViewportCoordinateSwizzlePositiveZNv = 4,
    ///No documentation found
    ViewportCoordinateSwizzleNegativeZNv = 5,
    ///No documentation found
    ViewportCoordinateSwizzlePositiveWNv = 6,
    ///No documentation found
    ViewportCoordinateSwizzleNegativeWNv = 7,
}
impl const Default for ViewportCoordinateSwizzleNV {
    fn default() -> Self {
        ViewportCoordinateSwizzlePositiveXNv
    }
}
impl ViewportCoordinateSwizzleNV {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
    }
}
///[VkViewportSwizzleNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkViewportSwizzleNV.html) - Structure specifying a viewport swizzle
///# C Specifications
///The [`ViewportSwizzleNV`] structure is defined as:
///```c
///// Provided by VK_NV_viewport_swizzle
///typedef struct VkViewportSwizzleNV {
///    VkViewportCoordinateSwizzleNV    x;
///    VkViewportCoordinateSwizzleNV    y;
///    VkViewportCoordinateSwizzleNV    z;
///    VkViewportCoordinateSwizzleNV    w;
///} VkViewportSwizzleNV;
///```
///# Members
/// - [`x`] is a [`ViewportCoordinateSwizzleNV`] value specifying the swizzle operation to apply to
///   the x component of the primitive
/// - [`y`] is a [`ViewportCoordinateSwizzleNV`] value specifying the swizzle operation to apply to
///   the y component of the primitive
/// - [`z`] is a [`ViewportCoordinateSwizzleNV`] value specifying the swizzle operation to apply to
///   the z component of the primitive
/// - [`w`] is a [`ViewportCoordinateSwizzleNV`] value specifying the swizzle operation to apply to
///   the w component of the primitive
///# Description
///Valid Usage (Implicit)
/// - [`x`]**must** be a valid [`ViewportCoordinateSwizzleNV`] value
/// - [`y`]**must** be a valid [`ViewportCoordinateSwizzleNV`] value
/// - [`z`]**must** be a valid [`ViewportCoordinateSwizzleNV`] value
/// - [`w`]**must** be a valid [`ViewportCoordinateSwizzleNV`] value
///# Related
/// - [`VK_NV_viewport_swizzle`]
/// - [`PipelineViewportSwizzleStateCreateInfoNV`]
/// - [`ViewportCoordinateSwizzleNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ViewportSwizzleNV {
    ///[`x`] is a [`ViewportCoordinateSwizzleNV`] value specifying the
    ///swizzle operation to apply to the x component of the primitive
    x: ViewportCoordinateSwizzleNV,
    ///[`y`] is a [`ViewportCoordinateSwizzleNV`] value specifying the
    ///swizzle operation to apply to the y component of the primitive
    y: ViewportCoordinateSwizzleNV,
    ///[`z`] is a [`ViewportCoordinateSwizzleNV`] value specifying the
    ///swizzle operation to apply to the z component of the primitive
    z: ViewportCoordinateSwizzleNV,
    ///[`w`] is a [`ViewportCoordinateSwizzleNV`] value specifying the
    ///swizzle operation to apply to the w component of the primitive
    w: ViewportCoordinateSwizzleNV,
}
///[VkPipelineViewportSwizzleStateCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportSwizzleStateCreateInfoNV.html) - Structure specifying swizzle applied to primitive clip coordinates
///# C Specifications
///Each primitive sent to a given viewport has a swizzle and **optional** negation
///applied to its clip coordinates.
///The swizzle that is applied depends on the viewport index, and is controlled
///by the [`PipelineViewportSwizzleStateCreateInfoNV`] pipeline state:
///```c
///// Provided by VK_NV_viewport_swizzle
///typedef struct VkPipelineViewportSwizzleStateCreateInfoNV {
///    VkStructureType                                sType;
///    const void*                                    pNext;
///    VkPipelineViewportSwizzleStateCreateFlagsNV    flags;
///    uint32_t                                       viewportCount;
///    const VkViewportSwizzleNV*                     pViewportSwizzles;
///} VkPipelineViewportSwizzleStateCreateInfoNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`viewport_count`] is the number of viewport swizzles used by the pipeline.
/// - [`p_viewport_swizzles`] is a pointer to an array of [`ViewportSwizzleNV`] structures, defining
///   the viewport swizzles.
///# Description
///Valid Usage
/// - [`viewport_count`]**must** be greater than or equal to the [`viewport_count`] set in
///   [`PipelineViewportStateCreateInfo`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV`
/// - [`flags`]**must** be `0`
/// - [`p_viewport_swizzles`]**must** be a valid pointer to an array of [`viewport_count`] valid
///   [`ViewportSwizzleNV`] structures
/// - [`viewport_count`]**must** be greater than `0`
///# Related
/// - [`VK_NV_viewport_swizzle`]
/// - [`PipelineViewportSwizzleStateCreateFlagsNV`]
/// - [`StructureType`]
/// - [`ViewportSwizzleNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PipelineViewportSwizzleStateCreateInfoNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: PipelineViewportSwizzleStateCreateFlagsNV,
    ///[`viewport_count`] is the number of viewport swizzles used by the
    ///pipeline.
    viewport_count: u32,
    ///[`p_viewport_swizzles`] is a pointer to an array of
    ///[`ViewportSwizzleNV`] structures, defining the viewport swizzles.
    p_viewport_swizzles: *mut ViewportSwizzleNV,
}
