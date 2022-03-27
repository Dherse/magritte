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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceLineRasterizationFeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
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
impl<'lt> Default for PhysicalDeviceLineRasterizationFeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            rectangular_lines: 0,
            bresenham_lines: 0,
            smooth_lines: 0,
            stippled_rectangular_lines: 0,
            stippled_bresenham_lines: 0,
            stippled_smooth_lines: 0,
        }
    }
}
impl<'lt> PhysicalDeviceLineRasterizationFeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::rectangular_lines`]
    pub fn rectangular_lines_raw(&self) -> Bool32 {
        self.rectangular_lines
    }
    ///Gets the raw value of [`Self::bresenham_lines`]
    pub fn bresenham_lines_raw(&self) -> Bool32 {
        self.bresenham_lines
    }
    ///Gets the raw value of [`Self::smooth_lines`]
    pub fn smooth_lines_raw(&self) -> Bool32 {
        self.smooth_lines
    }
    ///Gets the raw value of [`Self::stippled_rectangular_lines`]
    pub fn stippled_rectangular_lines_raw(&self) -> Bool32 {
        self.stippled_rectangular_lines
    }
    ///Gets the raw value of [`Self::stippled_bresenham_lines`]
    pub fn stippled_bresenham_lines_raw(&self) -> Bool32 {
        self.stippled_bresenham_lines
    }
    ///Gets the raw value of [`Self::stippled_smooth_lines`]
    pub fn stippled_smooth_lines_raw(&self) -> Bool32 {
        self.stippled_smooth_lines
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::rectangular_lines`]
    pub fn set_rectangular_lines_raw(&mut self, value: Bool32) -> &mut Self {
        self.rectangular_lines = value;
        self
    }
    ///Sets the raw value of [`Self::bresenham_lines`]
    pub fn set_bresenham_lines_raw(&mut self, value: Bool32) -> &mut Self {
        self.bresenham_lines = value;
        self
    }
    ///Sets the raw value of [`Self::smooth_lines`]
    pub fn set_smooth_lines_raw(&mut self, value: Bool32) -> &mut Self {
        self.smooth_lines = value;
        self
    }
    ///Sets the raw value of [`Self::stippled_rectangular_lines`]
    pub fn set_stippled_rectangular_lines_raw(&mut self, value: Bool32) -> &mut Self {
        self.stippled_rectangular_lines = value;
        self
    }
    ///Sets the raw value of [`Self::stippled_bresenham_lines`]
    pub fn set_stippled_bresenham_lines_raw(&mut self, value: Bool32) -> &mut Self {
        self.stippled_bresenham_lines = value;
        self
    }
    ///Sets the raw value of [`Self::stippled_smooth_lines`]
    pub fn set_stippled_smooth_lines_raw(&mut self, value: Bool32) -> &mut Self {
        self.stippled_smooth_lines = value;
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
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::rectangular_lines`]
    pub fn rectangular_lines(&self) -> bool {
        unsafe { std::mem::transmute(self.rectangular_lines as u8) }
    }
    ///Gets the value of [`Self::bresenham_lines`]
    pub fn bresenham_lines(&self) -> bool {
        unsafe { std::mem::transmute(self.bresenham_lines as u8) }
    }
    ///Gets the value of [`Self::smooth_lines`]
    pub fn smooth_lines(&self) -> bool {
        unsafe { std::mem::transmute(self.smooth_lines as u8) }
    }
    ///Gets the value of [`Self::stippled_rectangular_lines`]
    pub fn stippled_rectangular_lines(&self) -> bool {
        unsafe { std::mem::transmute(self.stippled_rectangular_lines as u8) }
    }
    ///Gets the value of [`Self::stippled_bresenham_lines`]
    pub fn stippled_bresenham_lines(&self) -> bool {
        unsafe { std::mem::transmute(self.stippled_bresenham_lines as u8) }
    }
    ///Gets the value of [`Self::stippled_smooth_lines`]
    pub fn stippled_smooth_lines(&self) -> bool {
        unsafe { std::mem::transmute(self.stippled_smooth_lines as u8) }
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::rectangular_lines`]
    pub fn rectangular_lines_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.rectangular_lines as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.rectangular_lines as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::bresenham_lines`]
    pub fn bresenham_lines_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.bresenham_lines as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.bresenham_lines as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::smooth_lines`]
    pub fn smooth_lines_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.smooth_lines as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.smooth_lines as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::stippled_rectangular_lines`]
    pub fn stippled_rectangular_lines_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.stippled_rectangular_lines as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.stippled_rectangular_lines as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::stippled_bresenham_lines`]
    pub fn stippled_bresenham_lines_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.stippled_bresenham_lines as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.stippled_bresenham_lines as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::stippled_smooth_lines`]
    pub fn stippled_smooth_lines_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.stippled_smooth_lines as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.stippled_smooth_lines as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::rectangular_lines`]
    pub fn set_rectangular_lines(&mut self, value: bool) -> &mut Self {
        self.rectangular_lines = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::bresenham_lines`]
    pub fn set_bresenham_lines(&mut self, value: bool) -> &mut Self {
        self.bresenham_lines = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::smooth_lines`]
    pub fn set_smooth_lines(&mut self, value: bool) -> &mut Self {
        self.smooth_lines = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::stippled_rectangular_lines`]
    pub fn set_stippled_rectangular_lines(&mut self, value: bool) -> &mut Self {
        self.stippled_rectangular_lines = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::stippled_bresenham_lines`]
    pub fn set_stippled_bresenham_lines(&mut self, value: bool) -> &mut Self {
        self.stippled_bresenham_lines = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::stippled_smooth_lines`]
    pub fn set_stippled_smooth_lines(&mut self, value: bool) -> &mut Self {
        self.stippled_smooth_lines = value as u8 as u32;
        self
    }
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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceLineRasterizationPropertiesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`line_sub_pixel_precision_bits`] is
    ///the number of bits of subpixel precision in framebuffer coordinates
    ///x<sub>f</sub> and y<sub>f</sub> when rasterizing [line
    ///segments](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines).
    line_sub_pixel_precision_bits: u32,
}
impl<'lt> Default for PhysicalDeviceLineRasterizationPropertiesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            line_sub_pixel_precision_bits: 0,
        }
    }
}
impl<'lt> PhysicalDeviceLineRasterizationPropertiesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::line_sub_pixel_precision_bits`]
    pub fn line_sub_pixel_precision_bits_raw(&self) -> u32 {
        self.line_sub_pixel_precision_bits
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::line_sub_pixel_precision_bits`]
    pub fn set_line_sub_pixel_precision_bits_raw(&mut self, value: u32) -> &mut Self {
        self.line_sub_pixel_precision_bits = value;
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
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::line_sub_pixel_precision_bits`]
    pub fn line_sub_pixel_precision_bits(&self) -> u32 {
        self.line_sub_pixel_precision_bits
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::line_sub_pixel_precision_bits`]
    pub fn line_sub_pixel_precision_bits_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::line_sub_pixel_precision_bits`]
    pub fn set_line_sub_pixel_precision_bits(&mut self, value: u32) -> &mut Self {
        self.line_sub_pixel_precision_bits = value;
        self
    }
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PipelineRasterizationLineStateCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
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
impl<'lt> Default for PipelineRasterizationLineStateCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            line_rasterization_mode: Default::default(),
            stippled_line_enable: 0,
            line_stipple_factor: 0,
            line_stipple_pattern: 0,
        }
    }
}
impl<'lt> PipelineRasterizationLineStateCreateInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::stippled_line_enable`]
    pub fn stippled_line_enable_raw(&self) -> Bool32 {
        self.stippled_line_enable
    }
    ///Gets the raw value of [`Self::line_stipple_factor`]
    pub fn line_stipple_factor_raw(&self) -> u32 {
        self.line_stipple_factor
    }
    ///Gets the raw value of [`Self::line_stipple_pattern`]
    pub fn line_stipple_pattern_raw(&self) -> u16 {
        self.line_stipple_pattern
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::stippled_line_enable`]
    pub fn set_stippled_line_enable_raw(&mut self, value: Bool32) -> &mut Self {
        self.stippled_line_enable = value;
        self
    }
    ///Sets the raw value of [`Self::line_stipple_factor`]
    pub fn set_line_stipple_factor_raw(&mut self, value: u32) -> &mut Self {
        self.line_stipple_factor = value;
        self
    }
    ///Sets the raw value of [`Self::line_stipple_pattern`]
    pub fn set_line_stipple_pattern_raw(&mut self, value: u16) -> &mut Self {
        self.line_stipple_pattern = value;
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
    ///Gets the value of [`Self::line_rasterization_mode`]
    pub fn line_rasterization_mode(&self) -> LineRasterizationModeEXT {
        self.line_rasterization_mode
    }
    ///Gets the value of [`Self::stippled_line_enable`]
    pub fn stippled_line_enable(&self) -> bool {
        unsafe { std::mem::transmute(self.stippled_line_enable as u8) }
    }
    ///Gets the value of [`Self::line_stipple_factor`]
    pub fn line_stipple_factor(&self) -> u32 {
        self.line_stipple_factor
    }
    ///Gets the value of [`Self::line_stipple_pattern`]
    pub fn line_stipple_pattern(&self) -> u16 {
        self.line_stipple_pattern
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::line_rasterization_mode`]
    pub fn line_rasterization_mode_mut(&mut self) -> &mut LineRasterizationModeEXT {
        &mut self.line_rasterization_mode
    }
    ///Gets a mutable reference to the value of [`Self::stippled_line_enable`]
    pub fn stippled_line_enable_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.stippled_line_enable as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.stippled_line_enable as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::line_stipple_factor`]
    pub fn line_stipple_factor_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::line_stipple_pattern`]
    pub fn line_stipple_pattern_mut(&mut self) -> &mut u16 {
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
    ///Sets the raw value of [`Self::line_rasterization_mode`]
    pub fn set_line_rasterization_mode(
        &mut self,
        value: crate::extensions::ext_line_rasterization::LineRasterizationModeEXT,
    ) -> &mut Self {
        self.line_rasterization_mode = value;
        self
    }
    ///Sets the raw value of [`Self::stippled_line_enable`]
    pub fn set_stippled_line_enable(&mut self, value: bool) -> &mut Self {
        self.stippled_line_enable = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::line_stipple_factor`]
    pub fn set_line_stipple_factor(&mut self, value: u32) -> &mut Self {
        self.line_stipple_factor = value;
        self
    }
    ///Sets the raw value of [`Self::line_stipple_pattern`]
    pub fn set_line_stipple_pattern(&mut self, value: u16) -> &mut Self {
        self.line_stipple_pattern = value;
        self
    }
}
