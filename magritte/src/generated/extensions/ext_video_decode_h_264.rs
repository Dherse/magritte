//![VK_EXT_video_decode_h264](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_video_decode_h264.html) - device extension
//!# Revision
//!3
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_video_decode_queue`]`
//! - **This is a *provisional* extension and **must** be used with caution.
//!See the [description](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#boilerplate-provisional-header) of provisional header files for enablement and stability details.**
//!# Contacts
//! - [peter.fang@amd.com]()
//!# New structures
//! - Extending [`VideoCapabilitiesKHR`]:
//! - [`VideoDecodeH264CapabilitiesEXT`]
//! - Extending [`VideoDecodeH264PictureInfoEXT`]:
//! - [`VideoDecodeH264MvcEXT`]
//! - Extending [`VideoDecodeInfoKHR`]:
//! - [`VideoDecodeH264PictureInfoEXT`]
//! - Extending [`VideoProfileKHR`], [`QueryPoolCreateInfo`], [`FormatProperties2`],
//!   [`ImageCreateInfo`], [`ImageViewCreateInfo`], [`BufferCreateInfo`]:
//! - [`VideoDecodeH264ProfileEXT`]
//! - Extending [`VideoReferenceSlotKHR`]:
//! - [`VideoDecodeH264DpbSlotInfoEXT`]
//! - Extending [`VideoSessionCreateInfoKHR`]:
//! - [`VideoDecodeH264SessionCreateInfoEXT`]
//! - Extending [`VideoSessionParametersCreateInfoKHR`]:
//! - [`VideoDecodeH264SessionParametersCreateInfoEXT`]
//! - Extending [`VideoSessionParametersUpdateInfoKHR`]:
//! - [`VideoDecodeH264SessionParametersAddInfoEXT`]
//!# New enums
//! - [`VideoDecodeH264PictureLayoutFlagBitsEXT`]
//!# New bitmasks
//! - [`VideoDecodeH264CreateFlagsEXT`]
//! - [`VideoDecodeH264PictureLayoutFlagsEXT`]
//!# New constants
//! - [`EXT_VIDEO_DECODE_H264_EXTENSION_NAME`]
//! - [`EXT_VIDEO_DECODE_H264_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_CAPABILITIES_EXT`
//! - `VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_DPB_SLOT_INFO_EXT`
//! - `VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_MVC_EXT`
//! - `VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_PICTURE_INFO_EXT`
//! - `VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_PROFILE_EXT`
//! - `VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_SESSION_CREATE_INFO_EXT`
//! - `VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_SESSION_PARAMETERS_ADD_INFO_EXT`
//! - `VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_SESSION_PARAMETERS_CREATE_INFO_EXT`
//! - Extending [`VideoCodecOperationFlagBitsKHR`]:
//! - `VK_VIDEO_CODEC_OPERATION_DECODE_H264_BIT_EXT`
//!# Version History
//! - Revision 1, 2018-6-11 (Peter Fang)
//! - Initial draft
//! - Revision 2, March 29 2021 (Tony Zlatinski)
//! - Spec and API Updates
//! - Revision 3, August 1 2021 (Srinath Kumarapuram)
//! - Rename `VkVideoDecodeH264FieldLayoutFlagsEXT` to
//![`VideoDecodeH264PictureLayoutFlagsEXT`],
//!`VkVideoDecodeH264FieldLayoutFlagBitsEXT` to
//![`VideoDecodeH264PictureLayoutFlagBitsEXT`] (along with the names of
//!enumerants it defines), and `VkVideoDecodeH264ProfileEXT.fieldLayout`
//!to `VkVideoDecodeH264ProfileEXT.pictureLayout`, following Vulkan
//!naming conventions.
//!# Other info
//! * 2021-03-29
//! * No known IP claims.
//!*
//! - Chunbo Chen, Intel
//! - HoHin Lau, AMD
//! - Jake Beju, AMD
//! - Peter Fang, AMD
//! - Ping Liu, Intel
//! - Srinath Kumarapuram, NVIDIA
//! - Tony Zlatinski, NVIDIA
//!# Related
//! - [`VideoDecodeH264CapabilitiesEXT`]
//! - [`VideoDecodeH264CreateFlagsEXT`]
//! - [`VideoDecodeH264DpbSlotInfoEXT`]
//! - [`VideoDecodeH264MvcEXT`]
//! - [`VideoDecodeH264PictureInfoEXT`]
//! - [`VideoDecodeH264PictureLayoutFlagBitsEXT`]
//! - [`VideoDecodeH264PictureLayoutFlagsEXT`]
//! - [`VideoDecodeH264ProfileEXT`]
//! - [`VideoDecodeH264SessionCreateInfoEXT`]
//! - [`VideoDecodeH264SessionParametersAddInfoEXT`]
//! - [`VideoDecodeH264SessionParametersCreateInfoEXT`]
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
#[doc(alias = "VK_EXT_VIDEO_DECODE_H264_SPEC_VERSION")]
pub const EXT_VIDEO_DECODE_H264_SPEC_VERSION: u32 = 3;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_VIDEO_DECODE_H264_EXTENSION_NAME")]
pub const EXT_VIDEO_DECODE_H264_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_video_decode_h264");
