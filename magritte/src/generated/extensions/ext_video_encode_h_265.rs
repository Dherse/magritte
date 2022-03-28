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
///[`VideoEncodeH265CapabilitiesEXT`] structure  **must**  be included in the
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
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_CAPABILITIES_EXT`
/// - [`input_mode_flags`] **must**  be a valid combination of
///   [`VideoEncodeH265InputModeFlagBitsEXT`] values
/// - [`input_mode_flags`] **must**  not be `0`
/// - [`output_mode_flags`] **must**  be a valid combination of
///   [`VideoEncodeH265OutputModeFlagBitsEXT`] values
/// - [`output_mode_flags`] **must**  not be `0`
/// - [`ctb_sizes`] **must**  be a valid combination of [`VideoEncodeH265CtbSizeFlagBitsEXT`] values
/// - [`ctb_sizes`] **must**  not be `0`
/// - [`transform_block_sizes`] **must**  be a valid combination of
///   [`VideoEncodeH265TransformBlockSizeFlagBitsEXT`] values
/// - [`transform_block_sizes`] **must**  not be `0`
/// - [`std_extension_version`] **must**  be a valid [`ExtensionProperties`] structure
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeH265CapabilitiesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
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
impl<'lt> Default for VideoEncodeH265CapabilitiesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
            input_mode_flags: Default::default(),
            output_mode_flags: Default::default(),
            ctb_sizes: Default::default(),
            transform_block_sizes: Default::default(),
            max_p_picture_l_0_reference_count: 0,
            max_b_picture_l_0_reference_count: 0,
            max_l_1_reference_count: 0,
            max_sub_layers_count: 0,
            min_log_2_min_luma_coding_block_size_minus_3: 0,
            max_log_2_min_luma_coding_block_size_minus_3: 0,
            min_log_2_min_luma_transform_block_size_minus_2: 0,
            max_log_2_min_luma_transform_block_size_minus_2: 0,
            min_max_transform_hierarchy_depth_inter: 0,
            max_max_transform_hierarchy_depth_inter: 0,
            min_max_transform_hierarchy_depth_intra: 0,
            max_max_transform_hierarchy_depth_intra: 0,
            max_diff_cu_qp_delta_depth: 0,
            min_max_num_merge_cand: 0,
            max_max_num_merge_cand: 0,
            std_extension_version: Default::default(),
        }
    }
}
impl<'lt> VideoEncodeH265CapabilitiesEXT<'lt> {
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
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> VideoEncodeH265CapabilityFlagsEXT {
        self.flags
    }
    ///Gets the value of [`Self::input_mode_flags`]
    pub fn input_mode_flags(&self) -> VideoEncodeH265InputModeFlagsEXT {
        self.input_mode_flags
    }
    ///Gets the value of [`Self::output_mode_flags`]
    pub fn output_mode_flags(&self) -> VideoEncodeH265OutputModeFlagsEXT {
        self.output_mode_flags
    }
    ///Gets the value of [`Self::ctb_sizes`]
    pub fn ctb_sizes(&self) -> VideoEncodeH265CtbSizeFlagsEXT {
        self.ctb_sizes
    }
    ///Gets the value of [`Self::transform_block_sizes`]
    pub fn transform_block_sizes(&self) -> VideoEncodeH265TransformBlockSizeFlagsEXT {
        self.transform_block_sizes
    }
    ///Gets the value of [`Self::max_p_picture_l_0_reference_count`]
    pub fn max_p_picture_l_0_reference_count(&self) -> u8 {
        self.max_p_picture_l_0_reference_count
    }
    ///Gets the value of [`Self::max_b_picture_l_0_reference_count`]
    pub fn max_b_picture_l_0_reference_count(&self) -> u8 {
        self.max_b_picture_l_0_reference_count
    }
    ///Gets the value of [`Self::max_l_1_reference_count`]
    pub fn max_l_1_reference_count(&self) -> u8 {
        self.max_l_1_reference_count
    }
    ///Gets the value of [`Self::max_sub_layers_count`]
    pub fn max_sub_layers_count(&self) -> u8 {
        self.max_sub_layers_count
    }
    ///Gets the value of [`Self::min_log_2_min_luma_coding_block_size_minus_3`]
    pub fn min_log_2_min_luma_coding_block_size_minus_3(&self) -> u8 {
        self.min_log_2_min_luma_coding_block_size_minus_3
    }
    ///Gets the value of [`Self::max_log_2_min_luma_coding_block_size_minus_3`]
    pub fn max_log_2_min_luma_coding_block_size_minus_3(&self) -> u8 {
        self.max_log_2_min_luma_coding_block_size_minus_3
    }
    ///Gets the value of [`Self::min_log_2_min_luma_transform_block_size_minus_2`]
    pub fn min_log_2_min_luma_transform_block_size_minus_2(&self) -> u8 {
        self.min_log_2_min_luma_transform_block_size_minus_2
    }
    ///Gets the value of [`Self::max_log_2_min_luma_transform_block_size_minus_2`]
    pub fn max_log_2_min_luma_transform_block_size_minus_2(&self) -> u8 {
        self.max_log_2_min_luma_transform_block_size_minus_2
    }
    ///Gets the value of [`Self::min_max_transform_hierarchy_depth_inter`]
    pub fn min_max_transform_hierarchy_depth_inter(&self) -> u8 {
        self.min_max_transform_hierarchy_depth_inter
    }
    ///Gets the value of [`Self::max_max_transform_hierarchy_depth_inter`]
    pub fn max_max_transform_hierarchy_depth_inter(&self) -> u8 {
        self.max_max_transform_hierarchy_depth_inter
    }
    ///Gets the value of [`Self::min_max_transform_hierarchy_depth_intra`]
    pub fn min_max_transform_hierarchy_depth_intra(&self) -> u8 {
        self.min_max_transform_hierarchy_depth_intra
    }
    ///Gets the value of [`Self::max_max_transform_hierarchy_depth_intra`]
    pub fn max_max_transform_hierarchy_depth_intra(&self) -> u8 {
        self.max_max_transform_hierarchy_depth_intra
    }
    ///Gets the value of [`Self::max_diff_cu_qp_delta_depth`]
    pub fn max_diff_cu_qp_delta_depth(&self) -> u8 {
        self.max_diff_cu_qp_delta_depth
    }
    ///Gets the value of [`Self::min_max_num_merge_cand`]
    pub fn min_max_num_merge_cand(&self) -> u8 {
        self.min_max_num_merge_cand
    }
    ///Gets the value of [`Self::max_max_num_merge_cand`]
    pub fn max_max_num_merge_cand(&self) -> u8 {
        self.max_max_num_merge_cand
    }
    ///Gets the value of [`Self::std_extension_version`]
    pub fn std_extension_version(&self) -> ExtensionProperties {
        self.std_extension_version
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut VideoEncodeH265CapabilityFlagsEXT {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::input_mode_flags`]
    pub fn input_mode_flags_mut(&mut self) -> &mut VideoEncodeH265InputModeFlagsEXT {
        &mut self.input_mode_flags
    }
    ///Gets a mutable reference to the value of [`Self::output_mode_flags`]
    pub fn output_mode_flags_mut(&mut self) -> &mut VideoEncodeH265OutputModeFlagsEXT {
        &mut self.output_mode_flags
    }
    ///Gets a mutable reference to the value of [`Self::ctb_sizes`]
    pub fn ctb_sizes_mut(&mut self) -> &mut VideoEncodeH265CtbSizeFlagsEXT {
        &mut self.ctb_sizes
    }
    ///Gets a mutable reference to the value of [`Self::transform_block_sizes`]
    pub fn transform_block_sizes_mut(&mut self) -> &mut VideoEncodeH265TransformBlockSizeFlagsEXT {
        &mut self.transform_block_sizes
    }
    ///Gets a mutable reference to the value of [`Self::max_p_picture_l_0_reference_count`]
    pub fn max_p_picture_l_0_reference_count_mut(&mut self) -> &mut u8 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_b_picture_l_0_reference_count`]
    pub fn max_b_picture_l_0_reference_count_mut(&mut self) -> &mut u8 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_l_1_reference_count`]
    pub fn max_l_1_reference_count_mut(&mut self) -> &mut u8 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_sub_layers_count`]
    pub fn max_sub_layers_count_mut(&mut self) -> &mut u8 {
        &mut getter
    }
    ///Gets a mutable reference to the value of
    /// [`Self::min_log_2_min_luma_coding_block_size_minus_3`]
    pub fn min_log_2_min_luma_coding_block_size_minus_3_mut(&mut self) -> &mut u8 {
        &mut getter
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_log_2_min_luma_coding_block_size_minus_3`]
    pub fn max_log_2_min_luma_coding_block_size_minus_3_mut(&mut self) -> &mut u8 {
        &mut getter
    }
    ///Gets a mutable reference to the value of
    /// [`Self::min_log_2_min_luma_transform_block_size_minus_2`]
    pub fn min_log_2_min_luma_transform_block_size_minus_2_mut(&mut self) -> &mut u8 {
        &mut getter
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_log_2_min_luma_transform_block_size_minus_2`]
    pub fn max_log_2_min_luma_transform_block_size_minus_2_mut(&mut self) -> &mut u8 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::min_max_transform_hierarchy_depth_inter`]
    pub fn min_max_transform_hierarchy_depth_inter_mut(&mut self) -> &mut u8 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_max_transform_hierarchy_depth_inter`]
    pub fn max_max_transform_hierarchy_depth_inter_mut(&mut self) -> &mut u8 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::min_max_transform_hierarchy_depth_intra`]
    pub fn min_max_transform_hierarchy_depth_intra_mut(&mut self) -> &mut u8 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_max_transform_hierarchy_depth_intra`]
    pub fn max_max_transform_hierarchy_depth_intra_mut(&mut self) -> &mut u8 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_diff_cu_qp_delta_depth`]
    pub fn max_diff_cu_qp_delta_depth_mut(&mut self) -> &mut u8 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::min_max_num_merge_cand`]
    pub fn min_max_num_merge_cand_mut(&mut self) -> &mut u8 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_max_num_merge_cand`]
    pub fn max_max_num_merge_cand_mut(&mut self) -> &mut u8 {
        &mut getter
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
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::flags`]
    pub fn set_flags(
        &mut self,
        value: crate::extensions::ext_video_encode_h_265::VideoEncodeH265CapabilityFlagsEXT,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::input_mode_flags`]
    pub fn set_input_mode_flags(
        &mut self,
        value: crate::extensions::ext_video_encode_h_265::VideoEncodeH265InputModeFlagsEXT,
    ) -> &mut Self {
        self.input_mode_flags = value;
        self
    }
    ///Sets the raw value of [`Self::output_mode_flags`]
    pub fn set_output_mode_flags(
        &mut self,
        value: crate::extensions::ext_video_encode_h_265::VideoEncodeH265OutputModeFlagsEXT,
    ) -> &mut Self {
        self.output_mode_flags = value;
        self
    }
    ///Sets the raw value of [`Self::ctb_sizes`]
    pub fn set_ctb_sizes(
        &mut self,
        value: crate::extensions::ext_video_encode_h_265::VideoEncodeH265CtbSizeFlagsEXT,
    ) -> &mut Self {
        self.ctb_sizes = value;
        self
    }
    ///Sets the raw value of [`Self::transform_block_sizes`]
    pub fn set_transform_block_sizes(
        &mut self,
        value: crate::extensions::ext_video_encode_h_265::VideoEncodeH265TransformBlockSizeFlagsEXT,
    ) -> &mut Self {
        self.transform_block_sizes = value;
        self
    }
    ///Sets the raw value of [`Self::max_p_picture_l_0_reference_count`]
    pub fn set_max_p_picture_l_0_reference_count(&mut self, value: u8) -> &mut Self {
        self.max_p_picture_l_0_reference_count = value;
        self
    }
    ///Sets the raw value of [`Self::max_b_picture_l_0_reference_count`]
    pub fn set_max_b_picture_l_0_reference_count(&mut self, value: u8) -> &mut Self {
        self.max_b_picture_l_0_reference_count = value;
        self
    }
    ///Sets the raw value of [`Self::max_l_1_reference_count`]
    pub fn set_max_l_1_reference_count(&mut self, value: u8) -> &mut Self {
        self.max_l_1_reference_count = value;
        self
    }
    ///Sets the raw value of [`Self::max_sub_layers_count`]
    pub fn set_max_sub_layers_count(&mut self, value: u8) -> &mut Self {
        self.max_sub_layers_count = value;
        self
    }
    ///Sets the raw value of [`Self::min_log_2_min_luma_coding_block_size_minus_3`]
    pub fn set_min_log_2_min_luma_coding_block_size_minus_3(&mut self, value: u8) -> &mut Self {
        self.min_log_2_min_luma_coding_block_size_minus_3 = value;
        self
    }
    ///Sets the raw value of [`Self::max_log_2_min_luma_coding_block_size_minus_3`]
    pub fn set_max_log_2_min_luma_coding_block_size_minus_3(&mut self, value: u8) -> &mut Self {
        self.max_log_2_min_luma_coding_block_size_minus_3 = value;
        self
    }
    ///Sets the raw value of [`Self::min_log_2_min_luma_transform_block_size_minus_2`]
    pub fn set_min_log_2_min_luma_transform_block_size_minus_2(&mut self, value: u8) -> &mut Self {
        self.min_log_2_min_luma_transform_block_size_minus_2 = value;
        self
    }
    ///Sets the raw value of [`Self::max_log_2_min_luma_transform_block_size_minus_2`]
    pub fn set_max_log_2_min_luma_transform_block_size_minus_2(&mut self, value: u8) -> &mut Self {
        self.max_log_2_min_luma_transform_block_size_minus_2 = value;
        self
    }
    ///Sets the raw value of [`Self::min_max_transform_hierarchy_depth_inter`]
    pub fn set_min_max_transform_hierarchy_depth_inter(&mut self, value: u8) -> &mut Self {
        self.min_max_transform_hierarchy_depth_inter = value;
        self
    }
    ///Sets the raw value of [`Self::max_max_transform_hierarchy_depth_inter`]
    pub fn set_max_max_transform_hierarchy_depth_inter(&mut self, value: u8) -> &mut Self {
        self.max_max_transform_hierarchy_depth_inter = value;
        self
    }
    ///Sets the raw value of [`Self::min_max_transform_hierarchy_depth_intra`]
    pub fn set_min_max_transform_hierarchy_depth_intra(&mut self, value: u8) -> &mut Self {
        self.min_max_transform_hierarchy_depth_intra = value;
        self
    }
    ///Sets the raw value of [`Self::max_max_transform_hierarchy_depth_intra`]
    pub fn set_max_max_transform_hierarchy_depth_intra(&mut self, value: u8) -> &mut Self {
        self.max_max_transform_hierarchy_depth_intra = value;
        self
    }
    ///Sets the raw value of [`Self::max_diff_cu_qp_delta_depth`]
    pub fn set_max_diff_cu_qp_delta_depth(&mut self, value: u8) -> &mut Self {
        self.max_diff_cu_qp_delta_depth = value;
        self
    }
    ///Sets the raw value of [`Self::min_max_num_merge_cand`]
    pub fn set_min_max_num_merge_cand(&mut self, value: u8) -> &mut Self {
        self.min_max_num_merge_cand = value;
        self
    }
    ///Sets the raw value of [`Self::max_max_num_merge_cand`]
    pub fn set_max_max_num_merge_cand(&mut self, value: u8) -> &mut Self {
        self.max_max_num_merge_cand = value;
        self
    }
    ///Sets the raw value of [`Self::std_extension_version`]
    pub fn set_std_extension_version(&mut self, value: crate::vulkan1_0::ExtensionProperties) -> &mut Self {
        self.std_extension_version = value;
        self
    }
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
/// - [`std_extension_version`] is a pointer to a [`ExtensionProperties`] structure specifying the
///   H.265 codec extension version.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_SESSION_CREATE_INFO_EXT`
/// - [`flags`] **must**  be `0`
/// - [`std_extension_version`] **must**  be a valid pointer to a valid [`ExtensionProperties`]
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeH265SessionCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: VideoEncodeH265CreateFlagsEXT,
    ///[`std_extension_version`] is a pointer to a [`ExtensionProperties`]
    ///structure specifying the H.265 codec extension version.
    std_extension_version: *const ExtensionProperties,
}
impl<'lt> Default for VideoEncodeH265SessionCreateInfoEXT<'lt> {
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
impl<'lt> VideoEncodeH265SessionCreateInfoEXT<'lt> {
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
    pub fn flags(&self) -> VideoEncodeH265CreateFlagsEXT {
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
    pub fn flags_mut(&mut self) -> &mut VideoEncodeH265CreateFlagsEXT {
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
        value: crate::extensions::ext_video_encode_h_265::VideoEncodeH265CreateFlagsEXT,
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
/// - [`vps_std_count`] is the number of VPS elements in [`vps_std`].
/// - [`vps_std`] is a pointer to an array of [`vps_std_count`][`StdVideoH265VideoParameterSet`]
///   structures representing H.265 video parameter sets.
/// - [`sps_std_count`] is the number of SPS elements in [`sps_std`].
/// - [`sps_std`] is a pointer to an array of [`sps_std_count`][`StdVideoH265SequenceParameterSet`]
///   structures representing H.265 sequence parameter sets.
/// - [`pps_std_count`] is the number of PPS elements in [`pps_std`].
/// - [`pps_std`] is a pointer to an array of [`pps_std_count`][`StdVideoH265PictureParameterSet`]
///   structures representing H.265 picture parameter sets.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_SESSION_PARAMETERS_ADD_INFO_EXT`
/// - If [`vps_std`] is not `NULL`, [`vps_std`] **must**  be a valid pointer to an array of
///   [`vps_std_count`][`StdVideoH265VideoParameterSet`] values
/// - If [`sps_std`] is not `NULL`, [`sps_std`] **must**  be a valid pointer to an array of
///   [`sps_std_count`][`StdVideoH265SequenceParameterSet`] values
/// - If [`pps_std`] is not `NULL`, [`pps_std`] **must**  be a valid pointer to an array of
///   [`pps_std_count`][`StdVideoH265PictureParameterSet`] values
/// - [`vps_std_count`] **must**  be greater than `0`
/// - [`sps_std_count`] **must**  be greater than `0`
/// - [`pps_std_count`] **must**  be greater than `0`
///
///## Valid Usage
/// - The values of [`vps_std_count`], [`sps_std_count`] and [`pps_std_count`] **must**  be less
///   than or equal to the values of
///   [`VideoEncodeH265SessionParametersCreateInfoEXT::max_vps_std_count`],
///   [`VideoEncodeH265SessionParametersCreateInfoEXT`]:`maxSpsStdCount`, and
///   [`VideoEncodeH265SessionParametersCreateInfoEXT`]:`maxPpsStdCount`, respectively
/// - Each [`StdVideoH265VideoParameterSet`] entry in [`vps_std`] **must**  have a unique H.265 VPS
///   ID
/// - Each [`StdVideoH265SequenceParameterSet`] entry in [`sps_std`] **must**  have a unique H.265
///   VPS-SPS ID pair
/// - Each [`StdVideoH265PictureParameterSet`] entry in [`pps_std`] **must**  have a unique H.265
///   VPS-SPS-PPS ID tuple
/// - Each entry to be added  **must**  have a unique, to the rest of the parameter array entries
///   and the existing parameters in the Video Session Parameters Object that is being updated,
///   VPS-SPS-PPS IDs
/// - Parameter entries that already exist in Video Session Parameters object with a particular
///   VPS-SPS-PPS IDs  **must**  not be replaced nor updated
/// - When creating a new object using a Video Session Parameters as a template, the array’s
///   parameters with the same VPS-SPS-PPS IDs as the ones from the template take precedence
/// - VPS/SPS/PPS parameters  **must**  comply with the limits specified in
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeH265SessionParametersAddInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`vps_std_count`] is the number of VPS elements in [`vps_std`].
    vps_std_count: u32,
    ///[`vps_std`] is a pointer to an array of [`vps_std_count`][`StdVideoH265VideoParameterSet`]
    /// structures representing H.265 video parameter sets.
    vps_std: *const StdVideoH265VideoParameterSet,
    ///[`sps_std_count`] is the number of SPS elements in [`sps_std`].
    sps_std_count: u32,
    ///[`sps_std`] is a pointer to an array of
    /// [`sps_std_count`][`StdVideoH265SequenceParameterSet`] structures representing H.265
    /// sequence parameter sets.
    sps_std: *const StdVideoH265SequenceParameterSet,
    ///[`pps_std_count`] is the number of PPS elements in [`pps_std`].
    pps_std_count: u32,
    ///[`pps_std`] is a pointer to an array of [`pps_std_count`][`StdVideoH265PictureParameterSet`]
    /// structures representing H.265 picture parameter sets.
    pps_std: *const StdVideoH265PictureParameterSet,
}
impl<'lt> Default for VideoEncodeH265SessionParametersAddInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            vps_std_count: 0,
            vps_std: std::ptr::null(),
            sps_std_count: 0,
            sps_std: std::ptr::null(),
            pps_std_count: 0,
            pps_std: std::ptr::null(),
        }
    }
}
impl<'lt> VideoEncodeH265SessionParametersAddInfoEXT<'lt> {
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
    ///Gets the value of [`Self::vps_std_count`]
    pub fn vps_std_count(&self) -> u32 {
        self.vps_std_count
    }
    ///Gets the value of [`Self::vps_std`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn vps_std(&self) -> &[StdVideoH265VideoParameterSet] {
        std::slice::from_raw_parts(self.vps_std, self.vps_std_count as usize)
    }
    ///Gets the value of [`Self::sps_std_count`]
    pub fn sps_std_count(&self) -> u32 {
        self.sps_std_count
    }
    ///Gets the value of [`Self::sps_std`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn sps_std(&self) -> &[StdVideoH265SequenceParameterSet] {
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
    pub unsafe fn pps_std(&self) -> &[StdVideoH265PictureParameterSet] {
        std::slice::from_raw_parts(self.pps_std, self.pps_std_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::vps_std_count`]
    pub fn vps_std_count_mut(&mut self) -> &mut u32 {
        &mut getter
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
    ///Sets the raw value of [`Self::vps_std_count`]
    pub fn set_vps_std_count(&mut self, value: u32) -> &mut Self {
        self.vps_std_count = value;
        self
    }
    ///Sets the raw value of [`Self::vps_std`]
    pub fn set_vps_std(&mut self, value: &'lt [crate::native::StdVideoH265VideoParameterSet]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.vps_std = value.as_ptr();
        self.vps_std_count = len_;
        self
    }
    ///Sets the raw value of [`Self::sps_std_count`]
    pub fn set_sps_std_count(&mut self, value: u32) -> &mut Self {
        self.sps_std_count = value;
        self
    }
    ///Sets the raw value of [`Self::sps_std`]
    pub fn set_sps_std(&mut self, value: &'lt [crate::native::StdVideoH265SequenceParameterSet]) -> &mut Self {
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
    pub fn set_pps_std(&mut self, value: &'lt [crate::native::StdVideoH265PictureParameterSet]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.pps_std = value.as_ptr();
        self.pps_std_count = len_;
        self
    }
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
/// - [`parameters_add_info`] is `NULL` or a pointer to a
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
///to add these entries.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_SESSION_PARAMETERS_CREATE_INFO_EXT`
/// - If [`parameters_add_info`] is not `NULL`, [`parameters_add_info`] **must**  be a valid pointer
///   to a valid [`VideoEncodeH265SessionParametersAddInfoEXT`] structure
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeH265SessionParametersCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
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
    ///[`parameters_add_info`] is `NULL` or a pointer to a
    ///[`VideoEncodeH265SessionParametersAddInfoEXT`] structure specifying
    ///the video session parameters to add upon creation of this object.
    parameters_add_info: *const VideoEncodeH265SessionParametersAddInfoEXT<'lt>,
}
impl<'lt> Default for VideoEncodeH265SessionParametersCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            max_vps_std_count: 0,
            max_sps_std_count: 0,
            max_pps_std_count: 0,
            parameters_add_info: std::ptr::null(),
        }
    }
}
impl<'lt> VideoEncodeH265SessionParametersCreateInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::parameters_add_info`]
    pub fn parameters_add_info_raw(&self) -> *const VideoEncodeH265SessionParametersAddInfoEXT<'lt> {
        self.parameters_add_info
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::parameters_add_info`]
    pub fn set_parameters_add_info_raw(
        &mut self,
        value: *const VideoEncodeH265SessionParametersAddInfoEXT<'lt>,
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
    ///Gets the value of [`Self::max_vps_std_count`]
    pub fn max_vps_std_count(&self) -> u32 {
        self.max_vps_std_count
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
    pub unsafe fn parameters_add_info(&self) -> &VideoEncodeH265SessionParametersAddInfoEXT<'lt> {
        &*self.parameters_add_info
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::max_vps_std_count`]
    pub fn max_vps_std_count_mut(&mut self) -> &mut u32 {
        &mut getter
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
    ///Sets the raw value of [`Self::max_vps_std_count`]
    pub fn set_max_vps_std_count(&mut self, value: u32) -> &mut Self {
        self.max_vps_std_count = value;
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
        value: &'lt crate::extensions::ext_video_encode_h_265::VideoEncodeH265SessionParametersAddInfoEXT<'lt>,
    ) -> &mut Self {
        self.parameters_add_info = value as *const _;
        self
    }
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
/// - [`reference_final_lists`] is `NULL` or a pointer to a [`VideoEncodeH265ReferenceListsEXT`]
///   structure specifying the reference lists to be used for the current picture.
/// - [`nalu_slice_segment_entry_count`] is the number of slice segment NALUs in the frame.
/// - [`nalu_slice_segment_entries`] is a pointer to an array of
///   [`VideoEncodeH265NaluSliceSegmentEXT`] structures specifying the division of the current
///   picture into slice segments and the properties of these slice segments.
/// - [`current_picture_info`] is a pointer to a [`StdVideoEncodeH265PictureInfo`] structure
///   specifying the syntax and other codec-specific information from the H.265 specification,
///   associated with this picture.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_VCL_FRAME_INFO_EXT`
/// - If [`reference_final_lists`] is not `NULL`, [`reference_final_lists`] **must**  be a valid
///   pointer to a valid [`VideoEncodeH265ReferenceListsEXT`] structure
/// - [`nalu_slice_segment_entries`] **must**  be a valid pointer to an array of
///   [`nalu_slice_segment_entry_count`] valid [`VideoEncodeH265NaluSliceSegmentEXT`] structures
/// - [`current_picture_info`] **must**  be a valid pointer to a valid
///   [`StdVideoEncodeH265PictureInfo`] value
/// - [`nalu_slice_segment_entry_count`] **must**  be greater than `0`
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeH265VclFrameInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`reference_final_lists`] is `NULL` or a pointer to a
    ///[`VideoEncodeH265ReferenceListsEXT`] structure specifying the
    ///reference lists to be used for the current picture.
    reference_final_lists: *const VideoEncodeH265ReferenceListsEXT<'lt>,
    ///[`nalu_slice_segment_entry_count`] is the number of slice segment NALUs in
    ///the frame.
    nalu_slice_segment_entry_count: u32,
    ///[`nalu_slice_segment_entries`] is a pointer to an array of
    ///[`VideoEncodeH265NaluSliceSegmentEXT`] structures specifying the
    ///division of the current picture into slice segments and the properties
    ///of these slice segments.
    nalu_slice_segment_entries: *const VideoEncodeH265NaluSliceSegmentEXT<'lt>,
    ///[`current_picture_info`] is a pointer to a
    ///[`StdVideoEncodeH265PictureInfo`] structure specifying the syntax and
    ///other codec-specific information from the H.265 specification,
    ///associated with this picture.
    current_picture_info: *const StdVideoEncodeH265PictureInfo,
}
impl<'lt> Default for VideoEncodeH265VclFrameInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            reference_final_lists: std::ptr::null(),
            nalu_slice_segment_entry_count: 0,
            nalu_slice_segment_entries: std::ptr::null(),
            current_picture_info: std::ptr::null(),
        }
    }
}
impl<'lt> VideoEncodeH265VclFrameInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::reference_final_lists`]
    pub fn reference_final_lists_raw(&self) -> *const VideoEncodeH265ReferenceListsEXT<'lt> {
        self.reference_final_lists
    }
    ///Gets the raw value of [`Self::nalu_slice_segment_entries`]
    pub fn nalu_slice_segment_entries_raw(&self) -> *const VideoEncodeH265NaluSliceSegmentEXT<'lt> {
        self.nalu_slice_segment_entries
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::reference_final_lists`]
    pub fn set_reference_final_lists_raw(&mut self, value: *const VideoEncodeH265ReferenceListsEXT<'lt>) -> &mut Self {
        self.reference_final_lists = value;
        self
    }
    ///Sets the raw value of [`Self::nalu_slice_segment_entries`]
    pub fn set_nalu_slice_segment_entries_raw(
        &mut self,
        value: *const VideoEncodeH265NaluSliceSegmentEXT<'lt>,
    ) -> &mut Self {
        self.nalu_slice_segment_entries = value;
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
    ///Gets the value of [`Self::reference_final_lists`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn reference_final_lists(&self) -> &VideoEncodeH265ReferenceListsEXT<'lt> {
        &*self.reference_final_lists
    }
    ///Gets the value of [`Self::nalu_slice_segment_entry_count`]
    pub fn nalu_slice_segment_entry_count(&self) -> u32 {
        self.nalu_slice_segment_entry_count
    }
    ///Gets the value of [`Self::nalu_slice_segment_entries`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn nalu_slice_segment_entries(&self) -> &[VideoEncodeH265NaluSliceSegmentEXT<'lt>] {
        std::slice::from_raw_parts(
            self.nalu_slice_segment_entries,
            self.nalu_slice_segment_entry_count as usize,
        )
    }
    ///Gets the value of [`Self::current_picture_info`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn current_picture_info(&self) -> &StdVideoEncodeH265PictureInfo {
        &*self.current_picture_info
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::nalu_slice_segment_entry_count`]
    pub fn nalu_slice_segment_entry_count_mut(&mut self) -> &mut u32 {
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
    ///Sets the raw value of [`Self::reference_final_lists`]
    pub fn set_reference_final_lists(
        &mut self,
        value: &'lt crate::extensions::ext_video_encode_h_265::VideoEncodeH265ReferenceListsEXT<'lt>,
    ) -> &mut Self {
        self.reference_final_lists = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::nalu_slice_segment_entry_count`]
    pub fn set_nalu_slice_segment_entry_count(&mut self, value: u32) -> &mut Self {
        self.nalu_slice_segment_entry_count = value;
        self
    }
    ///Sets the raw value of [`Self::nalu_slice_segment_entries`]
    pub fn set_nalu_slice_segment_entries(
        &mut self,
        value: &'lt [crate::extensions::ext_video_encode_h_265::VideoEncodeH265NaluSliceSegmentEXT<'lt>],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.nalu_slice_segment_entries = value.as_ptr();
        self.nalu_slice_segment_entry_count = len_;
        self
    }
    ///Sets the raw value of [`Self::current_picture_info`]
    pub fn set_current_picture_info(&mut self, value: &'lt crate::native::StdVideoEncodeH265PictureInfo) -> &mut Self {
        self.current_picture_info = value as *const _;
        self
    }
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
///   **must**  match the VPS provided in `vpsStd` of
///   [`VideoEncodeH265SessionParametersCreateInfoEXT`]. This is retrieved from the
///   [`VideoSessionParametersKHR`] object provided in [`VideoBeginCodingInfoKHR`].
/// - [`sps_id`] is the H.265 SPS ID for the H.265 SPS to insert in the bitstream. The SPS ID
///   **must**  match one of the IDs of the SPS(s) provided in `pSpsStd` of
///   [`VideoEncodeH265SessionParametersCreateInfoEXT`] to identify the SPS parameter set to insert
///   in the bitstream. This is retrieved from the [`VideoSessionParametersKHR`] object provided in
///   [`VideoBeginCodingInfoKHR`].
/// - [`emit_vps_enable`] enables the emitting of the VPS structure with id of [`vps_id`].
/// - [`emit_sps_enable`] enables the emitting of the SPS structure with id of [`sps_id`].
/// - [`pps_id_entry_count`] is the number of entries in the [`pps_id_entries`]. If this parameter
///   is `0` then no pps entries are going to be emitted in the bitstream.
/// - [`pps_id_entries`] is the H.265 PPS IDs for the H.265 PPS to insert in the bitstream. The PPS
///   IDs  **must**  match one of the IDs of the PPS(s) provided in `pPpsStd` of
///   [`VideoEncodeH265SessionParametersCreateInfoEXT`] to identify the PPS parameter set to insert
///   in the bitstream. This is retrieved from the [`VideoSessionParametersKHR`] object provided in
///   [`VideoBeginCodingInfoKHR`].
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_EMIT_PICTURE_PARAMETERS_EXT`
/// - If [`pps_id_entry_count`] is not `0`, [`pps_id_entries`] **must**  be a valid pointer to an
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeH265EmitPictureParametersEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`vps_id`] is the H.265 VPS ID for the H.265 VPS to insert in the
    ///bitstream.
    ///The VPS ID  **must**  match the VPS provided in `vpsStd` of
    ///[`VideoEncodeH265SessionParametersCreateInfoEXT`].
    ///This is retrieved from the [`VideoSessionParametersKHR`] object
    ///provided in [`VideoBeginCodingInfoKHR`].
    vps_id: u8,
    ///[`sps_id`] is the H.265 SPS ID for the H.265 SPS to insert in the
    ///bitstream.
    ///The SPS ID  **must**  match one of the IDs of the SPS(s) provided in
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
    ///The PPS IDs  **must**  match one of the IDs of the PPS(s) provided in
    ///`pPpsStd` of [`VideoEncodeH265SessionParametersCreateInfoEXT`]
    ///to identify the PPS parameter set to insert in the bitstream.
    ///This is retrieved from the [`VideoSessionParametersKHR`] object
    ///provided in [`VideoBeginCodingInfoKHR`].
    pps_id_entries: *const u8,
}
impl<'lt> Default for VideoEncodeH265EmitPictureParametersEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            vps_id: 0,
            sps_id: 0,
            emit_vps_enable: 0,
            emit_sps_enable: 0,
            pps_id_entry_count: 0,
            pps_id_entries: std::ptr::null(),
        }
    }
}
impl<'lt> VideoEncodeH265EmitPictureParametersEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::emit_vps_enable`]
    pub fn emit_vps_enable_raw(&self) -> Bool32 {
        self.emit_vps_enable
    }
    ///Gets the raw value of [`Self::emit_sps_enable`]
    pub fn emit_sps_enable_raw(&self) -> Bool32 {
        self.emit_sps_enable
    }
    ///Gets the raw value of [`Self::pps_id_entries`]
    pub fn pps_id_entries_raw(&self) -> *const u8 {
        self.pps_id_entries
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::emit_vps_enable`]
    pub fn set_emit_vps_enable_raw(&mut self, value: Bool32) -> &mut Self {
        self.emit_vps_enable = value;
        self
    }
    ///Sets the raw value of [`Self::emit_sps_enable`]
    pub fn set_emit_sps_enable_raw(&mut self, value: Bool32) -> &mut Self {
        self.emit_sps_enable = value;
        self
    }
    ///Sets the raw value of [`Self::pps_id_entries`]
    pub fn set_pps_id_entries_raw(&mut self, value: *const u8) -> &mut Self {
        self.pps_id_entries = value;
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
    ///Gets the value of [`Self::vps_id`]
    pub fn vps_id(&self) -> u8 {
        self.vps_id
    }
    ///Gets the value of [`Self::sps_id`]
    pub fn sps_id(&self) -> u8 {
        self.sps_id
    }
    ///Gets the value of [`Self::emit_vps_enable`]
    pub fn emit_vps_enable(&self) -> bool {
        unsafe { std::mem::transmute(self.emit_vps_enable as u8) }
    }
    ///Gets the value of [`Self::emit_sps_enable`]
    pub fn emit_sps_enable(&self) -> bool {
        unsafe { std::mem::transmute(self.emit_sps_enable as u8) }
    }
    ///Gets the value of [`Self::pps_id_entry_count`]
    pub fn pps_id_entry_count(&self) -> u32 {
        self.pps_id_entry_count
    }
    ///Gets the value of [`Self::pps_id_entries`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn pps_id_entries(&self) -> &[u8] {
        std::slice::from_raw_parts(self.pps_id_entries, self.pps_id_entry_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::vps_id`]
    pub fn vps_id_mut(&mut self) -> &mut u8 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::sps_id`]
    pub fn sps_id_mut(&mut self) -> &mut u8 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::emit_vps_enable`]
    pub fn emit_vps_enable_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.emit_vps_enable as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.emit_vps_enable as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::emit_sps_enable`]
    pub fn emit_sps_enable_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.emit_sps_enable as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.emit_sps_enable as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::pps_id_entry_count`]
    pub fn pps_id_entry_count_mut(&mut self) -> &mut u32 {
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
    ///Sets the raw value of [`Self::vps_id`]
    pub fn set_vps_id(&mut self, value: u8) -> &mut Self {
        self.vps_id = value;
        self
    }
    ///Sets the raw value of [`Self::sps_id`]
    pub fn set_sps_id(&mut self, value: u8) -> &mut Self {
        self.sps_id = value;
        self
    }
    ///Sets the raw value of [`Self::emit_vps_enable`]
    pub fn set_emit_vps_enable(&mut self, value: bool) -> &mut Self {
        self.emit_vps_enable = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::emit_sps_enable`]
    pub fn set_emit_sps_enable(&mut self, value: bool) -> &mut Self {
        self.emit_sps_enable = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::pps_id_entry_count`]
    pub fn set_pps_id_entry_count(&mut self, value: u32) -> &mut Self {
        self.pps_id_entry_count = value;
        self
    }
    ///Sets the raw value of [`Self::pps_id_entries`]
    pub fn set_pps_id_entries(&mut self, value: &'lt [u8]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.pps_id_entries = value.as_ptr();
        self.pps_id_entry_count = len_;
        self
    }
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
/// - [`reference_final_lists`] is `NULL` or a pointer to a [`VideoEncodeH265ReferenceListsEXT`]
///   structure specifying the reference lists to be used for the current slice segment. If
///   [`reference_final_lists`] is not `NULL`, these reference lists override the reference lists
///   provided in [`VideoEncodeH265VclFrameInfoEXT`]::[`reference_final_lists`].
/// - [`slice_segment_header_std`] is a pointer to a [`StdVideoEncodeH265SliceSegmentHeader`]
///   structure specifying the slice segment header for the current slice segment.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_NALU_SLICE_SEGMENT_EXT`
/// - [`p_next`] **must**  be `NULL`
/// - If [`reference_final_lists`] is not `NULL`, [`reference_final_lists`] **must**  be a valid
///   pointer to a valid [`VideoEncodeH265ReferenceListsEXT`] structure
/// - [`slice_segment_header_std`] **must**  be a valid pointer to a valid
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeH265NaluSliceSegmentEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`ctb_count`] is the number of CTBs in this slice segment.
    ctb_count: u32,
    ///[`reference_final_lists`] is `NULL` or a pointer to a
    ///[`VideoEncodeH265ReferenceListsEXT`] structure specifying the
    ///reference lists to be used for the current slice segment.
    ///If [`reference_final_lists`] is not `NULL`, these reference lists
    ///override the reference lists provided in
    ///[`VideoEncodeH265VclFrameInfoEXT`]::[`reference_final_lists`].
    reference_final_lists: *const VideoEncodeH265ReferenceListsEXT<'lt>,
    ///[`slice_segment_header_std`] is a pointer to a
    ///[`StdVideoEncodeH265SliceSegmentHeader`] structure specifying the slice
    ///segment header for the current slice segment.
    slice_segment_header_std: *const StdVideoEncodeH265SliceSegmentHeader,
}
impl<'lt> Default for VideoEncodeH265NaluSliceSegmentEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            ctb_count: 0,
            reference_final_lists: std::ptr::null(),
            slice_segment_header_std: std::ptr::null(),
        }
    }
}
impl<'lt> VideoEncodeH265NaluSliceSegmentEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::reference_final_lists`]
    pub fn reference_final_lists_raw(&self) -> *const VideoEncodeH265ReferenceListsEXT<'lt> {
        self.reference_final_lists
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::reference_final_lists`]
    pub fn set_reference_final_lists_raw(&mut self, value: *const VideoEncodeH265ReferenceListsEXT<'lt>) -> &mut Self {
        self.reference_final_lists = value;
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
    ///Gets the value of [`Self::ctb_count`]
    pub fn ctb_count(&self) -> u32 {
        self.ctb_count
    }
    ///Gets the value of [`Self::reference_final_lists`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn reference_final_lists(&self) -> &VideoEncodeH265ReferenceListsEXT<'lt> {
        &*self.reference_final_lists
    }
    ///Gets the value of [`Self::slice_segment_header_std`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn slice_segment_header_std(&self) -> &StdVideoEncodeH265SliceSegmentHeader {
        &*self.slice_segment_header_std
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::ctb_count`]
    pub fn ctb_count_mut(&mut self) -> &mut u32 {
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
    ///Sets the raw value of [`Self::ctb_count`]
    pub fn set_ctb_count(&mut self, value: u32) -> &mut Self {
        self.ctb_count = value;
        self
    }
    ///Sets the raw value of [`Self::reference_final_lists`]
    pub fn set_reference_final_lists(
        &mut self,
        value: &'lt crate::extensions::ext_video_encode_h_265::VideoEncodeH265ReferenceListsEXT<'lt>,
    ) -> &mut Self {
        self.reference_final_lists = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::slice_segment_header_std`]
    pub fn set_slice_segment_header_std(
        &mut self,
        value: &'lt crate::native::StdVideoEncodeH265SliceSegmentHeader,
    ) -> &mut Self {
        self.slice_segment_header_std = value as *const _;
        self
    }
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
///inferred picture type does not match the actual picture type.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_RATE_CONTROL_INFO_EXT`
/// - [`rate_control_structure`] **must**  be a valid
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeH265RateControlInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
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
impl<'lt> Default for VideoEncodeH265RateControlInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            gop_frame_count: 0,
            idr_period: 0,
            consecutive_b_frame_count: 0,
            rate_control_structure: Default::default(),
            sub_layer_count: 0,
        }
    }
}
impl<'lt> VideoEncodeH265RateControlInfoEXT<'lt> {
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
    ///Gets the value of [`Self::gop_frame_count`]
    pub fn gop_frame_count(&self) -> u32 {
        self.gop_frame_count
    }
    ///Gets the value of [`Self::idr_period`]
    pub fn idr_period(&self) -> u32 {
        self.idr_period
    }
    ///Gets the value of [`Self::consecutive_b_frame_count`]
    pub fn consecutive_b_frame_count(&self) -> u32 {
        self.consecutive_b_frame_count
    }
    ///Gets the value of [`Self::rate_control_structure`]
    pub fn rate_control_structure(&self) -> VideoEncodeH265RateControlStructureFlagBitsEXT {
        self.rate_control_structure
    }
    ///Gets the value of [`Self::sub_layer_count`]
    pub fn sub_layer_count(&self) -> u8 {
        self.sub_layer_count
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::gop_frame_count`]
    pub fn gop_frame_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::idr_period`]
    pub fn idr_period_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::consecutive_b_frame_count`]
    pub fn consecutive_b_frame_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::rate_control_structure`]
    pub fn rate_control_structure_mut(&mut self) -> &mut VideoEncodeH265RateControlStructureFlagBitsEXT {
        &mut self.rate_control_structure
    }
    ///Gets a mutable reference to the value of [`Self::sub_layer_count`]
    pub fn sub_layer_count_mut(&mut self) -> &mut u8 {
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
    ///Sets the raw value of [`Self::gop_frame_count`]
    pub fn set_gop_frame_count(&mut self, value: u32) -> &mut Self {
        self.gop_frame_count = value;
        self
    }
    ///Sets the raw value of [`Self::idr_period`]
    pub fn set_idr_period(&mut self, value: u32) -> &mut Self {
        self.idr_period = value;
        self
    }
    ///Sets the raw value of [`Self::consecutive_b_frame_count`]
    pub fn set_consecutive_b_frame_count(&mut self, value: u32) -> &mut Self {
        self.consecutive_b_frame_count = value;
        self
    }
    ///Sets the raw value of [`Self::rate_control_structure`]
    pub fn set_rate_control_structure(
        &mut self,
        value: crate::extensions::ext_video_encode_h_265::VideoEncodeH265RateControlStructureFlagBitsEXT,
    ) -> &mut Self {
        self.rate_control_structure = value;
        self
    }
    ///Sets the raw value of [`Self::sub_layer_count`]
    pub fn set_sub_layer_count(&mut self, value: u8) -> &mut Self {
        self.sub_layer_count = value;
        self
    }
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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
impl Default for VideoEncodeH265QpEXT {
    fn default() -> Self {
        Self {
            qp_i: 0,
            qp_p: 0,
            qp_b: 0,
        }
    }
}
impl VideoEncodeH265QpEXT {
    ///Gets the value of [`Self::qp_i`]
    pub fn qp_i(&self) -> i32 {
        self.qp_i
    }
    ///Gets the value of [`Self::qp_p`]
    pub fn qp_p(&self) -> i32 {
        self.qp_p
    }
    ///Gets the value of [`Self::qp_b`]
    pub fn qp_b(&self) -> i32 {
        self.qp_b
    }
    ///Gets a mutable reference to the value of [`Self::qp_i`]
    pub fn qp_i_mut(&mut self) -> &mut i32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::qp_p`]
    pub fn qp_p_mut(&mut self) -> &mut i32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::qp_b`]
    pub fn qp_b_mut(&mut self) -> &mut i32 {
        &mut getter
    }
    ///Sets the raw value of [`Self::qp_i`]
    pub fn set_qp_i(&mut self, value: i32) -> &mut Self {
        self.qp_i = value;
        self
    }
    ///Sets the raw value of [`Self::qp_p`]
    pub fn set_qp_p(&mut self, value: i32) -> &mut Self {
        self.qp_p = value;
        self
    }
    ///Sets the raw value of [`Self::qp_b`]
    pub fn set_qp_b(&mut self, value: i32) -> &mut Self {
        self.qp_b = value;
        self
    }
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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
impl Default for VideoEncodeH265FrameSizeEXT {
    fn default() -> Self {
        Self {
            frame_i_size: 0,
            frame_p_size: 0,
            frame_b_size: 0,
        }
    }
}
impl VideoEncodeH265FrameSizeEXT {
    ///Gets the value of [`Self::frame_i_size`]
    pub fn frame_i_size(&self) -> u32 {
        self.frame_i_size
    }
    ///Gets the value of [`Self::frame_p_size`]
    pub fn frame_p_size(&self) -> u32 {
        self.frame_p_size
    }
    ///Gets the value of [`Self::frame_b_size`]
    pub fn frame_b_size(&self) -> u32 {
        self.frame_b_size
    }
    ///Gets a mutable reference to the value of [`Self::frame_i_size`]
    pub fn frame_i_size_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::frame_p_size`]
    pub fn frame_p_size_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::frame_b_size`]
    pub fn frame_b_size_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Sets the raw value of [`Self::frame_i_size`]
    pub fn set_frame_i_size(&mut self, value: u32) -> &mut Self {
        self.frame_i_size = value;
        self
    }
    ///Sets the raw value of [`Self::frame_p_size`]
    pub fn set_frame_p_size(&mut self, value: u32) -> &mut Self {
        self.frame_p_size = value;
        self
    }
    ///Sets the raw value of [`Self::frame_b_size`]
    pub fn set_frame_b_size(&mut self, value: u32) -> &mut Self {
        self.frame_b_size = value;
        self
    }
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
///   limits, however these limits  **may**  be used as a guide in rate control calculations. If
///   enabled and not set properly, the [`max_qp`] limit may prevent the implementation from
///   respecting the [`max_frame_size`] limit.
///# Description
///H.265-specific per-layer rate control parameters  **must**  be specified by
///adding a [`VideoEncodeH265RateControlLayerInfoEXT`] structure to the
///[`p_next`] chain of each [`VideoEncodeRateControlLayerInfoKHR`]
///structure in a call to [`CmdControlVideoCodingKHR`] command, when the
///command buffer context has an active video encode H.265 session.
///## Valid Usage
/// - When [`VideoEncodeRateControlInfoKHR::rate_control_mode`] is
///   `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`, both [`use_min_qp`] and [`use_max_qp`] must
///   be set to [`TRUE`].
/// - When [`VideoEncodeRateControlInfoKHR::rate_control_mode`] is
///   `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`, the values provided in `minQP` must be
///   identical to those provided in [`max_qp`].
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_RATE_CONTROL_LAYER_INFO_EXT`
/// - [`initial_rc_qp`] **must**  be a valid [`VideoEncodeH265QpEXT`] structure
/// - [`min_qp`] **must**  be a valid [`VideoEncodeH265QpEXT`] structure
/// - [`max_qp`] **must**  be a valid [`VideoEncodeH265QpEXT`] structure
/// - [`max_frame_size`] **must**  be a valid [`VideoEncodeH265FrameSizeEXT`] structure
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeH265RateControlLayerInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
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
    ///within the specified limits, however these limits  **may**  be used as a
    ///guide in rate control calculations.
    ///If enabled and not set properly, the [`max_qp`] limit may prevent the
    ///implementation from respecting the [`max_frame_size`] limit.
    max_frame_size: VideoEncodeH265FrameSizeEXT,
}
impl<'lt> Default for VideoEncodeH265RateControlLayerInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            temporal_id: 0,
            use_initial_rc_qp: 0,
            initial_rc_qp: Default::default(),
            use_min_qp: 0,
            min_qp: Default::default(),
            use_max_qp: 0,
            max_qp: Default::default(),
            use_max_frame_size: 0,
            max_frame_size: Default::default(),
        }
    }
}
impl<'lt> VideoEncodeH265RateControlLayerInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::use_initial_rc_qp`]
    pub fn use_initial_rc_qp_raw(&self) -> Bool32 {
        self.use_initial_rc_qp
    }
    ///Gets the raw value of [`Self::use_min_qp`]
    pub fn use_min_qp_raw(&self) -> Bool32 {
        self.use_min_qp
    }
    ///Gets the raw value of [`Self::use_max_qp`]
    pub fn use_max_qp_raw(&self) -> Bool32 {
        self.use_max_qp
    }
    ///Gets the raw value of [`Self::use_max_frame_size`]
    pub fn use_max_frame_size_raw(&self) -> Bool32 {
        self.use_max_frame_size
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::use_initial_rc_qp`]
    pub fn set_use_initial_rc_qp_raw(&mut self, value: Bool32) -> &mut Self {
        self.use_initial_rc_qp = value;
        self
    }
    ///Sets the raw value of [`Self::use_min_qp`]
    pub fn set_use_min_qp_raw(&mut self, value: Bool32) -> &mut Self {
        self.use_min_qp = value;
        self
    }
    ///Sets the raw value of [`Self::use_max_qp`]
    pub fn set_use_max_qp_raw(&mut self, value: Bool32) -> &mut Self {
        self.use_max_qp = value;
        self
    }
    ///Sets the raw value of [`Self::use_max_frame_size`]
    pub fn set_use_max_frame_size_raw(&mut self, value: Bool32) -> &mut Self {
        self.use_max_frame_size = value;
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
    ///Gets the value of [`Self::temporal_id`]
    pub fn temporal_id(&self) -> u8 {
        self.temporal_id
    }
    ///Gets the value of [`Self::use_initial_rc_qp`]
    pub fn use_initial_rc_qp(&self) -> bool {
        unsafe { std::mem::transmute(self.use_initial_rc_qp as u8) }
    }
    ///Gets the value of [`Self::initial_rc_qp`]
    pub fn initial_rc_qp(&self) -> VideoEncodeH265QpEXT {
        self.initial_rc_qp
    }
    ///Gets the value of [`Self::use_min_qp`]
    pub fn use_min_qp(&self) -> bool {
        unsafe { std::mem::transmute(self.use_min_qp as u8) }
    }
    ///Gets the value of [`Self::min_qp`]
    pub fn min_qp(&self) -> VideoEncodeH265QpEXT {
        self.min_qp
    }
    ///Gets the value of [`Self::use_max_qp`]
    pub fn use_max_qp(&self) -> bool {
        unsafe { std::mem::transmute(self.use_max_qp as u8) }
    }
    ///Gets the value of [`Self::max_qp`]
    pub fn max_qp(&self) -> VideoEncodeH265QpEXT {
        self.max_qp
    }
    ///Gets the value of [`Self::use_max_frame_size`]
    pub fn use_max_frame_size(&self) -> bool {
        unsafe { std::mem::transmute(self.use_max_frame_size as u8) }
    }
    ///Gets the value of [`Self::max_frame_size`]
    pub fn max_frame_size(&self) -> VideoEncodeH265FrameSizeEXT {
        self.max_frame_size
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::temporal_id`]
    pub fn temporal_id_mut(&mut self) -> &mut u8 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::use_initial_rc_qp`]
    pub fn use_initial_rc_qp_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.use_initial_rc_qp as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.use_initial_rc_qp as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::initial_rc_qp`]
    pub fn initial_rc_qp_mut(&mut self) -> &mut VideoEncodeH265QpEXT {
        &mut self.initial_rc_qp
    }
    ///Gets a mutable reference to the value of [`Self::use_min_qp`]
    pub fn use_min_qp_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.use_min_qp as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.use_min_qp as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::min_qp`]
    pub fn min_qp_mut(&mut self) -> &mut VideoEncodeH265QpEXT {
        &mut self.min_qp
    }
    ///Gets a mutable reference to the value of [`Self::use_max_qp`]
    pub fn use_max_qp_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.use_max_qp as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.use_max_qp as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::max_qp`]
    pub fn max_qp_mut(&mut self) -> &mut VideoEncodeH265QpEXT {
        &mut self.max_qp
    }
    ///Gets a mutable reference to the value of [`Self::use_max_frame_size`]
    pub fn use_max_frame_size_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.use_max_frame_size as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.use_max_frame_size as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::max_frame_size`]
    pub fn max_frame_size_mut(&mut self) -> &mut VideoEncodeH265FrameSizeEXT {
        &mut self.max_frame_size
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
    ///Sets the raw value of [`Self::temporal_id`]
    pub fn set_temporal_id(&mut self, value: u8) -> &mut Self {
        self.temporal_id = value;
        self
    }
    ///Sets the raw value of [`Self::use_initial_rc_qp`]
    pub fn set_use_initial_rc_qp(&mut self, value: bool) -> &mut Self {
        self.use_initial_rc_qp = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::initial_rc_qp`]
    pub fn set_initial_rc_qp(
        &mut self,
        value: crate::extensions::ext_video_encode_h_265::VideoEncodeH265QpEXT,
    ) -> &mut Self {
        self.initial_rc_qp = value;
        self
    }
    ///Sets the raw value of [`Self::use_min_qp`]
    pub fn set_use_min_qp(&mut self, value: bool) -> &mut Self {
        self.use_min_qp = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::min_qp`]
    pub fn set_min_qp(&mut self, value: crate::extensions::ext_video_encode_h_265::VideoEncodeH265QpEXT) -> &mut Self {
        self.min_qp = value;
        self
    }
    ///Sets the raw value of [`Self::use_max_qp`]
    pub fn set_use_max_qp(&mut self, value: bool) -> &mut Self {
        self.use_max_qp = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::max_qp`]
    pub fn set_max_qp(&mut self, value: crate::extensions::ext_video_encode_h_265::VideoEncodeH265QpEXT) -> &mut Self {
        self.max_qp = value;
        self
    }
    ///Sets the raw value of [`Self::use_max_frame_size`]
    pub fn set_use_max_frame_size(&mut self, value: bool) -> &mut Self {
        self.use_max_frame_size = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::max_frame_size`]
    pub fn set_max_frame_size(
        &mut self,
        value: crate::extensions::ext_video_encode_h_265::VideoEncodeH265FrameSizeEXT,
    ) -> &mut Self {
        self.max_frame_size = value;
        self
    }
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
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_PROFILE_EXT`
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
#[derive(Debug)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeH265ProfileEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`std_profile_idc`] is a [`StdVideoH265ProfileIdc`] value specifying
    ///the H.265 codec profile IDC.
    std_profile_idc: StdVideoH265ProfileIdc,
}
impl<'lt> Default for VideoEncodeH265ProfileEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            std_profile_idc: Default::default(),
        }
    }
}
impl<'lt> VideoEncodeH265ProfileEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::std_profile_idc`]
    pub fn std_profile_idc_raw(&self) -> &StdVideoH265ProfileIdc {
        &self.std_profile_idc
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::std_profile_idc`]
    pub fn set_std_profile_idc_raw(&mut self, value: StdVideoH265ProfileIdc) -> &mut Self {
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
    pub fn std_profile_idc(&self) -> &StdVideoH265ProfileIdc {
        &self.std_profile_idc
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::std_profile_idc`]
    pub fn std_profile_idc_mut(&mut self) -> &mut StdVideoH265ProfileIdc {
        &mut self.std_profile_idc
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
    pub fn set_std_profile_idc(&mut self, value: crate::native::StdVideoH265ProfileIdc) -> &mut Self {
        self.std_profile_idc = value;
        self
    }
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
/// - [`std_reference_info`] is a pointer to a [`StdVideoEncodeH265ReferenceInfo`] structure
///   specifying the syntax and other codec-specific information from the H.265 specification,
///   associated with this reference picture.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_DPB_SLOT_INFO_EXT`
/// - [`p_next`] **must**  be `NULL`
/// - [`std_reference_info`] **must**  be a valid pointer to a valid
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeH265DpbSlotInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`slot_index`] is the [DPB Slot](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#dpb-slot) index for this picture.
    slot_index: i8,
    ///[`std_reference_info`] is a pointer to a
    ///[`StdVideoEncodeH265ReferenceInfo`] structure specifying the syntax and
    ///other codec-specific information from the H.265 specification,
    ///associated with this reference picture.
    std_reference_info: *const StdVideoEncodeH265ReferenceInfo,
}
impl<'lt> Default for VideoEncodeH265DpbSlotInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            slot_index: 0,
            std_reference_info: std::ptr::null(),
        }
    }
}
impl<'lt> VideoEncodeH265DpbSlotInfoEXT<'lt> {
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
    ///Gets the value of [`Self::slot_index`]
    pub fn slot_index(&self) -> i8 {
        self.slot_index
    }
    ///Gets the value of [`Self::std_reference_info`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn std_reference_info(&self) -> &StdVideoEncodeH265ReferenceInfo {
        &*self.std_reference_info
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::slot_index`]
    pub fn slot_index_mut(&mut self) -> &mut i8 {
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
    ///Sets the raw value of [`Self::slot_index`]
    pub fn set_slot_index(&mut self, value: i8) -> &mut Self {
        self.slot_index = value;
        self
    }
    ///Sets the raw value of [`Self::std_reference_info`]
    pub fn set_std_reference_info(&mut self, value: &'lt crate::native::StdVideoEncodeH265ReferenceInfo) -> &mut Self {
        self.std_reference_info = value as *const _;
        self
    }
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
/// - [`reference_list_0_entries`] is a pointer to an array of
///   [`reference_list_0_entry_count`][`VideoEncodeH265DpbSlotInfoEXT`] structures specifying the
///   reference list L0 entries for the current picture.
/// - [`reference_list_1_entry_count`] is the number of reference pictures in reference list L1 and
///   is identical to [`StdVideoEncodeH265SliceSegmentHeader`]`::num_ref_idx_l1_active_minus1` + 1.
/// - [`reference_list_1_entries`] is a pointer to an array of
///   [`reference_list_1_entry_count`][`VideoEncodeH265DpbSlotInfoEXT`] structures specifying the
///   reference list L1 entries for the current picture.
/// - [`reference_modifications`] is a pointer to a [`StdVideoEncodeH265ReferenceModifications`]
///   structure specifying reference list modifications.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_REFERENCE_LISTS_EXT`
/// - [`p_next`] **must**  be `NULL`
/// - If [`reference_list_0_entry_count`] is not `0`, [`reference_list_0_entries`] **must**  be a
///   valid pointer to an array of [`reference_list_0_entry_count`] valid
///   [`VideoEncodeH265DpbSlotInfoEXT`] structures
/// - If [`reference_list_1_entry_count`] is not `0`, [`reference_list_1_entries`] **must**  be a
///   valid pointer to an array of [`reference_list_1_entry_count`] valid
///   [`VideoEncodeH265DpbSlotInfoEXT`] structures
/// - [`reference_modifications`] **must**  be a valid pointer to a valid
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeH265ReferenceListsEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`reference_list_0_entry_count`] is the number of reference pictures in
    ///reference list L0 and is identical to
    ///[`StdVideoEncodeH265SliceSegmentHeader`]::`num_ref_idx_l0_active_minus1`
    ///+ 1.
    reference_list_0_entry_count: u8,
    ///[`reference_list_0_entries`] is a pointer to an array of
    ///[`reference_list_0_entry_count`][`VideoEncodeH265DpbSlotInfoEXT`]
    ///structures specifying the reference list L0 entries for the current
    ///picture.
    reference_list_0_entries: *const VideoEncodeH265DpbSlotInfoEXT<'lt>,
    ///[`reference_list_1_entry_count`] is the number of reference pictures in
    ///reference list L1 and is identical to
    ///[`StdVideoEncodeH265SliceSegmentHeader`]::`num_ref_idx_l1_active_minus1`
    ///+ 1.
    reference_list_1_entry_count: u8,
    ///[`reference_list_1_entries`] is a pointer to an array of
    ///[`reference_list_1_entry_count`][`VideoEncodeH265DpbSlotInfoEXT`]
    ///structures specifying the reference list L1 entries for the current
    ///picture.
    reference_list_1_entries: *const VideoEncodeH265DpbSlotInfoEXT<'lt>,
    ///[`reference_modifications`] is a pointer to a
    ///[`StdVideoEncodeH265ReferenceModifications`] structure specifying
    ///reference list modifications.
    reference_modifications: *const StdVideoEncodeH265ReferenceModifications,
}
impl<'lt> Default for VideoEncodeH265ReferenceListsEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            reference_list_0_entry_count: 0,
            reference_list_0_entries: std::ptr::null(),
            reference_list_1_entry_count: 0,
            reference_list_1_entries: std::ptr::null(),
            reference_modifications: std::ptr::null(),
        }
    }
}
impl<'lt> VideoEncodeH265ReferenceListsEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::reference_list_0_entries`]
    pub fn reference_list_0_entries_raw(&self) -> *const VideoEncodeH265DpbSlotInfoEXT<'lt> {
        self.reference_list_0_entries
    }
    ///Gets the raw value of [`Self::reference_list_1_entries`]
    pub fn reference_list_1_entries_raw(&self) -> *const VideoEncodeH265DpbSlotInfoEXT<'lt> {
        self.reference_list_1_entries
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::reference_list_0_entries`]
    pub fn set_reference_list_0_entries_raw(&mut self, value: *const VideoEncodeH265DpbSlotInfoEXT<'lt>) -> &mut Self {
        self.reference_list_0_entries = value;
        self
    }
    ///Sets the raw value of [`Self::reference_list_1_entries`]
    pub fn set_reference_list_1_entries_raw(&mut self, value: *const VideoEncodeH265DpbSlotInfoEXT<'lt>) -> &mut Self {
        self.reference_list_1_entries = value;
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
    ///Gets the value of [`Self::reference_list_0_entry_count`]
    pub fn reference_list_0_entry_count(&self) -> u8 {
        self.reference_list_0_entry_count
    }
    ///Gets the value of [`Self::reference_list_0_entries`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn reference_list_0_entries(&self) -> &[VideoEncodeH265DpbSlotInfoEXT<'lt>] {
        std::slice::from_raw_parts(
            self.reference_list_0_entries,
            self.reference_list_0_entry_count as usize,
        )
    }
    ///Gets the value of [`Self::reference_list_1_entry_count`]
    pub fn reference_list_1_entry_count(&self) -> u8 {
        self.reference_list_1_entry_count
    }
    ///Gets the value of [`Self::reference_list_1_entries`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn reference_list_1_entries(&self) -> &[VideoEncodeH265DpbSlotInfoEXT<'lt>] {
        std::slice::from_raw_parts(
            self.reference_list_1_entries,
            self.reference_list_1_entry_count as usize,
        )
    }
    ///Gets the value of [`Self::reference_modifications`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn reference_modifications(&self) -> &StdVideoEncodeH265ReferenceModifications {
        &*self.reference_modifications
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::reference_list_0_entry_count`]
    pub fn reference_list_0_entry_count_mut(&mut self) -> &mut u8 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::reference_list_1_entry_count`]
    pub fn reference_list_1_entry_count_mut(&mut self) -> &mut u8 {
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
    ///Sets the raw value of [`Self::reference_list_0_entry_count`]
    pub fn set_reference_list_0_entry_count(&mut self, value: u8) -> &mut Self {
        self.reference_list_0_entry_count = value;
        self
    }
    ///Sets the raw value of [`Self::reference_list_0_entries`]
    pub fn set_reference_list_0_entries(
        &mut self,
        value: &'lt [crate::extensions::ext_video_encode_h_265::VideoEncodeH265DpbSlotInfoEXT<'lt>],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.reference_list_0_entries = value.as_ptr();
        self.reference_list_0_entry_count = len_;
        self
    }
    ///Sets the raw value of [`Self::reference_list_1_entry_count`]
    pub fn set_reference_list_1_entry_count(&mut self, value: u8) -> &mut Self {
        self.reference_list_1_entry_count = value;
        self
    }
    ///Sets the raw value of [`Self::reference_list_1_entries`]
    pub fn set_reference_list_1_entries(
        &mut self,
        value: &'lt [crate::extensions::ext_video_encode_h_265::VideoEncodeH265DpbSlotInfoEXT<'lt>],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.reference_list_1_entries = value.as_ptr();
        self.reference_list_1_entry_count = len_;
        self
    }
    ///Sets the raw value of [`Self::reference_modifications`]
    pub fn set_reference_modifications(
        &mut self,
        value: &'lt crate::native::StdVideoEncodeH265ReferenceModifications,
    ) -> &mut Self {
        self.reference_modifications = value as *const _;
        self
    }
}
