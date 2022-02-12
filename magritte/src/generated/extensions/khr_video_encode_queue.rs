//![VK_KHR_video_encode_queue](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_video_encode_queue.html) - device extension
//!# Revision
//!3
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_video_queue`]`
//! - Requires `[`VK_KHR_synchronization2`]`
//! - **This is a *provisional* extension and **must** be used with caution.
//!See the [description](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#boilerplate-provisional-header) of provisional header files for enablement and stability details.**
//!# Contacts
//! - [ahmed.abdelkalek@amd.com]()
//!# New functions & commands
//! - [`CmdEncodeVideoKHR`]
//!# New structures
//! - [`VideoEncodeInfoKHR`]
//! - Extending [`VideoCodingControlInfoKHR`]:
//! - [`VideoEncodeRateControlInfoKHR`]
//! - [`VideoEncodeRateControlLayerInfoKHR`]
//!# New enums
//! - [`VideoEncodeFlagBitsKHR`]
//! - [`VideoEncodeRateControlFlagBitsKHR`]
//! - [`VideoEncodeRateControlModeFlagBitsKHR`]
//!# New bitmasks
//! - [`VideoEncodeFlagsKHR`]
//! - [`VideoEncodeRateControlFlagsKHR`]
//! - [`VideoEncodeRateControlModeFlagsKHR`]
//!# New constants
//! - [`KHR_VIDEO_ENCODE_QUEUE_EXTENSION_NAME`]
//! - [`KHR_VIDEO_ENCODE_QUEUE_SPEC_VERSION`]
//! - Extending [`AccessFlagBits2`]:
//! - `VK_ACCESS_2_VIDEO_ENCODE_READ_BIT_KHR`
//! - `VK_ACCESS_2_VIDEO_ENCODE_WRITE_BIT_KHR`
//!
//! - Extending [`BufferUsageFlagBits`]:
//! - `VK_BUFFER_USAGE_VIDEO_ENCODE_DST_BIT_KHR`
//! - `VK_BUFFER_USAGE_VIDEO_ENCODE_SRC_BIT_KHR`
//!
//! - Extending [`FormatFeatureFlagBits`]:
//! - `VK_FORMAT_FEATURE_VIDEO_ENCODE_DPB_BIT_KHR`
//! - `VK_FORMAT_FEATURE_VIDEO_ENCODE_INPUT_BIT_KHR`
//!
//! - Extending [`ImageLayout`]:
//! - `VK_IMAGE_LAYOUT_VIDEO_ENCODE_DPB_KHR`
//! - `VK_IMAGE_LAYOUT_VIDEO_ENCODE_DST_KHR`
//! - `VK_IMAGE_LAYOUT_VIDEO_ENCODE_SRC_KHR`
//!
//! - Extending [`ImageUsageFlagBits`]:
//! - `VK_IMAGE_USAGE_VIDEO_ENCODE_DPB_BIT_KHR`
//! - `VK_IMAGE_USAGE_VIDEO_ENCODE_DST_BIT_KHR`
//! - `VK_IMAGE_USAGE_VIDEO_ENCODE_SRC_BIT_KHR`
//!
//! - Extending [`PipelineStageFlagBits2`]:
//! - `VK_PIPELINE_STAGE_2_VIDEO_ENCODE_BIT_KHR`
//!
//! - Extending [`QueryType`]:
//! - `VK_QUERY_TYPE_VIDEO_ENCODE_BITSTREAM_BUFFER_RANGE_KHR`
//!
//! - Extending [`QueueFlagBits`]:
//! - `VK_QUEUE_VIDEO_ENCODE_BIT_KHR`
//!
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_VIDEO_ENCODE_INFO_KHR`
//! - `VK_STRUCTURE_TYPE_VIDEO_ENCODE_RATE_CONTROL_INFO_KHR`
//! - `VK_STRUCTURE_TYPE_VIDEO_ENCODE_RATE_CONTROL_LAYER_INFO_KHR`
//!
//!If [`VK_KHR_format_feature_flags2`] is supported:
//! - Extending [`FormatFeatureFlagBits2`]:
//! - `VK_FORMAT_FEATURE_2_VIDEO_ENCODE_DPB_BIT_KHR`
//! - `VK_FORMAT_FEATURE_2_VIDEO_ENCODE_INPUT_BIT_KHR`
//!# Version History
//! - Revision 1, 2018-07-23 (Ahmed Abdelkhalek)
//! - Initial draft
//!
//! - Revision 1.1, 10/29/2019 (Tony Zlatinski)
//! - Updated the reserved spec tokens and renamed VkVideoEncoderKHR to
//!VkVideoSessionKHR
//!
//! - Revision 1.6, Jan 08 2020 (Tony Zlatinski)
//! - API unify with the video_decode_queue spec
//!
//! - Revision 2, March 29 2021 (Tony Zlatinski)
//! - Spec and API updates.
//!
//! - Revision 3, 2021-09-30 (Jon Leech)
//! - Add interaction with `[`VK_KHR_format_feature_flags2`]` to `vk.xml`
//!# Other info
//! * 2021-09-30
//! * No known IP claims.
//!*
//! - Ahmed Abdelkhalek, AMD
//! - Damien Kessler, NVIDIA
//! - Daniel Rakos, AMD
//! - George Hao, AMD
//! - Jake Beju, AMD
//! - Peter Fang, AMD
//! - Piers Daniell, NVIDIA
//! - Srinath Kumarapuram, NVIDIA
//! - Thomas J. Meier, NVIDIA
//! - Tony Zlatinski, NVIDIA
//! - Yang Liu, AMD
//!# Related
//! - [`VideoEncodeFlagBitsKHR`]
//! - [`VideoEncodeFlagsKHR`]
//! - [`VideoEncodeInfoKHR`]
//! - [`VideoEncodeRateControlFlagBitsKHR`]
//! - [`VideoEncodeRateControlFlagsKHR`]
//! - [`VideoEncodeRateControlInfoKHR`]
//! - [`VideoEncodeRateControlLayerInfoKHR`]
//! - [`VideoEncodeRateControlModeFlagBitsKHR`]
//! - [`VideoEncodeRateControlModeFlagsKHR`]
//! - [`CmdEncodeVideoKHR`]
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
#[doc(alias = "VK_KHR_VIDEO_ENCODE_QUEUE_SPEC_VERSION")]
pub const KHR_VIDEO_ENCODE_QUEUE_SPEC_VERSION: u32 = 3;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_VIDEO_ENCODE_QUEUE_EXTENSION_NAME")]
pub const KHR_VIDEO_ENCODE_QUEUE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_video_encode_queue");
