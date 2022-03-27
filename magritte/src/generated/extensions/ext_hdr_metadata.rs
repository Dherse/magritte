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
#[derive(Clone, Debug, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct XyColorEXT {
    ///[`x`] is the x chromaticity coordinate.
    x: f32,
    ///[`y`] is the y chromaticity coordinate.
    y: f32,
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_HDR_METADATA_EXT`
/// - [`p_next`]**must** be `NULL`
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
#[derive(Clone, Debug, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct HdrMetadataEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
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
