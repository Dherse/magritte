//![VK_EXT_video_encode_h265](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_video_encode_h265.html) - device extension
//!# Description
//!This extension allows applications to compress a raw video sequence by using
//!the H.265/HEVC video compression standard.
//!# Revision
//!3
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_video_encode_queue`]`
//! - **This is a *provisional* extension and **must** be used with caution.
//!See the [description](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#boilerplate-provisional-header) of provisional header files for enablement and stability details.**
//!# Contacts
//! - Ahmed Abdelkhalek [aabdelkh](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_video_encode_h265]
//!   @aabdelkh%0A<<Here describe the issue or question you have about the VK_EXT_video_encode_h265
//!   extension>>)
//!# New structures
//! - [`VideoEncodeH265DpbSlotInfoEXT`]
//! - [`VideoEncodeH265FrameSizeEXT`]
//! - [`VideoEncodeH265NaluSliceEXT`]
//! - [`VideoEncodeH265QpEXT`]
//! - [`VideoEncodeH265ReferenceListsEXT`]
//! - Extending [`VideoCapabilitiesKHR`]:
//! - [`VideoEncodeH265CapabilitiesEXT`]
//! - Extending [`VideoEncodeInfoKHR`]:
//! - [`VideoEncodeH265EmitPictureParametersEXT`]
//! - [`VideoEncodeH265VclFrameInfoEXT`]
//! - Extending [`VideoEncodeRateControlInfoKHR`]:
//! - [`VideoEncodeH265RateControlInfoEXT`]
//! - Extending [`VideoEncodeRateControlLayerInfoKHR`]:
//! - [`VideoEncodeH265RateControlLayerInfoEXT`]
//! - Extending [`VideoProfileKHR`], [`QueryPoolCreateInfo`], [`FormatProperties2`],
//!   [`ImageCreateInfo`], [`ImageViewCreateInfo`], [`BufferCreateInfo`]:
//! - [`VideoEncodeH265ProfileEXT`]
//! - Extending [`VideoSessionCreateInfoKHR`]:
//! - [`VideoEncodeH265SessionCreateInfoEXT`]
//! - Extending [`VideoSessionParametersCreateInfoKHR`]:
//! - [`VideoEncodeH265SessionParametersCreateInfoEXT`]
//! - Extending [`VideoSessionParametersUpdateInfoKHR`]:
//! - [`VideoEncodeH265SessionParametersAddInfoEXT`]
//!# New enums
//! - [`VideoEncodeH265CtbSizeFlagBitsEXT`]
//! - [`VideoEncodeH265InputModeFlagBitsEXT`]
//! - [`VideoEncodeH265OutputModeFlagBitsEXT`]
//! - [`VideoEncodeH265RateControlStructureFlagBitsEXT`]
//!# New bitmasks
//! - [`VideoEncodeH265CapabilityFlagsEXT`]
//! - [`VideoEncodeH265CreateFlagsEXT`]
//! - [`VideoEncodeH265CtbSizeFlagsEXT`]
//! - [`VideoEncodeH265InputModeFlagsEXT`]
//! - [`VideoEncodeH265OutputModeFlagsEXT`]
//! - [`VideoEncodeH265RateControlStructureFlagsEXT`]
//!# New constants
//! - [`EXT_VIDEO_ENCODE_H265_EXTENSION_NAME`]
//! - [`EXT_VIDEO_ENCODE_H265_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_CAPABILITIES_EXT`
//! - `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_DPB_SLOT_INFO_EXT`
//! - `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_EMIT_PICTURE_PARAMETERS_EXT`
//! - `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_NALU_SLICE_EXT`
//! - `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_PROFILE_EXT`
//! - `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_RATE_CONTROL_INFO_EXT`
//! - `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_RATE_CONTROL_LAYER_INFO_EXT`
//! - `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_REFERENCE_LISTS_EXT`
//! - `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_SESSION_CREATE_INFO_EXT`
//! - `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_SESSION_PARAMETERS_ADD_INFO_EXT`
//! - `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_SESSION_PARAMETERS_CREATE_INFO_EXT`
//! - `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_VCL_FRAME_INFO_EXT`
//! - Extending [`VideoCodecOperationFlagBitsKHR`]:
//! - `VK_VIDEO_CODEC_OPERATION_ENCODE_H265_BIT_EXT`
//!# Version History
//! - Revision 0, 2019-11-14 (Ahmed Abdelkhalek)
//! - Initial draft
//! - Revision 0.5, 2020-02-13 (Tony Zlatinski)
//! - General Spec cleanup
//! - Added DPB structures
//! - Change the VCL frame encode structure
//! - Added a common Non-VCL Picture Paramarameters structure
//! - Revision 2, Oct 10 2021 (Srinath Kumarapuram)
//! - Vulkan Video Encode h.265 update and spec edits
//! - Revision 3, 2021-12-08 (Ahmed Abdelkhalek)
//! - Rate control updates
//!# Other info
//! * 2021-12-08
//! * No known IP claims.
//!*
//! - Ahmed Abdelkhalek, AMD
//! - George Hao, AMD
//! - Jake Beju, AMD
//! - Chunbo Chen, Intel
//! - Ping Liu, Intel
//! - Srinath Kumarapuram, NVIDIA
//! - Tony Zlatinski, NVIDIA
//!# Related
//! - [`VideoEncodeH265CapabilitiesEXT`]
//! - [`VideoEncodeH265CapabilityFlagsEXT`]
//! - [`VideoEncodeH265CreateFlagsEXT`]
//! - [`VideoEncodeH265CtbSizeFlagBitsEXT`]
//! - [`VideoEncodeH265CtbSizeFlagsEXT`]
//! - [`VideoEncodeH265DpbSlotInfoEXT`]
//! - [`VideoEncodeH265EmitPictureParametersEXT`]
//! - [`VideoEncodeH265FrameSizeEXT`]
//! - [`VideoEncodeH265InputModeFlagBitsEXT`]
//! - [`VideoEncodeH265InputModeFlagsEXT`]
//! - [`VideoEncodeH265NaluSliceEXT`]
//! - [`VideoEncodeH265OutputModeFlagBitsEXT`]
//! - [`VideoEncodeH265OutputModeFlagsEXT`]
//! - [`VideoEncodeH265ProfileEXT`]
//! - [`VideoEncodeH265QpEXT`]
//! - [`VideoEncodeH265RateControlInfoEXT`]
//! - [`VideoEncodeH265RateControlLayerInfoEXT`]
//! - [`VideoEncodeH265RateControlStructureFlagBitsEXT`]
//! - [`VideoEncodeH265RateControlStructureFlagsEXT`]
//! - [`VideoEncodeH265ReferenceListsEXT`]
//! - [`VideoEncodeH265SessionCreateInfoEXT`]
//! - [`VideoEncodeH265SessionParametersAddInfoEXT`]
//! - [`VideoEncodeH265SessionParametersCreateInfoEXT`]
//! - [`VideoEncodeH265VclFrameInfoEXT`]
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
#[doc(alias = "VK_EXT_VIDEO_ENCODE_H265_SPEC_VERSION")]
pub const EXT_VIDEO_ENCODE_H265_SPEC_VERSION: u32 = 3;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_VIDEO_ENCODE_H265_EXTENSION_NAME")]
pub const EXT_VIDEO_ENCODE_H265_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_video_encode_h265");
