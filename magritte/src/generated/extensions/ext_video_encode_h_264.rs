use crate::{
    native::{
        StdVideoEncodeH264PictureInfo, StdVideoEncodeH264RefMemMgmtCtrlOperations, StdVideoEncodeH264ReferenceInfo,
        StdVideoEncodeH264SliceHeader, StdVideoH264PictureParameterSet, StdVideoH264ProfileIdc,
        StdVideoH264SequenceParameterSet,
    },
    vulkan1_0::{BaseInStructure, Bool32, ExtensionProperties, Extent2D, StructureType},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_VIDEO_ENCODE_H264_SPEC_VERSION")]
pub const EXT_VIDEO_ENCODE_H264_SPEC_VERSION: u32 = 5;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_VIDEO_ENCODE_H264_EXTENSION_NAME")]
pub const EXT_VIDEO_ENCODE_H264_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_video_encode_h264");
///[VkVideoEncodeH264CapabilitiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264CapabilitiesEXT.html) - Structure specifying H.264 encode capabilities
///# C Specifications
///When calling [`GetPhysicalDeviceVideoCapabilitiesKHR`] with
///`pVideoProfile->videoCodecOperation` specified as
///`VK_VIDEO_CODEC_OPERATION_ENCODE_H264_BIT_EXT`, the
///[`VideoEncodeH264CapabilitiesEXT`] structure **must** be included in the
///[`p_next`] chain of the [`VideoCapabilitiesKHR`] structure to retrieve
///more capabilities specific to H.264 video encoding.The [`VideoEncodeH264CapabilitiesEXT`]
/// structure is defined as:
///```c
///// Provided by VK_EXT_video_encode_h264
///typedef struct VkVideoEncodeH264CapabilitiesEXT {
///    VkStructureType                        sType;
///    const void*                            pNext;
///    VkVideoEncodeH264CapabilityFlagsEXT    flags;
///    VkVideoEncodeH264InputModeFlagsEXT     inputModeFlags;
///    VkVideoEncodeH264OutputModeFlagsEXT    outputModeFlags;
///    uint8_t                                maxPPictureL0ReferenceCount;
///    uint8_t                                maxBPictureL0ReferenceCount;
///    uint8_t                                maxL1ReferenceCount;
///    VkBool32                               motionVectorsOverPicBoundariesFlag;
///    uint32_t                               maxBytesPerPicDenom;
///    uint32_t                               maxBitsPerMbDenom;
///    uint32_t                               log2MaxMvLengthHorizontal;
///    uint32_t                               log2MaxMvLengthVertical;
///    VkExtensionProperties                  stdExtensionVersion;
///} VkVideoEncodeH264CapabilitiesEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is a bitmask of [`VideoEncodeH264CapabilityFlagBitsEXT`] describing supported
///   encoding tools.
/// - [`input_mode_flags`] is a bitmask of [`VideoEncodeH264InputModeFlagBitsEXT`] describing
///   supported command buffer input granularities/modes.
/// - [`output_mode_flags`] is a bitmask of [`VideoEncodeH264OutputModeFlagBitsEXT`] describing
///   supported output (bitstream size reporting) granularities/modes.
/// - [`max_p_picture_l_0_reference_count`] reports the maximum number of reference pictures the
///   implementation supports in the reference list L0 for P pictures.
/// - [`max_b_picture_l_0_reference_count`] reports the maximum number of reference pictures the
///   implementation supports in the reference list L0 for B pictures. The reported value is `0` if
///   encoding of B pictures is not supported.
/// - [`max_l_1_reference_count`] reports the maximum number of reference pictures the
///   implementation supports in the reference list L1 if encoding of B pictures is supported. The
///   reported value is `0` if encoding of B pictures is not supported.
/// - [`motion_vectors_over_pic_boundaries_flag`] if [`TRUE`], indicates
///   motion_vectors_over_pic_boundaries_flag will be enabled if bitstream_restriction_flag is
///   enabled in StdVideoH264SpsVuiFlags.
/// - [`max_bytes_per_pic_denom`] reports the value that will be used for max_bytes_per_pic_denom if
///   bitstream_restriction_flag is enabled in StdVideoH264SpsVuiFlags.
/// - [`max_bits_per_mb_denom`] reports the value that will be used for max_bits_per_mb_denom if
///   bitstream_restriction_flag is enabled in StdVideoH264SpsVuiFlags.
/// - [`log_2_max_mv_length_horizontal`] reports the value that will be used for
///   log2_max_mv_length_horizontal if bitstream_restriction_flag is enabled in
///   StdVideoH264SpsVuiFlags.
/// - [`log_2_max_mv_length_vertical`] reports the value that will be used for
///   log2_max_mv_length_vertical if bitstream_restriction_flag is enabled in
///   StdVideoH264SpsVuiFlags.
/// - [`std_extension_version`] is the specific H.264 extension name and version supported by this
///   implementation.
///# Description
///When [`GetPhysicalDeviceVideoCapabilitiesKHR`] is called to query the
///capabilities with parameter `videoCodecOperation` specified as
///`VK_VIDEO_CODEC_OPERATION_ENCODE_H264_BIT_EXT`, a
///[`VideoEncodeH264CapabilitiesEXT`] structure **can** be chained to
///[`VideoCapabilitiesKHR`] to retrieve H.264 extension specific
///capabilities.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_CAPABILITIES_EXT`
/// - [`input_mode_flags`]**must** be a valid combination of [`VideoEncodeH264InputModeFlagBitsEXT`]
///   values
/// - [`input_mode_flags`]**must** not be `0`
/// - [`output_mode_flags`]**must** be a valid combination of
///   [`VideoEncodeH264OutputModeFlagBitsEXT`] values
/// - [`output_mode_flags`]**must** not be `0`
/// - [`std_extension_version`]**must** be a valid [`ExtensionProperties`] structure
///# Related
/// - [`VK_EXT_video_encode_h264`]
/// - [`Bool32`]
/// - [`ExtensionProperties`]
/// - [`StructureType`]
/// - [`VideoEncodeH264CapabilityFlagsEXT`]
/// - [`VideoEncodeH264InputModeFlagsEXT`]
/// - [`VideoEncodeH264OutputModeFlagsEXT`]
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
pub struct VideoEncodeH264CapabilitiesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`VideoEncodeH264CapabilityFlagBitsEXT`]
    ///describing supported encoding tools.
    flags: VideoEncodeH264CapabilityFlagsEXT,
    ///[`input_mode_flags`] is a bitmask of
    ///[`VideoEncodeH264InputModeFlagBitsEXT`] describing supported command
    ///buffer input granularities/modes.
    input_mode_flags: VideoEncodeH264InputModeFlagsEXT,
    ///[`output_mode_flags`] is a bitmask of
    ///[`VideoEncodeH264OutputModeFlagBitsEXT`] describing supported output
    ///(bitstream size reporting) granularities/modes.
    output_mode_flags: VideoEncodeH264OutputModeFlagsEXT,
    ///[`max_p_picture_l_0_reference_count`] reports the maximum number of
    ///reference pictures the implementation supports in the reference list L0
    ///for P pictures.
    max_p_picture_l_0_reference_count: u8,
    ///[`max_b_picture_l_0_reference_count`] reports the maximum number of
    ///reference pictures the implementation supports in the reference list L0
    ///for B pictures.
    ///The reported value is `0` if encoding of B pictures is not supported.
    max_b_picture_l_0_reference_count: u8,
    ///[`max_l_1_reference_count`] reports the maximum number of reference
    ///pictures the implementation supports in the reference list L1 if
    ///encoding of B pictures is supported.
    ///The reported value is `0` if encoding of B pictures is not supported.
    max_l_1_reference_count: u8,
    ///[`motion_vectors_over_pic_boundaries_flag`] if [`TRUE`], indicates
    ///motion_vectors_over_pic_boundaries_flag will be enabled if
    ///bitstream_restriction_flag is enabled in StdVideoH264SpsVuiFlags.
    motion_vectors_over_pic_boundaries_flag: Bool32,
    ///[`max_bytes_per_pic_denom`] reports the value that will be used for
    ///max_bytes_per_pic_denom if bitstream_restriction_flag is enabled in
    ///StdVideoH264SpsVuiFlags.
    max_bytes_per_pic_denom: u32,
    ///[`max_bits_per_mb_denom`] reports the value that will be used for
    ///max_bits_per_mb_denom if bitstream_restriction_flag is enabled in
    ///StdVideoH264SpsVuiFlags.
    max_bits_per_mb_denom: u32,
    ///[`log_2_max_mv_length_horizontal`] reports the value that will be used for
    ///log2_max_mv_length_horizontal if bitstream_restriction_flag is enabled
    ///in StdVideoH264SpsVuiFlags.
    log_2_max_mv_length_horizontal: u32,
    ///[`log_2_max_mv_length_vertical`] reports the value that will be used for
    ///log2_max_mv_length_vertical if bitstream_restriction_flag is enabled in
    ///StdVideoH264SpsVuiFlags.
    log_2_max_mv_length_vertical: u32,
    ///[`std_extension_version`] is the specific H.264 extension name and
    ///version supported by this implementation.
    std_extension_version: ExtensionProperties,
}
///[VkVideoEncodeH264SessionCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264SessionCreateInfoEXT.html) - Structure specifies H.264 encoder creation parameters
///# C Specifications
///The [`VideoEncodeH264SessionCreateInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_video_encode_h264
///typedef struct VkVideoEncodeH264SessionCreateInfoEXT {
///    VkStructureType                    sType;
///    const void*                        pNext;
///    VkVideoEncodeH264CreateFlagsEXT    flags;
///    VkExtent2D                         maxPictureSizeInMbs;
///    const VkExtensionProperties*       pStdExtensionVersion;
///} VkVideoEncodeH264SessionCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is a bitmask of [`VideoEncodeH264CreateFlagsEXT`] specifying H.264 encoder creation
///   flags.
/// - [`max_picture_size_in_mbs`] specifies the syntax element pic_width_in_mbs_minus1 + 1 and the
///   syntax element pic_height_in_map_units_minus1 + 1.
/// - [`p_std_extension_version`] is a pointer to a [`ExtensionProperties`] structure specifying
///   H.264 codec extensions.
///# Description
///A [`VideoEncodeH264SessionCreateInfoEXT`] structure **must** be chained to
///[`VideoSessionCreateInfoKHR`] when the function
///[`CreateVideoSessionKHR`] is called with `videoCodecOperation` in
///[`VideoSessionCreateInfoKHR`] set to
///`VK_VIDEO_CODEC_OPERATION_ENCODE_H264_BIT_EXT`.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_SESSION_CREATE_INFO_EXT`
/// - [`flags`]**must** be a valid combination of [`VideoEncodeH264CreateFlagBitsEXT`] values
/// - [`flags`]**must** not be `0`
/// - [`p_std_extension_version`]**must** be a valid pointer to a valid [`ExtensionProperties`]
///   structure
///# Related
/// - [`VK_EXT_video_encode_h264`]
/// - [`ExtensionProperties`]
/// - [`Extent2D`]
/// - [`StructureType`]
/// - [`VideoEncodeH264CreateFlagsEXT`]
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
pub struct VideoEncodeH264SessionCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`VideoEncodeH264CreateFlagsEXT`]
    ///specifying H.264 encoder creation flags.
    flags: VideoEncodeH264CreateFlagsEXT,
    ///[`max_picture_size_in_mbs`] specifies the syntax element
    ///pic_width_in_mbs_minus1 + 1 and the syntax element
    ///pic_height_in_map_units_minus1 + 1.
    max_picture_size_in_mbs: Extent2D,
    ///[`p_std_extension_version`] is a pointer to a [`ExtensionProperties`]
    ///structure specifying H.264 codec extensions.
    p_std_extension_version: *mut ExtensionProperties,
}
///[VkVideoEncodeH264SessionParametersAddInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264SessionParametersAddInfoEXT.html) - Structure specifies H.264 encoder parameter set information
///# C Specifications
///The [`VideoEncodeH264SessionParametersAddInfoEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_video_encode_h264
///typedef struct VkVideoEncodeH264SessionParametersAddInfoEXT {
///    VkStructureType                            sType;
///    const void*                                pNext;
///    uint32_t                                   spsStdCount;
///    const StdVideoH264SequenceParameterSet*    pSpsStd;
///    uint32_t                                   ppsStdCount;
///    const StdVideoH264PictureParameterSet*     pPpsStd;
///} VkVideoEncodeH264SessionParametersAddInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`sps_std_count`] is the number of SPS elements in the [`p_sps_std`]. Its value **must** be
///   less than or equal to the value of `maxSpsStdCount`.
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
///   to the object. `VK_ERROR_TOO_MANY_OBJECTS` will be returned if an attempt is made to add
///   additional data to this object at this point
/// - When the `maxPpsStdCount` number of parameters of type StdVideoH264PictureParameterSet in the
///   Video Session Parameters object is reached, no additional parameters of that type can be added
///   to the object. `VK_ERROR_TOO_MANY_OBJECTS` will be returned if an attempt is made to add
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
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_SESSION_PARAMETERS_ADD_INFO_EXT`
/// - If [`p_sps_std`] is not `NULL`, [`p_sps_std`]**must** be a valid pointer to an array of
///   [`sps_std_count`][`StdVideoH264SequenceParameterSet`] values
/// - If [`p_pps_std`] is not `NULL`, [`p_pps_std`]**must** be a valid pointer to an array of
///   [`pps_std_count`][`StdVideoH264PictureParameterSet`] values
/// - [`sps_std_count`]**must** be greater than `0`
/// - [`pps_std_count`]**must** be greater than `0`
///# Related
/// - [`VK_EXT_video_encode_h264`]
/// - [`StructureType`]
/// - [`VideoEncodeH264SessionParametersCreateInfoEXT`]
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
pub struct VideoEncodeH264SessionParametersAddInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`sps_std_count`] is the number of SPS elements in the [`p_sps_std`].
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
///[VkVideoEncodeH264SessionParametersCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264SessionParametersCreateInfoEXT.html) - Structure specifies H.264 encoder parameter set information
///# C Specifications
///The [`VideoEncodeH264SessionParametersCreateInfoEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_video_encode_h264
///typedef struct VkVideoEncodeH264SessionParametersCreateInfoEXT {
///    VkStructureType                                        sType;
///    const void*                                            pNext;
///    uint32_t                                               maxSpsStdCount;
///    uint32_t                                               maxPpsStdCount;
///    const VkVideoEncodeH264SessionParametersAddInfoEXT*    pParametersAddInfo;
///} VkVideoEncodeH264SessionParametersCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`max_sps_std_count`] is the maximum number of SPS parameters that the
///   [`VideoSessionParametersKHR`] can contain.
/// - [`max_pps_std_count`] is the maximum number of PPS parameters that the
///   [`VideoSessionParametersKHR`] can contain.
/// - [`p_parameters_add_info`] is `NULL` or a pointer to a
///   [`VideoEncodeH264SessionParametersAddInfoEXT`] structure specifying H.264 parameters to add
///   upon object creation.
///# Description
///A [`VideoEncodeH264SessionParametersCreateInfoEXT`] structure holding
///one H.264 SPS and at least one H.264 PPS paramater set **must** be chained to
///[`VideoSessionParametersCreateInfoKHR`] when calling
///[`CreateVideoSessionParametersKHR`] to store these parameter set(s) with
///the encoder parameter set object for later reference.
///The provided H.264 SPS/PPS parameters **must** be within the limits specified
///during encoder creation for the encoder specified in
///[`VideoSessionParametersCreateInfoKHR`].Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_SESSION_PARAMETERS_CREATE_INFO_EXT`
/// - If [`p_parameters_add_info`] is not `NULL`, [`p_parameters_add_info`]**must** be a valid
///   pointer to a valid [`VideoEncodeH264SessionParametersAddInfoEXT`] structure
///# Related
/// - [`VK_EXT_video_encode_h264`]
/// - [`StructureType`]
/// - [`VideoEncodeH264SessionParametersAddInfoEXT`]
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
pub struct VideoEncodeH264SessionParametersCreateInfoEXT<'lt> {
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
    ///[`VideoEncodeH264SessionParametersAddInfoEXT`] structure specifying
    ///H.264 parameters to add upon object creation.
    p_parameters_add_info: *mut VideoEncodeH264SessionParametersAddInfoEXT<'lt>,
}
///[VkVideoEncodeH264DpbSlotInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264DpbSlotInfoEXT.html) - Structure specifies H.264 encode DPB picture information
///# C Specifications
///The [`VideoEncodeH264DpbSlotInfoEXT`] structure, representing a
///reconstructed picture that is being used as a reference picture, is defined
///as:
///```c
///// Provided by VK_EXT_video_encode_h264
///typedef struct VkVideoEncodeH264DpbSlotInfoEXT {
///    VkStructureType                           sType;
///    const void*                               pNext;
///    int8_t                                    slotIndex;
///    const StdVideoEncodeH264ReferenceInfo*    pStdReferenceInfo;
///} VkVideoEncodeH264DpbSlotInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`slot_index`] is the [DPB Slot](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#dpb-slot)
///   index for this picture. [`slot_index`]**must** match the [`slot_index`] in
///   `pSetupReferenceSlot` of [`VideoEncodeInfoKHR`] in the command used to encode the
///   corresponding picture.
/// - [`p_std_reference_info`] is a pointer to a [`StdVideoEncodeH264ReferenceInfo`] structure
///   specifying the syntax and other codec-specific information from the H.264 specification
///   associated with this reference picture.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_DPB_SLOT_INFO_EXT`
/// - [`p_next`]**must** be `NULL`
/// - [`p_std_reference_info`]**must** be a valid pointer to a valid
///   [`StdVideoEncodeH264ReferenceInfo`] value
///# Related
/// - [`VK_EXT_video_encode_h264`]
/// - [`StructureType`]
/// - [`VideoEncodeH264ReferenceListsEXT`]
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
pub struct VideoEncodeH264DpbSlotInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`slot_index`] is the [DPB Slot](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#dpb-slot) index for this picture.
    ///[`slot_index`]**must** match the [`slot_index`] in
    ///`pSetupReferenceSlot` of [`VideoEncodeInfoKHR`] in the command
    ///used to encode the corresponding picture.
    slot_index: i8,
    ///[`p_std_reference_info`] is a pointer to a
    ///[`StdVideoEncodeH264ReferenceInfo`] structure specifying the syntax and
    ///other codec-specific information from the H.264 specification associated
    ///with this reference picture.
    p_std_reference_info: *mut StdVideoEncodeH264ReferenceInfo,
}
///[VkVideoEncodeH264VclFrameInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264VclFrameInfoEXT.html) - Structure specifies H.264 encode frame parameters
///# C Specifications
///The [`VideoEncodeH264VclFrameInfoEXT`] structure representing a frame
///encode operation is defined as:
///```c
///// Provided by VK_EXT_video_encode_h264
///typedef struct VkVideoEncodeH264VclFrameInfoEXT {
///    VkStructureType                              sType;
///    const void*                                  pNext;
///    const VkVideoEncodeH264ReferenceListsEXT*    pReferenceFinalLists;
///    uint32_t                                     naluSliceEntryCount;
///    const VkVideoEncodeH264NaluSliceEXT*         pNaluSliceEntries;
///    const StdVideoEncodeH264PictureInfo*         pCurrentPictureInfo;
///} VkVideoEncodeH264VclFrameInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`p_reference_final_lists`] is `NULL` or a pointer to a [`VideoEncodeH264ReferenceListsEXT`]
///   structure specifying the reference lists to be used for the current picture.
/// - [`nalu_slice_entry_count`] is the number of slice NALUs in the frame.
/// - [`p_nalu_slice_entries`] is a pointer to an array of
///   [`nalu_slice_entry_count`][`VideoEncodeH264NaluSliceEXT`] structures specifying the division
///   of the current picture into slices and the properties of these slices. This is an ordered
///   sequence; the NALUs are generated consecutively in
///   [`VideoEncodeInfoKHR::dst_bitstream_buffer`] in the same order as in this array.
/// - [`p_current_picture_info`] is a pointer to a [`StdVideoEncodeH264PictureInfo`] structure
///   specifying the syntax and other codec-specific information from the H.264 specification
///   associated with this picture. The information provided **must** reflect the decoded picture
///   marking operations that are applicable to this frame.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_VCL_FRAME_INFO_EXT`
/// - If [`p_reference_final_lists`] is not `NULL`, [`p_reference_final_lists`]**must** be a valid
///   pointer to a valid [`VideoEncodeH264ReferenceListsEXT`] structure
/// - [`p_nalu_slice_entries`]**must** be a valid pointer to an array of [`nalu_slice_entry_count`]
///   valid [`VideoEncodeH264NaluSliceEXT`] structures
/// - [`p_current_picture_info`]**must** be a valid pointer to a valid
///   [`StdVideoEncodeH264PictureInfo`] value
/// - [`nalu_slice_entry_count`]**must** be greater than `0`
///# Related
/// - [`VK_EXT_video_encode_h264`]
/// - [`StructureType`]
/// - [`VideoEncodeH264NaluSliceEXT`]
/// - [`VideoEncodeH264ReferenceListsEXT`]
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
pub struct VideoEncodeH264VclFrameInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`p_reference_final_lists`] is `NULL` or a pointer to a
    ///[`VideoEncodeH264ReferenceListsEXT`] structure specifying the
    ///reference lists to be used for the current picture.
    p_reference_final_lists: *mut VideoEncodeH264ReferenceListsEXT<'lt>,
    ///[`nalu_slice_entry_count`] is the number of slice NALUs in the frame.
    nalu_slice_entry_count: u32,
    ///[`p_nalu_slice_entries`] is a pointer to an array of
    ///[`nalu_slice_entry_count`][`VideoEncodeH264NaluSliceEXT`] structures
    ///specifying the division of the current picture into slices and the
    ///properties of these slices.
    ///This is an ordered sequence; the NALUs are generated consecutively in
    ///[`VideoEncodeInfoKHR`]::`dstBitstreamBuffer` in the same order
    ///as in this array.
    p_nalu_slice_entries: *mut VideoEncodeH264NaluSliceEXT<'lt>,
    ///[`p_current_picture_info`] is a pointer to a
    ///[`StdVideoEncodeH264PictureInfo`] structure specifying the syntax and
    ///other codec-specific information from the H.264 specification associated
    ///with this picture.
    ///The information provided **must** reflect the decoded picture marking
    ///operations that are applicable to this frame.
    p_current_picture_info: *mut StdVideoEncodeH264PictureInfo,
}
///[VkVideoEncodeH264ReferenceListsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264ReferenceListsEXT.html) - Structure specifies H.264 reference frame lists
///# C Specifications
///The [`VideoEncodeH264ReferenceListsEXT`] structure representing
///reference lists is defined as:
///```c
///// Provided by VK_EXT_video_encode_h264
///typedef struct VkVideoEncodeH264ReferenceListsEXT {
///    VkStructureType                                      sType;
///    const void*                                          pNext;
///    uint8_t                                              referenceList0EntryCount;
///    const VkVideoEncodeH264DpbSlotInfoEXT*               pReferenceList0Entries;
///    uint8_t                                              referenceList1EntryCount;
///    const VkVideoEncodeH264DpbSlotInfoEXT*               pReferenceList1Entries;
///    const StdVideoEncodeH264RefMemMgmtCtrlOperations*    pMemMgmtCtrlOperations;
///} VkVideoEncodeH264ReferenceListsEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`reference_list_0_entry_count`] is the number of reference pictures in reference list L0 and
///   is identical to [`StdVideoEncodeH264SliceHeader`]`::num_ref_idx_l0_active_minus1` + 1.
/// - [`p_reference_list_0_entries`] is a pointer to an array of
///   [`reference_list_0_entry_count`][`VideoEncodeH264DpbSlotInfoEXT`] structures specifying the
///   reference list L0 entries for the current picture. The entries provided **must** be ordered
///   after all reference list L0 modification operations are applied (i.e. final list order). The
///   entries provided **must** not reflect decoded picture marking operations in this frame that
///   are applicable to references; the impact of such operations **must** be reflected in future
///   frame encode commands. The slot index in each entry **must** match one of the slot indexes
///   provided in the `pReferenceSlots` of the parent [`VideoEncodeInfoKHR`] structure.
/// - [`reference_list_1_entry_count`] is the number of reference pictures in reference list L1 and
///   is identical to [`StdVideoEncodeH264SliceHeader`]`::num_ref_idx_l1_active_minus1` + 1.
/// - [`p_reference_list_1_entries`] is a pointer to an array of
///   [`reference_list_1_entry_count`][`VideoEncodeH264DpbSlotInfoEXT`] structures specifying the
///   reference list L1 entries for the current picture. The entries provided **must** be ordered
///   after all reference list L1 modification operations are applied (i.e. final list order). The
///   entries provided **must** not reflect decoded picture marking operations in this frame that
///   are applicable to references; the impact of such operations **must** be reflected in future
///   frame encode commands. The slot index in each entry **must** match one of the slot indexes
///   provided in the `pReferenceSlots` of the parent [`VideoEncodeInfoKHR`] structure.
/// - [`p_mem_mgmt_ctrl_operations`] is a pointer to a
///   [`StdVideoEncodeH264RefMemMgmtCtrlOperations`] structure specifying reference lists
///   modifications and decoded picture marking operations.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_REFERENCE_LISTS_EXT`
/// - [`p_next`]**must** be `NULL`
/// - If [`reference_list_0_entry_count`] is not `0`, [`p_reference_list_0_entries`]**must** be a
///   valid pointer to an array of [`reference_list_0_entry_count`] valid
///   [`VideoEncodeH264DpbSlotInfoEXT`] structures
/// - If [`reference_list_1_entry_count`] is not `0`, [`p_reference_list_1_entries`]**must** be a
///   valid pointer to an array of [`reference_list_1_entry_count`] valid
///   [`VideoEncodeH264DpbSlotInfoEXT`] structures
/// - [`p_mem_mgmt_ctrl_operations`]**must** be a valid pointer to a valid
///   [`StdVideoEncodeH264RefMemMgmtCtrlOperations`] value
///# Related
/// - [`VK_EXT_video_encode_h264`]
/// - [`StructureType`]
/// - [`VideoEncodeH264DpbSlotInfoEXT`]
/// - [`VideoEncodeH264NaluSliceEXT`]
/// - [`VideoEncodeH264VclFrameInfoEXT`]
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
pub struct VideoEncodeH264ReferenceListsEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`reference_list_0_entry_count`] is the number of reference pictures in
    ///reference list L0 and is identical to
    ///[`StdVideoEncodeH264SliceHeader`]::`num_ref_idx_l0_active_minus1` + 1.
    reference_list_0_entry_count: u8,
    ///[`p_reference_list_0_entries`] is a pointer to an array of
    ///[`reference_list_0_entry_count`][`VideoEncodeH264DpbSlotInfoEXT`]
    ///structures specifying the reference list L0 entries for the current
    ///picture.
    ///The entries provided **must** be ordered after all reference list L0
    ///modification operations are applied (i.e. final list order).
    ///The entries provided **must** not reflect decoded picture marking
    ///operations in this frame that are applicable to references; the impact
    ///of such operations **must** be reflected in future frame encode commands.
    ///The slot index in each entry **must** match one of the slot indexes
    ///provided in the `pReferenceSlots` of the parent
    ///[`VideoEncodeInfoKHR`] structure.
    p_reference_list_0_entries: *mut VideoEncodeH264DpbSlotInfoEXT<'lt>,
    ///[`reference_list_1_entry_count`] is the number of reference pictures in
    ///reference list L1 and is identical to
    ///[`StdVideoEncodeH264SliceHeader`]::`num_ref_idx_l1_active_minus1` + 1.
    reference_list_1_entry_count: u8,
    ///[`p_reference_list_1_entries`] is a pointer to an array of
    ///[`reference_list_1_entry_count`][`VideoEncodeH264DpbSlotInfoEXT`]
    ///structures specifying the reference list L1 entries for the current
    ///picture.
    ///The entries provided **must** be ordered after all reference list L1
    ///modification operations are applied (i.e. final list order).
    ///The entries provided **must** not reflect decoded picture marking
    ///operations in this frame that are applicable to references; the impact
    ///of such operations **must** be reflected in future frame encode commands.
    ///The slot index in each entry **must** match one of the slot indexes
    ///provided in the `pReferenceSlots` of the parent
    ///[`VideoEncodeInfoKHR`] structure.
    p_reference_list_1_entries: *mut VideoEncodeH264DpbSlotInfoEXT<'lt>,
    ///[`p_mem_mgmt_ctrl_operations`] is a pointer to a
    ///[`StdVideoEncodeH264RefMemMgmtCtrlOperations`] structure specifying
    ///reference lists modifications and decoded picture marking operations.
    p_mem_mgmt_ctrl_operations: *mut StdVideoEncodeH264RefMemMgmtCtrlOperations,
}
///[VkVideoEncodeH264EmitPictureParametersEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264EmitPictureParametersEXT.html) - Structure specifies H.264 encode SPS NALU insertion parameters
///# C Specifications
///The [`VideoEncodeH264EmitPictureParametersEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_video_encode_h264
///typedef struct VkVideoEncodeH264EmitPictureParametersEXT {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint8_t            spsId;
///    VkBool32           emitSpsEnable;
///    uint32_t           ppsIdEntryCount;
///    const uint8_t*     ppsIdEntries;
///} VkVideoEncodeH264EmitPictureParametersEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`sps_id`] is the H.264 SPS ID for the H.264 SPS to insert in the bitstream. The SPS ID
///   **must** match the SPS provided in `spsStd` of
///   [`VideoEncodeH264SessionParametersCreateInfoEXT`]. This is retrieved from the
///   [`VideoSessionParametersKHR`] object provided in [`VideoBeginCodingInfoKHR`].
/// - [`emit_sps_enable`] enables the emitting of the SPS structure with id of [`sps_id`].
/// - [`pps_id_entry_count`] is the number of entries in the [`pps_id_entries`]. If this parameter
///   is `0` then no pps entries are going to be emitted in the bitstream.
/// - [`pps_id_entries`] is a pointer to an array of H.264 PPS IDs for the H.264 PPS to insert in
///   the bitstream. The PPS IDs **must** match one of the IDs of the PPS(s) provided in `pPpsStd`
///   of [`VideoEncodeH264SessionParametersCreateInfoEXT`] to identify the PPS parameter set to
///   insert in the bitstream. This is retrieved from the [`VideoSessionParametersKHR`] object
///   provided in [`VideoBeginCodingInfoKHR`].
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_EMIT_PICTURE_PARAMETERS_EXT`
/// - [`pps_id_entries`]**must** be a valid pointer to an array of [`pps_id_entry_count`]`uint8_t`
///   values
/// - [`pps_id_entry_count`]**must** be greater than `0`
///# Related
/// - [`VK_EXT_video_encode_h264`]
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VideoEncodeH264EmitPictureParametersEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`sps_id`] is the H.264 SPS ID for the H.264 SPS to insert in the
    ///bitstream.
    ///The SPS ID **must** match the SPS provided in `spsStd` of
    ///[`VideoEncodeH264SessionParametersCreateInfoEXT`].
    ///This is retrieved from the [`VideoSessionParametersKHR`] object
    ///provided in [`VideoBeginCodingInfoKHR`].
    sps_id: u8,
    ///[`emit_sps_enable`] enables the emitting of the SPS structure with id of
    ///[`sps_id`].
    emit_sps_enable: Bool32,
    ///[`pps_id_entry_count`] is the number of entries in the
    ///[`pps_id_entries`].
    ///If this parameter is `0` then no pps entries are going to be emitted in
    ///the bitstream.
    pps_id_entry_count: u32,
    ///[`pps_id_entries`] is a pointer to an array of H.264 PPS IDs for the
    ///H.264 PPS to insert in the bitstream.
    ///The PPS IDs **must** match one of the IDs of the PPS(s) provided in
    ///`pPpsStd` of [`VideoEncodeH264SessionParametersCreateInfoEXT`]
    ///to identify the PPS parameter set to insert in the bitstream.
    ///This is retrieved from the [`VideoSessionParametersKHR`] object
    ///provided in [`VideoBeginCodingInfoKHR`].
    pps_id_entries: *mut u8,
}
///[VkVideoEncodeH264ProfileEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264ProfileEXT.html) - Structure specifying H.264 encode profile
///# C Specifications
///The [`VideoEncodeH264ProfileEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_video_encode_h264
///typedef struct VkVideoEncodeH264ProfileEXT {
///    VkStructureType           sType;
///    const void*               pNext;
///    StdVideoH264ProfileIdc    stdProfileIdc;
///} VkVideoEncodeH264ProfileEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`std_profile_idc`] is a [`StdVideoH264ProfileIdc`] value specifying the H.264 codec profile
///   IDC.
///# Description
///An H.264 encode profile is specified by including a
///[`VideoEncodeH264ProfileEXT`] structure in the [`p_next`] chain of the
///[`VideoProfileKHR`] structure when
///[`VideoProfileKHR::video_codec_operation`] is
///`VK_VIDEO_CODEC_OPERATION_ENCODE_H264_BIT_EXT`.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_PROFILE_EXT`
///# Related
/// - [`VK_EXT_video_encode_h264`]
/// - [`StructureType`]
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
pub struct VideoEncodeH264ProfileEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`std_profile_idc`] is a [`StdVideoH264ProfileIdc`] value specifying
    ///the H.264 codec profile IDC.
    std_profile_idc: StdVideoH264ProfileIdc,
}
///[VkVideoEncodeH264NaluSliceEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264NaluSliceEXT.html) - Structure specifies H.264 encode slice NALU parameters
///# C Specifications
///The [`VideoEncodeH264NaluSliceEXT`] structure representing a slice is
///defined as:
///```c
///// Provided by VK_EXT_video_encode_h264
///typedef struct VkVideoEncodeH264NaluSliceEXT {
///    VkStructureType                              sType;
///    const void*                                  pNext;
///    uint32_t                                     mbCount;
///    const VkVideoEncodeH264ReferenceListsEXT*    pReferenceFinalLists;
///    const StdVideoEncodeH264SliceHeader*         pSliceHeaderStd;
///} VkVideoEncodeH264NaluSliceEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`mb_count`] is the number of macroblocks in this slice.
/// - [`p_reference_final_lists`] is `NULL` or a pointer to a [`VideoEncodeH264ReferenceListsEXT`]
///   structure specifying the reference lists to be used for the current slice. If
///   [`p_reference_final_lists`] is not `NULL`, these reference lists override the reference lists
///   provided in [`VideoEncodeH264VclFrameInfoEXT`]::[`p_reference_final_lists`].
/// - [`p_slice_header_std`] is a pointer to a [`StdVideoEncodeH264SliceHeader`] structure
///   specifying the slice header for the current slice.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_NALU_SLICE_EXT`
/// - [`p_next`]**must** be `NULL`
/// - If [`p_reference_final_lists`] is not `NULL`, [`p_reference_final_lists`]**must** be a valid
///   pointer to a valid [`VideoEncodeH264ReferenceListsEXT`] structure
/// - [`p_slice_header_std`]**must** be a valid pointer to a valid [`StdVideoEncodeH264SliceHeader`]
///   value
///# Related
/// - [`VK_EXT_video_encode_h264`]
/// - [`StructureType`]
/// - [`VideoEncodeH264ReferenceListsEXT`]
/// - [`VideoEncodeH264VclFrameInfoEXT`]
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
pub struct VideoEncodeH264NaluSliceEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`mb_count`] is the number of macroblocks in this slice.
    mb_count: u32,
    ///[`p_reference_final_lists`] is `NULL` or a pointer to a
    ///[`VideoEncodeH264ReferenceListsEXT`] structure specifying the
    ///reference lists to be used for the current slice.
    ///If [`p_reference_final_lists`] is not `NULL`, these reference lists
    ///override the reference lists provided in
    ///[`VideoEncodeH264VclFrameInfoEXT`]::[`p_reference_final_lists`].
    p_reference_final_lists: *mut VideoEncodeH264ReferenceListsEXT<'lt>,
    ///[`p_slice_header_std`] is a pointer to a
    ///[`StdVideoEncodeH264SliceHeader`] structure specifying the slice header
    ///for the current slice.
    p_slice_header_std: *mut StdVideoEncodeH264SliceHeader,
}
///[VkVideoEncodeH264RateControlInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264RateControlInfoEXT.html) - Structure describing H.264 stream rate control parameters
///# C Specifications
///The [`VideoEncodeH264RateControlInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_video_encode_h264
///typedef struct VkVideoEncodeH264RateControlInfoEXT {
///    VkStructureType                                     sType;
///    const void*                                         pNext;
///    uint32_t                                            gopFrameCount;
///    uint32_t                                            idrPeriod;
///    uint32_t                                            consecutiveBFrameCount;
///    VkVideoEncodeH264RateControlStructureFlagBitsEXT    rateControlStructure;
///    uint8_t                                             temporalLayerCount;
///} VkVideoEncodeH264RateControlInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`gop_frame_count`] is the number of frames contained within the group of pictures (GOP),
///   starting from an intra frame and until the next intra frame. If it is set to 0, the
///   implementation chooses a suitable value. If it is set to `UINT32_MAX`, the GOP length is
///   treated as infinite.
/// - [`idr_period`] is the interval, in terms of number of frames, between two IDR frames. If it is
///   set to 0, the implementation chooses a suitable value. If it is set to `UINT32_MAX`, the IDR
///   period is treated as infinite.
/// - [`consecutive_b_frame_count`] is the number of consecutive B-frames between I- and/or P-frames
///   within the GOP.
/// - [`rate_control_structure`] is a [`VideoEncodeH264RateControlStructureFlagBitsEXT`] value
///   specifying the expected encode stream reference structure, to aid in rate control
///   calculations.
/// - [`temporal_layer_count`] specifies the number of temporal layers enabled in the stream.
///# Description
///In order to provide H.264-specific stream rate control parameters, add a
///[`VideoEncodeH264RateControlInfoEXT`] structure to the [`p_next`] chain
///of the [`VideoEncodeRateControlInfoKHR`] structure in the [`p_next`]
///chain of the [`VideoCodingControlInfoKHR`] structure passed to the
///[`CmdControlVideoCodingKHR`] command.The parameters from this structure act as a guidance for
/// implementations to
///apply various rate control heuristics.It is possible to infer the picture type to be used when
/// encoding a frame,
///on the basis of the values provided for [`consecutive_b_frame_count`],
///[`idr_period`], and [`gop_frame_count`], but this inferred picture type
///will not be used by implementations to override the picture type provided in
///[`CmdEncodeVideoKHR`].
///Additionally, it is not required for the video session to be reset if the
///inferred picture type does not match the actual picture type.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_RATE_CONTROL_INFO_EXT`
/// - [`rate_control_structure`]**must** be a valid
///   [`VideoEncodeH264RateControlStructureFlagBitsEXT`] value
///# Related
/// - [`VK_EXT_video_encode_h264`]
/// - [`StructureType`]
/// - [`VideoEncodeH264RateControlStructureFlagBitsEXT`]
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
pub struct VideoEncodeH264RateControlInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`gop_frame_count`] is the number of frames contained within the group
    ///of pictures (GOP), starting from an intra frame and until the next intra
    ///frame.
    ///If it is set to 0, the implementation chooses a suitable value.
    ///If it is set to `UINT32_MAX`, the GOP length is treated as infinite.
    gop_frame_count: u32,
    ///[`idr_period`] is the interval, in terms of number of frames, between
    ///two IDR frames.
    ///If it is set to 0, the implementation chooses a suitable value.
    ///If it is set to `UINT32_MAX`, the IDR period is treated as infinite.
    idr_period: u32,
    ///[`consecutive_b_frame_count`] is the number of consecutive B-frames
    ///between I- and/or P-frames within the GOP.
    consecutive_b_frame_count: u32,
    ///[`rate_control_structure`] is a
    ///[`VideoEncodeH264RateControlStructureFlagBitsEXT`] value specifying
    ///the expected encode stream reference structure, to aid in rate control
    ///calculations.
    rate_control_structure: VideoEncodeH264RateControlStructureFlagBitsEXT,
    ///[`temporal_layer_count`] specifies the number of temporal layers enabled
    ///in the stream.
    temporal_layer_count: u8,
}
///[VkVideoEncodeH264QpEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264QpEXT.html) - Structure describing H.264 QP values per picture type
///# C Specifications
///The [`VideoEncodeH264QpEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_video_encode_h264
///typedef struct VkVideoEncodeH264QpEXT {
///    int32_t    qpI;
///    int32_t    qpP;
///    int32_t    qpB;
///} VkVideoEncodeH264QpEXT;
///```
///# Members
/// - [`qp_i`] is the QP to be used for I-frames.
/// - [`qp_p`] is the QP to be used for P-frames.
/// - [`qp_b`] is the QP to be used for B-frames.
///# Related
/// - [`VK_EXT_video_encode_h264`]
/// - [`VideoEncodeH264RateControlLayerInfoEXT`]
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
pub struct VideoEncodeH264QpEXT {
    ///[`qp_i`] is the QP to be used for I-frames.
    qp_i: i32,
    ///[`qp_p`] is the QP to be used for P-frames.
    qp_p: i32,
    ///[`qp_b`] is the QP to be used for B-frames.
    qp_b: i32,
}
///[VkVideoEncodeH264FrameSizeEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264FrameSizeEXT.html) - Structure describing frame size values per H.264 picture type
///# C Specifications
///The [`VideoEncodeH264FrameSizeEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_video_encode_h264
///typedef struct VkVideoEncodeH264FrameSizeEXT {
///    uint32_t    frameISize;
///    uint32_t    framePSize;
///    uint32_t    frameBSize;
///} VkVideoEncodeH264FrameSizeEXT;
///```
///# Members
/// - [`frame_i_size`] is the size in bytes to be used for I-frames.
/// - [`frame_p_size`] is the size in bytes to be used for P-frames.
/// - [`frame_b_size`] is the size in bytes to be used for B-frames.
///# Related
/// - [`VK_EXT_video_encode_h264`]
/// - [`VideoEncodeH264RateControlLayerInfoEXT`]
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
pub struct VideoEncodeH264FrameSizeEXT {
    ///[`frame_i_size`] is the size in bytes to be used for I-frames.
    frame_i_size: u32,
    ///[`frame_p_size`] is the size in bytes to be used for P-frames.
    frame_p_size: u32,
    ///[`frame_b_size`] is the size in bytes to be used for B-frames.
    frame_b_size: u32,
}
///[VkVideoEncodeH264RateControlLayerInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264RateControlLayerInfoEXT.html) - Structure describing H.264 per-layer rate control parameters
///# C Specifications
///The [`VideoEncodeH264RateControlLayerInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_video_encode_h264
///typedef struct VkVideoEncodeH264RateControlLayerInfoEXT {
///    VkStructureType                  sType;
///    const void*                      pNext;
///    uint8_t                          temporalLayerId;
///    VkBool32                         useInitialRcQp;
///    VkVideoEncodeH264QpEXT           initialRcQp;
///    VkBool32                         useMinQp;
///    VkVideoEncodeH264QpEXT           minQp;
///    VkBool32                         useMaxQp;
///    VkVideoEncodeH264QpEXT           maxQp;
///    VkBool32                         useMaxFrameSize;
///    VkVideoEncodeH264FrameSizeEXT    maxFrameSize;
///} VkVideoEncodeH264RateControlLayerInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`temporal_layer_id`] specifies the H.264 temporal layer ID of the video coding layer that
///   settings provided in this structure and its parent [`VideoEncodeRateControlLayerInfoKHR`]
///   structure apply to.
/// - [`use_initial_rc_qp`] indicates whether the values within [`initial_rc_qp`] should be used by
///   the implementation.
/// - [`initial_rc_qp`] provides the QP values for each picture type, to be used in rate control
///   calculations at the start of video encode operations on a newly-created video session, or
///   immediately after a session reset. These values are ignored when
///   [`VideoEncodeRateControlInfoKHR::rate_control_mode`] is
///   `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`.
/// - [`use_min_qp`] indicates whether the values within [`min_qp`] should be used by the
///   implementation. When it is set to [`FALSE`], the implementation ignores the values in
///   [`min_qp`] and chooses suitable values.
/// - [`min_qp`] provides the lower bound on the QP values for each picture type, to be used in rate
///   control calculations.
/// - [`use_max_qp`] indicates whether the values within [`max_qp`] should be used by the
///   implementation. When it is set to [`FALSE`], the implementation ignores the values in
///   [`max_qp`] and chooses suitable values.
/// - [`max_qp`] provides the upper bound on the QP values for each picture type, to be used in rate
///   control calculations.
/// - [`use_max_frame_size`] indicates whether the values within [`max_frame_size`] should be used
///   by the implementation.
/// - [`max_frame_size`] provides the upper bound on the encoded frame size for each picture type.
///   The implementation does not guarantee the encoded frame sizes will be within the specified
///   limits, however these limits **may** be used as a guide in rate control calculations. If
///   enabled and not set properly, the [`max_qp`] limit may prevent the implementation from
///   respecting the [`max_frame_size`] limit.
///# Description
///H.264-specific per-layer rate control parameters **must** be specified by
///adding a [`VideoEncodeH264RateControlLayerInfoEXT`] structure to the
///[`p_next`] chain of each [`VideoEncodeRateControlLayerInfoKHR`]
///structure in a call to [`CmdControlVideoCodingKHR`] command, when the
///command buffer context has an active video encode H.264 session.Valid Usage
/// - When [`VideoEncodeRateControlInfoKHR::rate_control_mode`] is
///   `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`, both [`use_min_qp`] and [`use_max_qp`] must
///   be set to [`TRUE`].
/// - When [`VideoEncodeRateControlInfoKHR::rate_control_mode`] is
///   `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`, the values provided in `minQP` must be
///   identical to those provided in [`max_qp`].
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_RATE_CONTROL_LAYER_INFO_EXT`
/// - [`initial_rc_qp`]**must** be a valid [`VideoEncodeH264QpEXT`] structure
/// - [`min_qp`]**must** be a valid [`VideoEncodeH264QpEXT`] structure
/// - [`max_qp`]**must** be a valid [`VideoEncodeH264QpEXT`] structure
/// - [`max_frame_size`]**must** be a valid [`VideoEncodeH264FrameSizeEXT`] structure
///# Related
/// - [`VK_EXT_video_encode_h264`]
/// - [`Bool32`]
/// - [`StructureType`]
/// - [`VideoEncodeH264FrameSizeEXT`]
/// - [`VideoEncodeH264QpEXT`]
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
pub struct VideoEncodeH264RateControlLayerInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`temporal_layer_id`] specifies the H.264 temporal layer ID of the video
    ///coding layer that settings provided in this structure and its parent
    ///[`VideoEncodeRateControlLayerInfoKHR`] structure apply to.
    temporal_layer_id: u8,
    ///[`use_initial_rc_qp`] indicates whether the values within
    ///[`initial_rc_qp`] should be used by the implementation.
    use_initial_rc_qp: Bool32,
    ///[`initial_rc_qp`] provides the QP values for each picture type, to be
    ///used in rate control calculations at the start of video encode
    ///operations on a newly-created video session, or immediately after a
    ///session reset.
    ///These values are ignored when
    ///[`VideoEncodeRateControlInfoKHR`]::`rateControlMode` is
    ///`VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`.
    initial_rc_qp: VideoEncodeH264QpEXT,
    ///[`use_min_qp`] indicates whether the values within [`min_qp`] should be
    ///used by the implementation.
    ///When it is set to [`FALSE`], the implementation ignores the values
    ///in [`min_qp`] and chooses suitable values.
    use_min_qp: Bool32,
    ///[`min_qp`] provides the lower bound on the QP values for each picture
    ///type, to be used in rate control calculations.
    min_qp: VideoEncodeH264QpEXT,
    ///[`use_max_qp`] indicates whether the values within [`max_qp`] should be
    ///used by the implementation.
    ///When it is set to [`FALSE`], the implementation ignores the values
    ///in [`max_qp`] and chooses suitable values.
    use_max_qp: Bool32,
    ///[`max_qp`] provides the upper bound on the QP values for each picture
    ///type, to be used in rate control calculations.
    max_qp: VideoEncodeH264QpEXT,
    ///[`use_max_frame_size`] indicates whether the values within
    ///[`max_frame_size`] should be used by the implementation.
    use_max_frame_size: Bool32,
    ///[`max_frame_size`] provides the upper bound on the encoded frame size
    ///for each picture type.
    ///The implementation does not guarantee the encoded frame sizes will be
    ///within the specified limits, however these limits **may** be used as a
    ///guide in rate control calculations.
    ///If enabled and not set properly, the [`max_qp`] limit may prevent the
    ///implementation from respecting the [`max_frame_size`] limit.
    max_frame_size: VideoEncodeH264FrameSizeEXT,
}
