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
///## Valid Usage (Implicit)
/// - [`x`] **must**  be a valid [`ViewportCoordinateSwizzleNV`] value
/// - [`y`] **must**  be a valid [`ViewportCoordinateSwizzleNV`] value
/// - [`z`] **must**  be a valid [`ViewportCoordinateSwizzleNV`] value
/// - [`w`] **must**  be a valid [`ViewportCoordinateSwizzleNV`] value
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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
impl Default for ViewportSwizzleNV {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
            w: Default::default(),
        }
    }
}
impl ViewportSwizzleNV {
    ///Gets the value of [`Self::x`]
    pub fn x(&self) -> ViewportCoordinateSwizzleNV {
        self.x
    }
    ///Gets the value of [`Self::y`]
    pub fn y(&self) -> ViewportCoordinateSwizzleNV {
        self.y
    }
    ///Gets the value of [`Self::z`]
    pub fn z(&self) -> ViewportCoordinateSwizzleNV {
        self.z
    }
    ///Gets the value of [`Self::w`]
    pub fn w(&self) -> ViewportCoordinateSwizzleNV {
        self.w
    }
    ///Gets a mutable reference to the value of [`Self::x`]
    pub fn x_mut(&mut self) -> &mut ViewportCoordinateSwizzleNV {
        &mut self.x
    }
    ///Gets a mutable reference to the value of [`Self::y`]
    pub fn y_mut(&mut self) -> &mut ViewportCoordinateSwizzleNV {
        &mut self.y
    }
    ///Gets a mutable reference to the value of [`Self::z`]
    pub fn z_mut(&mut self) -> &mut ViewportCoordinateSwizzleNV {
        &mut self.z
    }
    ///Gets a mutable reference to the value of [`Self::w`]
    pub fn w_mut(&mut self) -> &mut ViewportCoordinateSwizzleNV {
        &mut self.w
    }
    ///Sets the raw value of [`Self::x`]
    pub fn set_x(&mut self, value: crate::extensions::nv_viewport_swizzle::ViewportCoordinateSwizzleNV) -> &mut Self {
        self.x = value;
        self
    }
    ///Sets the raw value of [`Self::y`]
    pub fn set_y(&mut self, value: crate::extensions::nv_viewport_swizzle::ViewportCoordinateSwizzleNV) -> &mut Self {
        self.y = value;
        self
    }
    ///Sets the raw value of [`Self::z`]
    pub fn set_z(&mut self, value: crate::extensions::nv_viewport_swizzle::ViewportCoordinateSwizzleNV) -> &mut Self {
        self.z = value;
        self
    }
    ///Sets the raw value of [`Self::w`]
    pub fn set_w(&mut self, value: crate::extensions::nv_viewport_swizzle::ViewportCoordinateSwizzleNV) -> &mut Self {
        self.w = value;
        self
    }
}
///[VkPipelineViewportSwizzleStateCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportSwizzleStateCreateInfoNV.html) - Structure specifying swizzle applied to primitive clip coordinates
///# C Specifications
///Each primitive sent to a given viewport has a swizzle and  **optional**  negation
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
/// - [`viewport_swizzles`] is a pointer to an array of [`ViewportSwizzleNV`] structures, defining
///   the viewport swizzles.
///# Description
///## Valid Usage
/// - [`viewport_count`] **must**  be greater than or equal to the [`viewport_count`] set in
///   [`PipelineViewportStateCreateInfo`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV`
/// - [`flags`] **must**  be `0`
/// - [`viewport_swizzles`] **must**  be a valid pointer to an array of [`viewport_count`] valid
///   [`ViewportSwizzleNV`] structures
/// - [`viewport_count`] **must**  be greater than `0`
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PipelineViewportSwizzleStateCreateInfoNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: PipelineViewportSwizzleStateCreateFlagsNV,
    ///[`viewport_count`] is the number of viewport swizzles used by the
    ///pipeline.
    viewport_count: u32,
    ///[`viewport_swizzles`] is a pointer to an array of
    ///[`ViewportSwizzleNV`] structures, defining the viewport swizzles.
    viewport_swizzles: *const ViewportSwizzleNV,
}
impl<'lt> Default for PipelineViewportSwizzleStateCreateInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
            viewport_count: 0,
            viewport_swizzles: std::ptr::null(),
        }
    }
}
impl<'lt> PipelineViewportSwizzleStateCreateInfoNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::viewport_swizzles`]
    pub fn viewport_swizzles_raw(&self) -> *const ViewportSwizzleNV {
        self.viewport_swizzles
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::viewport_swizzles`]
    pub fn set_viewport_swizzles_raw(&mut self, value: *const ViewportSwizzleNV) -> &mut Self {
        self.viewport_swizzles = value;
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
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> PipelineViewportSwizzleStateCreateFlagsNV {
        self.flags
    }
    ///Gets the value of [`Self::viewport_count`]
    pub fn viewport_count(&self) -> u32 {
        self.viewport_count
    }
    ///Gets the value of [`Self::viewport_swizzles`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn viewport_swizzles(&self) -> &[ViewportSwizzleNV] {
        std::slice::from_raw_parts(self.viewport_swizzles, self.viewport_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut PipelineViewportSwizzleStateCreateFlagsNV {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::viewport_count`]
    pub fn viewport_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::flags`]
    pub fn set_flags(
        &mut self,
        value: crate::extensions::nv_viewport_swizzle::PipelineViewportSwizzleStateCreateFlagsNV,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::viewport_count`]
    pub fn set_viewport_count(&mut self, value: u32) -> &mut Self {
        self.viewport_count = value;
        self
    }
    ///Sets the raw value of [`Self::viewport_swizzles`]
    pub fn set_viewport_swizzles(
        &mut self,
        value: &'lt [crate::extensions::nv_viewport_swizzle::ViewportSwizzleNV],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.viewport_swizzles = value.as_ptr();
        self.viewport_count = len_;
        self
    }
}
