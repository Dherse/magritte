//![VK_KHR_video_queue](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_video_queue.html) - device extension
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//! - Requires `[`VK_KHR_sampler_ycbcr_conversion`]`
//! - **This is a *provisional* extension and **must** be used with caution.
//!See the [description](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#boilerplate-provisional-header) of provisional header files for enablement and stability details.**
//!# Contacts
//! - Tony Zlatinski [tzlatinski](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_video_queue]
//!   @tzlatinski%0A<<Here describe the issue or question you have about the VK_KHR_video_queue
//!   extension>>)
//!# New handles
//! - [`VideoSessionKHR`]
//! - [`VideoSessionParametersKHR`]
//!# New functions & commands
//! - [`BindVideoSessionMemoryKHR`]
//! - [`CmdBeginVideoCodingKHR`]
//! - [`CmdControlVideoCodingKHR`]
//! - [`CmdEndVideoCodingKHR`]
//! - [`CreateVideoSessionKHR`]
//! - [`CreateVideoSessionParametersKHR`]
//! - [`DestroyVideoSessionKHR`]
//! - [`DestroyVideoSessionParametersKHR`]
//! - [`GetPhysicalDeviceVideoCapabilitiesKHR`]
//! - [`GetPhysicalDeviceVideoFormatPropertiesKHR`]
//! - [`GetVideoSessionMemoryRequirementsKHR`]
//! - [`UpdateVideoSessionParametersKHR`]
//!# New structures
//! - [`PhysicalDeviceVideoFormatInfoKHR`]
//! - [`VideoBeginCodingInfoKHR`]
//! - [`VideoBindMemoryKHR`]
//! - [`VideoCapabilitiesKHR`]
//! - [`VideoCodingControlInfoKHR`]
//! - [`VideoEndCodingInfoKHR`]
//! - [`VideoFormatPropertiesKHR`]
//! - [`VideoGetMemoryPropertiesKHR`]
//! - [`VideoPictureResourceKHR`]
//! - [`VideoReferenceSlotKHR`]
//! - [`VideoSessionCreateInfoKHR`]
//! - [`VideoSessionParametersCreateInfoKHR`]
//! - [`VideoSessionParametersUpdateInfoKHR`]
//! - Extending [`FormatProperties2`], [`ImageCreateInfo`], [`ImageViewCreateInfo`],
//!   [`BufferCreateInfo`]:
//! - [`VideoProfilesKHR`]
//!
//! - Extending [`QueryPoolCreateInfo`], [`FormatProperties2`], [`ImageCreateInfo`],
//!   [`ImageViewCreateInfo`], [`BufferCreateInfo`]:
//! - [`VideoProfileKHR`]
//!
//! - Extending [`QueueFamilyProperties2`]:
//! - [`QueueFamilyQueryResultStatusProperties2KHR`]
//! - [`VideoQueueFamilyProperties2KHR`]
//!# New enums
//! - [`QueryResultStatusKHR`]
//! - [`VideoCapabilityFlagBitsKHR`]
//! - [`VideoChromaSubsamplingFlagBitsKHR`]
//! - [`VideoCodecOperationFlagBitsKHR`]
//! - [`VideoCodingControlFlagBitsKHR`]
//! - [`VideoCodingQualityPresetFlagBitsKHR`]
//! - [`VideoComponentBitDepthFlagBitsKHR`]
//! - [`VideoSessionCreateFlagBitsKHR`]
//!# New bitmasks
//! - [`VideoBeginCodingFlagsKHR`]
//! - [`VideoCapabilityFlagsKHR`]
//! - [`VideoChromaSubsamplingFlagsKHR`]
//! - [`VideoCodecOperationFlagsKHR`]
//! - [`VideoCodingControlFlagsKHR`]
//! - [`VideoCodingQualityPresetFlagsKHR`]
//! - [`VideoComponentBitDepthFlagsKHR`]
//! - [`VideoEndCodingFlagsKHR`]
//! - [`VideoSessionCreateFlagsKHR`]
//!# New constants
//! - [`KHR_VIDEO_QUEUE_EXTENSION_NAME`]
//! - [`KHR_VIDEO_QUEUE_SPEC_VERSION`]
//! - Extending [`ObjectType`]:
//! - `VK_OBJECT_TYPE_VIDEO_SESSION_KHR`
//! - `VK_OBJECT_TYPE_VIDEO_SESSION_PARAMETERS_KHR`
//!
//! - Extending [`QueryResultFlagBits`]:
//! - `VK_QUERY_RESULT_WITH_STATUS_BIT_KHR`
//!
//! - Extending [`QueryType`]:
//! - `VK_QUERY_TYPE_RESULT_STATUS_ONLY_KHR`
//!
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR`
//! - `VK_STRUCTURE_TYPE_QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_2_KHR`
//! - `VK_STRUCTURE_TYPE_VIDEO_BEGIN_CODING_INFO_KHR`
//! - `VK_STRUCTURE_TYPE_VIDEO_BIND_MEMORY_KHR`
//! - `VK_STRUCTURE_TYPE_VIDEO_CAPABILITIES_KHR`
//! - `VK_STRUCTURE_TYPE_VIDEO_CODING_CONTROL_INFO_KHR`
//! - `VK_STRUCTURE_TYPE_VIDEO_END_CODING_INFO_KHR`
//! - `VK_STRUCTURE_TYPE_VIDEO_FORMAT_PROPERTIES_KHR`
//! - `VK_STRUCTURE_TYPE_VIDEO_GET_MEMORY_PROPERTIES_KHR`
//! - `VK_STRUCTURE_TYPE_VIDEO_PICTURE_RESOURCE_KHR`
//! - `VK_STRUCTURE_TYPE_VIDEO_PROFILES_KHR`
//! - `VK_STRUCTURE_TYPE_VIDEO_PROFILE_KHR`
//! - `VK_STRUCTURE_TYPE_VIDEO_QUEUE_FAMILY_PROPERTIES_2_KHR`
//! - `VK_STRUCTURE_TYPE_VIDEO_REFERENCE_SLOT_KHR`
//! - `VK_STRUCTURE_TYPE_VIDEO_SESSION_CREATE_INFO_KHR`
//! - `VK_STRUCTURE_TYPE_VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR`
//! - `VK_STRUCTURE_TYPE_VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR`
//!# Version History
//! - Revision 0.1, 2019-11-21 (Tony Zlatinski)
//! - Initial draft
//!
//! - Revision 0.2, 2019-11-27 (Tony Zlatinski)
//! - Make vulkan video core common between decode and encode
//!
//! - Revision 1, March 29 2021 (Tony Zlatinski)
//! - Spec and API updates.
//!
//! - Revision 2, August 1 2021 (Srinath Kumarapuram)
//! - Rename `VkVideoCapabilitiesFlagBitsKHR` to
//![`VideoCapabilityFlagBitsKHR`] (along with the names of enumerants it
//!defines) and `VkVideoCapabilitiesFlagsKHR` to
//![`VideoCapabilityFlagsKHR`], following Vulkan naming conventions.
//!# Other info
//! * 2021-03-29
//! * No known IP claims.
//!*
//! - Ahmed Abdelkhalek, AMD
//! - George Hao, AMD
//! - Jake Beju, AMD
//! - Piers Daniell, NVIDIA
//! - Srinath Kumarapuram, NVIDIA
//! - Tobias Hector, AMD
//! - Tony Zlatinski, NVIDIA
//!# Related
//! - [`PhysicalDeviceVideoFormatInfoKHR`]
//! - [`QueryResultStatusKHR`]
//! - [`QueueFamilyQueryResultStatusProperties2KHR`]
//! - [`VideoBeginCodingFlagsKHR`]
//! - [`VideoBeginCodingInfoKHR`]
//! - [`VideoBindMemoryKHR`]
//! - [`VideoCapabilitiesKHR`]
//! - [`VideoCapabilityFlagBitsKHR`]
//! - [`VideoCapabilityFlagsKHR`]
//! - [`VideoChromaSubsamplingFlagBitsKHR`]
//! - [`VideoChromaSubsamplingFlagsKHR`]
//! - [`VideoCodecOperationFlagBitsKHR`]
//! - [`VideoCodecOperationFlagsKHR`]
//! - [`VideoCodingControlFlagBitsKHR`]
//! - [`VideoCodingControlFlagsKHR`]
//! - [`VideoCodingControlInfoKHR`]
//! - [`VideoCodingQualityPresetFlagBitsKHR`]
//! - [`VideoCodingQualityPresetFlagsKHR`]
//! - [`VideoComponentBitDepthFlagBitsKHR`]
//! - [`VideoComponentBitDepthFlagsKHR`]
//! - [`VideoEndCodingFlagsKHR`]
//! - [`VideoEndCodingInfoKHR`]
//! - [`VideoFormatPropertiesKHR`]
//! - [`VideoGetMemoryPropertiesKHR`]
//! - [`VideoPictureResourceKHR`]
//! - [`VideoProfileKHR`]
//! - [`VideoProfilesKHR`]
//! - [`VideoQueueFamilyProperties2KHR`]
//! - [`VideoReferenceSlotKHR`]
//! - [`VideoSessionCreateFlagBitsKHR`]
//! - [`VideoSessionCreateFlagsKHR`]
//! - [`VideoSessionCreateInfoKHR`]
//! - [`VideoSessionKHR`]
//! - [`VideoSessionParametersCreateInfoKHR`]
//! - [`VideoSessionParametersKHR`]
//! - [`VideoSessionParametersUpdateInfoKHR`]
//! - [`BindVideoSessionMemoryKHR`]
//! - [`CmdBeginVideoCodingKHR`]
//! - [`CmdControlVideoCodingKHR`]
//! - [`CmdEndVideoCodingKHR`]
//! - [`CreateVideoSessionKHR`]
//! - [`CreateVideoSessionParametersKHR`]
//! - [`DestroyVideoSessionKHR`]
//! - [`DestroyVideoSessionParametersKHR`]
//! - [`GetPhysicalDeviceVideoCapabilitiesKHR`]
//! - [`GetPhysicalDeviceVideoFormatPropertiesKHR`]
//! - [`GetVideoSessionMemoryRequirementsKHR`]
//! - [`UpdateVideoSessionParametersKHR`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_VIDEO_QUEUE_SPEC_VERSION")]
pub const KHR_VIDEO_QUEUE_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_VIDEO_QUEUE_EXTENSION_NAME")]
pub const KHR_VIDEO_QUEUE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_video_queue");
///[VkQueryResultStatusKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryResultStatusKHR.html) - Specific status codes for operations
///# C Specifications
///Specific status codes that **can** be returned from a query are:
///```c
///// Provided by VK_KHR_video_queue
///typedef enum VkQueryResultStatusKHR {
///    VK_QUERY_RESULT_STATUS_ERROR_KHR = -1,
///    VK_QUERY_RESULT_STATUS_NOT_READY_KHR = 0,
///    VK_QUERY_RESULT_STATUS_COMPLETE_KHR = 1,
///} VkQueryResultStatusKHR;
///```
///# Description
/// - [`QUERY_RESULT_STATUS_NOT_READY`] specifies that the query
///result is not yet available.
/// - [`QUERY_RESULT_STATUS_ERROR`] specifies that operations did not
///complete successfully.
/// - [`QUERY_RESULT_STATUS_COMPLETE`] specifies that operations
///completed successfully and the query result is available.
///# Related
/// - [`VK_KHR_video_queue`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkQueryResultStatusKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct QueryResultStatusKHR(i32);
impl const Default for QueryResultStatusKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for QueryResultStatusKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("QueryResultStatusKHR")
            .field(match *self {
                Self::QUERY_RESULT_STATUS_ERROR => &"QUERY_RESULT_STATUS_ERROR",
                Self::QUERY_RESULT_STATUS_NOT_READY => &"QUERY_RESULT_STATUS_NOT_READY",
                Self::QUERY_RESULT_STATUS_COMPLETE => &"QUERY_RESULT_STATUS_COMPLETE",
                other => unreachable!("invalid value for `QueryResultStatusKHR`: {:?}", other),
            })
            .finish()
    }
}
impl QueryResultStatusKHR {
    ///[`QUERY_RESULT_STATUS_ERROR`] specifies that operations did not
    ///complete successfully.
    pub const QUERY_RESULT_STATUS_ERROR: Self = Self(-1);
    ///[`QUERY_RESULT_STATUS_NOT_READY`] specifies that the query
    ///result is not yet available.
    pub const QUERY_RESULT_STATUS_NOT_READY: Self = Self(0);
    ///[`QUERY_RESULT_STATUS_COMPLETE`] specifies that operations
    ///completed successfully and the query result is available.
    pub const QUERY_RESULT_STATUS_COMPLETE: Self = Self(1);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
}
