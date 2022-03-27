use crate::{
    native::{
        StdVideoEncodeH265PictureInfo, StdVideoEncodeH265ReferenceInfo, StdVideoEncodeH265ReferenceModifications,
        StdVideoEncodeH265SliceSegmentHeader, StdVideoH265PictureParameterSet, StdVideoH265ProfileIdc,
        StdVideoH265SequenceParameterSet, StdVideoH265VideoParameterSet,
    },
    vulkan1_0::{BaseInStructure, Bool32, ExtensionProperties, StructureType},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_VIDEO_ENCODE_H265_SPEC_VERSION")]
pub const EXT_VIDEO_ENCODE_H265_SPEC_VERSION: u32 = 5;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_VIDEO_ENCODE_H265_EXTENSION_NAME")]
pub const EXT_VIDEO_ENCODE_H265_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_video_encode_h265");
///[VkVideoEncodeH265CapabilitiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265CapabilitiesEXT.html) - Structure specifying H.265 encode capabilities
///# C Specifications
///When calling [`GetPhysicalDeviceVideoCapabilitiesKHR`] with
///`pVideoProfile->videoCodecOperation` specified as
///`VK_VIDEO_CODEC_OPERATION_ENCODE_H265_BIT_EXT`, the
///[`VideoEncodeH265CapabilitiesEXT`] structure **must** be included in the
///[`p_next`] chain of the [`VideoCapabilitiesKHR`] structure to retrieve
///more capabilities specific to H.265 video encoding.The [`VideoEncodeH265CapabilitiesEXT`]
/// structure is defined as:
///```c
///// Provided by VK_EXT_video_encode_h265
///typedef struct VkVideoEncodeH265CapabilitiesEXT {
///    VkStructureType                                sType;
///    const void*                                    pNext;
///    VkVideoEncodeH265CapabilityFlagsEXT            flags;
///    VkVideoEncodeH265InputModeFlagsEXT             inputModeFlags;
///    VkVideoEncodeH265OutputModeFlagsEXT            outputModeFlags;
///    VkVideoEncodeH265CtbSizeFlagsEXT               ctbSizes;
///    VkVideoEncodeH265TransformBlockSizeFlagsEXT    transformBlockSizes;
///    uint8_t                                        maxPPictureL0ReferenceCount;
///    uint8_t                                        maxBPictureL0ReferenceCount;
///    uint8_t                                        maxL1ReferenceCount;
///    uint8_t                                        maxSubLayersCount;
///    uint8_t                                        minLog2MinLumaCodingBlockSizeMinus3;
///    uint8_t                                        maxLog2MinLumaCodingBlockSizeMinus3;
///    uint8_t                                        minLog2MinLumaTransformBlockSizeMinus2;
///    uint8_t                                        maxLog2MinLumaTransformBlockSizeMinus2;
///    uint8_t                                        minMaxTransformHierarchyDepthInter;
///    uint8_t                                        maxMaxTransformHierarchyDepthInter;
///    uint8_t                                        minMaxTransformHierarchyDepthIntra;
///    uint8_t                                        maxMaxTransformHierarchyDepthIntra;
///    uint8_t                                        maxDiffCuQpDeltaDepth;
///    uint8_t                                        minMaxNumMergeCand;
///    uint8_t                                        maxMaxNumMergeCand;
///    VkExtensionProperties                          stdExtensionVersion;
///} VkVideoEncodeH265CapabilitiesEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is a bitmask of [`VideoEncodeH265CapabilityFlagBitsEXT`] describing supported
///   encoding tools.
/// - [`input_mode_flags`] is a bitmask of [`VideoEncodeH265InputModeFlagBitsEXT`] describing the
///   command buffer input granularities/modes supported by the implementation.
/// - [`output_mode_flags`] is a bitmask of [`VideoEncodeH265OutputModeFlagBitsEXT`] describing the
///   output (bitstream size reporting) granularities/modes supported by the implementation.
/// - [`ctb_sizes`] is a bitmask of [`VideoEncodeH265CtbSizeFlagBitsEXT`] describing the supported
///   CTB sizes.
/// - [`transform_block_sizes`] is a bitmask of [`VideoEncodeH265TransformBlockSizeFlagBitsEXT`]
///   describing the supported transform block sizes.
/// - [`max_p_picture_l_0_reference_count`] reports the maximum number of reference pictures the
///   implementation supports in the reference list L0 for P pictures.
/// - [`max_b_picture_l_0_reference_count`] reports the maximum number of reference pictures the
///   implementation supports in the reference list L0 for B pictures. The reported value is `0` if
///   encoding of B pictures is not supported.
/// - [`max_l_1_reference_count`] reports the maximum number of reference pictures the
///   implementation supports in the reference list L1 if encoding of B pictures is supported. The
///   reported value is `0` if encoding of B pictures is not supported.
/// - [`max_sub_layers_count`] reports the maximum number of sublayers.
/// - [`min_log_2_min_luma_coding_block_size_minus_3`] reports the minimum value that may be set for
///   log2_min_luma_coding_block_size_minus3 in StdVideoH265SequenceParameterSet.
/// - [`max_log_2_min_luma_coding_block_size_minus_3`] reports the maximum value that may be set for
///   log2_min_luma_coding_block_size_minus3 in StdVideoH265SequenceParameterSet.
/// - [`min_log_2_min_luma_transform_block_size_minus_2`] reports the minimum value that may be set
///   for log2_min_luma_transform_block_size_minus2 in StdVideoH265SequenceParameterSet.
/// - [`max_log_2_min_luma_transform_block_size_minus_2`] reports the maximum value that may be set
///   for log2_min_luma_transform_block_size_minus2 in StdVideoH265SequenceParameterSet.
/// - [`min_max_transform_hierarchy_depth_inter`] reports the minimum value that may be set for
///   max_transform_hierarchy_depth_inter in StdVideoH265SequenceParameterSet.
/// - [`max_max_transform_hierarchy_depth_inter`] reports the maximum value that may be set for
///   max_transform_hierarchy_depth_inter in StdVideoH265SequenceParameterSet.
/// - [`min_max_transform_hierarchy_depth_intra`] reports the minimum value that may be set for
///   max_transform_hierarchy_depth_intra in StdVideoH265SequenceParameterSet.
/// - [`max_max_transform_hierarchy_depth_intra`] reports the maximum value that may be set for
///   max_transform_hierarchy_depth_intra in StdVideoH265SequenceParameterSet.
/// - [`max_diff_cu_qp_delta_depth`] reports the maximum value that may be set for
///   diff_cu_qp_delta_depth in StdVideoH265PictureParameterSet.
/// - [`min_max_num_merge_cand`] reports the minimum value that may be set for MaxNumMergeCand in
///   StdVideoEncodeH265SliceHeader.
/// - [`max_max_num_merge_cand`] reports the maximum value that may be set for MaxNumMergeCand in
///   StdVideoEncodeH265SliceHeader.
/// - [`std_extension_version`] is a [`ExtensionProperties`] structure in which the H.265 extension
///   name and version supported by the implementation are returned.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_CAPABILITIES_EXT`
/// - [`input_mode_flags`]**must** be a valid combination of [`VideoEncodeH265InputModeFlagBitsEXT`]
///   values
/// - [`input_mode_flags`]**must** not be `0`
/// - [`output_mode_flags`]**must** be a valid combination of
///   [`VideoEncodeH265OutputModeFlagBitsEXT`] values
/// - [`output_mode_flags`]**must** not be `0`
/// - [`ctb_sizes`]**must** be a valid combination of [`VideoEncodeH265CtbSizeFlagBitsEXT`] values
/// - [`ctb_sizes`]**must** not be `0`
/// - [`transform_block_sizes`]**must** be a valid combination of
///   [`VideoEncodeH265TransformBlockSizeFlagBitsEXT`] values
/// - [`transform_block_sizes`]**must** not be `0`
/// - [`std_extension_version`]**must** be a valid [`ExtensionProperties`] structure
///# Related
/// - [`VK_EXT_video_encode_h265`]
/// - [`ExtensionProperties`]
/// - [`StructureType`]
/// - [`VideoEncodeH265CapabilityFlagsEXT`]
/// - [`VideoEncodeH265CtbSizeFlagsEXT`]
/// - [`VideoEncodeH265InputModeFlagsEXT`]
/// - [`VideoEncodeH265OutputModeFlagsEXT`]
/// - [`VideoEncodeH265TransformBlockSizeFlagsEXT`]
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
pub struct VideoEncodeH265CapabilitiesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`VideoEncodeH265CapabilityFlagBitsEXT`]
    ///describing supported encoding tools.
    flags: VideoEncodeH265CapabilityFlagsEXT,
    ///[`input_mode_flags`] is a bitmask of
    ///[`VideoEncodeH265InputModeFlagBitsEXT`] describing the command
    ///buffer input granularities/modes supported by the implementation.
    input_mode_flags: VideoEncodeH265InputModeFlagsEXT,
    ///[`output_mode_flags`] is a bitmask of
    ///[`VideoEncodeH265OutputModeFlagBitsEXT`] describing the output
    ///(bitstream size reporting) granularities/modes supported by the
    ///implementation.
    output_mode_flags: VideoEncodeH265OutputModeFlagsEXT,
    ///[`ctb_sizes`] is a bitmask of [`VideoEncodeH265CtbSizeFlagBitsEXT`]
    ///describing the supported CTB sizes.
    ctb_sizes: VideoEncodeH265CtbSizeFlagsEXT,
    ///[`transform_block_sizes`] is a bitmask of
    ///[`VideoEncodeH265TransformBlockSizeFlagBitsEXT`] describing the
    ///supported transform block sizes.
    transform_block_sizes: VideoEncodeH265TransformBlockSizeFlagsEXT,
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
    ///[`max_sub_layers_count`] reports the maximum number of sublayers.
    max_sub_layers_count: u8,
    ///[`min_log_2_min_luma_coding_block_size_minus_3`] reports the minimum value that
    ///may be set for log2_min_luma_coding_block_size_minus3 in
    ///StdVideoH265SequenceParameterSet.
    min_log_2_min_luma_coding_block_size_minus_3: u8,
    ///[`max_log_2_min_luma_coding_block_size_minus_3`] reports the maximum value that
    ///may be set for log2_min_luma_coding_block_size_minus3 in
    ///StdVideoH265SequenceParameterSet.
    max_log_2_min_luma_coding_block_size_minus_3: u8,
    ///[`min_log_2_min_luma_transform_block_size_minus_2`] reports the minimum value
    ///that may be set for log2_min_luma_transform_block_size_minus2 in
    ///StdVideoH265SequenceParameterSet.
    min_log_2_min_luma_transform_block_size_minus_2: u8,
    ///[`max_log_2_min_luma_transform_block_size_minus_2`] reports the maximum value
    ///that may be set for log2_min_luma_transform_block_size_minus2 in
    ///StdVideoH265SequenceParameterSet.
    max_log_2_min_luma_transform_block_size_minus_2: u8,
    ///[`min_max_transform_hierarchy_depth_inter`] reports the minimum value that
    ///may be set for max_transform_hierarchy_depth_inter in
    ///StdVideoH265SequenceParameterSet.
    min_max_transform_hierarchy_depth_inter: u8,
    ///[`max_max_transform_hierarchy_depth_inter`] reports the maximum value that
    ///may be set for max_transform_hierarchy_depth_inter in
    ///StdVideoH265SequenceParameterSet.
    max_max_transform_hierarchy_depth_inter: u8,
    ///[`min_max_transform_hierarchy_depth_intra`] reports the minimum value that
    ///may be set for max_transform_hierarchy_depth_intra in
    ///StdVideoH265SequenceParameterSet.
    min_max_transform_hierarchy_depth_intra: u8,
    ///[`max_max_transform_hierarchy_depth_intra`] reports the maximum value that
    ///may be set for max_transform_hierarchy_depth_intra in
    ///StdVideoH265SequenceParameterSet.
    max_max_transform_hierarchy_depth_intra: u8,
    ///[`max_diff_cu_qp_delta_depth`] reports the maximum value that may be set
    ///for diff_cu_qp_delta_depth in StdVideoH265PictureParameterSet.
    max_diff_cu_qp_delta_depth: u8,
    ///[`min_max_num_merge_cand`] reports the minimum value that may be set for
    ///MaxNumMergeCand in StdVideoEncodeH265SliceHeader.
    min_max_num_merge_cand: u8,
    ///[`max_max_num_merge_cand`] reports the maximum value that may be set for
    ///MaxNumMergeCand in StdVideoEncodeH265SliceHeader.
    max_max_num_merge_cand: u8,
    ///[`std_extension_version`] is a [`ExtensionProperties`] structure in
    ///which the H.265 extension name and version supported by the
    ///implementation are returned.
    std_extension_version: ExtensionProperties,
}
///[VkVideoEncodeH265SessionCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265SessionCreateInfoEXT.html) - Structure specifies H.265 encoder creation parameters
///# C Specifications
///When creating a Video Session object with
///[`VideoSessionCreateInfoKHR`]::`pVideoProfile->videoCodecOperation`
///specified as `VK_VIDEO_CODEC_OPERATION_ENCODE_H265_BIT_EXT`, add a
///[`VideoEncodeH265SessionCreateInfoEXT`] structure to the [`p_next`]
///chain of the [`VideoSessionCreateInfoKHR`] structure passed to
///[`CreateVideoSessionKHR`] in order to specify the H.265-specific video
///encoder session creation parameters.The [`VideoEncodeH265SessionCreateInfoEXT`] structure is
/// defined as:
///```c
///// Provided by VK_EXT_video_encode_h265
///typedef struct VkVideoEncodeH265SessionCreateInfoEXT {
///    VkStructureType                    sType;
///    const void*                        pNext;
///    VkVideoEncodeH265CreateFlagsEXT    flags;
///    const VkExtensionProperties*       pStdExtensionVersion;
///} VkVideoEncodeH265SessionCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`p_std_extension_version`] is a pointer to a [`ExtensionProperties`] structure specifying the
///   H.265 codec extension version.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_SESSION_CREATE_INFO_EXT`
/// - [`flags`]**must** be `0`
/// - [`p_std_extension_version`]**must** be a valid pointer to a valid [`ExtensionProperties`]
///   structure
///# Related
/// - [`VK_EXT_video_encode_h265`]
/// - [`ExtensionProperties`]
/// - [`StructureType`]
/// - [`VideoEncodeH265CreateFlagsEXT`]
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
pub struct VideoEncodeH265SessionCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: VideoEncodeH265CreateFlagsEXT,
    ///[`p_std_extension_version`] is a pointer to a [`ExtensionProperties`]
    ///structure specifying the H.265 codec extension version.
    p_std_extension_version: *mut ExtensionProperties,
}
///[VkVideoEncodeH265SessionParametersAddInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265SessionParametersAddInfoEXT.html) - Structure specifies H.265 encoder parameter set info
///# C Specifications
///The [`VideoEncodeH265SessionParametersAddInfoEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_video_encode_h265
///typedef struct VkVideoEncodeH265SessionParametersAddInfoEXT {
///    VkStructureType                            sType;
///    const void*                                pNext;
///    uint32_t                                   vpsStdCount;
///    const StdVideoH265VideoParameterSet*       pVpsStd;
///    uint32_t                                   spsStdCount;
///    const StdVideoH265SequenceParameterSet*    pSpsStd;
///    uint32_t                                   ppsStdCount;
///    const StdVideoH265PictureParameterSet*     pPpsStd;
///} VkVideoEncodeH265SessionParametersAddInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`vps_std_count`] is the number of VPS elements in [`p_vps_std`].
/// - [`p_vps_std`] is a pointer to an array of [`vps_std_count`][`StdVideoH265VideoParameterSet`]
///   structures representing H.265 video parameter sets.
/// - [`sps_std_count`] is the number of SPS elements in [`p_sps_std`].
/// - [`p_sps_std`] is a pointer to an array of
///   [`sps_std_count`][`StdVideoH265SequenceParameterSet`] structures representing H.265 sequence
///   parameter sets.
/// - [`pps_std_count`] is the number of PPS elements in [`p_pps_std`].
/// - [`p_pps_std`] is a pointer to an array of [`pps_std_count`][`StdVideoH265PictureParameterSet`]
///   structures representing H.265 picture parameter sets.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_SESSION_PARAMETERS_ADD_INFO_EXT`
/// - If [`p_vps_std`] is not `NULL`, [`p_vps_std`]**must** be a valid pointer to an array of
///   [`vps_std_count`][`StdVideoH265VideoParameterSet`] values
/// - If [`p_sps_std`] is not `NULL`, [`p_sps_std`]**must** be a valid pointer to an array of
///   [`sps_std_count`][`StdVideoH265SequenceParameterSet`] values
/// - If [`p_pps_std`] is not `NULL`, [`p_pps_std`]**must** be a valid pointer to an array of
///   [`pps_std_count`][`StdVideoH265PictureParameterSet`] values
/// - [`vps_std_count`]**must** be greater than `0`
/// - [`sps_std_count`]**must** be greater than `0`
/// - [`pps_std_count`]**must** be greater than `0`
///Valid Usage
/// - The values of [`vps_std_count`], [`sps_std_count`] and [`pps_std_count`]**must** be less than
///   or equal to the values of
///   [`VideoEncodeH265SessionParametersCreateInfoEXT::max_vps_std_count`],
///   [`VideoEncodeH265SessionParametersCreateInfoEXT`]:`maxSpsStdCount`, and
///   [`VideoEncodeH265SessionParametersCreateInfoEXT`]:`maxPpsStdCount`, respectively
/// - Each [`StdVideoH265VideoParameterSet`] entry in [`p_vps_std`]**must** have a unique H.265 VPS
///   ID
/// - Each [`StdVideoH265SequenceParameterSet`] entry in [`p_sps_std`]**must** have a unique H.265
///   VPS-SPS ID pair
/// - Each [`StdVideoH265PictureParameterSet`] entry in [`p_pps_std`]**must** have a unique H.265
///   VPS-SPS-PPS ID tuple
/// - Each entry to be added **must** have a unique, to the rest of the parameter array entries and
///   the existing parameters in the Video Session Parameters Object that is being updated,
///   VPS-SPS-PPS IDs
/// - Parameter entries that already exist in Video Session Parameters object with a particular
///   VPS-SPS-PPS IDs **must** not be replaced nor updated
/// - When creating a new object using a Video Session Parameters as a template, the array’s
///   parameters with the same VPS-SPS-PPS IDs as the ones from the template take precedence
/// - VPS/SPS/PPS parameters **must** comply with the limits specified in
///   [`VideoSessionCreateInfoKHR`] during Video Session creation
///# Related
/// - [`VK_EXT_video_encode_h265`]
/// - [`StructureType`]
/// - [`VideoEncodeH265SessionParametersCreateInfoEXT`]
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
pub struct VideoEncodeH265SessionParametersAddInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`vps_std_count`] is the number of VPS elements in [`p_vps_std`].
    vps_std_count: u32,
    ///[`p_vps_std`] is a pointer to an array of [`vps_std_count`][`StdVideoH265VideoParameterSet`]
    /// structures representing H.265 video parameter sets.
    p_vps_std: *mut StdVideoH265VideoParameterSet,
    ///[`sps_std_count`] is the number of SPS elements in [`p_sps_std`].
    sps_std_count: u32,
    ///[`p_sps_std`] is a pointer to an array of
    /// [`sps_std_count`][`StdVideoH265SequenceParameterSet`] structures representing H.265
    /// sequence parameter sets.
    p_sps_std: *mut StdVideoH265SequenceParameterSet,
    ///[`pps_std_count`] is the number of PPS elements in [`p_pps_std`].
    pps_std_count: u32,
    ///[`p_pps_std`] is a pointer to an array of
    /// [`pps_std_count`][`StdVideoH265PictureParameterSet`] structures representing H.265
    /// picture parameter sets.
    p_pps_std: *mut StdVideoH265PictureParameterSet,
}
///[VkVideoEncodeH265SessionParametersCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265SessionParametersCreateInfoEXT.html) - Structure specifies H.265 encoder parameter set info
///# C Specifications
///The [`VideoEncodeH265SessionParametersCreateInfoEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_video_encode_h265
///typedef struct VkVideoEncodeH265SessionParametersCreateInfoEXT {
///    VkStructureType                                        sType;
///    const void*                                            pNext;
///    uint32_t                                               maxVpsStdCount;
///    uint32_t                                               maxSpsStdCount;
///    uint32_t                                               maxPpsStdCount;
///    const VkVideoEncodeH265SessionParametersAddInfoEXT*    pParametersAddInfo;
///} VkVideoEncodeH265SessionParametersCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`max_vps_std_count`] is the maximum number of entries of type
///   [`StdVideoH265VideoParameterSet`] within [`VideoSessionParametersKHR`].
/// - [`max_sps_std_count`] is the maximum number of entries of type
///   [`StdVideoH265SequenceParameterSet`] within [`VideoSessionParametersKHR`].
/// - [`max_pps_std_count`] is the maximum number of entries of type
///   [`StdVideoH265PictureParameterSet`] within [`VideoSessionParametersKHR`].
/// - [`p_parameters_add_info`] is `NULL` or a pointer to a
///   [`VideoEncodeH265SessionParametersAddInfoEXT`] structure specifying the video session
///   parameters to add upon creation of this object.
///# Description
///When a [`VideoSessionParametersKHR`] object contains
///[`max_vps_std_count`][`StdVideoH265VideoParameterSet`] entries, no
///additional [`StdVideoH265VideoParameterSet`] entries can be added to it,
///and `VK_ERROR_TOO_MANY_OBJECTS` will be returned if an attempt is made
///to add these entries.
///When a [`VideoSessionParametersKHR`] object contains
///[`max_sps_std_count`][`StdVideoH265SequenceParameterSet`] entries, no
///additional [`StdVideoH265SequenceParameterSet`] entries can be added to it,
///and `VK_ERROR_TOO_MANY_OBJECTS` will be returned if an attempt is made
///to add these entries.
///When a [`VideoSessionParametersKHR`] object contains
///[`max_pps_std_count`][`StdVideoH265PictureParameterSet`] entries, no
///additional [`StdVideoH265PictureParameterSet`] entries can be added to it,
///and `VK_ERROR_TOO_MANY_OBJECTS` will be returned if an attempt is made
///to add these entries.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_SESSION_PARAMETERS_CREATE_INFO_EXT`
/// - If [`p_parameters_add_info`] is not `NULL`, [`p_parameters_add_info`]**must** be a valid
///   pointer to a valid [`VideoEncodeH265SessionParametersAddInfoEXT`] structure
///# Related
/// - [`VK_EXT_video_encode_h265`]
/// - [`StructureType`]
/// - [`VideoEncodeH265SessionParametersAddInfoEXT`]
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
pub struct VideoEncodeH265SessionParametersCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`max_vps_std_count`] is the maximum number of entries of type
    ///[`StdVideoH265VideoParameterSet`] within
    ///[`VideoSessionParametersKHR`].
    max_vps_std_count: u32,
    ///[`max_sps_std_count`] is the maximum number of entries of type
    ///[`StdVideoH265SequenceParameterSet`] within
    ///[`VideoSessionParametersKHR`].
    max_sps_std_count: u32,
    ///[`max_pps_std_count`] is the maximum number of entries of type
    ///[`StdVideoH265PictureParameterSet`] within
    ///[`VideoSessionParametersKHR`].
    max_pps_std_count: u32,
    ///[`p_parameters_add_info`] is `NULL` or a pointer to a
    ///[`VideoEncodeH265SessionParametersAddInfoEXT`] structure specifying
    ///the video session parameters to add upon creation of this object.
    p_parameters_add_info: *mut VideoEncodeH265SessionParametersAddInfoEXT<'lt>,
}
///[VkVideoEncodeH265VclFrameInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265VclFrameInfoEXT.html) - Structure specifies H.265 encode frame parameters
///# C Specifications
///The [`VideoEncodeH265VclFrameInfoEXT`] structure representing a frame
///encode operation is defined as:
///```c
///// Provided by VK_EXT_video_encode_h265
///typedef struct VkVideoEncodeH265VclFrameInfoEXT {
///    VkStructureType                                sType;
///    const void*                                    pNext;
///    const VkVideoEncodeH265ReferenceListsEXT*      pReferenceFinalLists;
///    uint32_t                                       naluSliceSegmentEntryCount;
///    const VkVideoEncodeH265NaluSliceSegmentEXT*    pNaluSliceSegmentEntries;
///    const StdVideoEncodeH265PictureInfo*           pCurrentPictureInfo;
///} VkVideoEncodeH265VclFrameInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`p_reference_final_lists`] is `NULL` or a pointer to a [`VideoEncodeH265ReferenceListsEXT`]
///   structure specifying the reference lists to be used for the current picture.
/// - [`nalu_slice_segment_entry_count`] is the number of slice segment NALUs in the frame.
/// - [`p_nalu_slice_segment_entries`] is a pointer to an array of
///   [`VideoEncodeH265NaluSliceSegmentEXT`] structures specifying the division of the current
///   picture into slice segments and the properties of these slice segments.
/// - [`p_current_picture_info`] is a pointer to a [`StdVideoEncodeH265PictureInfo`] structure
///   specifying the syntax and other codec-specific information from the H.265 specification,
///   associated with this picture.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_VCL_FRAME_INFO_EXT`
/// - If [`p_reference_final_lists`] is not `NULL`, [`p_reference_final_lists`]**must** be a valid
///   pointer to a valid [`VideoEncodeH265ReferenceListsEXT`] structure
/// - [`p_nalu_slice_segment_entries`]**must** be a valid pointer to an array of
///   [`nalu_slice_segment_entry_count`] valid [`VideoEncodeH265NaluSliceSegmentEXT`] structures
/// - [`p_current_picture_info`]**must** be a valid pointer to a valid
///   [`StdVideoEncodeH265PictureInfo`] value
/// - [`nalu_slice_segment_entry_count`]**must** be greater than `0`
///# Related
/// - [`VK_EXT_video_encode_h265`]
/// - [`StructureType`]
/// - [`VideoEncodeH265NaluSliceSegmentEXT`]
/// - [`VideoEncodeH265ReferenceListsEXT`]
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
pub struct VideoEncodeH265VclFrameInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`p_reference_final_lists`] is `NULL` or a pointer to a
    ///[`VideoEncodeH265ReferenceListsEXT`] structure specifying the
    ///reference lists to be used for the current picture.
    p_reference_final_lists: *mut VideoEncodeH265ReferenceListsEXT<'lt>,
    ///[`nalu_slice_segment_entry_count`] is the number of slice segment NALUs in
    ///the frame.
    nalu_slice_segment_entry_count: u32,
    ///[`p_nalu_slice_segment_entries`] is a pointer to an array of
    ///[`VideoEncodeH265NaluSliceSegmentEXT`] structures specifying the
    ///division of the current picture into slice segments and the properties
    ///of these slice segments.
    p_nalu_slice_segment_entries: *mut VideoEncodeH265NaluSliceSegmentEXT<'lt>,
    ///[`p_current_picture_info`] is a pointer to a
    ///[`StdVideoEncodeH265PictureInfo`] structure specifying the syntax and
    ///other codec-specific information from the H.265 specification,
    ///associated with this picture.
    p_current_picture_info: *mut StdVideoEncodeH265PictureInfo,
}
///[VkVideoEncodeH265EmitPictureParametersEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265EmitPictureParametersEXT.html) - Structure specifies H.265 encode VPS NALU insertion parameters
///# C Specifications
///The [`VideoEncodeH265EmitPictureParametersEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_video_encode_h265
///typedef struct VkVideoEncodeH265EmitPictureParametersEXT {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint8_t            vpsId;
///    uint8_t            spsId;
///    VkBool32           emitVpsEnable;
///    VkBool32           emitSpsEnable;
///    uint32_t           ppsIdEntryCount;
///    const uint8_t*     ppsIdEntries;
///} VkVideoEncodeH265EmitPictureParametersEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`vps_id`] is the H.265 VPS ID for the H.265 VPS to insert in the bitstream. The VPS ID
///   **must** match the VPS provided in `vpsStd` of
///   [`VideoEncodeH265SessionParametersCreateInfoEXT`]. This is retrieved from the
///   [`VideoSessionParametersKHR`] object provided in [`VideoBeginCodingInfoKHR`].
/// - [`sps_id`] is the H.265 SPS ID for the H.265 SPS to insert in the bitstream. The SPS ID
///   **must** match one of the IDs of the SPS(s) provided in `pSpsStd` of
///   [`VideoEncodeH265SessionParametersCreateInfoEXT`] to identify the SPS parameter set to insert
///   in the bitstream. This is retrieved from the [`VideoSessionParametersKHR`] object provided in
///   [`VideoBeginCodingInfoKHR`].
/// - [`emit_vps_enable`] enables the emitting of the VPS structure with id of [`vps_id`].
/// - [`emit_sps_enable`] enables the emitting of the SPS structure with id of [`sps_id`].
/// - [`pps_id_entry_count`] is the number of entries in the [`pps_id_entries`]. If this parameter
///   is `0` then no pps entries are going to be emitted in the bitstream.
/// - [`pps_id_entries`] is the H.265 PPS IDs for the H.265 PPS to insert in the bitstream. The PPS
///   IDs **must** match one of the IDs of the PPS(s) provided in `pPpsStd` of
///   [`VideoEncodeH265SessionParametersCreateInfoEXT`] to identify the PPS parameter set to insert
///   in the bitstream. This is retrieved from the [`VideoSessionParametersKHR`] object provided in
///   [`VideoBeginCodingInfoKHR`].
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_EMIT_PICTURE_PARAMETERS_EXT`
/// - If [`pps_id_entry_count`] is not `0`, [`pps_id_entries`]**must** be a valid pointer to an
///   array of [`pps_id_entry_count`]`uint8_t` values
///# Related
/// - [`VK_EXT_video_encode_h265`]
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
pub struct VideoEncodeH265EmitPictureParametersEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`vps_id`] is the H.265 VPS ID for the H.265 VPS to insert in the
    ///bitstream.
    ///The VPS ID **must** match the VPS provided in `vpsStd` of
    ///[`VideoEncodeH265SessionParametersCreateInfoEXT`].
    ///This is retrieved from the [`VideoSessionParametersKHR`] object
    ///provided in [`VideoBeginCodingInfoKHR`].
    vps_id: u8,
    ///[`sps_id`] is the H.265 SPS ID for the H.265 SPS to insert in the
    ///bitstream.
    ///The SPS ID **must** match one of the IDs of the SPS(s) provided in
    ///`pSpsStd` of [`VideoEncodeH265SessionParametersCreateInfoEXT`]
    ///to identify the SPS parameter set to insert in the bitstream.
    ///This is retrieved from the [`VideoSessionParametersKHR`] object
    ///provided in [`VideoBeginCodingInfoKHR`].
    sps_id: u8,
    ///[`emit_vps_enable`] enables the emitting of the VPS structure with id of
    ///[`vps_id`].
    emit_vps_enable: Bool32,
    ///[`emit_sps_enable`] enables the emitting of the SPS structure with id of
    ///[`sps_id`].
    emit_sps_enable: Bool32,
    ///[`pps_id_entry_count`] is the number of entries in the
    ///[`pps_id_entries`].
    ///If this parameter is `0` then no pps entries are going to be emitted in
    ///the bitstream.
    pps_id_entry_count: u32,
    ///[`pps_id_entries`] is the H.265 PPS IDs for the H.265 PPS to insert in
    ///the bitstream.
    ///The PPS IDs **must** match one of the IDs of the PPS(s) provided in
    ///`pPpsStd` of [`VideoEncodeH265SessionParametersCreateInfoEXT`]
    ///to identify the PPS parameter set to insert in the bitstream.
    ///This is retrieved from the [`VideoSessionParametersKHR`] object
    ///provided in [`VideoBeginCodingInfoKHR`].
    pps_id_entries: *mut u8,
}
///[VkVideoEncodeH265NaluSliceSegmentEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265NaluSliceSegmentEXT.html) - Structure specifies H.265 encode slice segment NALU parameters
///# C Specifications
///The [`VideoEncodeH265NaluSliceSegmentEXT`] structure representing a
///slice segment is defined as:
///```c
///// Provided by VK_EXT_video_encode_h265
///typedef struct VkVideoEncodeH265NaluSliceSegmentEXT {
///    VkStructureType                                sType;
///    const void*                                    pNext;
///    uint32_t                                       ctbCount;
///    const VkVideoEncodeH265ReferenceListsEXT*      pReferenceFinalLists;
///    const StdVideoEncodeH265SliceSegmentHeader*    pSliceSegmentHeaderStd;
///} VkVideoEncodeH265NaluSliceSegmentEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`ctb_count`] is the number of CTBs in this slice segment.
/// - [`p_reference_final_lists`] is `NULL` or a pointer to a [`VideoEncodeH265ReferenceListsEXT`]
///   structure specifying the reference lists to be used for the current slice segment. If
///   [`p_reference_final_lists`] is not `NULL`, these reference lists override the reference lists
///   provided in [`VideoEncodeH265VclFrameInfoEXT`]::[`p_reference_final_lists`].
/// - [`p_slice_segment_header_std`] is a pointer to a [`StdVideoEncodeH265SliceSegmentHeader`]
///   structure specifying the slice segment header for the current slice segment.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_NALU_SLICE_SEGMENT_EXT`
/// - [`p_next`]**must** be `NULL`
/// - If [`p_reference_final_lists`] is not `NULL`, [`p_reference_final_lists`]**must** be a valid
///   pointer to a valid [`VideoEncodeH265ReferenceListsEXT`] structure
/// - [`p_slice_segment_header_std`]**must** be a valid pointer to a valid
///   [`StdVideoEncodeH265SliceSegmentHeader`] value
///# Related
/// - [`VK_EXT_video_encode_h265`]
/// - [`StructureType`]
/// - [`VideoEncodeH265ReferenceListsEXT`]
/// - [`VideoEncodeH265VclFrameInfoEXT`]
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
pub struct VideoEncodeH265NaluSliceSegmentEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`ctb_count`] is the number of CTBs in this slice segment.
    ctb_count: u32,
    ///[`p_reference_final_lists`] is `NULL` or a pointer to a
    ///[`VideoEncodeH265ReferenceListsEXT`] structure specifying the
    ///reference lists to be used for the current slice segment.
    ///If [`p_reference_final_lists`] is not `NULL`, these reference lists
    ///override the reference lists provided in
    ///[`VideoEncodeH265VclFrameInfoEXT`]::[`p_reference_final_lists`].
    p_reference_final_lists: *mut VideoEncodeH265ReferenceListsEXT<'lt>,
    ///[`p_slice_segment_header_std`] is a pointer to a
    ///[`StdVideoEncodeH265SliceSegmentHeader`] structure specifying the slice
    ///segment header for the current slice segment.
    p_slice_segment_header_std: *mut StdVideoEncodeH265SliceSegmentHeader,
}
///[VkVideoEncodeH265RateControlInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265RateControlInfoEXT.html) - Structure describing H.265 stream rate control parameters
///# C Specifications
///The [`VideoEncodeH265RateControlInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_video_encode_h265
///typedef struct VkVideoEncodeH265RateControlInfoEXT {
///    VkStructureType                                     sType;
///    const void*                                         pNext;
///    uint32_t                                            gopFrameCount;
///    uint32_t                                            idrPeriod;
///    uint32_t                                            consecutiveBFrameCount;
///    VkVideoEncodeH265RateControlStructureFlagBitsEXT    rateControlStructure;
///    uint8_t                                             subLayerCount;
///} VkVideoEncodeH265RateControlInfoEXT;
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
/// - [`rate_control_structure`] is a [`VideoEncodeH265RateControlStructureFlagBitsEXT`] value
///   specifying the expected encode stream reference structure, to aid in rate control
///   calculations.
/// - [`sub_layer_count`] specifies the number of sub layers enabled in the stream.
///# Description
///In order to provide H.265-specific stream rate control parameters, add a
///[`VideoEncodeH265RateControlInfoEXT`] structure to the [`p_next`] chain
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
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_RATE_CONTROL_INFO_EXT`
/// - [`rate_control_structure`]**must** be a valid
///   [`VideoEncodeH265RateControlStructureFlagBitsEXT`] value
///# Related
/// - [`VK_EXT_video_encode_h265`]
/// - [`StructureType`]
/// - [`VideoEncodeH265RateControlStructureFlagBitsEXT`]
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
pub struct VideoEncodeH265RateControlInfoEXT<'lt> {
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
    ///[`VideoEncodeH265RateControlStructureFlagBitsEXT`] value specifying
    ///the expected encode stream reference structure, to aid in rate control
    ///calculations.
    rate_control_structure: VideoEncodeH265RateControlStructureFlagBitsEXT,
    ///[`sub_layer_count`] specifies the number of sub layers enabled in the
    ///stream.
    sub_layer_count: u8,
}
///[VkVideoEncodeH265QpEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265QpEXT.html) - Structure describing H.265 QP values per picture type
///# C Specifications
///The [`VideoEncodeH265QpEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_video_encode_h265
///typedef struct VkVideoEncodeH265QpEXT {
///    int32_t    qpI;
///    int32_t    qpP;
///    int32_t    qpB;
///} VkVideoEncodeH265QpEXT;
///```
///# Members
/// - [`qp_i`] is the QP to be used for I-frames.
/// - [`qp_p`] is the QP to be used for P-frames.
/// - [`qp_b`] is the QP to be used for B-frames.
///# Related
/// - [`VK_EXT_video_encode_h265`]
/// - [`VideoEncodeH265RateControlLayerInfoEXT`]
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
pub struct VideoEncodeH265QpEXT {
    ///[`qp_i`] is the QP to be used for I-frames.
    qp_i: i32,
    ///[`qp_p`] is the QP to be used for P-frames.
    qp_p: i32,
    ///[`qp_b`] is the QP to be used for B-frames.
    qp_b: i32,
}
///[VkVideoEncodeH265FrameSizeEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265FrameSizeEXT.html) - Structure describing frame size values per H.265 picture type
///# C Specifications
///The [`VideoEncodeH265FrameSizeEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_video_encode_h265
///typedef struct VkVideoEncodeH265FrameSizeEXT {
///    uint32_t    frameISize;
///    uint32_t    framePSize;
///    uint32_t    frameBSize;
///} VkVideoEncodeH265FrameSizeEXT;
///```
///# Members
/// - [`frame_i_size`] is the size in bytes to be used for I-frames.
/// - [`frame_p_size`] is the size in bytes to be used for P-frames.
/// - [`frame_b_size`] is the size in bytes to be used for B-frames.
///# Related
/// - [`VK_EXT_video_encode_h265`]
/// - [`VideoEncodeH265RateControlLayerInfoEXT`]
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
pub struct VideoEncodeH265FrameSizeEXT {
    ///[`frame_i_size`] is the size in bytes to be used for I-frames.
    frame_i_size: u32,
    ///[`frame_p_size`] is the size in bytes to be used for P-frames.
    frame_p_size: u32,
    ///[`frame_b_size`] is the size in bytes to be used for B-frames.
    frame_b_size: u32,
}
///[VkVideoEncodeH265RateControlLayerInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265RateControlLayerInfoEXT.html) - Structure describing H.265 per-layer rate control parameters
///# C Specifications
///The [`VideoEncodeH265RateControlLayerInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_video_encode_h265
///typedef struct VkVideoEncodeH265RateControlLayerInfoEXT {
///    VkStructureType                  sType;
///    const void*                      pNext;
///    uint8_t                          temporalId;
///    VkBool32                         useInitialRcQp;
///    VkVideoEncodeH265QpEXT           initialRcQp;
///    VkBool32                         useMinQp;
///    VkVideoEncodeH265QpEXT           minQp;
///    VkBool32                         useMaxQp;
///    VkVideoEncodeH265QpEXT           maxQp;
///    VkBool32                         useMaxFrameSize;
///    VkVideoEncodeH265FrameSizeEXT    maxFrameSize;
///} VkVideoEncodeH265RateControlLayerInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`temporal_id`] specifies the H.265 temporal ID of the video coding layer that settings
///   provided in this structure and its parent [`VideoEncodeRateControlLayerInfoKHR`] structure
///   apply to.
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
///H.265-specific per-layer rate control parameters **must** be specified by
///adding a [`VideoEncodeH265RateControlLayerInfoEXT`] structure to the
///[`p_next`] chain of each [`VideoEncodeRateControlLayerInfoKHR`]
///structure in a call to [`CmdControlVideoCodingKHR`] command, when the
///command buffer context has an active video encode H.265 session.Valid Usage
/// - When [`VideoEncodeRateControlInfoKHR::rate_control_mode`] is
///   `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`, both [`use_min_qp`] and [`use_max_qp`] must
///   be set to [`TRUE`].
/// - When [`VideoEncodeRateControlInfoKHR::rate_control_mode`] is
///   `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`, the values provided in `minQP` must be
///   identical to those provided in [`max_qp`].
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_RATE_CONTROL_LAYER_INFO_EXT`
/// - [`initial_rc_qp`]**must** be a valid [`VideoEncodeH265QpEXT`] structure
/// - [`min_qp`]**must** be a valid [`VideoEncodeH265QpEXT`] structure
/// - [`max_qp`]**must** be a valid [`VideoEncodeH265QpEXT`] structure
/// - [`max_frame_size`]**must** be a valid [`VideoEncodeH265FrameSizeEXT`] structure
///# Related
/// - [`VK_EXT_video_encode_h265`]
/// - [`Bool32`]
/// - [`StructureType`]
/// - [`VideoEncodeH265FrameSizeEXT`]
/// - [`VideoEncodeH265QpEXT`]
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
pub struct VideoEncodeH265RateControlLayerInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`temporal_id`] specifies the H.265 temporal ID of the video coding
    ///layer that settings provided in this structure and its parent
    ///[`VideoEncodeRateControlLayerInfoKHR`] structure apply to.
    temporal_id: u8,
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
    initial_rc_qp: VideoEncodeH265QpEXT,
    ///[`use_min_qp`] indicates whether the values within [`min_qp`] should be
    ///used by the implementation.
    ///When it is set to [`FALSE`], the implementation ignores the values
    ///in [`min_qp`] and chooses suitable values.
    use_min_qp: Bool32,
    ///[`min_qp`] provides the lower bound on the QP values for each picture
    ///type, to be used in rate control calculations.
    min_qp: VideoEncodeH265QpEXT,
    ///[`use_max_qp`] indicates whether the values within [`max_qp`] should be
    ///used by the implementation.
    ///When it is set to [`FALSE`], the implementation ignores the values
    ///in [`max_qp`] and chooses suitable values.
    use_max_qp: Bool32,
    ///[`max_qp`] provides the upper bound on the QP values for each picture
    ///type, to be used in rate control calculations.
    max_qp: VideoEncodeH265QpEXT,
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
    max_frame_size: VideoEncodeH265FrameSizeEXT,
}
///[VkVideoEncodeH265ProfileEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265ProfileEXT.html) - Structure specifying H.265 encode profile
///# C Specifications
///The [`VideoEncodeH265ProfileEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_video_encode_h265
///typedef struct VkVideoEncodeH265ProfileEXT {
///    VkStructureType           sType;
///    const void*               pNext;
///    StdVideoH265ProfileIdc    stdProfileIdc;
///} VkVideoEncodeH265ProfileEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`std_profile_idc`] is a [`StdVideoH265ProfileIdc`] value specifying the H.265 codec profile
///   IDC.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_PROFILE_EXT`
///# Related
/// - [`VK_EXT_video_encode_h265`]
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
pub struct VideoEncodeH265ProfileEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`std_profile_idc`] is a [`StdVideoH265ProfileIdc`] value specifying
    ///the H.265 codec profile IDC.
    std_profile_idc: StdVideoH265ProfileIdc,
}
///[VkVideoEncodeH265DpbSlotInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265DpbSlotInfoEXT.html) - Structure specifies H.265 encode decoded pic info
///# C Specifications
///The [`VideoEncodeH265DpbSlotInfoEXT`] structure, representing a
///reconstructed picture that is being used as a reference picture, is defined
///as:
///```c
///// Provided by VK_EXT_video_encode_h265
///typedef struct VkVideoEncodeH265DpbSlotInfoEXT {
///    VkStructureType                           sType;
///    const void*                               pNext;
///    int8_t                                    slotIndex;
///    const StdVideoEncodeH265ReferenceInfo*    pStdReferenceInfo;
///} VkVideoEncodeH265DpbSlotInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`slot_index`] is the [DPB Slot](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#dpb-slot)
///   index for this picture.
/// - [`p_std_reference_info`] is a pointer to a [`StdVideoEncodeH265ReferenceInfo`] structure
///   specifying the syntax and other codec-specific information from the H.265 specification,
///   associated with this reference picture.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_DPB_SLOT_INFO_EXT`
/// - [`p_next`]**must** be `NULL`
/// - [`p_std_reference_info`]**must** be a valid pointer to a valid
///   [`StdVideoEncodeH265ReferenceInfo`] value
///# Related
/// - [`VK_EXT_video_encode_h265`]
/// - [`StructureType`]
/// - [`VideoEncodeH265ReferenceListsEXT`]
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
pub struct VideoEncodeH265DpbSlotInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`slot_index`] is the [DPB Slot](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#dpb-slot) index for this picture.
    slot_index: i8,
    ///[`p_std_reference_info`] is a pointer to a
    ///[`StdVideoEncodeH265ReferenceInfo`] structure specifying the syntax and
    ///other codec-specific information from the H.265 specification,
    ///associated with this reference picture.
    p_std_reference_info: *mut StdVideoEncodeH265ReferenceInfo,
}
///[VkVideoEncodeH265ReferenceListsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265ReferenceListsEXT.html) - Structure specifies H.265 reference frame lists
///# C Specifications
///The [`VideoEncodeH265ReferenceListsEXT`] structure representing
///reference lists is defined as:
///```c
///// Provided by VK_EXT_video_encode_h265
///typedef struct VkVideoEncodeH265ReferenceListsEXT {
///    VkStructureType                                    sType;
///    const void*                                        pNext;
///    uint8_t                                            referenceList0EntryCount;
///    const VkVideoEncodeH265DpbSlotInfoEXT*             pReferenceList0Entries;
///    uint8_t                                            referenceList1EntryCount;
///    const VkVideoEncodeH265DpbSlotInfoEXT*             pReferenceList1Entries;
///    const StdVideoEncodeH265ReferenceModifications*    pReferenceModifications;
///} VkVideoEncodeH265ReferenceListsEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`reference_list_0_entry_count`] is the number of reference pictures in reference list L0 and
///   is identical to [`StdVideoEncodeH265SliceSegmentHeader`]`::num_ref_idx_l0_active_minus1` + 1.
/// - [`p_reference_list_0_entries`] is a pointer to an array of
///   [`reference_list_0_entry_count`][`VideoEncodeH265DpbSlotInfoEXT`] structures specifying the
///   reference list L0 entries for the current picture.
/// - [`reference_list_1_entry_count`] is the number of reference pictures in reference list L1 and
///   is identical to [`StdVideoEncodeH265SliceSegmentHeader`]`::num_ref_idx_l1_active_minus1` + 1.
/// - [`p_reference_list_1_entries`] is a pointer to an array of
///   [`reference_list_1_entry_count`][`VideoEncodeH265DpbSlotInfoEXT`] structures specifying the
///   reference list L1 entries for the current picture.
/// - [`p_reference_modifications`] is a pointer to a [`StdVideoEncodeH265ReferenceModifications`]
///   structure specifying reference list modifications.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_REFERENCE_LISTS_EXT`
/// - [`p_next`]**must** be `NULL`
/// - If [`reference_list_0_entry_count`] is not `0`, [`p_reference_list_0_entries`]**must** be a
///   valid pointer to an array of [`reference_list_0_entry_count`] valid
///   [`VideoEncodeH265DpbSlotInfoEXT`] structures
/// - If [`reference_list_1_entry_count`] is not `0`, [`p_reference_list_1_entries`]**must** be a
///   valid pointer to an array of [`reference_list_1_entry_count`] valid
///   [`VideoEncodeH265DpbSlotInfoEXT`] structures
/// - [`p_reference_modifications`]**must** be a valid pointer to a valid
///   [`StdVideoEncodeH265ReferenceModifications`] value
///# Related
/// - [`VK_EXT_video_encode_h265`]
/// - [`StructureType`]
/// - [`VideoEncodeH265DpbSlotInfoEXT`]
/// - [`VideoEncodeH265NaluSliceSegmentEXT`]
/// - [`VideoEncodeH265VclFrameInfoEXT`]
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
pub struct VideoEncodeH265ReferenceListsEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`reference_list_0_entry_count`] is the number of reference pictures in
    ///reference list L0 and is identical to
    ///[`StdVideoEncodeH265SliceSegmentHeader`]::`num_ref_idx_l0_active_minus1`
    ///+ 1.
    reference_list_0_entry_count: u8,
    ///[`p_reference_list_0_entries`] is a pointer to an array of
    ///[`reference_list_0_entry_count`][`VideoEncodeH265DpbSlotInfoEXT`]
    ///structures specifying the reference list L0 entries for the current
    ///picture.
    p_reference_list_0_entries: *mut VideoEncodeH265DpbSlotInfoEXT<'lt>,
    ///[`reference_list_1_entry_count`] is the number of reference pictures in
    ///reference list L1 and is identical to
    ///[`StdVideoEncodeH265SliceSegmentHeader`]::`num_ref_idx_l1_active_minus1`
    ///+ 1.
    reference_list_1_entry_count: u8,
    ///[`p_reference_list_1_entries`] is a pointer to an array of
    ///[`reference_list_1_entry_count`][`VideoEncodeH265DpbSlotInfoEXT`]
    ///structures specifying the reference list L1 entries for the current
    ///picture.
    p_reference_list_1_entries: *mut VideoEncodeH265DpbSlotInfoEXT<'lt>,
    ///[`p_reference_modifications`] is a pointer to a
    ///[`StdVideoEncodeH265ReferenceModifications`] structure specifying
    ///reference list modifications.
    p_reference_modifications: *mut StdVideoEncodeH265ReferenceModifications,
}
