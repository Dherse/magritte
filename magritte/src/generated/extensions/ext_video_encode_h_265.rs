//![VK_EXT_video_encode_h265](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_video_encode_h265.html) - device extension
//!# Description
//!This extension allows applications to compress a raw video sequence by using
//!the H.265/HEVC video compression standard.
//!# Revision
//!6
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_video_encode_queue`]`
//! - **This is a *provisional* extension and  **must**  be used with caution. See the [description](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#boilerplate-provisional-header)
//!   of provisional header files for enablement and stability details.**
//!# Contacts
//! - Ahmed Abdelkhalek [aabdelkh](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_video_encode_h265]
//!   @aabdelkh%0A<<Here describe the issue or question you have about the VK_EXT_video_encode_h265
//!   extension>>)
//!# New structures
//! - [`VideoEncodeH265DpbSlotInfoEXT`]
//! - [`VideoEncodeH265FrameSizeEXT`]
//! - [`VideoEncodeH265NaluSliceSegmentEXT`]
//! - [`VideoEncodeH265QpEXT`]
//! - [`VideoEncodeH265ReferenceListsEXT`]
//! - Extending [`VideoEncodeCapabilitiesKHR`]:  - [`VideoEncodeH265CapabilitiesEXT`]
//! - Extending [`VideoEncodeInfoKHR`]:  - [`VideoEncodeH265EmitPictureParametersEXT`]  -
//!   [`VideoEncodeH265VclFrameInfoEXT`]
//! - Extending [`VideoEncodeRateControlInfoKHR`]:  - [`VideoEncodeH265RateControlInfoEXT`]
//! - Extending [`VideoEncodeRateControlLayerInfoKHR`]:  -
//!   [`VideoEncodeH265RateControlLayerInfoEXT`]
//! - Extending [`VideoProfileKHR`], [`QueryPoolCreateInfo`], [`FormatProperties2`],
//!   [`ImageCreateInfo`], [`ImageViewCreateInfo`], [`BufferCreateInfo`]:  -
//!   [`VideoEncodeH265ProfileEXT`]
//! - Extending [`VideoSessionParametersCreateInfoKHR`]:  -
//!   [`VideoEncodeH265SessionParametersCreateInfoEXT`]
//! - Extending [`VideoSessionParametersUpdateInfoKHR`]:  -
//!   [`VideoEncodeH265SessionParametersAddInfoEXT`]
//!# New enums
//! - [`VideoEncodeH265CapabilityFlagBitsEXT`]
//! - [`VideoEncodeH265CtbSizeFlagBitsEXT`]
//! - [`VideoEncodeH265InputModeFlagBitsEXT`]
//! - [`VideoEncodeH265OutputModeFlagBitsEXT`]
//! - [`VideoEncodeH265RateControlStructureFlagBitsEXT`]
//! - [`VideoEncodeH265TransformBlockSizeFlagBitsEXT`]
//!# New bitmasks
//! - [`VideoEncodeH265CapabilityFlagsEXT`]
//! - [`VideoEncodeH265CtbSizeFlagsEXT`]
//! - [`VideoEncodeH265InputModeFlagsEXT`]
//! - [`VideoEncodeH265OutputModeFlagsEXT`]
//! - [`VideoEncodeH265RateControlStructureFlagsEXT`]
//! - [`VideoEncodeH265TransformBlockSizeFlagsEXT`]
//!# New constants
//! - [`EXT_VIDEO_ENCODE_H265_EXTENSION_NAME`]
//! - [`EXT_VIDEO_ENCODE_H265_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_CAPABILITIES_EXT`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_DPB_SLOT_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_EMIT_PICTURE_PARAMETERS_EXT`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_NALU_SLICE_SEGMENT_EXT`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_PROFILE_EXT`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_RATE_CONTROL_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_RATE_CONTROL_LAYER_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_REFERENCE_LISTS_EXT`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_SESSION_PARAMETERS_ADD_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_SESSION_PARAMETERS_CREATE_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_VCL_FRAME_INFO_EXT`
//! - Extending [`VideoCodecOperationFlagBitsKHR`]:  -
//!   `VK_VIDEO_CODEC_OPERATION_ENCODE_H265_BIT_EXT`
//!# Version History
//! - Revision 0, 2019-11-14 (Ahmed Abdelkhalek)  - Initial draft
//! - Revision 0.5, 2020-02-13 (Tony Zlatinski)  - General Spec cleanup  - Added DPB structures  -
//!   Change the VCL frame encode structure  - Added a common Non-VCL Picture Paramarameters
//!   structure
//! - Revision 2, Oct 10 2021 (Srinath Kumarapuram)  - Vulkan Video Encode h.265 update and spec
//!   edits
//! - Revision 3, 2021-12-08 (Ahmed Abdelkhalek)  - Rate control updates
//! - Revision 4, 2022-01-11 (Ahmed Abdelkhalek)  - Replace occurrences of "slice" by "slice
//!   segment" and rename structures/enums to reflect this.
//! - Revision 5, 2022-02-10 (Ahmed Abdelkhalek)  - Updates to encode capability interface
//! - Revision 6, 2022-03-16 (Ahmed Abdelkhalek)  - Relocate Std header version reporting/requesting
//!   from this extension to VK_KHR_video_queue extension.  - Remove the now empty
//!   VkVideoEncodeH265SessionCreateInfoEXT.
//!# Other info
//! * 2022-03-16
//! * No known IP claims.
//! * - Ahmed Abdelkhalek, AMD  - George Hao, AMD  - Jake Beju, AMD  - Chunbo Chen, Intel  - Ping
//!   Liu, Intel  - Srinath Kumarapuram, NVIDIA  - Tony Zlatinski, NVIDIA
//!# Related
//! - [`VideoEncodeH265CapabilitiesEXT`]
//! - [`VideoEncodeH265CapabilityFlagBitsEXT`]
//! - [`VideoEncodeH265CapabilityFlagsEXT`]
//! - [`VideoEncodeH265CtbSizeFlagBitsEXT`]
//! - [`VideoEncodeH265CtbSizeFlagsEXT`]
//! - [`VideoEncodeH265DpbSlotInfoEXT`]
//! - [`VideoEncodeH265EmitPictureParametersEXT`]
//! - [`VideoEncodeH265FrameSizeEXT`]
//! - [`VideoEncodeH265InputModeFlagBitsEXT`]
//! - [`VideoEncodeH265InputModeFlagsEXT`]
//! - [`VideoEncodeH265NaluSliceSegmentEXT`]
//! - [`VideoEncodeH265OutputModeFlagBitsEXT`]
//! - [`VideoEncodeH265OutputModeFlagsEXT`]
//! - [`VideoEncodeH265ProfileEXT`]
//! - [`VideoEncodeH265QpEXT`]
//! - [`VideoEncodeH265RateControlInfoEXT`]
//! - [`VideoEncodeH265RateControlLayerInfoEXT`]
//! - [`VideoEncodeH265RateControlStructureFlagBitsEXT`]
//! - [`VideoEncodeH265RateControlStructureFlagsEXT`]
//! - [`VideoEncodeH265ReferenceListsEXT`]
//! - [`VideoEncodeH265SessionParametersAddInfoEXT`]
//! - [`VideoEncodeH265SessionParametersCreateInfoEXT`]
//! - [`VideoEncodeH265TransformBlockSizeFlagBitsEXT`]
//! - [`VideoEncodeH265TransformBlockSizeFlagsEXT`]
//! - [`VideoEncodeH265VclFrameInfoEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    native::{
        StdVideoEncodeH265PictureInfo, StdVideoEncodeH265ReferenceInfo, StdVideoEncodeH265ReferenceModifications,
        StdVideoEncodeH265SliceSegmentHeader, StdVideoH265PictureParameterSet, StdVideoH265ProfileIdc,
        StdVideoH265SequenceParameterSet, StdVideoH265VideoParameterSet,
    },
    vulkan1_0::{BaseInStructure, Bool32, StructureType},
};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    ffi::CStr,
    iter::{Extend, FromIterator, IntoIterator},
    marker::PhantomData,
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_VIDEO_ENCODE_H265_SPEC_VERSION")]
pub const EXT_VIDEO_ENCODE_H265_SPEC_VERSION: u32 = 6;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_VIDEO_ENCODE_H265_EXTENSION_NAME")]
pub const EXT_VIDEO_ENCODE_H265_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_video_encode_h265");
///[VkVideoEncodeH265CapabilityFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265CapabilityFlagBitsEXT.html) - Video encode H.265 capability flags
///# C Specifications
///Bits which  **may**  be set in
///[`VideoEncodeH265CapabilitiesEXT::flags`], indicating the encoding
///tools supported, are:
///```c
///// Provided by VK_EXT_video_encode_h265
///typedef enum VkVideoEncodeH265CapabilityFlagBitsEXT {
///    VK_VIDEO_ENCODE_H265_CAPABILITY_SEPARATE_COLOUR_PLANE_BIT_EXT = 0x00000001,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_SCALING_LISTS_BIT_EXT = 0x00000002,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_SAMPLE_ADAPTIVE_OFFSET_ENABLED_BIT_EXT = 0x00000004,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_PCM_ENABLE_BIT_EXT = 0x00000008,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_SPS_TEMPORAL_MVP_ENABLED_BIT_EXT = 0x00000010,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_HRD_COMPLIANCE_BIT_EXT = 0x00000020,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_INIT_QP_MINUS26_BIT_EXT = 0x00000040,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_LOG2_PARALLEL_MERGE_LEVEL_MINUS2_BIT_EXT = 0x00000080,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_SIGN_DATA_HIDING_ENABLED_BIT_EXT = 0x00000100,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_TRANSFORM_SKIP_ENABLED_BIT_EXT = 0x00000200,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT_BIT_EXT = 0x00000400,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_WEIGHTED_PRED_BIT_EXT = 0x00000800,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_WEIGHTED_BIPRED_BIT_EXT = 0x00001000,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_WEIGHTED_PRED_NO_TABLE_BIT_EXT = 0x00002000,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_TRANSQUANT_BYPASS_ENABLED_BIT_EXT = 0x00004000,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_ENTROPY_CODING_SYNC_ENABLED_BIT_EXT = 0x00008000,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_DEBLOCKING_FILTER_OVERRIDE_ENABLED_BIT_EXT = 0x00010000,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_MULTIPLE_TILE_PER_FRAME_BIT_EXT = 0x00020000,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_MULTIPLE_SLICE_PER_TILE_BIT_EXT = 0x00040000,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_MULTIPLE_TILE_PER_SLICE_BIT_EXT = 0x00080000,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_SLICE_SEGMENT_CTB_COUNT_BIT_EXT = 0x00100000,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_ROW_UNALIGNED_SLICE_SEGMENT_BIT_EXT = 0x00200000,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_DEPENDENT_SLICE_SEGMENT_BIT_EXT = 0x00400000,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_DIFFERENT_SLICE_TYPE_BIT_EXT = 0x00800000,
///} VkVideoEncodeH265CapabilityFlagBitsEXT;
///```
///# Description
/// - [`SEPARATE_COLOUR_PLANE`] reports if enabling separate_colour_plane_flag in
///   StdVideoH265SpsFlags is supported.
/// - [`SCALING_LISTS`] reports if enabling scaling_list_enabled_flag and
///   sps_scaling_list_data_present_flag in StdVideoH265SpsFlags, or enabling
///   pps_scaling_list_data_present_flag in StdVideoH265PpsFlags are supproted.
/// - [`SAMPLE_ADAPTIVE_OFFSET_ENABLED`] reports if enabling sample_adaptive_offset_enabled_flag in
///   StdVideoH265SpsFlags is supported.
/// - [`PCM_ENABLE`] reports if enabling pcm_enable_flag in StdVideoH265SpsFlags is supported.
/// - [`SPS_TEMPORAL_MVP_ENABLED`] reports if enabling sps_temporal_mvp_enabled_flag in
///   StdVideoH265SpsFlags is supported.
/// - [`HRD_COMPLIANCE`] reports if the implementation guarantees generating a HRD compliant
///   bitstream if nal_hrd_parameters_present_flag, vcl_hrd_parameters_present_flag, or
///   sub_pic_hrd_params_present_flag are enabled in StdVideoH265HrdFlags, or
///   vui_hrd_parameters_present_flag is enabled in StdVideoH265SpsVuiFlags.
/// - [`INIT_QP_MINUS26`] reports if setting non-zero init_qp_minus26 in
///   StdVideoH265PictureParameterSet is supported.
/// - [`LOG2_PARALLEL_MERGE_LEVEL_MINUS_2`] reports if setting non-zero value for
///   log2_parallel_merge_level_minus2 in StdVideoH265PictureParameterSet is supported.
/// - [`SIGN_DATA_HIDING_ENABLED`] reports if enabling sign_data_hiding_enabled_flag in
///   StdVideoH265PpsFlags is supported.
/// - [`TRANSFORM_SKIP_ENABLED`] reports if enabling transform_skip_enabled_flag in
///   StdVideoH265PpsFlags is supported.
/// - [`PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT`] reports if enabling
///   pps_slice_chroma_qp_offsets_present_flag in StdVideoH265PpsFlags is supported.
/// - [`WEIGHTED_PRED`] reports if enabling weighted_pred_flag in StdVideoH265PpsFlags is supported.
/// - [`WEIGHTED_BIPRED`] reports if enabling weighted_bipred_flag in StdVideoH265PpsFlags is
///   supported.
/// - [`WEIGHTED_PRED_NO_TABLE`] reports that when weighted_pred_flag or weighted_bipred_flag in
///   StdVideoH265PpsFlags are enabled, the implementation is able to internally decide syntax for
///   pred_weight_table.
/// - [`TRANSQUANT_BYPASS_ENABLED`] reports if enabling transquant_bypass_enabled_flag in
///   StdVideoH265PpsFlags is supported.
/// - [`ENTROPY_CODING_SYNC_ENABLED`] reports if enabling entropy_coding_sync_enabled_flag in
///   StdVideoH265PpsFlags is supported.
/// - [`DEBLOCKING_FILTER_OVERRIDE_ENABLED`] reports if enabling
///   deblocking_filter_override_enabled_flag in StdVideoH265PpsFlags is supported.
/// - [`MULTIPLE_TILE_PER_FRAME`] reports if encoding multiple tiles per frame is supported. If not
///   set, the implementation is only able to encode a single tile for each frame.
/// - [`MULTIPLE_SLICE_PER_TILE`] reports if encoding multiple slices per tile is supported. If not
///   set, the implementation is only able to encode a single slice for each tile.
/// - [`MULTIPLE_TILE_PER_SLICE`] reports if encoding multiple tiles per slice is supported. If not
///   set, the implementation is only able to encode a single tile for each slice.
/// - [`SLICE_SEGMENT_CTB_COUNT`] reports support for configuring
///   [`VideoEncodeH265NaluSliceSegmentEXT::ctb_count`] and slice_segment_address in
///   StdVideoEncodeH265SliceSegmentHeader for each slice segment in a frame with multiple slice
///   segments. If not supported, the implementation decides the number of CTBs in each slice
///   segment based on [`VideoEncodeH265VclFrameInfoEXT::nalu_slice_segment_entry_count`].
/// - [`ROW_UNALIGNED_SLICE_SEGMENT`] reports that each slice segment in a frame with a single or
///   multiple tiles per slice may begin or finish at any offset in a CTB row. If not supported, all
///   slice segments in such a frame  **must**  begin at the start of a CTB row (and hence each
///   slice segment  **must**  finish at the end of a CTB row). Also reports that each slice segment
///   in a frame with multiple slices per tile may begin or finish at any offset within the
///   enclosing tile’s CTB row. If not supported, slice segments in such a frame  **must**  begin at
///   the start of the enclosing tile’s CTB row (and hence each slice segment  **must**  finish at
///   the end of the enclosing tile’s CTB row).
/// - [`DEPENDENT_SLICE_SEGMENT`] reports if enabling dependent_slice_segment_flag in
///   StdVideoEncodeH265SliceHeaderFlags is supported.
/// - [`DIFFERENT_SLICE_TYPE`] reports that when [`MULTIPLE_SLICE_PER_TILE`] is supported and a
///   frame is encoded with multiple slices, the implementation allows encoding each slice segment
///   with a different [`StdVideoEncodeH265SliceSegmentHeader`]::slice_type. If not supported, all
///   slice segments of the frame  **must**  be encoded with the same `slice_type` which corresponds
///   to the picture type of the frame. For example, all slice segments of a P-frame would be
///   encoded as P-slices.
///# Related
/// - [`VK_EXT_video_encode_h265`]
/// - [`VideoEncodeH265CapabilityFlagsEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoEncodeH265CapabilityFlagBitsEXT")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoEncodeH265CapabilityFlagBitsEXT(u32);
impl const Default for VideoEncodeH265CapabilityFlagBitsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl VideoEncodeH265CapabilityFlagBitsEXT {
    ///[`SEPARATE_COLOUR_PLANE`]
    ///reports if enabling separate_colour_plane_flag in StdVideoH265SpsFlags
    ///is supported.
    pub const SEPARATE_COLOUR_PLANE: Self = Self(1);
    ///[`SCALING_LISTS`] reports if
    ///enabling scaling_list_enabled_flag and
    ///sps_scaling_list_data_present_flag in StdVideoH265SpsFlags, or enabling
    ///pps_scaling_list_data_present_flag in StdVideoH265PpsFlags are
    ///supproted.
    pub const SCALING_LISTS: Self = Self(2);
    ///[`SAMPLE_ADAPTIVE_OFFSET_ENABLED`]
    ///reports if enabling sample_adaptive_offset_enabled_flag in
    ///StdVideoH265SpsFlags is supported.
    pub const SAMPLE_ADAPTIVE_OFFSET_ENABLED: Self = Self(4);
    ///[`PCM_ENABLE`] reports if
    ///enabling pcm_enable_flag in StdVideoH265SpsFlags is supported.
    pub const PCM_ENABLE: Self = Self(8);
    ///[`SPS_TEMPORAL_MVP_ENABLED`]
    ///reports if enabling sps_temporal_mvp_enabled_flag in
    ///StdVideoH265SpsFlags is supported.
    pub const SPS_TEMPORAL_MVP_ENABLED: Self = Self(16);
    ///[`HRD_COMPLIANCE`] reports if
    ///the implementation guarantees generating a HRD compliant bitstream if
    ///nal_hrd_parameters_present_flag, vcl_hrd_parameters_present_flag, or
    ///sub_pic_hrd_params_present_flag are enabled in StdVideoH265HrdFlags, or
    ///vui_hrd_parameters_present_flag is enabled in StdVideoH265SpsVuiFlags.
    pub const HRD_COMPLIANCE: Self = Self(32);
    ///[`INIT_QP_MINUS26`] reports if
    ///setting non-zero init_qp_minus26 in StdVideoH265PictureParameterSet is
    ///supported.
    pub const INIT_QP_MINUS26: Self = Self(64);
    ///[`LOG2_PARALLEL_MERGE_LEVEL_MINUS_2`]
    ///reports if setting non-zero value for log2_parallel_merge_level_minus2
    ///in StdVideoH265PictureParameterSet is supported.
    pub const LOG2_PARALLEL_MERGE_LEVEL_MINUS_2: Self = Self(128);
    ///[`SIGN_DATA_HIDING_ENABLED`]
    ///reports if enabling sign_data_hiding_enabled_flag in
    ///StdVideoH265PpsFlags is supported.
    pub const SIGN_DATA_HIDING_ENABLED: Self = Self(256);
    ///[`TRANSFORM_SKIP_ENABLED`]
    ///reports if enabling transform_skip_enabled_flag in StdVideoH265PpsFlags
    ///is supported.
    pub const TRANSFORM_SKIP_ENABLED: Self = Self(512);
    ///[`PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT`]
    ///reports if enabling pps_slice_chroma_qp_offsets_present_flag in
    ///StdVideoH265PpsFlags is supported.
    pub const PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT: Self = Self(1024);
    ///[`WEIGHTED_PRED`] reports if
    ///enabling weighted_pred_flag in StdVideoH265PpsFlags is supported.
    pub const WEIGHTED_PRED: Self = Self(2048);
    ///[`WEIGHTED_BIPRED`] reports if
    ///enabling weighted_bipred_flag in StdVideoH265PpsFlags is supported.
    pub const WEIGHTED_BIPRED: Self = Self(4096);
    ///[`WEIGHTED_PRED_NO_TABLE`]
    ///reports that when weighted_pred_flag or weighted_bipred_flag in
    ///StdVideoH265PpsFlags are enabled, the implementation is able to
    ///internally decide syntax for pred_weight_table.
    pub const WEIGHTED_PRED_NO_TABLE: Self = Self(8192);
    ///[`TRANSQUANT_BYPASS_ENABLED`]
    ///reports if enabling transquant_bypass_enabled_flag in
    ///StdVideoH265PpsFlags is supported.
    pub const TRANSQUANT_BYPASS_ENABLED: Self = Self(16384);
    ///[`ENTROPY_CODING_SYNC_ENABLED`]
    ///reports if enabling entropy_coding_sync_enabled_flag in
    ///StdVideoH265PpsFlags is supported.
    pub const ENTROPY_CODING_SYNC_ENABLED: Self = Self(32768);
    ///[`DEBLOCKING_FILTER_OVERRIDE_ENABLED`]
    ///reports if enabling deblocking_filter_override_enabled_flag in
    ///StdVideoH265PpsFlags is supported.
    pub const DEBLOCKING_FILTER_OVERRIDE_ENABLED: Self = Self(65536);
    ///[`MULTIPLE_TILE_PER_FRAME`]
    ///reports if encoding multiple tiles per frame is supported.
    ///If not set, the implementation is only able to encode a single tile for
    ///each frame.
    pub const MULTIPLE_TILE_PER_FRAME: Self = Self(131072);
    ///[`MULTIPLE_SLICE_PER_TILE`]
    ///reports if encoding multiple slices per tile is supported.
    ///If not set, the implementation is only able to encode a single slice for
    ///each tile.
    pub const MULTIPLE_SLICE_PER_TILE: Self = Self(262144);
    ///[`MULTIPLE_TILE_PER_SLICE`]
    ///reports if encoding multiple tiles per slice is supported.
    ///If not set, the implementation is only able to encode a single tile for
    ///each slice.
    pub const MULTIPLE_TILE_PER_SLICE: Self = Self(524288);
    ///[`SLICE_SEGMENT_CTB_COUNT`]
    ///reports support for configuring
    ///[`VideoEncodeH265NaluSliceSegmentEXT`]::`ctbCount` and
    ///slice_segment_address in StdVideoEncodeH265SliceSegmentHeader for each
    ///slice segment in a frame with multiple slice segments.
    ///If not supported, the implementation decides the number of CTBs in each
    ///slice segment based on
    ///[`VideoEncodeH265VclFrameInfoEXT`]::`naluSliceSegmentEntryCount`.
    pub const SLICE_SEGMENT_CTB_COUNT: Self = Self(1048576);
    ///[`ROW_UNALIGNED_SLICE_SEGMENT`]
    ///reports that each slice segment in a frame with a single or multiple
    ///tiles per slice may begin or finish at any offset in a CTB row.
    ///If not supported, all slice segments in such a frame  **must**  begin at the
    ///start of a CTB row (and hence each slice segment  **must**  finish at the end
    ///of a CTB row).
    ///Also reports that each slice segment in a frame with multiple slices per
    ///tile may begin or finish at any offset within the enclosing tile’s CTB
    ///row.
    ///If not supported, slice segments in such a frame  **must**  begin at the
    ///start of the enclosing tile’s CTB row (and hence each slice segment
    /// **must**  finish at the end of the enclosing tile’s CTB row).
    pub const ROW_UNALIGNED_SLICE_SEGMENT: Self = Self(2097152);
    ///[`DEPENDENT_SLICE_SEGMENT`]
    ///reports if enabling dependent_slice_segment_flag in
    ///StdVideoEncodeH265SliceHeaderFlags is supported.
    pub const DEPENDENT_SLICE_SEGMENT: Self = Self(4194304);
    ///[`DIFFERENT_SLICE_TYPE`]
    ///reports that when
    ///[`MULTIPLE_SLICE_PER_TILE`] is
    ///supported and a frame is encoded with multiple slices, the
    ///implementation allows encoding each slice segment with a different
    ///[`StdVideoEncodeH265SliceSegmentHeader`]::slice_type.
    ///If not supported, all slice segments of the frame  **must**  be encoded with
    ///the same `slice_type` which corresponds to the picture type of the
    ///frame.
    ///For example, all slice segments of a P-frame would be encoded as
    ///P-slices.
    pub const DIFFERENT_SLICE_TYPE: Self = Self(8388608);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe.
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
///[VkVideoEncodeH265InputModeFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265InputModeFlagBitsEXT.html) - Video encode H.265 input modes
///# C Specifications
///Bits which  **may**  be set in
///[`VideoEncodeH265CapabilitiesEXT::input_mode_flags`], indicating the
///commmand buffer input granularities supported by the implementation, are:
///```c
///// Provided by VK_EXT_video_encode_h265
///typedef enum VkVideoEncodeH265InputModeFlagBitsEXT {
///    VK_VIDEO_ENCODE_H265_INPUT_MODE_FRAME_BIT_EXT = 0x00000001,
///    VK_VIDEO_ENCODE_H265_INPUT_MODE_SLICE_SEGMENT_BIT_EXT = 0x00000002,
///    VK_VIDEO_ENCODE_H265_INPUT_MODE_NON_VCL_BIT_EXT = 0x00000004,
///} VkVideoEncodeH265InputModeFlagBitsEXT;
///```
///# Description
/// - [`FRAME`] indicates that a single command buffer  **must**  at least encode an entire frame.
///   Any non-VCL NALUs  **must**  be encoded using the same command buffer as the frame if
///   [`NON_VCL`] is not supported.
/// - [`SLICE_SEGMENT`] indicates that a single command buffer  **must**  at least encode a single
///   slice segment. Any non-VCL NALUs  **must**  be encoded using the same command buffer as the
///   first slice segment of the frame if [`NON_VCL`] is not supported.
/// - [`NON_VCL`] indicates that a single command buffer  **may**  encode a non-VCL NALU by itself.
///An implementation  **must**  support at least one of
///[`FRAME`] or
///[`SLICE_SEGMENT`].If [`SLICE_SEGMENT`] is not
///supported, the following two additional restrictions apply for frames
///encoded with multiple slice segments.
///First, all frame slice segments  **must**  have the same pReferenceFinalLists.
///Second, the order in which slice segments appear in
///[`VideoEncodeH265VclFrameInfoEXT::nalu_slice_segment_entries`] or in
///the command buffer  **must**  match the placement order of the slice segments in
///the frame.
///# Related
/// - [`VK_EXT_video_encode_h265`]
/// - [`VideoEncodeH265InputModeFlagsEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoEncodeH265InputModeFlagBitsEXT")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoEncodeH265InputModeFlagBitsEXT(u32);
impl const Default for VideoEncodeH265InputModeFlagBitsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl VideoEncodeH265InputModeFlagBitsEXT {
    ///[`FRAME`] indicates that a
    ///single command buffer  **must**  at least encode an entire frame.
    ///Any non-VCL NALUs  **must**  be encoded using the same command buffer as the
    ///frame if [`NON_VCL`] is not
    ///supported.
    pub const FRAME: Self = Self(1);
    ///[`SLICE_SEGMENT`] indicates
    ///that a single command buffer  **must**  at least encode a single slice
    ///segment.
    ///Any non-VCL NALUs  **must**  be encoded using the same command buffer as the
    ///first slice segment of the frame if
    ///[`NON_VCL`] is not supported.
    pub const SLICE_SEGMENT: Self = Self(2);
    ///[`NON_VCL`] indicates that a
    ///single command buffer  **may**  encode a non-VCL NALU by itself.
    pub const NON_VCL: Self = Self(4);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe.
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
///[VkVideoEncodeH265OutputModeFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265OutputModeFlagBitsEXT.html) - Video encode H.265 output modes
///# C Specifications
///Bits which  **may**  be set in
///[`VideoEncodeH265CapabilitiesEXT::output_mode_flags`], indicating
///the minimum bitstream generation commands that  **must**  be included between
///each [`cmd_begin_video_coding_khr`] and [`cmd_end_video_coding_khr`] pair
///(henceforth simply begin/end pair), are:
///```c
///// Provided by VK_EXT_video_encode_h265
///typedef enum VkVideoEncodeH265OutputModeFlagBitsEXT {
///    VK_VIDEO_ENCODE_H265_OUTPUT_MODE_FRAME_BIT_EXT = 0x00000001,
///    VK_VIDEO_ENCODE_H265_OUTPUT_MODE_SLICE_SEGMENT_BIT_EXT = 0x00000002,
///    VK_VIDEO_ENCODE_H265_OUTPUT_MODE_NON_VCL_BIT_EXT = 0x00000004,
///} VkVideoEncodeH265OutputModeFlagBitsEXT;
///```
///# Description
/// - [`FRAME`] indicates that calls to generate all NALUs of a frame  **must**  be included within
///   a single begin/end pair. Any non-VCL NALUs  **must**  be encoded within the same begin/end
///   pair if [`NON_VCL`] is not supported.
/// - [`SLICE_SEGMENT`] indicates that each begin/end pair  **must**  encode at least one slice
///   segment. Any non-VCL NALUs  **must**  be encoded within the same begin/end pair as the first
///   slice segment of the frame if [`NON_VCL`] is not supported.
/// - [`NON_VCL`] indicates that each begin/end pair  **may**  encode only a non-VCL NALU by itself.
///   An implementation  **must**  support at least one of [`FRAME`] or [`SLICE_SEGMENT`].
///A single begin/end pair  **must**  not encode more than a single frame.The bitstreams of NALUs
/// generated within a single begin/end pair are written
///continuously into the same bitstream buffer (any padding between the NALUs
/// **must**  be compliant to the H.265 standard).The supported input modes  **must**  be coarser or
/// equal to the supported output
///modes.
///For example, it is illegal to report slice segment input is supported but
///only frame output is supported.An implementation  **must**  report one of the following
/// combinations of
///input/output modes:
/// - Input: Frame, Output: Frame
/// - Input: Frame, Output: Frame and Non-VCL
/// - Input: Frame, Output: Slice Segment
/// - Input: Frame, Output: Slice Segment and Non-VCL
/// - Input: Slice Segment, Output: Slice Segment
/// - Input: Slice Segment, Output: Slice Segment and Non-VCL
/// - Input: Frame and Non-VCL, Output: Frame and Non-VCL
/// - Input: Frame and Non-VCL, Output: Slice Segment and Non-VCL
/// - Input: Slice Segment and Non-VCL, Output: Slice Segment and Non-VCL
///# Related
/// - [`VK_EXT_video_encode_h265`]
/// - [`VideoEncodeH265OutputModeFlagsEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoEncodeH265OutputModeFlagBitsEXT")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoEncodeH265OutputModeFlagBitsEXT(u32);
impl const Default for VideoEncodeH265OutputModeFlagBitsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl VideoEncodeH265OutputModeFlagBitsEXT {
    ///[`FRAME`] indicates that
    ///calls to generate all NALUs of a frame  **must**  be included within a single
    ///begin/end pair.
    ///Any non-VCL NALUs  **must**  be encoded within the same begin/end pair if
    ///[`NON_VCL`] is not supported.
    pub const FRAME: Self = Self(1);
    ///[`SLICE_SEGMENT`] indicates
    ///that each begin/end pair  **must**  encode at least one slice segment.
    ///Any non-VCL NALUs  **must**  be encoded within the same begin/end pair as the
    ///first slice segment of the frame if
    ///[`NON_VCL`] is not supported.
    pub const SLICE_SEGMENT: Self = Self(2);
    ///[`NON_VCL`] indicates that
    ///each begin/end pair  **may**  encode only a non-VCL NALU by itself.
    ///An implementation  **must**  support at least one of
    ///[`FRAME`] or
    ///[`SLICE_SEGMENT`].
    pub const NON_VCL: Self = Self(4);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe.
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
///[VkVideoEncodeH265RateControlStructureFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265RateControlStructureFlagBitsEXT.html) - Video encode H.265 rate control structure flags
///# C Specifications
///Possible values of
///[`VideoEncodeH265RateControlInfoEXT::rate_control_structure`],
///specifying a video stream reference structure as a hint for the rate control
///implementation, are:
///```c
///// Provided by VK_EXT_video_encode_h265
///typedef enum VkVideoEncodeH265RateControlStructureFlagBitsEXT {
///    VK_VIDEO_ENCODE_H265_RATE_CONTROL_STRUCTURE_UNKNOWN_EXT = 0,
///    VK_VIDEO_ENCODE_H265_RATE_CONTROL_STRUCTURE_FLAT_BIT_EXT = 0x00000001,
///    VK_VIDEO_ENCODE_H265_RATE_CONTROL_STRUCTURE_DYADIC_BIT_EXT = 0x00000002,
///} VkVideoEncodeH265RateControlStructureFlagBitsEXT;
///```
///# Description
/// - [`UNKNOWN`] is `0`, and specifies a reference structure unknown at the time of stream rate
///   control configuration.
/// - [`FLAT`] specifies a flat reference structure.
/// - [`DYADIC`] specifies a dyadic reference structure.
///# Related
/// - [`VK_EXT_video_encode_h265`]
/// - [`VideoEncodeH265RateControlInfoEXT`]
/// - [`VideoEncodeH265RateControlStructureFlagsEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoEncodeH265RateControlStructureFlagBitsEXT")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoEncodeH265RateControlStructureFlagBitsEXT(u32);
impl const Default for VideoEncodeH265RateControlStructureFlagBitsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl VideoEncodeH265RateControlStructureFlagBitsEXT {
    ///[`UNKNOWN`] is `0`,
    ///and specifies a reference structure unknown at the time of stream rate
    ///control configuration.
    pub const UNKNOWN: Self = Self(0);
    ///[`FLAT`] specifies
    ///a flat reference structure.
    pub const FLAT: Self = Self(1);
    ///[`DYADIC`]
    ///specifies a dyadic reference structure.
    pub const DYADIC: Self = Self(2);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe.
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
///[VkVideoEncodeH265CtbSizeFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265CtbSizeFlagBitsEXT.html) - Supported CTB sizes for H.265 video encode
///# C Specifications
///Bits which  **may**  be set in
///[`VideoEncodeH265CapabilitiesEXT::ctb_sizes`], indicating the CTB
///sizes supported by the implementation, are:
///```c
///// Provided by VK_EXT_video_encode_h265
///typedef enum VkVideoEncodeH265CtbSizeFlagBitsEXT {
///    VK_VIDEO_ENCODE_H265_CTB_SIZE_16_BIT_EXT = 0x00000001,
///    VK_VIDEO_ENCODE_H265_CTB_SIZE_32_BIT_EXT = 0x00000002,
///    VK_VIDEO_ENCODE_H265_CTB_SIZE_64_BIT_EXT = 0x00000004,
///} VkVideoEncodeH265CtbSizeFlagBitsEXT;
///```
///# Description
/// - [`16`] specifies that a CTB size of 16x16 is supported.
/// - [`32`] specifies that a CTB size of 32x32 is supported.
/// - [`64`] specifies that a CTB size of 64x64 is supported.
///# Related
/// - [`VK_EXT_video_encode_h265`]
/// - [`VideoEncodeH265CtbSizeFlagsEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoEncodeH265CtbSizeFlagBitsEXT")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoEncodeH265CtbSizeFlagBitsEXT(u32);
impl const Default for VideoEncodeH265CtbSizeFlagBitsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl VideoEncodeH265CtbSizeFlagBitsEXT {
    ///[`16`] specifies that a CTB size
    ///of 16x16 is supported.
    pub const _16: Self = Self(1);
    ///[`32`] specifies that a CTB size
    ///of 32x32 is supported.
    pub const _32: Self = Self(2);
    ///[`64`] specifies that a CTB size
    ///of 64x64 is supported.
    pub const _64: Self = Self(4);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe.
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
///[VkVideoEncodeH265TransformBlockSizeFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265TransformBlockSizeFlagBitsEXT.html) - Supported transform block sizes for H.265 video encode
///# C Specifications
///Bits which  **may**  be set in
///[`VideoEncodeH265CapabilitiesEXT::transform_block_sizes`],
///indicating the transform block sizes supported by the implementation, are:
///```c
///// Provided by VK_EXT_video_encode_h265
///typedef enum VkVideoEncodeH265TransformBlockSizeFlagBitsEXT {
///    VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_4_BIT_EXT = 0x00000001,
///    VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_8_BIT_EXT = 0x00000002,
///    VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_16_BIT_EXT = 0x00000004,
///    VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_32_BIT_EXT = 0x00000008,
///} VkVideoEncodeH265TransformBlockSizeFlagBitsEXT;
///```
///# Description
/// - [`4`] specifies that a transform block size of 4x4 is supported.
/// - [`8`] specifies that a transform block size of 8x8 is supported.
/// - [`16`] specifies that a transform block size of 16x16 is supported.
/// - [`32`] specifies that a transform block size of 32x32 is supported.
///# Related
/// - [`VK_EXT_video_encode_h265`]
/// - [`VideoEncodeH265TransformBlockSizeFlagsEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoEncodeH265TransformBlockSizeFlagBitsEXT")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoEncodeH265TransformBlockSizeFlagBitsEXT(u32);
impl const Default for VideoEncodeH265TransformBlockSizeFlagBitsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl VideoEncodeH265TransformBlockSizeFlagBitsEXT {
    ///[`4`] specifies that
    ///a transform block size of 4x4 is supported.
    pub const _4: Self = Self(1);
    ///[`8`] specifies that
    ///a transform block size of 8x8 is supported.
    pub const _8: Self = Self(2);
    ///[`16`] specifies
    ///that a transform block size of 16x16 is supported.
    pub const _16: Self = Self(4);
    ///[`32`] specifies
    ///that a transform block size of 32x32 is supported.
    pub const _32: Self = Self(8);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe.
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
///[VkVideoEncodeH265CapabilityFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265CapabilityFlagBitsEXT.html) - Video encode H.265 capability flags
///# C Specifications
///Bits which  **may**  be set in
///[`VideoEncodeH265CapabilitiesEXT::flags`], indicating the encoding
///tools supported, are:
///```c
///// Provided by VK_EXT_video_encode_h265
///typedef enum VkVideoEncodeH265CapabilityFlagBitsEXT {
///    VK_VIDEO_ENCODE_H265_CAPABILITY_SEPARATE_COLOUR_PLANE_BIT_EXT = 0x00000001,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_SCALING_LISTS_BIT_EXT = 0x00000002,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_SAMPLE_ADAPTIVE_OFFSET_ENABLED_BIT_EXT = 0x00000004,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_PCM_ENABLE_BIT_EXT = 0x00000008,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_SPS_TEMPORAL_MVP_ENABLED_BIT_EXT = 0x00000010,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_HRD_COMPLIANCE_BIT_EXT = 0x00000020,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_INIT_QP_MINUS26_BIT_EXT = 0x00000040,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_LOG2_PARALLEL_MERGE_LEVEL_MINUS2_BIT_EXT = 0x00000080,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_SIGN_DATA_HIDING_ENABLED_BIT_EXT = 0x00000100,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_TRANSFORM_SKIP_ENABLED_BIT_EXT = 0x00000200,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT_BIT_EXT = 0x00000400,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_WEIGHTED_PRED_BIT_EXT = 0x00000800,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_WEIGHTED_BIPRED_BIT_EXT = 0x00001000,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_WEIGHTED_PRED_NO_TABLE_BIT_EXT = 0x00002000,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_TRANSQUANT_BYPASS_ENABLED_BIT_EXT = 0x00004000,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_ENTROPY_CODING_SYNC_ENABLED_BIT_EXT = 0x00008000,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_DEBLOCKING_FILTER_OVERRIDE_ENABLED_BIT_EXT = 0x00010000,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_MULTIPLE_TILE_PER_FRAME_BIT_EXT = 0x00020000,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_MULTIPLE_SLICE_PER_TILE_BIT_EXT = 0x00040000,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_MULTIPLE_TILE_PER_SLICE_BIT_EXT = 0x00080000,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_SLICE_SEGMENT_CTB_COUNT_BIT_EXT = 0x00100000,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_ROW_UNALIGNED_SLICE_SEGMENT_BIT_EXT = 0x00200000,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_DEPENDENT_SLICE_SEGMENT_BIT_EXT = 0x00400000,
///    VK_VIDEO_ENCODE_H265_CAPABILITY_DIFFERENT_SLICE_TYPE_BIT_EXT = 0x00800000,
///} VkVideoEncodeH265CapabilityFlagBitsEXT;
///```
///# Description
/// - [`SEPARATE_COLOUR_PLANE`] reports if enabling separate_colour_plane_flag in
///   StdVideoH265SpsFlags is supported.
/// - [`SCALING_LISTS`] reports if enabling scaling_list_enabled_flag and
///   sps_scaling_list_data_present_flag in StdVideoH265SpsFlags, or enabling
///   pps_scaling_list_data_present_flag in StdVideoH265PpsFlags are supproted.
/// - [`SAMPLE_ADAPTIVE_OFFSET_ENABLED`] reports if enabling sample_adaptive_offset_enabled_flag in
///   StdVideoH265SpsFlags is supported.
/// - [`PCM_ENABLE`] reports if enabling pcm_enable_flag in StdVideoH265SpsFlags is supported.
/// - [`SPS_TEMPORAL_MVP_ENABLED`] reports if enabling sps_temporal_mvp_enabled_flag in
///   StdVideoH265SpsFlags is supported.
/// - [`HRD_COMPLIANCE`] reports if the implementation guarantees generating a HRD compliant
///   bitstream if nal_hrd_parameters_present_flag, vcl_hrd_parameters_present_flag, or
///   sub_pic_hrd_params_present_flag are enabled in StdVideoH265HrdFlags, or
///   vui_hrd_parameters_present_flag is enabled in StdVideoH265SpsVuiFlags.
/// - [`INIT_QP_MINUS26`] reports if setting non-zero init_qp_minus26 in
///   StdVideoH265PictureParameterSet is supported.
/// - [`LOG2_PARALLEL_MERGE_LEVEL_MINUS_2`] reports if setting non-zero value for
///   log2_parallel_merge_level_minus2 in StdVideoH265PictureParameterSet is supported.
/// - [`SIGN_DATA_HIDING_ENABLED`] reports if enabling sign_data_hiding_enabled_flag in
///   StdVideoH265PpsFlags is supported.
/// - [`TRANSFORM_SKIP_ENABLED`] reports if enabling transform_skip_enabled_flag in
///   StdVideoH265PpsFlags is supported.
/// - [`PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT`] reports if enabling
///   pps_slice_chroma_qp_offsets_present_flag in StdVideoH265PpsFlags is supported.
/// - [`WEIGHTED_PRED`] reports if enabling weighted_pred_flag in StdVideoH265PpsFlags is supported.
/// - [`WEIGHTED_BIPRED`] reports if enabling weighted_bipred_flag in StdVideoH265PpsFlags is
///   supported.
/// - [`WEIGHTED_PRED_NO_TABLE`] reports that when weighted_pred_flag or weighted_bipred_flag in
///   StdVideoH265PpsFlags are enabled, the implementation is able to internally decide syntax for
///   pred_weight_table.
/// - [`TRANSQUANT_BYPASS_ENABLED`] reports if enabling transquant_bypass_enabled_flag in
///   StdVideoH265PpsFlags is supported.
/// - [`ENTROPY_CODING_SYNC_ENABLED`] reports if enabling entropy_coding_sync_enabled_flag in
///   StdVideoH265PpsFlags is supported.
/// - [`DEBLOCKING_FILTER_OVERRIDE_ENABLED`] reports if enabling
///   deblocking_filter_override_enabled_flag in StdVideoH265PpsFlags is supported.
/// - [`MULTIPLE_TILE_PER_FRAME`] reports if encoding multiple tiles per frame is supported. If not
///   set, the implementation is only able to encode a single tile for each frame.
/// - [`MULTIPLE_SLICE_PER_TILE`] reports if encoding multiple slices per tile is supported. If not
///   set, the implementation is only able to encode a single slice for each tile.
/// - [`MULTIPLE_TILE_PER_SLICE`] reports if encoding multiple tiles per slice is supported. If not
///   set, the implementation is only able to encode a single tile for each slice.
/// - [`SLICE_SEGMENT_CTB_COUNT`] reports support for configuring
///   [`VideoEncodeH265NaluSliceSegmentEXT::ctb_count`] and slice_segment_address in
///   StdVideoEncodeH265SliceSegmentHeader for each slice segment in a frame with multiple slice
///   segments. If not supported, the implementation decides the number of CTBs in each slice
///   segment based on [`VideoEncodeH265VclFrameInfoEXT::nalu_slice_segment_entry_count`].
/// - [`ROW_UNALIGNED_SLICE_SEGMENT`] reports that each slice segment in a frame with a single or
///   multiple tiles per slice may begin or finish at any offset in a CTB row. If not supported, all
///   slice segments in such a frame  **must**  begin at the start of a CTB row (and hence each
///   slice segment  **must**  finish at the end of a CTB row). Also reports that each slice segment
///   in a frame with multiple slices per tile may begin or finish at any offset within the
///   enclosing tile’s CTB row. If not supported, slice segments in such a frame  **must**  begin at
///   the start of the enclosing tile’s CTB row (and hence each slice segment  **must**  finish at
///   the end of the enclosing tile’s CTB row).
/// - [`DEPENDENT_SLICE_SEGMENT`] reports if enabling dependent_slice_segment_flag in
///   StdVideoEncodeH265SliceHeaderFlags is supported.
/// - [`DIFFERENT_SLICE_TYPE`] reports that when [`MULTIPLE_SLICE_PER_TILE`] is supported and a
///   frame is encoded with multiple slices, the implementation allows encoding each slice segment
///   with a different [`StdVideoEncodeH265SliceSegmentHeader`]::slice_type. If not supported, all
///   slice segments of the frame  **must**  be encoded with the same `slice_type` which corresponds
///   to the picture type of the frame. For example, all slice segments of a P-frame would be
///   encoded as P-slices.
///# Related
/// - [`VK_EXT_video_encode_h265`]
/// - [`VideoEncodeH265CapabilityFlagsEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoEncodeH265CapabilityFlagsEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoEncodeH265CapabilityFlagsEXT(u32);
impl const Default for VideoEncodeH265CapabilityFlagsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl From<VideoEncodeH265CapabilityFlagBitsEXT> for VideoEncodeH265CapabilityFlagsEXT {
    fn from(from: VideoEncodeH265CapabilityFlagBitsEXT) -> Self {
        unsafe { Self::from_bits_unchecked(from.bits()) }
    }
}
impl VideoEncodeH265CapabilityFlagsEXT {
    ///[`SEPARATE_COLOUR_PLANE`]
    ///reports if enabling separate_colour_plane_flag in StdVideoH265SpsFlags
    ///is supported.
    pub const SEPARATE_COLOUR_PLANE: Self = Self(1);
    ///[`SCALING_LISTS`] reports if
    ///enabling scaling_list_enabled_flag and
    ///sps_scaling_list_data_present_flag in StdVideoH265SpsFlags, or enabling
    ///pps_scaling_list_data_present_flag in StdVideoH265PpsFlags are
    ///supproted.
    pub const SCALING_LISTS: Self = Self(2);
    ///[`SAMPLE_ADAPTIVE_OFFSET_ENABLED`]
    ///reports if enabling sample_adaptive_offset_enabled_flag in
    ///StdVideoH265SpsFlags is supported.
    pub const SAMPLE_ADAPTIVE_OFFSET_ENABLED: Self = Self(4);
    ///[`PCM_ENABLE`] reports if
    ///enabling pcm_enable_flag in StdVideoH265SpsFlags is supported.
    pub const PCM_ENABLE: Self = Self(8);
    ///[`SPS_TEMPORAL_MVP_ENABLED`]
    ///reports if enabling sps_temporal_mvp_enabled_flag in
    ///StdVideoH265SpsFlags is supported.
    pub const SPS_TEMPORAL_MVP_ENABLED: Self = Self(16);
    ///[`HRD_COMPLIANCE`] reports if
    ///the implementation guarantees generating a HRD compliant bitstream if
    ///nal_hrd_parameters_present_flag, vcl_hrd_parameters_present_flag, or
    ///sub_pic_hrd_params_present_flag are enabled in StdVideoH265HrdFlags, or
    ///vui_hrd_parameters_present_flag is enabled in StdVideoH265SpsVuiFlags.
    pub const HRD_COMPLIANCE: Self = Self(32);
    ///[`INIT_QP_MINUS26`] reports if
    ///setting non-zero init_qp_minus26 in StdVideoH265PictureParameterSet is
    ///supported.
    pub const INIT_QP_MINUS_26: Self = Self(64);
    ///[`LOG2_PARALLEL_MERGE_LEVEL_MINUS_2`]
    ///reports if setting non-zero value for log2_parallel_merge_level_minus2
    ///in StdVideoH265PictureParameterSet is supported.
    pub const LOG_2_PARALLEL_MERGE_LEVEL_MINUS_2: Self = Self(128);
    ///[`SIGN_DATA_HIDING_ENABLED`]
    ///reports if enabling sign_data_hiding_enabled_flag in
    ///StdVideoH265PpsFlags is supported.
    pub const SIGN_DATA_HIDING_ENABLED: Self = Self(256);
    ///[`TRANSFORM_SKIP_ENABLED`]
    ///reports if enabling transform_skip_enabled_flag in StdVideoH265PpsFlags
    ///is supported.
    pub const TRANSFORM_SKIP_ENABLED: Self = Self(512);
    ///[`PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT`]
    ///reports if enabling pps_slice_chroma_qp_offsets_present_flag in
    ///StdVideoH265PpsFlags is supported.
    pub const PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT: Self = Self(1024);
    ///[`WEIGHTED_PRED`] reports if
    ///enabling weighted_pred_flag in StdVideoH265PpsFlags is supported.
    pub const WEIGHTED_PRED: Self = Self(2048);
    ///[`WEIGHTED_BIPRED`] reports if
    ///enabling weighted_bipred_flag in StdVideoH265PpsFlags is supported.
    pub const WEIGHTED_BIPRED: Self = Self(4096);
    ///[`WEIGHTED_PRED_NO_TABLE`]
    ///reports that when weighted_pred_flag or weighted_bipred_flag in
    ///StdVideoH265PpsFlags are enabled, the implementation is able to
    ///internally decide syntax for pred_weight_table.
    pub const WEIGHTED_PRED_NO_TABLE: Self = Self(8192);
    ///[`TRANSQUANT_BYPASS_ENABLED`]
    ///reports if enabling transquant_bypass_enabled_flag in
    ///StdVideoH265PpsFlags is supported.
    pub const TRANSQUANT_BYPASS_ENABLED: Self = Self(16384);
    ///[`ENTROPY_CODING_SYNC_ENABLED`]
    ///reports if enabling entropy_coding_sync_enabled_flag in
    ///StdVideoH265PpsFlags is supported.
    pub const ENTROPY_CODING_SYNC_ENABLED: Self = Self(32768);
    ///[`DEBLOCKING_FILTER_OVERRIDE_ENABLED`]
    ///reports if enabling deblocking_filter_override_enabled_flag in
    ///StdVideoH265PpsFlags is supported.
    pub const DEBLOCKING_FILTER_OVERRIDE_ENABLED: Self = Self(65536);
    ///[`MULTIPLE_TILE_PER_FRAME`]
    ///reports if encoding multiple tiles per frame is supported.
    ///If not set, the implementation is only able to encode a single tile for
    ///each frame.
    pub const MULTIPLE_TILE_PER_FRAME: Self = Self(131072);
    ///[`MULTIPLE_SLICE_PER_TILE`]
    ///reports if encoding multiple slices per tile is supported.
    ///If not set, the implementation is only able to encode a single slice for
    ///each tile.
    pub const MULTIPLE_SLICE_PER_TILE: Self = Self(262144);
    ///[`MULTIPLE_TILE_PER_SLICE`]
    ///reports if encoding multiple tiles per slice is supported.
    ///If not set, the implementation is only able to encode a single tile for
    ///each slice.
    pub const MULTIPLE_TILE_PER_SLICE: Self = Self(524288);
    ///[`SLICE_SEGMENT_CTB_COUNT`]
    ///reports support for configuring
    ///[`VideoEncodeH265NaluSliceSegmentEXT`]::`ctbCount` and
    ///slice_segment_address in StdVideoEncodeH265SliceSegmentHeader for each
    ///slice segment in a frame with multiple slice segments.
    ///If not supported, the implementation decides the number of CTBs in each
    ///slice segment based on
    ///[`VideoEncodeH265VclFrameInfoEXT`]::`naluSliceSegmentEntryCount`.
    pub const SLICE_SEGMENT_CTB_COUNT: Self = Self(1048576);
    ///[`ROW_UNALIGNED_SLICE_SEGMENT`]
    ///reports that each slice segment in a frame with a single or multiple
    ///tiles per slice may begin or finish at any offset in a CTB row.
    ///If not supported, all slice segments in such a frame  **must**  begin at the
    ///start of a CTB row (and hence each slice segment  **must**  finish at the end
    ///of a CTB row).
    ///Also reports that each slice segment in a frame with multiple slices per
    ///tile may begin or finish at any offset within the enclosing tile’s CTB
    ///row.
    ///If not supported, slice segments in such a frame  **must**  begin at the
    ///start of the enclosing tile’s CTB row (and hence each slice segment
    /// **must**  finish at the end of the enclosing tile’s CTB row).
    pub const ROW_UNALIGNED_SLICE_SEGMENT: Self = Self(2097152);
    ///[`DEPENDENT_SLICE_SEGMENT`]
    ///reports if enabling dependent_slice_segment_flag in
    ///StdVideoEncodeH265SliceHeaderFlags is supported.
    pub const DEPENDENT_SLICE_SEGMENT: Self = Self(4194304);
    ///[`DIFFERENT_SLICE_TYPE`]
    ///reports that when
    ///[`MULTIPLE_SLICE_PER_TILE`] is
    ///supported and a frame is encoded with multiple slices, the
    ///implementation allows encoding each slice segment with a different
    ///[`StdVideoEncodeH265SliceSegmentHeader`]::slice_type.
    ///If not supported, all slice segments of the frame  **must**  be encoded with
    ///the same `slice_type` which corresponds to the picture type of the
    ///frame.
    ///For example, all slice segments of a P-frame would be encoded as
    ///P-slices.
    pub const DIFFERENT_SLICE_TYPE: Self = Self(8388608);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        {
            all |= Self::SEPARATE_COLOUR_PLANE;
        }
        {
            all |= Self::SCALING_LISTS;
        }
        {
            all |= Self::SAMPLE_ADAPTIVE_OFFSET_ENABLED;
        }
        {
            all |= Self::PCM_ENABLE;
        }
        {
            all |= Self::SPS_TEMPORAL_MVP_ENABLED;
        }
        {
            all |= Self::HRD_COMPLIANCE;
        }
        {
            all |= Self::INIT_QP_MINUS_26;
        }
        {
            all |= Self::LOG_2_PARALLEL_MERGE_LEVEL_MINUS_2;
        }
        {
            all |= Self::SIGN_DATA_HIDING_ENABLED;
        }
        {
            all |= Self::TRANSFORM_SKIP_ENABLED;
        }
        {
            all |= Self::PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT;
        }
        {
            all |= Self::WEIGHTED_PRED;
        }
        {
            all |= Self::WEIGHTED_BIPRED;
        }
        {
            all |= Self::WEIGHTED_PRED_NO_TABLE;
        }
        {
            all |= Self::TRANSQUANT_BYPASS_ENABLED;
        }
        {
            all |= Self::ENTROPY_CODING_SYNC_ENABLED;
        }
        {
            all |= Self::DEBLOCKING_FILTER_OVERRIDE_ENABLED;
        }
        {
            all |= Self::MULTIPLE_TILE_PER_FRAME;
        }
        {
            all |= Self::MULTIPLE_SLICE_PER_TILE;
        }
        {
            all |= Self::MULTIPLE_TILE_PER_SLICE;
        }
        {
            all |= Self::SLICE_SEGMENT_CTB_COUNT;
        }
        {
            all |= Self::ROW_UNALIGNED_SLICE_SEGMENT;
        }
        {
            all |= Self::DEPENDENT_SLICE_SEGMENT;
        }
        {
            all |= Self::DIFFERENT_SLICE_TYPE;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for VideoEncodeH265CapabilityFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for VideoEncodeH265CapabilityFlagsEXT {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for VideoEncodeH265CapabilityFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for VideoEncodeH265CapabilityFlagsEXT {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for VideoEncodeH265CapabilityFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for VideoEncodeH265CapabilityFlagsEXT {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for VideoEncodeH265CapabilityFlagsEXT {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for VideoEncodeH265CapabilityFlagsEXT {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for VideoEncodeH265CapabilityFlagsEXT {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<VideoEncodeH265CapabilityFlagsEXT> for VideoEncodeH265CapabilityFlagsEXT {
    fn extend<T: IntoIterator<Item = VideoEncodeH265CapabilityFlagsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<VideoEncodeH265CapabilityFlagBitsEXT> for VideoEncodeH265CapabilityFlagsEXT {
    fn extend<T: IntoIterator<Item = VideoEncodeH265CapabilityFlagBitsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<VideoEncodeH265CapabilityFlagBitsEXT>>::from(i));
        }
    }
}
impl FromIterator<VideoEncodeH265CapabilityFlagsEXT> for VideoEncodeH265CapabilityFlagsEXT {
    fn from_iter<T: IntoIterator<Item = VideoEncodeH265CapabilityFlagsEXT>>(
        iterator: T,
    ) -> VideoEncodeH265CapabilityFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeH265CapabilityFlagsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<VideoEncodeH265CapabilityFlagBitsEXT> for VideoEncodeH265CapabilityFlagsEXT {
    fn from_iter<T: IntoIterator<Item = VideoEncodeH265CapabilityFlagBitsEXT>>(
        iterator: T,
    ) -> VideoEncodeH265CapabilityFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeH265CapabilityFlagBitsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for VideoEncodeH265CapabilityFlagsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoEncodeH265CapabilityFlagsEXT);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoEncodeH265CapabilityFlagsEXT::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self
                        .0
                        .contains(VideoEncodeH265CapabilityFlagsEXT::SEPARATE_COLOUR_PLANE)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(SEPARATE_COLOUR_PLANE))?;
                    }
                    if self.0.contains(VideoEncodeH265CapabilityFlagsEXT::SCALING_LISTS) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(SCALING_LISTS))?;
                    }
                    if self
                        .0
                        .contains(VideoEncodeH265CapabilityFlagsEXT::SAMPLE_ADAPTIVE_OFFSET_ENABLED)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(SAMPLE_ADAPTIVE_OFFSET_ENABLED))?;
                    }
                    if self.0.contains(VideoEncodeH265CapabilityFlagsEXT::PCM_ENABLE) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(PCM_ENABLE))?;
                    }
                    if self
                        .0
                        .contains(VideoEncodeH265CapabilityFlagsEXT::SPS_TEMPORAL_MVP_ENABLED)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(SPS_TEMPORAL_MVP_ENABLED))?;
                    }
                    if self.0.contains(VideoEncodeH265CapabilityFlagsEXT::HRD_COMPLIANCE) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(HRD_COMPLIANCE))?;
                    }
                    if self.0.contains(VideoEncodeH265CapabilityFlagsEXT::INIT_QP_MINUS_26) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(INIT_QP_MINUS_26))?;
                    }
                    if self
                        .0
                        .contains(VideoEncodeH265CapabilityFlagsEXT::LOG_2_PARALLEL_MERGE_LEVEL_MINUS_2)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(LOG_2_PARALLEL_MERGE_LEVEL_MINUS_2))?;
                    }
                    if self
                        .0
                        .contains(VideoEncodeH265CapabilityFlagsEXT::SIGN_DATA_HIDING_ENABLED)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(SIGN_DATA_HIDING_ENABLED))?;
                    }
                    if self
                        .0
                        .contains(VideoEncodeH265CapabilityFlagsEXT::TRANSFORM_SKIP_ENABLED)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(TRANSFORM_SKIP_ENABLED))?;
                    }
                    if self
                        .0
                        .contains(VideoEncodeH265CapabilityFlagsEXT::PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT))?;
                    }
                    if self.0.contains(VideoEncodeH265CapabilityFlagsEXT::WEIGHTED_PRED) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(WEIGHTED_PRED))?;
                    }
                    if self.0.contains(VideoEncodeH265CapabilityFlagsEXT::WEIGHTED_BIPRED) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(WEIGHTED_BIPRED))?;
                    }
                    if self
                        .0
                        .contains(VideoEncodeH265CapabilityFlagsEXT::WEIGHTED_PRED_NO_TABLE)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(WEIGHTED_PRED_NO_TABLE))?;
                    }
                    if self
                        .0
                        .contains(VideoEncodeH265CapabilityFlagsEXT::TRANSQUANT_BYPASS_ENABLED)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(TRANSQUANT_BYPASS_ENABLED))?;
                    }
                    if self
                        .0
                        .contains(VideoEncodeH265CapabilityFlagsEXT::ENTROPY_CODING_SYNC_ENABLED)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(ENTROPY_CODING_SYNC_ENABLED))?;
                    }
                    if self
                        .0
                        .contains(VideoEncodeH265CapabilityFlagsEXT::DEBLOCKING_FILTER_OVERRIDE_ENABLED)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(DEBLOCKING_FILTER_OVERRIDE_ENABLED))?;
                    }
                    if self
                        .0
                        .contains(VideoEncodeH265CapabilityFlagsEXT::MULTIPLE_TILE_PER_FRAME)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(MULTIPLE_TILE_PER_FRAME))?;
                    }
                    if self
                        .0
                        .contains(VideoEncodeH265CapabilityFlagsEXT::MULTIPLE_SLICE_PER_TILE)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(MULTIPLE_SLICE_PER_TILE))?;
                    }
                    if self
                        .0
                        .contains(VideoEncodeH265CapabilityFlagsEXT::MULTIPLE_TILE_PER_SLICE)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(MULTIPLE_TILE_PER_SLICE))?;
                    }
                    if self
                        .0
                        .contains(VideoEncodeH265CapabilityFlagsEXT::SLICE_SEGMENT_CTB_COUNT)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(SLICE_SEGMENT_CTB_COUNT))?;
                    }
                    if self
                        .0
                        .contains(VideoEncodeH265CapabilityFlagsEXT::ROW_UNALIGNED_SLICE_SEGMENT)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(ROW_UNALIGNED_SLICE_SEGMENT))?;
                    }
                    if self
                        .0
                        .contains(VideoEncodeH265CapabilityFlagsEXT::DEPENDENT_SLICE_SEGMENT)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(DEPENDENT_SLICE_SEGMENT))?;
                    }
                    if self.0.contains(VideoEncodeH265CapabilityFlagsEXT::DIFFERENT_SLICE_TYPE) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(DIFFERENT_SLICE_TYPE))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoEncodeH265CapabilityFlagsEXT))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoEncodeH265InputModeFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265InputModeFlagBitsEXT.html) - Video encode H.265 input modes
///# C Specifications
///Bits which  **may**  be set in
///[`VideoEncodeH265CapabilitiesEXT::input_mode_flags`], indicating the
///commmand buffer input granularities supported by the implementation, are:
///```c
///// Provided by VK_EXT_video_encode_h265
///typedef enum VkVideoEncodeH265InputModeFlagBitsEXT {
///    VK_VIDEO_ENCODE_H265_INPUT_MODE_FRAME_BIT_EXT = 0x00000001,
///    VK_VIDEO_ENCODE_H265_INPUT_MODE_SLICE_SEGMENT_BIT_EXT = 0x00000002,
///    VK_VIDEO_ENCODE_H265_INPUT_MODE_NON_VCL_BIT_EXT = 0x00000004,
///} VkVideoEncodeH265InputModeFlagBitsEXT;
///```
///# Description
/// - [`FRAME`] indicates that a single command buffer  **must**  at least encode an entire frame.
///   Any non-VCL NALUs  **must**  be encoded using the same command buffer as the frame if
///   [`NON_VCL`] is not supported.
/// - [`SLICE_SEGMENT`] indicates that a single command buffer  **must**  at least encode a single
///   slice segment. Any non-VCL NALUs  **must**  be encoded using the same command buffer as the
///   first slice segment of the frame if [`NON_VCL`] is not supported.
/// - [`NON_VCL`] indicates that a single command buffer  **may**  encode a non-VCL NALU by itself.
///An implementation  **must**  support at least one of
///[`FRAME`] or
///[`SLICE_SEGMENT`].If [`SLICE_SEGMENT`] is not
///supported, the following two additional restrictions apply for frames
///encoded with multiple slice segments.
///First, all frame slice segments  **must**  have the same pReferenceFinalLists.
///Second, the order in which slice segments appear in
///[`VideoEncodeH265VclFrameInfoEXT::nalu_slice_segment_entries`] or in
///the command buffer  **must**  match the placement order of the slice segments in
///the frame.
///# Related
/// - [`VK_EXT_video_encode_h265`]
/// - [`VideoEncodeH265InputModeFlagsEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoEncodeH265InputModeFlagsEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoEncodeH265InputModeFlagsEXT(u32);
impl const Default for VideoEncodeH265InputModeFlagsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl From<VideoEncodeH265InputModeFlagBitsEXT> for VideoEncodeH265InputModeFlagsEXT {
    fn from(from: VideoEncodeH265InputModeFlagBitsEXT) -> Self {
        unsafe { Self::from_bits_unchecked(from.bits()) }
    }
}
impl VideoEncodeH265InputModeFlagsEXT {
    ///[`FRAME`] indicates that a
    ///single command buffer  **must**  at least encode an entire frame.
    ///Any non-VCL NALUs  **must**  be encoded using the same command buffer as the
    ///frame if [`NON_VCL`] is not
    ///supported.
    pub const FRAME: Self = Self(1);
    ///[`SLICE_SEGMENT`] indicates
    ///that a single command buffer  **must**  at least encode a single slice
    ///segment.
    ///Any non-VCL NALUs  **must**  be encoded using the same command buffer as the
    ///first slice segment of the frame if
    ///[`NON_VCL`] is not supported.
    pub const SLICE_SEGMENT: Self = Self(2);
    ///[`NON_VCL`] indicates that a
    ///single command buffer  **may**  encode a non-VCL NALU by itself.
    pub const NON_VCL: Self = Self(4);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        {
            all |= Self::FRAME;
        }
        {
            all |= Self::SLICE_SEGMENT;
        }
        {
            all |= Self::NON_VCL;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for VideoEncodeH265InputModeFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for VideoEncodeH265InputModeFlagsEXT {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for VideoEncodeH265InputModeFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for VideoEncodeH265InputModeFlagsEXT {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for VideoEncodeH265InputModeFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for VideoEncodeH265InputModeFlagsEXT {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for VideoEncodeH265InputModeFlagsEXT {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for VideoEncodeH265InputModeFlagsEXT {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for VideoEncodeH265InputModeFlagsEXT {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<VideoEncodeH265InputModeFlagsEXT> for VideoEncodeH265InputModeFlagsEXT {
    fn extend<T: IntoIterator<Item = VideoEncodeH265InputModeFlagsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<VideoEncodeH265InputModeFlagBitsEXT> for VideoEncodeH265InputModeFlagsEXT {
    fn extend<T: IntoIterator<Item = VideoEncodeH265InputModeFlagBitsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<VideoEncodeH265InputModeFlagBitsEXT>>::from(i));
        }
    }
}
impl FromIterator<VideoEncodeH265InputModeFlagsEXT> for VideoEncodeH265InputModeFlagsEXT {
    fn from_iter<T: IntoIterator<Item = VideoEncodeH265InputModeFlagsEXT>>(
        iterator: T,
    ) -> VideoEncodeH265InputModeFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeH265InputModeFlagsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<VideoEncodeH265InputModeFlagBitsEXT> for VideoEncodeH265InputModeFlagsEXT {
    fn from_iter<T: IntoIterator<Item = VideoEncodeH265InputModeFlagBitsEXT>>(
        iterator: T,
    ) -> VideoEncodeH265InputModeFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeH265InputModeFlagBitsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for VideoEncodeH265InputModeFlagsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoEncodeH265InputModeFlagsEXT);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoEncodeH265InputModeFlagsEXT::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(VideoEncodeH265InputModeFlagsEXT::FRAME) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(FRAME))?;
                    }
                    if self.0.contains(VideoEncodeH265InputModeFlagsEXT::SLICE_SEGMENT) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(SLICE_SEGMENT))?;
                    }
                    if self.0.contains(VideoEncodeH265InputModeFlagsEXT::NON_VCL) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(NON_VCL))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoEncodeH265InputModeFlagsEXT))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoEncodeH265OutputModeFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265OutputModeFlagBitsEXT.html) - Video encode H.265 output modes
///# C Specifications
///Bits which  **may**  be set in
///[`VideoEncodeH265CapabilitiesEXT::output_mode_flags`], indicating
///the minimum bitstream generation commands that  **must**  be included between
///each [`cmd_begin_video_coding_khr`] and [`cmd_end_video_coding_khr`] pair
///(henceforth simply begin/end pair), are:
///```c
///// Provided by VK_EXT_video_encode_h265
///typedef enum VkVideoEncodeH265OutputModeFlagBitsEXT {
///    VK_VIDEO_ENCODE_H265_OUTPUT_MODE_FRAME_BIT_EXT = 0x00000001,
///    VK_VIDEO_ENCODE_H265_OUTPUT_MODE_SLICE_SEGMENT_BIT_EXT = 0x00000002,
///    VK_VIDEO_ENCODE_H265_OUTPUT_MODE_NON_VCL_BIT_EXT = 0x00000004,
///} VkVideoEncodeH265OutputModeFlagBitsEXT;
///```
///# Description
/// - [`FRAME`] indicates that calls to generate all NALUs of a frame  **must**  be included within
///   a single begin/end pair. Any non-VCL NALUs  **must**  be encoded within the same begin/end
///   pair if [`NON_VCL`] is not supported.
/// - [`SLICE_SEGMENT`] indicates that each begin/end pair  **must**  encode at least one slice
///   segment. Any non-VCL NALUs  **must**  be encoded within the same begin/end pair as the first
///   slice segment of the frame if [`NON_VCL`] is not supported.
/// - [`NON_VCL`] indicates that each begin/end pair  **may**  encode only a non-VCL NALU by itself.
///   An implementation  **must**  support at least one of [`FRAME`] or [`SLICE_SEGMENT`].
///A single begin/end pair  **must**  not encode more than a single frame.The bitstreams of NALUs
/// generated within a single begin/end pair are written
///continuously into the same bitstream buffer (any padding between the NALUs
/// **must**  be compliant to the H.265 standard).The supported input modes  **must**  be coarser or
/// equal to the supported output
///modes.
///For example, it is illegal to report slice segment input is supported but
///only frame output is supported.An implementation  **must**  report one of the following
/// combinations of
///input/output modes:
/// - Input: Frame, Output: Frame
/// - Input: Frame, Output: Frame and Non-VCL
/// - Input: Frame, Output: Slice Segment
/// - Input: Frame, Output: Slice Segment and Non-VCL
/// - Input: Slice Segment, Output: Slice Segment
/// - Input: Slice Segment, Output: Slice Segment and Non-VCL
/// - Input: Frame and Non-VCL, Output: Frame and Non-VCL
/// - Input: Frame and Non-VCL, Output: Slice Segment and Non-VCL
/// - Input: Slice Segment and Non-VCL, Output: Slice Segment and Non-VCL
///# Related
/// - [`VK_EXT_video_encode_h265`]
/// - [`VideoEncodeH265OutputModeFlagsEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoEncodeH265OutputModeFlagsEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoEncodeH265OutputModeFlagsEXT(u32);
impl const Default for VideoEncodeH265OutputModeFlagsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl From<VideoEncodeH265OutputModeFlagBitsEXT> for VideoEncodeH265OutputModeFlagsEXT {
    fn from(from: VideoEncodeH265OutputModeFlagBitsEXT) -> Self {
        unsafe { Self::from_bits_unchecked(from.bits()) }
    }
}
impl VideoEncodeH265OutputModeFlagsEXT {
    ///[`FRAME`] indicates that
    ///calls to generate all NALUs of a frame  **must**  be included within a single
    ///begin/end pair.
    ///Any non-VCL NALUs  **must**  be encoded within the same begin/end pair if
    ///[`NON_VCL`] is not supported.
    pub const FRAME: Self = Self(1);
    ///[`SLICE_SEGMENT`] indicates
    ///that each begin/end pair  **must**  encode at least one slice segment.
    ///Any non-VCL NALUs  **must**  be encoded within the same begin/end pair as the
    ///first slice segment of the frame if
    ///[`NON_VCL`] is not supported.
    pub const SLICE_SEGMENT: Self = Self(2);
    ///[`NON_VCL`] indicates that
    ///each begin/end pair  **may**  encode only a non-VCL NALU by itself.
    ///An implementation  **must**  support at least one of
    ///[`FRAME`] or
    ///[`SLICE_SEGMENT`].
    pub const NON_VCL: Self = Self(4);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        {
            all |= Self::FRAME;
        }
        {
            all |= Self::SLICE_SEGMENT;
        }
        {
            all |= Self::NON_VCL;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for VideoEncodeH265OutputModeFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for VideoEncodeH265OutputModeFlagsEXT {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for VideoEncodeH265OutputModeFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for VideoEncodeH265OutputModeFlagsEXT {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for VideoEncodeH265OutputModeFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for VideoEncodeH265OutputModeFlagsEXT {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for VideoEncodeH265OutputModeFlagsEXT {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for VideoEncodeH265OutputModeFlagsEXT {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for VideoEncodeH265OutputModeFlagsEXT {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<VideoEncodeH265OutputModeFlagsEXT> for VideoEncodeH265OutputModeFlagsEXT {
    fn extend<T: IntoIterator<Item = VideoEncodeH265OutputModeFlagsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<VideoEncodeH265OutputModeFlagBitsEXT> for VideoEncodeH265OutputModeFlagsEXT {
    fn extend<T: IntoIterator<Item = VideoEncodeH265OutputModeFlagBitsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<VideoEncodeH265OutputModeFlagBitsEXT>>::from(i));
        }
    }
}
impl FromIterator<VideoEncodeH265OutputModeFlagsEXT> for VideoEncodeH265OutputModeFlagsEXT {
    fn from_iter<T: IntoIterator<Item = VideoEncodeH265OutputModeFlagsEXT>>(
        iterator: T,
    ) -> VideoEncodeH265OutputModeFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeH265OutputModeFlagsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<VideoEncodeH265OutputModeFlagBitsEXT> for VideoEncodeH265OutputModeFlagsEXT {
    fn from_iter<T: IntoIterator<Item = VideoEncodeH265OutputModeFlagBitsEXT>>(
        iterator: T,
    ) -> VideoEncodeH265OutputModeFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeH265OutputModeFlagBitsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for VideoEncodeH265OutputModeFlagsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoEncodeH265OutputModeFlagsEXT);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoEncodeH265OutputModeFlagsEXT::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(VideoEncodeH265OutputModeFlagsEXT::FRAME) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(FRAME))?;
                    }
                    if self.0.contains(VideoEncodeH265OutputModeFlagsEXT::SLICE_SEGMENT) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(SLICE_SEGMENT))?;
                    }
                    if self.0.contains(VideoEncodeH265OutputModeFlagsEXT::NON_VCL) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(NON_VCL))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoEncodeH265OutputModeFlagsEXT))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoEncodeH265RateControlStructureFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265RateControlStructureFlagBitsEXT.html) - Video encode H.265 rate control structure flags
///# C Specifications
///Possible values of
///[`VideoEncodeH265RateControlInfoEXT::rate_control_structure`],
///specifying a video stream reference structure as a hint for the rate control
///implementation, are:
///```c
///// Provided by VK_EXT_video_encode_h265
///typedef enum VkVideoEncodeH265RateControlStructureFlagBitsEXT {
///    VK_VIDEO_ENCODE_H265_RATE_CONTROL_STRUCTURE_UNKNOWN_EXT = 0,
///    VK_VIDEO_ENCODE_H265_RATE_CONTROL_STRUCTURE_FLAT_BIT_EXT = 0x00000001,
///    VK_VIDEO_ENCODE_H265_RATE_CONTROL_STRUCTURE_DYADIC_BIT_EXT = 0x00000002,
///} VkVideoEncodeH265RateControlStructureFlagBitsEXT;
///```
///# Description
/// - [`UNKNOWN`] is `0`, and specifies a reference structure unknown at the time of stream rate
///   control configuration.
/// - [`FLAT`] specifies a flat reference structure.
/// - [`DYADIC`] specifies a dyadic reference structure.
///# Related
/// - [`VK_EXT_video_encode_h265`]
/// - [`VideoEncodeH265RateControlInfoEXT`]
/// - [`VideoEncodeH265RateControlStructureFlagsEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoEncodeH265RateControlStructureFlagsEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoEncodeH265RateControlStructureFlagsEXT(u32);
impl const Default for VideoEncodeH265RateControlStructureFlagsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl From<VideoEncodeH265RateControlStructureFlagBitsEXT> for VideoEncodeH265RateControlStructureFlagsEXT {
    fn from(from: VideoEncodeH265RateControlStructureFlagBitsEXT) -> Self {
        unsafe { Self::from_bits_unchecked(from.bits()) }
    }
}
impl VideoEncodeH265RateControlStructureFlagsEXT {
    ///[`UNKNOWN`] is `0`,
    ///and specifies a reference structure unknown at the time of stream rate
    ///control configuration.
    pub const UNKNOWN: Self = Self(0);
    ///[`FLAT`] specifies
    ///a flat reference structure.
    pub const FLAT: Self = Self(1);
    ///[`DYADIC`]
    ///specifies a dyadic reference structure.
    pub const DYADIC: Self = Self(2);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        {
            all |= Self::UNKNOWN;
        }
        {
            all |= Self::FLAT;
        }
        {
            all |= Self::DYADIC;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for VideoEncodeH265RateControlStructureFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for VideoEncodeH265RateControlStructureFlagsEXT {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for VideoEncodeH265RateControlStructureFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for VideoEncodeH265RateControlStructureFlagsEXT {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for VideoEncodeH265RateControlStructureFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for VideoEncodeH265RateControlStructureFlagsEXT {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for VideoEncodeH265RateControlStructureFlagsEXT {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for VideoEncodeH265RateControlStructureFlagsEXT {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for VideoEncodeH265RateControlStructureFlagsEXT {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<VideoEncodeH265RateControlStructureFlagsEXT> for VideoEncodeH265RateControlStructureFlagsEXT {
    fn extend<T: IntoIterator<Item = VideoEncodeH265RateControlStructureFlagsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<VideoEncodeH265RateControlStructureFlagBitsEXT> for VideoEncodeH265RateControlStructureFlagsEXT {
    fn extend<T: IntoIterator<Item = VideoEncodeH265RateControlStructureFlagBitsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(
                self,
                <Self as From<VideoEncodeH265RateControlStructureFlagBitsEXT>>::from(i),
            );
        }
    }
}
impl FromIterator<VideoEncodeH265RateControlStructureFlagsEXT> for VideoEncodeH265RateControlStructureFlagsEXT {
    fn from_iter<T: IntoIterator<Item = VideoEncodeH265RateControlStructureFlagsEXT>>(
        iterator: T,
    ) -> VideoEncodeH265RateControlStructureFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeH265RateControlStructureFlagsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<VideoEncodeH265RateControlStructureFlagBitsEXT> for VideoEncodeH265RateControlStructureFlagsEXT {
    fn from_iter<T: IntoIterator<Item = VideoEncodeH265RateControlStructureFlagBitsEXT>>(
        iterator: T,
    ) -> VideoEncodeH265RateControlStructureFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeH265RateControlStructureFlagBitsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for VideoEncodeH265RateControlStructureFlagsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoEncodeH265RateControlStructureFlagsEXT);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoEncodeH265RateControlStructureFlagsEXT::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(VideoEncodeH265RateControlStructureFlagsEXT::UNKNOWN) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(UNKNOWN))?;
                    }
                    if self.0.contains(VideoEncodeH265RateControlStructureFlagsEXT::FLAT) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(FLAT))?;
                    }
                    if self.0.contains(VideoEncodeH265RateControlStructureFlagsEXT::DYADIC) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(DYADIC))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoEncodeH265RateControlStructureFlagsEXT))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoEncodeH265CtbSizeFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265CtbSizeFlagBitsEXT.html) - Supported CTB sizes for H.265 video encode
///# C Specifications
///Bits which  **may**  be set in
///[`VideoEncodeH265CapabilitiesEXT::ctb_sizes`], indicating the CTB
///sizes supported by the implementation, are:
///```c
///// Provided by VK_EXT_video_encode_h265
///typedef enum VkVideoEncodeH265CtbSizeFlagBitsEXT {
///    VK_VIDEO_ENCODE_H265_CTB_SIZE_16_BIT_EXT = 0x00000001,
///    VK_VIDEO_ENCODE_H265_CTB_SIZE_32_BIT_EXT = 0x00000002,
///    VK_VIDEO_ENCODE_H265_CTB_SIZE_64_BIT_EXT = 0x00000004,
///} VkVideoEncodeH265CtbSizeFlagBitsEXT;
///```
///# Description
/// - [`16`] specifies that a CTB size of 16x16 is supported.
/// - [`32`] specifies that a CTB size of 32x32 is supported.
/// - [`64`] specifies that a CTB size of 64x64 is supported.
///# Related
/// - [`VK_EXT_video_encode_h265`]
/// - [`VideoEncodeH265CtbSizeFlagsEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoEncodeH265CtbSizeFlagsEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoEncodeH265CtbSizeFlagsEXT(u32);
impl const Default for VideoEncodeH265CtbSizeFlagsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl From<VideoEncodeH265CtbSizeFlagBitsEXT> for VideoEncodeH265CtbSizeFlagsEXT {
    fn from(from: VideoEncodeH265CtbSizeFlagBitsEXT) -> Self {
        unsafe { Self::from_bits_unchecked(from.bits()) }
    }
}
impl VideoEncodeH265CtbSizeFlagsEXT {
    ///[`16`] specifies that a CTB size
    ///of 16x16 is supported.
    pub const _16: Self = Self(1);
    ///[`32`] specifies that a CTB size
    ///of 32x32 is supported.
    pub const _32: Self = Self(2);
    ///[`64`] specifies that a CTB size
    ///of 64x64 is supported.
    pub const _64: Self = Self(4);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        {
            all |= Self::_16;
        }
        {
            all |= Self::_32;
        }
        {
            all |= Self::_64;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for VideoEncodeH265CtbSizeFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for VideoEncodeH265CtbSizeFlagsEXT {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for VideoEncodeH265CtbSizeFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for VideoEncodeH265CtbSizeFlagsEXT {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for VideoEncodeH265CtbSizeFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for VideoEncodeH265CtbSizeFlagsEXT {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for VideoEncodeH265CtbSizeFlagsEXT {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for VideoEncodeH265CtbSizeFlagsEXT {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for VideoEncodeH265CtbSizeFlagsEXT {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<VideoEncodeH265CtbSizeFlagsEXT> for VideoEncodeH265CtbSizeFlagsEXT {
    fn extend<T: IntoIterator<Item = VideoEncodeH265CtbSizeFlagsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<VideoEncodeH265CtbSizeFlagBitsEXT> for VideoEncodeH265CtbSizeFlagsEXT {
    fn extend<T: IntoIterator<Item = VideoEncodeH265CtbSizeFlagBitsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<VideoEncodeH265CtbSizeFlagBitsEXT>>::from(i));
        }
    }
}
impl FromIterator<VideoEncodeH265CtbSizeFlagsEXT> for VideoEncodeH265CtbSizeFlagsEXT {
    fn from_iter<T: IntoIterator<Item = VideoEncodeH265CtbSizeFlagsEXT>>(
        iterator: T,
    ) -> VideoEncodeH265CtbSizeFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeH265CtbSizeFlagsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<VideoEncodeH265CtbSizeFlagBitsEXT> for VideoEncodeH265CtbSizeFlagsEXT {
    fn from_iter<T: IntoIterator<Item = VideoEncodeH265CtbSizeFlagBitsEXT>>(
        iterator: T,
    ) -> VideoEncodeH265CtbSizeFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeH265CtbSizeFlagBitsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for VideoEncodeH265CtbSizeFlagsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoEncodeH265CtbSizeFlagsEXT);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoEncodeH265CtbSizeFlagsEXT::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(VideoEncodeH265CtbSizeFlagsEXT::_16) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(_16))?;
                    }
                    if self.0.contains(VideoEncodeH265CtbSizeFlagsEXT::_32) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(_32))?;
                    }
                    if self.0.contains(VideoEncodeH265CtbSizeFlagsEXT::_64) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(_64))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoEncodeH265CtbSizeFlagsEXT))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoEncodeH265TransformBlockSizeFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265TransformBlockSizeFlagBitsEXT.html) - Supported transform block sizes for H.265 video encode
///# C Specifications
///Bits which  **may**  be set in
///[`VideoEncodeH265CapabilitiesEXT::transform_block_sizes`],
///indicating the transform block sizes supported by the implementation, are:
///```c
///// Provided by VK_EXT_video_encode_h265
///typedef enum VkVideoEncodeH265TransformBlockSizeFlagBitsEXT {
///    VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_4_BIT_EXT = 0x00000001,
///    VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_8_BIT_EXT = 0x00000002,
///    VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_16_BIT_EXT = 0x00000004,
///    VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_32_BIT_EXT = 0x00000008,
///} VkVideoEncodeH265TransformBlockSizeFlagBitsEXT;
///```
///# Description
/// - [`4`] specifies that a transform block size of 4x4 is supported.
/// - [`8`] specifies that a transform block size of 8x8 is supported.
/// - [`16`] specifies that a transform block size of 16x16 is supported.
/// - [`32`] specifies that a transform block size of 32x32 is supported.
///# Related
/// - [`VK_EXT_video_encode_h265`]
/// - [`VideoEncodeH265TransformBlockSizeFlagsEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoEncodeH265TransformBlockSizeFlagsEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoEncodeH265TransformBlockSizeFlagsEXT(u32);
impl const Default for VideoEncodeH265TransformBlockSizeFlagsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl From<VideoEncodeH265TransformBlockSizeFlagBitsEXT> for VideoEncodeH265TransformBlockSizeFlagsEXT {
    fn from(from: VideoEncodeH265TransformBlockSizeFlagBitsEXT) -> Self {
        unsafe { Self::from_bits_unchecked(from.bits()) }
    }
}
impl VideoEncodeH265TransformBlockSizeFlagsEXT {
    ///[`4`] specifies that
    ///a transform block size of 4x4 is supported.
    pub const _4: Self = Self(1);
    ///[`8`] specifies that
    ///a transform block size of 8x8 is supported.
    pub const _8: Self = Self(2);
    ///[`16`] specifies
    ///that a transform block size of 16x16 is supported.
    pub const _16: Self = Self(4);
    ///[`32`] specifies
    ///that a transform block size of 32x32 is supported.
    pub const _32: Self = Self(8);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        {
            all |= Self::_4;
        }
        {
            all |= Self::_8;
        }
        {
            all |= Self::_16;
        }
        {
            all |= Self::_32;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for VideoEncodeH265TransformBlockSizeFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for VideoEncodeH265TransformBlockSizeFlagsEXT {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for VideoEncodeH265TransformBlockSizeFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for VideoEncodeH265TransformBlockSizeFlagsEXT {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for VideoEncodeH265TransformBlockSizeFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for VideoEncodeH265TransformBlockSizeFlagsEXT {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for VideoEncodeH265TransformBlockSizeFlagsEXT {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for VideoEncodeH265TransformBlockSizeFlagsEXT {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for VideoEncodeH265TransformBlockSizeFlagsEXT {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<VideoEncodeH265TransformBlockSizeFlagsEXT> for VideoEncodeH265TransformBlockSizeFlagsEXT {
    fn extend<T: IntoIterator<Item = VideoEncodeH265TransformBlockSizeFlagsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<VideoEncodeH265TransformBlockSizeFlagBitsEXT> for VideoEncodeH265TransformBlockSizeFlagsEXT {
    fn extend<T: IntoIterator<Item = VideoEncodeH265TransformBlockSizeFlagBitsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(
                self,
                <Self as From<VideoEncodeH265TransformBlockSizeFlagBitsEXT>>::from(i),
            );
        }
    }
}
impl FromIterator<VideoEncodeH265TransformBlockSizeFlagsEXT> for VideoEncodeH265TransformBlockSizeFlagsEXT {
    fn from_iter<T: IntoIterator<Item = VideoEncodeH265TransformBlockSizeFlagsEXT>>(
        iterator: T,
    ) -> VideoEncodeH265TransformBlockSizeFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeH265TransformBlockSizeFlagsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<VideoEncodeH265TransformBlockSizeFlagBitsEXT> for VideoEncodeH265TransformBlockSizeFlagsEXT {
    fn from_iter<T: IntoIterator<Item = VideoEncodeH265TransformBlockSizeFlagBitsEXT>>(
        iterator: T,
    ) -> VideoEncodeH265TransformBlockSizeFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeH265TransformBlockSizeFlagBitsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for VideoEncodeH265TransformBlockSizeFlagsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoEncodeH265TransformBlockSizeFlagsEXT);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoEncodeH265TransformBlockSizeFlagsEXT::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(VideoEncodeH265TransformBlockSizeFlagsEXT::_4) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(_4))?;
                    }
                    if self.0.contains(VideoEncodeH265TransformBlockSizeFlagsEXT::_8) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(_8))?;
                    }
                    if self.0.contains(VideoEncodeH265TransformBlockSizeFlagsEXT::_16) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(_16))?;
                    }
                    if self.0.contains(VideoEncodeH265TransformBlockSizeFlagsEXT::_32) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(_32))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoEncodeH265TransformBlockSizeFlagsEXT))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoEncodeH265CapabilitiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265CapabilitiesEXT.html) - Structure specifying H.265 encode capabilities
///# C Specifications
///When calling [`get_physical_device_video_capabilities_khr`] with
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
///# Related
/// - [`VK_EXT_video_encode_h265`]
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
#[doc(alias = "VkVideoEncodeH265CapabilitiesEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeH265CapabilitiesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`VideoEncodeH265CapabilityFlagBitsEXT`]
    ///describing supported encoding tools.
    pub flags: VideoEncodeH265CapabilityFlagsEXT,
    ///[`input_mode_flags`] is a bitmask of
    ///[`VideoEncodeH265InputModeFlagBitsEXT`] describing the command
    ///buffer input granularities/modes supported by the implementation.
    pub input_mode_flags: VideoEncodeH265InputModeFlagsEXT,
    ///[`output_mode_flags`] is a bitmask of
    ///[`VideoEncodeH265OutputModeFlagBitsEXT`] describing the output
    ///(bitstream size reporting) granularities/modes supported by the
    ///implementation.
    pub output_mode_flags: VideoEncodeH265OutputModeFlagsEXT,
    ///[`ctb_sizes`] is a bitmask of [`VideoEncodeH265CtbSizeFlagBitsEXT`]
    ///describing the supported CTB sizes.
    pub ctb_sizes: VideoEncodeH265CtbSizeFlagsEXT,
    ///[`transform_block_sizes`] is a bitmask of
    ///[`VideoEncodeH265TransformBlockSizeFlagBitsEXT`] describing the
    ///supported transform block sizes.
    pub transform_block_sizes: VideoEncodeH265TransformBlockSizeFlagsEXT,
    ///[`max_p_picture_l_0_reference_count`] reports the maximum number of
    ///reference pictures the implementation supports in the reference list L0
    ///for P pictures.
    pub max_p_picture_l_0_reference_count: u8,
    ///[`max_b_picture_l_0_reference_count`] reports the maximum number of
    ///reference pictures the implementation supports in the reference list L0
    ///for B pictures.
    ///The reported value is `0` if encoding of B pictures is not supported.
    pub max_b_picture_l_0_reference_count: u8,
    ///[`max_l_1_reference_count`] reports the maximum number of reference
    ///pictures the implementation supports in the reference list L1 if
    ///encoding of B pictures is supported.
    ///The reported value is `0` if encoding of B pictures is not supported.
    pub max_l_1_reference_count: u8,
    ///[`max_sub_layers_count`] reports the maximum number of sublayers.
    pub max_sub_layers_count: u8,
    ///[`min_log_2_min_luma_coding_block_size_minus_3`] reports the minimum value that
    ///may be set for log2_min_luma_coding_block_size_minus3 in
    ///StdVideoH265SequenceParameterSet.
    pub min_log_2_min_luma_coding_block_size_minus_3: u8,
    ///[`max_log_2_min_luma_coding_block_size_minus_3`] reports the maximum value that
    ///may be set for log2_min_luma_coding_block_size_minus3 in
    ///StdVideoH265SequenceParameterSet.
    pub max_log_2_min_luma_coding_block_size_minus_3: u8,
    ///[`min_log_2_min_luma_transform_block_size_minus_2`] reports the minimum value
    ///that may be set for log2_min_luma_transform_block_size_minus2 in
    ///StdVideoH265SequenceParameterSet.
    pub min_log_2_min_luma_transform_block_size_minus_2: u8,
    ///[`max_log_2_min_luma_transform_block_size_minus_2`] reports the maximum value
    ///that may be set for log2_min_luma_transform_block_size_minus2 in
    ///StdVideoH265SequenceParameterSet.
    pub max_log_2_min_luma_transform_block_size_minus_2: u8,
    ///[`min_max_transform_hierarchy_depth_inter`] reports the minimum value that
    ///may be set for max_transform_hierarchy_depth_inter in
    ///StdVideoH265SequenceParameterSet.
    pub min_max_transform_hierarchy_depth_inter: u8,
    ///[`max_max_transform_hierarchy_depth_inter`] reports the maximum value that
    ///may be set for max_transform_hierarchy_depth_inter in
    ///StdVideoH265SequenceParameterSet.
    pub max_max_transform_hierarchy_depth_inter: u8,
    ///[`min_max_transform_hierarchy_depth_intra`] reports the minimum value that
    ///may be set for max_transform_hierarchy_depth_intra in
    ///StdVideoH265SequenceParameterSet.
    pub min_max_transform_hierarchy_depth_intra: u8,
    ///[`max_max_transform_hierarchy_depth_intra`] reports the maximum value that
    ///may be set for max_transform_hierarchy_depth_intra in
    ///StdVideoH265SequenceParameterSet.
    pub max_max_transform_hierarchy_depth_intra: u8,
    ///[`max_diff_cu_qp_delta_depth`] reports the maximum value that may be set
    ///for diff_cu_qp_delta_depth in StdVideoH265PictureParameterSet.
    pub max_diff_cu_qp_delta_depth: u8,
    ///[`min_max_num_merge_cand`] reports the minimum value that may be set for
    ///MaxNumMergeCand in StdVideoEncodeH265SliceHeader.
    pub min_max_num_merge_cand: u8,
    ///[`max_max_num_merge_cand`] reports the maximum value that may be set for
    ///MaxNumMergeCand in StdVideoEncodeH265SliceHeader.
    pub max_max_num_merge_cand: u8,
}
impl<'lt> Default for VideoEncodeH265CapabilitiesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_ENCODE_H265_CAPABILITIES_EXT,
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
        }
    }
}
impl<'lt> VideoEncodeH265CapabilitiesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
        &mut self.max_p_picture_l_0_reference_count
    }
    ///Gets a mutable reference to the value of [`Self::max_b_picture_l_0_reference_count`]
    pub fn max_b_picture_l_0_reference_count_mut(&mut self) -> &mut u8 {
        &mut self.max_b_picture_l_0_reference_count
    }
    ///Gets a mutable reference to the value of [`Self::max_l_1_reference_count`]
    pub fn max_l_1_reference_count_mut(&mut self) -> &mut u8 {
        &mut self.max_l_1_reference_count
    }
    ///Gets a mutable reference to the value of [`Self::max_sub_layers_count`]
    pub fn max_sub_layers_count_mut(&mut self) -> &mut u8 {
        &mut self.max_sub_layers_count
    }
    ///Gets a mutable reference to the value of
    /// [`Self::min_log_2_min_luma_coding_block_size_minus_3`]
    pub fn min_log_2_min_luma_coding_block_size_minus_3_mut(&mut self) -> &mut u8 {
        &mut self.min_log_2_min_luma_coding_block_size_minus_3
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_log_2_min_luma_coding_block_size_minus_3`]
    pub fn max_log_2_min_luma_coding_block_size_minus_3_mut(&mut self) -> &mut u8 {
        &mut self.max_log_2_min_luma_coding_block_size_minus_3
    }
    ///Gets a mutable reference to the value of
    /// [`Self::min_log_2_min_luma_transform_block_size_minus_2`]
    pub fn min_log_2_min_luma_transform_block_size_minus_2_mut(&mut self) -> &mut u8 {
        &mut self.min_log_2_min_luma_transform_block_size_minus_2
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_log_2_min_luma_transform_block_size_minus_2`]
    pub fn max_log_2_min_luma_transform_block_size_minus_2_mut(&mut self) -> &mut u8 {
        &mut self.max_log_2_min_luma_transform_block_size_minus_2
    }
    ///Gets a mutable reference to the value of [`Self::min_max_transform_hierarchy_depth_inter`]
    pub fn min_max_transform_hierarchy_depth_inter_mut(&mut self) -> &mut u8 {
        &mut self.min_max_transform_hierarchy_depth_inter
    }
    ///Gets a mutable reference to the value of [`Self::max_max_transform_hierarchy_depth_inter`]
    pub fn max_max_transform_hierarchy_depth_inter_mut(&mut self) -> &mut u8 {
        &mut self.max_max_transform_hierarchy_depth_inter
    }
    ///Gets a mutable reference to the value of [`Self::min_max_transform_hierarchy_depth_intra`]
    pub fn min_max_transform_hierarchy_depth_intra_mut(&mut self) -> &mut u8 {
        &mut self.min_max_transform_hierarchy_depth_intra
    }
    ///Gets a mutable reference to the value of [`Self::max_max_transform_hierarchy_depth_intra`]
    pub fn max_max_transform_hierarchy_depth_intra_mut(&mut self) -> &mut u8 {
        &mut self.max_max_transform_hierarchy_depth_intra
    }
    ///Gets a mutable reference to the value of [`Self::max_diff_cu_qp_delta_depth`]
    pub fn max_diff_cu_qp_delta_depth_mut(&mut self) -> &mut u8 {
        &mut self.max_diff_cu_qp_delta_depth
    }
    ///Gets a mutable reference to the value of [`Self::min_max_num_merge_cand`]
    pub fn min_max_num_merge_cand_mut(&mut self) -> &mut u8 {
        &mut self.min_max_num_merge_cand
    }
    ///Gets a mutable reference to the value of [`Self::max_max_num_merge_cand`]
    pub fn max_max_num_merge_cand_mut(&mut self) -> &mut u8 {
        &mut self.max_max_num_merge_cand
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn set_flags(
        mut self,
        value: crate::extensions::ext_video_encode_h_265::VideoEncodeH265CapabilityFlagsEXT,
    ) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::input_mode_flags`]
    pub fn set_input_mode_flags(
        mut self,
        value: crate::extensions::ext_video_encode_h_265::VideoEncodeH265InputModeFlagsEXT,
    ) -> Self {
        self.input_mode_flags = value;
        self
    }
    ///Sets the value of [`Self::output_mode_flags`]
    pub fn set_output_mode_flags(
        mut self,
        value: crate::extensions::ext_video_encode_h_265::VideoEncodeH265OutputModeFlagsEXT,
    ) -> Self {
        self.output_mode_flags = value;
        self
    }
    ///Sets the value of [`Self::ctb_sizes`]
    pub fn set_ctb_sizes(
        mut self,
        value: crate::extensions::ext_video_encode_h_265::VideoEncodeH265CtbSizeFlagsEXT,
    ) -> Self {
        self.ctb_sizes = value;
        self
    }
    ///Sets the value of [`Self::transform_block_sizes`]
    pub fn set_transform_block_sizes(
        mut self,
        value: crate::extensions::ext_video_encode_h_265::VideoEncodeH265TransformBlockSizeFlagsEXT,
    ) -> Self {
        self.transform_block_sizes = value;
        self
    }
    ///Sets the value of [`Self::max_p_picture_l_0_reference_count`]
    pub fn set_max_p_picture_l_0_reference_count(mut self, value: u8) -> Self {
        self.max_p_picture_l_0_reference_count = value;
        self
    }
    ///Sets the value of [`Self::max_b_picture_l_0_reference_count`]
    pub fn set_max_b_picture_l_0_reference_count(mut self, value: u8) -> Self {
        self.max_b_picture_l_0_reference_count = value;
        self
    }
    ///Sets the value of [`Self::max_l_1_reference_count`]
    pub fn set_max_l_1_reference_count(mut self, value: u8) -> Self {
        self.max_l_1_reference_count = value;
        self
    }
    ///Sets the value of [`Self::max_sub_layers_count`]
    pub fn set_max_sub_layers_count(mut self, value: u8) -> Self {
        self.max_sub_layers_count = value;
        self
    }
    ///Sets the value of [`Self::min_log_2_min_luma_coding_block_size_minus_3`]
    pub fn set_min_log_2_min_luma_coding_block_size_minus_3(mut self, value: u8) -> Self {
        self.min_log_2_min_luma_coding_block_size_minus_3 = value;
        self
    }
    ///Sets the value of [`Self::max_log_2_min_luma_coding_block_size_minus_3`]
    pub fn set_max_log_2_min_luma_coding_block_size_minus_3(mut self, value: u8) -> Self {
        self.max_log_2_min_luma_coding_block_size_minus_3 = value;
        self
    }
    ///Sets the value of [`Self::min_log_2_min_luma_transform_block_size_minus_2`]
    pub fn set_min_log_2_min_luma_transform_block_size_minus_2(mut self, value: u8) -> Self {
        self.min_log_2_min_luma_transform_block_size_minus_2 = value;
        self
    }
    ///Sets the value of [`Self::max_log_2_min_luma_transform_block_size_minus_2`]
    pub fn set_max_log_2_min_luma_transform_block_size_minus_2(mut self, value: u8) -> Self {
        self.max_log_2_min_luma_transform_block_size_minus_2 = value;
        self
    }
    ///Sets the value of [`Self::min_max_transform_hierarchy_depth_inter`]
    pub fn set_min_max_transform_hierarchy_depth_inter(mut self, value: u8) -> Self {
        self.min_max_transform_hierarchy_depth_inter = value;
        self
    }
    ///Sets the value of [`Self::max_max_transform_hierarchy_depth_inter`]
    pub fn set_max_max_transform_hierarchy_depth_inter(mut self, value: u8) -> Self {
        self.max_max_transform_hierarchy_depth_inter = value;
        self
    }
    ///Sets the value of [`Self::min_max_transform_hierarchy_depth_intra`]
    pub fn set_min_max_transform_hierarchy_depth_intra(mut self, value: u8) -> Self {
        self.min_max_transform_hierarchy_depth_intra = value;
        self
    }
    ///Sets the value of [`Self::max_max_transform_hierarchy_depth_intra`]
    pub fn set_max_max_transform_hierarchy_depth_intra(mut self, value: u8) -> Self {
        self.max_max_transform_hierarchy_depth_intra = value;
        self
    }
    ///Sets the value of [`Self::max_diff_cu_qp_delta_depth`]
    pub fn set_max_diff_cu_qp_delta_depth(mut self, value: u8) -> Self {
        self.max_diff_cu_qp_delta_depth = value;
        self
    }
    ///Sets the value of [`Self::min_max_num_merge_cand`]
    pub fn set_min_max_num_merge_cand(mut self, value: u8) -> Self {
        self.min_max_num_merge_cand = value;
        self
    }
    ///Sets the value of [`Self::max_max_num_merge_cand`]
    pub fn set_max_max_num_merge_cand(mut self, value: u8) -> Self {
        self.max_max_num_merge_cand = value;
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
#[doc(alias = "VkVideoEncodeH265SessionParametersAddInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeH265SessionParametersAddInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`vps_std_count`] is the number of VPS elements in [`vps_std`].
    pub vps_std_count: u32,
    ///[`vps_std`] is a pointer to an array of [`vps_std_count`][`StdVideoH265VideoParameterSet`]
    /// structures representing H.265 video parameter sets.
    pub vps_std: *const StdVideoH265VideoParameterSet,
    ///[`sps_std_count`] is the number of SPS elements in [`sps_std`].
    pub sps_std_count: u32,
    ///[`sps_std`] is a pointer to an array of
    /// [`sps_std_count`][`StdVideoH265SequenceParameterSet`] structures representing H.265
    /// sequence parameter sets.
    pub sps_std: *const StdVideoH265SequenceParameterSet,
    ///[`pps_std_count`] is the number of PPS elements in [`pps_std`].
    pub pps_std_count: u32,
    ///[`pps_std`] is a pointer to an array of [`pps_std_count`][`StdVideoH265PictureParameterSet`]
    /// structures representing H.265 picture parameter sets.
    pub pps_std: *const StdVideoH265PictureParameterSet,
}
impl<'lt> Default for VideoEncodeH265SessionParametersAddInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_ENCODE_H265_SESSION_PARAMETERS_ADD_INFO_EXT,
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
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    pub fn vps_std(&self) -> *const StdVideoH265VideoParameterSet {
        self.vps_std
    }
    ///Gets the value of [`Self::sps_std_count`]
    pub fn sps_std_count(&self) -> u32 {
        self.sps_std_count
    }
    ///Gets the value of [`Self::sps_std`]
    pub fn sps_std(&self) -> *const StdVideoH265SequenceParameterSet {
        self.sps_std
    }
    ///Gets the value of [`Self::pps_std_count`]
    pub fn pps_std_count(&self) -> u32 {
        self.pps_std_count
    }
    ///Gets the value of [`Self::pps_std`]
    pub fn pps_std(&self) -> *const StdVideoH265PictureParameterSet {
        self.pps_std
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::vps_std_count`]
    pub fn vps_std_count_mut(&mut self) -> &mut u32 {
        &mut self.vps_std_count
    }
    ///Gets a mutable reference to the value of [`Self::sps_std_count`]
    pub fn sps_std_count_mut(&mut self) -> &mut u32 {
        &mut self.sps_std_count
    }
    ///Gets a mutable reference to the value of [`Self::pps_std_count`]
    pub fn pps_std_count_mut(&mut self) -> &mut u32 {
        &mut self.pps_std_count
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::vps_std_count`]
    pub fn set_vps_std_count(mut self, value: u32) -> Self {
        self.vps_std_count = value;
        self
    }
    ///Sets the value of [`Self::vps_std`]
    pub fn set_vps_std(mut self, value: *const crate::native::StdVideoH265VideoParameterSet) -> Self {
        self.vps_std = value;
        self
    }
    ///Sets the value of [`Self::sps_std_count`]
    pub fn set_sps_std_count(mut self, value: u32) -> Self {
        self.sps_std_count = value;
        self
    }
    ///Sets the value of [`Self::sps_std`]
    pub fn set_sps_std(mut self, value: *const crate::native::StdVideoH265SequenceParameterSet) -> Self {
        self.sps_std = value;
        self
    }
    ///Sets the value of [`Self::pps_std_count`]
    pub fn set_pps_std_count(mut self, value: u32) -> Self {
        self.pps_std_count = value;
        self
    }
    ///Sets the value of [`Self::pps_std`]
    pub fn set_pps_std(mut self, value: *const crate::native::StdVideoH265PictureParameterSet) -> Self {
        self.pps_std = value;
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
#[doc(alias = "VkVideoEncodeH265SessionParametersCreateInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeH265SessionParametersCreateInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`max_vps_std_count`] is the maximum number of entries of type
    ///[`StdVideoH265VideoParameterSet`] within
    ///[`VideoSessionParametersKHR`].
    pub max_vps_std_count: u32,
    ///[`max_sps_std_count`] is the maximum number of entries of type
    ///[`StdVideoH265SequenceParameterSet`] within
    ///[`VideoSessionParametersKHR`].
    pub max_sps_std_count: u32,
    ///[`max_pps_std_count`] is the maximum number of entries of type
    ///[`StdVideoH265PictureParameterSet`] within
    ///[`VideoSessionParametersKHR`].
    pub max_pps_std_count: u32,
    ///[`parameters_add_info`] is `NULL` or a pointer to a
    ///[`VideoEncodeH265SessionParametersAddInfoEXT`] structure specifying
    ///the video session parameters to add upon creation of this object.
    pub parameters_add_info: *const VideoEncodeH265SessionParametersAddInfoEXT<'lt>,
}
impl<'lt> Default for VideoEncodeH265SessionParametersCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_ENCODE_H265_SESSION_PARAMETERS_CREATE_INFO_EXT,
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
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::parameters_add_info`]
    pub fn set_parameters_add_info_raw(
        mut self,
        value: *const VideoEncodeH265SessionParametersAddInfoEXT<'lt>,
    ) -> Self {
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
        &mut self.max_vps_std_count
    }
    ///Gets a mutable reference to the value of [`Self::max_sps_std_count`]
    pub fn max_sps_std_count_mut(&mut self) -> &mut u32 {
        &mut self.max_sps_std_count
    }
    ///Gets a mutable reference to the value of [`Self::max_pps_std_count`]
    pub fn max_pps_std_count_mut(&mut self) -> &mut u32 {
        &mut self.max_pps_std_count
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::max_vps_std_count`]
    pub fn set_max_vps_std_count(mut self, value: u32) -> Self {
        self.max_vps_std_count = value;
        self
    }
    ///Sets the value of [`Self::max_sps_std_count`]
    pub fn set_max_sps_std_count(mut self, value: u32) -> Self {
        self.max_sps_std_count = value;
        self
    }
    ///Sets the value of [`Self::max_pps_std_count`]
    pub fn set_max_pps_std_count(mut self, value: u32) -> Self {
        self.max_pps_std_count = value;
        self
    }
    ///Sets the value of [`Self::parameters_add_info`]
    pub fn set_parameters_add_info(
        mut self,
        value: &'lt crate::extensions::ext_video_encode_h_265::VideoEncodeH265SessionParametersAddInfoEXT<'lt>,
    ) -> Self {
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
#[doc(alias = "VkVideoEncodeH265VclFrameInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeH265VclFrameInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`reference_final_lists`] is `NULL` or a pointer to a
    ///[`VideoEncodeH265ReferenceListsEXT`] structure specifying the
    ///reference lists to be used for the current picture.
    pub reference_final_lists: *const VideoEncodeH265ReferenceListsEXT<'lt>,
    ///[`nalu_slice_segment_entry_count`] is the number of slice segment NALUs in
    ///the frame.
    pub nalu_slice_segment_entry_count: u32,
    ///[`nalu_slice_segment_entries`] is a pointer to an array of
    ///[`VideoEncodeH265NaluSliceSegmentEXT`] structures specifying the
    ///division of the current picture into slice segments and the properties
    ///of these slice segments.
    pub nalu_slice_segment_entries: *const VideoEncodeH265NaluSliceSegmentEXT<'lt>,
    ///[`current_picture_info`] is a pointer to a
    ///[`StdVideoEncodeH265PictureInfo`] structure specifying the syntax and
    ///other codec-specific information from the H.265 specification,
    ///associated with this picture.
    pub current_picture_info: *const StdVideoEncodeH265PictureInfo,
}
impl<'lt> Default for VideoEncodeH265VclFrameInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_ENCODE_H265_VCL_FRAME_INFO_EXT,
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
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::reference_final_lists`]
    pub fn set_reference_final_lists_raw(mut self, value: *const VideoEncodeH265ReferenceListsEXT<'lt>) -> Self {
        self.reference_final_lists = value;
        self
    }
    ///Sets the raw value of [`Self::nalu_slice_segment_entries`]
    pub fn set_nalu_slice_segment_entries_raw(mut self, value: *const VideoEncodeH265NaluSliceSegmentEXT<'lt>) -> Self {
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
    pub fn current_picture_info(&self) -> *const StdVideoEncodeH265PictureInfo {
        self.current_picture_info
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::nalu_slice_segment_entry_count`]
    pub fn nalu_slice_segment_entry_count_mut(&mut self) -> &mut u32 {
        &mut self.nalu_slice_segment_entry_count
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::reference_final_lists`]
    pub fn set_reference_final_lists(
        mut self,
        value: &'lt crate::extensions::ext_video_encode_h_265::VideoEncodeH265ReferenceListsEXT<'lt>,
    ) -> Self {
        self.reference_final_lists = value as *const _;
        self
    }
    ///Sets the value of [`Self::nalu_slice_segment_entry_count`]
    pub fn set_nalu_slice_segment_entry_count(mut self, value: u32) -> Self {
        self.nalu_slice_segment_entry_count = value;
        self
    }
    ///Sets the value of [`Self::nalu_slice_segment_entries`]
    pub fn set_nalu_slice_segment_entries(
        mut self,
        value: &'lt [crate::extensions::ext_video_encode_h_265::VideoEncodeH265NaluSliceSegmentEXT<'lt>],
    ) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.nalu_slice_segment_entries = value.as_ptr();
        self.nalu_slice_segment_entry_count = len_;
        self
    }
    ///Sets the value of [`Self::current_picture_info`]
    pub fn set_current_picture_info(mut self, value: *const crate::native::StdVideoEncodeH265PictureInfo) -> Self {
        self.current_picture_info = value;
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
#[doc(alias = "VkVideoEncodeH265EmitPictureParametersEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeH265EmitPictureParametersEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`vps_id`] is the H.265 VPS ID for the H.265 VPS to insert in the
    ///bitstream.
    ///The VPS ID  **must**  match the VPS provided in `vpsStd` of
    ///[`VideoEncodeH265SessionParametersCreateInfoEXT`].
    ///This is retrieved from the [`VideoSessionParametersKHR`] object
    ///provided in [`VideoBeginCodingInfoKHR`].
    pub vps_id: u8,
    ///[`sps_id`] is the H.265 SPS ID for the H.265 SPS to insert in the
    ///bitstream.
    ///The SPS ID  **must**  match one of the IDs of the SPS(s) provided in
    ///`pSpsStd` of [`VideoEncodeH265SessionParametersCreateInfoEXT`]
    ///to identify the SPS parameter set to insert in the bitstream.
    ///This is retrieved from the [`VideoSessionParametersKHR`] object
    ///provided in [`VideoBeginCodingInfoKHR`].
    pub sps_id: u8,
    ///[`emit_vps_enable`] enables the emitting of the VPS structure with id of
    ///[`vps_id`].
    pub emit_vps_enable: Bool32,
    ///[`emit_sps_enable`] enables the emitting of the SPS structure with id of
    ///[`sps_id`].
    pub emit_sps_enable: Bool32,
    ///[`pps_id_entry_count`] is the number of entries in the
    ///[`pps_id_entries`].
    ///If this parameter is `0` then no pps entries are going to be emitted in
    ///the bitstream.
    pub pps_id_entry_count: u32,
    ///[`pps_id_entries`] is the H.265 PPS IDs for the H.265 PPS to insert in
    ///the bitstream.
    ///The PPS IDs  **must**  match one of the IDs of the PPS(s) provided in
    ///`pPpsStd` of [`VideoEncodeH265SessionParametersCreateInfoEXT`]
    ///to identify the PPS parameter set to insert in the bitstream.
    ///This is retrieved from the [`VideoSessionParametersKHR`] object
    ///provided in [`VideoBeginCodingInfoKHR`].
    pub pps_id_entries: *const u8,
}
impl<'lt> Default for VideoEncodeH265EmitPictureParametersEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_ENCODE_H265_EMIT_PICTURE_PARAMETERS_EXT,
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
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::emit_vps_enable`]
    pub fn set_emit_vps_enable_raw(mut self, value: Bool32) -> Self {
        self.emit_vps_enable = value;
        self
    }
    ///Sets the raw value of [`Self::emit_sps_enable`]
    pub fn set_emit_sps_enable_raw(mut self, value: Bool32) -> Self {
        self.emit_sps_enable = value;
        self
    }
    ///Sets the raw value of [`Self::pps_id_entries`]
    pub fn set_pps_id_entries_raw(mut self, value: *const u8) -> Self {
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
        &mut self.vps_id
    }
    ///Gets a mutable reference to the value of [`Self::sps_id`]
    pub fn sps_id_mut(&mut self) -> &mut u8 {
        &mut self.sps_id
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
        &mut self.pps_id_entry_count
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::vps_id`]
    pub fn set_vps_id(mut self, value: u8) -> Self {
        self.vps_id = value;
        self
    }
    ///Sets the value of [`Self::sps_id`]
    pub fn set_sps_id(mut self, value: u8) -> Self {
        self.sps_id = value;
        self
    }
    ///Sets the value of [`Self::emit_vps_enable`]
    pub fn set_emit_vps_enable(mut self, value: bool) -> Self {
        self.emit_vps_enable = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::emit_sps_enable`]
    pub fn set_emit_sps_enable(mut self, value: bool) -> Self {
        self.emit_sps_enable = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::pps_id_entry_count`]
    pub fn set_pps_id_entry_count(mut self, value: u32) -> Self {
        self.pps_id_entry_count = value;
        self
    }
    ///Sets the value of [`Self::pps_id_entries`]
    pub fn set_pps_id_entries(mut self, value: &'lt [u8]) -> Self {
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
#[doc(alias = "VkVideoEncodeH265NaluSliceSegmentEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeH265NaluSliceSegmentEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`ctb_count`] is the number of CTBs in this slice segment.
    pub ctb_count: u32,
    ///[`reference_final_lists`] is `NULL` or a pointer to a
    ///[`VideoEncodeH265ReferenceListsEXT`] structure specifying the
    ///reference lists to be used for the current slice segment.
    ///If [`reference_final_lists`] is not `NULL`, these reference lists
    ///override the reference lists provided in
    ///[`VideoEncodeH265VclFrameInfoEXT`]::[`reference_final_lists`].
    pub reference_final_lists: *const VideoEncodeH265ReferenceListsEXT<'lt>,
    ///[`slice_segment_header_std`] is a pointer to a
    ///[`StdVideoEncodeH265SliceSegmentHeader`] structure specifying the slice
    ///segment header for the current slice segment.
    pub slice_segment_header_std: *const StdVideoEncodeH265SliceSegmentHeader,
}
impl<'lt> Default for VideoEncodeH265NaluSliceSegmentEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_ENCODE_H265_NALU_SLICE_SEGMENT_EXT,
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
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::reference_final_lists`]
    pub fn set_reference_final_lists_raw(mut self, value: *const VideoEncodeH265ReferenceListsEXT<'lt>) -> Self {
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
    pub fn slice_segment_header_std(&self) -> *const StdVideoEncodeH265SliceSegmentHeader {
        self.slice_segment_header_std
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::ctb_count`]
    pub fn ctb_count_mut(&mut self) -> &mut u32 {
        &mut self.ctb_count
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::ctb_count`]
    pub fn set_ctb_count(mut self, value: u32) -> Self {
        self.ctb_count = value;
        self
    }
    ///Sets the value of [`Self::reference_final_lists`]
    pub fn set_reference_final_lists(
        mut self,
        value: &'lt crate::extensions::ext_video_encode_h_265::VideoEncodeH265ReferenceListsEXT<'lt>,
    ) -> Self {
        self.reference_final_lists = value as *const _;
        self
    }
    ///Sets the value of [`Self::slice_segment_header_std`]
    pub fn set_slice_segment_header_std(
        mut self,
        value: *const crate::native::StdVideoEncodeH265SliceSegmentHeader,
    ) -> Self {
        self.slice_segment_header_std = value;
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
///[`cmd_control_video_coding_khr`] command.The parameters from this structure act as a guidance
/// for implementations to
///apply various rate control heuristics.It is possible to infer the picture type to be used when
/// encoding a frame,
///on the basis of the values provided for [`consecutive_b_frame_count`],
///[`idr_period`], and [`gop_frame_count`], but this inferred picture type
///will not be used by implementations to override the picture type provided in
///[`cmd_encode_video_khr`].
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
#[doc(alias = "VkVideoEncodeH265RateControlInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeH265RateControlInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`gop_frame_count`] is the number of frames contained within the group
    ///of pictures (GOP), starting from an intra frame and until the next intra
    ///frame.
    ///If it is set to 0, the implementation chooses a suitable value.
    ///If it is set to `UINT32_MAX`, the GOP length is treated as infinite.
    pub gop_frame_count: u32,
    ///[`idr_period`] is the interval, in terms of number of frames, between
    ///two IDR frames.
    ///If it is set to 0, the implementation chooses a suitable value.
    ///If it is set to `UINT32_MAX`, the IDR period is treated as infinite.
    pub idr_period: u32,
    ///[`consecutive_b_frame_count`] is the number of consecutive B-frames
    ///between I- and/or P-frames within the GOP.
    pub consecutive_b_frame_count: u32,
    ///[`rate_control_structure`] is a
    ///[`VideoEncodeH265RateControlStructureFlagBitsEXT`] value specifying
    ///the expected encode stream reference structure, to aid in rate control
    ///calculations.
    pub rate_control_structure: VideoEncodeH265RateControlStructureFlagBitsEXT,
    ///[`sub_layer_count`] specifies the number of sub layers enabled in the
    ///stream.
    pub sub_layer_count: u8,
}
impl<'lt> Default for VideoEncodeH265RateControlInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_ENCODE_H265_RATE_CONTROL_INFO_EXT,
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
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
        &mut self.gop_frame_count
    }
    ///Gets a mutable reference to the value of [`Self::idr_period`]
    pub fn idr_period_mut(&mut self) -> &mut u32 {
        &mut self.idr_period
    }
    ///Gets a mutable reference to the value of [`Self::consecutive_b_frame_count`]
    pub fn consecutive_b_frame_count_mut(&mut self) -> &mut u32 {
        &mut self.consecutive_b_frame_count
    }
    ///Gets a mutable reference to the value of [`Self::rate_control_structure`]
    pub fn rate_control_structure_mut(&mut self) -> &mut VideoEncodeH265RateControlStructureFlagBitsEXT {
        &mut self.rate_control_structure
    }
    ///Gets a mutable reference to the value of [`Self::sub_layer_count`]
    pub fn sub_layer_count_mut(&mut self) -> &mut u8 {
        &mut self.sub_layer_count
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::gop_frame_count`]
    pub fn set_gop_frame_count(mut self, value: u32) -> Self {
        self.gop_frame_count = value;
        self
    }
    ///Sets the value of [`Self::idr_period`]
    pub fn set_idr_period(mut self, value: u32) -> Self {
        self.idr_period = value;
        self
    }
    ///Sets the value of [`Self::consecutive_b_frame_count`]
    pub fn set_consecutive_b_frame_count(mut self, value: u32) -> Self {
        self.consecutive_b_frame_count = value;
        self
    }
    ///Sets the value of [`Self::rate_control_structure`]
    pub fn set_rate_control_structure(
        mut self,
        value: crate::extensions::ext_video_encode_h_265::VideoEncodeH265RateControlStructureFlagBitsEXT,
    ) -> Self {
        self.rate_control_structure = value;
        self
    }
    ///Sets the value of [`Self::sub_layer_count`]
    pub fn set_sub_layer_count(mut self, value: u8) -> Self {
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
#[doc(alias = "VkVideoEncodeH265QpEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VideoEncodeH265QpEXT {
    ///[`qp_i`] is the QP to be used for I-frames.
    pub qp_i: i32,
    ///[`qp_p`] is the QP to be used for P-frames.
    pub qp_p: i32,
    ///[`qp_b`] is the QP to be used for B-frames.
    pub qp_b: i32,
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
        &mut self.qp_i
    }
    ///Gets a mutable reference to the value of [`Self::qp_p`]
    pub fn qp_p_mut(&mut self) -> &mut i32 {
        &mut self.qp_p
    }
    ///Gets a mutable reference to the value of [`Self::qp_b`]
    pub fn qp_b_mut(&mut self) -> &mut i32 {
        &mut self.qp_b
    }
    ///Sets the value of [`Self::qp_i`]
    pub fn set_qp_i(mut self, value: i32) -> Self {
        self.qp_i = value;
        self
    }
    ///Sets the value of [`Self::qp_p`]
    pub fn set_qp_p(mut self, value: i32) -> Self {
        self.qp_p = value;
        self
    }
    ///Sets the value of [`Self::qp_b`]
    pub fn set_qp_b(mut self, value: i32) -> Self {
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
#[doc(alias = "VkVideoEncodeH265FrameSizeEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VideoEncodeH265FrameSizeEXT {
    ///[`frame_i_size`] is the size in bytes to be used for I-frames.
    pub frame_i_size: u32,
    ///[`frame_p_size`] is the size in bytes to be used for P-frames.
    pub frame_p_size: u32,
    ///[`frame_b_size`] is the size in bytes to be used for B-frames.
    pub frame_b_size: u32,
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
        &mut self.frame_i_size
    }
    ///Gets a mutable reference to the value of [`Self::frame_p_size`]
    pub fn frame_p_size_mut(&mut self) -> &mut u32 {
        &mut self.frame_p_size
    }
    ///Gets a mutable reference to the value of [`Self::frame_b_size`]
    pub fn frame_b_size_mut(&mut self) -> &mut u32 {
        &mut self.frame_b_size
    }
    ///Sets the value of [`Self::frame_i_size`]
    pub fn set_frame_i_size(mut self, value: u32) -> Self {
        self.frame_i_size = value;
        self
    }
    ///Sets the value of [`Self::frame_p_size`]
    pub fn set_frame_p_size(mut self, value: u32) -> Self {
        self.frame_p_size = value;
        self
    }
    ///Sets the value of [`Self::frame_b_size`]
    pub fn set_frame_b_size(mut self, value: u32) -> Self {
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
///structure in a call to [`cmd_control_video_coding_khr`] command, when the
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
#[doc(alias = "VkVideoEncodeH265RateControlLayerInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeH265RateControlLayerInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`temporal_id`] specifies the H.265 temporal ID of the video coding
    ///layer that settings provided in this structure and its parent
    ///[`VideoEncodeRateControlLayerInfoKHR`] structure apply to.
    pub temporal_id: u8,
    ///[`use_initial_rc_qp`] indicates whether the values within
    ///[`initial_rc_qp`] should be used by the implementation.
    pub use_initial_rc_qp: Bool32,
    ///[`initial_rc_qp`] provides the QP values for each picture type, to be
    ///used in rate control calculations at the start of video encode
    ///operations on a newly-created video session, or immediately after a
    ///session reset.
    ///These values are ignored when
    ///[`VideoEncodeRateControlInfoKHR`]::`rateControlMode` is
    ///`VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`.
    pub initial_rc_qp: VideoEncodeH265QpEXT,
    ///[`use_min_qp`] indicates whether the values within [`min_qp`] should be
    ///used by the implementation.
    ///When it is set to [`FALSE`], the implementation ignores the values
    ///in [`min_qp`] and chooses suitable values.
    pub use_min_qp: Bool32,
    ///[`min_qp`] provides the lower bound on the QP values for each picture
    ///type, to be used in rate control calculations.
    pub min_qp: VideoEncodeH265QpEXT,
    ///[`use_max_qp`] indicates whether the values within [`max_qp`] should be
    ///used by the implementation.
    ///When it is set to [`FALSE`], the implementation ignores the values
    ///in [`max_qp`] and chooses suitable values.
    pub use_max_qp: Bool32,
    ///[`max_qp`] provides the upper bound on the QP values for each picture
    ///type, to be used in rate control calculations.
    pub max_qp: VideoEncodeH265QpEXT,
    ///[`use_max_frame_size`] indicates whether the values within
    ///[`max_frame_size`] should be used by the implementation.
    pub use_max_frame_size: Bool32,
    ///[`max_frame_size`] provides the upper bound on the encoded frame size
    ///for each picture type.
    ///The implementation does not guarantee the encoded frame sizes will be
    ///within the specified limits, however these limits  **may**  be used as a
    ///guide in rate control calculations.
    ///If enabled and not set properly, the [`max_qp`] limit may prevent the
    ///implementation from respecting the [`max_frame_size`] limit.
    pub max_frame_size: VideoEncodeH265FrameSizeEXT,
}
impl<'lt> Default for VideoEncodeH265RateControlLayerInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_ENCODE_H265_RATE_CONTROL_LAYER_INFO_EXT,
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
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::use_initial_rc_qp`]
    pub fn set_use_initial_rc_qp_raw(mut self, value: Bool32) -> Self {
        self.use_initial_rc_qp = value;
        self
    }
    ///Sets the raw value of [`Self::use_min_qp`]
    pub fn set_use_min_qp_raw(mut self, value: Bool32) -> Self {
        self.use_min_qp = value;
        self
    }
    ///Sets the raw value of [`Self::use_max_qp`]
    pub fn set_use_max_qp_raw(mut self, value: Bool32) -> Self {
        self.use_max_qp = value;
        self
    }
    ///Sets the raw value of [`Self::use_max_frame_size`]
    pub fn set_use_max_frame_size_raw(mut self, value: Bool32) -> Self {
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
        &mut self.temporal_id
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
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::temporal_id`]
    pub fn set_temporal_id(mut self, value: u8) -> Self {
        self.temporal_id = value;
        self
    }
    ///Sets the value of [`Self::use_initial_rc_qp`]
    pub fn set_use_initial_rc_qp(mut self, value: bool) -> Self {
        self.use_initial_rc_qp = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::initial_rc_qp`]
    pub fn set_initial_rc_qp(mut self, value: crate::extensions::ext_video_encode_h_265::VideoEncodeH265QpEXT) -> Self {
        self.initial_rc_qp = value;
        self
    }
    ///Sets the value of [`Self::use_min_qp`]
    pub fn set_use_min_qp(mut self, value: bool) -> Self {
        self.use_min_qp = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::min_qp`]
    pub fn set_min_qp(mut self, value: crate::extensions::ext_video_encode_h_265::VideoEncodeH265QpEXT) -> Self {
        self.min_qp = value;
        self
    }
    ///Sets the value of [`Self::use_max_qp`]
    pub fn set_use_max_qp(mut self, value: bool) -> Self {
        self.use_max_qp = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::max_qp`]
    pub fn set_max_qp(mut self, value: crate::extensions::ext_video_encode_h_265::VideoEncodeH265QpEXT) -> Self {
        self.max_qp = value;
        self
    }
    ///Sets the value of [`Self::use_max_frame_size`]
    pub fn set_use_max_frame_size(mut self, value: bool) -> Self {
        self.use_max_frame_size = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::max_frame_size`]
    pub fn set_max_frame_size(
        mut self,
        value: crate::extensions::ext_video_encode_h_265::VideoEncodeH265FrameSizeEXT,
    ) -> Self {
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
#[doc(alias = "VkVideoEncodeH265ProfileEXT")]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeH265ProfileEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`std_profile_idc`] is a [`StdVideoH265ProfileIdc`] value specifying
    ///the H.265 codec profile IDC.
    pub std_profile_idc: StdVideoH265ProfileIdc,
}
impl<'lt> Default for VideoEncodeH265ProfileEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_ENCODE_H265_PROFILE_EXT,
            p_next: std::ptr::null(),
            std_profile_idc: unsafe { std::mem::zeroed() },
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
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::std_profile_idc`]
    pub fn set_std_profile_idc_raw(mut self, value: StdVideoH265ProfileIdc) -> Self {
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
    pub fn std_profile_idc(&self) -> StdVideoH265ProfileIdc {
        self.std_profile_idc
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::std_profile_idc`]
    pub fn std_profile_idc_mut(&mut self) -> &mut StdVideoH265ProfileIdc {
        &mut self.std_profile_idc
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::std_profile_idc`]
    pub fn set_std_profile_idc(mut self, value: crate::native::StdVideoH265ProfileIdc) -> Self {
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
#[doc(alias = "VkVideoEncodeH265DpbSlotInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeH265DpbSlotInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`slot_index`] is the [DPB Slot](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#dpb-slot) index for this picture.
    pub slot_index: i8,
    ///[`std_reference_info`] is a pointer to a
    ///[`StdVideoEncodeH265ReferenceInfo`] structure specifying the syntax and
    ///other codec-specific information from the H.265 specification,
    ///associated with this reference picture.
    pub std_reference_info: *const StdVideoEncodeH265ReferenceInfo,
}
impl<'lt> Default for VideoEncodeH265DpbSlotInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_ENCODE_H265_DPB_SLOT_INFO_EXT,
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
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    pub fn std_reference_info(&self) -> *const StdVideoEncodeH265ReferenceInfo {
        self.std_reference_info
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::slot_index`]
    pub fn slot_index_mut(&mut self) -> &mut i8 {
        &mut self.slot_index
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::slot_index`]
    pub fn set_slot_index(mut self, value: i8) -> Self {
        self.slot_index = value;
        self
    }
    ///Sets the value of [`Self::std_reference_info`]
    pub fn set_std_reference_info(mut self, value: *const crate::native::StdVideoEncodeH265ReferenceInfo) -> Self {
        self.std_reference_info = value;
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
#[doc(alias = "VkVideoEncodeH265ReferenceListsEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeH265ReferenceListsEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`reference_list_0_entry_count`] is the number of reference pictures in
    ///reference list L0 and is identical to
    ///[`StdVideoEncodeH265SliceSegmentHeader`]::`num_ref_idx_l0_active_minus1`
    ///+ 1.
    pub reference_list_0_entry_count: u8,
    ///[`reference_list_0_entries`] is a pointer to an array of
    ///[`reference_list_0_entry_count`][`VideoEncodeH265DpbSlotInfoEXT`]
    ///structures specifying the reference list L0 entries for the current
    ///picture.
    pub reference_list_0_entries: *const VideoEncodeH265DpbSlotInfoEXT<'lt>,
    ///[`reference_list_1_entry_count`] is the number of reference pictures in
    ///reference list L1 and is identical to
    ///[`StdVideoEncodeH265SliceSegmentHeader`]::`num_ref_idx_l1_active_minus1`
    ///+ 1.
    pub reference_list_1_entry_count: u8,
    ///[`reference_list_1_entries`] is a pointer to an array of
    ///[`reference_list_1_entry_count`][`VideoEncodeH265DpbSlotInfoEXT`]
    ///structures specifying the reference list L1 entries for the current
    ///picture.
    pub reference_list_1_entries: *const VideoEncodeH265DpbSlotInfoEXT<'lt>,
    ///[`reference_modifications`] is a pointer to a
    ///[`StdVideoEncodeH265ReferenceModifications`] structure specifying
    ///reference list modifications.
    pub reference_modifications: *const StdVideoEncodeH265ReferenceModifications,
}
impl<'lt> Default for VideoEncodeH265ReferenceListsEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_ENCODE_H265_REFERENCE_LISTS_EXT,
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
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::reference_list_0_entries`]
    pub fn set_reference_list_0_entries_raw(mut self, value: *const VideoEncodeH265DpbSlotInfoEXT<'lt>) -> Self {
        self.reference_list_0_entries = value;
        self
    }
    ///Sets the raw value of [`Self::reference_list_1_entries`]
    pub fn set_reference_list_1_entries_raw(mut self, value: *const VideoEncodeH265DpbSlotInfoEXT<'lt>) -> Self {
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
    pub fn reference_modifications(&self) -> *const StdVideoEncodeH265ReferenceModifications {
        self.reference_modifications
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::reference_list_0_entry_count`]
    pub fn reference_list_0_entry_count_mut(&mut self) -> &mut u8 {
        &mut self.reference_list_0_entry_count
    }
    ///Gets a mutable reference to the value of [`Self::reference_list_1_entry_count`]
    pub fn reference_list_1_entry_count_mut(&mut self) -> &mut u8 {
        &mut self.reference_list_1_entry_count
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::reference_list_0_entry_count`]
    pub fn set_reference_list_0_entry_count(mut self, value: u8) -> Self {
        self.reference_list_0_entry_count = value;
        self
    }
    ///Sets the value of [`Self::reference_list_0_entries`]
    pub fn set_reference_list_0_entries(
        mut self,
        value: &'lt [crate::extensions::ext_video_encode_h_265::VideoEncodeH265DpbSlotInfoEXT<'lt>],
    ) -> Self {
        let len_ = value.len() as u8;
        let len_ = len_;
        self.reference_list_0_entries = value.as_ptr();
        self.reference_list_0_entry_count = len_;
        self
    }
    ///Sets the value of [`Self::reference_list_1_entry_count`]
    pub fn set_reference_list_1_entry_count(mut self, value: u8) -> Self {
        self.reference_list_1_entry_count = value;
        self
    }
    ///Sets the value of [`Self::reference_list_1_entries`]
    pub fn set_reference_list_1_entries(
        mut self,
        value: &'lt [crate::extensions::ext_video_encode_h_265::VideoEncodeH265DpbSlotInfoEXT<'lt>],
    ) -> Self {
        let len_ = value.len() as u8;
        let len_ = len_;
        self.reference_list_1_entries = value.as_ptr();
        self.reference_list_1_entry_count = len_;
        self
    }
    ///Sets the value of [`Self::reference_modifications`]
    pub fn set_reference_modifications(
        mut self,
        value: *const crate::native::StdVideoEncodeH265ReferenceModifications,
    ) -> Self {
        self.reference_modifications = value;
        self
    }
}
