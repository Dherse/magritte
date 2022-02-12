//![VK_EXT_video_encode_h264](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_video_encode_h264.html) - device extension
//!# Description
//!This extension allows applications to compress a raw video sequence by using
//!the H.264/AVC video compression standard.
//!# Revision
//!3
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_video_encode_queue`]`
//! - **This is a *provisional* extension and **must** be used with caution.
//!See the [description](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#boilerplate-provisional-header) of provisional header files for enablement and stability details.**
//!# Contacts
//! - Ahmed Abdelkhalek [aabdelkh](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_video_encode_h264]
//!   @aabdelkh%0A<<Here describe the issue or question you have about the VK_EXT_video_encode_h264
//!   extension>>)
//!# New structures
//! - [`VideoEncodeH264DpbSlotInfoEXT`]
//! - [`VideoEncodeH264FrameSizeEXT`]
//! - [`VideoEncodeH264NaluSliceEXT`]
//! - [`VideoEncodeH264QpEXT`]
//! - Extending [`VideoCapabilitiesKHR`]:
//! - [`VideoEncodeH264CapabilitiesEXT`]
//! - Extending [`VideoEncodeInfoKHR`]:
//! - [`VideoEncodeH264EmitPictureParametersEXT`]
//! - [`VideoEncodeH264VclFrameInfoEXT`]
//! - Extending [`VideoEncodeRateControlInfoKHR`]:
//! - [`VideoEncodeH264RateControlInfoEXT`]
//! - Extending [`VideoEncodeRateControlLayerInfoKHR`]:
//! - [`VideoEncodeH264RateControlLayerInfoEXT`]
//! - Extending [`VideoProfileKHR`], [`QueryPoolCreateInfo`], [`FormatProperties2`],
//!   [`ImageCreateInfo`], [`ImageViewCreateInfo`], [`BufferCreateInfo`]:
//! - [`VideoEncodeH264ProfileEXT`]
//! - Extending [`VideoSessionCreateInfoKHR`]:
//! - [`VideoEncodeH264SessionCreateInfoEXT`]
//! - Extending [`VideoSessionParametersCreateInfoKHR`]:
//! - [`VideoEncodeH264SessionParametersCreateInfoEXT`]
//! - Extending [`VideoSessionParametersUpdateInfoKHR`]:
//! - [`VideoEncodeH264SessionParametersAddInfoEXT`]
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
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_CAPABILITIES_EXT`
//! - `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_DPB_SLOT_INFO_EXT`
//! - `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_EMIT_PICTURE_PARAMETERS_EXT`
//! - `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_NALU_SLICE_EXT`
//! - `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_PROFILE_EXT`
//! - `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_RATE_CONTROL_INFO_EXT`
//! - `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_RATE_CONTROL_LAYER_INFO_EXT`
//! - `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_SESSION_CREATE_INFO_EXT`
//! - `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_SESSION_PARAMETERS_ADD_INFO_EXT`
//! - `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_SESSION_PARAMETERS_CREATE_INFO_EXT`
//! - `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_VCL_FRAME_INFO_EXT`
//! - Extending [`VideoCodecOperationFlagBitsKHR`]:
//! - `VK_VIDEO_CODEC_OPERATION_ENCODE_H264_BIT_EXT`
//!# Version History
//! - Revision 0, 2018-7-23 (Ahmed Abdelkhalek)
//! - Initial draft
//! - Revision 0.5, 2020-02-13 (Tony Zlatinski)
//! - General Spec cleanup
//! - Added DPB structures
//! - Change the VCL frame encode structure
//! - Added a common Non-VCL Picture Paramarameters structure
//! - Revision 1, 2021-03-29 (Tony Zlatinski)
//! - Spec and API updates
//! - Revision 2, August 1 2021 (Srinath Kumarapuram)
//! - Rename `VkVideoEncodeH264CapabilitiesFlagsEXT` to
//![`VideoEncodeH264CapabilityFlagsEXT`] and
//!`VkVideoEncodeH264CapabilitiesFlagsEXT` to
//![`VideoEncodeH264CapabilityFlagsEXT`], following Vulkan naming
//!conventions.
//! - Revision 3, 2021-12-08 (Ahmed Abdelkhalek)
//! - Rate control updates
//!# Other info
//! * 2021-12-08
//! * No known IP claims.
//!*
//! - Ahmed Abdelkhalek, AMD
//! - Daniel Rakos, AMD
//! - George Hao, AMD
//! - Jake Beju, AMD
//! - Peter Fang, AMD
//! - Ping Liu, Intel
//! - Srinath Kumarapuram, NVIDIA
//! - Tony Zlatinski, NVIDIA
//! - Yang Liu, AMD
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
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_VIDEO_ENCODE_H264_SPEC_VERSION")]
pub const EXT_VIDEO_ENCODE_H264_SPEC_VERSION: u32 = 3;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_VIDEO_ENCODE_H264_EXTENSION_NAME")]
pub const EXT_VIDEO_ENCODE_H264_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_video_encode_h264");
