use crate::vulkan1_0::{BaseInStructure, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_HDR_METADATA_SPEC_VERSION")]
pub const EXT_HDR_METADATA_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_HDR_METADATA_EXTENSION_NAME")]
pub const EXT_HDR_METADATA_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_hdr_metadata");
///[VkXYColorEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkXYColorEXT.html) - Specify X,Y chromaticity coordinates
///# C Specifications
///The [`XyColorEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_hdr_metadata
///typedef struct VkXYColorEXT {
///    float    x;
///    float    y;
///} VkXYColorEXT;
///```
///# Members
/// - [`x`] is the x chromaticity coordinate.
/// - [`y`] is the y chromaticity coordinate.
///# Description
///Chromaticity coordinates are as specified in CIE 15:2004 “Calculation of
///chromaticity coordinates” (Section 7.3) and are limited to between 0 and 1
///for real colors for the reference monitor.
///# Related
/// - [`VK_EXT_hdr_metadata`]
/// - [`HdrMetadataEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct XyColorEXT {
    ///[`x`] is the x chromaticity coordinate.
    x: f32,
    ///[`y`] is the y chromaticity coordinate.
    y: f32,
}
impl Default for XyColorEXT {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}
impl XyColorEXT {
    ///Gets the value of [`Self::x`]
    pub fn x(&self) -> f32 {
        self.x
    }
    ///Gets the value of [`Self::y`]
    pub fn y(&self) -> f32 {
        self.y
    }
    ///Gets a mutable reference to the value of [`Self::x`]
    pub fn x_mut(&mut self) -> &mut f32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::y`]
    pub fn y_mut(&mut self) -> &mut f32 {
        &mut getter
    }
    ///Sets the raw value of [`Self::x`]
    pub fn set_x(&mut self, value: f32) -> &mut Self {
        self.x = value;
        self
    }
    ///Sets the raw value of [`Self::y`]
    pub fn set_y(&mut self, value: f32) -> &mut Self {
        self.y = value;
        self
    }
}
///[VkHdrMetadataEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkHdrMetadataEXT.html) - Specify Hdr metadata
///# C Specifications
///The [`HdrMetadataEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_hdr_metadata
///typedef struct VkHdrMetadataEXT {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkXYColorEXT       displayPrimaryRed;
///    VkXYColorEXT       displayPrimaryGreen;
///    VkXYColorEXT       displayPrimaryBlue;
///    VkXYColorEXT       whitePoint;
///    float              maxLuminance;
///    float              minLuminance;
///    float              maxContentLightLevel;
///    float              maxFrameAverageLightLevel;
///} VkHdrMetadataEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`display_primary_red`] is a [`XyColorEXT`] structure specifying the reference monitor’s red
///   primary in chromaticity coordinates
/// - [`display_primary_green`] is a [`XyColorEXT`] structure specifying the reference monitor’s
///   green primary in chromaticity coordinates
/// - [`display_primary_blue`] is a [`XyColorEXT`] structure specifying the reference monitor’s blue
///   primary in chromaticity coordinates
/// - [`white_point`] is a [`XyColorEXT`] structure specifying the reference monitor’s white-point
///   in chromaticity coordinates
/// - [`max_luminance`] is the maximum luminance of the reference monitor in nits
/// - [`min_luminance`] is the minimum luminance of the reference monitor in nits
/// - [`max_content_light_level`] is content’s maximum luminance in nits
/// - [`max_frame_average_light_level`] is the maximum frame average light level in nits
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_HDR_METADATA_EXT`
/// - [`p_next`] **must**  be `NULL`
///# Related
/// - [`VK_EXT_hdr_metadata`]
/// - [`StructureType`]
/// - [`XyColorEXT`]
/// - [`SetHdrMetadataEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct HdrMetadataEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`display_primary_red`] is a [`XyColorEXT`] structure specifying the
    ///reference monitor’s red primary in chromaticity coordinates
    display_primary_red: XyColorEXT,
    ///[`display_primary_green`] is a [`XyColorEXT`] structure specifying
    ///the reference monitor’s green primary in chromaticity coordinates
    display_primary_green: XyColorEXT,
    ///[`display_primary_blue`] is a [`XyColorEXT`] structure specifying
    ///the reference monitor’s blue primary in chromaticity coordinates
    display_primary_blue: XyColorEXT,
    ///[`white_point`] is a [`XyColorEXT`] structure specifying the
    ///reference monitor’s white-point in chromaticity coordinates
    white_point: XyColorEXT,
    ///[`max_luminance`] is the maximum luminance of the reference monitor in
    ///nits
    max_luminance: f32,
    ///[`min_luminance`] is the minimum luminance of the reference monitor in
    ///nits
    min_luminance: f32,
    ///[`max_content_light_level`] is content’s maximum luminance in nits
    max_content_light_level: f32,
    ///[`max_frame_average_light_level`] is the maximum frame average light level
    ///in nits
    max_frame_average_light_level: f32,
}
impl<'lt> Default for HdrMetadataEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            display_primary_red: Default::default(),
            display_primary_green: Default::default(),
            display_primary_blue: Default::default(),
            white_point: Default::default(),
            max_luminance: 0.0,
            min_luminance: 0.0,
            max_content_light_level: 0.0,
            max_frame_average_light_level: 0.0,
        }
    }
}
impl<'lt> HdrMetadataEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
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
    ///Gets the value of [`Self::display_primary_red`]
    pub fn display_primary_red(&self) -> XyColorEXT {
        self.display_primary_red
    }
    ///Gets the value of [`Self::display_primary_green`]
    pub fn display_primary_green(&self) -> XyColorEXT {
        self.display_primary_green
    }
    ///Gets the value of [`Self::display_primary_blue`]
    pub fn display_primary_blue(&self) -> XyColorEXT {
        self.display_primary_blue
    }
    ///Gets the value of [`Self::white_point`]
    pub fn white_point(&self) -> XyColorEXT {
        self.white_point
    }
    ///Gets the value of [`Self::max_luminance`]
    pub fn max_luminance(&self) -> f32 {
        self.max_luminance
    }
    ///Gets the value of [`Self::min_luminance`]
    pub fn min_luminance(&self) -> f32 {
        self.min_luminance
    }
    ///Gets the value of [`Self::max_content_light_level`]
    pub fn max_content_light_level(&self) -> f32 {
        self.max_content_light_level
    }
    ///Gets the value of [`Self::max_frame_average_light_level`]
    pub fn max_frame_average_light_level(&self) -> f32 {
        self.max_frame_average_light_level
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::display_primary_red`]
    pub fn display_primary_red_mut(&mut self) -> &mut XyColorEXT {
        &mut self.display_primary_red
    }
    ///Gets a mutable reference to the value of [`Self::display_primary_green`]
    pub fn display_primary_green_mut(&mut self) -> &mut XyColorEXT {
        &mut self.display_primary_green
    }
    ///Gets a mutable reference to the value of [`Self::display_primary_blue`]
    pub fn display_primary_blue_mut(&mut self) -> &mut XyColorEXT {
        &mut self.display_primary_blue
    }
    ///Gets a mutable reference to the value of [`Self::white_point`]
    pub fn white_point_mut(&mut self) -> &mut XyColorEXT {
        &mut self.white_point
    }
    ///Gets a mutable reference to the value of [`Self::max_luminance`]
    pub fn max_luminance_mut(&mut self) -> &mut f32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::min_luminance`]
    pub fn min_luminance_mut(&mut self) -> &mut f32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_content_light_level`]
    pub fn max_content_light_level_mut(&mut self) -> &mut f32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_frame_average_light_level`]
    pub fn max_frame_average_light_level_mut(&mut self) -> &mut f32 {
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
    ///Sets the raw value of [`Self::display_primary_red`]
    pub fn set_display_primary_red(&mut self, value: crate::extensions::ext_hdr_metadata::XyColorEXT) -> &mut Self {
        self.display_primary_red = value;
        self
    }
    ///Sets the raw value of [`Self::display_primary_green`]
    pub fn set_display_primary_green(&mut self, value: crate::extensions::ext_hdr_metadata::XyColorEXT) -> &mut Self {
        self.display_primary_green = value;
        self
    }
    ///Sets the raw value of [`Self::display_primary_blue`]
    pub fn set_display_primary_blue(&mut self, value: crate::extensions::ext_hdr_metadata::XyColorEXT) -> &mut Self {
        self.display_primary_blue = value;
        self
    }
    ///Sets the raw value of [`Self::white_point`]
    pub fn set_white_point(&mut self, value: crate::extensions::ext_hdr_metadata::XyColorEXT) -> &mut Self {
        self.white_point = value;
        self
    }
    ///Sets the raw value of [`Self::max_luminance`]
    pub fn set_max_luminance(&mut self, value: f32) -> &mut Self {
        self.max_luminance = value;
        self
    }
    ///Sets the raw value of [`Self::min_luminance`]
    pub fn set_min_luminance(&mut self, value: f32) -> &mut Self {
        self.min_luminance = value;
        self
    }
    ///Sets the raw value of [`Self::max_content_light_level`]
    pub fn set_max_content_light_level(&mut self, value: f32) -> &mut Self {
        self.max_content_light_level = value;
        self
    }
    ///Sets the raw value of [`Self::max_frame_average_light_level`]
    pub fn set_max_frame_average_light_level(&mut self, value: f32) -> &mut Self {
        self.max_frame_average_light_level = value;
        self
    }
}
