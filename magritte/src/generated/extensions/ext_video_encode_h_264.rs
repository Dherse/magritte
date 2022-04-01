//![VK_EXT_video_encode_h264](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_video_encode_h264.html) - device extension
//!# Description
//!This extension allows applications to compress a raw video sequence by using
//!the H.264/AVC video compression standard.
//!# Revision
//!5
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_video_encode_queue`]`
//! - **This is a *provisional* extension and  **must**  be used with caution. See the [description](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#boilerplate-provisional-header)
//!   of provisional header files for enablement and stability details.**
//!# Contacts
//! - Ahmed Abdelkhalek [aabdelkh](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_video_encode_h264]
//!   @aabdelkh%0A<<Here describe the issue or question you have about the VK_EXT_video_encode_h264
//!   extension>>)
//!# New structures
//! - [`VideoEncodeH264DpbSlotInfoEXT`]
//! - [`VideoEncodeH264FrameSizeEXT`]
//! - [`VideoEncodeH264NaluSliceEXT`]
//! - [`VideoEncodeH264QpEXT`]
//! - [`VideoEncodeH264ReferenceListsEXT`]
//! - Extending [`VideoEncodeCapabilitiesKHR`]:  - [`VideoEncodeH264CapabilitiesEXT`]
//! - Extending [`VideoEncodeInfoKHR`]:  - [`VideoEncodeH264EmitPictureParametersEXT`]  -
//!   [`VideoEncodeH264VclFrameInfoEXT`]
//! - Extending [`VideoEncodeRateControlInfoKHR`]:  - [`VideoEncodeH264RateControlInfoEXT`]
//! - Extending [`VideoEncodeRateControlLayerInfoKHR`]:  -
//!   [`VideoEncodeH264RateControlLayerInfoEXT`]
//! - Extending [`VideoProfileKHR`], [`QueryPoolCreateInfo`], [`FormatProperties2`],
//!   [`ImageCreateInfo`], [`ImageViewCreateInfo`], [`BufferCreateInfo`]:  -
//!   [`VideoEncodeH264ProfileEXT`]
//! - Extending [`VideoSessionCreateInfoKHR`]:  - [`VideoEncodeH264SessionCreateInfoEXT`]
//! - Extending [`VideoSessionParametersCreateInfoKHR`]:  -
//!   [`VideoEncodeH264SessionParametersCreateInfoEXT`]
//! - Extending [`VideoSessionParametersUpdateInfoKHR`]:  -
//!   [`VideoEncodeH264SessionParametersAddInfoEXT`]
//!# New enums
//! - [`VideoEncodeH264CapabilityFlagBitsEXT`]
//! - [`VideoEncodeH264CreateFlagBitsEXT`]
//! - [`VideoEncodeH264InputModeFlagBitsEXT`]
//! - [`VideoEncodeH264OutputModeFlagBitsEXT`]
//! - [`VideoEncodeH264RateControlStructureFlagBitsEXT`]
//!# New bitmasks
//! - [`VideoEncodeH264CapabilityFlagsEXT`]
//! - [`VideoEncodeH264CreateFlagsEXT`]
//! - [`VideoEncodeH264InputModeFlagsEXT`]
//! - [`VideoEncodeH264OutputModeFlagsEXT`]
//! - [`VideoEncodeH264RateControlStructureFlagsEXT`]
//!# New constants
//! - [`EXT_VIDEO_ENCODE_H264_EXTENSION_NAME`]
//! - [`EXT_VIDEO_ENCODE_H264_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_CAPABILITIES_EXT`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_DPB_SLOT_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_EMIT_PICTURE_PARAMETERS_EXT`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_NALU_SLICE_EXT`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_PROFILE_EXT`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_RATE_CONTROL_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_RATE_CONTROL_LAYER_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_REFERENCE_LISTS_EXT`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_SESSION_CREATE_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_SESSION_PARAMETERS_ADD_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_SESSION_PARAMETERS_CREATE_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_VCL_FRAME_INFO_EXT`
//! - Extending [`VideoCodecOperationFlagBitsKHR`]:  -
//!   `VK_VIDEO_CODEC_OPERATION_ENCODE_H264_BIT_EXT`
//!# Version History
//! - Revision 0, 2018-7-23 (Ahmed Abdelkhalek)  - Initial draft
//! - Revision 0.5, 2020-02-13 (Tony Zlatinski)  - General Spec cleanup  - Added DPB structures  -
//!   Change the VCL frame encode structure  - Added a common Non-VCL Picture Paramarameters
//!   structure
//! - Revision 1, 2021-03-29 (Tony Zlatinski)  - Spec and API updates
//! - Revision 2, August 1 2021 (Srinath Kumarapuram)  - Rename
//!   `VkVideoEncodeH264CapabilitiesFlagsEXT` to [`VideoEncodeH264CapabilityFlagsEXT`] and
//!   `VkVideoEncodeH264CapabilitiesFlagsEXT` to [`VideoEncodeH264CapabilityFlagsEXT`], following
//!   Vulkan naming conventions.
//! - Revision 3, 2021-12-08 (Ahmed Abdelkhalek)  - Rate control updates
//! - Revision 4, 2022-02-04 (Ahmed Abdelkhalek)  - Align VkVideoEncodeH264VclFrameInfoEXT structure
//!   to similar one in VK_EXT_video_encode_h265 extension
//! - Revision 5, 2022-02-10 (Ahmed Abdelkhalek)  - Updates to encode capability interface
//!# Other info
//! * 2022-02-10
//! * No known IP claims.
//! * - Ahmed Abdelkhalek, AMD  - Daniel Rakos, AMD  - George Hao, AMD  - Jake Beju, AMD  - Peter
//!   Fang, AMD  - Ping Liu, Intel  - Srinath Kumarapuram, NVIDIA  - Tony Zlatinski, NVIDIA  - Yang
//!   Liu, AMD
//!# Related
//! - [`VideoEncodeH264CapabilitiesEXT`]
//! - [`VideoEncodeH264CapabilityFlagBitsEXT`]
//! - [`VideoEncodeH264CapabilityFlagsEXT`]
//! - [`VideoEncodeH264CreateFlagBitsEXT`]
//! - [`VideoEncodeH264CreateFlagsEXT`]
//! - [`VideoEncodeH264DpbSlotInfoEXT`]
//! - [`VideoEncodeH264EmitPictureParametersEXT`]
//! - [`VideoEncodeH264FrameSizeEXT`]
//! - [`VideoEncodeH264InputModeFlagBitsEXT`]
//! - [`VideoEncodeH264InputModeFlagsEXT`]
//! - [`VideoEncodeH264NaluSliceEXT`]
//! - [`VideoEncodeH264OutputModeFlagBitsEXT`]
//! - [`VideoEncodeH264OutputModeFlagsEXT`]
//! - [`VideoEncodeH264ProfileEXT`]
//! - [`VideoEncodeH264QpEXT`]
//! - [`VideoEncodeH264RateControlInfoEXT`]
//! - [`VideoEncodeH264RateControlLayerInfoEXT`]
//! - [`VideoEncodeH264RateControlStructureFlagBitsEXT`]
//! - [`VideoEncodeH264RateControlStructureFlagsEXT`]
//! - [`VideoEncodeH264ReferenceListsEXT`]
//! - [`VideoEncodeH264SessionCreateInfoEXT`]
//! - [`VideoEncodeH264SessionParametersAddInfoEXT`]
//! - [`VideoEncodeH264SessionParametersCreateInfoEXT`]
//! - [`VideoEncodeH264VclFrameInfoEXT`]
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
        StdVideoEncodeH264PictureInfo, StdVideoEncodeH264RefMemMgmtCtrlOperations, StdVideoEncodeH264ReferenceInfo,
        StdVideoEncodeH264SliceHeader, StdVideoH264PictureParameterSet, StdVideoH264ProfileIdc,
        StdVideoH264SequenceParameterSet,
    },
    vulkan1_0::{BaseInStructure, Bool32, ExtensionProperties, Extent2D, StructureType},
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
#[doc(alias = "VK_EXT_VIDEO_ENCODE_H264_SPEC_VERSION")]
pub const EXT_VIDEO_ENCODE_H264_SPEC_VERSION: u32 = 5;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_VIDEO_ENCODE_H264_EXTENSION_NAME")]
pub const EXT_VIDEO_ENCODE_H264_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_video_encode_h264");
///[VkVideoEncodeH264CapabilityFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264CapabilityFlagBitsEXT.html) - Video encode H.264 capability flags
///# C Specifications
///Bits which  **may**  be set in
///[`VideoEncodeH264CapabilitiesEXT::flags`], indicating the encoding
///tools supported, are:
///```c
///// Provided by VK_EXT_video_encode_h264
///typedef enum VkVideoEncodeH264CapabilityFlagBitsEXT {
///    VK_VIDEO_ENCODE_H264_CAPABILITY_DIRECT_8X8_INFERENCE_BIT_EXT = 0x00000001,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_SEPARATE_COLOUR_PLANE_BIT_EXT = 0x00000002,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_QPPRIME_Y_ZERO_TRANSFORM_BYPASS_BIT_EXT = 0x00000004,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_SCALING_LISTS_BIT_EXT = 0x00000008,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_HRD_COMPLIANCE_BIT_EXT = 0x00000010,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_CHROMA_QP_OFFSET_BIT_EXT = 0x00000020,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_SECOND_CHROMA_QP_OFFSET_BIT_EXT = 0x00000040,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_PIC_INIT_QP_MINUS26_BIT_EXT = 0x00000080,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_WEIGHTED_PRED_BIT_EXT = 0x00000100,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_WEIGHTED_BIPRED_EXPLICIT_BIT_EXT = 0x00000200,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_WEIGHTED_BIPRED_IMPLICIT_BIT_EXT = 0x00000400,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_WEIGHTED_PRED_NO_TABLE_BIT_EXT = 0x00000800,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_TRANSFORM_8X8_BIT_EXT = 0x00001000,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_CABAC_BIT_EXT = 0x00002000,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_CAVLC_BIT_EXT = 0x00004000,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_DEBLOCKING_FILTER_DISABLED_BIT_EXT = 0x00008000,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_DEBLOCKING_FILTER_ENABLED_BIT_EXT = 0x00010000,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_DEBLOCKING_FILTER_PARTIAL_BIT_EXT = 0x00020000,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_DISABLE_DIRECT_SPATIAL_MV_PRED_BIT_EXT = 0x00040000,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_MULTIPLE_SLICE_PER_FRAME_BIT_EXT = 0x00080000,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_SLICE_MB_COUNT_BIT_EXT = 0x00100000,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_ROW_UNALIGNED_SLICE_BIT_EXT = 0x00200000,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_DIFFERENT_SLICE_TYPE_BIT_EXT = 0x00400000,
///} VkVideoEncodeH264CapabilityFlagBitsEXT;
///```
///# Description
/// - [`VideoEncodeH264CapabilityDirect8X8InferenceExt`] reports if enabling
///   direct_8x8_inference_flag in StdVideoH264SpsFlags is supported.
/// - [`VideoEncodeH264CapabilitySeparateColourPlaneExt`] reports if enabling
///   separate_colour_plane_flag in StdVideoH264SpsFlags is supported.
/// - [`VideoEncodeH264CapabilityQpprimeYZeroTransformBypassExt`] reports if enabling
///   qpprime_y_zero_transform_bypass_flag in StdVideoH264SpsFlags is supported.
/// - [`VideoEncodeH264CapabilityScalingListsExt`] reports if enabling
///   seq_scaling_matrix_present_flag in StdVideoH264SpsFlags or pic_scaling_matrix_present_flag in
///   StdVideoH264PpsFlags are supported.
/// - [`VideoEncodeH264CapabilityHrdComplianceExt`] reports if the implementation guarantees
///   generating a HRD compliant bitstream if nal_hrd_parameters_present_flag or
///   vcl_hrd_parameters_present_flag are enabled in StdVideoH264SpsVuiFlags.
/// - [`VideoEncodeH264CapabilityChromaQpOffsetExt`] reports if setting non-zero
///   chroma_qp_index_offset in StdVideoH264PictureParameterSet is supported.
/// - [`VideoEncodeH264CapabilitySecondChromaQpOffsetExt`] reports if setting non-zero
///   second_chroma_qp_index_offset in StdVideoH264PictureParameterSet is supported.
/// - [`VideoEncodeH264CapabilityPicInitQpMinus26Ext`] reports if setting non-zero
///   pic_init_qp_minus26 in StdVideoH264PictureParameterSet is supported.
/// - [`VideoEncodeH264CapabilityWeightedPredExt`] reports if enabling weighted_pred_flag in
///   StdVideoH264PpsFlags is supported.
/// - [`VideoEncodeH264CapabilityWeightedBipredExplicitExt`] reports if using
///   STD_VIDEO_H264_WEIGHTED_BIPRED_IDC_EXPLICIT from StdVideoH264WeightedBipredIdc is supported.
/// - [`VideoEncodeH264CapabilityWeightedBipredImplicitExt`] reports if using
///   STD_VIDEO_H264_WEIGHTED_BIPRED_IDC_IMPLICIT from StdVideoH264WeightedBipredIdc is supported.
/// - [`VideoEncodeH264CapabilityWeightedPredNoTableExt`] reports that when weighted_pred_flag is
///   enabled or STD_VIDEO_H264_WEIGHTED_BIPRED_IDC_EXPLICIT from StdVideoH264WeightedBipredIdc is
///   used, the implementation is able to internally decide syntax for pred_weight_table.
/// - [`VideoEncodeH264CapabilityTransform8X8Ext`] reports if enabling transform_8x8_mode_flag in
///   StdVideoH264PpsFlags is supported.
/// - [`VideoEncodeH264CapabilityCabacExt`] reports if CABAC entropy coding is supported.
/// - [`VideoEncodeH264CapabilityCavlcExt`] reports if CAVLC entropy coding is supported. An
///   implementation  **must**  support at least one entropy coding mode.
/// - [`VideoEncodeH264CapabilityDeblockingFilterDisabledExt`] reports if using
///   STD_VIDEO_H264_DISABLE_DEBLOCKING_FILTER_IDC_DISABLED from
///   StdVideoH264DisableDeblockingFilterIdc is supported.
/// - [`VideoEncodeH264CapabilityDeblockingFilterEnabledExt`] reports if using
///   STD_VIDEO_H264_DISABLE_DEBLOCKING_FILTER_IDC_ENABLED from
///   StdVideoH264DisableDeblockingFilterIdc is supported.
/// - [`VideoEncodeH264CapabilityDeblockingFilterPartialExt`] reports if using
///   STD_VIDEO_H264_DISABLE_DEBLOCKING_FILTER_IDC_PARTIAL from
///   StdVideoH264DisableDeblockingFilterIdc is supported. An implementation  **must**  support at
///   least one deblocking filter mode.
/// - [`VideoEncodeH264CapabilityDisableDirectSpatialMvPredExt`] reports if disabling
///   [`StdVideoEncodeH264SliceHeaderFlags`]::direct_spatial_mv_pred_flag is supported when it is
///   present in the slice header.
/// - [`VideoEncodeH264CapabilityMultipleSlicePerFrameExt`] reports if encoding multiple slices per
///   frame is supported. If not set, the implementation is only able to encode a single slice for
///   the entire frame.
/// - [`VideoEncodeH264CapabilitySliceMbCountExt`] reports support for configuring
///   [`VideoEncodeH264NaluSliceEXT::mb_count`] and first_mb_in_slice in
///   StdVideoEncodeH264SliceHeader for each slice in a frame with multiple slices. If not
///   supported, the implementation decides the number of macroblocks in each slice based on
///   [`VideoEncodeH264VclFrameInfoEXT::nalu_slice_entry_count`].
/// - [`VideoEncodeH264CapabilityRowUnalignedSliceExt`] reports that each slice in a frame with
///   multiple slices may begin or finish at any offset in a macroblock row. If not supported, all
///   slices in the frame  **must**  begin at the start of a macroblock row (and hence each slice
///   **must**  finish at the end of a macroblock row).
/// - [`VideoEncodeH264CapabilityDifferentSliceTypeExt`] reports that when
///   [`VideoEncodeH264CapabilityMultipleSlicePerFrameExt`] is supported and a frame is encoded with
///   multiple slices, the implementation allows encoding each slice with a different
///   [`StdVideoEncodeH264SliceHeader`]::slice_type. If not supported, all slices of the frame
///   **must**  be encoded with the same `slice_type` which corresponds to the picture type of the
///   frame. For example, all slices of a P-frame would be encoded as P-slices.
///# Related
/// - [`VK_EXT_video_encode_h264`]
/// - [`VideoEncodeH264CapabilityFlagsEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoEncodeH264CapabilityFlagBitsEXT")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(u32)]
pub enum VideoEncodeH264CapabilityFlagBitsEXT {
    #[doc(hidden)]
    Empty = 0,
    ///[`VideoEncodeH264CapabilityDirect8X8InferenceExt`]
    ///reports if enabling direct_8x8_inference_flag in StdVideoH264SpsFlags is
    ///supported.
    VideoEncodeH264CapabilityDirect8X8InferenceExt = 1,
    ///[`VideoEncodeH264CapabilitySeparateColourPlaneExt`]
    ///reports if enabling separate_colour_plane_flag in StdVideoH264SpsFlags
    ///is supported.
    VideoEncodeH264CapabilitySeparateColourPlaneExt = 2,
    ///[`VideoEncodeH264CapabilityQpprimeYZeroTransformBypassExt`]
    ///reports if enabling qpprime_y_zero_transform_bypass_flag in
    ///StdVideoH264SpsFlags is supported.
    VideoEncodeH264CapabilityQpprimeYZeroTransformBypassExt = 4,
    ///[`VideoEncodeH264CapabilityScalingListsExt`] reports if
    ///enabling seq_scaling_matrix_present_flag in StdVideoH264SpsFlags or
    ///pic_scaling_matrix_present_flag in StdVideoH264PpsFlags are supported.
    VideoEncodeH264CapabilityScalingListsExt = 8,
    ///[`VideoEncodeH264CapabilityHrdComplianceExt`] reports if
    ///the implementation guarantees generating a HRD compliant bitstream if
    ///nal_hrd_parameters_present_flag or vcl_hrd_parameters_present_flag are
    ///enabled in StdVideoH264SpsVuiFlags.
    VideoEncodeH264CapabilityHrdComplianceExt = 16,
    ///[`VideoEncodeH264CapabilityChromaQpOffsetExt`] reports
    ///if setting non-zero chroma_qp_index_offset in
    ///StdVideoH264PictureParameterSet is supported.
    VideoEncodeH264CapabilityChromaQpOffsetExt = 32,
    ///[`VideoEncodeH264CapabilitySecondChromaQpOffsetExt`]
    ///reports if setting non-zero second_chroma_qp_index_offset in
    ///StdVideoH264PictureParameterSet is supported.
    VideoEncodeH264CapabilitySecondChromaQpOffsetExt = 64,
    ///[`VideoEncodeH264CapabilityPicInitQpMinus26Ext`]
    ///reports if setting non-zero pic_init_qp_minus26 in
    ///StdVideoH264PictureParameterSet is supported.
    VideoEncodeH264CapabilityPicInitQpMinus26Ext = 128,
    ///[`VideoEncodeH264CapabilityWeightedPredExt`] reports if
    ///enabling weighted_pred_flag in StdVideoH264PpsFlags is supported.
    VideoEncodeH264CapabilityWeightedPredExt = 256,
    ///[`VideoEncodeH264CapabilityWeightedBipredExplicitExt`]
    ///reports if using STD_VIDEO_H264_WEIGHTED_BIPRED_IDC_EXPLICIT from
    ///StdVideoH264WeightedBipredIdc is supported.
    VideoEncodeH264CapabilityWeightedBipredExplicitExt = 512,
    ///[`VideoEncodeH264CapabilityWeightedBipredImplicitExt`]
    ///reports if using STD_VIDEO_H264_WEIGHTED_BIPRED_IDC_IMPLICIT from
    ///StdVideoH264WeightedBipredIdc is supported.
    VideoEncodeH264CapabilityWeightedBipredImplicitExt = 1024,
    ///[`VideoEncodeH264CapabilityWeightedPredNoTableExt`]
    ///reports that when weighted_pred_flag is enabled or
    ///STD_VIDEO_H264_WEIGHTED_BIPRED_IDC_EXPLICIT from
    ///StdVideoH264WeightedBipredIdc is used, the implementation is able to
    ///internally decide syntax for pred_weight_table.
    VideoEncodeH264CapabilityWeightedPredNoTableExt = 2048,
    ///[`VideoEncodeH264CapabilityTransform8X8Ext`] reports if
    ///enabling transform_8x8_mode_flag in StdVideoH264PpsFlags is supported.
    VideoEncodeH264CapabilityTransform8X8Ext = 4096,
    ///[`VideoEncodeH264CapabilityCabacExt`] reports if CABAC
    ///entropy coding is supported.
    VideoEncodeH264CapabilityCabacExt = 8192,
    ///[`VideoEncodeH264CapabilityCavlcExt`] reports if CAVLC
    ///entropy coding is supported.
    ///An implementation  **must**  support at least one entropy coding mode.
    VideoEncodeH264CapabilityCavlcExt = 16384,
    ///[`VideoEncodeH264CapabilityDeblockingFilterDisabledExt`]
    ///reports if using STD_VIDEO_H264_DISABLE_DEBLOCKING_FILTER_IDC_DISABLED
    ///from StdVideoH264DisableDeblockingFilterIdc is supported.
    VideoEncodeH264CapabilityDeblockingFilterDisabledExt = 32768,
    ///[`VideoEncodeH264CapabilityDeblockingFilterEnabledExt`]
    ///reports if using STD_VIDEO_H264_DISABLE_DEBLOCKING_FILTER_IDC_ENABLED
    ///from StdVideoH264DisableDeblockingFilterIdc is supported.
    VideoEncodeH264CapabilityDeblockingFilterEnabledExt = 65536,
    ///[`VideoEncodeH264CapabilityDeblockingFilterPartialExt`]
    ///reports if using STD_VIDEO_H264_DISABLE_DEBLOCKING_FILTER_IDC_PARTIAL
    ///from StdVideoH264DisableDeblockingFilterIdc is supported.
    ///An implementation  **must**  support at least one deblocking filter mode.
    VideoEncodeH264CapabilityDeblockingFilterPartialExt = 131072,
    ///[`VideoEncodeH264CapabilityDisableDirectSpatialMvPredExt`]
    ///reports if disabling
    ///[`StdVideoEncodeH264SliceHeaderFlags`]::direct_spatial_mv_pred_flag is
    ///supported when it is present in the slice header.
    VideoEncodeH264CapabilityDisableDirectSpatialMvPredExt = 262144,
    ///[`VideoEncodeH264CapabilityMultipleSlicePerFrameExt`]
    ///reports if encoding multiple slices per frame is supported.
    ///If not set, the implementation is only able to encode a single slice for
    ///the entire frame.
    VideoEncodeH264CapabilityMultipleSlicePerFrameExt = 524288,
    ///[`VideoEncodeH264CapabilitySliceMbCountExt`] reports
    ///support for configuring
    ///[`VideoEncodeH264NaluSliceEXT`]::`mbCount` and first_mb_in_slice
    ///in StdVideoEncodeH264SliceHeader for each slice in a frame with multiple
    ///slices.
    ///If not supported, the implementation decides the number of macroblocks
    ///in each slice based on
    ///[`VideoEncodeH264VclFrameInfoEXT`]::`naluSliceEntryCount`.
    VideoEncodeH264CapabilitySliceMbCountExt = 1048576,
    ///[`VideoEncodeH264CapabilityRowUnalignedSliceExt`]
    ///reports that each slice in a frame with multiple slices may begin or
    ///finish at any offset in a macroblock row.
    ///If not supported, all slices in the frame  **must**  begin at the start of a
    ///macroblock row (and hence each slice  **must**  finish at the end of a
    ///macroblock row).
    VideoEncodeH264CapabilityRowUnalignedSliceExt = 2097152,
    ///[`VideoEncodeH264CapabilityDifferentSliceTypeExt`]
    ///reports that when
    ///[`VideoEncodeH264CapabilityMultipleSlicePerFrameExt`]
    ///is supported and a frame is encoded with multiple slices, the
    ///implementation allows encoding each slice with a different
    ///[`StdVideoEncodeH264SliceHeader`]::slice_type.
    ///If not supported, all slices of the frame  **must**  be encoded with the same
    ///`slice_type` which corresponds to the picture type of the frame.
    ///For example, all slices of a P-frame would be encoded as P-slices.
    VideoEncodeH264CapabilityDifferentSliceTypeExt = 4194304,
}
impl const Default for VideoEncodeH264CapabilityFlagBitsEXT {
    fn default() -> Self {
        Self::Empty
    }
}
impl VideoEncodeH264CapabilityFlagBitsEXT {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> u32 {
        *self as u32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: u32) -> u32 {
        std::mem::transmute(bits)
    }
}
///[VkVideoEncodeH264InputModeFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264InputModeFlagBitsEXT.html) - Video encode H.264 input modes
///# C Specifications
///The `inputModeFlags` field reports the various commmand buffer input
///granularities supported by the implementation as follows:
///```c
///// Provided by VK_EXT_video_encode_h264
///typedef enum VkVideoEncodeH264InputModeFlagBitsEXT {
///    VK_VIDEO_ENCODE_H264_INPUT_MODE_FRAME_BIT_EXT = 0x00000001,
///    VK_VIDEO_ENCODE_H264_INPUT_MODE_SLICE_BIT_EXT = 0x00000002,
///    VK_VIDEO_ENCODE_H264_INPUT_MODE_NON_VCL_BIT_EXT = 0x00000004,
///} VkVideoEncodeH264InputModeFlagBitsEXT;
///```
///# Description
/// - [`VideoEncodeH264InputModeFrameExt`] indicates that a single command buffer  **must**  at
///   least encode an entire frame. Any non-VCL NALUs  **must**  be encoded using the same command
///   buffer as the frame if [`VideoEncodeH264InputModeNonVclExt`] is not supported.
/// - [`VideoEncodeH264InputModeSliceExt`] indicates that a single command buffer  **must**  at
///   least encode a single slice. Any non-VCL NALUs  **must**  be encoded using the same command
///   buffer as the first slice of the frame if [`VideoEncodeH264InputModeNonVclExt`] is not
///   supported.
/// - [`VideoEncodeH264InputModeNonVclExt`] indicates that a single command buffer  **may**  encode
///   a non-VCL NALU by itself.
///An implementation  **must**  support at least one of
///[`VideoEncodeH264InputModeFrameExt`] or
///[`VideoEncodeH264InputModeSliceExt`].If [`VideoEncodeH264InputModeSliceExt`] is not supported,
/// the
///following two additional restrictions apply for frames encoded with multiple
///slices.
///First, all frame slices  **must**  have the same pRefList0ModOperations and the
///same pRefList1ModOperations.
///Second, the order in which slices appear in
///[`VideoEncodeH264VclFrameInfoEXT::nalu_slice_entries`] or in the
///command buffer  **must**  match the placement order of the slices in the frame.
///# Related
/// - [`VK_EXT_video_encode_h264`]
/// - [`VideoEncodeH264InputModeFlagsEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoEncodeH264InputModeFlagBitsEXT")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(u32)]
pub enum VideoEncodeH264InputModeFlagBitsEXT {
    #[doc(hidden)]
    Empty = 0,
    ///[`VideoEncodeH264InputModeFrameExt`] indicates that a
    ///single command buffer  **must**  at least encode an entire frame.
    ///Any non-VCL NALUs  **must**  be encoded using the same command buffer as the
    ///frame if [`VideoEncodeH264InputModeNonVclExt`] is not
    ///supported.
    VideoEncodeH264InputModeFrameExt = 1,
    ///[`VideoEncodeH264InputModeSliceExt`] indicates that a
    ///single command buffer  **must**  at least encode a single slice.
    ///Any non-VCL NALUs  **must**  be encoded using the same command buffer as the
    ///first slice of the frame if
    ///[`VideoEncodeH264InputModeNonVclExt`] is not supported.
    VideoEncodeH264InputModeSliceExt = 2,
    ///[`VideoEncodeH264InputModeNonVclExt`] indicates that a
    ///single command buffer  **may**  encode a non-VCL NALU by itself.
    VideoEncodeH264InputModeNonVclExt = 4,
}
impl const Default for VideoEncodeH264InputModeFlagBitsEXT {
    fn default() -> Self {
        Self::Empty
    }
}
impl VideoEncodeH264InputModeFlagBitsEXT {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> u32 {
        *self as u32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: u32) -> u32 {
        std::mem::transmute(bits)
    }
}
///[VkVideoEncodeH264OutputModeFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264OutputModeFlagBitsEXT.html) - Video encode H.264 output modes
///# C Specifications
///Bits which  **may**  be set in
///[`VideoEncodeH264CapabilitiesEXT::output_mode_flags`], indicating
///the minimum bitstream generation commands that  **must**  be included between
///each [`CmdBeginVideoCodingKHR`] and [`CmdEndVideoCodingKHR`] pair
///(henceforth simply begin/end pair), are:
///```c
///// Provided by VK_EXT_video_encode_h264
///typedef enum VkVideoEncodeH264OutputModeFlagBitsEXT {
///    VK_VIDEO_ENCODE_H264_OUTPUT_MODE_FRAME_BIT_EXT = 0x00000001,
///    VK_VIDEO_ENCODE_H264_OUTPUT_MODE_SLICE_BIT_EXT = 0x00000002,
///    VK_VIDEO_ENCODE_H264_OUTPUT_MODE_NON_VCL_BIT_EXT = 0x00000004,
///} VkVideoEncodeH264OutputModeFlagBitsEXT;
///```
///# Description
/// - [`VideoEncodeH264OutputModeFrameExt`] indicates that calls to generate all NALUs of a frame
///   **must**  be included within a single begin/end pair. Any non-VCL NALUs  **must**  be encoded
///   within the same begin/end pair if [`VideoEncodeH264OutputModeNonVclExt`] is not supported.
/// - [`VideoEncodeH264OutputModeSliceExt`] indicates that each begin/end pair  **must**  encode at
///   least one slice. Any non-VCL NALUs  **must**  be encoded within the same begin/end pair as the
///   first slice of the frame if [`VideoEncodeH264OutputModeNonVclExt`] is not supported.
/// - [`VideoEncodeH264OutputModeNonVclExt`] indicates that each begin/end pair  **may**  encode
///   only a non-VCL NALU by itself. An implementation  **must**  support at least one of
///   [`VideoEncodeH264OutputModeFrameExt`] or [`VideoEncodeH264OutputModeSliceExt`].
///A single begin/end pair  **must**  not encode more than a single frame.The bitstreams of NALUs
/// generated within a single begin/end pair are written
///continuously into the same bitstream buffer (any padding between the NALUs
/// **must**  be compliant to the H.264 standard).The supported input modes  **must**  be coarser or
/// equal to the supported output
///modes.
///For example, it is illegal to report slice input is supported but only frame
///output is supported.An implementation  **must**  report one of the following combinations of
///input/output modes:
/// - Input: Frame, Output: Frame
/// - Input: Frame, Output: Frame and Non-VCL
/// - Input: Frame, Output: Slice
/// - Input: Frame, Output: Slice and Non-VCL
/// - Input: Slice, Output: Slice
/// - Input: Slice, Output: Slice and Non-VCL
/// - Input: Frame and Non-VCL, Output: Frame and Non-VCL
/// - Input: Frame and Non-VCL, Output: Slice and Non-VCL
/// - Input: Slice and Non-VCL, Output: Slice and Non-VCL
///# Related
/// - [`VK_EXT_video_encode_h264`]
/// - [`VideoEncodeH264OutputModeFlagsEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoEncodeH264OutputModeFlagBitsEXT")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(u32)]
pub enum VideoEncodeH264OutputModeFlagBitsEXT {
    #[doc(hidden)]
    Empty = 0,
    ///[`VideoEncodeH264OutputModeFrameExt`] indicates that
    ///calls to generate all NALUs of a frame  **must**  be included within a single
    ///begin/end pair.
    ///Any non-VCL NALUs  **must**  be encoded within the same begin/end pair if
    ///[`VideoEncodeH264OutputModeNonVclExt`] is not supported.
    VideoEncodeH264OutputModeFrameExt = 1,
    ///[`VideoEncodeH264OutputModeSliceExt`] indicates that each
    ///begin/end pair  **must**  encode at least one slice.
    ///Any non-VCL NALUs  **must**  be encoded within the same begin/end pair as the
    ///first slice of the frame if
    ///[`VideoEncodeH264OutputModeNonVclExt`] is not supported.
    VideoEncodeH264OutputModeSliceExt = 2,
    ///[`VideoEncodeH264OutputModeNonVclExt`] indicates that
    ///each begin/end pair  **may**  encode only a non-VCL NALU by itself.
    ///An implementation  **must**  support at least one of
    ///[`VideoEncodeH264OutputModeFrameExt`] or
    ///[`VideoEncodeH264OutputModeSliceExt`].
    VideoEncodeH264OutputModeNonVclExt = 4,
}
impl const Default for VideoEncodeH264OutputModeFlagBitsEXT {
    fn default() -> Self {
        Self::Empty
    }
}
impl VideoEncodeH264OutputModeFlagBitsEXT {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> u32 {
        *self as u32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: u32) -> u32 {
        std::mem::transmute(bits)
    }
}
///[VkVideoEncodeH264CreateFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264CreateFlagBitsEXT.html) - Video encode session creation flags
///# C Specifications
///Bits which  **can**  be set in
///[`VideoEncodeH264SessionCreateInfoEXT::flags`] are:
///```c
///// Provided by VK_EXT_video_encode_h264
///typedef enum VkVideoEncodeH264CreateFlagBitsEXT {
///    VK_VIDEO_ENCODE_H264_CREATE_DEFAULT_EXT = 0,
///    VK_VIDEO_ENCODE_H264_CREATE_RESERVED_0_BIT_EXT = 0x00000001,
///} VkVideoEncodeH264CreateFlagBitsEXT;
///```
///# Description
/// - [`VideoEncodeH264CreateDefaultExt`] is 0, and specifies no additional creation flags.
/// - [`VideoEncodeH264CreateReserved0Ext`] The current version of the specification has reserved
///   this value for future use.
///# Related
/// - [`VK_EXT_video_encode_h264`]
/// - [`VideoEncodeH264CreateFlagsEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoEncodeH264CreateFlagBitsEXT")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(u32)]
pub enum VideoEncodeH264CreateFlagBitsEXT {
    ///[`VideoEncodeH264CreateDefaultExt`] is 0, and specifies no
    ///additional creation flags.
    VideoEncodeH264CreateDefaultExt = 0,
    ///[`VideoEncodeH264CreateReserved0Ext`] The current version
    ///of the specification has reserved this value for future use.
    VideoEncodeH264CreateReserved0Ext = 1,
}
impl const Default for VideoEncodeH264CreateFlagBitsEXT {
    fn default() -> Self {
        Self::VideoEncodeH264CreateDefaultExt
    }
}
impl VideoEncodeH264CreateFlagBitsEXT {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> u32 {
        *self as u32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: u32) -> u32 {
        std::mem::transmute(bits)
    }
}
///[VkVideoEncodeH264RateControlStructureFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264RateControlStructureFlagBitsEXT.html) - Video encode H.264 rate control structure flags
///# C Specifications
///The `rateControlStructure` in [`VideoEncodeH264RateControlInfoEXT`]
///specifies one of the following video stream reference structures as a hint
///for the rate control implementation:
///```c
///// Provided by VK_EXT_video_encode_h264
///typedef enum VkVideoEncodeH264RateControlStructureFlagBitsEXT {
///    VK_VIDEO_ENCODE_H264_RATE_CONTROL_STRUCTURE_UNKNOWN_EXT = 0,
///    VK_VIDEO_ENCODE_H264_RATE_CONTROL_STRUCTURE_FLAT_BIT_EXT = 0x00000001,
///    VK_VIDEO_ENCODE_H264_RATE_CONTROL_STRUCTURE_DYADIC_BIT_EXT = 0x00000002,
///} VkVideoEncodeH264RateControlStructureFlagBitsEXT;
///```
///# Description
///```c specifies a dyadic reference structure.
///```
///# Related
/// - [`VK_EXT_video_encode_h264`]
/// - [`VideoEncodeH264RateControlInfoEXT`]
/// - [`VideoEncodeH264RateControlStructureFlagsEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoEncodeH264RateControlStructureFlagBitsEXT")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(u32)]
pub enum VideoEncodeH264RateControlStructureFlagBitsEXT {
    ///No documentation found
    VideoEncodeH264RateControlStructureUnknownExt = 0,
    ///No documentation found
    VideoEncodeH264RateControlStructureFlatExt = 1,
    ///No documentation found
    VideoEncodeH264RateControlStructureDyadicExt = 2,
}
impl const Default for VideoEncodeH264RateControlStructureFlagBitsEXT {
    fn default() -> Self {
        Self::VideoEncodeH264RateControlStructureUnknownExt
    }
}
impl VideoEncodeH264RateControlStructureFlagBitsEXT {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> u32 {
        *self as u32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: u32) -> u32 {
        std::mem::transmute(bits)
    }
}
///[VkVideoEncodeH264CapabilityFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264CapabilityFlagBitsEXT.html) - Video encode H.264 capability flags
///# C Specifications
///Bits which  **may**  be set in
///[`VideoEncodeH264CapabilitiesEXT::flags`], indicating the encoding
///tools supported, are:
///```c
///// Provided by VK_EXT_video_encode_h264
///typedef enum VkVideoEncodeH264CapabilityFlagBitsEXT {
///    VK_VIDEO_ENCODE_H264_CAPABILITY_DIRECT_8X8_INFERENCE_BIT_EXT = 0x00000001,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_SEPARATE_COLOUR_PLANE_BIT_EXT = 0x00000002,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_QPPRIME_Y_ZERO_TRANSFORM_BYPASS_BIT_EXT = 0x00000004,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_SCALING_LISTS_BIT_EXT = 0x00000008,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_HRD_COMPLIANCE_BIT_EXT = 0x00000010,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_CHROMA_QP_OFFSET_BIT_EXT = 0x00000020,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_SECOND_CHROMA_QP_OFFSET_BIT_EXT = 0x00000040,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_PIC_INIT_QP_MINUS26_BIT_EXT = 0x00000080,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_WEIGHTED_PRED_BIT_EXT = 0x00000100,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_WEIGHTED_BIPRED_EXPLICIT_BIT_EXT = 0x00000200,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_WEIGHTED_BIPRED_IMPLICIT_BIT_EXT = 0x00000400,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_WEIGHTED_PRED_NO_TABLE_BIT_EXT = 0x00000800,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_TRANSFORM_8X8_BIT_EXT = 0x00001000,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_CABAC_BIT_EXT = 0x00002000,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_CAVLC_BIT_EXT = 0x00004000,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_DEBLOCKING_FILTER_DISABLED_BIT_EXT = 0x00008000,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_DEBLOCKING_FILTER_ENABLED_BIT_EXT = 0x00010000,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_DEBLOCKING_FILTER_PARTIAL_BIT_EXT = 0x00020000,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_DISABLE_DIRECT_SPATIAL_MV_PRED_BIT_EXT = 0x00040000,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_MULTIPLE_SLICE_PER_FRAME_BIT_EXT = 0x00080000,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_SLICE_MB_COUNT_BIT_EXT = 0x00100000,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_ROW_UNALIGNED_SLICE_BIT_EXT = 0x00200000,
///    VK_VIDEO_ENCODE_H264_CAPABILITY_DIFFERENT_SLICE_TYPE_BIT_EXT = 0x00400000,
///} VkVideoEncodeH264CapabilityFlagBitsEXT;
///```
///# Description
/// - [`VideoEncodeH264CapabilityDirect8X8InferenceExt`] reports if enabling
///   direct_8x8_inference_flag in StdVideoH264SpsFlags is supported.
/// - [`VideoEncodeH264CapabilitySeparateColourPlaneExt`] reports if enabling
///   separate_colour_plane_flag in StdVideoH264SpsFlags is supported.
/// - [`VideoEncodeH264CapabilityQpprimeYZeroTransformBypassExt`] reports if enabling
///   qpprime_y_zero_transform_bypass_flag in StdVideoH264SpsFlags is supported.
/// - [`VideoEncodeH264CapabilityScalingListsExt`] reports if enabling
///   seq_scaling_matrix_present_flag in StdVideoH264SpsFlags or pic_scaling_matrix_present_flag in
///   StdVideoH264PpsFlags are supported.
/// - [`VideoEncodeH264CapabilityHrdComplianceExt`] reports if the implementation guarantees
///   generating a HRD compliant bitstream if nal_hrd_parameters_present_flag or
///   vcl_hrd_parameters_present_flag are enabled in StdVideoH264SpsVuiFlags.
/// - [`VideoEncodeH264CapabilityChromaQpOffsetExt`] reports if setting non-zero
///   chroma_qp_index_offset in StdVideoH264PictureParameterSet is supported.
/// - [`VideoEncodeH264CapabilitySecondChromaQpOffsetExt`] reports if setting non-zero
///   second_chroma_qp_index_offset in StdVideoH264PictureParameterSet is supported.
/// - [`VideoEncodeH264CapabilityPicInitQpMinus26Ext`] reports if setting non-zero
///   pic_init_qp_minus26 in StdVideoH264PictureParameterSet is supported.
/// - [`VideoEncodeH264CapabilityWeightedPredExt`] reports if enabling weighted_pred_flag in
///   StdVideoH264PpsFlags is supported.
/// - [`VideoEncodeH264CapabilityWeightedBipredExplicitExt`] reports if using
///   STD_VIDEO_H264_WEIGHTED_BIPRED_IDC_EXPLICIT from StdVideoH264WeightedBipredIdc is supported.
/// - [`VideoEncodeH264CapabilityWeightedBipredImplicitExt`] reports if using
///   STD_VIDEO_H264_WEIGHTED_BIPRED_IDC_IMPLICIT from StdVideoH264WeightedBipredIdc is supported.
/// - [`VideoEncodeH264CapabilityWeightedPredNoTableExt`] reports that when weighted_pred_flag is
///   enabled or STD_VIDEO_H264_WEIGHTED_BIPRED_IDC_EXPLICIT from StdVideoH264WeightedBipredIdc is
///   used, the implementation is able to internally decide syntax for pred_weight_table.
/// - [`VideoEncodeH264CapabilityTransform8X8Ext`] reports if enabling transform_8x8_mode_flag in
///   StdVideoH264PpsFlags is supported.
/// - [`VideoEncodeH264CapabilityCabacExt`] reports if CABAC entropy coding is supported.
/// - [`VideoEncodeH264CapabilityCavlcExt`] reports if CAVLC entropy coding is supported. An
///   implementation  **must**  support at least one entropy coding mode.
/// - [`VideoEncodeH264CapabilityDeblockingFilterDisabledExt`] reports if using
///   STD_VIDEO_H264_DISABLE_DEBLOCKING_FILTER_IDC_DISABLED from
///   StdVideoH264DisableDeblockingFilterIdc is supported.
/// - [`VideoEncodeH264CapabilityDeblockingFilterEnabledExt`] reports if using
///   STD_VIDEO_H264_DISABLE_DEBLOCKING_FILTER_IDC_ENABLED from
///   StdVideoH264DisableDeblockingFilterIdc is supported.
/// - [`VideoEncodeH264CapabilityDeblockingFilterPartialExt`] reports if using
///   STD_VIDEO_H264_DISABLE_DEBLOCKING_FILTER_IDC_PARTIAL from
///   StdVideoH264DisableDeblockingFilterIdc is supported. An implementation  **must**  support at
///   least one deblocking filter mode.
/// - [`VideoEncodeH264CapabilityDisableDirectSpatialMvPredExt`] reports if disabling
///   [`StdVideoEncodeH264SliceHeaderFlags`]::direct_spatial_mv_pred_flag is supported when it is
///   present in the slice header.
/// - [`VideoEncodeH264CapabilityMultipleSlicePerFrameExt`] reports if encoding multiple slices per
///   frame is supported. If not set, the implementation is only able to encode a single slice for
///   the entire frame.
/// - [`VideoEncodeH264CapabilitySliceMbCountExt`] reports support for configuring
///   [`VideoEncodeH264NaluSliceEXT::mb_count`] and first_mb_in_slice in
///   StdVideoEncodeH264SliceHeader for each slice in a frame with multiple slices. If not
///   supported, the implementation decides the number of macroblocks in each slice based on
///   [`VideoEncodeH264VclFrameInfoEXT::nalu_slice_entry_count`].
/// - [`VideoEncodeH264CapabilityRowUnalignedSliceExt`] reports that each slice in a frame with
///   multiple slices may begin or finish at any offset in a macroblock row. If not supported, all
///   slices in the frame  **must**  begin at the start of a macroblock row (and hence each slice
///   **must**  finish at the end of a macroblock row).
/// - [`VideoEncodeH264CapabilityDifferentSliceTypeExt`] reports that when
///   [`VideoEncodeH264CapabilityMultipleSlicePerFrameExt`] is supported and a frame is encoded with
///   multiple slices, the implementation allows encoding each slice with a different
///   [`StdVideoEncodeH264SliceHeader`]::slice_type. If not supported, all slices of the frame
///   **must**  be encoded with the same `slice_type` which corresponds to the picture type of the
///   frame. For example, all slices of a P-frame would be encoded as P-slices.
///# Related
/// - [`VK_EXT_video_encode_h264`]
/// - [`VideoEncodeH264CapabilityFlagsEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoEncodeH264CapabilityFlagsEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoEncodeH264CapabilityFlagsEXT(u32);
impl const Default for VideoEncodeH264CapabilityFlagsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl From<VideoEncodeH264CapabilityFlagBitsEXT> for VideoEncodeH264CapabilityFlagsEXT {
    fn from(from: VideoEncodeH264CapabilityFlagBitsEXT) -> Self {
        unsafe { Self::from_bits_unchecked(from as u32) }
    }
}
impl VideoEncodeH264CapabilityFlagsEXT {
    ///[`VideoEncodeH264CapabilityDirect8X8InferenceExt`]
    ///reports if enabling direct_8x8_inference_flag in StdVideoH264SpsFlags is
    ///supported.
    pub const VIDEO_ENCODE_H_264_CAPABILITY_DIRECT_8_X_8_INFERENCE_EXT: Self = Self(1);
    ///[`VideoEncodeH264CapabilitySeparateColourPlaneExt`]
    ///reports if enabling separate_colour_plane_flag in StdVideoH264SpsFlags
    ///is supported.
    pub const VIDEO_ENCODE_H_264_CAPABILITY_SEPARATE_COLOUR_PLANE_EXT: Self = Self(2);
    ///[`VideoEncodeH264CapabilityQpprimeYZeroTransformBypassExt`]
    ///reports if enabling qpprime_y_zero_transform_bypass_flag in
    ///StdVideoH264SpsFlags is supported.
    pub const VIDEO_ENCODE_H_264_CAPABILITY_QPPRIME_Y_ZERO_TRANSFORM_BYPASS_EXT: Self = Self(4);
    ///[`VideoEncodeH264CapabilityScalingListsExt`] reports if
    ///enabling seq_scaling_matrix_present_flag in StdVideoH264SpsFlags or
    ///pic_scaling_matrix_present_flag in StdVideoH264PpsFlags are supported.
    pub const VIDEO_ENCODE_H_264_CAPABILITY_SCALING_LISTS_EXT: Self = Self(8);
    ///[`VideoEncodeH264CapabilityHrdComplianceExt`] reports if
    ///the implementation guarantees generating a HRD compliant bitstream if
    ///nal_hrd_parameters_present_flag or vcl_hrd_parameters_present_flag are
    ///enabled in StdVideoH264SpsVuiFlags.
    pub const VIDEO_ENCODE_H_264_CAPABILITY_HRD_COMPLIANCE_EXT: Self = Self(16);
    ///[`VideoEncodeH264CapabilityChromaQpOffsetExt`] reports
    ///if setting non-zero chroma_qp_index_offset in
    ///StdVideoH264PictureParameterSet is supported.
    pub const VIDEO_ENCODE_H_264_CAPABILITY_CHROMA_QP_OFFSET_EXT: Self = Self(32);
    ///[`VideoEncodeH264CapabilitySecondChromaQpOffsetExt`]
    ///reports if setting non-zero second_chroma_qp_index_offset in
    ///StdVideoH264PictureParameterSet is supported.
    pub const VIDEO_ENCODE_H_264_CAPABILITY_SECOND_CHROMA_QP_OFFSET_EXT: Self = Self(64);
    ///[`VideoEncodeH264CapabilityPicInitQpMinus26Ext`]
    ///reports if setting non-zero pic_init_qp_minus26 in
    ///StdVideoH264PictureParameterSet is supported.
    pub const VIDEO_ENCODE_H_264_CAPABILITY_PIC_INIT_QP_MINUS_26_EXT: Self = Self(128);
    ///[`VideoEncodeH264CapabilityWeightedPredExt`] reports if
    ///enabling weighted_pred_flag in StdVideoH264PpsFlags is supported.
    pub const VIDEO_ENCODE_H_264_CAPABILITY_WEIGHTED_PRED_EXT: Self = Self(256);
    ///[`VideoEncodeH264CapabilityWeightedBipredExplicitExt`]
    ///reports if using STD_VIDEO_H264_WEIGHTED_BIPRED_IDC_EXPLICIT from
    ///StdVideoH264WeightedBipredIdc is supported.
    pub const VIDEO_ENCODE_H_264_CAPABILITY_WEIGHTED_BIPRED_EXPLICIT_EXT: Self = Self(512);
    ///[`VideoEncodeH264CapabilityWeightedBipredImplicitExt`]
    ///reports if using STD_VIDEO_H264_WEIGHTED_BIPRED_IDC_IMPLICIT from
    ///StdVideoH264WeightedBipredIdc is supported.
    pub const VIDEO_ENCODE_H_264_CAPABILITY_WEIGHTED_BIPRED_IMPLICIT_EXT: Self = Self(1024);
    ///[`VideoEncodeH264CapabilityWeightedPredNoTableExt`]
    ///reports that when weighted_pred_flag is enabled or
    ///STD_VIDEO_H264_WEIGHTED_BIPRED_IDC_EXPLICIT from
    ///StdVideoH264WeightedBipredIdc is used, the implementation is able to
    ///internally decide syntax for pred_weight_table.
    pub const VIDEO_ENCODE_H_264_CAPABILITY_WEIGHTED_PRED_NO_TABLE_EXT: Self = Self(2048);
    ///[`VideoEncodeH264CapabilityTransform8X8Ext`] reports if
    ///enabling transform_8x8_mode_flag in StdVideoH264PpsFlags is supported.
    pub const VIDEO_ENCODE_H_264_CAPABILITY_TRANSFORM_8_X_8_EXT: Self = Self(4096);
    ///[`VideoEncodeH264CapabilityCabacExt`] reports if CABAC
    ///entropy coding is supported.
    pub const VIDEO_ENCODE_H_264_CAPABILITY_CABAC_EXT: Self = Self(8192);
    ///[`VideoEncodeH264CapabilityCavlcExt`] reports if CAVLC
    ///entropy coding is supported.
    ///An implementation  **must**  support at least one entropy coding mode.
    pub const VIDEO_ENCODE_H_264_CAPABILITY_CAVLC_EXT: Self = Self(16384);
    ///[`VideoEncodeH264CapabilityDeblockingFilterDisabledExt`]
    ///reports if using STD_VIDEO_H264_DISABLE_DEBLOCKING_FILTER_IDC_DISABLED
    ///from StdVideoH264DisableDeblockingFilterIdc is supported.
    pub const VIDEO_ENCODE_H_264_CAPABILITY_DEBLOCKING_FILTER_DISABLED_EXT: Self = Self(32768);
    ///[`VideoEncodeH264CapabilityDeblockingFilterEnabledExt`]
    ///reports if using STD_VIDEO_H264_DISABLE_DEBLOCKING_FILTER_IDC_ENABLED
    ///from StdVideoH264DisableDeblockingFilterIdc is supported.
    pub const VIDEO_ENCODE_H_264_CAPABILITY_DEBLOCKING_FILTER_ENABLED_EXT: Self = Self(65536);
    ///[`VideoEncodeH264CapabilityDeblockingFilterPartialExt`]
    ///reports if using STD_VIDEO_H264_DISABLE_DEBLOCKING_FILTER_IDC_PARTIAL
    ///from StdVideoH264DisableDeblockingFilterIdc is supported.
    ///An implementation  **must**  support at least one deblocking filter mode.
    pub const VIDEO_ENCODE_H_264_CAPABILITY_DEBLOCKING_FILTER_PARTIAL_EXT: Self = Self(131072);
    ///[`VideoEncodeH264CapabilityDisableDirectSpatialMvPredExt`]
    ///reports if disabling
    ///[`StdVideoEncodeH264SliceHeaderFlags`]::direct_spatial_mv_pred_flag is
    ///supported when it is present in the slice header.
    pub const VIDEO_ENCODE_H_264_CAPABILITY_DISABLE_DIRECT_SPATIAL_MV_PRED_EXT: Self = Self(262144);
    ///[`VideoEncodeH264CapabilityMultipleSlicePerFrameExt`]
    ///reports if encoding multiple slices per frame is supported.
    ///If not set, the implementation is only able to encode a single slice for
    ///the entire frame.
    pub const VIDEO_ENCODE_H_264_CAPABILITY_MULTIPLE_SLICE_PER_FRAME_EXT: Self = Self(524288);
    ///[`VideoEncodeH264CapabilitySliceMbCountExt`] reports
    ///support for configuring
    ///[`VideoEncodeH264NaluSliceEXT`]::`mbCount` and first_mb_in_slice
    ///in StdVideoEncodeH264SliceHeader for each slice in a frame with multiple
    ///slices.
    ///If not supported, the implementation decides the number of macroblocks
    ///in each slice based on
    ///[`VideoEncodeH264VclFrameInfoEXT`]::`naluSliceEntryCount`.
    pub const VIDEO_ENCODE_H_264_CAPABILITY_SLICE_MB_COUNT_EXT: Self = Self(1048576);
    ///[`VideoEncodeH264CapabilityRowUnalignedSliceExt`]
    ///reports that each slice in a frame with multiple slices may begin or
    ///finish at any offset in a macroblock row.
    ///If not supported, all slices in the frame  **must**  begin at the start of a
    ///macroblock row (and hence each slice  **must**  finish at the end of a
    ///macroblock row).
    pub const VIDEO_ENCODE_H_264_CAPABILITY_ROW_UNALIGNED_SLICE_EXT: Self = Self(2097152);
    ///[`VideoEncodeH264CapabilityDifferentSliceTypeExt`]
    ///reports that when
    ///[`VideoEncodeH264CapabilityMultipleSlicePerFrameExt`]
    ///is supported and a frame is encoded with multiple slices, the
    ///implementation allows encoding each slice with a different
    ///[`StdVideoEncodeH264SliceHeader`]::slice_type.
    ///If not supported, all slices of the frame  **must**  be encoded with the same
    ///`slice_type` which corresponds to the picture type of the frame.
    ///For example, all slices of a P-frame would be encoded as P-slices.
    pub const VIDEO_ENCODE_H_264_CAPABILITY_DIFFERENT_SLICE_TYPE_EXT: Self = Self(4194304);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    pub const fn all() -> Self {
        Self::empty()
            | Self::VIDEO_ENCODE_H_264_CAPABILITY_DIRECT_8_X_8_INFERENCE_EXT
            | Self::VIDEO_ENCODE_H_264_CAPABILITY_SEPARATE_COLOUR_PLANE_EXT
            | Self::VIDEO_ENCODE_H_264_CAPABILITY_QPPRIME_Y_ZERO_TRANSFORM_BYPASS_EXT
            | Self::VIDEO_ENCODE_H_264_CAPABILITY_SCALING_LISTS_EXT
            | Self::VIDEO_ENCODE_H_264_CAPABILITY_HRD_COMPLIANCE_EXT
            | Self::VIDEO_ENCODE_H_264_CAPABILITY_CHROMA_QP_OFFSET_EXT
            | Self::VIDEO_ENCODE_H_264_CAPABILITY_SECOND_CHROMA_QP_OFFSET_EXT
            | Self::VIDEO_ENCODE_H_264_CAPABILITY_PIC_INIT_QP_MINUS_26_EXT
            | Self::VIDEO_ENCODE_H_264_CAPABILITY_WEIGHTED_PRED_EXT
            | Self::VIDEO_ENCODE_H_264_CAPABILITY_WEIGHTED_BIPRED_EXPLICIT_EXT
            | Self::VIDEO_ENCODE_H_264_CAPABILITY_WEIGHTED_BIPRED_IMPLICIT_EXT
            | Self::VIDEO_ENCODE_H_264_CAPABILITY_WEIGHTED_PRED_NO_TABLE_EXT
            | Self::VIDEO_ENCODE_H_264_CAPABILITY_TRANSFORM_8_X_8_EXT
            | Self::VIDEO_ENCODE_H_264_CAPABILITY_CABAC_EXT
            | Self::VIDEO_ENCODE_H_264_CAPABILITY_CAVLC_EXT
            | Self::VIDEO_ENCODE_H_264_CAPABILITY_DEBLOCKING_FILTER_DISABLED_EXT
            | Self::VIDEO_ENCODE_H_264_CAPABILITY_DEBLOCKING_FILTER_ENABLED_EXT
            | Self::VIDEO_ENCODE_H_264_CAPABILITY_DEBLOCKING_FILTER_PARTIAL_EXT
            | Self::VIDEO_ENCODE_H_264_CAPABILITY_DISABLE_DIRECT_SPATIAL_MV_PRED_EXT
            | Self::VIDEO_ENCODE_H_264_CAPABILITY_MULTIPLE_SLICE_PER_FRAME_EXT
            | Self::VIDEO_ENCODE_H_264_CAPABILITY_SLICE_MB_COUNT_EXT
            | Self::VIDEO_ENCODE_H_264_CAPABILITY_ROW_UNALIGNED_SLICE_EXT
            | Self::VIDEO_ENCODE_H_264_CAPABILITY_DIFFERENT_SLICE_TYPE_EXT
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
impl const std::ops::BitOr for VideoEncodeH264CapabilityFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for VideoEncodeH264CapabilityFlagsEXT {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for VideoEncodeH264CapabilityFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for VideoEncodeH264CapabilityFlagsEXT {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for VideoEncodeH264CapabilityFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for VideoEncodeH264CapabilityFlagsEXT {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for VideoEncodeH264CapabilityFlagsEXT {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for VideoEncodeH264CapabilityFlagsEXT {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for VideoEncodeH264CapabilityFlagsEXT {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<VideoEncodeH264CapabilityFlagsEXT> for VideoEncodeH264CapabilityFlagsEXT {
    fn extend<T: IntoIterator<Item = VideoEncodeH264CapabilityFlagsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<VideoEncodeH264CapabilityFlagBitsEXT> for VideoEncodeH264CapabilityFlagsEXT {
    fn extend<T: IntoIterator<Item = VideoEncodeH264CapabilityFlagBitsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<VideoEncodeH264CapabilityFlagBitsEXT>>::from(i));
        }
    }
}
impl FromIterator<VideoEncodeH264CapabilityFlagsEXT> for VideoEncodeH264CapabilityFlagsEXT {
    fn from_iter<T: IntoIterator<Item = VideoEncodeH264CapabilityFlagsEXT>>(
        iterator: T,
    ) -> VideoEncodeH264CapabilityFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeH264CapabilityFlagsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<VideoEncodeH264CapabilityFlagBitsEXT> for VideoEncodeH264CapabilityFlagsEXT {
    fn from_iter<T: IntoIterator<Item = VideoEncodeH264CapabilityFlagBitsEXT>>(
        iterator: T,
    ) -> VideoEncodeH264CapabilityFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeH264CapabilityFlagBitsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for VideoEncodeH264CapabilityFlagsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoEncodeH264CapabilityFlagsEXT);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoEncodeH264CapabilityFlagsEXT::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(
                        VideoEncodeH264CapabilityFlagsEXT::VIDEO_ENCODE_H_264_CAPABILITY_DIRECT_8_X_8_INFERENCE_EXT,
                    ) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(VIDEO_ENCODE_H_264_CAPABILITY_DIRECT_8_X_8_INFERENCE_EXT))?;
                    }
                    if self.0.contains(
                        VideoEncodeH264CapabilityFlagsEXT::VIDEO_ENCODE_H_264_CAPABILITY_SEPARATE_COLOUR_PLANE_EXT,
                    ) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(VIDEO_ENCODE_H_264_CAPABILITY_SEPARATE_COLOUR_PLANE_EXT))?;
                    }
                    if self . 0 . contains (VideoEncodeH264CapabilityFlagsEXT :: VIDEO_ENCODE_H_264_CAPABILITY_QPPRIME_Y_ZERO_TRANSFORM_BYPASS_EXT) { if ! first { first = false ; f . write_str (" | ") ? ; } f . write_str (stringify ! (VIDEO_ENCODE_H_264_CAPABILITY_QPPRIME_Y_ZERO_TRANSFORM_BYPASS_EXT)) ? ; }
                    if self
                        .0
                        .contains(VideoEncodeH264CapabilityFlagsEXT::VIDEO_ENCODE_H_264_CAPABILITY_SCALING_LISTS_EXT)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(VIDEO_ENCODE_H_264_CAPABILITY_SCALING_LISTS_EXT))?;
                    }
                    if self
                        .0
                        .contains(VideoEncodeH264CapabilityFlagsEXT::VIDEO_ENCODE_H_264_CAPABILITY_HRD_COMPLIANCE_EXT)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(VIDEO_ENCODE_H_264_CAPABILITY_HRD_COMPLIANCE_EXT))?;
                    }
                    if self
                        .0
                        .contains(VideoEncodeH264CapabilityFlagsEXT::VIDEO_ENCODE_H_264_CAPABILITY_CHROMA_QP_OFFSET_EXT)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(VIDEO_ENCODE_H_264_CAPABILITY_CHROMA_QP_OFFSET_EXT))?;
                    }
                    if self.0.contains(
                        VideoEncodeH264CapabilityFlagsEXT::VIDEO_ENCODE_H_264_CAPABILITY_SECOND_CHROMA_QP_OFFSET_EXT,
                    ) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(VIDEO_ENCODE_H_264_CAPABILITY_SECOND_CHROMA_QP_OFFSET_EXT))?;
                    }
                    if self.0.contains(
                        VideoEncodeH264CapabilityFlagsEXT::VIDEO_ENCODE_H_264_CAPABILITY_PIC_INIT_QP_MINUS_26_EXT,
                    ) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(VIDEO_ENCODE_H_264_CAPABILITY_PIC_INIT_QP_MINUS_26_EXT))?;
                    }
                    if self
                        .0
                        .contains(VideoEncodeH264CapabilityFlagsEXT::VIDEO_ENCODE_H_264_CAPABILITY_WEIGHTED_PRED_EXT)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(VIDEO_ENCODE_H_264_CAPABILITY_WEIGHTED_PRED_EXT))?;
                    }
                    if self.0.contains(
                        VideoEncodeH264CapabilityFlagsEXT::VIDEO_ENCODE_H_264_CAPABILITY_WEIGHTED_BIPRED_EXPLICIT_EXT,
                    ) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(VIDEO_ENCODE_H_264_CAPABILITY_WEIGHTED_BIPRED_EXPLICIT_EXT))?;
                    }
                    if self.0.contains(
                        VideoEncodeH264CapabilityFlagsEXT::VIDEO_ENCODE_H_264_CAPABILITY_WEIGHTED_BIPRED_IMPLICIT_EXT,
                    ) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(VIDEO_ENCODE_H_264_CAPABILITY_WEIGHTED_BIPRED_IMPLICIT_EXT))?;
                    }
                    if self.0.contains(
                        VideoEncodeH264CapabilityFlagsEXT::VIDEO_ENCODE_H_264_CAPABILITY_WEIGHTED_PRED_NO_TABLE_EXT,
                    ) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(VIDEO_ENCODE_H_264_CAPABILITY_WEIGHTED_PRED_NO_TABLE_EXT))?;
                    }
                    if self
                        .0
                        .contains(VideoEncodeH264CapabilityFlagsEXT::VIDEO_ENCODE_H_264_CAPABILITY_TRANSFORM_8_X_8_EXT)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(VIDEO_ENCODE_H_264_CAPABILITY_TRANSFORM_8_X_8_EXT))?;
                    }
                    if self
                        .0
                        .contains(VideoEncodeH264CapabilityFlagsEXT::VIDEO_ENCODE_H_264_CAPABILITY_CABAC_EXT)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(VIDEO_ENCODE_H_264_CAPABILITY_CABAC_EXT))?;
                    }
                    if self
                        .0
                        .contains(VideoEncodeH264CapabilityFlagsEXT::VIDEO_ENCODE_H_264_CAPABILITY_CAVLC_EXT)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(VIDEO_ENCODE_H_264_CAPABILITY_CAVLC_EXT))?;
                    }
                    if self.0.contains(
                        VideoEncodeH264CapabilityFlagsEXT::VIDEO_ENCODE_H_264_CAPABILITY_DEBLOCKING_FILTER_DISABLED_EXT,
                    ) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(VIDEO_ENCODE_H_264_CAPABILITY_DEBLOCKING_FILTER_DISABLED_EXT))?;
                    }
                    if self.0.contains(
                        VideoEncodeH264CapabilityFlagsEXT::VIDEO_ENCODE_H_264_CAPABILITY_DEBLOCKING_FILTER_ENABLED_EXT,
                    ) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(VIDEO_ENCODE_H_264_CAPABILITY_DEBLOCKING_FILTER_ENABLED_EXT))?;
                    }
                    if self.0.contains(
                        VideoEncodeH264CapabilityFlagsEXT::VIDEO_ENCODE_H_264_CAPABILITY_DEBLOCKING_FILTER_PARTIAL_EXT,
                    ) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(VIDEO_ENCODE_H_264_CAPABILITY_DEBLOCKING_FILTER_PARTIAL_EXT))?;
                    }
                    if self . 0 . contains (VideoEncodeH264CapabilityFlagsEXT :: VIDEO_ENCODE_H_264_CAPABILITY_DISABLE_DIRECT_SPATIAL_MV_PRED_EXT) { if ! first { first = false ; f . write_str (" | ") ? ; } f . write_str (stringify ! (VIDEO_ENCODE_H_264_CAPABILITY_DISABLE_DIRECT_SPATIAL_MV_PRED_EXT)) ? ; }
                    if self.0.contains(
                        VideoEncodeH264CapabilityFlagsEXT::VIDEO_ENCODE_H_264_CAPABILITY_MULTIPLE_SLICE_PER_FRAME_EXT,
                    ) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(VIDEO_ENCODE_H_264_CAPABILITY_MULTIPLE_SLICE_PER_FRAME_EXT))?;
                    }
                    if self
                        .0
                        .contains(VideoEncodeH264CapabilityFlagsEXT::VIDEO_ENCODE_H_264_CAPABILITY_SLICE_MB_COUNT_EXT)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(VIDEO_ENCODE_H_264_CAPABILITY_SLICE_MB_COUNT_EXT))?;
                    }
                    if self.0.contains(
                        VideoEncodeH264CapabilityFlagsEXT::VIDEO_ENCODE_H_264_CAPABILITY_ROW_UNALIGNED_SLICE_EXT,
                    ) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(VIDEO_ENCODE_H_264_CAPABILITY_ROW_UNALIGNED_SLICE_EXT))?;
                    }
                    if self.0.contains(
                        VideoEncodeH264CapabilityFlagsEXT::VIDEO_ENCODE_H_264_CAPABILITY_DIFFERENT_SLICE_TYPE_EXT,
                    ) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(VIDEO_ENCODE_H_264_CAPABILITY_DIFFERENT_SLICE_TYPE_EXT))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoEncodeH264CapabilityFlagsEXT))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoEncodeH264InputModeFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264InputModeFlagBitsEXT.html) - Video encode H.264 input modes
///# C Specifications
///The `inputModeFlags` field reports the various commmand buffer input
///granularities supported by the implementation as follows:
///```c
///// Provided by VK_EXT_video_encode_h264
///typedef enum VkVideoEncodeH264InputModeFlagBitsEXT {
///    VK_VIDEO_ENCODE_H264_INPUT_MODE_FRAME_BIT_EXT = 0x00000001,
///    VK_VIDEO_ENCODE_H264_INPUT_MODE_SLICE_BIT_EXT = 0x00000002,
///    VK_VIDEO_ENCODE_H264_INPUT_MODE_NON_VCL_BIT_EXT = 0x00000004,
///} VkVideoEncodeH264InputModeFlagBitsEXT;
///```
///# Description
/// - [`VideoEncodeH264InputModeFrameExt`] indicates that a single command buffer  **must**  at
///   least encode an entire frame. Any non-VCL NALUs  **must**  be encoded using the same command
///   buffer as the frame if [`VideoEncodeH264InputModeNonVclExt`] is not supported.
/// - [`VideoEncodeH264InputModeSliceExt`] indicates that a single command buffer  **must**  at
///   least encode a single slice. Any non-VCL NALUs  **must**  be encoded using the same command
///   buffer as the first slice of the frame if [`VideoEncodeH264InputModeNonVclExt`] is not
///   supported.
/// - [`VideoEncodeH264InputModeNonVclExt`] indicates that a single command buffer  **may**  encode
///   a non-VCL NALU by itself.
///An implementation  **must**  support at least one of
///[`VideoEncodeH264InputModeFrameExt`] or
///[`VideoEncodeH264InputModeSliceExt`].If [`VideoEncodeH264InputModeSliceExt`] is not supported,
/// the
///following two additional restrictions apply for frames encoded with multiple
///slices.
///First, all frame slices  **must**  have the same pRefList0ModOperations and the
///same pRefList1ModOperations.
///Second, the order in which slices appear in
///[`VideoEncodeH264VclFrameInfoEXT::nalu_slice_entries`] or in the
///command buffer  **must**  match the placement order of the slices in the frame.
///# Related
/// - [`VK_EXT_video_encode_h264`]
/// - [`VideoEncodeH264InputModeFlagsEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoEncodeH264InputModeFlagsEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoEncodeH264InputModeFlagsEXT(u32);
impl const Default for VideoEncodeH264InputModeFlagsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl From<VideoEncodeH264InputModeFlagBitsEXT> for VideoEncodeH264InputModeFlagsEXT {
    fn from(from: VideoEncodeH264InputModeFlagBitsEXT) -> Self {
        unsafe { Self::from_bits_unchecked(from as u32) }
    }
}
impl VideoEncodeH264InputModeFlagsEXT {
    ///[`VideoEncodeH264InputModeFrameExt`] indicates that a
    ///single command buffer  **must**  at least encode an entire frame.
    ///Any non-VCL NALUs  **must**  be encoded using the same command buffer as the
    ///frame if [`VideoEncodeH264InputModeNonVclExt`] is not
    ///supported.
    pub const VIDEO_ENCODE_H_264_INPUT_MODE_FRAME_EXT: Self = Self(1);
    ///[`VideoEncodeH264InputModeSliceExt`] indicates that a
    ///single command buffer  **must**  at least encode a single slice.
    ///Any non-VCL NALUs  **must**  be encoded using the same command buffer as the
    ///first slice of the frame if
    ///[`VideoEncodeH264InputModeNonVclExt`] is not supported.
    pub const VIDEO_ENCODE_H_264_INPUT_MODE_SLICE_EXT: Self = Self(2);
    ///[`VideoEncodeH264InputModeNonVclExt`] indicates that a
    ///single command buffer  **may**  encode a non-VCL NALU by itself.
    pub const VIDEO_ENCODE_H_264_INPUT_MODE_NON_VCL_EXT: Self = Self(4);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    pub const fn all() -> Self {
        Self::empty()
            | Self::VIDEO_ENCODE_H_264_INPUT_MODE_FRAME_EXT
            | Self::VIDEO_ENCODE_H_264_INPUT_MODE_SLICE_EXT
            | Self::VIDEO_ENCODE_H_264_INPUT_MODE_NON_VCL_EXT
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
impl const std::ops::BitOr for VideoEncodeH264InputModeFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for VideoEncodeH264InputModeFlagsEXT {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for VideoEncodeH264InputModeFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for VideoEncodeH264InputModeFlagsEXT {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for VideoEncodeH264InputModeFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for VideoEncodeH264InputModeFlagsEXT {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for VideoEncodeH264InputModeFlagsEXT {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for VideoEncodeH264InputModeFlagsEXT {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for VideoEncodeH264InputModeFlagsEXT {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<VideoEncodeH264InputModeFlagsEXT> for VideoEncodeH264InputModeFlagsEXT {
    fn extend<T: IntoIterator<Item = VideoEncodeH264InputModeFlagsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<VideoEncodeH264InputModeFlagBitsEXT> for VideoEncodeH264InputModeFlagsEXT {
    fn extend<T: IntoIterator<Item = VideoEncodeH264InputModeFlagBitsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<VideoEncodeH264InputModeFlagBitsEXT>>::from(i));
        }
    }
}
impl FromIterator<VideoEncodeH264InputModeFlagsEXT> for VideoEncodeH264InputModeFlagsEXT {
    fn from_iter<T: IntoIterator<Item = VideoEncodeH264InputModeFlagsEXT>>(
        iterator: T,
    ) -> VideoEncodeH264InputModeFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeH264InputModeFlagsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<VideoEncodeH264InputModeFlagBitsEXT> for VideoEncodeH264InputModeFlagsEXT {
    fn from_iter<T: IntoIterator<Item = VideoEncodeH264InputModeFlagBitsEXT>>(
        iterator: T,
    ) -> VideoEncodeH264InputModeFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeH264InputModeFlagBitsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for VideoEncodeH264InputModeFlagsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoEncodeH264InputModeFlagsEXT);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoEncodeH264InputModeFlagsEXT::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self
                        .0
                        .contains(VideoEncodeH264InputModeFlagsEXT::VIDEO_ENCODE_H_264_INPUT_MODE_FRAME_EXT)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(VIDEO_ENCODE_H_264_INPUT_MODE_FRAME_EXT))?;
                    }
                    if self
                        .0
                        .contains(VideoEncodeH264InputModeFlagsEXT::VIDEO_ENCODE_H_264_INPUT_MODE_SLICE_EXT)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(VIDEO_ENCODE_H_264_INPUT_MODE_SLICE_EXT))?;
                    }
                    if self
                        .0
                        .contains(VideoEncodeH264InputModeFlagsEXT::VIDEO_ENCODE_H_264_INPUT_MODE_NON_VCL_EXT)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(VIDEO_ENCODE_H_264_INPUT_MODE_NON_VCL_EXT))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoEncodeH264InputModeFlagsEXT))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoEncodeH264OutputModeFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264OutputModeFlagBitsEXT.html) - Video encode H.264 output modes
///# C Specifications
///Bits which  **may**  be set in
///[`VideoEncodeH264CapabilitiesEXT::output_mode_flags`], indicating
///the minimum bitstream generation commands that  **must**  be included between
///each [`CmdBeginVideoCodingKHR`] and [`CmdEndVideoCodingKHR`] pair
///(henceforth simply begin/end pair), are:
///```c
///// Provided by VK_EXT_video_encode_h264
///typedef enum VkVideoEncodeH264OutputModeFlagBitsEXT {
///    VK_VIDEO_ENCODE_H264_OUTPUT_MODE_FRAME_BIT_EXT = 0x00000001,
///    VK_VIDEO_ENCODE_H264_OUTPUT_MODE_SLICE_BIT_EXT = 0x00000002,
///    VK_VIDEO_ENCODE_H264_OUTPUT_MODE_NON_VCL_BIT_EXT = 0x00000004,
///} VkVideoEncodeH264OutputModeFlagBitsEXT;
///```
///# Description
/// - [`VideoEncodeH264OutputModeFrameExt`] indicates that calls to generate all NALUs of a frame
///   **must**  be included within a single begin/end pair. Any non-VCL NALUs  **must**  be encoded
///   within the same begin/end pair if [`VideoEncodeH264OutputModeNonVclExt`] is not supported.
/// - [`VideoEncodeH264OutputModeSliceExt`] indicates that each begin/end pair  **must**  encode at
///   least one slice. Any non-VCL NALUs  **must**  be encoded within the same begin/end pair as the
///   first slice of the frame if [`VideoEncodeH264OutputModeNonVclExt`] is not supported.
/// - [`VideoEncodeH264OutputModeNonVclExt`] indicates that each begin/end pair  **may**  encode
///   only a non-VCL NALU by itself. An implementation  **must**  support at least one of
///   [`VideoEncodeH264OutputModeFrameExt`] or [`VideoEncodeH264OutputModeSliceExt`].
///A single begin/end pair  **must**  not encode more than a single frame.The bitstreams of NALUs
/// generated within a single begin/end pair are written
///continuously into the same bitstream buffer (any padding between the NALUs
/// **must**  be compliant to the H.264 standard).The supported input modes  **must**  be coarser or
/// equal to the supported output
///modes.
///For example, it is illegal to report slice input is supported but only frame
///output is supported.An implementation  **must**  report one of the following combinations of
///input/output modes:
/// - Input: Frame, Output: Frame
/// - Input: Frame, Output: Frame and Non-VCL
/// - Input: Frame, Output: Slice
/// - Input: Frame, Output: Slice and Non-VCL
/// - Input: Slice, Output: Slice
/// - Input: Slice, Output: Slice and Non-VCL
/// - Input: Frame and Non-VCL, Output: Frame and Non-VCL
/// - Input: Frame and Non-VCL, Output: Slice and Non-VCL
/// - Input: Slice and Non-VCL, Output: Slice and Non-VCL
///# Related
/// - [`VK_EXT_video_encode_h264`]
/// - [`VideoEncodeH264OutputModeFlagsEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoEncodeH264OutputModeFlagsEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoEncodeH264OutputModeFlagsEXT(u32);
impl const Default for VideoEncodeH264OutputModeFlagsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl From<VideoEncodeH264OutputModeFlagBitsEXT> for VideoEncodeH264OutputModeFlagsEXT {
    fn from(from: VideoEncodeH264OutputModeFlagBitsEXT) -> Self {
        unsafe { Self::from_bits_unchecked(from as u32) }
    }
}
impl VideoEncodeH264OutputModeFlagsEXT {
    ///[`VideoEncodeH264OutputModeFrameExt`] indicates that
    ///calls to generate all NALUs of a frame  **must**  be included within a single
    ///begin/end pair.
    ///Any non-VCL NALUs  **must**  be encoded within the same begin/end pair if
    ///[`VideoEncodeH264OutputModeNonVclExt`] is not supported.
    pub const VIDEO_ENCODE_H_264_OUTPUT_MODE_FRAME_EXT: Self = Self(1);
    ///[`VideoEncodeH264OutputModeSliceExt`] indicates that each
    ///begin/end pair  **must**  encode at least one slice.
    ///Any non-VCL NALUs  **must**  be encoded within the same begin/end pair as the
    ///first slice of the frame if
    ///[`VideoEncodeH264OutputModeNonVclExt`] is not supported.
    pub const VIDEO_ENCODE_H_264_OUTPUT_MODE_SLICE_EXT: Self = Self(2);
    ///[`VideoEncodeH264OutputModeNonVclExt`] indicates that
    ///each begin/end pair  **may**  encode only a non-VCL NALU by itself.
    ///An implementation  **must**  support at least one of
    ///[`VideoEncodeH264OutputModeFrameExt`] or
    ///[`VideoEncodeH264OutputModeSliceExt`].
    pub const VIDEO_ENCODE_H_264_OUTPUT_MODE_NON_VCL_EXT: Self = Self(4);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    pub const fn all() -> Self {
        Self::empty()
            | Self::VIDEO_ENCODE_H_264_OUTPUT_MODE_FRAME_EXT
            | Self::VIDEO_ENCODE_H_264_OUTPUT_MODE_SLICE_EXT
            | Self::VIDEO_ENCODE_H_264_OUTPUT_MODE_NON_VCL_EXT
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
impl const std::ops::BitOr for VideoEncodeH264OutputModeFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for VideoEncodeH264OutputModeFlagsEXT {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for VideoEncodeH264OutputModeFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for VideoEncodeH264OutputModeFlagsEXT {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for VideoEncodeH264OutputModeFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for VideoEncodeH264OutputModeFlagsEXT {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for VideoEncodeH264OutputModeFlagsEXT {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for VideoEncodeH264OutputModeFlagsEXT {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for VideoEncodeH264OutputModeFlagsEXT {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<VideoEncodeH264OutputModeFlagsEXT> for VideoEncodeH264OutputModeFlagsEXT {
    fn extend<T: IntoIterator<Item = VideoEncodeH264OutputModeFlagsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<VideoEncodeH264OutputModeFlagBitsEXT> for VideoEncodeH264OutputModeFlagsEXT {
    fn extend<T: IntoIterator<Item = VideoEncodeH264OutputModeFlagBitsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<VideoEncodeH264OutputModeFlagBitsEXT>>::from(i));
        }
    }
}
impl FromIterator<VideoEncodeH264OutputModeFlagsEXT> for VideoEncodeH264OutputModeFlagsEXT {
    fn from_iter<T: IntoIterator<Item = VideoEncodeH264OutputModeFlagsEXT>>(
        iterator: T,
    ) -> VideoEncodeH264OutputModeFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeH264OutputModeFlagsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<VideoEncodeH264OutputModeFlagBitsEXT> for VideoEncodeH264OutputModeFlagsEXT {
    fn from_iter<T: IntoIterator<Item = VideoEncodeH264OutputModeFlagBitsEXT>>(
        iterator: T,
    ) -> VideoEncodeH264OutputModeFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeH264OutputModeFlagBitsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for VideoEncodeH264OutputModeFlagsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoEncodeH264OutputModeFlagsEXT);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoEncodeH264OutputModeFlagsEXT::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self
                        .0
                        .contains(VideoEncodeH264OutputModeFlagsEXT::VIDEO_ENCODE_H_264_OUTPUT_MODE_FRAME_EXT)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(VIDEO_ENCODE_H_264_OUTPUT_MODE_FRAME_EXT))?;
                    }
                    if self
                        .0
                        .contains(VideoEncodeH264OutputModeFlagsEXT::VIDEO_ENCODE_H_264_OUTPUT_MODE_SLICE_EXT)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(VIDEO_ENCODE_H_264_OUTPUT_MODE_SLICE_EXT))?;
                    }
                    if self
                        .0
                        .contains(VideoEncodeH264OutputModeFlagsEXT::VIDEO_ENCODE_H_264_OUTPUT_MODE_NON_VCL_EXT)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(VIDEO_ENCODE_H_264_OUTPUT_MODE_NON_VCL_EXT))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoEncodeH264OutputModeFlagsEXT))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoEncodeH264CreateFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264CreateFlagBitsEXT.html) - Video encode session creation flags
///# C Specifications
///Bits which  **can**  be set in
///[`VideoEncodeH264SessionCreateInfoEXT::flags`] are:
///```c
///// Provided by VK_EXT_video_encode_h264
///typedef enum VkVideoEncodeH264CreateFlagBitsEXT {
///    VK_VIDEO_ENCODE_H264_CREATE_DEFAULT_EXT = 0,
///    VK_VIDEO_ENCODE_H264_CREATE_RESERVED_0_BIT_EXT = 0x00000001,
///} VkVideoEncodeH264CreateFlagBitsEXT;
///```
///# Description
/// - [`VideoEncodeH264CreateDefaultExt`] is 0, and specifies no additional creation flags.
/// - [`VideoEncodeH264CreateReserved0Ext`] The current version of the specification has reserved
///   this value for future use.
///# Related
/// - [`VK_EXT_video_encode_h264`]
/// - [`VideoEncodeH264CreateFlagsEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoEncodeH264CreateFlagsEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoEncodeH264CreateFlagsEXT(u32);
impl const Default for VideoEncodeH264CreateFlagsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl From<VideoEncodeH264CreateFlagBitsEXT> for VideoEncodeH264CreateFlagsEXT {
    fn from(from: VideoEncodeH264CreateFlagBitsEXT) -> Self {
        unsafe { Self::from_bits_unchecked(from as u32) }
    }
}
impl VideoEncodeH264CreateFlagsEXT {
    ///[`VideoEncodeH264CreateDefaultExt`] is 0, and specifies no
    ///additional creation flags.
    pub const VIDEO_ENCODE_H_264_CREATE_DEFAULT_EXT: Self = Self(0);
    ///[`VideoEncodeH264CreateReserved0Ext`] The current version
    ///of the specification has reserved this value for future use.
    pub const VIDEO_ENCODE_H_264_CREATE_RESERVED_0_EXT: Self = Self(1);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    pub const fn all() -> Self {
        Self::empty() | Self::VIDEO_ENCODE_H_264_CREATE_DEFAULT_EXT | Self::VIDEO_ENCODE_H_264_CREATE_RESERVED_0_EXT
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
impl const std::ops::BitOr for VideoEncodeH264CreateFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for VideoEncodeH264CreateFlagsEXT {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for VideoEncodeH264CreateFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for VideoEncodeH264CreateFlagsEXT {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for VideoEncodeH264CreateFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for VideoEncodeH264CreateFlagsEXT {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for VideoEncodeH264CreateFlagsEXT {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for VideoEncodeH264CreateFlagsEXT {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for VideoEncodeH264CreateFlagsEXT {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<VideoEncodeH264CreateFlagsEXT> for VideoEncodeH264CreateFlagsEXT {
    fn extend<T: IntoIterator<Item = VideoEncodeH264CreateFlagsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<VideoEncodeH264CreateFlagBitsEXT> for VideoEncodeH264CreateFlagsEXT {
    fn extend<T: IntoIterator<Item = VideoEncodeH264CreateFlagBitsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<VideoEncodeH264CreateFlagBitsEXT>>::from(i));
        }
    }
}
impl FromIterator<VideoEncodeH264CreateFlagsEXT> for VideoEncodeH264CreateFlagsEXT {
    fn from_iter<T: IntoIterator<Item = VideoEncodeH264CreateFlagsEXT>>(iterator: T) -> VideoEncodeH264CreateFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeH264CreateFlagsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<VideoEncodeH264CreateFlagBitsEXT> for VideoEncodeH264CreateFlagsEXT {
    fn from_iter<T: IntoIterator<Item = VideoEncodeH264CreateFlagBitsEXT>>(
        iterator: T,
    ) -> VideoEncodeH264CreateFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeH264CreateFlagBitsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for VideoEncodeH264CreateFlagsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoEncodeH264CreateFlagsEXT);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoEncodeH264CreateFlagsEXT::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self
                        .0
                        .contains(VideoEncodeH264CreateFlagsEXT::VIDEO_ENCODE_H_264_CREATE_DEFAULT_EXT)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(VIDEO_ENCODE_H_264_CREATE_DEFAULT_EXT))?;
                    }
                    if self
                        .0
                        .contains(VideoEncodeH264CreateFlagsEXT::VIDEO_ENCODE_H_264_CREATE_RESERVED_0_EXT)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(VIDEO_ENCODE_H_264_CREATE_RESERVED_0_EXT))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoEncodeH264CreateFlagsEXT))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoEncodeH264RateControlStructureFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264RateControlStructureFlagBitsEXT.html) - Video encode H.264 rate control structure flags
///# C Specifications
///The `rateControlStructure` in [`VideoEncodeH264RateControlInfoEXT`]
///specifies one of the following video stream reference structures as a hint
///for the rate control implementation:
///```c
///// Provided by VK_EXT_video_encode_h264
///typedef enum VkVideoEncodeH264RateControlStructureFlagBitsEXT {
///    VK_VIDEO_ENCODE_H264_RATE_CONTROL_STRUCTURE_UNKNOWN_EXT = 0,
///    VK_VIDEO_ENCODE_H264_RATE_CONTROL_STRUCTURE_FLAT_BIT_EXT = 0x00000001,
///    VK_VIDEO_ENCODE_H264_RATE_CONTROL_STRUCTURE_DYADIC_BIT_EXT = 0x00000002,
///} VkVideoEncodeH264RateControlStructureFlagBitsEXT;
///```
///# Description
///```c specifies a dyadic reference structure.
///```
///# Related
/// - [`VK_EXT_video_encode_h264`]
/// - [`VideoEncodeH264RateControlInfoEXT`]
/// - [`VideoEncodeH264RateControlStructureFlagsEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoEncodeH264RateControlStructureFlagsEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoEncodeH264RateControlStructureFlagsEXT(u32);
impl const Default for VideoEncodeH264RateControlStructureFlagsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl From<VideoEncodeH264RateControlStructureFlagBitsEXT> for VideoEncodeH264RateControlStructureFlagsEXT {
    fn from(from: VideoEncodeH264RateControlStructureFlagBitsEXT) -> Self {
        unsafe { Self::from_bits_unchecked(from as u32) }
    }
}
impl VideoEncodeH264RateControlStructureFlagsEXT {
    ///No documentation found
    pub const VIDEO_ENCODE_H_264_RATE_CONTROL_STRUCTURE_UNKNOWN_EXT: Self = Self(0);
    ///No documentation found
    pub const VIDEO_ENCODE_H_264_RATE_CONTROL_STRUCTURE_FLAT_EXT: Self = Self(1);
    ///No documentation found
    pub const VIDEO_ENCODE_H_264_RATE_CONTROL_STRUCTURE_DYADIC_EXT: Self = Self(2);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    pub const fn all() -> Self {
        Self::empty()
            | Self::VIDEO_ENCODE_H_264_RATE_CONTROL_STRUCTURE_UNKNOWN_EXT
            | Self::VIDEO_ENCODE_H_264_RATE_CONTROL_STRUCTURE_FLAT_EXT
            | Self::VIDEO_ENCODE_H_264_RATE_CONTROL_STRUCTURE_DYADIC_EXT
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
impl const std::ops::BitOr for VideoEncodeH264RateControlStructureFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for VideoEncodeH264RateControlStructureFlagsEXT {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for VideoEncodeH264RateControlStructureFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for VideoEncodeH264RateControlStructureFlagsEXT {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for VideoEncodeH264RateControlStructureFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for VideoEncodeH264RateControlStructureFlagsEXT {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for VideoEncodeH264RateControlStructureFlagsEXT {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for VideoEncodeH264RateControlStructureFlagsEXT {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for VideoEncodeH264RateControlStructureFlagsEXT {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<VideoEncodeH264RateControlStructureFlagsEXT> for VideoEncodeH264RateControlStructureFlagsEXT {
    fn extend<T: IntoIterator<Item = VideoEncodeH264RateControlStructureFlagsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<VideoEncodeH264RateControlStructureFlagBitsEXT> for VideoEncodeH264RateControlStructureFlagsEXT {
    fn extend<T: IntoIterator<Item = VideoEncodeH264RateControlStructureFlagBitsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(
                self,
                <Self as From<VideoEncodeH264RateControlStructureFlagBitsEXT>>::from(i),
            );
        }
    }
}
impl FromIterator<VideoEncodeH264RateControlStructureFlagsEXT> for VideoEncodeH264RateControlStructureFlagsEXT {
    fn from_iter<T: IntoIterator<Item = VideoEncodeH264RateControlStructureFlagsEXT>>(
        iterator: T,
    ) -> VideoEncodeH264RateControlStructureFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeH264RateControlStructureFlagsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<VideoEncodeH264RateControlStructureFlagBitsEXT> for VideoEncodeH264RateControlStructureFlagsEXT {
    fn from_iter<T: IntoIterator<Item = VideoEncodeH264RateControlStructureFlagBitsEXT>>(
        iterator: T,
    ) -> VideoEncodeH264RateControlStructureFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeH264RateControlStructureFlagBitsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for VideoEncodeH264RateControlStructureFlagsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoEncodeH264RateControlStructureFlagsEXT);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoEncodeH264RateControlStructureFlagsEXT::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self . 0 . contains (VideoEncodeH264RateControlStructureFlagsEXT :: VIDEO_ENCODE_H_264_RATE_CONTROL_STRUCTURE_UNKNOWN_EXT) { if ! first { first = false ; f . write_str (" | ") ? ; } f . write_str (stringify ! (VIDEO_ENCODE_H_264_RATE_CONTROL_STRUCTURE_UNKNOWN_EXT)) ? ; }
                    if self.0.contains(
                        VideoEncodeH264RateControlStructureFlagsEXT::VIDEO_ENCODE_H_264_RATE_CONTROL_STRUCTURE_FLAT_EXT,
                    ) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(VIDEO_ENCODE_H_264_RATE_CONTROL_STRUCTURE_FLAT_EXT))?;
                    }
                    if self . 0 . contains (VideoEncodeH264RateControlStructureFlagsEXT :: VIDEO_ENCODE_H_264_RATE_CONTROL_STRUCTURE_DYADIC_EXT) { if ! first { first = false ; f . write_str (" | ") ? ; } f . write_str (stringify ! (VIDEO_ENCODE_H_264_RATE_CONTROL_STRUCTURE_DYADIC_EXT)) ? ; }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoEncodeH264RateControlStructureFlagsEXT))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoEncodeH264CapabilitiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264CapabilitiesEXT.html) - Structure specifying H.264 encode capabilities
///# C Specifications
///When calling [`GetPhysicalDeviceVideoCapabilitiesKHR`] with
///`pVideoProfile->videoCodecOperation` specified as
///`VK_VIDEO_CODEC_OPERATION_ENCODE_H264_BIT_EXT`, the
///[`VideoEncodeH264CapabilitiesEXT`] structure  **must**  be included in the
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
///[`VideoEncodeH264CapabilitiesEXT`] structure  **can**  be chained to
///[`VideoCapabilitiesKHR`] to retrieve H.264 extension specific
///capabilities.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_CAPABILITIES_EXT`
/// - [`input_mode_flags`] **must**  be a valid combination of
///   [`VideoEncodeH264InputModeFlagBitsEXT`] values
/// - [`input_mode_flags`] **must**  not be `0`
/// - [`output_mode_flags`] **must**  be a valid combination of
///   [`VideoEncodeH264OutputModeFlagBitsEXT`] values
/// - [`output_mode_flags`] **must**  not be `0`
/// - [`std_extension_version`] **must**  be a valid [`ExtensionProperties`] structure
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
#[doc(alias = "VkVideoEncodeH264CapabilitiesEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeH264CapabilitiesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`VideoEncodeH264CapabilityFlagBitsEXT`]
    ///describing supported encoding tools.
    pub flags: VideoEncodeH264CapabilityFlagsEXT,
    ///[`input_mode_flags`] is a bitmask of
    ///[`VideoEncodeH264InputModeFlagBitsEXT`] describing supported command
    ///buffer input granularities/modes.
    pub input_mode_flags: VideoEncodeH264InputModeFlagsEXT,
    ///[`output_mode_flags`] is a bitmask of
    ///[`VideoEncodeH264OutputModeFlagBitsEXT`] describing supported output
    ///(bitstream size reporting) granularities/modes.
    pub output_mode_flags: VideoEncodeH264OutputModeFlagsEXT,
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
    ///[`motion_vectors_over_pic_boundaries_flag`] if [`TRUE`], indicates
    ///motion_vectors_over_pic_boundaries_flag will be enabled if
    ///bitstream_restriction_flag is enabled in StdVideoH264SpsVuiFlags.
    pub motion_vectors_over_pic_boundaries_flag: Bool32,
    ///[`max_bytes_per_pic_denom`] reports the value that will be used for
    ///max_bytes_per_pic_denom if bitstream_restriction_flag is enabled in
    ///StdVideoH264SpsVuiFlags.
    pub max_bytes_per_pic_denom: u32,
    ///[`max_bits_per_mb_denom`] reports the value that will be used for
    ///max_bits_per_mb_denom if bitstream_restriction_flag is enabled in
    ///StdVideoH264SpsVuiFlags.
    pub max_bits_per_mb_denom: u32,
    ///[`log_2_max_mv_length_horizontal`] reports the value that will be used for
    ///log2_max_mv_length_horizontal if bitstream_restriction_flag is enabled
    ///in StdVideoH264SpsVuiFlags.
    pub log_2_max_mv_length_horizontal: u32,
    ///[`log_2_max_mv_length_vertical`] reports the value that will be used for
    ///log2_max_mv_length_vertical if bitstream_restriction_flag is enabled in
    ///StdVideoH264SpsVuiFlags.
    pub log_2_max_mv_length_vertical: u32,
    ///[`std_extension_version`] is the specific H.264 extension name and
    ///version supported by this implementation.
    pub std_extension_version: ExtensionProperties,
}
impl<'lt> Default for VideoEncodeH264CapabilitiesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VideoEncodeH264CapabilitiesExt,
            p_next: std::ptr::null(),
            flags: Default::default(),
            input_mode_flags: Default::default(),
            output_mode_flags: Default::default(),
            max_p_picture_l_0_reference_count: 0,
            max_b_picture_l_0_reference_count: 0,
            max_l_1_reference_count: 0,
            motion_vectors_over_pic_boundaries_flag: 0,
            max_bytes_per_pic_denom: 0,
            max_bits_per_mb_denom: 0,
            log_2_max_mv_length_horizontal: 0,
            log_2_max_mv_length_vertical: 0,
            std_extension_version: Default::default(),
        }
    }
}
impl<'lt> VideoEncodeH264CapabilitiesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::motion_vectors_over_pic_boundaries_flag`]
    pub fn motion_vectors_over_pic_boundaries_flag_raw(&self) -> Bool32 {
        self.motion_vectors_over_pic_boundaries_flag
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::motion_vectors_over_pic_boundaries_flag`]
    pub fn set_motion_vectors_over_pic_boundaries_flag_raw(&mut self, value: Bool32) -> &mut Self {
        self.motion_vectors_over_pic_boundaries_flag = value;
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
    pub fn flags(&self) -> VideoEncodeH264CapabilityFlagsEXT {
        self.flags
    }
    ///Gets the value of [`Self::input_mode_flags`]
    pub fn input_mode_flags(&self) -> VideoEncodeH264InputModeFlagsEXT {
        self.input_mode_flags
    }
    ///Gets the value of [`Self::output_mode_flags`]
    pub fn output_mode_flags(&self) -> VideoEncodeH264OutputModeFlagsEXT {
        self.output_mode_flags
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
    ///Gets the value of [`Self::motion_vectors_over_pic_boundaries_flag`]
    pub fn motion_vectors_over_pic_boundaries_flag(&self) -> bool {
        unsafe { std::mem::transmute(self.motion_vectors_over_pic_boundaries_flag as u8) }
    }
    ///Gets the value of [`Self::max_bytes_per_pic_denom`]
    pub fn max_bytes_per_pic_denom(&self) -> u32 {
        self.max_bytes_per_pic_denom
    }
    ///Gets the value of [`Self::max_bits_per_mb_denom`]
    pub fn max_bits_per_mb_denom(&self) -> u32 {
        self.max_bits_per_mb_denom
    }
    ///Gets the value of [`Self::log_2_max_mv_length_horizontal`]
    pub fn log_2_max_mv_length_horizontal(&self) -> u32 {
        self.log_2_max_mv_length_horizontal
    }
    ///Gets the value of [`Self::log_2_max_mv_length_vertical`]
    pub fn log_2_max_mv_length_vertical(&self) -> u32 {
        self.log_2_max_mv_length_vertical
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
    pub fn flags_mut(&mut self) -> &mut VideoEncodeH264CapabilityFlagsEXT {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::input_mode_flags`]
    pub fn input_mode_flags_mut(&mut self) -> &mut VideoEncodeH264InputModeFlagsEXT {
        &mut self.input_mode_flags
    }
    ///Gets a mutable reference to the value of [`Self::output_mode_flags`]
    pub fn output_mode_flags_mut(&mut self) -> &mut VideoEncodeH264OutputModeFlagsEXT {
        &mut self.output_mode_flags
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
    ///Gets a mutable reference to the value of [`Self::motion_vectors_over_pic_boundaries_flag`]
    pub fn motion_vectors_over_pic_boundaries_flag_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.motion_vectors_over_pic_boundaries_flag as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.motion_vectors_over_pic_boundaries_flag as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::max_bytes_per_pic_denom`]
    pub fn max_bytes_per_pic_denom_mut(&mut self) -> &mut u32 {
        &mut self.max_bytes_per_pic_denom
    }
    ///Gets a mutable reference to the value of [`Self::max_bits_per_mb_denom`]
    pub fn max_bits_per_mb_denom_mut(&mut self) -> &mut u32 {
        &mut self.max_bits_per_mb_denom
    }
    ///Gets a mutable reference to the value of [`Self::log_2_max_mv_length_horizontal`]
    pub fn log_2_max_mv_length_horizontal_mut(&mut self) -> &mut u32 {
        &mut self.log_2_max_mv_length_horizontal
    }
    ///Gets a mutable reference to the value of [`Self::log_2_max_mv_length_vertical`]
    pub fn log_2_max_mv_length_vertical_mut(&mut self) -> &mut u32 {
        &mut self.log_2_max_mv_length_vertical
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
        value: crate::extensions::ext_video_encode_h_264::VideoEncodeH264CapabilityFlagsEXT,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::input_mode_flags`]
    pub fn set_input_mode_flags(
        &mut self,
        value: crate::extensions::ext_video_encode_h_264::VideoEncodeH264InputModeFlagsEXT,
    ) -> &mut Self {
        self.input_mode_flags = value;
        self
    }
    ///Sets the raw value of [`Self::output_mode_flags`]
    pub fn set_output_mode_flags(
        &mut self,
        value: crate::extensions::ext_video_encode_h_264::VideoEncodeH264OutputModeFlagsEXT,
    ) -> &mut Self {
        self.output_mode_flags = value;
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
    ///Sets the raw value of [`Self::motion_vectors_over_pic_boundaries_flag`]
    pub fn set_motion_vectors_over_pic_boundaries_flag(&mut self, value: bool) -> &mut Self {
        self.motion_vectors_over_pic_boundaries_flag = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::max_bytes_per_pic_denom`]
    pub fn set_max_bytes_per_pic_denom(&mut self, value: u32) -> &mut Self {
        self.max_bytes_per_pic_denom = value;
        self
    }
    ///Sets the raw value of [`Self::max_bits_per_mb_denom`]
    pub fn set_max_bits_per_mb_denom(&mut self, value: u32) -> &mut Self {
        self.max_bits_per_mb_denom = value;
        self
    }
    ///Sets the raw value of [`Self::log_2_max_mv_length_horizontal`]
    pub fn set_log_2_max_mv_length_horizontal(&mut self, value: u32) -> &mut Self {
        self.log_2_max_mv_length_horizontal = value;
        self
    }
    ///Sets the raw value of [`Self::log_2_max_mv_length_vertical`]
    pub fn set_log_2_max_mv_length_vertical(&mut self, value: u32) -> &mut Self {
        self.log_2_max_mv_length_vertical = value;
        self
    }
    ///Sets the raw value of [`Self::std_extension_version`]
    pub fn set_std_extension_version(&mut self, value: crate::vulkan1_0::ExtensionProperties) -> &mut Self {
        self.std_extension_version = value;
        self
    }
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
/// - [`std_extension_version`] is a pointer to a [`ExtensionProperties`] structure specifying H.264
///   codec extensions.
///# Description
///A [`VideoEncodeH264SessionCreateInfoEXT`] structure  **must**  be chained to
///[`VideoSessionCreateInfoKHR`] when the function
///[`CreateVideoSessionKHR`] is called with `videoCodecOperation` in
///[`VideoSessionCreateInfoKHR`] set to
///`VK_VIDEO_CODEC_OPERATION_ENCODE_H264_BIT_EXT`.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_SESSION_CREATE_INFO_EXT`
/// - [`flags`] **must**  be a valid combination of [`VideoEncodeH264CreateFlagBitsEXT`] values
/// - [`flags`] **must**  not be `0`
/// - [`std_extension_version`] **must**  be a valid pointer to a valid [`ExtensionProperties`]
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
#[doc(alias = "VkVideoEncodeH264SessionCreateInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeH264SessionCreateInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`VideoEncodeH264CreateFlagsEXT`]
    ///specifying H.264 encoder creation flags.
    pub flags: VideoEncodeH264CreateFlagsEXT,
    ///[`max_picture_size_in_mbs`] specifies the syntax element
    ///pic_width_in_mbs_minus1 + 1 and the syntax element
    ///pic_height_in_map_units_minus1 + 1.
    pub max_picture_size_in_mbs: Extent2D,
    ///[`std_extension_version`] is a pointer to a [`ExtensionProperties`]
    ///structure specifying H.264 codec extensions.
    pub std_extension_version: *const ExtensionProperties,
}
impl<'lt> Default for VideoEncodeH264SessionCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VideoEncodeH264SessionCreateInfoExt,
            p_next: std::ptr::null(),
            flags: Default::default(),
            max_picture_size_in_mbs: Default::default(),
            std_extension_version: std::ptr::null(),
        }
    }
}
impl<'lt> VideoEncodeH264SessionCreateInfoEXT<'lt> {
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
    pub fn flags(&self) -> VideoEncodeH264CreateFlagsEXT {
        self.flags
    }
    ///Gets the value of [`Self::max_picture_size_in_mbs`]
    pub fn max_picture_size_in_mbs(&self) -> Extent2D {
        self.max_picture_size_in_mbs
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
    pub fn flags_mut(&mut self) -> &mut VideoEncodeH264CreateFlagsEXT {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::max_picture_size_in_mbs`]
    pub fn max_picture_size_in_mbs_mut(&mut self) -> &mut Extent2D {
        &mut self.max_picture_size_in_mbs
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
        value: crate::extensions::ext_video_encode_h_264::VideoEncodeH264CreateFlagsEXT,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::max_picture_size_in_mbs`]
    pub fn set_max_picture_size_in_mbs(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.max_picture_size_in_mbs = value;
        self
    }
    ///Sets the raw value of [`Self::std_extension_version`]
    pub fn set_std_extension_version(&mut self, value: &'lt crate::vulkan1_0::ExtensionProperties) -> &mut Self {
        self.std_extension_version = value as *const _;
        self
    }
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
/// - [`sps_std_count`] is the number of SPS elements in the [`sps_std`]. Its value  **must**  be
///   less than or equal to the value of `maxSpsStdCount`.
/// - [`sps_std`] is a pointer to an array of [`StdVideoH264SequenceParameterSet`] structures
///   representing H.264 sequence parameter sets. Each element of the array  **must**  have a unique
///   H.264 SPS ID.
/// - [`pps_std_count`] is the number of PPS provided in [`pps_std`]. Its value  **must**  be less
///   than or equal to the value of `maxPpsStdCount`.
/// - [`pps_std`] is a pointer to an array of [`StdVideoH264PictureParameterSet`] structures
///   representing H.264 picture parameter sets. Each element of the array  **must**  have a unique
///   H.264 SPS-PPS ID pair.
///# Description
///## Valid Usage
/// - The values of [`sps_std_count`] and [`pps_std_count`] **must**  be less than or equal to the
///   values of `maxSpsStdCount` and `maxPpsStdCount`, respectively
/// - When the `maxSpsStdCount` number of parameters of type StdVideoH264SequenceParameterSet in the
///   Video Session Parameters object is reached, no additional parameters of that type can be added
///   to the object. `VK_ERROR_TOO_MANY_OBJECTS` will be returned if an attempt is made to add
///   additional data to this object at this point
/// - When the `maxPpsStdCount` number of parameters of type StdVideoH264PictureParameterSet in the
///   Video Session Parameters object is reached, no additional parameters of that type can be added
///   to the object. `VK_ERROR_TOO_MANY_OBJECTS` will be returned if an attempt is made to add
///   additional data to this object at this point
/// - Each entry to be added  **must**  have a unique, to the rest of the parameter array entries
///   and the existing parameters in the Video Session Parameters Object that is being updated,
///   SPS-PPS IDs
/// - Parameter entries that already exist in Video Session Parameters object with a particular
///   SPS-PPS IDs  **cannot**  be replaced nor updated
/// - When creating a new object using a Video Session Parameters as a template, the array’s
///   parameters with the same SPS-PPS IDs as the ones from the template take precedence
/// - SPS/PPS parameters  **must**  comply with the limits specified in
///   [`VideoSessionCreateInfoKHR`] during Video Session creation
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_SESSION_PARAMETERS_ADD_INFO_EXT`
/// - If [`sps_std`] is not `NULL`, [`sps_std`] **must**  be a valid pointer to an array of
///   [`sps_std_count`][`StdVideoH264SequenceParameterSet`] values
/// - If [`pps_std`] is not `NULL`, [`pps_std`] **must**  be a valid pointer to an array of
///   [`pps_std_count`][`StdVideoH264PictureParameterSet`] values
/// - [`sps_std_count`] **must**  be greater than `0`
/// - [`pps_std_count`] **must**  be greater than `0`
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
#[doc(alias = "VkVideoEncodeH264SessionParametersAddInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeH264SessionParametersAddInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`sps_std_count`] is the number of SPS elements in the [`sps_std`].
    ///Its value  **must**  be less than or equal to the value of
    ///`maxSpsStdCount`.
    pub sps_std_count: u32,
    ///[`sps_std`] is a pointer to an array of
    ///[`StdVideoH264SequenceParameterSet`] structures representing H.264
    ///sequence parameter sets.
    ///Each element of the array  **must**  have a unique H.264 SPS ID.
    pub sps_std: *const StdVideoH264SequenceParameterSet,
    ///[`pps_std_count`] is the number of PPS provided in [`pps_std`].
    ///Its value  **must**  be less than or equal to the value of
    ///`maxPpsStdCount`.
    pub pps_std_count: u32,
    ///[`pps_std`] is a pointer to an array of
    ///[`StdVideoH264PictureParameterSet`] structures representing H.264
    ///picture parameter sets.
    ///Each element of the array  **must**  have a unique H.264 SPS-PPS ID pair.
    pub pps_std: *const StdVideoH264PictureParameterSet,
}
impl<'lt> Default for VideoEncodeH264SessionParametersAddInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VideoEncodeH264SessionParametersAddInfoExt,
            p_next: std::ptr::null(),
            sps_std_count: 0,
            sps_std: std::ptr::null(),
            pps_std_count: 0,
            pps_std: std::ptr::null(),
        }
    }
}
impl<'lt> VideoEncodeH264SessionParametersAddInfoEXT<'lt> {
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
        &mut self.sps_std_count
    }
    ///Gets a mutable reference to the value of [`Self::pps_std_count`]
    pub fn pps_std_count_mut(&mut self) -> &mut u32 {
        &mut self.pps_std_count
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
/// - [`parameters_add_info`] is `NULL` or a pointer to a
///   [`VideoEncodeH264SessionParametersAddInfoEXT`] structure specifying H.264 parameters to add
///   upon object creation.
///# Description
///A [`VideoEncodeH264SessionParametersCreateInfoEXT`] structure holding
///one H.264 SPS and at least one H.264 PPS paramater set  **must**  be chained to
///[`VideoSessionParametersCreateInfoKHR`] when calling
///[`CreateVideoSessionParametersKHR`] to store these parameter set(s) with
///the encoder parameter set object for later reference.
///The provided H.264 SPS/PPS parameters  **must**  be within the limits specified
///during encoder creation for the encoder specified in
///[`VideoSessionParametersCreateInfoKHR`].
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_SESSION_PARAMETERS_CREATE_INFO_EXT`
/// - If [`parameters_add_info`] is not `NULL`, [`parameters_add_info`] **must**  be a valid pointer
///   to a valid [`VideoEncodeH264SessionParametersAddInfoEXT`] structure
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
#[doc(alias = "VkVideoEncodeH264SessionParametersCreateInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeH264SessionParametersCreateInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`max_sps_std_count`] is the maximum number of SPS parameters that the
    ///[`VideoSessionParametersKHR`] can contain.
    pub max_sps_std_count: u32,
    ///[`max_pps_std_count`] is the maximum number of PPS parameters that the
    ///[`VideoSessionParametersKHR`] can contain.
    pub max_pps_std_count: u32,
    ///[`parameters_add_info`] is `NULL` or a pointer to a
    ///[`VideoEncodeH264SessionParametersAddInfoEXT`] structure specifying
    ///H.264 parameters to add upon object creation.
    pub parameters_add_info: *const VideoEncodeH264SessionParametersAddInfoEXT<'lt>,
}
impl<'lt> Default for VideoEncodeH264SessionParametersCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VideoEncodeH264SessionParametersCreateInfoExt,
            p_next: std::ptr::null(),
            max_sps_std_count: 0,
            max_pps_std_count: 0,
            parameters_add_info: std::ptr::null(),
        }
    }
}
impl<'lt> VideoEncodeH264SessionParametersCreateInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::parameters_add_info`]
    pub fn parameters_add_info_raw(&self) -> *const VideoEncodeH264SessionParametersAddInfoEXT<'lt> {
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
        value: *const VideoEncodeH264SessionParametersAddInfoEXT<'lt>,
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
    pub unsafe fn parameters_add_info(&self) -> &VideoEncodeH264SessionParametersAddInfoEXT<'lt> {
        &*self.parameters_add_info
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::max_sps_std_count`]
    pub fn max_sps_std_count_mut(&mut self) -> &mut u32 {
        &mut self.max_sps_std_count
    }
    ///Gets a mutable reference to the value of [`Self::max_pps_std_count`]
    pub fn max_pps_std_count_mut(&mut self) -> &mut u32 {
        &mut self.max_pps_std_count
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
        value: &'lt crate::extensions::ext_video_encode_h_264::VideoEncodeH264SessionParametersAddInfoEXT<'lt>,
    ) -> &mut Self {
        self.parameters_add_info = value as *const _;
        self
    }
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
///   index for this picture. [`slot_index`] **must**  match the [`slot_index`] in
///   `pSetupReferenceSlot` of [`VideoEncodeInfoKHR`] in the command used to encode the
///   corresponding picture.
/// - [`std_reference_info`] is a pointer to a [`StdVideoEncodeH264ReferenceInfo`] structure
///   specifying the syntax and other codec-specific information from the H.264 specification
///   associated with this reference picture.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_DPB_SLOT_INFO_EXT`
/// - [`p_next`] **must**  be `NULL`
/// - [`std_reference_info`] **must**  be a valid pointer to a valid
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
#[doc(alias = "VkVideoEncodeH264DpbSlotInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeH264DpbSlotInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`slot_index`] is the [DPB Slot](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#dpb-slot) index for this picture.
    ///[`slot_index`] **must**  match the [`slot_index`] in
    ///`pSetupReferenceSlot` of [`VideoEncodeInfoKHR`] in the command
    ///used to encode the corresponding picture.
    pub slot_index: i8,
    ///[`std_reference_info`] is a pointer to a
    ///[`StdVideoEncodeH264ReferenceInfo`] structure specifying the syntax and
    ///other codec-specific information from the H.264 specification associated
    ///with this reference picture.
    pub std_reference_info: *const StdVideoEncodeH264ReferenceInfo,
}
impl<'lt> Default for VideoEncodeH264DpbSlotInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VideoEncodeH264DpbSlotInfoExt,
            p_next: std::ptr::null(),
            slot_index: 0,
            std_reference_info: std::ptr::null(),
        }
    }
}
impl<'lt> VideoEncodeH264DpbSlotInfoEXT<'lt> {
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
    pub unsafe fn std_reference_info(&self) -> &StdVideoEncodeH264ReferenceInfo {
        &*self.std_reference_info
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::slot_index`]
    pub fn slot_index_mut(&mut self) -> &mut i8 {
        &mut self.slot_index
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
    pub fn set_std_reference_info(&mut self, value: &'lt crate::native::StdVideoEncodeH264ReferenceInfo) -> &mut Self {
        self.std_reference_info = value as *const _;
        self
    }
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
/// - [`reference_final_lists`] is `NULL` or a pointer to a [`VideoEncodeH264ReferenceListsEXT`]
///   structure specifying the reference lists to be used for the current picture.
/// - [`nalu_slice_entry_count`] is the number of slice NALUs in the frame.
/// - [`nalu_slice_entries`] is a pointer to an array of
///   [`nalu_slice_entry_count`][`VideoEncodeH264NaluSliceEXT`] structures specifying the division
///   of the current picture into slices and the properties of these slices. This is an ordered
///   sequence; the NALUs are generated consecutively in
///   [`VideoEncodeInfoKHR::dst_bitstream_buffer`] in the same order as in this array.
/// - [`current_picture_info`] is a pointer to a [`StdVideoEncodeH264PictureInfo`] structure
///   specifying the syntax and other codec-specific information from the H.264 specification
///   associated with this picture. The information provided  **must**  reflect the decoded picture
///   marking operations that are applicable to this frame.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_VCL_FRAME_INFO_EXT`
/// - If [`reference_final_lists`] is not `NULL`, [`reference_final_lists`] **must**  be a valid
///   pointer to a valid [`VideoEncodeH264ReferenceListsEXT`] structure
/// - [`nalu_slice_entries`] **must**  be a valid pointer to an array of [`nalu_slice_entry_count`]
///   valid [`VideoEncodeH264NaluSliceEXT`] structures
/// - [`current_picture_info`] **must**  be a valid pointer to a valid
///   [`StdVideoEncodeH264PictureInfo`] value
/// - [`nalu_slice_entry_count`] **must**  be greater than `0`
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
#[doc(alias = "VkVideoEncodeH264VclFrameInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeH264VclFrameInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`reference_final_lists`] is `NULL` or a pointer to a
    ///[`VideoEncodeH264ReferenceListsEXT`] structure specifying the
    ///reference lists to be used for the current picture.
    pub reference_final_lists: *const VideoEncodeH264ReferenceListsEXT<'lt>,
    ///[`nalu_slice_entry_count`] is the number of slice NALUs in the frame.
    pub nalu_slice_entry_count: u32,
    ///[`nalu_slice_entries`] is a pointer to an array of
    ///[`nalu_slice_entry_count`][`VideoEncodeH264NaluSliceEXT`] structures
    ///specifying the division of the current picture into slices and the
    ///properties of these slices.
    ///This is an ordered sequence; the NALUs are generated consecutively in
    ///[`VideoEncodeInfoKHR`]::`dstBitstreamBuffer` in the same order
    ///as in this array.
    pub nalu_slice_entries: *const VideoEncodeH264NaluSliceEXT<'lt>,
    ///[`current_picture_info`] is a pointer to a
    ///[`StdVideoEncodeH264PictureInfo`] structure specifying the syntax and
    ///other codec-specific information from the H.264 specification associated
    ///with this picture.
    ///The information provided  **must**  reflect the decoded picture marking
    ///operations that are applicable to this frame.
    pub current_picture_info: *const StdVideoEncodeH264PictureInfo,
}
impl<'lt> Default for VideoEncodeH264VclFrameInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VideoEncodeH264VclFrameInfoExt,
            p_next: std::ptr::null(),
            reference_final_lists: std::ptr::null(),
            nalu_slice_entry_count: 0,
            nalu_slice_entries: std::ptr::null(),
            current_picture_info: std::ptr::null(),
        }
    }
}
impl<'lt> VideoEncodeH264VclFrameInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::reference_final_lists`]
    pub fn reference_final_lists_raw(&self) -> *const VideoEncodeH264ReferenceListsEXT<'lt> {
        self.reference_final_lists
    }
    ///Gets the raw value of [`Self::nalu_slice_entries`]
    pub fn nalu_slice_entries_raw(&self) -> *const VideoEncodeH264NaluSliceEXT<'lt> {
        self.nalu_slice_entries
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::reference_final_lists`]
    pub fn set_reference_final_lists_raw(&mut self, value: *const VideoEncodeH264ReferenceListsEXT<'lt>) -> &mut Self {
        self.reference_final_lists = value;
        self
    }
    ///Sets the raw value of [`Self::nalu_slice_entries`]
    pub fn set_nalu_slice_entries_raw(&mut self, value: *const VideoEncodeH264NaluSliceEXT<'lt>) -> &mut Self {
        self.nalu_slice_entries = value;
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
    pub unsafe fn reference_final_lists(&self) -> &VideoEncodeH264ReferenceListsEXT<'lt> {
        &*self.reference_final_lists
    }
    ///Gets the value of [`Self::nalu_slice_entry_count`]
    pub fn nalu_slice_entry_count(&self) -> u32 {
        self.nalu_slice_entry_count
    }
    ///Gets the value of [`Self::nalu_slice_entries`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn nalu_slice_entries(&self) -> &[VideoEncodeH264NaluSliceEXT<'lt>] {
        std::slice::from_raw_parts(self.nalu_slice_entries, self.nalu_slice_entry_count as usize)
    }
    ///Gets the value of [`Self::current_picture_info`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn current_picture_info(&self) -> &StdVideoEncodeH264PictureInfo {
        &*self.current_picture_info
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::nalu_slice_entry_count`]
    pub fn nalu_slice_entry_count_mut(&mut self) -> &mut u32 {
        &mut self.nalu_slice_entry_count
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
        value: &'lt crate::extensions::ext_video_encode_h_264::VideoEncodeH264ReferenceListsEXT<'lt>,
    ) -> &mut Self {
        self.reference_final_lists = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::nalu_slice_entry_count`]
    pub fn set_nalu_slice_entry_count(&mut self, value: u32) -> &mut Self {
        self.nalu_slice_entry_count = value;
        self
    }
    ///Sets the raw value of [`Self::nalu_slice_entries`]
    pub fn set_nalu_slice_entries(
        &mut self,
        value: &'lt [crate::extensions::ext_video_encode_h_264::VideoEncodeH264NaluSliceEXT<'lt>],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.nalu_slice_entries = value.as_ptr();
        self.nalu_slice_entry_count = len_;
        self
    }
    ///Sets the raw value of [`Self::current_picture_info`]
    pub fn set_current_picture_info(&mut self, value: &'lt crate::native::StdVideoEncodeH264PictureInfo) -> &mut Self {
        self.current_picture_info = value as *const _;
        self
    }
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
/// - [`reference_list_0_entries`] is a pointer to an array of
///   [`reference_list_0_entry_count`][`VideoEncodeH264DpbSlotInfoEXT`] structures specifying the
///   reference list L0 entries for the current picture. The entries provided  **must**  be ordered
///   after all reference list L0 modification operations are applied (i.e. final list order). The
///   entries provided  **must**  not reflect decoded picture marking operations in this frame that
///   are applicable to references; the impact of such operations  **must**  be reflected in future
///   frame encode commands. The slot index in each entry  **must**  match one of the slot indexes
///   provided in the `pReferenceSlots` of the parent [`VideoEncodeInfoKHR`] structure.
/// - [`reference_list_1_entry_count`] is the number of reference pictures in reference list L1 and
///   is identical to [`StdVideoEncodeH264SliceHeader`]`::num_ref_idx_l1_active_minus1` + 1.
/// - [`reference_list_1_entries`] is a pointer to an array of
///   [`reference_list_1_entry_count`][`VideoEncodeH264DpbSlotInfoEXT`] structures specifying the
///   reference list L1 entries for the current picture. The entries provided  **must**  be ordered
///   after all reference list L1 modification operations are applied (i.e. final list order). The
///   entries provided  **must**  not reflect decoded picture marking operations in this frame that
///   are applicable to references; the impact of such operations  **must**  be reflected in future
///   frame encode commands. The slot index in each entry  **must**  match one of the slot indexes
///   provided in the `pReferenceSlots` of the parent [`VideoEncodeInfoKHR`] structure.
/// - [`mem_mgmt_ctrl_operations`] is a pointer to a [`StdVideoEncodeH264RefMemMgmtCtrlOperations`]
///   structure specifying reference lists modifications and decoded picture marking operations.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_REFERENCE_LISTS_EXT`
/// - [`p_next`] **must**  be `NULL`
/// - If [`reference_list_0_entry_count`] is not `0`, [`reference_list_0_entries`] **must**  be a
///   valid pointer to an array of [`reference_list_0_entry_count`] valid
///   [`VideoEncodeH264DpbSlotInfoEXT`] structures
/// - If [`reference_list_1_entry_count`] is not `0`, [`reference_list_1_entries`] **must**  be a
///   valid pointer to an array of [`reference_list_1_entry_count`] valid
///   [`VideoEncodeH264DpbSlotInfoEXT`] structures
/// - [`mem_mgmt_ctrl_operations`] **must**  be a valid pointer to a valid
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
#[doc(alias = "VkVideoEncodeH264ReferenceListsEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeH264ReferenceListsEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`reference_list_0_entry_count`] is the number of reference pictures in
    ///reference list L0 and is identical to
    ///[`StdVideoEncodeH264SliceHeader`]::`num_ref_idx_l0_active_minus1` + 1.
    pub reference_list_0_entry_count: u8,
    ///[`reference_list_0_entries`] is a pointer to an array of
    ///[`reference_list_0_entry_count`][`VideoEncodeH264DpbSlotInfoEXT`]
    ///structures specifying the reference list L0 entries for the current
    ///picture.
    ///The entries provided  **must**  be ordered after all reference list L0
    ///modification operations are applied (i.e. final list order).
    ///The entries provided  **must**  not reflect decoded picture marking
    ///operations in this frame that are applicable to references; the impact
    ///of such operations  **must**  be reflected in future frame encode commands.
    ///The slot index in each entry  **must**  match one of the slot indexes
    ///provided in the `pReferenceSlots` of the parent
    ///[`VideoEncodeInfoKHR`] structure.
    pub reference_list_0_entries: *const VideoEncodeH264DpbSlotInfoEXT<'lt>,
    ///[`reference_list_1_entry_count`] is the number of reference pictures in
    ///reference list L1 and is identical to
    ///[`StdVideoEncodeH264SliceHeader`]::`num_ref_idx_l1_active_minus1` + 1.
    pub reference_list_1_entry_count: u8,
    ///[`reference_list_1_entries`] is a pointer to an array of
    ///[`reference_list_1_entry_count`][`VideoEncodeH264DpbSlotInfoEXT`]
    ///structures specifying the reference list L1 entries for the current
    ///picture.
    ///The entries provided  **must**  be ordered after all reference list L1
    ///modification operations are applied (i.e. final list order).
    ///The entries provided  **must**  not reflect decoded picture marking
    ///operations in this frame that are applicable to references; the impact
    ///of such operations  **must**  be reflected in future frame encode commands.
    ///The slot index in each entry  **must**  match one of the slot indexes
    ///provided in the `pReferenceSlots` of the parent
    ///[`VideoEncodeInfoKHR`] structure.
    pub reference_list_1_entries: *const VideoEncodeH264DpbSlotInfoEXT<'lt>,
    ///[`mem_mgmt_ctrl_operations`] is a pointer to a
    ///[`StdVideoEncodeH264RefMemMgmtCtrlOperations`] structure specifying
    ///reference lists modifications and decoded picture marking operations.
    pub mem_mgmt_ctrl_operations: *const StdVideoEncodeH264RefMemMgmtCtrlOperations,
}
impl<'lt> Default for VideoEncodeH264ReferenceListsEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VideoEncodeH264ReferenceListsExt,
            p_next: std::ptr::null(),
            reference_list_0_entry_count: 0,
            reference_list_0_entries: std::ptr::null(),
            reference_list_1_entry_count: 0,
            reference_list_1_entries: std::ptr::null(),
            mem_mgmt_ctrl_operations: std::ptr::null(),
        }
    }
}
impl<'lt> VideoEncodeH264ReferenceListsEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::reference_list_0_entries`]
    pub fn reference_list_0_entries_raw(&self) -> *const VideoEncodeH264DpbSlotInfoEXT<'lt> {
        self.reference_list_0_entries
    }
    ///Gets the raw value of [`Self::reference_list_1_entries`]
    pub fn reference_list_1_entries_raw(&self) -> *const VideoEncodeH264DpbSlotInfoEXT<'lt> {
        self.reference_list_1_entries
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::reference_list_0_entries`]
    pub fn set_reference_list_0_entries_raw(&mut self, value: *const VideoEncodeH264DpbSlotInfoEXT<'lt>) -> &mut Self {
        self.reference_list_0_entries = value;
        self
    }
    ///Sets the raw value of [`Self::reference_list_1_entries`]
    pub fn set_reference_list_1_entries_raw(&mut self, value: *const VideoEncodeH264DpbSlotInfoEXT<'lt>) -> &mut Self {
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
    pub unsafe fn reference_list_0_entries(&self) -> &[VideoEncodeH264DpbSlotInfoEXT<'lt>] {
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
    pub unsafe fn reference_list_1_entries(&self) -> &[VideoEncodeH264DpbSlotInfoEXT<'lt>] {
        std::slice::from_raw_parts(
            self.reference_list_1_entries,
            self.reference_list_1_entry_count as usize,
        )
    }
    ///Gets the value of [`Self::mem_mgmt_ctrl_operations`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn mem_mgmt_ctrl_operations(&self) -> &StdVideoEncodeH264RefMemMgmtCtrlOperations {
        &*self.mem_mgmt_ctrl_operations
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
        value: &'lt [crate::extensions::ext_video_encode_h_264::VideoEncodeH264DpbSlotInfoEXT<'lt>],
    ) -> &mut Self {
        let len_ = value.len() as u8;
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
        value: &'lt [crate::extensions::ext_video_encode_h_264::VideoEncodeH264DpbSlotInfoEXT<'lt>],
    ) -> &mut Self {
        let len_ = value.len() as u8;
        let len_ = len_;
        self.reference_list_1_entries = value.as_ptr();
        self.reference_list_1_entry_count = len_;
        self
    }
    ///Sets the raw value of [`Self::mem_mgmt_ctrl_operations`]
    pub fn set_mem_mgmt_ctrl_operations(
        &mut self,
        value: &'lt crate::native::StdVideoEncodeH264RefMemMgmtCtrlOperations,
    ) -> &mut Self {
        self.mem_mgmt_ctrl_operations = value as *const _;
        self
    }
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
///   **must**  match the SPS provided in `spsStd` of
///   [`VideoEncodeH264SessionParametersCreateInfoEXT`]. This is retrieved from the
///   [`VideoSessionParametersKHR`] object provided in [`VideoBeginCodingInfoKHR`].
/// - [`emit_sps_enable`] enables the emitting of the SPS structure with id of [`sps_id`].
/// - [`pps_id_entry_count`] is the number of entries in the [`pps_id_entries`]. If this parameter
///   is `0` then no pps entries are going to be emitted in the bitstream.
/// - [`pps_id_entries`] is a pointer to an array of H.264 PPS IDs for the H.264 PPS to insert in
///   the bitstream. The PPS IDs  **must**  match one of the IDs of the PPS(s) provided in `pPpsStd`
///   of [`VideoEncodeH264SessionParametersCreateInfoEXT`] to identify the PPS parameter set to
///   insert in the bitstream. This is retrieved from the [`VideoSessionParametersKHR`] object
///   provided in [`VideoBeginCodingInfoKHR`].
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_EMIT_PICTURE_PARAMETERS_EXT`
/// - [`pps_id_entries`] **must**  be a valid pointer to an array of [`pps_id_entry_count`]`uint8_t`
///   values
/// - [`pps_id_entry_count`] **must**  be greater than `0`
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
#[doc(alias = "VkVideoEncodeH264EmitPictureParametersEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeH264EmitPictureParametersEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`sps_id`] is the H.264 SPS ID for the H.264 SPS to insert in the
    ///bitstream.
    ///The SPS ID  **must**  match the SPS provided in `spsStd` of
    ///[`VideoEncodeH264SessionParametersCreateInfoEXT`].
    ///This is retrieved from the [`VideoSessionParametersKHR`] object
    ///provided in [`VideoBeginCodingInfoKHR`].
    pub sps_id: u8,
    ///[`emit_sps_enable`] enables the emitting of the SPS structure with id of
    ///[`sps_id`].
    pub emit_sps_enable: Bool32,
    ///[`pps_id_entry_count`] is the number of entries in the
    ///[`pps_id_entries`].
    ///If this parameter is `0` then no pps entries are going to be emitted in
    ///the bitstream.
    pub pps_id_entry_count: u32,
    ///[`pps_id_entries`] is a pointer to an array of H.264 PPS IDs for the
    ///H.264 PPS to insert in the bitstream.
    ///The PPS IDs  **must**  match one of the IDs of the PPS(s) provided in
    ///`pPpsStd` of [`VideoEncodeH264SessionParametersCreateInfoEXT`]
    ///to identify the PPS parameter set to insert in the bitstream.
    ///This is retrieved from the [`VideoSessionParametersKHR`] object
    ///provided in [`VideoBeginCodingInfoKHR`].
    pub pps_id_entries: *const u8,
}
impl<'lt> Default for VideoEncodeH264EmitPictureParametersEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VideoEncodeH264EmitPictureParametersExt,
            p_next: std::ptr::null(),
            sps_id: 0,
            emit_sps_enable: 0,
            pps_id_entry_count: 0,
            pps_id_entries: std::ptr::null(),
        }
    }
}
impl<'lt> VideoEncodeH264EmitPictureParametersEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
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
    ///Gets the value of [`Self::sps_id`]
    pub fn sps_id(&self) -> u8 {
        self.sps_id
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
    ///Gets a mutable reference to the value of [`Self::sps_id`]
    pub fn sps_id_mut(&mut self) -> &mut u8 {
        &mut self.sps_id
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
    ///Sets the raw value of [`Self::sps_id`]
    pub fn set_sps_id(&mut self, value: u8) -> &mut Self {
        self.sps_id = value;
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
///`VK_VIDEO_CODEC_OPERATION_ENCODE_H264_BIT_EXT`.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_PROFILE_EXT`
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
#[doc(alias = "VkVideoEncodeH264ProfileEXT")]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeH264ProfileEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`std_profile_idc`] is a [`StdVideoH264ProfileIdc`] value specifying
    ///the H.264 codec profile IDC.
    pub std_profile_idc: StdVideoH264ProfileIdc,
}
impl<'lt> Default for VideoEncodeH264ProfileEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VideoEncodeH264ProfileExt,
            p_next: std::ptr::null(),
            std_profile_idc: unsafe { std::mem::zeroed() },
        }
    }
}
impl<'lt> VideoEncodeH264ProfileEXT<'lt> {
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
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::std_profile_idc`]
    pub fn std_profile_idc_mut(&mut self) -> &mut StdVideoH264ProfileIdc {
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
    pub fn set_std_profile_idc(&mut self, value: crate::native::StdVideoH264ProfileIdc) -> &mut Self {
        self.std_profile_idc = value;
        self
    }
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
/// - [`reference_final_lists`] is `NULL` or a pointer to a [`VideoEncodeH264ReferenceListsEXT`]
///   structure specifying the reference lists to be used for the current slice. If
///   [`reference_final_lists`] is not `NULL`, these reference lists override the reference lists
///   provided in [`VideoEncodeH264VclFrameInfoEXT`]::[`reference_final_lists`].
/// - [`slice_header_std`] is a pointer to a [`StdVideoEncodeH264SliceHeader`] structure specifying
///   the slice header for the current slice.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_NALU_SLICE_EXT`
/// - [`p_next`] **must**  be `NULL`
/// - If [`reference_final_lists`] is not `NULL`, [`reference_final_lists`] **must**  be a valid
///   pointer to a valid [`VideoEncodeH264ReferenceListsEXT`] structure
/// - [`slice_header_std`] **must**  be a valid pointer to a valid [`StdVideoEncodeH264SliceHeader`]
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
#[doc(alias = "VkVideoEncodeH264NaluSliceEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeH264NaluSliceEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`mb_count`] is the number of macroblocks in this slice.
    pub mb_count: u32,
    ///[`reference_final_lists`] is `NULL` or a pointer to a
    ///[`VideoEncodeH264ReferenceListsEXT`] structure specifying the
    ///reference lists to be used for the current slice.
    ///If [`reference_final_lists`] is not `NULL`, these reference lists
    ///override the reference lists provided in
    ///[`VideoEncodeH264VclFrameInfoEXT`]::[`reference_final_lists`].
    pub reference_final_lists: *const VideoEncodeH264ReferenceListsEXT<'lt>,
    ///[`slice_header_std`] is a pointer to a
    ///[`StdVideoEncodeH264SliceHeader`] structure specifying the slice header
    ///for the current slice.
    pub slice_header_std: *const StdVideoEncodeH264SliceHeader,
}
impl<'lt> Default for VideoEncodeH264NaluSliceEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VideoEncodeH264NaluSliceExt,
            p_next: std::ptr::null(),
            mb_count: 0,
            reference_final_lists: std::ptr::null(),
            slice_header_std: std::ptr::null(),
        }
    }
}
impl<'lt> VideoEncodeH264NaluSliceEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::reference_final_lists`]
    pub fn reference_final_lists_raw(&self) -> *const VideoEncodeH264ReferenceListsEXT<'lt> {
        self.reference_final_lists
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::reference_final_lists`]
    pub fn set_reference_final_lists_raw(&mut self, value: *const VideoEncodeH264ReferenceListsEXT<'lt>) -> &mut Self {
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
    ///Gets the value of [`Self::mb_count`]
    pub fn mb_count(&self) -> u32 {
        self.mb_count
    }
    ///Gets the value of [`Self::reference_final_lists`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn reference_final_lists(&self) -> &VideoEncodeH264ReferenceListsEXT<'lt> {
        &*self.reference_final_lists
    }
    ///Gets the value of [`Self::slice_header_std`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn slice_header_std(&self) -> &StdVideoEncodeH264SliceHeader {
        &*self.slice_header_std
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::mb_count`]
    pub fn mb_count_mut(&mut self) -> &mut u32 {
        &mut self.mb_count
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
    ///Sets the raw value of [`Self::mb_count`]
    pub fn set_mb_count(&mut self, value: u32) -> &mut Self {
        self.mb_count = value;
        self
    }
    ///Sets the raw value of [`Self::reference_final_lists`]
    pub fn set_reference_final_lists(
        &mut self,
        value: &'lt crate::extensions::ext_video_encode_h_264::VideoEncodeH264ReferenceListsEXT<'lt>,
    ) -> &mut Self {
        self.reference_final_lists = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::slice_header_std`]
    pub fn set_slice_header_std(&mut self, value: &'lt crate::native::StdVideoEncodeH264SliceHeader) -> &mut Self {
        self.slice_header_std = value as *const _;
        self
    }
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
///inferred picture type does not match the actual picture type.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_RATE_CONTROL_INFO_EXT`
/// - [`rate_control_structure`] **must**  be a valid
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
#[doc(alias = "VkVideoEncodeH264RateControlInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeH264RateControlInfoEXT<'lt> {
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
    ///[`VideoEncodeH264RateControlStructureFlagBitsEXT`] value specifying
    ///the expected encode stream reference structure, to aid in rate control
    ///calculations.
    pub rate_control_structure: VideoEncodeH264RateControlStructureFlagBitsEXT,
    ///[`temporal_layer_count`] specifies the number of temporal layers enabled
    ///in the stream.
    pub temporal_layer_count: u8,
}
impl<'lt> Default for VideoEncodeH264RateControlInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VideoEncodeH264RateControlInfoExt,
            p_next: std::ptr::null(),
            gop_frame_count: 0,
            idr_period: 0,
            consecutive_b_frame_count: 0,
            rate_control_structure: Default::default(),
            temporal_layer_count: 0,
        }
    }
}
impl<'lt> VideoEncodeH264RateControlInfoEXT<'lt> {
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
    pub fn rate_control_structure(&self) -> VideoEncodeH264RateControlStructureFlagBitsEXT {
        self.rate_control_structure
    }
    ///Gets the value of [`Self::temporal_layer_count`]
    pub fn temporal_layer_count(&self) -> u8 {
        self.temporal_layer_count
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
    pub fn rate_control_structure_mut(&mut self) -> &mut VideoEncodeH264RateControlStructureFlagBitsEXT {
        &mut self.rate_control_structure
    }
    ///Gets a mutable reference to the value of [`Self::temporal_layer_count`]
    pub fn temporal_layer_count_mut(&mut self) -> &mut u8 {
        &mut self.temporal_layer_count
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
        value: crate::extensions::ext_video_encode_h_264::VideoEncodeH264RateControlStructureFlagBitsEXT,
    ) -> &mut Self {
        self.rate_control_structure = value;
        self
    }
    ///Sets the raw value of [`Self::temporal_layer_count`]
    pub fn set_temporal_layer_count(&mut self, value: u8) -> &mut Self {
        self.temporal_layer_count = value;
        self
    }
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
#[doc(alias = "VkVideoEncodeH264QpEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VideoEncodeH264QpEXT {
    ///[`qp_i`] is the QP to be used for I-frames.
    pub qp_i: i32,
    ///[`qp_p`] is the QP to be used for P-frames.
    pub qp_p: i32,
    ///[`qp_b`] is the QP to be used for B-frames.
    pub qp_b: i32,
}
impl Default for VideoEncodeH264QpEXT {
    fn default() -> Self {
        Self {
            qp_i: 0,
            qp_p: 0,
            qp_b: 0,
        }
    }
}
impl VideoEncodeH264QpEXT {
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
#[doc(alias = "VkVideoEncodeH264FrameSizeEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VideoEncodeH264FrameSizeEXT {
    ///[`frame_i_size`] is the size in bytes to be used for I-frames.
    pub frame_i_size: u32,
    ///[`frame_p_size`] is the size in bytes to be used for P-frames.
    pub frame_p_size: u32,
    ///[`frame_b_size`] is the size in bytes to be used for B-frames.
    pub frame_b_size: u32,
}
impl Default for VideoEncodeH264FrameSizeEXT {
    fn default() -> Self {
        Self {
            frame_i_size: 0,
            frame_p_size: 0,
            frame_b_size: 0,
        }
    }
}
impl VideoEncodeH264FrameSizeEXT {
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
///   limits, however these limits  **may**  be used as a guide in rate control calculations. If
///   enabled and not set properly, the [`max_qp`] limit may prevent the implementation from
///   respecting the [`max_frame_size`] limit.
///# Description
///H.264-specific per-layer rate control parameters  **must**  be specified by
///adding a [`VideoEncodeH264RateControlLayerInfoEXT`] structure to the
///[`p_next`] chain of each [`VideoEncodeRateControlLayerInfoKHR`]
///structure in a call to [`CmdControlVideoCodingKHR`] command, when the
///command buffer context has an active video encode H.264 session.
///## Valid Usage
/// - When [`VideoEncodeRateControlInfoKHR::rate_control_mode`] is
///   `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`, both [`use_min_qp`] and [`use_max_qp`] must
///   be set to [`TRUE`].
/// - When [`VideoEncodeRateControlInfoKHR::rate_control_mode`] is
///   `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`, the values provided in `minQP` must be
///   identical to those provided in [`max_qp`].
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_RATE_CONTROL_LAYER_INFO_EXT`
/// - [`initial_rc_qp`] **must**  be a valid [`VideoEncodeH264QpEXT`] structure
/// - [`min_qp`] **must**  be a valid [`VideoEncodeH264QpEXT`] structure
/// - [`max_qp`] **must**  be a valid [`VideoEncodeH264QpEXT`] structure
/// - [`max_frame_size`] **must**  be a valid [`VideoEncodeH264FrameSizeEXT`] structure
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
#[doc(alias = "VkVideoEncodeH264RateControlLayerInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeH264RateControlLayerInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`temporal_layer_id`] specifies the H.264 temporal layer ID of the video
    ///coding layer that settings provided in this structure and its parent
    ///[`VideoEncodeRateControlLayerInfoKHR`] structure apply to.
    pub temporal_layer_id: u8,
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
    pub initial_rc_qp: VideoEncodeH264QpEXT,
    ///[`use_min_qp`] indicates whether the values within [`min_qp`] should be
    ///used by the implementation.
    ///When it is set to [`FALSE`], the implementation ignores the values
    ///in [`min_qp`] and chooses suitable values.
    pub use_min_qp: Bool32,
    ///[`min_qp`] provides the lower bound on the QP values for each picture
    ///type, to be used in rate control calculations.
    pub min_qp: VideoEncodeH264QpEXT,
    ///[`use_max_qp`] indicates whether the values within [`max_qp`] should be
    ///used by the implementation.
    ///When it is set to [`FALSE`], the implementation ignores the values
    ///in [`max_qp`] and chooses suitable values.
    pub use_max_qp: Bool32,
    ///[`max_qp`] provides the upper bound on the QP values for each picture
    ///type, to be used in rate control calculations.
    pub max_qp: VideoEncodeH264QpEXT,
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
    pub max_frame_size: VideoEncodeH264FrameSizeEXT,
}
impl<'lt> Default for VideoEncodeH264RateControlLayerInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VideoEncodeH264RateControlLayerInfoExt,
            p_next: std::ptr::null(),
            temporal_layer_id: 0,
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
impl<'lt> VideoEncodeH264RateControlLayerInfoEXT<'lt> {
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
    ///Gets the value of [`Self::temporal_layer_id`]
    pub fn temporal_layer_id(&self) -> u8 {
        self.temporal_layer_id
    }
    ///Gets the value of [`Self::use_initial_rc_qp`]
    pub fn use_initial_rc_qp(&self) -> bool {
        unsafe { std::mem::transmute(self.use_initial_rc_qp as u8) }
    }
    ///Gets the value of [`Self::initial_rc_qp`]
    pub fn initial_rc_qp(&self) -> VideoEncodeH264QpEXT {
        self.initial_rc_qp
    }
    ///Gets the value of [`Self::use_min_qp`]
    pub fn use_min_qp(&self) -> bool {
        unsafe { std::mem::transmute(self.use_min_qp as u8) }
    }
    ///Gets the value of [`Self::min_qp`]
    pub fn min_qp(&self) -> VideoEncodeH264QpEXT {
        self.min_qp
    }
    ///Gets the value of [`Self::use_max_qp`]
    pub fn use_max_qp(&self) -> bool {
        unsafe { std::mem::transmute(self.use_max_qp as u8) }
    }
    ///Gets the value of [`Self::max_qp`]
    pub fn max_qp(&self) -> VideoEncodeH264QpEXT {
        self.max_qp
    }
    ///Gets the value of [`Self::use_max_frame_size`]
    pub fn use_max_frame_size(&self) -> bool {
        unsafe { std::mem::transmute(self.use_max_frame_size as u8) }
    }
    ///Gets the value of [`Self::max_frame_size`]
    pub fn max_frame_size(&self) -> VideoEncodeH264FrameSizeEXT {
        self.max_frame_size
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::temporal_layer_id`]
    pub fn temporal_layer_id_mut(&mut self) -> &mut u8 {
        &mut self.temporal_layer_id
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
    pub fn initial_rc_qp_mut(&mut self) -> &mut VideoEncodeH264QpEXT {
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
    pub fn min_qp_mut(&mut self) -> &mut VideoEncodeH264QpEXT {
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
    pub fn max_qp_mut(&mut self) -> &mut VideoEncodeH264QpEXT {
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
    pub fn max_frame_size_mut(&mut self) -> &mut VideoEncodeH264FrameSizeEXT {
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
    ///Sets the raw value of [`Self::temporal_layer_id`]
    pub fn set_temporal_layer_id(&mut self, value: u8) -> &mut Self {
        self.temporal_layer_id = value;
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
        value: crate::extensions::ext_video_encode_h_264::VideoEncodeH264QpEXT,
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
    pub fn set_min_qp(&mut self, value: crate::extensions::ext_video_encode_h_264::VideoEncodeH264QpEXT) -> &mut Self {
        self.min_qp = value;
        self
    }
    ///Sets the raw value of [`Self::use_max_qp`]
    pub fn set_use_max_qp(&mut self, value: bool) -> &mut Self {
        self.use_max_qp = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::max_qp`]
    pub fn set_max_qp(&mut self, value: crate::extensions::ext_video_encode_h_264::VideoEncodeH264QpEXT) -> &mut Self {
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
        value: crate::extensions::ext_video_encode_h_264::VideoEncodeH264FrameSizeEXT,
    ) -> &mut Self {
        self.max_frame_size = value;
        self
    }
}
