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
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VideoDecodeH264ProfileEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`std_profile_idc`] is a [`StdVideoH264ProfileIdc`] value specifying
    ///the H.264 codec profile IDC
    std_profile_idc: StdVideoH264ProfileIdc,
    ///[`picture_layout`] is a bitmask of
    ///[`VideoDecodeH264PictureLayoutFlagBitsEXT`] specifying the layout of
    ///the decoded picture’s contents depending on the nature (progressive vs.
    ///interlaced) of the input content.
    picture_layout: VideoDecodeH264PictureLayoutFlagsEXT,
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VideoDecodeH264CapabilitiesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
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
/// - [`p_std_extension_version`] is a pointer to a [`ExtensionProperties`] structure specifying the
///   H.264 codec extensions defined in `StdVideoH264Extensions`.
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_SESSION_CREATE_INFO_EXT`
/// - [`flags`]**must** be `0`
/// - [`p_std_extension_version`]**must** be a valid pointer to a valid [`ExtensionProperties`]
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VideoDecodeH264SessionCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: VideoDecodeH264CreateFlagsEXT,
    ///[`p_std_extension_version`] is a pointer to a [`ExtensionProperties`]
    ///structure specifying the H.264 codec extensions defined in
    ///`StdVideoH264Extensions`.
    p_std_extension_version: *mut ExtensionProperties,
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
/// - [`sps_std_count`] is the number of SPS elements in [`p_sps_std`]. Its value **must** be less
///   than or equal to the value of `maxSpsStdCount`.
/// - [`p_sps_std`] is a pointer to an array of [`StdVideoH264SequenceParameterSet`] structures
///   representing H.264 sequence parameter sets. Each element of the array **must** have a unique
///   H.264 SPS ID.
/// - [`pps_std_count`] is the number of PPS provided in [`p_pps_std`]. Its value **must** be less
///   than or equal to the value of `maxPpsStdCount`.
/// - [`p_pps_std`] is a pointer to an array of [`StdVideoH264PictureParameterSet`] structures
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
/// - If [`p_sps_std`] is not `NULL`, [`p_sps_std`]**must** be a valid pointer to an array of
///   [`sps_std_count`][`StdVideoH264SequenceParameterSet`] values
/// - If [`p_pps_std`] is not `NULL`, [`p_pps_std`]**must** be a valid pointer to an array of
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VideoDecodeH264SessionParametersAddInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`sps_std_count`] is the number of SPS elements in [`p_sps_std`].
    ///Its value **must** be less than or equal to the value of
    ///`maxSpsStdCount`.
    sps_std_count: u32,
    ///[`p_sps_std`] is a pointer to an array of
    ///[`StdVideoH264SequenceParameterSet`] structures representing H.264
    ///sequence parameter sets.
    ///Each element of the array **must** have a unique H.264 SPS ID.
    p_sps_std: *mut StdVideoH264SequenceParameterSet,
    ///[`pps_std_count`] is the number of PPS provided in [`p_pps_std`].
    ///Its value **must** be less than or equal to the value of
    ///`maxPpsStdCount`.
    pps_std_count: u32,
    ///[`p_pps_std`] is a pointer to an array of
    ///[`StdVideoH264PictureParameterSet`] structures representing H.264
    ///picture parameter sets.
    ///Each element of the array **must** have a unique H.264 SPS-PPS ID pair.
    p_pps_std: *mut StdVideoH264PictureParameterSet,
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
/// - [`p_parameters_add_info`] is `NULL` or a pointer to a
///   [`VideoDecodeH264SessionParametersAddInfoEXT`] structure specifying H.264 parameters to add
///   upon object creation.
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_SESSION_PARAMETERS_CREATE_INFO_EXT`
/// - If [`p_parameters_add_info`] is not `NULL`, [`p_parameters_add_info`]**must** be a valid
///   pointer to a valid [`VideoDecodeH264SessionParametersAddInfoEXT`] structure
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VideoDecodeH264SessionParametersCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`max_sps_std_count`] is the maximum number of SPS parameters that the
    ///[`VideoSessionParametersKHR`] can contain.
    max_sps_std_count: u32,
    ///[`max_pps_std_count`] is the maximum number of PPS parameters that the
    ///[`VideoSessionParametersKHR`] can contain.
    max_pps_std_count: u32,
    ///[`p_parameters_add_info`] is `NULL` or a pointer to a
    ///[`VideoDecodeH264SessionParametersAddInfoEXT`] structure specifying
    ///H.264 parameters to add upon object creation.
    p_parameters_add_info: *mut VideoDecodeH264SessionParametersAddInfoEXT<'lt>,
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
/// - [`p_std_picture_info`] is a pointer to a [`StdVideoDecodeH264PictureInfo`] structure
///   specifying the codec standard specific picture information from the H.264 specification.
/// - [`slices_count`] is the number of slices in this picture.
/// - [`p_slices_data_offsets`] is a pointer to an array of [`slices_count`] offsets indicating the
///   start offset of each slice within the bitstream buffer.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_PICTURE_INFO_EXT`
/// - [`p_std_picture_info`]**must** be a valid pointer to a valid [`StdVideoDecodeH264PictureInfo`]
///   value
/// - [`p_slices_data_offsets`]**must** be a valid pointer to an array of [`slices_count`]`uint32_t`
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VideoDecodeH264PictureInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`p_std_picture_info`] is a pointer to a
    ///[`StdVideoDecodeH264PictureInfo`] structure specifying the codec
    ///standard specific picture information from the H.264 specification.
    p_std_picture_info: *mut StdVideoDecodeH264PictureInfo,
    ///[`slices_count`] is the number of slices in this picture.
    slices_count: u32,
    ///[`p_slices_data_offsets`] is a pointer to an array of [`slices_count`]
    ///offsets indicating the start offset of each slice within the bitstream
    ///buffer.
    p_slices_data_offsets: *mut u32,
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
/// - [`p_std_reference_info`] is a pointer to a [`StdVideoDecodeH264ReferenceInfo`] structure
///   specifying the codec standard specific picture reference information from the H.264
///   specification.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_DPB_SLOT_INFO_EXT`
/// - [`p_std_reference_info`]**must** be a valid pointer to a valid
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VideoDecodeH264DpbSlotInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///No documentation found
    p_next: *mut BaseInStructure<'lt>,
    ///[`p_std_reference_info`] is a pointer to a
    ///[`StdVideoDecodeH264ReferenceInfo`] structure specifying the codec
    ///standard specific picture reference information from the H.264
    ///specification.
    p_std_reference_info: *mut StdVideoDecodeH264ReferenceInfo,
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
/// - [`p_std_mvc`] is a pointer to a [`StdVideoDecodeH264Mvc`] structure specifying H.264 codec
///   specification information for MVC.
///# Description
///When the content type is H.264 MVC, a [`VideoDecodeH264MvcEXT`]
///structure **must** be chained to [`VideoDecodeH264PictureInfoEXT`].Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_MVC_EXT`
/// - [`p_std_mvc`]**must** be a valid pointer to a valid [`StdVideoDecodeH264Mvc`] value
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VideoDecodeH264MvcEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`p_std_mvc`] is a pointer to a [`StdVideoDecodeH264Mvc`] structure
    ///specifying H.264 codec specification information for MVC.
    p_std_mvc: *mut StdVideoDecodeH264Mvc,
}
