//![VK_EXT_video_decode_h265](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_video_decode_h265.html) - device extension
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_video_decode_queue`]`
//! - **This is a *provisional* extension and **must** be used with caution.
//!See the [description](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#boilerplate-provisional-header) of provisional header files for enablement and stability details.**
//!# Contacts
//! - [peter.fang@amd.com]()
//!# New structures
//! - Extending [`VideoCapabilitiesKHR`]:
//! - [`VideoDecodeH265CapabilitiesEXT`]
//!
//! - Extending [`VideoDecodeInfoKHR`]:
//! - [`VideoDecodeH265PictureInfoEXT`]
//!
//! - Extending [`VideoProfileKHR`], [`QueryPoolCreateInfo`], [`FormatProperties2`],
//!   [`ImageCreateInfo`], [`ImageViewCreateInfo`], [`BufferCreateInfo`]:
//! - [`VideoDecodeH265ProfileEXT`]
//!
//! - Extending [`VideoReferenceSlotKHR`]:
//! - [`VideoDecodeH265DpbSlotInfoEXT`]
//!
//! - Extending [`VideoSessionCreateInfoKHR`]:
//! - [`VideoDecodeH265SessionCreateInfoEXT`]
//!
//! - Extending [`VideoSessionParametersCreateInfoKHR`]:
//! - [`VideoDecodeH265SessionParametersCreateInfoEXT`]
//!
//! - Extending [`VideoSessionParametersUpdateInfoKHR`]:
//! - [`VideoDecodeH265SessionParametersAddInfoEXT`]
//!# New bitmasks
//! - [`VideoDecodeH265CreateFlagsEXT`]
//!# New constants
//! - [`EXT_VIDEO_DECODE_H265_EXTENSION_NAME`]
//! - [`EXT_VIDEO_DECODE_H265_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_CAPABILITIES_EXT`
//! - `VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_DPB_SLOT_INFO_EXT`
//! - `VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_PICTURE_INFO_EXT`
//! - `VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_PROFILE_EXT`
//! - `VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_SESSION_CREATE_INFO_EXT`
//! - `VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_SESSION_PARAMETERS_ADD_INFO_EXT`
//! - `VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_SESSION_PARAMETERS_CREATE_INFO_EXT`
//!
//! - Extending [`VideoCodecOperationFlagBitsKHR`]:
//! - `VK_VIDEO_CODEC_OPERATION_DECODE_H265_BIT_EXT`
//!# Version History
//! - Revision 1, 2018-6-11 (Peter Fang)
//! - Initial draft
//!
//! - Revision 1.6, March 29 2021 (Tony Zlatinski)
//! - Spec and API updates.
//!# Other info
//! * 2021-03-29
//! * No known IP claims.
//!*
//! - HoHin Lau, AMD
//! - Jake Beju, AMD
//! - Peter Fang, AMD
//! - Ping Liu, Intel
//! - Srinath Kumarapuram, NVIDIA
//! - Tony Zlatinski, NVIDIA
//!# Related
//! - [`VideoDecodeH265CapabilitiesEXT`]
//! - [`VideoDecodeH265CreateFlagsEXT`]
//! - [`VideoDecodeH265DpbSlotInfoEXT`]
//! - [`VideoDecodeH265PictureInfoEXT`]
//! - [`VideoDecodeH265ProfileEXT`]
//! - [`VideoDecodeH265SessionCreateInfoEXT`]
//! - [`VideoDecodeH265SessionParametersAddInfoEXT`]
//! - [`VideoDecodeH265SessionParametersCreateInfoEXT`]
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
#[doc(alias = "VK_EXT_VIDEO_DECODE_H265_SPEC_VERSION")]
pub const EXT_VIDEO_DECODE_H265_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_VIDEO_DECODE_H265_EXTENSION_NAME")]
pub const EXT_VIDEO_DECODE_H265_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_video_decode_h265");
