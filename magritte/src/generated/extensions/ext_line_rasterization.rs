#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
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
/// - [`LINE_RASTERIZATION_MODE_DEFAULT`] is equivalent to
///[`LINE_RASTERIZATION_MODE_RECTANGULAR`] if
///[`PhysicalDeviceLimits::strict_lines`] is [`TRUE`],
///otherwise lines are drawn as non-`strictLines` parallelograms.
///Both of these modes are defined in [Basic Line
///Segment Rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines-basic).
/// - [`LINE_RASTERIZATION_MODE_RECTANGULAR`] specifies lines drawn
///as if they were rectangles extruded from the line
/// - [`LINE_RASTERIZATION_MODE_BRESENHAM`] specifies lines drawn by
///determining which pixel diamonds the line intersects and exits, as
///defined in [Bresenham Line Segment
///Rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines-bresenham).
/// - [`LINE_RASTERIZATION_MODE_RECTANGULAR_SMOOTH`] specifies lines
///drawn if they were rectangles extruded from the line, with alpha
///falloff, as defined in [Smooth Lines](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines-smooth).
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
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct LineRasterizationModeEXT(i32);
impl const Default for LineRasterizationModeEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for LineRasterizationModeEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("LineRasterizationModeEXT")
            .field(match *self {
                Self::LINE_RASTERIZATION_MODE_DEFAULT => &"LINE_RASTERIZATION_MODE_DEFAULT",
                Self::LINE_RASTERIZATION_MODE_RECTANGULAR => &"LINE_RASTERIZATION_MODE_RECTANGULAR",
                Self::LINE_RASTERIZATION_MODE_BRESENHAM => &"LINE_RASTERIZATION_MODE_BRESENHAM",
                Self::LINE_RASTERIZATION_MODE_RECTANGULAR_SMOOTH => &"LINE_RASTERIZATION_MODE_RECTANGULAR_SMOOTH",
                other => unreachable!("invalid value for `LineRasterizationModeEXT`: {:?}", other),
            })
            .finish()
    }
}
impl LineRasterizationModeEXT {
    ///[`LINE_RASTERIZATION_MODE_DEFAULT`] is equivalent to
    ///[`LINE_RASTERIZATION_MODE_RECTANGULAR`] if
    ///[`PhysicalDeviceLimits`]::`strictLines` is [`TRUE`],
    ///otherwise lines are drawn as non-`strictLines` parallelograms.
    ///Both of these modes are defined in [Basic Line
    ///Segment Rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines-basic).
    pub const LINE_RASTERIZATION_MODE_DEFAULT: Self = Self(0);
    ///[`LINE_RASTERIZATION_MODE_RECTANGULAR`] specifies lines drawn
    ///as if they were rectangles extruded from the line
    pub const LINE_RASTERIZATION_MODE_RECTANGULAR: Self = Self(1);
    ///[`LINE_RASTERIZATION_MODE_BRESENHAM`] specifies lines drawn by
    ///determining which pixel diamonds the line intersects and exits, as
    ///defined in [Bresenham Line Segment
    ///Rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines-bresenham).
    pub const LINE_RASTERIZATION_MODE_BRESENHAM: Self = Self(2);
    ///[`LINE_RASTERIZATION_MODE_RECTANGULAR_SMOOTH`] specifies lines
    ///drawn if they were rectangles extruded from the line, with alpha
    ///falloff, as defined in [Smooth Lines](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines-smooth).
    pub const LINE_RASTERIZATION_MODE_RECTANGULAR_SMOOTH: Self = Self(3);
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
