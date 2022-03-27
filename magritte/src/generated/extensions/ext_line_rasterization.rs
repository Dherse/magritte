use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_LINE_RASTERIZATION_SPEC_VERSION")]
pub const EXT_LINE_RASTERIZATION_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_LINE_RASTERIZATION_EXTENSION_NAME")]
pub const EXT_LINE_RASTERIZATION_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_line_rasterization");
///[VkLineRasterizationModeEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkLineRasterizationModeEXT.html) - Line rasterization modes
///# C Specifications
///Possible values of
///[`PipelineRasterizationLineStateCreateInfoEXT::line_rasterization_mode`]
///are:
///```c
///// Provided by VK_EXT_line_rasterization
///typedef enum VkLineRasterizationModeEXT {
///    VK_LINE_RASTERIZATION_MODE_DEFAULT_EXT = 0,
///    VK_LINE_RASTERIZATION_MODE_RECTANGULAR_EXT = 1,
///    VK_LINE_RASTERIZATION_MODE_BRESENHAM_EXT = 2,
///    VK_LINE_RASTERIZATION_MODE_RECTANGULAR_SMOOTH_EXT = 3,
///} VkLineRasterizationModeEXT;
///```
///# Description
/// - [`LineRasterizationModeDefaultExt`] is equivalent to [`LineRasterizationModeRectangularExt`] if [`PhysicalDeviceLimits::strict_lines`] is [`TRUE`], otherwise lines are drawn as non-`strictLines` parallelograms. Both of these modes are defined in [Basic Line Segment Rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines-basic).
/// - [`LineRasterizationModeRectangularExt`] specifies lines drawn as if they were rectangles
///   extruded from the line
/// - [`LineRasterizationModeBresenhamExt`] specifies lines drawn by determining which pixel diamonds the line intersects and exits, as defined in [Bresenham Line Segment Rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines-bresenham).
/// - [`LineRasterizationModeRectangularSmoothExt`] specifies lines drawn if they were rectangles extruded from the line, with alpha falloff, as defined in [Smooth Lines](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines-smooth).
///# Related
/// - [`VK_EXT_line_rasterization`]
/// - [`PipelineRasterizationLineStateCreateInfoEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkLineRasterizationModeEXT")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum LineRasterizationModeEXT {
    ///[`LineRasterizationModeDefaultExt`] is equivalent to
    ///[`LineRasterizationModeRectangularExt`] if
    ///[`PhysicalDeviceLimits`]::`strictLines` is [`TRUE`],
    ///otherwise lines are drawn as non-`strictLines` parallelograms.
    ///Both of these modes are defined in [Basic Line
    ///Segment Rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines-basic).
    LineRasterizationModeDefaultExt = 0,
    ///[`LineRasterizationModeRectangularExt`] specifies lines drawn
    ///as if they were rectangles extruded from the line
    LineRasterizationModeRectangularExt = 1,
    ///[`LineRasterizationModeBresenhamExt`] specifies lines drawn by
    ///determining which pixel diamonds the line intersects and exits, as
    ///defined in [Bresenham Line Segment
    ///Rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines-bresenham).
    LineRasterizationModeBresenhamExt = 2,
    ///[`LineRasterizationModeRectangularSmoothExt`] specifies lines
    ///drawn if they were rectangles extruded from the line, with alpha
    ///falloff, as defined in [Smooth Lines](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines-smooth).
    LineRasterizationModeRectangularSmoothExt = 3,
}
impl const Default for LineRasterizationModeEXT {
    fn default() -> Self {
        LineRasterizationModeDefaultExt
    }
}
impl LineRasterizationModeEXT {
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
///[VkPhysicalDeviceLineRasterizationFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceLineRasterizationFeaturesEXT.html) - Structure describing the line rasterization features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceLineRasterizationFeaturesEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_line_rasterization
///typedef struct VkPhysicalDeviceLineRasterizationFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           rectangularLines;
///    VkBool32           bresenhamLines;
///    VkBool32           smoothLines;
///    VkBool32           stippledRectangularLines;
///    VkBool32           stippledBresenhamLines;
///    VkBool32           stippledSmoothLines;
///} VkPhysicalDeviceLineRasterizationFeaturesEXT;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`rectangular_lines`] indicates whether the implementation supports [rectangular line rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines).
/// - [`bresenham_lines`] indicates whether the implementation supports [Bresenham-style line rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines-bresenham).
/// - [`smooth_lines`] indicates whether the implementation supports [smooth line rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines-smooth).
/// - [`stippled_rectangular_lines`] indicates whether the implementation supports [stippled line rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines-stipple)
///   with `VK_LINE_RASTERIZATION_MODE_RECTANGULAR_EXT` lines, or with
///   `VK_LINE_RASTERIZATION_MODE_DEFAULT_EXT` lines if [`PhysicalDeviceLimits::strict_lines`] is
///   [`TRUE`].
/// - [`stippled_bresenham_lines`] indicates whether the implementation supports [stippled line rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines-stipple)
///   with `VK_LINE_RASTERIZATION_MODE_BRESENHAM_EXT` lines.
/// - [`stippled_smooth_lines`] indicates whether the implementation supports [stippled line rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines-stipple)
///   with `VK_LINE_RASTERIZATION_MODE_RECTANGULAR_SMOOTH_EXT` lines.
///If the [`PhysicalDeviceLineRasterizationFeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceLineRasterizationFeaturesEXT`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_EXT`
///# Related
/// - [`VK_EXT_line_rasterization`]
/// - [`Bool32`]
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
pub struct PhysicalDeviceLineRasterizationFeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`rectangular_lines`] indicates whether
    ///the implementation supports [rectangular line
    ///rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines).
    rectangular_lines: Bool32,
    ///[`bresenham_lines`] indicates whether the
    ///implementation supports [Bresenham-style line
    ///rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines-bresenham).
    bresenham_lines: Bool32,
    ///[`smooth_lines`] indicates whether the
    ///implementation supports [smooth line
    ///rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines-smooth).
    smooth_lines: Bool32,
    ///[`stippled_rectangular_lines`]
    ///indicates whether the implementation supports
    ///[stippled line rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines-stipple) with
    ///`VK_LINE_RASTERIZATION_MODE_RECTANGULAR_EXT` lines, or with
    ///`VK_LINE_RASTERIZATION_MODE_DEFAULT_EXT` lines if
    ///[`PhysicalDeviceLimits`]::`strictLines` is [`TRUE`].
    stippled_rectangular_lines: Bool32,
    ///[`stippled_bresenham_lines`]
    ///indicates whether the implementation supports
    ///[stippled line rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines-stipple) with
    ///`VK_LINE_RASTERIZATION_MODE_BRESENHAM_EXT` lines.
    stippled_bresenham_lines: Bool32,
    ///[`stippled_smooth_lines`] indicates
    ///whether the implementation supports [stippled
    ///line rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines-stipple) with
    ///`VK_LINE_RASTERIZATION_MODE_RECTANGULAR_SMOOTH_EXT` lines.
    stippled_smooth_lines: Bool32,
}
///[VkPhysicalDeviceLineRasterizationPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceLineRasterizationPropertiesEXT.html) - Structure describing line rasterization properties supported by an implementation
///# C Specifications
///The [`PhysicalDeviceLineRasterizationPropertiesEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_line_rasterization
///typedef struct VkPhysicalDeviceLineRasterizationPropertiesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           lineSubPixelPrecisionBits;
///} VkPhysicalDeviceLineRasterizationPropertiesEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`line_sub_pixel_precision_bits`] is the number of bits of subpixel precision in framebuffer coordinates x<sub>f</sub> and y<sub>f</sub> when rasterizing [line segments](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines).
///# Description
///If the [`PhysicalDeviceLineRasterizationPropertiesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_EXT`
///# Related
/// - [`VK_EXT_line_rasterization`]
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
pub struct PhysicalDeviceLineRasterizationPropertiesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`line_sub_pixel_precision_bits`] is
    ///the number of bits of subpixel precision in framebuffer coordinates
    ///x<sub>f</sub> and y<sub>f</sub> when rasterizing [line
    ///segments](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines).
    line_sub_pixel_precision_bits: u32,
}
///[VkPipelineRasterizationLineStateCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationLineStateCreateInfoEXT.html) - Structure specifying parameters of a newly created pipeline line rasterization state
///# C Specifications
///Line segment rasterization options are controlled by the
///[`PipelineRasterizationLineStateCreateInfoEXT`] structure.The
/// [`PipelineRasterizationLineStateCreateInfoEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_line_rasterization
///typedef struct VkPipelineRasterizationLineStateCreateInfoEXT {
///    VkStructureType               sType;
///    const void*                   pNext;
///    VkLineRasterizationModeEXT    lineRasterizationMode;
///    VkBool32                      stippledLineEnable;
///    uint32_t                      lineStippleFactor;
///    uint16_t                      lineStipplePattern;
///} VkPipelineRasterizationLineStateCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`line_rasterization_mode`] is a [`LineRasterizationModeEXT`] value selecting the style of
///   line rasterization.
/// - [`stippled_line_enable`] enables [stippled line rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines-stipple).
/// - [`line_stipple_factor`] is the repeat factor used in stippled line rasterization.
/// - [`line_stipple_pattern`] is the bit pattern used in stippled line rasterization.
///# Description
///If [`stippled_line_enable`] is [`FALSE`], the values of
///[`line_stipple_factor`] and [`line_stipple_pattern`] are ignored.Valid Usage
/// - If [`line_rasterization_mode`] is `VK_LINE_RASTERIZATION_MODE_RECTANGULAR_EXT`, then the [rectangularLines](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-rectangularLines)
///   feature **must** be enabled
/// - If [`line_rasterization_mode`] is `VK_LINE_RASTERIZATION_MODE_BRESENHAM_EXT`, then the [bresenhamLines](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-bresenhamLines)
///   feature **must** be enabled
/// -    If [`line_rasterization_mode`] is `VK_LINE_RASTERIZATION_MODE_RECTANGULAR_SMOOTH_EXT`, then the [smoothLines](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-bresenhamLines) feature **must** be enabled
/// -    If [`stippled_line_enable`] is [`TRUE`] and [`line_rasterization_mode`] is `VK_LINE_RASTERIZATION_MODE_RECTANGULAR_EXT`, then the [stippledRectangularLines](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-stippledRectangularLines) feature **must** be enabled
/// -    If [`stippled_line_enable`] is [`TRUE`] and [`line_rasterization_mode`] is `VK_LINE_RASTERIZATION_MODE_BRESENHAM_EXT`, then the [stippledBresenhamLines](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-stippledBresenhamLines) feature **must** be enabled
/// -    If [`stippled_line_enable`] is [`TRUE`] and [`line_rasterization_mode`] is `VK_LINE_RASTERIZATION_MODE_RECTANGULAR_SMOOTH_EXT`, then the [stippledSmoothLines](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-stippledSmoothLines) feature **must** be enabled
/// -    If [`stippled_line_enable`] is [`TRUE`] and [`line_rasterization_mode`] is `VK_LINE_RASTERIZATION_MODE_DEFAULT_EXT`, then the [stippledRectangularLines](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-stippledRectangularLines) feature **must** be enabled and [`PhysicalDeviceLimits::strict_lines`]**must** be [`TRUE`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_EXT`
/// - [`line_rasterization_mode`]**must** be a valid [`LineRasterizationModeEXT`] value
///# Related
/// - [`VK_EXT_line_rasterization`]
/// - [`Bool32`]
/// - [`LineRasterizationModeEXT`]
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
pub struct PipelineRasterizationLineStateCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`line_rasterization_mode`] is a [`LineRasterizationModeEXT`] value
    ///selecting the style of line rasterization.
    line_rasterization_mode: LineRasterizationModeEXT,
    ///[`stippled_line_enable`] enables [stippled
    ///line rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines-stipple).
    stippled_line_enable: Bool32,
    ///[`line_stipple_factor`] is the repeat factor used in stippled line
    ///rasterization.
    line_stipple_factor: u32,
    ///[`line_stipple_pattern`] is the bit pattern used in stippled line
    ///rasterization.
    line_stipple_pattern: u16,
}
