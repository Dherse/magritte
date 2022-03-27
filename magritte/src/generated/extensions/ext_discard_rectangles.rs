use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Rect2D, StructureType};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DISCARD_RECTANGLES_SPEC_VERSION")]
pub const EXT_DISCARD_RECTANGLES_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DISCARD_RECTANGLES_EXTENSION_NAME")]
pub const EXT_DISCARD_RECTANGLES_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_discard_rectangles");
///[VkDiscardRectangleModeEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDiscardRectangleModeEXT.html) - Specify the discard rectangle mode
///# C Specifications
///[`DiscardRectangleModeEXT`] values are:
///```c
///// Provided by VK_EXT_discard_rectangles
///typedef enum VkDiscardRectangleModeEXT {
///    VK_DISCARD_RECTANGLE_MODE_INCLUSIVE_EXT = 0,
///    VK_DISCARD_RECTANGLE_MODE_EXCLUSIVE_EXT = 1,
///} VkDiscardRectangleModeEXT;
///```
///# Description
/// - [`DiscardRectangleModeInclusiveExt`] specifies that the discard rectangle test is inclusive.
/// - [`DiscardRectangleModeExclusiveExt`] specifies that the discard rectangle test is exclusive.
///# Related
/// - [`VK_EXT_discard_rectangles`]
/// - [`PipelineDiscardRectangleStateCreateInfoEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDiscardRectangleModeEXT")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum DiscardRectangleModeEXT {
    ///[`DiscardRectangleModeInclusiveExt`] specifies that the discard
    ///rectangle test is inclusive.
    DiscardRectangleModeInclusiveExt = 0,
    ///[`DiscardRectangleModeExclusiveExt`] specifies that the discard
    ///rectangle test is exclusive.
    DiscardRectangleModeExclusiveExt = 1,
}
impl const Default for DiscardRectangleModeEXT {
    fn default() -> Self {
        DiscardRectangleModeInclusiveExt
    }
}
impl DiscardRectangleModeEXT {
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
///[VkPhysicalDeviceDiscardRectanglePropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDiscardRectanglePropertiesEXT.html) - Structure describing discard rectangle limits that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceDiscardRectanglePropertiesEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_discard_rectangles
///typedef struct VkPhysicalDeviceDiscardRectanglePropertiesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           maxDiscardRectangles;
///} VkPhysicalDeviceDiscardRectanglePropertiesEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`max_discard_rectangles`] is the maximum number of active discard rectangles that **can** be
///   specified.
///# Description
///If the [`PhysicalDeviceDiscardRectanglePropertiesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT`
///# Related
/// - [`VK_EXT_discard_rectangles`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceDiscardRectanglePropertiesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`max_discard_rectangles`] is the
    ///maximum number of active discard rectangles that **can** be specified.
    max_discard_rectangles: u32,
}
///[VkPipelineDiscardRectangleStateCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineDiscardRectangleStateCreateInfoEXT.html) - Structure specifying discard rectangle
///# C Specifications
///The [`PipelineDiscardRectangleStateCreateInfoEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_discard_rectangles
///typedef struct VkPipelineDiscardRectangleStateCreateInfoEXT {
///    VkStructureType                                  sType;
///    const void*                                      pNext;
///    VkPipelineDiscardRectangleStateCreateFlagsEXT    flags;
///    VkDiscardRectangleModeEXT                        discardRectangleMode;
///    uint32_t                                         discardRectangleCount;
///    const VkRect2D*                                  pDiscardRectangles;
///} VkPipelineDiscardRectangleStateCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`discard_rectangle_mode`] is a [`DiscardRectangleModeEXT`] value determining whether the
///   discard rectangle test is inclusive or exclusive.
/// - [`discard_rectangle_count`] is the number of discard rectangles to use.
/// - [`p_discard_rectangles`] is a pointer to an array of [`Rect2D`] structures defining discard
///   rectangles.
///# Description
///If the `VK_DYNAMIC_STATE_DISCARD_RECTANGLE_EXT` dynamic state is enabled
///for a pipeline, the [`p_discard_rectangles`] member is ignored.When this structure is included
/// in the [`p_next`] chain of
///[`GraphicsPipelineCreateInfo`], it defines parameters of the discard
///rectangle test.
///If this structure is not included in the [`p_next`] chain, it is equivalent
///to specifying this structure with a [`discard_rectangle_count`] of `0`.Valid Usage
/// - [`discard_rectangle_count`]**must** be less than or equal to
///   [`PhysicalDeviceDiscardRectanglePropertiesEXT::max_discard_rectangles`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT`
/// - [`flags`]**must** be `0`
/// - [`discard_rectangle_mode`]**must** be a valid [`DiscardRectangleModeEXT`] value
///# Related
/// - [`VK_EXT_discard_rectangles`]
/// - [`DiscardRectangleModeEXT`]
/// - [`PipelineDiscardRectangleStateCreateFlagsEXT`]
/// - [`Rect2D`]
/// - [`StructureType`]
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
pub struct PipelineDiscardRectangleStateCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: PipelineDiscardRectangleStateCreateFlagsEXT,
    ///[`discard_rectangle_mode`] is a [`DiscardRectangleModeEXT`] value
    ///determining whether the discard rectangle test is inclusive or
    ///exclusive.
    discard_rectangle_mode: DiscardRectangleModeEXT,
    ///[`discard_rectangle_count`] is the number of discard rectangles to use.
    discard_rectangle_count: u32,
    ///[`p_discard_rectangles`] is a pointer to an array of [`Rect2D`]
    ///structures defining discard rectangles.
    p_discard_rectangles: *mut Rect2D,
}
