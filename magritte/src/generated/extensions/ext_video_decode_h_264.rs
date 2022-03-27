use crate::{
    native::{
        StdVideoDecodeH264Mvc, StdVideoDecodeH264PictureInfo, StdVideoDecodeH264ReferenceInfo,
        StdVideoH264PictureParameterSet, StdVideoH264ProfileIdc, StdVideoH264SequenceParameterSet,
    },
    vulkan1_0::{BaseInStructure, BaseOutStructure, ExtensionProperties, Offset2D, StructureType},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_VIDEO_DECODE_H264_SPEC_VERSION")]
pub const EXT_VIDEO_DECODE_H264_SPEC_VERSION: u32 = 3;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_VIDEO_DECODE_H264_EXTENSION_NAME")]
pub const EXT_VIDEO_DECODE_H264_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_video_decode_h264");
///[VkVideoDecodeH264ProfileEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH264ProfileEXT.html) - Structure specifying H.264 decode profile
///# C Specifications
///The [`VideoDecodeH264ProfileEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_video_decode_h264
///typedef struct VkVideoDecodeH264ProfileEXT {
///    VkStructureType                           sType;
///    const void*                               pNext;
///    StdVideoH264ProfileIdc                    stdProfileIdc;
///    VkVideoDecodeH264PictureLayoutFlagsEXT    pictureLayout;
///} VkVideoDecodeH264ProfileEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`std_profile_idc`] is a [`StdVideoH264ProfileIdc`] value specifying the H.264 codec profile
///   IDC
/// - [`picture_layout`] is a bitmask of [`VideoDecodeH264PictureLayoutFlagBitsEXT`] specifying the
///   layout of the decoded picture’s contents depending on the nature (progressive vs. interlaced)
///   of the input content.
///# Description
///Valid Usage
/// - If the [`VideoDecodeH264ProfileEXT`] structure is included in the [`p_next`] chain of the
///   [`VideoCapabilitiesKHR`] structure passed to [`GetPhysicalDeviceVideoCapabilitiesKHR`], the
///   value in [`picture_layout`] is treated as a bitmask of requested picture layouts. It is always
///   valid to use the value `VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_PROGRESSIVE_EXT` as the
///   implementation is guaranteed to support decoding of progressive content.
/// - If the [`VideoDecodeH264ProfileEXT`] structure is included in the [`p_next`] chain of the
///   [`VideoSessionCreateInfoKHR`] structure passed to [`CreateVideoSessionKHR`], the value in
///   [`picture_layout`]**must** be exactly one of
///   `VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_PROGRESSIVE_EXT`,
///   `VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_INTERLACED_INTERLEAVED_LINES_BIT_EXT` or
///   `VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_INTERLACED_SEPARATE_PLANES_BIT_EXT`.
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_PROFILE_EXT`
///# Related
/// - [`VK_EXT_video_decode_h264`]
/// - [`StructureType`]
/// - [`VideoDecodeH264PictureLayoutFlagsEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoDecodeH264ProfileEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`std_profile_idc`] is a [`StdVideoH264ProfileIdc`] value specifying
    ///the H.264 codec profile IDC
    std_profile_idc: StdVideoH264ProfileIdc,
    ///[`picture_layout`] is a bitmask of
    ///[`VideoDecodeH264PictureLayoutFlagBitsEXT`] specifying the layout of
    ///the decoded picture’s contents depending on the nature (progressive vs.
    ///interlaced) of the input content.
    picture_layout: VideoDecodeH264PictureLayoutFlagsEXT,
}
impl<'lt> Default for VideoDecodeH264ProfileEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            std_profile_idc: Default::default(),
            picture_layout: Default::default(),
        }
    }
}
impl<'lt> VideoDecodeH264ProfileEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::std_profile_idc`]
    pub fn std_profile_idc_raw(&self) -> &StdVideoH264ProfileIdc {
        &self.std_profile_idc
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::std_profile_idc`]
    pub fn set_std_profile_idc_raw(&mut self, value: StdVideoH264ProfileIdc) -> &mut Self {
        self.std_profile_idc = value;
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
    ///Gets the value of [`Self::std_profile_idc`]
    pub fn std_profile_idc(&self) -> &StdVideoH264ProfileIdc {
        &self.std_profile_idc
    }
    ///Gets the value of [`Self::picture_layout`]
    pub fn picture_layout(&self) -> VideoDecodeH264PictureLayoutFlagsEXT {
        self.picture_layout
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::std_profile_idc`]
    pub fn std_profile_idc_mut(&mut self) -> &mut StdVideoH264ProfileIdc {
        &mut self.std_profile_idc
    }
    ///Gets a mutable reference to the value of [`Self::picture_layout`]
    pub fn picture_layout_mut(&mut self) -> &mut VideoDecodeH264PictureLayoutFlagsEXT {
        &mut self.picture_layout
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
    ///Sets the raw value of [`Self::std_profile_idc`]
    pub fn set_std_profile_idc(&mut self, value: crate::native::StdVideoH264ProfileIdc) -> &mut Self {
        self.std_profile_idc = value;
        self
    }
    ///Sets the raw value of [`Self::picture_layout`]
    pub fn set_picture_layout(
        &mut self,
        value: crate::extensions::ext_video_decode_h_264::VideoDecodeH264PictureLayoutFlagsEXT,
    ) -> &mut Self {
        self.picture_layout = value;
        self
    }
}
///[VkVideoDecodeH264CapabilitiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH264CapabilitiesEXT.html) - Structure specifying H.264 decode capabilities
///# C Specifications
///The [`VideoDecodeH264CapabilitiesEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_video_decode_h264
///typedef struct VkVideoDecodeH264CapabilitiesEXT {
///    VkStructureType          sType;
///    void*                    pNext;
///    uint32_t                 maxLevel;
///    VkOffset2D               fieldOffsetGranularity;
///    VkExtensionProperties    stdExtensionVersion;
///} VkVideoDecodeH264CapabilitiesEXT;
///```
///# Members
///When using [`GetPhysicalDeviceVideoCapabilitiesKHR`] to query the
///capabilities for the input `pVideoProfile` with
///`videoCodecOperation` specified as
///`VK_VIDEO_CODEC_OPERATION_DECODE_H264_BIT_EXT`, a
///[`VideoDecodeH264CapabilitiesEXT`] structure **must** be chained to
///[`VideoCapabilitiesKHR`] to get this H.264 decode profile specific
///capabilities.
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`max_level`] is the maximum H.264 level supported by the device.
/// - [`field_offset_granularity`] - if Interlaced Video Content is suported, the maximum field
///   offset granularity supported for the picture resource.
/// - [`std_extension_version`] is a [`ExtensionProperties`] structure specifying the H.264
///   extension name and version supported by this implementation.
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_CAPABILITIES_EXT`
///# Related
/// - [`VK_EXT_video_decode_h264`]
/// - [`ExtensionProperties`]
/// - [`Offset2D`]
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
pub struct VideoDecodeH264CapabilitiesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`max_level`] is the maximum H.264 level supported by the device.
    max_level: u32,
    ///[`field_offset_granularity`] - if Interlaced Video Content is suported,
    ///the maximum field offset granularity supported for the picture resource.
    field_offset_granularity: Offset2D,
    ///[`std_extension_version`] is a [`ExtensionProperties`] structure
    ///specifying the H.264 extension name and version supported by this
    ///implementation.
    std_extension_version: ExtensionProperties,
}
impl<'lt> Default for VideoDecodeH264CapabilitiesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            max_level: 0,
            field_offset_granularity: Default::default(),
            std_extension_version: Default::default(),
        }
    }
}
impl<'lt> VideoDecodeH264CapabilitiesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::max_level`]
    pub fn max_level_raw(&self) -> u32 {
        self.max_level
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::max_level`]
    pub fn set_max_level_raw(&mut self, value: u32) -> &mut Self {
        self.max_level = value;
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
    ///Gets the value of [`Self::max_level`]
    pub fn max_level(&self) -> u32 {
        self.max_level
    }
    ///Gets the value of [`Self::field_offset_granularity`]
    pub fn field_offset_granularity(&self) -> Offset2D {
        self.field_offset_granularity
    }
    ///Gets the value of [`Self::std_extension_version`]
    pub fn std_extension_version(&self) -> ExtensionProperties {
        self.std_extension_version
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
    ///Gets a mutable reference to the value of [`Self::max_level`]
    pub fn max_level_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::field_offset_granularity`]
    pub fn field_offset_granularity_mut(&mut self) -> &mut Offset2D {
        &mut self.field_offset_granularity
    }
    ///Gets a mutable reference to the value of [`Self::std_extension_version`]
    pub fn std_extension_version_mut(&mut self) -> &mut ExtensionProperties {
        &mut self.std_extension_version
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
    ///Sets the raw value of [`Self::max_level`]
    pub fn set_max_level(&mut self, value: u32) -> &mut Self {
        self.max_level = value;
        self
    }
    ///Sets the raw value of [`Self::field_offset_granularity`]
    pub fn set_field_offset_granularity(&mut self, value: crate::vulkan1_0::Offset2D) -> &mut Self {
        self.field_offset_granularity = value;
        self
    }
    ///Sets the raw value of [`Self::std_extension_version`]
    pub fn set_std_extension_version(&mut self, value: crate::vulkan1_0::ExtensionProperties) -> &mut Self {
        self.std_extension_version = value;
        self
    }
}
///[VkVideoDecodeH264SessionCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH264SessionCreateInfoEXT.html) - Structure specifies H.264 decode session creation parameters
///# C Specifications
///The [`VideoDecodeH264SessionCreateInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_video_decode_h264
///typedef struct VkVideoDecodeH264SessionCreateInfoEXT {
///    VkStructureType                    sType;
///    const void*                        pNext;
///    VkVideoDecodeH264CreateFlagsEXT    flags;
///    const VkExtensionProperties*       pStdExtensionVersion;
///} VkVideoDecodeH264SessionCreateInfoEXT;
///```
///# Members
///A [`VideoDecodeH264SessionCreateInfoEXT`] structure **can** be chained to
///[`VideoSessionCreateInfoKHR`] when the function
///[`CreateVideoSessionKHR`] is called to create a video session for H.264
///decode.
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`std_extension_version`] is a pointer to a [`ExtensionProperties`] structure specifying the
///   H.264 codec extensions defined in `StdVideoH264Extensions`.
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_SESSION_CREATE_INFO_EXT`
/// - [`flags`]**must** be `0`
/// - [`std_extension_version`]**must** be a valid pointer to a valid [`ExtensionProperties`]
///   structure
///# Related
/// - [`VK_EXT_video_decode_h264`]
/// - [`ExtensionProperties`]
/// - [`StructureType`]
/// - [`VideoDecodeH264CreateFlagsEXT`]
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
pub struct VideoDecodeH264SessionCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: VideoDecodeH264CreateFlagsEXT,
    ///[`std_extension_version`] is a pointer to a [`ExtensionProperties`]
    ///structure specifying the H.264 codec extensions defined in
    ///`StdVideoH264Extensions`.
    std_extension_version: *const ExtensionProperties,
}
impl<'lt> Default for VideoDecodeH264SessionCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
            std_extension_version: std::ptr::null(),
        }
    }
}
impl<'lt> VideoDecodeH264SessionCreateInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::std_extension_version`]
    pub fn std_extension_version_raw(&self) -> *const ExtensionProperties {
        self.std_extension_version
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::std_extension_version`]
    pub fn set_std_extension_version_raw(&mut self, value: *const ExtensionProperties) -> &mut Self {
        self.std_extension_version = value;
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
    pub fn flags(&self) -> VideoDecodeH264CreateFlagsEXT {
        self.flags
    }
    ///Gets the value of [`Self::std_extension_version`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn std_extension_version(&self) -> &ExtensionProperties {
        &*self.std_extension_version
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut VideoDecodeH264CreateFlagsEXT {
        &mut self.flags
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
        value: crate::extensions::ext_video_decode_h_264::VideoDecodeH264CreateFlagsEXT,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::std_extension_version`]
    pub fn set_std_extension_version(&mut self, value: &'lt crate::vulkan1_0::ExtensionProperties) -> &mut Self {
        self.std_extension_version = value as *const _;
        self
    }
}
///[VkVideoDecodeH264SessionParametersAddInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH264SessionParametersAddInfoEXT.html) - Structure specifies H.264 decoder parameter set information
///# C Specifications
///The [`VideoDecodeH264SessionParametersAddInfoEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_video_decode_h264
///typedef struct VkVideoDecodeH264SessionParametersAddInfoEXT {
///    VkStructureType                            sType;
///    const void*                                pNext;
///    uint32_t                                   spsStdCount;
///    const StdVideoH264SequenceParameterSet*    pSpsStd;
///    uint32_t                                   ppsStdCount;
///    const StdVideoH264PictureParameterSet*     pPpsStd;
///} VkVideoDecodeH264SessionParametersAddInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`sps_std_count`] is the number of SPS elements in [`sps_std`]. Its value **must** be less
///   than or equal to the value of `maxSpsStdCount`.
/// - [`sps_std`] is a pointer to an array of [`StdVideoH264SequenceParameterSet`] structures
///   representing H.264 sequence parameter sets. Each element of the array **must** have a unique
///   H.264 SPS ID.
/// - [`pps_std_count`] is the number of PPS provided in [`pps_std`]. Its value **must** be less
///   than or equal to the value of `maxPpsStdCount`.
/// - [`pps_std`] is a pointer to an array of [`StdVideoH264PictureParameterSet`] structures
///   representing H.264 picture parameter sets. Each element of the array **must** have a unique
///   H.264 SPS-PPS ID pair.
///# Description
///Valid Usage
/// - The values of [`sps_std_count`] and [`pps_std_count`]**must** be less than or equal to the
///   values of `maxSpsStdCount` and `maxPpsStdCount`, respectively
/// - When the `maxSpsStdCount` number of parameters of type StdVideoH264SequenceParameterSet in the
///   Video Session Parameters object is reached, no additional parameters of that type can be added
///   to this object. `VK_ERROR_TOO_MANY_OBJECTS` will be returned if an attempt is made to add
///   additional data to this object at this point
/// - When the `maxPpsStdCount` number of parameters of type StdVideoH264PictureParameterSet in the
///   Video Session Parameters object is reached, no additional parameters of that type can be added
///   to this object. `VK_ERROR_TOO_MANY_OBJECTS` will be returned if an attempt is made to add
///   additional data to this object at this point
/// - Each entry to be added **must** have a unique, to the rest of the parameter array entries and
///   the existing parameters in the Video Session Parameters Object that is being updated, SPS-PPS
///   IDs
/// - Parameter entries that already exist in Video Session Parameters object with a particular
///   SPS-PPS IDs **cannot** be replaced nor updated
/// - When creating a new object using a Video Session Parameters as a template, the array’s
///   parameters with the same SPS-PPS IDs as the ones from the template take precedence
/// - SPS/PPS parameters **must** comply with the limits specified in [`VideoSessionCreateInfoKHR`]
///   during Video Session creation
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_SESSION_PARAMETERS_ADD_INFO_EXT`
/// - If [`sps_std`] is not `NULL`, [`sps_std`]**must** be a valid pointer to an array of
///   [`sps_std_count`][`StdVideoH264SequenceParameterSet`] values
/// - If [`pps_std`] is not `NULL`, [`pps_std`]**must** be a valid pointer to an array of
///   [`pps_std_count`][`StdVideoH264PictureParameterSet`] values
/// - [`sps_std_count`]**must** be greater than `0`
/// - [`pps_std_count`]**must** be greater than `0`
///# Related
/// - [`VK_EXT_video_decode_h264`]
/// - [`StructureType`]
/// - [`VideoDecodeH264SessionParametersCreateInfoEXT`]
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
pub struct VideoDecodeH264SessionParametersAddInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`sps_std_count`] is the number of SPS elements in [`sps_std`].
    ///Its value **must** be less than or equal to the value of
    ///`maxSpsStdCount`.
    sps_std_count: u32,
    ///[`sps_std`] is a pointer to an array of
    ///[`StdVideoH264SequenceParameterSet`] structures representing H.264
    ///sequence parameter sets.
    ///Each element of the array **must** have a unique H.264 SPS ID.
    sps_std: *const StdVideoH264SequenceParameterSet,
    ///[`pps_std_count`] is the number of PPS provided in [`pps_std`].
    ///Its value **must** be less than or equal to the value of
    ///`maxPpsStdCount`.
    pps_std_count: u32,
    ///[`pps_std`] is a pointer to an array of
    ///[`StdVideoH264PictureParameterSet`] structures representing H.264
    ///picture parameter sets.
    ///Each element of the array **must** have a unique H.264 SPS-PPS ID pair.
    pps_std: *const StdVideoH264PictureParameterSet,
}
impl<'lt> Default for VideoDecodeH264SessionParametersAddInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            sps_std_count: 0,
            sps_std: std::ptr::null(),
            pps_std_count: 0,
            pps_std: std::ptr::null(),
        }
    }
}
impl<'lt> VideoDecodeH264SessionParametersAddInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::sps_std_count`]
    pub fn sps_std_count_raw(&self) -> u32 {
        self.sps_std_count
    }
    ///Gets the raw value of [`Self::pps_std_count`]
    pub fn pps_std_count_raw(&self) -> u32 {
        self.pps_std_count
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::sps_std_count`]
    pub fn set_sps_std_count_raw(&mut self, value: u32) -> &mut Self {
        self.sps_std_count = value;
        self
    }
    ///Sets the raw value of [`Self::pps_std_count`]
    pub fn set_pps_std_count_raw(&mut self, value: u32) -> &mut Self {
        self.pps_std_count = value;
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
    ///Gets the value of [`Self::sps_std_count`]
    pub fn sps_std_count(&self) -> u32 {
        self.sps_std_count
    }
    ///Gets the value of [`Self::sps_std`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn sps_std(&self) -> &[StdVideoH264SequenceParameterSet] {
        std::slice::from_raw_parts(self.sps_std, self.sps_std_count as usize)
    }
    ///Gets the value of [`Self::pps_std_count`]
    pub fn pps_std_count(&self) -> u32 {
        self.pps_std_count
    }
    ///Gets the value of [`Self::pps_std`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn pps_std(&self) -> &[StdVideoH264PictureParameterSet] {
        std::slice::from_raw_parts(self.pps_std, self.pps_std_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::sps_std_count`]
    pub fn sps_std_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::pps_std_count`]
    pub fn pps_std_count_mut(&mut self) -> &mut u32 {
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
    ///Sets the raw value of [`Self::sps_std_count`]
    pub fn set_sps_std_count(&mut self, value: u32) -> &mut Self {
        self.sps_std_count = value;
        self
    }
    ///Sets the raw value of [`Self::sps_std`]
    pub fn set_sps_std(&mut self, value: &'lt [crate::native::StdVideoH264SequenceParameterSet]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.sps_std = value.as_ptr();
        self.sps_std_count = len_;
        self
    }
    ///Sets the raw value of [`Self::pps_std_count`]
    pub fn set_pps_std_count(&mut self, value: u32) -> &mut Self {
        self.pps_std_count = value;
        self
    }
    ///Sets the raw value of [`Self::pps_std`]
    pub fn set_pps_std(&mut self, value: &'lt [crate::native::StdVideoH264PictureParameterSet]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.pps_std = value.as_ptr();
        self.pps_std_count = len_;
        self
    }
}
///[VkVideoDecodeH264SessionParametersCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH264SessionParametersCreateInfoEXT.html) - Structure specifies H.264 decoder parameter set information
///# C Specifications
///The [`VideoDecodeH264SessionParametersCreateInfoEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_video_decode_h264
///typedef struct VkVideoDecodeH264SessionParametersCreateInfoEXT {
///    VkStructureType                                        sType;
///    const void*                                            pNext;
///    uint32_t                                               maxSpsStdCount;
///    uint32_t                                               maxPpsStdCount;
///    const VkVideoDecodeH264SessionParametersAddInfoEXT*    pParametersAddInfo;
///} VkVideoDecodeH264SessionParametersCreateInfoEXT;
///```
///# Members
///A [`VideoDecodeH264SessionParametersCreateInfoEXT`] structure holding
///one H.264 SPS and at least one H.264 PPS paramater set **must** be chained to
///[`VideoSessionParametersCreateInfoKHR`] when calling
///[`CreateVideoSessionParametersKHR`] to store these parameter set(s) with
///the decoder parameter set object for later reference.
///The provided H.264 SPS/PPS parameters **must** be within the limits specified
///during decoder creation for the decoder specified in
///[`VideoSessionParametersCreateInfoKHR`].
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`max_sps_std_count`] is the maximum number of SPS parameters that the
///   [`VideoSessionParametersKHR`] can contain.
/// - [`max_pps_std_count`] is the maximum number of PPS parameters that the
///   [`VideoSessionParametersKHR`] can contain.
/// - [`parameters_add_info`] is `NULL` or a pointer to a
///   [`VideoDecodeH264SessionParametersAddInfoEXT`] structure specifying H.264 parameters to add
///   upon object creation.
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_SESSION_PARAMETERS_CREATE_INFO_EXT`
/// - If [`parameters_add_info`] is not `NULL`, [`parameters_add_info`]**must** be a valid pointer
///   to a valid [`VideoDecodeH264SessionParametersAddInfoEXT`] structure
///# Related
/// - [`VK_EXT_video_decode_h264`]
/// - [`StructureType`]
/// - [`VideoDecodeH264SessionParametersAddInfoEXT`]
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
pub struct VideoDecodeH264SessionParametersCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`max_sps_std_count`] is the maximum number of SPS parameters that the
    ///[`VideoSessionParametersKHR`] can contain.
    max_sps_std_count: u32,
    ///[`max_pps_std_count`] is the maximum number of PPS parameters that the
    ///[`VideoSessionParametersKHR`] can contain.
    max_pps_std_count: u32,
    ///[`parameters_add_info`] is `NULL` or a pointer to a
    ///[`VideoDecodeH264SessionParametersAddInfoEXT`] structure specifying
    ///H.264 parameters to add upon object creation.
    parameters_add_info: *const VideoDecodeH264SessionParametersAddInfoEXT<'lt>,
}
impl<'lt> Default for VideoDecodeH264SessionParametersCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            max_sps_std_count: 0,
            max_pps_std_count: 0,
            parameters_add_info: std::ptr::null(),
        }
    }
}
impl<'lt> VideoDecodeH264SessionParametersCreateInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::max_sps_std_count`]
    pub fn max_sps_std_count_raw(&self) -> u32 {
        self.max_sps_std_count
    }
    ///Gets the raw value of [`Self::max_pps_std_count`]
    pub fn max_pps_std_count_raw(&self) -> u32 {
        self.max_pps_std_count
    }
    ///Gets the raw value of [`Self::parameters_add_info`]
    pub fn parameters_add_info_raw(&self) -> *const VideoDecodeH264SessionParametersAddInfoEXT<'lt> {
        self.parameters_add_info
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::max_sps_std_count`]
    pub fn set_max_sps_std_count_raw(&mut self, value: u32) -> &mut Self {
        self.max_sps_std_count = value;
        self
    }
    ///Sets the raw value of [`Self::max_pps_std_count`]
    pub fn set_max_pps_std_count_raw(&mut self, value: u32) -> &mut Self {
        self.max_pps_std_count = value;
        self
    }
    ///Sets the raw value of [`Self::parameters_add_info`]
    pub fn set_parameters_add_info_raw(
        &mut self,
        value: *const VideoDecodeH264SessionParametersAddInfoEXT<'lt>,
    ) -> &mut Self {
        self.parameters_add_info = value;
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
    ///Gets the value of [`Self::max_sps_std_count`]
    pub fn max_sps_std_count(&self) -> u32 {
        self.max_sps_std_count
    }
    ///Gets the value of [`Self::max_pps_std_count`]
    pub fn max_pps_std_count(&self) -> u32 {
        self.max_pps_std_count
    }
    ///Gets the value of [`Self::parameters_add_info`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn parameters_add_info(&self) -> &VideoDecodeH264SessionParametersAddInfoEXT<'lt> {
        &*self.parameters_add_info
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::max_sps_std_count`]
    pub fn max_sps_std_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_pps_std_count`]
    pub fn max_pps_std_count_mut(&mut self) -> &mut u32 {
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
    ///Sets the raw value of [`Self::max_sps_std_count`]
    pub fn set_max_sps_std_count(&mut self, value: u32) -> &mut Self {
        self.max_sps_std_count = value;
        self
    }
    ///Sets the raw value of [`Self::max_pps_std_count`]
    pub fn set_max_pps_std_count(&mut self, value: u32) -> &mut Self {
        self.max_pps_std_count = value;
        self
    }
    ///Sets the raw value of [`Self::parameters_add_info`]
    pub fn set_parameters_add_info(
        &mut self,
        value: &'lt crate::extensions::ext_video_decode_h_264::VideoDecodeH264SessionParametersAddInfoEXT<'lt>,
    ) -> &mut Self {
        self.parameters_add_info = value as *const _;
        self
    }
}
///[VkVideoDecodeH264PictureInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH264PictureInfoEXT.html) - Structure specifies H.264 decode picture parameters when decoding a picture
///# C Specifications
///The [`VideoDecodeH264PictureInfoEXT`] structure represents a picture
///decode operation and is defined as:
///```c
///// Provided by VK_EXT_video_decode_h264
///typedef struct VkVideoDecodeH264PictureInfoEXT {
///    VkStructureType                         sType;
///    const void*                             pNext;
///    const StdVideoDecodeH264PictureInfo*    pStdPictureInfo;
///    uint32_t                                slicesCount;
///    const uint32_t*                         pSlicesDataOffsets;
///} VkVideoDecodeH264PictureInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`std_picture_info`] is a pointer to a [`StdVideoDecodeH264PictureInfo`] structure specifying
///   the codec standard specific picture information from the H.264 specification.
/// - [`slices_count`] is the number of slices in this picture.
/// - [`slices_data_offsets`] is a pointer to an array of [`slices_count`] offsets indicating the
///   start offset of each slice within the bitstream buffer.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_PICTURE_INFO_EXT`
/// - [`std_picture_info`]**must** be a valid pointer to a valid [`StdVideoDecodeH264PictureInfo`]
///   value
/// - [`slices_data_offsets`]**must** be a valid pointer to an array of [`slices_count`]`uint32_t`
///   values
/// - [`slices_count`]**must** be greater than `0`
///# Related
/// - [`VK_EXT_video_decode_h264`]
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
pub struct VideoDecodeH264PictureInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`std_picture_info`] is a pointer to a
    ///[`StdVideoDecodeH264PictureInfo`] structure specifying the codec
    ///standard specific picture information from the H.264 specification.
    std_picture_info: *const StdVideoDecodeH264PictureInfo,
    ///[`slices_count`] is the number of slices in this picture.
    slices_count: u32,
    ///[`slices_data_offsets`] is a pointer to an array of [`slices_count`]
    ///offsets indicating the start offset of each slice within the bitstream
    ///buffer.
    slices_data_offsets: *const u32,
}
impl<'lt> Default for VideoDecodeH264PictureInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            std_picture_info: std::ptr::null(),
            slices_count: 0,
            slices_data_offsets: std::ptr::null(),
        }
    }
}
impl<'lt> VideoDecodeH264PictureInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::slices_count`]
    pub fn slices_count_raw(&self) -> u32 {
        self.slices_count
    }
    ///Gets the raw value of [`Self::slices_data_offsets`]
    pub fn slices_data_offsets_raw(&self) -> *const u32 {
        self.slices_data_offsets
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::slices_count`]
    pub fn set_slices_count_raw(&mut self, value: u32) -> &mut Self {
        self.slices_count = value;
        self
    }
    ///Sets the raw value of [`Self::slices_data_offsets`]
    pub fn set_slices_data_offsets_raw(&mut self, value: *const u32) -> &mut Self {
        self.slices_data_offsets = value;
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
    ///Gets the value of [`Self::std_picture_info`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn std_picture_info(&self) -> &StdVideoDecodeH264PictureInfo {
        &*self.std_picture_info
    }
    ///Gets the value of [`Self::slices_count`]
    pub fn slices_count(&self) -> u32 {
        self.slices_count
    }
    ///Gets the value of [`Self::slices_data_offsets`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn slices_data_offsets(&self) -> &[u32] {
        std::slice::from_raw_parts(self.slices_data_offsets, self.slices_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::slices_count`]
    pub fn slices_count_mut(&mut self) -> &mut u32 {
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
    ///Sets the raw value of [`Self::std_picture_info`]
    pub fn set_std_picture_info(&mut self, value: &'lt crate::native::StdVideoDecodeH264PictureInfo) -> &mut Self {
        self.std_picture_info = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::slices_count`]
    pub fn set_slices_count(&mut self, value: u32) -> &mut Self {
        self.slices_count = value;
        self
    }
    ///Sets the raw value of [`Self::slices_data_offsets`]
    pub fn set_slices_data_offsets(&mut self, value: &'lt [u32]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.slices_data_offsets = value.as_ptr();
        self.slices_count = len_;
        self
    }
}
///[VkVideoDecodeH264DpbSlotInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH264DpbSlotInfoEXT.html) - Structure specifies H.264 decode DPB picture information
///# C Specifications
///The [`VideoDecodeH264DpbSlotInfoEXT`] structure correlates a DPB Slot
///index with codec-specific information and is defined as:
///```c
///// Provided by VK_EXT_video_decode_h264
///typedef struct VkVideoDecodeH264DpbSlotInfoEXT {
///    VkStructureType                           sType;
///    const void*                               pNext;
///    const StdVideoDecodeH264ReferenceInfo*    pStdReferenceInfo;
///} VkVideoDecodeH264DpbSlotInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`std_reference_info`] is a pointer to a [`StdVideoDecodeH264ReferenceInfo`] structure
///   specifying the codec standard specific picture reference information from the H.264
///   specification.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_DPB_SLOT_INFO_EXT`
/// - [`std_reference_info`]**must** be a valid pointer to a valid
///   [`StdVideoDecodeH264ReferenceInfo`] value
///# Related
/// - [`VK_EXT_video_decode_h264`]
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
pub struct VideoDecodeH264DpbSlotInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///No documentation found
    p_next: *const BaseInStructure<'lt>,
    ///[`std_reference_info`] is a pointer to a
    ///[`StdVideoDecodeH264ReferenceInfo`] structure specifying the codec
    ///standard specific picture reference information from the H.264
    ///specification.
    std_reference_info: *const StdVideoDecodeH264ReferenceInfo,
}
impl<'lt> Default for VideoDecodeH264DpbSlotInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            std_reference_info: std::ptr::null(),
        }
    }
}
impl<'lt> VideoDecodeH264DpbSlotInfoEXT<'lt> {
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
    ///Gets the value of [`Self::std_reference_info`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn std_reference_info(&self) -> &StdVideoDecodeH264ReferenceInfo {
        &*self.std_reference_info
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
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
    ///Sets the raw value of [`Self::std_reference_info`]
    pub fn set_std_reference_info(&mut self, value: &'lt crate::native::StdVideoDecodeH264ReferenceInfo) -> &mut Self {
        self.std_reference_info = value as *const _;
        self
    }
}
///[VkVideoDecodeH264MvcEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH264MvcEXT.html) - Structure specifies parameters of mvc views
///# C Specifications
///The [`VideoDecodeH264MvcEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_video_decode_h264
///typedef struct VkVideoDecodeH264MvcEXT {
///    VkStructureType                 sType;
///    const void*                     pNext;
///    const StdVideoDecodeH264Mvc*    pStdMvc;
///} VkVideoDecodeH264MvcEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`std_mvc`] is a pointer to a [`StdVideoDecodeH264Mvc`] structure specifying H.264 codec
///   specification information for MVC.
///# Description
///When the content type is H.264 MVC, a [`VideoDecodeH264MvcEXT`]
///structure **must** be chained to [`VideoDecodeH264PictureInfoEXT`].Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_MVC_EXT`
/// - [`std_mvc`]**must** be a valid pointer to a valid [`StdVideoDecodeH264Mvc`] value
///# Related
/// - [`VK_EXT_video_decode_h264`]
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
pub struct VideoDecodeH264MvcEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`std_mvc`] is a pointer to a [`StdVideoDecodeH264Mvc`] structure
    ///specifying H.264 codec specification information for MVC.
    std_mvc: *const StdVideoDecodeH264Mvc,
}
impl<'lt> Default for VideoDecodeH264MvcEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            std_mvc: std::ptr::null(),
        }
    }
}
impl<'lt> VideoDecodeH264MvcEXT<'lt> {
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
    ///Gets the value of [`Self::std_mvc`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn std_mvc(&self) -> &StdVideoDecodeH264Mvc {
        &*self.std_mvc
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
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
    ///Sets the raw value of [`Self::std_mvc`]
    pub fn set_std_mvc(&mut self, value: &'lt crate::native::StdVideoDecodeH264Mvc) -> &mut Self {
        self.std_mvc = value as *const _;
        self
    }
}
