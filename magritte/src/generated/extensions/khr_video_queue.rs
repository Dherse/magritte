//![VK_KHR_video_queue](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_video_queue.html) - device extension
//!# Revision
//!3
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_get_physical_device_properties2`]`
//! - Requires `[`khr_sampler_ycbcr_conversion`]`
//! - **This is a *provisional* extension and  **must**  be used with caution. See the [description](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#boilerplate-provisional-header)
//!   of provisional header files for enablement and stability details.**
//!# Contacts
//! - Tony Zlatinski [tzlatinski](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_video_queue]
//!   @tzlatinski%0A<<Here describe the issue or question you have about the VK_KHR_video_queue
//!   extension>>)
//!# New object types
//! - [`VideoSessionKHR`]
//! - [`VideoSessionParametersKHR`]
//!# New commands
//! - [`bind_video_session_memory_khr`]
//! - [`cmd_begin_video_coding_khr`]
//! - [`cmd_control_video_coding_khr`]
//! - [`cmd_end_video_coding_khr`]
//! - [`create_video_session_khr`]
//! - [`create_video_session_parameters_khr`]
//! - [`destroy_video_session_khr`]
//! - [`destroy_video_session_parameters_khr`]
//! - [`get_physical_device_video_capabilities_khr`]
//! - [`get_physical_device_video_format_properties_khr`]
//! - [`get_video_session_memory_requirements_khr`]
//! - [`update_video_session_parameters_khr`]
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
//!   [`BufferCreateInfo`]:  - [`VideoProfilesKHR`]
//! - Extending [`QueryPoolCreateInfo`], [`FormatProperties2`], [`ImageCreateInfo`],
//!   [`ImageViewCreateInfo`], [`BufferCreateInfo`]:  - [`VideoProfileKHR`]
//! - Extending [`QueueFamilyProperties2`]:  - [`QueueFamilyQueryResultStatusProperties2KHR`]  -
//!   [`VideoQueueFamilyProperties2KHR`]
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
//! - Extending [`ObjectType`]:  - `VK_OBJECT_TYPE_VIDEO_SESSION_KHR`  -
//!   `VK_OBJECT_TYPE_VIDEO_SESSION_PARAMETERS_KHR`
//! - Extending [`QueryResultFlagBits`]:  - `VK_QUERY_RESULT_WITH_STATUS_BIT_KHR`
//! - Extending [`QueryType`]:  - `VK_QUERY_TYPE_RESULT_STATUS_ONLY_KHR`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_2_KHR`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_BEGIN_CODING_INFO_KHR`  - `VK_STRUCTURE_TYPE_VIDEO_BIND_MEMORY_KHR`
//!   - `VK_STRUCTURE_TYPE_VIDEO_CAPABILITIES_KHR`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_CODING_CONTROL_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_END_CODING_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_FORMAT_PROPERTIES_KHR`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_GET_MEMORY_PROPERTIES_KHR`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_PICTURE_RESOURCE_KHR`  - `VK_STRUCTURE_TYPE_VIDEO_PROFILES_KHR`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_PROFILE_KHR`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_QUEUE_FAMILY_PROPERTIES_2_KHR`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_REFERENCE_SLOT_KHR`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_SESSION_CREATE_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR`
//!# Version history
//! - Revision 0.1, 2019-11-21 (Tony Zlatinski)  - Initial draft
//! - Revision 0.2, 2019-11-27 (Tony Zlatinski)  - Make vulkan video core common between decode and
//!   encode
//! - Revision 1, March 29 2021 (Tony Zlatinski)  - Spec and API updates.
//! - Revision 2, August 1 2021 (Srinath Kumarapuram)  - Rename `VkVideoCapabilitiesFlagBitsKHR` to
//!   [`VideoCapabilityFlagBitsKHR`] (along with the names of enumerants it defines) and
//!   `VkVideoCapabilitiesFlagsKHR` to [`VideoCapabilityFlagsKHR`], following Vulkan naming
//!   conventions.
//! - Revision 3, 2022-03-16 (Ahmed Abdelkhalek)  - Relocate Std header version reporting/requesting
//!   from codec-operation specific extensions to this extension.  - Make Std header versions
//!   codec-operation specific instead of only codec-specific.
//!# Other information
//! * 2022-03-16
//! * No known IP claims.
//! * - Ahmed Abdelkhalek, AMD  - George Hao, AMD  - Jake Beju, AMD  - Piers Daniell, NVIDIA  -
//!   Srinath Kumarapuram, NVIDIA  - Tobias Hector, AMD  - Tony Zlatinski, NVIDIA
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
//! - [`bind_video_session_memory_khr`]
//! - [`cmd_begin_video_coding_khr`]
//! - [`cmd_control_video_coding_khr`]
//! - [`cmd_end_video_coding_khr`]
//! - [`create_video_session_khr`]
//! - [`create_video_session_parameters_khr`]
//! - [`destroy_video_session_khr`]
//! - [`destroy_video_session_parameters_khr`]
//! - [`get_physical_device_video_capabilities_khr`]
//! - [`get_physical_device_video_format_properties_khr`]
//! - [`get_video_session_memory_requirements_khr`]
//! - [`update_video_session_parameters_khr`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
#[cfg(feature = "VK_EXT_video_decode_h264")]
use crate::extensions::ext_video_decode_h264::VideoDecodeH264DpbSlotInfoEXT;
#[cfg(feature = "VK_EXT_video_decode_h264")]
use crate::extensions::ext_video_decode_h264::VideoDecodeH264ProfileEXT;
#[cfg(feature = "VK_EXT_video_decode_h264")]
use crate::extensions::ext_video_decode_h264::VideoDecodeH264SessionParametersAddInfoEXT;
#[cfg(feature = "VK_EXT_video_decode_h264")]
use crate::extensions::ext_video_decode_h264::VideoDecodeH264SessionParametersCreateInfoEXT;
#[cfg(feature = "VK_EXT_video_decode_h265")]
use crate::extensions::ext_video_decode_h265::VideoDecodeH265DpbSlotInfoEXT;
#[cfg(feature = "VK_EXT_video_decode_h265")]
use crate::extensions::ext_video_decode_h265::VideoDecodeH265ProfileEXT;
#[cfg(feature = "VK_EXT_video_decode_h265")]
use crate::extensions::ext_video_decode_h265::VideoDecodeH265SessionParametersAddInfoEXT;
#[cfg(feature = "VK_EXT_video_decode_h265")]
use crate::extensions::ext_video_decode_h265::VideoDecodeH265SessionParametersCreateInfoEXT;
#[cfg(feature = "VK_EXT_video_encode_h264")]
use crate::extensions::ext_video_encode_h264::VideoEncodeH264ProfileEXT;
#[cfg(feature = "VK_EXT_video_encode_h264")]
use crate::extensions::ext_video_encode_h264::VideoEncodeH264SessionParametersAddInfoEXT;
#[cfg(feature = "VK_EXT_video_encode_h264")]
use crate::extensions::ext_video_encode_h264::VideoEncodeH264SessionParametersCreateInfoEXT;
#[cfg(feature = "VK_EXT_video_encode_h265")]
use crate::extensions::ext_video_encode_h265::VideoEncodeH265ProfileEXT;
#[cfg(feature = "VK_EXT_video_encode_h265")]
use crate::extensions::ext_video_encode_h265::VideoEncodeH265SessionParametersAddInfoEXT;
#[cfg(feature = "VK_EXT_video_encode_h265")]
use crate::extensions::ext_video_encode_h265::VideoEncodeH265SessionParametersCreateInfoEXT;
#[cfg(feature = "VK_KHR_video_decode_queue")]
use crate::extensions::khr_video_decode_queue::VideoDecodeCapabilitiesKHR;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::extensions::khr_video_encode_queue::VideoEncodeCapabilitiesKHR;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::extensions::khr_video_encode_queue::VideoEncodeRateControlInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::extensions::khr_video_encode_queue::VideoEncodeRateControlLayerInfoKHR;
use crate::{
    entry::Entry,
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, BaseOutStructure, Bool32, CommandBuffer, Device, DeviceMemory,
        DeviceSize, ExtensionProperties, Extent2D, Format, ImageUsageFlags, ImageView, Instance, Offset2D,
        PhysicalDevice, StructureType, VulkanResultCodes,
    },
    vulkan1_1::MemoryRequirements2,
    AsRaw, Handle, SmallVec, Unique, VulkanResult,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    ffi::CStr,
    iter::{Extend, FromIterator, IntoIterator},
    marker::PhantomData,
    mem::MaybeUninit,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_VIDEO_QUEUE_SPEC_VERSION")]
pub const KHR_VIDEO_QUEUE_SPEC_VERSION: u32 = 3;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_VIDEO_QUEUE_EXTENSION_NAME")]
pub const KHR_VIDEO_QUEUE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_video_queue");
///[vkGetPhysicalDeviceVideoCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceVideoCapabilitiesKHR.html) - Query video decode or encode capabilities
///# C Specifications
///To query video decode or encode capabilities for a specific codec, call:
///```c
///// Provided by VK_KHR_video_queue
///VkResult vkGetPhysicalDeviceVideoCapabilitiesKHR(
///    VkPhysicalDevice                            physicalDevice,
///    const VkVideoProfileKHR*                    pVideoProfile,
///    VkVideoCapabilitiesKHR*                     pCapabilities);
///```
///# Parameters
/// - [`physical_device`] is the physical device whose video decode or encode capabilities will be
///   queried.
/// - [`p_video_profile`] is a pointer to a [`VideoProfileKHR`] structure with a chained
///   codec-operation specific video profile structure.
/// - [`p_capabilities`] is a pointer to a [`VideoCapabilitiesKHR`] structure in which the
///   capabilities are returned.
///# Description
///## Valid Usage (Implicit)
/// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
/// - [`p_video_profile`] **must**  be a valid pointer to a valid [`VideoProfileKHR`] structure
/// - [`p_capabilities`] **must**  be a valid pointer to a [`VideoCapabilitiesKHR`] structure
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_EXTENSION_NOT_PRESENT`  - `VK_ERROR_INITIALIZATION_FAILED`  -
///   `VK_ERROR_FEATURE_NOT_PRESENT`  - `VK_ERROR_FORMAT_NOT_SUPPORTED`
///# Related
/// - [`khr_video_queue`]
/// - [`PhysicalDevice`]
/// - [`VideoCapabilitiesKHR`]
/// - [`VideoProfileKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetPhysicalDeviceVideoCapabilitiesKHR")]
pub type FNGetPhysicalDeviceVideoCapabilitiesKhr = Option<
    for<'lt> unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_video_profile: *const VideoProfileKHR<'lt>,
        p_capabilities: *mut VideoCapabilitiesKHR<'lt>,
    ) -> VulkanResultCodes,
>;
///[vkGetPhysicalDeviceVideoFormatPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceVideoFormatPropertiesKHR.html) - Query supported Video Decode and Encode image formats
///# C Specifications
///To enumerate the supported output, input and DPB image formats for a
///specific codec operation and video profile, call:
///```c
///// Provided by VK_KHR_video_queue
///VkResult vkGetPhysicalDeviceVideoFormatPropertiesKHR(
///    VkPhysicalDevice                            physicalDevice,
///    const VkPhysicalDeviceVideoFormatInfoKHR*   pVideoFormatInfo,
///    uint32_t*                                   pVideoFormatPropertyCount,
///    VkVideoFormatPropertiesKHR*                 pVideoFormatProperties);
///```
///# Parameters
/// - [`physical_device`] is the physical device being queried.
/// - [`p_video_format_info`] is a pointer to a [`PhysicalDeviceVideoFormatInfoKHR`] structure
///   specifying the codec and video profile for which information is returned.
/// - [`p_video_format_property_count`] is a pointer to an integer related to the number of video
///   format properties available or queried, as described below.
/// - [`p_video_format_properties`] is a pointer to an array of [`VideoFormatPropertiesKHR`]
///   structures in which supported formats are returned.
///# Description
///If [`p_video_format_properties`] is `NULL`, then the number of video format
///properties supported for the given [`physical_device`] is returned in
///[`p_video_format_property_count`].
///Otherwise, [`p_video_format_property_count`] **must**  point to a variable set by
///the user to the number of elements in the [`p_video_format_properties`]
///array, and on return the variable is overwritten with the number of values
///actually written to [`p_video_format_properties`].
///If the value of [`p_video_format_property_count`] is less than the number of
///video format properties supported, at most [`p_video_format_property_count`]
///values will be written to [`p_video_format_properties`], and
///`VK_INCOMPLETE` will be returned instead of `VK_SUCCESS`, to
///indicate that not all the available values were returned.If an implementation reports
///`VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_COINCIDE_BIT_KHR` is
///supported but
///`VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_DISTINCT_BIT_KHR` is not
///supported in [`VideoDecodeCapabilitiesKHR::flags`], then to query
///for video format properties for decode DPB or output, `imageUsage` **must**
///have both `VK_IMAGE_USAGE_VIDEO_DECODE_DPB_BIT_KHR` and
///`VK_IMAGE_USAGE_VIDEO_DECODE_DST_BIT_KHR` set.
///Otherwise, the call will fail with `VK_ERROR_FORMAT_NOT_SUPPORTED`.If an implementation reports
///`VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_DISTINCT_BIT_KHR` is
///supported but
///`VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_COINCIDE_BIT_KHR` is not
///supported in [`VideoDecodeCapabilitiesKHR::flags`], then to query
///for video format properties for decode DPB, `imageUsage` **must**  have
///`VK_IMAGE_USAGE_VIDEO_DECODE_DPB_BIT_KHR` set and
///`VK_IMAGE_USAGE_VIDEO_DECODE_DST_BIT_KHR` not set.
///Otherwise, the call will fail with `VK_ERROR_FORMAT_NOT_SUPPORTED`.
///Similarly, to query for video format properties for decode output,
///`imageUsage` **must**  have `VK_IMAGE_USAGE_VIDEO_DECODE_DST_BIT_KHR`
///set and `VK_IMAGE_USAGE_VIDEO_DECODE_DPB_BIT_KHR` not set.
///Otherwise, the call will fail with `VK_ERROR_FORMAT_NOT_SUPPORTED`.
///## Valid Usage
/// - The `imageUsage` enum of [`PhysicalDeviceVideoFormatInfoKHR`] **must**  contain at least one
///   of the following video image usage bit(s): `VK_IMAGE_USAGE_VIDEO_DECODE_DST_BIT_KHR`,
///   `VK_IMAGE_USAGE_VIDEO_DECODE_DPB_BIT_KHR`, `VK_IMAGE_USAGE_VIDEO_ENCODE_SRC_BIT_KHR`, or
///   `VK_IMAGE_USAGE_VIDEO_ENCODE_DPB_BIT_KHR`
///
///## Valid Usage (Implicit)
/// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
/// - [`p_video_format_info`] **must**  be a valid pointer to a valid
///   [`PhysicalDeviceVideoFormatInfoKHR`] structure
/// - [`p_video_format_property_count`] **must**  be a valid pointer to a `uint32_t` value
/// - If the value referenced by [`p_video_format_property_count`] is not `0`, and
///   [`p_video_format_properties`] is not `NULL`, [`p_video_format_properties`] **must**  be a
///   valid pointer to an array of [`p_video_format_property_count`][`VideoFormatPropertiesKHR`]
///   structures
///
///## Return Codes
/// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
/// * - `VK_ERROR_EXTENSION_NOT_PRESENT`  - `VK_ERROR_INITIALIZATION_FAILED`  -
///   `VK_ERROR_FORMAT_NOT_SUPPORTED`
///# Related
/// - [`khr_video_queue`]
/// - [`PhysicalDevice`]
/// - [`PhysicalDeviceVideoFormatInfoKHR`]
/// - [`VideoFormatPropertiesKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetPhysicalDeviceVideoFormatPropertiesKHR")]
pub type FNGetPhysicalDeviceVideoFormatPropertiesKhr = Option<
    for<'lt> unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_video_format_info: *const PhysicalDeviceVideoFormatInfoKHR<'lt>,
        p_video_format_property_count: *mut u32,
        p_video_format_properties: *mut VideoFormatPropertiesKHR<'lt>,
    ) -> VulkanResultCodes,
>;
///[vkCreateVideoSessionKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateVideoSessionKHR.html) - Creates a video session object
///# C Specifications
///To create a video session object, call:
///```c
///// Provided by VK_KHR_video_queue
///VkResult vkCreateVideoSessionKHR(
///    VkDevice                                    device,
///    const VkVideoSessionCreateInfoKHR*          pCreateInfo,
///    const VkAllocationCallbacks*                pAllocator,
///    VkVideoSessionKHR*                          pVideoSession);
///```
///# Parameters
/// - [`device`] is the logical device that creates the decode or encode session object.
/// - [`p_create_info`] is a pointer to a [`VideoSessionCreateInfoKHR`] structure containing
///   parameters specifying the creation of the decode or encode session.
/// - [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)
///   chapter.
/// - [`p_video_session`] is a pointer to a [`VideoSessionKHR`] structure specifying the decode or
///   encode video session object which will be created by this function when it returns
///   `VK_SUCCESS`
///# Description
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_create_info`] **must**  be a valid pointer to a valid [`VideoSessionCreateInfoKHR`]
///   structure
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
/// - [`p_video_session`] **must**  be a valid pointer to a [`VideoSessionKHR`] handle
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
///   `VK_ERROR_INITIALIZATION_FAILED`  - `VK_ERROR_INCOMPATIBLE_DRIVER`  -
///   `VK_ERROR_FEATURE_NOT_PRESENT`
///# Related
/// - [`khr_video_queue`]
/// - [`AllocationCallbacks`]
/// - [`Device`]
/// - [`VideoSessionCreateInfoKHR`]
/// - [`VideoSessionKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCreateVideoSessionKHR")]
pub type FNCreateVideoSessionKhr = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_create_info: *const VideoSessionCreateInfoKHR<'lt>,
        p_allocator: *const AllocationCallbacks<'lt>,
        p_video_session: *mut VideoSessionKHR,
    ) -> VulkanResultCodes,
>;
///[vkDestroyVideoSessionKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyVideoSessionKHR.html) - Destroy decode session object
///# C Specifications
///To destroy a decode session object, call:
///```c
///// Provided by VK_KHR_video_queue
///void vkDestroyVideoSessionKHR(
///    VkDevice                                    device,
///    VkVideoSessionKHR                           videoSession,
///    const VkAllocationCallbacks*                pAllocator);
///```
///# Parameters
/// - [`device`] is the device that was used for the creation of the video session.
/// - [`video_session`] is the decode or encode video session to be destroyed.
/// - [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)
///   chapter.
///# Description
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`video_session`] **must**  be a valid [`VideoSessionKHR`] handle
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
/// - [`video_session`] **must**  have been created, allocated, or retrieved from [`device`]
///# Related
/// - [`khr_video_queue`]
/// - [`AllocationCallbacks`]
/// - [`Device`]
/// - [`VideoSessionKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkDestroyVideoSessionKHR")]
pub type FNDestroyVideoSessionKhr = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        video_session: VideoSessionKHR,
        p_allocator: *const AllocationCallbacks<'lt>,
    ),
>;
///[vkCreateVideoSessionParametersKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateVideoSessionParametersKHR.html) - Creates video session video session parameter object
///# C Specifications
///To create a video session parameters object, call:
///```c
///// Provided by VK_KHR_video_queue
///VkResult vkCreateVideoSessionParametersKHR(
///    VkDevice                                    device,
///    const VkVideoSessionParametersCreateInfoKHR* pCreateInfo,
///    const VkAllocationCallbacks*                pAllocator,
///    VkVideoSessionParametersKHR*                pVideoSessionParameters);
///```
///# Parameters
/// - [`device`] is the logical device that was used for the creation of the video session object.
/// - [`p_create_info`] is a pointer to [`VideoSessionParametersCreateInfoKHR`] structure specifying
///   the video session parameters.
/// - [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)
///   chapter.
/// - [`p_video_session_parameters`] is a pointer to a [`VideoSessionParametersKHR`] handle in which
///   the video session parameters object is returned.
///# Description
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_create_info`] **must**  be a valid pointer to a valid
///   [`VideoSessionParametersCreateInfoKHR`] structure
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
/// - [`p_video_session_parameters`] **must**  be a valid pointer to a [`VideoSessionParametersKHR`]
///   handle
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_INITIALIZATION_FAILED`  - `VK_ERROR_OUT_OF_HOST_MEMORY`  -
///   `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_TOO_MANY_OBJECTS`
///# Related
/// - [`khr_video_queue`]
/// - [`AllocationCallbacks`]
/// - [`Device`]
/// - [`VideoSessionParametersCreateInfoKHR`]
/// - [`VideoSessionParametersKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCreateVideoSessionParametersKHR")]
pub type FNCreateVideoSessionParametersKhr = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_create_info: *const VideoSessionParametersCreateInfoKHR<'lt>,
        p_allocator: *const AllocationCallbacks<'lt>,
        p_video_session_parameters: *mut VideoSessionParametersKHR,
    ) -> VulkanResultCodes,
>;
///[vkUpdateVideoSessionParametersKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUpdateVideoSessionParametersKHR.html) - Update video session video session parameter object
///# C Specifications
///To update, add, or remove video session parameters state, call:
///```c
///// Provided by VK_KHR_video_queue
///VkResult vkUpdateVideoSessionParametersKHR(
///    VkDevice                                    device,
///    VkVideoSessionParametersKHR                 videoSessionParameters,
///    const VkVideoSessionParametersUpdateInfoKHR* pUpdateInfo);
///```
///# Parameters
/// - [`device`] is the logical device that was used for the creation of the video session object.
/// - [`video_session_parameters`] is the video session object that is going to be updated.
/// - [`p_update_info`] is a pointer to a [`VideoSessionParametersUpdateInfoKHR`] structure
///   containing the session parameters update information.
///# Description
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`video_session_parameters`] **must**  be a valid [`VideoSessionParametersKHR`] handle
/// - [`p_update_info`] **must**  be a valid pointer to a valid
///   [`VideoSessionParametersUpdateInfoKHR`] structure
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_INITIALIZATION_FAILED`  - `VK_ERROR_TOO_MANY_OBJECTS`
///# Related
/// - [`khr_video_queue`]
/// - [`Device`]
/// - [`VideoSessionParametersKHR`]
/// - [`VideoSessionParametersUpdateInfoKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkUpdateVideoSessionParametersKHR")]
pub type FNUpdateVideoSessionParametersKhr = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        video_session_parameters: VideoSessionParametersKHR,
        p_update_info: *const VideoSessionParametersUpdateInfoKHR<'lt>,
    ) -> VulkanResultCodes,
>;
///[vkDestroyVideoSessionParametersKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyVideoSessionParametersKHR.html) - Destroy video session parameters object
///# C Specifications
///To destroy a video session object, call:
///```c
///// Provided by VK_KHR_video_queue
///void vkDestroyVideoSessionParametersKHR(
///    VkDevice                                    device,
///    VkVideoSessionParametersKHR                 videoSessionParameters,
///    const VkAllocationCallbacks*                pAllocator);
///```
///# Parameters
/// - [`device`] is the device the video session was created with.
/// - [`video_session_parameters`] is the video session parameters object to be destroyed.
/// - [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)
///   chapter.
///# Description
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`video_session_parameters`] **must**  be a valid [`VideoSessionParametersKHR`] handle
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
///# Related
/// - [`khr_video_queue`]
/// - [`AllocationCallbacks`]
/// - [`Device`]
/// - [`VideoSessionParametersKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkDestroyVideoSessionParametersKHR")]
pub type FNDestroyVideoSessionParametersKhr = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        video_session_parameters: VideoSessionParametersKHR,
        p_allocator: *const AllocationCallbacks<'lt>,
    ),
>;
///[vkGetVideoSessionMemoryRequirementsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetVideoSessionMemoryRequirementsKHR.html) - Get Memory Requirements
///# C Specifications
///To get memory requirements for a video session, call:
///```c
///// Provided by VK_KHR_video_queue
///VkResult vkGetVideoSessionMemoryRequirementsKHR(
///    VkDevice                                    device,
///    VkVideoSessionKHR                           videoSession,
///    uint32_t*                                   pVideoSessionMemoryRequirementsCount,
///    VkVideoGetMemoryPropertiesKHR*              pVideoSessionMemoryRequirements);
///```
///# Parameters
/// - [`device`] is the logical device that owns the video session.
/// - [`video_session`] is the video session to query.
/// - [`p_video_session_memory_requirements_count`] is a pointer to an integer related to the number
///   of memory heap requirements available or queried, as described below.
/// - [`p_video_session_memory_requirements`] is `NULL` or a pointer to an array of
///   [`VideoGetMemoryPropertiesKHR`] structures in which the memory heap requirements of the video
///   session are returned.
///# Description
///If [`p_video_session_memory_requirements`] is `NULL`, then the number of
///memory heap types required for the video session is returned in
///[`p_video_session_memory_requirements_count`].
///Otherwise, [`p_video_session_memory_requirements_count`] **must**  point to a
///variable set by the user with the number of elements in the
///[`p_video_session_memory_requirements`] array, and on return the variable is
///overwritten with the number of formats actually written to
///[`p_video_session_memory_requirements`].
///If [`p_video_session_memory_requirements_count`] is less than the number of
///memory heap types required for the video session, then at most
///[`p_video_session_memory_requirements_count`] elements will be written to
///[`p_video_session_memory_requirements`], and `VK_INCOMPLETE` will be
///returned, instead of `VK_SUCCESS`, to indicate that not all required
///memory heap types were returned.
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`video_session`] **must**  be a valid [`VideoSessionKHR`] handle
/// - [`p_video_session_memory_requirements_count`] **must**  be a valid pointer to a `uint32_t`
///   value
/// - If the value referenced by [`p_video_session_memory_requirements_count`] is not `0`, and
///   [`p_video_session_memory_requirements`] is not `NULL`, [`p_video_session_memory_requirements`]
///   **must**  be a valid pointer to an array of
///   [`p_video_session_memory_requirements_count`][`VideoGetMemoryPropertiesKHR`] structures
/// - [`video_session`] **must**  have been created, allocated, or retrieved from [`device`]
///
///## Return Codes
/// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
/// * - `VK_ERROR_INITIALIZATION_FAILED`
///# Related
/// - [`khr_video_queue`]
/// - [`Device`]
/// - [`VideoGetMemoryPropertiesKHR`]
/// - [`VideoSessionKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetVideoSessionMemoryRequirementsKHR")]
pub type FNGetVideoSessionMemoryRequirementsKhr = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        video_session: VideoSessionKHR,
        p_video_session_memory_requirements_count: *mut u32,
        p_video_session_memory_requirements: *mut VideoGetMemoryPropertiesKHR<'lt>,
    ) -> VulkanResultCodes,
>;
///[vkBindVideoSessionMemoryKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindVideoSessionMemoryKHR.html) - Bind Video Memory
///# C Specifications
///To attach memory to a video session object, call:
///```c
///// Provided by VK_KHR_video_queue
///VkResult vkBindVideoSessionMemoryKHR(
///    VkDevice                                    device,
///    VkVideoSessionKHR                           videoSession,
///    uint32_t                                    videoSessionBindMemoryCount,
///    const VkVideoBindMemoryKHR*                 pVideoSessionBindMemories);
///```
///# Parameters
/// - [`device`] is the logical device that owns the video session’s memory.
/// - [`video_session`] is the video session to be bound with device memory.
/// - [`video_session_bind_memory_count`] is the number of [`p_video_session_bind_memories`] to be
///   bound.
/// - [`p_video_session_bind_memories`] is a pointer to an array of [`VideoBindMemoryKHR`]
///   structures specifying memory regions to be bound to a device memory heap.
///# Description
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`video_session`] **must**  be a valid [`VideoSessionKHR`] handle
/// - [`p_video_session_bind_memories`] **must**  be a valid pointer to an array of
///   [`video_session_bind_memory_count`] valid [`VideoBindMemoryKHR`] structures
/// - [`video_session_bind_memory_count`] **must**  be greater than `0`
/// - [`video_session`] **must**  have been created, allocated, or retrieved from [`device`]
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
///   `VK_ERROR_INITIALIZATION_FAILED`
///# Related
/// - [`khr_video_queue`]
/// - [`Device`]
/// - [`VideoBindMemoryKHR`]
/// - [`VideoSessionKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkBindVideoSessionMemoryKHR")]
pub type FNBindVideoSessionMemoryKhr = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        video_session: VideoSessionKHR,
        video_session_bind_memory_count: u32,
        p_video_session_bind_memories: *const VideoBindMemoryKHR<'lt>,
    ) -> VulkanResultCodes,
>;
///[vkCmdBeginVideoCodingKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginVideoCodingKHR.html) - Start decode jobs
///# C Specifications
///To start video decode or encode operations, call:
///```c
///// Provided by VK_KHR_video_queue
///void vkCmdBeginVideoCodingKHR(
///    VkCommandBuffer                             commandBuffer,
///    const VkVideoBeginCodingInfoKHR*            pBeginInfo);
///```
///# Parameters
/// - [`command_buffer`] is the command buffer to be used when recording commands for the video
///   decode or encode operations.
/// - [`p_begin_info`] is a pointer to a [`VideoBeginCodingInfoKHR`] structure.
///# Description
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`p_begin_info`] **must**  be a valid pointer to a valid [`VideoBeginCodingInfoKHR`] structure
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support decode, or
///   encode operations
/// - This command  **must**  only be called outside of a render pass instance
/// - [`command_buffer`] **must**  be a primary [`CommandBuffer`]
///
///## Host Synchronization
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
///## Command Properties
///# Related
/// - [`khr_video_queue`]
/// - [`CommandBuffer`]
/// - [`VideoBeginCodingInfoKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdBeginVideoCodingKHR")]
pub type FNCmdBeginVideoCodingKhr = Option<
    for<'lt> unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_begin_info: *const VideoBeginCodingInfoKHR<'lt>,
    ),
>;
///[vkCmdControlVideoCodingKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdControlVideoCodingKHR.html) - Set encode rate control parameters
///# C Specifications
///To apply dynamic controls to video decode or video encode operations, call:
///```c
///// Provided by VK_KHR_video_queue
///void vkCmdControlVideoCodingKHR(
///    VkCommandBuffer                             commandBuffer,
///    const VkVideoCodingControlInfoKHR*          pCodingControlInfo);
///```
///# Parameters
/// - [`command_buffer`] is the command buffer to be filled by this function.
/// - [`p_coding_control_info`] is a pointer to a [`VideoCodingControlInfoKHR`] structure.
///# Description
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`p_coding_control_info`] **must**  be a valid pointer to a valid
///   [`VideoCodingControlInfoKHR`] structure
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support decode, or
///   encode operations
/// - This command  **must**  only be called outside of a render pass instance
/// - [`command_buffer`] **must**  be a primary [`CommandBuffer`]
///
///## Host Synchronization
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
///## Command Properties
///# Related
/// - [`khr_video_queue`]
/// - [`CommandBuffer`]
/// - [`VideoCodingControlInfoKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdControlVideoCodingKHR")]
pub type FNCmdControlVideoCodingKhr = Option<
    for<'lt> unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_coding_control_info: *const VideoCodingControlInfoKHR<'lt>,
    ),
>;
///[vkCmdEndVideoCodingKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndVideoCodingKHR.html) - End decode jobs
///# C Specifications
///To end video decode or encode operations, call:
///```c
///// Provided by VK_KHR_video_queue
///void vkCmdEndVideoCodingKHR(
///    VkCommandBuffer                             commandBuffer,
///    const VkVideoEndCodingInfoKHR*              pEndCodingInfo);
///```
///# Parameters
/// - [`command_buffer`] is the command buffer to be filled by this function.
/// - [`p_end_coding_info`] is a pointer to a [`VideoEndCodingInfoKHR`] structure.
///# Description
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`p_end_coding_info`] **must**  be a valid pointer to a valid [`VideoEndCodingInfoKHR`]
///   structure
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support decode, or
///   encode operations
/// - This command  **must**  only be called outside of a render pass instance
/// - [`command_buffer`] **must**  be a primary [`CommandBuffer`]
///
///## Host Synchronization
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
///## Command Properties
///# Related
/// - [`khr_video_queue`]
/// - [`CommandBuffer`]
/// - [`VideoEndCodingInfoKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdEndVideoCodingKHR")]
pub type FNCmdEndVideoCodingKhr = Option<
    for<'lt> unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_end_coding_info: *const VideoEndCodingInfoKHR<'lt>,
    ),
>;
///[VkQueryResultStatusKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryResultStatusKHR.html) - Specific status codes for operations
///# C Specifications
///Specific status codes that  **can**  be returned from a query are:
///```c
///// Provided by VK_KHR_video_queue
///typedef enum VkQueryResultStatusKHR {
///    VK_QUERY_RESULT_STATUS_ERROR_KHR = -1,
///    VK_QUERY_RESULT_STATUS_NOT_READY_KHR = 0,
///    VK_QUERY_RESULT_STATUS_COMPLETE_KHR = 1,
///} VkQueryResultStatusKHR;
///```
///# Description
/// - [`NOT_READY`] specifies that the query result is not yet available.
/// - [`ERROR`] specifies that operations did not complete successfully.
/// - [`COMPLETE`] specifies that operations completed successfully and the query result is
///   available.
///# Related
/// - [`khr_video_queue`]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct QueryResultStatusKHR(i32);
impl const Default for QueryResultStatusKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl QueryResultStatusKHR {
    ///[`ERROR`] specifies that operations did not
    ///complete successfully.
    pub const ERROR: Self = Self(-1);
    ///[`NOT_READY`] specifies that the query
    ///result is not yet available.
    pub const NOT_READY: Self = Self(0);
    ///[`COMPLETE`] specifies that operations
    ///completed successfully and the query result is available.
    pub const COMPLETE: Self = Self(1);
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
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe.
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
impl std::fmt::Debug for QueryResultStatusKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(QueryResultStatusKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == QueryResultStatusKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    match self.0 {
                        QueryResultStatusKHR::ERROR => f.write_str("ERROR")?,
                        QueryResultStatusKHR::NOT_READY => f.write_str("NOT_READY")?,
                        QueryResultStatusKHR::COMPLETE => f.write_str("COMPLETE")?,
                        _ => f.write_str("invalid")?,
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(QueryResultStatusKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoCodecOperationFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoCodecOperationFlagBitsKHR.html) - Video codec operation types
///# C Specifications
///The codec operations are defined with the
///[`VideoCodecOperationFlagBitsKHR`] enum:
///```c
///// Provided by VK_KHR_video_queue
///typedef enum VkVideoCodecOperationFlagBitsKHR {
///    VK_VIDEO_CODEC_OPERATION_INVALID_BIT_KHR = 0,
///#ifdef VK_ENABLE_BETA_EXTENSIONS
///  // Provided by VK_EXT_video_encode_h264
///    VK_VIDEO_CODEC_OPERATION_ENCODE_H264_BIT_EXT = 0x00010000,
///#endif
///#ifdef VK_ENABLE_BETA_EXTENSIONS
///  // Provided by VK_EXT_video_encode_h265
///    VK_VIDEO_CODEC_OPERATION_ENCODE_H265_BIT_EXT = 0x00020000,
///#endif
///#ifdef VK_ENABLE_BETA_EXTENSIONS
///  // Provided by VK_EXT_video_decode_h264
///    VK_VIDEO_CODEC_OPERATION_DECODE_H264_BIT_EXT = 0x00000001,
///#endif
///#ifdef VK_ENABLE_BETA_EXTENSIONS
///  // Provided by VK_EXT_video_decode_h265
///    VK_VIDEO_CODEC_OPERATION_DECODE_H265_BIT_EXT = 0x00000002,
///#endif
///} VkVideoCodecOperationFlagBitsKHR;
///```
///# Description
///Each decode or encode codec-specific extension extends this enumeration with
///the appropriate bit corresponding to the extension’s codec operation:
/// - [`INVALID`] - No video operations are supported for this queue family.
/// - [`ENCODE_H264_EXT`] - H.264 video encode operations are supported by this queue family.
/// - [`DECODE_H264_EXT`] - H.264 video decode operations are supported by this queue family.
/// - [`DECODE_H265_EXT`] - H.265 video decode operations are supported by this queue family.
///# Related
/// - [`khr_video_queue`]
/// - [`VideoCodecOperationFlagsKHR`]
/// - [`VideoProfileKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoCodecOperationFlagBitsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoCodecOperationFlagBitsKHR(u32);
impl const Default for VideoCodecOperationFlagBitsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl VideoCodecOperationFlagBitsKHR {
    ///[`INVALID`] - No video operations are
    ///supported for this queue family.
    pub const INVALID: Self = Self(0);
    ///[`ENCODE_H264_EXT`] - H.264 video encode
    ///operations are supported by this queue family.
    ///
    ///Provided by [`crate::extensions::ext_video_encode_h264`]
    #[cfg(feature = "VK_EXT_video_encode_h264")]
    pub const ENCODE_H264_EXT: Self = Self(65536);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_video_encode_h265`]
    #[cfg(feature = "VK_EXT_video_encode_h265")]
    pub const ENCODE_H265_EXT: Self = Self(131072);
    ///[`DECODE_H264_EXT`] - H.264 video decode
    ///operations are supported by this queue family.
    ///
    ///Provided by [`crate::extensions::ext_video_decode_h264`]
    #[cfg(feature = "VK_EXT_video_decode_h264")]
    pub const DECODE_H264_EXT: Self = Self(1);
    ///[`DECODE_H265_EXT`] - H.265 video decode
    ///operations are supported by this queue family.
    ///
    ///Provided by [`crate::extensions::ext_video_decode_h265`]
    #[cfg(feature = "VK_EXT_video_decode_h265")]
    pub const DECODE_H265_EXT: Self = Self(2);
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
impl std::fmt::Debug for VideoCodecOperationFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoCodecOperationFlagBitsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoCodecOperationFlagBitsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    match self.0 {
                        VideoCodecOperationFlagBitsKHR::INVALID => f.write_str("INVALID")?,
                        #[cfg(feature = "VK_EXT_video_encode_h264")]
                        VideoCodecOperationFlagBitsKHR::ENCODE_H264_EXT => f.write_str("ENCODE_H264_EXT")?,
                        #[cfg(feature = "VK_EXT_video_encode_h265")]
                        VideoCodecOperationFlagBitsKHR::ENCODE_H265_EXT => f.write_str("ENCODE_H265_EXT")?,
                        #[cfg(feature = "VK_EXT_video_decode_h264")]
                        VideoCodecOperationFlagBitsKHR::DECODE_H264_EXT => f.write_str("DECODE_H264_EXT")?,
                        #[cfg(feature = "VK_EXT_video_decode_h265")]
                        VideoCodecOperationFlagBitsKHR::DECODE_H265_EXT => f.write_str("DECODE_H265_EXT")?,
                        _ => f.write_str("invalid")?,
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoCodecOperationFlagBitsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoChromaSubsamplingFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoChromaSubsamplingFlagBitsKHR.html) - Video chroma subsampling
///# C Specifications
///The video format chroma subsampling is defined with the following enums:
///```c
///// Provided by VK_KHR_video_queue
///typedef enum VkVideoChromaSubsamplingFlagBitsKHR {
///    VK_VIDEO_CHROMA_SUBSAMPLING_INVALID_BIT_KHR = 0,
///    VK_VIDEO_CHROMA_SUBSAMPLING_MONOCHROME_BIT_KHR = 0x00000001,
///    VK_VIDEO_CHROMA_SUBSAMPLING_420_BIT_KHR = 0x00000002,
///    VK_VIDEO_CHROMA_SUBSAMPLING_422_BIT_KHR = 0x00000004,
///    VK_VIDEO_CHROMA_SUBSAMPLING_444_BIT_KHR = 0x00000008,
///} VkVideoChromaSubsamplingFlagBitsKHR;
///```
///# Description
/// - [`MONOCHROME`] - the format is monochrome.
/// - [`420`] - the format is 4:2:0 chroma subsampled. The two chroma components are each subsampled
///   at a factor of 2 both horizontally and vertically.
/// - [`422`] - the format is 4:2:2 chroma subsampled. The two chroma components are sampled at half
///   the sample rate of luma. The horizontal chroma resolution is halved.
/// - [`444`] - the format is 4:4:4 chroma sampled. Each of the three YCbCr components have the same
///   sample rate, thus there is no chroma subsampling.
///# Related
/// - [`khr_video_queue`]
/// - [`VideoChromaSubsamplingFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoChromaSubsamplingFlagBitsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoChromaSubsamplingFlagBitsKHR(u32);
impl const Default for VideoChromaSubsamplingFlagBitsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl VideoChromaSubsamplingFlagBitsKHR {
    ///No documentation found
    pub const INVALID: Self = Self(0);
    ///[`MONOCHROME`] - the format is
    ///monochrome.
    pub const MONOCHROME: Self = Self(1);
    ///[`420`] - the format is 4:2:0
    ///chroma subsampled.
    ///The two chroma components are each subsampled at a factor of 2 both
    ///horizontally and vertically.
    pub const _420: Self = Self(2);
    ///[`422`] - the format is 4:2:2
    ///chroma subsampled.
    ///The two chroma components are sampled at half the sample rate of luma.
    ///The horizontal chroma resolution is halved.
    pub const _422: Self = Self(4);
    ///[`444`] - the format is 4:4:4
    ///chroma sampled.
    ///Each of the three YCbCr components have the same sample rate, thus there
    ///is no chroma subsampling.
    pub const _444: Self = Self(8);
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
impl std::fmt::Debug for VideoChromaSubsamplingFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoChromaSubsamplingFlagBitsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoChromaSubsamplingFlagBitsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    match self.0 {
                        VideoChromaSubsamplingFlagBitsKHR::INVALID => f.write_str("INVALID")?,
                        VideoChromaSubsamplingFlagBitsKHR::MONOCHROME => f.write_str("MONOCHROME")?,
                        VideoChromaSubsamplingFlagBitsKHR::_420 => f.write_str("420")?,
                        VideoChromaSubsamplingFlagBitsKHR::_422 => f.write_str("422")?,
                        VideoChromaSubsamplingFlagBitsKHR::_444 => f.write_str("444")?,
                        _ => f.write_str("invalid")?,
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoChromaSubsamplingFlagBitsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoComponentBitDepthFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoComponentBitDepthFlagBitsKHR.html) - Video component bit depth
///# C Specifications
///The video format component bit depth is defined with the following enums:
///```c
///// Provided by VK_KHR_video_queue
///typedef enum VkVideoComponentBitDepthFlagBitsKHR {
///    VK_VIDEO_COMPONENT_BIT_DEPTH_INVALID_KHR = 0,
///    VK_VIDEO_COMPONENT_BIT_DEPTH_8_BIT_KHR = 0x00000001,
///    VK_VIDEO_COMPONENT_BIT_DEPTH_10_BIT_KHR = 0x00000004,
///    VK_VIDEO_COMPONENT_BIT_DEPTH_12_BIT_KHR = 0x00000010,
///} VkVideoComponentBitDepthFlagBitsKHR;
///```
///# Description
/// - [`VIDEO_COMPONENT_DEPTH8`] - the format component bit depth is 8 bits.
/// - [`VIDEO_COMPONENT_DEPTH10`] - the format component bit depth is 10 bits.
/// - [`VIDEO_COMPONENT_DEPTH12`] - the format component bit depth is 12 bits.
///# Related
/// - [`khr_video_queue`]
/// - [`VideoComponentBitDepthFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoComponentBitDepthFlagBitsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoComponentBitDepthFlagBitsKHR(u32);
impl const Default for VideoComponentBitDepthFlagBitsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl VideoComponentBitDepthFlagBitsKHR {
    ///No documentation found
    pub const VIDEO_COMPONENT_DEPTH_INVALID: Self = Self(0);
    ///[`VIDEO_COMPONENT_DEPTH8`] - the format component bit
    ///depth is 8 bits.
    pub const VIDEO_COMPONENT_DEPTH8: Self = Self(1);
    ///[`VIDEO_COMPONENT_DEPTH10`] - the format component bit
    ///depth is 10 bits.
    pub const VIDEO_COMPONENT_DEPTH10: Self = Self(4);
    ///[`VIDEO_COMPONENT_DEPTH12`] - the format component bit
    ///depth is 12 bits.
    pub const VIDEO_COMPONENT_DEPTH12: Self = Self(16);
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
impl std::fmt::Debug for VideoComponentBitDepthFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoComponentBitDepthFlagBitsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoComponentBitDepthFlagBitsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    match self.0 {
                        VideoComponentBitDepthFlagBitsKHR::VIDEO_COMPONENT_DEPTH_INVALID => {
                            f.write_str("VIDEO_COMPONENT_DEPTH_INVALID")?
                        },
                        VideoComponentBitDepthFlagBitsKHR::VIDEO_COMPONENT_DEPTH8 => {
                            f.write_str("VIDEO_COMPONENT_DEPTH8")?
                        },
                        VideoComponentBitDepthFlagBitsKHR::VIDEO_COMPONENT_DEPTH10 => {
                            f.write_str("VIDEO_COMPONENT_DEPTH10")?
                        },
                        VideoComponentBitDepthFlagBitsKHR::VIDEO_COMPONENT_DEPTH12 => {
                            f.write_str("VIDEO_COMPONENT_DEPTH12")?
                        },
                        _ => f.write_str("invalid")?,
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoComponentBitDepthFlagBitsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoCapabilityFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoCapabilityFlagBitsKHR.html) - Video Decode and Encode Capability Flags
///# C Specifications
///The [`VideoCapabilitiesKHR`] flags are defined with the following
///enumeration:
///```c
///// Provided by VK_KHR_video_queue
///typedef enum VkVideoCapabilityFlagBitsKHR {
///    VK_VIDEO_CAPABILITY_PROTECTED_CONTENT_BIT_KHR = 0x00000001,
///    VK_VIDEO_CAPABILITY_SEPARATE_REFERENCE_IMAGES_BIT_KHR = 0x00000002,
///} VkVideoCapabilityFlagBitsKHR;
///```
///# Description
/// - [`PROTECTED_CONTENT`] - the decode or encode session supports protected content.
/// - [`SEPARATE_REFERENCE_IMAGES`] - the DPB or Reconstructed Video Picture Resources for the video
///   session  **may**  be created as a separate [`Image`] for each DPB picture. If not supported,
///   the DPB  **must**  be created as single multi-layered image where each layer represents one of
///   the DPB Video Picture Resources.
///# Related
/// - [`khr_video_queue`]
/// - [`VideoCapabilityFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoCapabilityFlagBitsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoCapabilityFlagBitsKHR(u32);
impl const Default for VideoCapabilityFlagBitsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl VideoCapabilityFlagBitsKHR {
    ///[`PROTECTED_CONTENT`] - the decode or
    ///encode session supports protected content.
    pub const PROTECTED_CONTENT: Self = Self(1);
    ///[`SEPARATE_REFERENCE_IMAGES`] - the DPB or
    ///Reconstructed Video Picture Resources for the video session  **may**  be
    ///created as a separate [`Image`] for each DPB picture.
    ///If not supported, the DPB  **must**  be created as single multi-layered image
    ///where each layer represents one of the DPB Video Picture Resources.
    pub const SEPARATE_REFERENCE_IMAGES: Self = Self(2);
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
impl std::fmt::Debug for VideoCapabilityFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoCapabilityFlagBitsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoCapabilityFlagBitsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    match self.0 {
                        VideoCapabilityFlagBitsKHR::PROTECTED_CONTENT => f.write_str("PROTECTED_CONTENT")?,
                        VideoCapabilityFlagBitsKHR::SEPARATE_REFERENCE_IMAGES => {
                            f.write_str("SEPARATE_REFERENCE_IMAGES")?
                        },
                        _ => f.write_str("invalid")?,
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoCapabilityFlagBitsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoSessionCreateFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoSessionCreateFlagBitsKHR.html) - Video decode or encode video session creation flags
///# C Specifications
///The decode or encode session creation flags defined with the following
///enums:
///```c
///// Provided by VK_KHR_video_queue
///typedef enum VkVideoSessionCreateFlagBitsKHR {
///    VK_VIDEO_SESSION_CREATE_DEFAULT_KHR = 0,
///    VK_VIDEO_SESSION_CREATE_PROTECTED_CONTENT_BIT_KHR = 0x00000001,
///} VkVideoSessionCreateFlagBitsKHR;
///```
///# Description
/// - [`PROTECTED_CONTENT`] - create the video session for use with protected video content
///# Related
/// - [`khr_video_queue`]
/// - [`VideoSessionCreateFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoSessionCreateFlagBitsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoSessionCreateFlagBitsKHR(u32);
impl const Default for VideoSessionCreateFlagBitsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl VideoSessionCreateFlagBitsKHR {
    ///No documentation found
    pub const DEFAULT: Self = Self(0);
    ///[`PROTECTED_CONTENT`] - create the
    ///video session for use with protected video content
    pub const PROTECTED_CONTENT: Self = Self(1);
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
impl std::fmt::Debug for VideoSessionCreateFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoSessionCreateFlagBitsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoSessionCreateFlagBitsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    match self.0 {
                        VideoSessionCreateFlagBitsKHR::DEFAULT => f.write_str("DEFAULT")?,
                        VideoSessionCreateFlagBitsKHR::PROTECTED_CONTENT => f.write_str("PROTECTED_CONTENT")?,
                        _ => f.write_str("invalid")?,
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoSessionCreateFlagBitsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoCodingQualityPresetFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoCodingQualityPresetFlagBitsKHR.html) - Video codec profile types
///# C Specifications
///The decode preset types are defined with the following:
///```c
///// Provided by VK_KHR_video_queue
///typedef enum VkVideoCodingQualityPresetFlagBitsKHR {
///    VK_VIDEO_CODING_QUALITY_PRESET_NORMAL_BIT_KHR = 0x00000001,
///    VK_VIDEO_CODING_QUALITY_PRESET_POWER_BIT_KHR = 0x00000002,
///    VK_VIDEO_CODING_QUALITY_PRESET_QUALITY_BIT_KHR = 0x00000004,
///} VkVideoCodingQualityPresetFlagBitsKHR;
///```
///# Description
/// - [`NORMAL`] defines normal decode case.
/// - [`POWER`] defines power efficient case.
/// - [`QUALITY`] defines quality focus case.
///# Related
/// - [`khr_video_queue`]
/// - [`VideoCodingQualityPresetFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoCodingQualityPresetFlagBitsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoCodingQualityPresetFlagBitsKHR(u32);
impl const Default for VideoCodingQualityPresetFlagBitsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl VideoCodingQualityPresetFlagBitsKHR {
    ///[`NORMAL`] defines normal
    ///decode case.
    pub const NORMAL: Self = Self(1);
    ///[`POWER`] defines power
    ///efficient case.
    pub const POWER: Self = Self(2);
    ///[`QUALITY`] defines quality
    ///focus case.
    pub const QUALITY: Self = Self(4);
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
impl std::fmt::Debug for VideoCodingQualityPresetFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoCodingQualityPresetFlagBitsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoCodingQualityPresetFlagBitsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    match self.0 {
                        VideoCodingQualityPresetFlagBitsKHR::NORMAL => f.write_str("NORMAL")?,
                        VideoCodingQualityPresetFlagBitsKHR::POWER => f.write_str("POWER")?,
                        VideoCodingQualityPresetFlagBitsKHR::QUALITY => f.write_str("QUALITY")?,
                        _ => f.write_str("invalid")?,
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoCodingQualityPresetFlagBitsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoCodingControlFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoCodingControlFlagBitsKHR.html) - Video Coding Control Command Flags
///# C Specifications
///The [`cmd_control_video_coding_khr`] flags are defined with the following
///enumeration:
///```c
///// Provided by VK_KHR_video_queue
///typedef enum VkVideoCodingControlFlagBitsKHR {
///    VK_VIDEO_CODING_CONTROL_DEFAULT_KHR = 0,
///    VK_VIDEO_CODING_CONTROL_RESET_BIT_KHR = 0x00000001,
///} VkVideoCodingControlFlagBitsKHR;
///```
///# Description
/// - [`DEFAULT`] indicates a request for the coding control paramaters to be applied to the current
///   state of the bound video session.
/// - [`RESET`] indicates a request for the bound video session device context to be reset before
///   the coding control parameters are applied.
///A newly created video session  **must**  be reset before use for video decode or
///encode operations.
///The reset operation returns all session DPB slots to the unused state (see
///[DPB Slot States](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-session-dpb-slot-states)).
///For encode sessions, the reset operation returns rate control configuration
///to implementation default settings.
///After decode or encode operations are performed on a session, the reset
///operation  **may**  be used to return the video session device context to the
///same initial state as after the reset of a newly created video session.
///This  **may**  be used when different video sequences are processed with the same
///session.
///# Related
/// - [`khr_video_queue`]
/// - [`VideoCodingControlFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoCodingControlFlagBitsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoCodingControlFlagBitsKHR(u32);
impl const Default for VideoCodingControlFlagBitsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl VideoCodingControlFlagBitsKHR {
    ///[`DEFAULT`] indicates a request for the
    ///coding control paramaters to be applied to the current state of the
    ///bound video session.
    pub const DEFAULT: Self = Self(0);
    ///[`RESET`] indicates a request for the
    ///bound video session device context to be reset before the coding control
    ///parameters are applied.
    pub const RESET: Self = Self(1);
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
impl std::fmt::Debug for VideoCodingControlFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoCodingControlFlagBitsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoCodingControlFlagBitsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    match self.0 {
                        VideoCodingControlFlagBitsKHR::DEFAULT => f.write_str("DEFAULT")?,
                        VideoCodingControlFlagBitsKHR::RESET => f.write_str("RESET")?,
                        _ => f.write_str("invalid")?,
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoCodingControlFlagBitsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoCodecOperationFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoCodecOperationFlagBitsKHR.html) - Video codec operation types
///# C Specifications
///The codec operations are defined with the
///[`VideoCodecOperationFlagBitsKHR`] enum:
///```c
///// Provided by VK_KHR_video_queue
///typedef enum VkVideoCodecOperationFlagBitsKHR {
///    VK_VIDEO_CODEC_OPERATION_INVALID_BIT_KHR = 0,
///#ifdef VK_ENABLE_BETA_EXTENSIONS
///  // Provided by VK_EXT_video_encode_h264
///    VK_VIDEO_CODEC_OPERATION_ENCODE_H264_BIT_EXT = 0x00010000,
///#endif
///#ifdef VK_ENABLE_BETA_EXTENSIONS
///  // Provided by VK_EXT_video_encode_h265
///    VK_VIDEO_CODEC_OPERATION_ENCODE_H265_BIT_EXT = 0x00020000,
///#endif
///#ifdef VK_ENABLE_BETA_EXTENSIONS
///  // Provided by VK_EXT_video_decode_h264
///    VK_VIDEO_CODEC_OPERATION_DECODE_H264_BIT_EXT = 0x00000001,
///#endif
///#ifdef VK_ENABLE_BETA_EXTENSIONS
///  // Provided by VK_EXT_video_decode_h265
///    VK_VIDEO_CODEC_OPERATION_DECODE_H265_BIT_EXT = 0x00000002,
///#endif
///} VkVideoCodecOperationFlagBitsKHR;
///```
///# Description
///Each decode or encode codec-specific extension extends this enumeration with
///the appropriate bit corresponding to the extension’s codec operation:
/// - [`INVALID`] - No video operations are supported for this queue family.
/// - [`ENCODE_H264_EXT`] - H.264 video encode operations are supported by this queue family.
/// - [`DECODE_H264_EXT`] - H.264 video decode operations are supported by this queue family.
/// - [`DECODE_H265_EXT`] - H.265 video decode operations are supported by this queue family.
///# Related
/// - [`khr_video_queue`]
/// - [`VideoCodecOperationFlagsKHR`]
/// - [`VideoProfileKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoCodecOperationFlagsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoCodecOperationFlagsKHR(u32);
impl const Default for VideoCodecOperationFlagsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl From<VideoCodecOperationFlagBitsKHR> for VideoCodecOperationFlagsKHR {
    fn from(from: VideoCodecOperationFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(from.bits()) }
    }
}
impl VideoCodecOperationFlagsKHR {
    ///[`INVALID`] - No video operations are
    ///supported for this queue family.
    pub const INVALID: Self = Self(0);
    ///[`ENCODE_H264_EXT`] - H.264 video encode
    ///operations are supported by this queue family.
    ///
    ///Provided by [`crate::extensions::ext_video_encode_h264`]
    #[cfg(feature = "VK_EXT_video_encode_h264")]
    pub const ENCODE_H264_EXT: Self = Self(65536);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_video_encode_h265`]
    #[cfg(feature = "VK_EXT_video_encode_h265")]
    pub const ENCODE_H265_EXT: Self = Self(131072);
    ///[`DECODE_H264_EXT`] - H.264 video decode
    ///operations are supported by this queue family.
    ///
    ///Provided by [`crate::extensions::ext_video_decode_h264`]
    #[cfg(feature = "VK_EXT_video_decode_h264")]
    pub const DECODE_H264_EXT: Self = Self(1);
    ///[`DECODE_H265_EXT`] - H.265 video decode
    ///operations are supported by this queue family.
    ///
    ///Provided by [`crate::extensions::ext_video_decode_h265`]
    #[cfg(feature = "VK_EXT_video_decode_h265")]
    pub const DECODE_H265_EXT: Self = Self(2);
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
            all |= Self::INVALID;
        }
        #[cfg(feature = "VK_EXT_video_encode_h264")]
        {
            all |= Self::ENCODE_H264_EXT;
        }
        #[cfg(feature = "VK_EXT_video_encode_h265")]
        {
            all |= Self::ENCODE_H265_EXT;
        }
        #[cfg(feature = "VK_EXT_video_decode_h264")]
        {
            all |= Self::DECODE_H264_EXT;
        }
        #[cfg(feature = "VK_EXT_video_decode_h265")]
        {
            all |= Self::DECODE_H265_EXT;
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
impl const std::ops::BitOr for VideoCodecOperationFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for VideoCodecOperationFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for VideoCodecOperationFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for VideoCodecOperationFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for VideoCodecOperationFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for VideoCodecOperationFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for VideoCodecOperationFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for VideoCodecOperationFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for VideoCodecOperationFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<VideoCodecOperationFlagsKHR> for VideoCodecOperationFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoCodecOperationFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<VideoCodecOperationFlagBitsKHR> for VideoCodecOperationFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoCodecOperationFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<VideoCodecOperationFlagBitsKHR>>::from(i));
        }
    }
}
impl FromIterator<VideoCodecOperationFlagsKHR> for VideoCodecOperationFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoCodecOperationFlagsKHR>>(iterator: T) -> VideoCodecOperationFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoCodecOperationFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<VideoCodecOperationFlagBitsKHR> for VideoCodecOperationFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoCodecOperationFlagBitsKHR>>(iterator: T) -> VideoCodecOperationFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoCodecOperationFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for VideoCodecOperationFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoCodecOperationFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoCodecOperationFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(VideoCodecOperationFlagsKHR::INVALID) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(INVALID))?;
                    }
                    #[cfg(feature = "VK_EXT_video_encode_h264")]
                    if self.0.contains(VideoCodecOperationFlagsKHR::ENCODE_H264_EXT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ENCODE_H264_EXT))?;
                    }
                    #[cfg(feature = "VK_EXT_video_encode_h265")]
                    if self.0.contains(VideoCodecOperationFlagsKHR::ENCODE_H265_EXT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ENCODE_H265_EXT))?;
                    }
                    #[cfg(feature = "VK_EXT_video_decode_h264")]
                    if self.0.contains(VideoCodecOperationFlagsKHR::DECODE_H264_EXT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DECODE_H264_EXT))?;
                    }
                    #[cfg(feature = "VK_EXT_video_decode_h265")]
                    if self.0.contains(VideoCodecOperationFlagsKHR::DECODE_H265_EXT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DECODE_H265_EXT))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoCodecOperationFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoCapabilityFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoCapabilityFlagBitsKHR.html) - Video Decode and Encode Capability Flags
///# C Specifications
///The [`VideoCapabilitiesKHR`] flags are defined with the following
///enumeration:
///```c
///// Provided by VK_KHR_video_queue
///typedef enum VkVideoCapabilityFlagBitsKHR {
///    VK_VIDEO_CAPABILITY_PROTECTED_CONTENT_BIT_KHR = 0x00000001,
///    VK_VIDEO_CAPABILITY_SEPARATE_REFERENCE_IMAGES_BIT_KHR = 0x00000002,
///} VkVideoCapabilityFlagBitsKHR;
///```
///# Description
/// - [`PROTECTED_CONTENT`] - the decode or encode session supports protected content.
/// - [`SEPARATE_REFERENCE_IMAGES`] - the DPB or Reconstructed Video Picture Resources for the video
///   session  **may**  be created as a separate [`Image`] for each DPB picture. If not supported,
///   the DPB  **must**  be created as single multi-layered image where each layer represents one of
///   the DPB Video Picture Resources.
///# Related
/// - [`khr_video_queue`]
/// - [`VideoCapabilityFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoCapabilityFlagsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoCapabilityFlagsKHR(u32);
impl const Default for VideoCapabilityFlagsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl From<VideoCapabilityFlagBitsKHR> for VideoCapabilityFlagsKHR {
    fn from(from: VideoCapabilityFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(from.bits()) }
    }
}
impl VideoCapabilityFlagsKHR {
    ///[`PROTECTED_CONTENT`] - the decode or
    ///encode session supports protected content.
    pub const PROTECTED_CONTENT: Self = Self(1);
    ///[`SEPARATE_REFERENCE_IMAGES`] - the DPB or
    ///Reconstructed Video Picture Resources for the video session  **may**  be
    ///created as a separate [`Image`] for each DPB picture.
    ///If not supported, the DPB  **must**  be created as single multi-layered image
    ///where each layer represents one of the DPB Video Picture Resources.
    pub const SEPARATE_REFERENCE_IMAGES: Self = Self(2);
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
            all |= Self::PROTECTED_CONTENT;
        }
        {
            all |= Self::SEPARATE_REFERENCE_IMAGES;
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
impl const std::ops::BitOr for VideoCapabilityFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for VideoCapabilityFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for VideoCapabilityFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for VideoCapabilityFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for VideoCapabilityFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for VideoCapabilityFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for VideoCapabilityFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for VideoCapabilityFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for VideoCapabilityFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<VideoCapabilityFlagsKHR> for VideoCapabilityFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoCapabilityFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<VideoCapabilityFlagBitsKHR> for VideoCapabilityFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoCapabilityFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<VideoCapabilityFlagBitsKHR>>::from(i));
        }
    }
}
impl FromIterator<VideoCapabilityFlagsKHR> for VideoCapabilityFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoCapabilityFlagsKHR>>(iterator: T) -> VideoCapabilityFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoCapabilityFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<VideoCapabilityFlagBitsKHR> for VideoCapabilityFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoCapabilityFlagBitsKHR>>(iterator: T) -> VideoCapabilityFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoCapabilityFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for VideoCapabilityFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoCapabilityFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoCapabilityFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(VideoCapabilityFlagsKHR::PROTECTED_CONTENT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PROTECTED_CONTENT))?;
                    }
                    if self.0.contains(VideoCapabilityFlagsKHR::SEPARATE_REFERENCE_IMAGES) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(SEPARATE_REFERENCE_IMAGES))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoCapabilityFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoSessionCreateFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoSessionCreateFlagBitsKHR.html) - Video decode or encode video session creation flags
///# C Specifications
///The decode or encode session creation flags defined with the following
///enums:
///```c
///// Provided by VK_KHR_video_queue
///typedef enum VkVideoSessionCreateFlagBitsKHR {
///    VK_VIDEO_SESSION_CREATE_DEFAULT_KHR = 0,
///    VK_VIDEO_SESSION_CREATE_PROTECTED_CONTENT_BIT_KHR = 0x00000001,
///} VkVideoSessionCreateFlagBitsKHR;
///```
///# Description
/// - [`PROTECTED_CONTENT`] - create the video session for use with protected video content
///# Related
/// - [`khr_video_queue`]
/// - [`VideoSessionCreateFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoSessionCreateFlagsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoSessionCreateFlagsKHR(u32);
impl const Default for VideoSessionCreateFlagsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl From<VideoSessionCreateFlagBitsKHR> for VideoSessionCreateFlagsKHR {
    fn from(from: VideoSessionCreateFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(from.bits()) }
    }
}
impl VideoSessionCreateFlagsKHR {
    ///No documentation found
    pub const DEFAULT: Self = Self(0);
    ///[`PROTECTED_CONTENT`] - create the
    ///video session for use with protected video content
    pub const PROTECTED_CONTENT: Self = Self(1);
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
            all |= Self::DEFAULT;
        }
        {
            all |= Self::PROTECTED_CONTENT;
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
impl const std::ops::BitOr for VideoSessionCreateFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for VideoSessionCreateFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for VideoSessionCreateFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for VideoSessionCreateFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for VideoSessionCreateFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for VideoSessionCreateFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for VideoSessionCreateFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for VideoSessionCreateFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for VideoSessionCreateFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<VideoSessionCreateFlagsKHR> for VideoSessionCreateFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoSessionCreateFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<VideoSessionCreateFlagBitsKHR> for VideoSessionCreateFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoSessionCreateFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<VideoSessionCreateFlagBitsKHR>>::from(i));
        }
    }
}
impl FromIterator<VideoSessionCreateFlagsKHR> for VideoSessionCreateFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoSessionCreateFlagsKHR>>(iterator: T) -> VideoSessionCreateFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoSessionCreateFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<VideoSessionCreateFlagBitsKHR> for VideoSessionCreateFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoSessionCreateFlagBitsKHR>>(iterator: T) -> VideoSessionCreateFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoSessionCreateFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for VideoSessionCreateFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoSessionCreateFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoSessionCreateFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(VideoSessionCreateFlagsKHR::DEFAULT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DEFAULT))?;
                    }
                    if self.0.contains(VideoSessionCreateFlagsKHR::PROTECTED_CONTENT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PROTECTED_CONTENT))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoSessionCreateFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoBeginCodingFlagsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoBeginCodingFlagsKHR.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_KHR_video_queue
///typedef VkFlags VkVideoBeginCodingFlagsKHR;
///```
///# Related
/// - [`khr_video_queue`]
/// - [`VideoBeginCodingInfoKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoBeginCodingFlagsKHR(u32);
impl const Default for VideoBeginCodingFlagsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for VideoBeginCodingFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(VideoBeginCodingFlagsKHR))
            .field(&self.0)
            .finish()
    }
}
///[VkVideoEndCodingFlagsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEndCodingFlagsKHR.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_KHR_video_queue
///typedef VkFlags VkVideoEndCodingFlagsKHR;
///```
///# Related
/// - [`khr_video_queue`]
/// - [`VideoEndCodingInfoKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoEndCodingFlagsKHR(u32);
impl const Default for VideoEndCodingFlagsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for VideoEndCodingFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(VideoEndCodingFlagsKHR))
            .field(&self.0)
            .finish()
    }
}
///[VkVideoCodingQualityPresetFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoCodingQualityPresetFlagBitsKHR.html) - Video codec profile types
///# C Specifications
///The decode preset types are defined with the following:
///```c
///// Provided by VK_KHR_video_queue
///typedef enum VkVideoCodingQualityPresetFlagBitsKHR {
///    VK_VIDEO_CODING_QUALITY_PRESET_NORMAL_BIT_KHR = 0x00000001,
///    VK_VIDEO_CODING_QUALITY_PRESET_POWER_BIT_KHR = 0x00000002,
///    VK_VIDEO_CODING_QUALITY_PRESET_QUALITY_BIT_KHR = 0x00000004,
///} VkVideoCodingQualityPresetFlagBitsKHR;
///```
///# Description
/// - [`NORMAL`] defines normal decode case.
/// - [`POWER`] defines power efficient case.
/// - [`QUALITY`] defines quality focus case.
///# Related
/// - [`khr_video_queue`]
/// - [`VideoCodingQualityPresetFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoCodingQualityPresetFlagsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoCodingQualityPresetFlagsKHR(u32);
impl const Default for VideoCodingQualityPresetFlagsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl From<VideoCodingQualityPresetFlagBitsKHR> for VideoCodingQualityPresetFlagsKHR {
    fn from(from: VideoCodingQualityPresetFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(from.bits()) }
    }
}
impl VideoCodingQualityPresetFlagsKHR {
    ///[`NORMAL`] defines normal
    ///decode case.
    pub const NORMAL: Self = Self(1);
    ///[`POWER`] defines power
    ///efficient case.
    pub const POWER: Self = Self(2);
    ///[`QUALITY`] defines quality
    ///focus case.
    pub const QUALITY: Self = Self(4);
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
            all |= Self::NORMAL;
        }
        {
            all |= Self::POWER;
        }
        {
            all |= Self::QUALITY;
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
impl const std::ops::BitOr for VideoCodingQualityPresetFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for VideoCodingQualityPresetFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for VideoCodingQualityPresetFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for VideoCodingQualityPresetFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for VideoCodingQualityPresetFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for VideoCodingQualityPresetFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for VideoCodingQualityPresetFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for VideoCodingQualityPresetFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for VideoCodingQualityPresetFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<VideoCodingQualityPresetFlagsKHR> for VideoCodingQualityPresetFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoCodingQualityPresetFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<VideoCodingQualityPresetFlagBitsKHR> for VideoCodingQualityPresetFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoCodingQualityPresetFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<VideoCodingQualityPresetFlagBitsKHR>>::from(i));
        }
    }
}
impl FromIterator<VideoCodingQualityPresetFlagsKHR> for VideoCodingQualityPresetFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoCodingQualityPresetFlagsKHR>>(
        iterator: T,
    ) -> VideoCodingQualityPresetFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoCodingQualityPresetFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<VideoCodingQualityPresetFlagBitsKHR> for VideoCodingQualityPresetFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoCodingQualityPresetFlagBitsKHR>>(
        iterator: T,
    ) -> VideoCodingQualityPresetFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoCodingQualityPresetFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for VideoCodingQualityPresetFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoCodingQualityPresetFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoCodingQualityPresetFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(VideoCodingQualityPresetFlagsKHR::NORMAL) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(NORMAL))?;
                    }
                    if self.0.contains(VideoCodingQualityPresetFlagsKHR::POWER) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(POWER))?;
                    }
                    if self.0.contains(VideoCodingQualityPresetFlagsKHR::QUALITY) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(QUALITY))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoCodingQualityPresetFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoCodingControlFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoCodingControlFlagBitsKHR.html) - Video Coding Control Command Flags
///# C Specifications
///The [`cmd_control_video_coding_khr`] flags are defined with the following
///enumeration:
///```c
///// Provided by VK_KHR_video_queue
///typedef enum VkVideoCodingControlFlagBitsKHR {
///    VK_VIDEO_CODING_CONTROL_DEFAULT_KHR = 0,
///    VK_VIDEO_CODING_CONTROL_RESET_BIT_KHR = 0x00000001,
///} VkVideoCodingControlFlagBitsKHR;
///```
///# Description
/// - [`DEFAULT`] indicates a request for the coding control paramaters to be applied to the current
///   state of the bound video session.
/// - [`RESET`] indicates a request for the bound video session device context to be reset before
///   the coding control parameters are applied.
///A newly created video session  **must**  be reset before use for video decode or
///encode operations.
///The reset operation returns all session DPB slots to the unused state (see
///[DPB Slot States](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-session-dpb-slot-states)).
///For encode sessions, the reset operation returns rate control configuration
///to implementation default settings.
///After decode or encode operations are performed on a session, the reset
///operation  **may**  be used to return the video session device context to the
///same initial state as after the reset of a newly created video session.
///This  **may**  be used when different video sequences are processed with the same
///session.
///# Related
/// - [`khr_video_queue`]
/// - [`VideoCodingControlFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoCodingControlFlagsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoCodingControlFlagsKHR(u32);
impl const Default for VideoCodingControlFlagsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl From<VideoCodingControlFlagBitsKHR> for VideoCodingControlFlagsKHR {
    fn from(from: VideoCodingControlFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(from.bits()) }
    }
}
impl VideoCodingControlFlagsKHR {
    ///[`DEFAULT`] indicates a request for the
    ///coding control paramaters to be applied to the current state of the
    ///bound video session.
    pub const DEFAULT: Self = Self(0);
    ///[`RESET`] indicates a request for the
    ///bound video session device context to be reset before the coding control
    ///parameters are applied.
    pub const RESET: Self = Self(1);
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
            all |= Self::DEFAULT;
        }
        {
            all |= Self::RESET;
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
impl const std::ops::BitOr for VideoCodingControlFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for VideoCodingControlFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for VideoCodingControlFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for VideoCodingControlFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for VideoCodingControlFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for VideoCodingControlFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for VideoCodingControlFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for VideoCodingControlFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for VideoCodingControlFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<VideoCodingControlFlagsKHR> for VideoCodingControlFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoCodingControlFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<VideoCodingControlFlagBitsKHR> for VideoCodingControlFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoCodingControlFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<VideoCodingControlFlagBitsKHR>>::from(i));
        }
    }
}
impl FromIterator<VideoCodingControlFlagsKHR> for VideoCodingControlFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoCodingControlFlagsKHR>>(iterator: T) -> VideoCodingControlFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoCodingControlFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<VideoCodingControlFlagBitsKHR> for VideoCodingControlFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoCodingControlFlagBitsKHR>>(iterator: T) -> VideoCodingControlFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoCodingControlFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for VideoCodingControlFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoCodingControlFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoCodingControlFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(VideoCodingControlFlagsKHR::DEFAULT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DEFAULT))?;
                    }
                    if self.0.contains(VideoCodingControlFlagsKHR::RESET) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(RESET))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoCodingControlFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoChromaSubsamplingFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoChromaSubsamplingFlagBitsKHR.html) - Video chroma subsampling
///# C Specifications
///The video format chroma subsampling is defined with the following enums:
///```c
///// Provided by VK_KHR_video_queue
///typedef enum VkVideoChromaSubsamplingFlagBitsKHR {
///    VK_VIDEO_CHROMA_SUBSAMPLING_INVALID_BIT_KHR = 0,
///    VK_VIDEO_CHROMA_SUBSAMPLING_MONOCHROME_BIT_KHR = 0x00000001,
///    VK_VIDEO_CHROMA_SUBSAMPLING_420_BIT_KHR = 0x00000002,
///    VK_VIDEO_CHROMA_SUBSAMPLING_422_BIT_KHR = 0x00000004,
///    VK_VIDEO_CHROMA_SUBSAMPLING_444_BIT_KHR = 0x00000008,
///} VkVideoChromaSubsamplingFlagBitsKHR;
///```
///# Description
/// - [`MONOCHROME`] - the format is monochrome.
/// - [`420`] - the format is 4:2:0 chroma subsampled. The two chroma components are each subsampled
///   at a factor of 2 both horizontally and vertically.
/// - [`422`] - the format is 4:2:2 chroma subsampled. The two chroma components are sampled at half
///   the sample rate of luma. The horizontal chroma resolution is halved.
/// - [`444`] - the format is 4:4:4 chroma sampled. Each of the three YCbCr components have the same
///   sample rate, thus there is no chroma subsampling.
///# Related
/// - [`khr_video_queue`]
/// - [`VideoChromaSubsamplingFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoChromaSubsamplingFlagsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoChromaSubsamplingFlagsKHR(u32);
impl const Default for VideoChromaSubsamplingFlagsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl From<VideoChromaSubsamplingFlagBitsKHR> for VideoChromaSubsamplingFlagsKHR {
    fn from(from: VideoChromaSubsamplingFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(from.bits()) }
    }
}
impl VideoChromaSubsamplingFlagsKHR {
    ///No documentation found
    pub const INVALID: Self = Self(0);
    ///[`MONOCHROME`] - the format is
    ///monochrome.
    pub const MONOCHROME: Self = Self(1);
    ///[`420`] - the format is 4:2:0
    ///chroma subsampled.
    ///The two chroma components are each subsampled at a factor of 2 both
    ///horizontally and vertically.
    pub const _420: Self = Self(2);
    ///[`422`] - the format is 4:2:2
    ///chroma subsampled.
    ///The two chroma components are sampled at half the sample rate of luma.
    ///The horizontal chroma resolution is halved.
    pub const _422: Self = Self(4);
    ///[`444`] - the format is 4:4:4
    ///chroma sampled.
    ///Each of the three YCbCr components have the same sample rate, thus there
    ///is no chroma subsampling.
    pub const _444: Self = Self(8);
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
            all |= Self::INVALID;
        }
        {
            all |= Self::MONOCHROME;
        }
        {
            all |= Self::_420;
        }
        {
            all |= Self::_422;
        }
        {
            all |= Self::_444;
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
impl const std::ops::BitOr for VideoChromaSubsamplingFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for VideoChromaSubsamplingFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for VideoChromaSubsamplingFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for VideoChromaSubsamplingFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for VideoChromaSubsamplingFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for VideoChromaSubsamplingFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for VideoChromaSubsamplingFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for VideoChromaSubsamplingFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for VideoChromaSubsamplingFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<VideoChromaSubsamplingFlagsKHR> for VideoChromaSubsamplingFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoChromaSubsamplingFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<VideoChromaSubsamplingFlagBitsKHR> for VideoChromaSubsamplingFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoChromaSubsamplingFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<VideoChromaSubsamplingFlagBitsKHR>>::from(i));
        }
    }
}
impl FromIterator<VideoChromaSubsamplingFlagsKHR> for VideoChromaSubsamplingFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoChromaSubsamplingFlagsKHR>>(
        iterator: T,
    ) -> VideoChromaSubsamplingFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoChromaSubsamplingFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<VideoChromaSubsamplingFlagBitsKHR> for VideoChromaSubsamplingFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoChromaSubsamplingFlagBitsKHR>>(
        iterator: T,
    ) -> VideoChromaSubsamplingFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoChromaSubsamplingFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for VideoChromaSubsamplingFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoChromaSubsamplingFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoChromaSubsamplingFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(VideoChromaSubsamplingFlagsKHR::INVALID) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(INVALID))?;
                    }
                    if self.0.contains(VideoChromaSubsamplingFlagsKHR::MONOCHROME) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(MONOCHROME))?;
                    }
                    if self.0.contains(VideoChromaSubsamplingFlagsKHR::_420) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(_420))?;
                    }
                    if self.0.contains(VideoChromaSubsamplingFlagsKHR::_422) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(_422))?;
                    }
                    if self.0.contains(VideoChromaSubsamplingFlagsKHR::_444) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(_444))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoChromaSubsamplingFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoComponentBitDepthFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoComponentBitDepthFlagBitsKHR.html) - Video component bit depth
///# C Specifications
///The video format component bit depth is defined with the following enums:
///```c
///// Provided by VK_KHR_video_queue
///typedef enum VkVideoComponentBitDepthFlagBitsKHR {
///    VK_VIDEO_COMPONENT_BIT_DEPTH_INVALID_KHR = 0,
///    VK_VIDEO_COMPONENT_BIT_DEPTH_8_BIT_KHR = 0x00000001,
///    VK_VIDEO_COMPONENT_BIT_DEPTH_10_BIT_KHR = 0x00000004,
///    VK_VIDEO_COMPONENT_BIT_DEPTH_12_BIT_KHR = 0x00000010,
///} VkVideoComponentBitDepthFlagBitsKHR;
///```
///# Description
/// - [`VIDEO_COMPONENT_DEPTH8`] - the format component bit depth is 8 bits.
/// - [`VIDEO_COMPONENT_DEPTH10`] - the format component bit depth is 10 bits.
/// - [`VIDEO_COMPONENT_DEPTH12`] - the format component bit depth is 12 bits.
///# Related
/// - [`khr_video_queue`]
/// - [`VideoComponentBitDepthFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoComponentBitDepthFlagsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoComponentBitDepthFlagsKHR(u32);
impl const Default for VideoComponentBitDepthFlagsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl From<VideoComponentBitDepthFlagBitsKHR> for VideoComponentBitDepthFlagsKHR {
    fn from(from: VideoComponentBitDepthFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(from.bits()) }
    }
}
impl VideoComponentBitDepthFlagsKHR {
    ///No documentation found
    pub const VIDEO_COMPONENT_DEPTH_INVALID: Self = Self(0);
    ///[`VIDEO_COMPONENT_DEPTH8`] - the format component bit
    ///depth is 8 bits.
    pub const VIDEO_COMPONENT_DEPTH8: Self = Self(1);
    ///[`VIDEO_COMPONENT_DEPTH10`] - the format component bit
    ///depth is 10 bits.
    pub const VIDEO_COMPONENT_DEPTH10: Self = Self(4);
    ///[`VIDEO_COMPONENT_DEPTH12`] - the format component bit
    ///depth is 12 bits.
    pub const VIDEO_COMPONENT_DEPTH12: Self = Self(16);
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
            all |= Self::VIDEO_COMPONENT_DEPTH_INVALID;
        }
        {
            all |= Self::VIDEO_COMPONENT_DEPTH8;
        }
        {
            all |= Self::VIDEO_COMPONENT_DEPTH10;
        }
        {
            all |= Self::VIDEO_COMPONENT_DEPTH12;
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
impl const std::ops::BitOr for VideoComponentBitDepthFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for VideoComponentBitDepthFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for VideoComponentBitDepthFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for VideoComponentBitDepthFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for VideoComponentBitDepthFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for VideoComponentBitDepthFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for VideoComponentBitDepthFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for VideoComponentBitDepthFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for VideoComponentBitDepthFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<VideoComponentBitDepthFlagsKHR> for VideoComponentBitDepthFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoComponentBitDepthFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<VideoComponentBitDepthFlagBitsKHR> for VideoComponentBitDepthFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoComponentBitDepthFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<VideoComponentBitDepthFlagBitsKHR>>::from(i));
        }
    }
}
impl FromIterator<VideoComponentBitDepthFlagsKHR> for VideoComponentBitDepthFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoComponentBitDepthFlagsKHR>>(
        iterator: T,
    ) -> VideoComponentBitDepthFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoComponentBitDepthFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<VideoComponentBitDepthFlagBitsKHR> for VideoComponentBitDepthFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoComponentBitDepthFlagBitsKHR>>(
        iterator: T,
    ) -> VideoComponentBitDepthFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoComponentBitDepthFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for VideoComponentBitDepthFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoComponentBitDepthFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoComponentBitDepthFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self
                        .0
                        .contains(VideoComponentBitDepthFlagsKHR::VIDEO_COMPONENT_DEPTH_INVALID)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(VIDEO_COMPONENT_DEPTH_INVALID))?;
                    }
                    if self.0.contains(VideoComponentBitDepthFlagsKHR::VIDEO_COMPONENT_DEPTH8) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(VIDEO_COMPONENT_DEPTH8))?;
                    }
                    if self.0.contains(VideoComponentBitDepthFlagsKHR::VIDEO_COMPONENT_DEPTH10) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(VIDEO_COMPONENT_DEPTH10))?;
                    }
                    if self.0.contains(VideoComponentBitDepthFlagsKHR::VIDEO_COMPONENT_DEPTH12) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(VIDEO_COMPONENT_DEPTH12))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoComponentBitDepthFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoQueueFamilyProperties2KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoQueueFamilyProperties2KHR.html) - Structure specifying the codec video operations
///# C Specifications
///The [`VideoQueueFamilyProperties2KHR`] structure is defined as:
///```c
///// Provided by VK_KHR_video_queue
///typedef struct VkVideoQueueFamilyProperties2KHR {
///    VkStructureType                  sType;
///    void*                            pNext;
///    VkVideoCodecOperationFlagsKHR    videoCodecOperations;
///} VkVideoQueueFamilyProperties2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`video_codec_operations`] is a bitmask of [`VideoCodecOperationFlagBitsKHR`] specifying
///   supported video codec operation(s).
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_QUEUE_FAMILY_PROPERTIES_2_KHR`
/// - [`video_codec_operations`] **must**  be a valid combination of
///   [`VideoCodecOperationFlagBitsKHR`] values
/// - [`video_codec_operations`] **must**  not be `0`
///# Related
/// - [`khr_video_queue`]
/// - [`StructureType`]
/// - [`VideoCodecOperationFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoQueueFamilyProperties2KHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct VideoQueueFamilyProperties2KHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`video_codec_operations`] is a bitmask of
    ///[`VideoCodecOperationFlagBitsKHR`] specifying supported video codec
    ///operation(s).
    pub video_codec_operations: VideoCodecOperationFlagsKHR,
}
impl<'lt> Default for VideoQueueFamilyProperties2KHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_QUEUE_FAMILY_PROPERTIES2_KHR,
            p_next: std::ptr::null_mut(),
            video_codec_operations: Default::default(),
        }
    }
}
impl<'lt> VideoQueueFamilyProperties2KHR<'lt> {
    ///Creates a static version of this structure
    pub fn make_static(mut self) -> VideoQueueFamilyProperties2KHR<'static> {
        unsafe {
            self.p_next = std::ptr::null_mut() as _;
            std::mem::transmute(self)
        }
    }
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
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
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::video_codec_operations`]
    pub fn video_codec_operations(&self) -> VideoCodecOperationFlagsKHR {
        self.video_codec_operations
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::video_codec_operations`]
    pub fn video_codec_operations_mut(&mut self) -> &mut VideoCodecOperationFlagsKHR {
        &mut self.video_codec_operations
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::video_codec_operations`]
    pub fn set_video_codec_operations(
        &mut self,
        value: crate::extensions::khr_video_queue::VideoCodecOperationFlagsKHR,
    ) -> &mut Self {
        self.video_codec_operations = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::video_codec_operations`]
    pub fn with_video_codec_operations(
        mut self,
        value: crate::extensions::khr_video_queue::VideoCodecOperationFlagsKHR,
    ) -> Self {
        self.video_codec_operations = value;
        self
    }
}
///[VkQueueFamilyQueryResultStatusProperties2KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyQueryResultStatusProperties2KHR.html) - Structure specifying support for result status query
///# C Specifications
///The [`QueueFamilyQueryResultStatusProperties2KHR`] structure is defined
///as:
///```c
///// Provided by VK_KHR_video_queue
///typedef struct VkQueueFamilyQueryResultStatusProperties2KHR {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           supported;
///} VkQueueFamilyQueryResultStatusProperties2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`supported`] reports [`TRUE`] if query type `VK_QUERY_TYPE_RESULT_STATUS_ONLY_KHR` and use of
///   `VK_QUERY_RESULT_WITH_STATUS_BIT_KHR` are supported.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_2_KHR`
///# Related
/// - [`khr_video_queue`]
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
#[doc(alias = "VkQueueFamilyQueryResultStatusProperties2KHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct QueueFamilyQueryResultStatusProperties2KHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`supported`] reports [`TRUE`] if query type
    ///`VK_QUERY_TYPE_RESULT_STATUS_ONLY_KHR` and use of
    ///`VK_QUERY_RESULT_WITH_STATUS_BIT_KHR` are supported.
    pub supported: Bool32,
}
impl<'lt> Default for QueueFamilyQueryResultStatusProperties2KHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES2_KHR,
            p_next: std::ptr::null_mut(),
            supported: 0,
        }
    }
}
impl<'lt> QueueFamilyQueryResultStatusProperties2KHR<'lt> {
    ///Creates a static version of this structure
    pub fn make_static(mut self) -> QueueFamilyQueryResultStatusProperties2KHR<'static> {
        unsafe {
            self.p_next = std::ptr::null_mut() as _;
            std::mem::transmute(self)
        }
    }
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::supported`]
    pub fn supported_raw(&self) -> Bool32 {
        self.supported
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::supported`]
    pub fn set_supported_raw(&mut self, value: Bool32) -> &mut Self {
        self.supported = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::supported`]
    pub fn with_supported_raw(mut self, value: Bool32) -> Self {
        self.supported = value;
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
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::supported`]
    pub fn supported(&self) -> bool {
        unsafe { std::mem::transmute(self.supported as u8) }
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::supported`]
    pub fn supported_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.supported as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.supported as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::supported`]
    pub fn set_supported(&mut self, value: bool) -> &mut Self {
        self.supported = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::supported`]
    pub fn with_supported(mut self, value: bool) -> Self {
        self.supported = value as u8 as u32;
        self
    }
}
///[VkVideoProfilesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoProfilesKHR.html) - Structure enumerating the video profiles
///# C Specifications
///The [`VideoProfilesKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_video_queue
///typedef struct VkVideoProfilesKHR {
///    VkStructureType             sType;
///    void*                       pNext;
///    uint32_t                    profileCount;
///    const VkVideoProfileKHR*    pProfiles;
///} VkVideoProfilesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`profile_count`] is an integer which holds the number of video profiles included in
///   [`profiles`].
/// - [`profiles`] is a pointer to an array of [`VideoProfileKHR`] structures. Each
///   [`VideoProfileKHR`] structure  **must**  chain the corresponding codec-operation specific
///   extension video profile structure.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_PROFILES_KHR`
/// - [`profiles`] **must**  be a valid pointer to an array of [`profile_count`] valid
///   [`VideoProfileKHR`] structures
/// - [`profile_count`] **must**  be greater than `0`
///# Related
/// - [`khr_video_queue`]
/// - [`PhysicalDeviceVideoFormatInfoKHR`]
/// - [`StructureType`]
/// - [`VideoProfileKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoProfilesKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct VideoProfilesKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`profile_count`] is an integer which holds the number of video
    ///profiles included in [`profiles`].
    pub profile_count: u32,
    ///[`profiles`] is a pointer to an array of [`VideoProfileKHR`]
    ///structures.
    ///Each [`VideoProfileKHR`] structure  **must**  chain the corresponding
    ///codec-operation specific extension video profile structure.
    pub profiles: *const VideoProfileKHR<'lt>,
}
impl<'lt> Default for VideoProfilesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_PROFILES_KHR,
            p_next: std::ptr::null_mut(),
            profile_count: 0,
            profiles: std::ptr::null(),
        }
    }
}
impl<'lt> VideoProfilesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::profiles`]
    pub fn profiles_raw(&self) -> *const VideoProfileKHR<'lt> {
        self.profiles
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::profiles`]
    pub fn set_profiles_raw(&mut self, value: *const VideoProfileKHR<'lt>) -> &mut Self {
        self.profiles = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::profiles`]
    pub fn with_profiles_raw(mut self, value: *const VideoProfileKHR<'lt>) -> Self {
        self.profiles = value;
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
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::profile_count`]
    pub fn profile_count(&self) -> u32 {
        self.profile_count
    }
    ///Gets the value of [`Self::profiles`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn profiles(&self) -> &[VideoProfileKHR<'lt>] {
        std::slice::from_raw_parts(self.profiles, self.profile_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::profile_count`]
    pub fn profile_count_mut(&mut self) -> &mut u32 {
        &mut self.profile_count
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::profile_count`]
    pub fn set_profile_count(&mut self, value: u32) -> &mut Self {
        self.profile_count = value;
        self
    }
    ///Sets the value of [`Self::profiles`]
    pub fn set_profiles(
        &mut self,
        value: &'lt [crate::extensions::khr_video_queue::VideoProfileKHR<'lt>],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.profiles = value.as_ptr();
        self.profile_count = len_;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::profile_count`]
    pub fn with_profile_count(mut self, value: u32) -> Self {
        self.profile_count = value;
        self
    }
    ///Sets the value of [`Self::profiles`]
    pub fn with_profiles(mut self, value: &'lt [crate::extensions::khr_video_queue::VideoProfileKHR<'lt>]) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.profiles = value.as_ptr();
        self.profile_count = len_;
        self
    }
}
///[VkPhysicalDeviceVideoFormatInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVideoFormatInfoKHR.html) - Structure specifying the codec video format
///# C Specifications
///The [`PhysicalDeviceVideoFormatInfoKHR`] input structure is defined as:
///```c
///// Provided by VK_KHR_video_queue
///typedef struct VkPhysicalDeviceVideoFormatInfoKHR {
///    VkStructureType              sType;
///    void*                        pNext;
///    VkImageUsageFlags            imageUsage;
///    const VkVideoProfilesKHR*    pVideoProfiles;
///} VkPhysicalDeviceVideoFormatInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`image_usage`] is a bitmask of [`ImageUsageFlagBits`] specifying intended video image usages.
/// - [`video_profiles`] is a pointer to a [`VideoProfilesKHR`] structure providing the video
///   profile(s) of video session(s) that will use the image. For most use cases, the image is used
///   by a single video session and a single video profile is provided. For a use case such as
///   transcode, where a decode session output image  **may**  be used as encode input for one or
///   more encode sessions, multiple video profiles representing the video sessions that will share
///   the image  **may**  be provided.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
///# Related
/// - [`khr_video_queue`]
/// - [`ImageUsageFlags`]
/// - [`StructureType`]
/// - [`VideoProfilesKHR`]
/// - [`get_physical_device_video_format_properties_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceVideoFormatInfoKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceVideoFormatInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`image_usage`] is a bitmask of [`ImageUsageFlagBits`] specifying
    ///intended video image usages.
    pub image_usage: ImageUsageFlags,
    ///[`video_profiles`] is a pointer to a [`VideoProfilesKHR`]
    ///structure providing the video profile(s) of video session(s) that will
    ///use the image.
    ///For most use cases, the image is used by a single video session and a
    ///single video profile is provided.
    ///For a use case such as transcode, where a decode session output image
    /// **may**  be used as encode input for one or more encode sessions, multiple
    ///video profiles representing the video sessions that will share the image
    /// **may**  be provided.
    pub video_profiles: *const VideoProfilesKHR<'lt>,
}
impl<'lt> Default for PhysicalDeviceVideoFormatInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR,
            p_next: std::ptr::null_mut(),
            image_usage: Default::default(),
            video_profiles: std::ptr::null(),
        }
    }
}
impl<'lt> PhysicalDeviceVideoFormatInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::video_profiles`]
    pub fn video_profiles_raw(&self) -> *const VideoProfilesKHR<'lt> {
        self.video_profiles
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::video_profiles`]
    pub fn set_video_profiles_raw(&mut self, value: *const VideoProfilesKHR<'lt>) -> &mut Self {
        self.video_profiles = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::video_profiles`]
    pub fn with_video_profiles_raw(mut self, value: *const VideoProfilesKHR<'lt>) -> Self {
        self.video_profiles = value;
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
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::image_usage`]
    pub fn image_usage(&self) -> ImageUsageFlags {
        self.image_usage
    }
    ///Gets the value of [`Self::video_profiles`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn video_profiles(&self) -> &VideoProfilesKHR<'lt> {
        &*self.video_profiles
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::image_usage`]
    pub fn image_usage_mut(&mut self) -> &mut ImageUsageFlags {
        &mut self.image_usage
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::image_usage`]
    pub fn set_image_usage(&mut self, value: crate::vulkan1_0::ImageUsageFlags) -> &mut Self {
        self.image_usage = value;
        self
    }
    ///Sets the value of [`Self::video_profiles`]
    pub fn set_video_profiles(
        &mut self,
        value: &'lt crate::extensions::khr_video_queue::VideoProfilesKHR<'lt>,
    ) -> &mut Self {
        self.video_profiles = value as *const _;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::image_usage`]
    pub fn with_image_usage(mut self, value: crate::vulkan1_0::ImageUsageFlags) -> Self {
        self.image_usage = value;
        self
    }
    ///Sets the value of [`Self::video_profiles`]
    pub fn with_video_profiles(
        mut self,
        value: &'lt crate::extensions::khr_video_queue::VideoProfilesKHR<'lt>,
    ) -> Self {
        self.video_profiles = value as *const _;
        self
    }
}
///[VkVideoFormatPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoFormatPropertiesKHR.html) - Structure enumerating the video image formats
///# C Specifications
///The [`VideoFormatPropertiesKHR`] output structure for
///[`get_physical_device_video_format_properties_khr`] is defined as:
///```c
///// Provided by VK_KHR_video_queue
///typedef struct VkVideoFormatPropertiesKHR {
///    VkStructureType    sType;
///    void*              pNext;
///    VkFormat           format;
///} VkVideoFormatPropertiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`format`] is one of the supported formats reported by the implementation.
///# Description
///If the `pVideoProfiles` or `imageUsage` provided in input structure
///`pVideoFormatInfo` are not supported,
///`VK_ERROR_FORMAT_NOT_SUPPORTED` is returned.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_FORMAT_PROPERTIES_KHR`
/// - [`p_next`] **must**  be `NULL`
///# Related
/// - [`khr_video_queue`]
/// - [`Format`]
/// - [`StructureType`]
/// - [`get_physical_device_video_format_properties_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoFormatPropertiesKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct VideoFormatPropertiesKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`format`] is one of the supported formats reported by the
    ///implementation.
    pub format: Format,
}
impl<'lt> Default for VideoFormatPropertiesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_FORMAT_PROPERTIES_KHR,
            p_next: std::ptr::null_mut(),
            format: Default::default(),
        }
    }
}
impl<'lt> VideoFormatPropertiesKHR<'lt> {
    ///Creates a static version of this structure
    pub fn make_static(mut self) -> VideoFormatPropertiesKHR<'static> {
        unsafe {
            self.p_next = std::ptr::null_mut() as _;
            std::mem::transmute(self)
        }
    }
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
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
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::format`]
    pub fn format(&self) -> Format {
        self.format
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::format`]
    pub fn format_mut(&mut self) -> &mut Format {
        &mut self.format
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::format`]
    pub fn set_format(&mut self, value: crate::vulkan1_0::Format) -> &mut Self {
        self.format = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::format`]
    pub fn with_format(mut self, value: crate::vulkan1_0::Format) -> Self {
        self.format = value;
        self
    }
}
///[VkVideoProfileKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoProfileKHR.html) - Structure specifying the codec video profile
///# C Specifications
///A video profile is defined by [`VideoProfileKHR`] structure as:
///```c
///// Provided by VK_KHR_video_queue
///typedef struct VkVideoProfileKHR {
///    VkStructureType                     sType;
///    void*                               pNext;
///    VkVideoCodecOperationFlagBitsKHR    videoCodecOperation;
///    VkVideoChromaSubsamplingFlagsKHR    chromaSubsampling;
///    VkVideoComponentBitDepthFlagsKHR    lumaBitDepth;
///    VkVideoComponentBitDepthFlagsKHR    chromaBitDepth;
///} VkVideoProfileKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`video_codec_operation`] is a [`VideoCodecOperationFlagBitsKHR`] value specifying a video
///   codec operation.
/// - [`chroma_subsampling`] is a bitmask of [`VideoChromaSubsamplingFlagBitsKHR`] specifying video
///   chroma subsampling information.
/// - [`luma_bit_depth`] is a bitmask of [`VideoComponentBitDepthFlagBitsKHR`] specifying video luma
///   bit depth information.
/// - [`chroma_bit_depth`] is a bitmask of [`VideoComponentBitDepthFlagBitsKHR`] specifying video
///   chroma bit depth information.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_PROFILE_KHR`
/// - [`video_codec_operation`] **must**  be a valid [`VideoCodecOperationFlagBitsKHR`] value
/// - [`chroma_subsampling`] **must**  be a valid combination of
///   [`VideoChromaSubsamplingFlagBitsKHR`] values
/// - [`chroma_subsampling`] **must**  not be `0`
/// - [`luma_bit_depth`] **must**  be a valid combination of [`VideoComponentBitDepthFlagBitsKHR`]
///   values
/// - [`luma_bit_depth`] **must**  not be `0`
/// - [`chroma_bit_depth`] **must**  be a valid combination of [`VideoComponentBitDepthFlagBitsKHR`]
///   values
/// - [`chroma_bit_depth`] **must**  not be `0`
///# Related
/// - [`khr_video_queue`]
/// - [`StructureType`]
/// - [`VideoChromaSubsamplingFlagsKHR`]
/// - [`VideoCodecOperationFlagBitsKHR`]
/// - [`VideoComponentBitDepthFlagsKHR`]
/// - [`VideoProfilesKHR`]
/// - [`VideoSessionCreateInfoKHR`]
/// - [`get_physical_device_video_capabilities_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoProfileKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct VideoProfileKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`video_codec_operation`] is a [`VideoCodecOperationFlagBitsKHR`]
    ///value specifying a video codec operation.
    pub video_codec_operation: VideoCodecOperationFlagBitsKHR,
    ///[`chroma_subsampling`] is a bitmask of
    ///[`VideoChromaSubsamplingFlagBitsKHR`] specifying video chroma
    ///subsampling information.
    pub chroma_subsampling: VideoChromaSubsamplingFlagsKHR,
    ///[`luma_bit_depth`] is a bitmask of
    ///[`VideoComponentBitDepthFlagBitsKHR`] specifying video luma bit
    ///depth information.
    pub luma_bit_depth: VideoComponentBitDepthFlagsKHR,
    ///[`chroma_bit_depth`] is a bitmask of
    ///[`VideoComponentBitDepthFlagBitsKHR`] specifying video chroma bit
    ///depth information.
    pub chroma_bit_depth: VideoComponentBitDepthFlagsKHR,
}
impl<'lt> Default for VideoProfileKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_PROFILE_KHR,
            p_next: std::ptr::null_mut(),
            video_codec_operation: Default::default(),
            chroma_subsampling: Default::default(),
            luma_bit_depth: Default::default(),
            chroma_bit_depth: Default::default(),
        }
    }
}
impl<'lt> VideoProfileKHR<'lt> {
    ///Creates a static version of this structure
    pub fn make_static(mut self) -> VideoProfileKHR<'static> {
        unsafe {
            self.p_next = std::ptr::null_mut() as _;
            std::mem::transmute(self)
        }
    }
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
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
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::video_codec_operation`]
    pub fn video_codec_operation(&self) -> VideoCodecOperationFlagBitsKHR {
        self.video_codec_operation
    }
    ///Gets the value of [`Self::chroma_subsampling`]
    pub fn chroma_subsampling(&self) -> VideoChromaSubsamplingFlagsKHR {
        self.chroma_subsampling
    }
    ///Gets the value of [`Self::luma_bit_depth`]
    pub fn luma_bit_depth(&self) -> VideoComponentBitDepthFlagsKHR {
        self.luma_bit_depth
    }
    ///Gets the value of [`Self::chroma_bit_depth`]
    pub fn chroma_bit_depth(&self) -> VideoComponentBitDepthFlagsKHR {
        self.chroma_bit_depth
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::video_codec_operation`]
    pub fn video_codec_operation_mut(&mut self) -> &mut VideoCodecOperationFlagBitsKHR {
        &mut self.video_codec_operation
    }
    ///Gets a mutable reference to the value of [`Self::chroma_subsampling`]
    pub fn chroma_subsampling_mut(&mut self) -> &mut VideoChromaSubsamplingFlagsKHR {
        &mut self.chroma_subsampling
    }
    ///Gets a mutable reference to the value of [`Self::luma_bit_depth`]
    pub fn luma_bit_depth_mut(&mut self) -> &mut VideoComponentBitDepthFlagsKHR {
        &mut self.luma_bit_depth
    }
    ///Gets a mutable reference to the value of [`Self::chroma_bit_depth`]
    pub fn chroma_bit_depth_mut(&mut self) -> &mut VideoComponentBitDepthFlagsKHR {
        &mut self.chroma_bit_depth
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::video_codec_operation`]
    pub fn set_video_codec_operation(
        &mut self,
        value: crate::extensions::khr_video_queue::VideoCodecOperationFlagBitsKHR,
    ) -> &mut Self {
        self.video_codec_operation = value;
        self
    }
    ///Sets the value of [`Self::chroma_subsampling`]
    pub fn set_chroma_subsampling(
        &mut self,
        value: crate::extensions::khr_video_queue::VideoChromaSubsamplingFlagsKHR,
    ) -> &mut Self {
        self.chroma_subsampling = value;
        self
    }
    ///Sets the value of [`Self::luma_bit_depth`]
    pub fn set_luma_bit_depth(
        &mut self,
        value: crate::extensions::khr_video_queue::VideoComponentBitDepthFlagsKHR,
    ) -> &mut Self {
        self.luma_bit_depth = value;
        self
    }
    ///Sets the value of [`Self::chroma_bit_depth`]
    pub fn set_chroma_bit_depth(
        &mut self,
        value: crate::extensions::khr_video_queue::VideoComponentBitDepthFlagsKHR,
    ) -> &mut Self {
        self.chroma_bit_depth = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::video_codec_operation`]
    pub fn with_video_codec_operation(
        mut self,
        value: crate::extensions::khr_video_queue::VideoCodecOperationFlagBitsKHR,
    ) -> Self {
        self.video_codec_operation = value;
        self
    }
    ///Sets the value of [`Self::chroma_subsampling`]
    pub fn with_chroma_subsampling(
        mut self,
        value: crate::extensions::khr_video_queue::VideoChromaSubsamplingFlagsKHR,
    ) -> Self {
        self.chroma_subsampling = value;
        self
    }
    ///Sets the value of [`Self::luma_bit_depth`]
    pub fn with_luma_bit_depth(
        mut self,
        value: crate::extensions::khr_video_queue::VideoComponentBitDepthFlagsKHR,
    ) -> Self {
        self.luma_bit_depth = value;
        self
    }
    ///Sets the value of [`Self::chroma_bit_depth`]
    pub fn with_chroma_bit_depth(
        mut self,
        value: crate::extensions::khr_video_queue::VideoComponentBitDepthFlagsKHR,
    ) -> Self {
        self.chroma_bit_depth = value;
        self
    }
}
#[cfg(feature = "VK_EXT_video_decode_h264")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, VideoDecodeH264ProfileEXT<'extender>> for VideoProfileKHR<'this>
{
    type Out = VideoProfileKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut VideoDecodeH264ProfileEXT<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut VideoDecodeH264ProfileEXT<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
    #[must_use]
    #[inline]
    fn chain_opt(self, new: Option<&'other mut VideoDecodeH264ProfileEXT<'extender>>) -> Self::Out {
        match new {
            Some(new) => self.chain(new),
            None => unsafe { std::mem::transmute(self) },
        }
    }
}
#[cfg(feature = "VK_EXT_video_decode_h265")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, VideoDecodeH265ProfileEXT<'extender>> for VideoProfileKHR<'this>
{
    type Out = VideoProfileKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut VideoDecodeH265ProfileEXT<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut VideoDecodeH265ProfileEXT<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
    #[must_use]
    #[inline]
    fn chain_opt(self, new: Option<&'other mut VideoDecodeH265ProfileEXT<'extender>>) -> Self::Out {
        match new {
            Some(new) => self.chain(new),
            None => unsafe { std::mem::transmute(self) },
        }
    }
}
#[cfg(feature = "VK_EXT_video_encode_h264")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, VideoEncodeH264ProfileEXT<'extender>> for VideoProfileKHR<'this>
{
    type Out = VideoProfileKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut VideoEncodeH264ProfileEXT<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut VideoEncodeH264ProfileEXT<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
    #[must_use]
    #[inline]
    fn chain_opt(self, new: Option<&'other mut VideoEncodeH264ProfileEXT<'extender>>) -> Self::Out {
        match new {
            Some(new) => self.chain(new),
            None => unsafe { std::mem::transmute(self) },
        }
    }
}
#[cfg(feature = "VK_EXT_video_encode_h265")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, VideoEncodeH265ProfileEXT<'extender>> for VideoProfileKHR<'this>
{
    type Out = VideoProfileKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut VideoEncodeH265ProfileEXT<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut VideoEncodeH265ProfileEXT<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
    #[must_use]
    #[inline]
    fn chain_opt(self, new: Option<&'other mut VideoEncodeH265ProfileEXT<'extender>>) -> Self::Out {
        match new {
            Some(new) => self.chain(new),
            None => unsafe { std::mem::transmute(self) },
        }
    }
}
///[VkVideoCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoCapabilitiesKHR.html) - Structure specifying parameters of video capabilities
///# C Specifications
///The [`VideoCapabilitiesKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_video_queue
///typedef struct VkVideoCapabilitiesKHR {
///    VkStructureType              sType;
///    void*                        pNext;
///    VkVideoCapabilityFlagsKHR    capabilityFlags;
///    VkDeviceSize                 minBitstreamBufferOffsetAlignment;
///    VkDeviceSize                 minBitstreamBufferSizeAlignment;
///    VkExtent2D                   videoPictureExtentGranularity;
///    VkExtent2D                   minExtent;
///    VkExtent2D                   maxExtent;
///    uint32_t                     maxReferencePicturesSlotsCount;
///    uint32_t                     maxReferencePicturesActiveCount;
///    VkExtensionProperties        stdHeaderVersion;
///} VkVideoCapabilitiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`capability_flags`] is a bitmask of [`VideoCapabilityFlagBitsKHR`] specifying capability
///   flags.
/// - [`min_bitstream_buffer_offset_alignment`] is the minimum alignment for the input or output
///   bitstream buffer offset.
/// - [`min_bitstream_buffer_size_alignment`] is the minimum alignment for the input or output
///   bitstream buffer size
/// - [`video_picture_extent_granularity`] is the minimum size alignment of the extent with the
///   required padding for the decoded or encoded video images.
/// - [`min_extent`] is the minimum width and height of the decoded or encoded video.
/// - [`max_extent`] is the maximum width and height of the decoded or encoded video.
/// - [`max_reference_pictures_slots_count`] is the maximum number of DPB Slots supported by the
///   implementation for a single video session instance.
/// - [`max_reference_pictures_active_count`] is the maximum slots that can be used as [Reference Pictures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#reference-picture)
///   with a single decode or encode operation.
/// - [`std_header_version`] is a [`ExtensionProperties`] structure reporting the Video Std header
///   version supported for the `codecOperation` requested in
///   [`get_physical_device_video_capabilities_khr`]`::pVideoProfile`.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_CAPABILITIES_KHR`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**
///   be either `NULL` or a pointer to a valid instance of [`VideoDecodeCapabilitiesKHR`] or
///   [`VideoEncodeCapabilitiesKHR`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
///# Related
/// - [`khr_video_queue`]
/// - [`DeviceSize`]
/// - [`ExtensionProperties`]
/// - [`Extent2D`]
/// - [`StructureType`]
/// - [`VideoCapabilityFlagsKHR`]
/// - [`get_physical_device_video_capabilities_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoCapabilitiesKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct VideoCapabilitiesKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`capability_flags`] is a bitmask of [`VideoCapabilityFlagBitsKHR`]
    ///specifying capability flags.
    pub capability_flags: VideoCapabilityFlagsKHR,
    ///[`min_bitstream_buffer_offset_alignment`] is the minimum alignment for the
    ///input or output bitstream buffer offset.
    pub min_bitstream_buffer_offset_alignment: DeviceSize,
    ///[`min_bitstream_buffer_size_alignment`] is the minimum alignment for the
    ///input or output bitstream buffer size
    pub min_bitstream_buffer_size_alignment: DeviceSize,
    ///[`video_picture_extent_granularity`] is the minimum size alignment of the
    ///extent with the required padding for the decoded or encoded video
    ///images.
    pub video_picture_extent_granularity: Extent2D,
    ///[`min_extent`] is the minimum width and height of the decoded or
    ///encoded video.
    pub min_extent: Extent2D,
    ///[`max_extent`] is the maximum width and height of the decoded or
    ///encoded video.
    pub max_extent: Extent2D,
    ///[`max_reference_pictures_slots_count`] is the maximum number of DPB Slots
    ///supported by the implementation for a single video session instance.
    pub max_reference_pictures_slots_count: u32,
    ///[`max_reference_pictures_active_count`] is the maximum slots that can be
    ///used as [Reference Pictures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#reference-picture) with a single decode or
    ///encode operation.
    pub max_reference_pictures_active_count: u32,
    ///[`std_header_version`] is a [`ExtensionProperties`] structure
    ///reporting the Video Std header version supported for the
    ///`codecOperation` requested in
    ///[`get_physical_device_video_capabilities_khr`]::`pVideoProfile`.
    pub std_header_version: ExtensionProperties,
}
impl<'lt> Default for VideoCapabilitiesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_CAPABILITIES_KHR,
            p_next: std::ptr::null_mut(),
            capability_flags: Default::default(),
            min_bitstream_buffer_offset_alignment: Default::default(),
            min_bitstream_buffer_size_alignment: Default::default(),
            video_picture_extent_granularity: Default::default(),
            min_extent: Default::default(),
            max_extent: Default::default(),
            max_reference_pictures_slots_count: 0,
            max_reference_pictures_active_count: 0,
            std_header_version: Default::default(),
        }
    }
}
impl<'lt> VideoCapabilitiesKHR<'lt> {
    ///Creates a static version of this structure
    pub fn make_static(mut self) -> VideoCapabilitiesKHR<'static> {
        unsafe {
            self.p_next = std::ptr::null_mut() as _;
            std::mem::transmute(self)
        }
    }
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
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
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::capability_flags`]
    pub fn capability_flags(&self) -> VideoCapabilityFlagsKHR {
        self.capability_flags
    }
    ///Gets the value of [`Self::min_bitstream_buffer_offset_alignment`]
    pub fn min_bitstream_buffer_offset_alignment(&self) -> DeviceSize {
        self.min_bitstream_buffer_offset_alignment
    }
    ///Gets the value of [`Self::min_bitstream_buffer_size_alignment`]
    pub fn min_bitstream_buffer_size_alignment(&self) -> DeviceSize {
        self.min_bitstream_buffer_size_alignment
    }
    ///Gets the value of [`Self::video_picture_extent_granularity`]
    pub fn video_picture_extent_granularity(&self) -> Extent2D {
        self.video_picture_extent_granularity
    }
    ///Gets the value of [`Self::min_extent`]
    pub fn min_extent(&self) -> Extent2D {
        self.min_extent
    }
    ///Gets the value of [`Self::max_extent`]
    pub fn max_extent(&self) -> Extent2D {
        self.max_extent
    }
    ///Gets the value of [`Self::max_reference_pictures_slots_count`]
    pub fn max_reference_pictures_slots_count(&self) -> u32 {
        self.max_reference_pictures_slots_count
    }
    ///Gets the value of [`Self::max_reference_pictures_active_count`]
    pub fn max_reference_pictures_active_count(&self) -> u32 {
        self.max_reference_pictures_active_count
    }
    ///Gets the value of [`Self::std_header_version`]
    pub fn std_header_version(&self) -> ExtensionProperties {
        self.std_header_version
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::capability_flags`]
    pub fn capability_flags_mut(&mut self) -> &mut VideoCapabilityFlagsKHR {
        &mut self.capability_flags
    }
    ///Gets a mutable reference to the value of [`Self::min_bitstream_buffer_offset_alignment`]
    pub fn min_bitstream_buffer_offset_alignment_mut(&mut self) -> &mut DeviceSize {
        &mut self.min_bitstream_buffer_offset_alignment
    }
    ///Gets a mutable reference to the value of [`Self::min_bitstream_buffer_size_alignment`]
    pub fn min_bitstream_buffer_size_alignment_mut(&mut self) -> &mut DeviceSize {
        &mut self.min_bitstream_buffer_size_alignment
    }
    ///Gets a mutable reference to the value of [`Self::video_picture_extent_granularity`]
    pub fn video_picture_extent_granularity_mut(&mut self) -> &mut Extent2D {
        &mut self.video_picture_extent_granularity
    }
    ///Gets a mutable reference to the value of [`Self::min_extent`]
    pub fn min_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.min_extent
    }
    ///Gets a mutable reference to the value of [`Self::max_extent`]
    pub fn max_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.max_extent
    }
    ///Gets a mutable reference to the value of [`Self::max_reference_pictures_slots_count`]
    pub fn max_reference_pictures_slots_count_mut(&mut self) -> &mut u32 {
        &mut self.max_reference_pictures_slots_count
    }
    ///Gets a mutable reference to the value of [`Self::max_reference_pictures_active_count`]
    pub fn max_reference_pictures_active_count_mut(&mut self) -> &mut u32 {
        &mut self.max_reference_pictures_active_count
    }
    ///Gets a mutable reference to the value of [`Self::std_header_version`]
    pub fn std_header_version_mut(&mut self) -> &mut ExtensionProperties {
        &mut self.std_header_version
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::capability_flags`]
    pub fn set_capability_flags(
        &mut self,
        value: crate::extensions::khr_video_queue::VideoCapabilityFlagsKHR,
    ) -> &mut Self {
        self.capability_flags = value;
        self
    }
    ///Sets the value of [`Self::min_bitstream_buffer_offset_alignment`]
    pub fn set_min_bitstream_buffer_offset_alignment(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.min_bitstream_buffer_offset_alignment = value;
        self
    }
    ///Sets the value of [`Self::min_bitstream_buffer_size_alignment`]
    pub fn set_min_bitstream_buffer_size_alignment(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.min_bitstream_buffer_size_alignment = value;
        self
    }
    ///Sets the value of [`Self::video_picture_extent_granularity`]
    pub fn set_video_picture_extent_granularity(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.video_picture_extent_granularity = value;
        self
    }
    ///Sets the value of [`Self::min_extent`]
    pub fn set_min_extent(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.min_extent = value;
        self
    }
    ///Sets the value of [`Self::max_extent`]
    pub fn set_max_extent(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.max_extent = value;
        self
    }
    ///Sets the value of [`Self::max_reference_pictures_slots_count`]
    pub fn set_max_reference_pictures_slots_count(&mut self, value: u32) -> &mut Self {
        self.max_reference_pictures_slots_count = value;
        self
    }
    ///Sets the value of [`Self::max_reference_pictures_active_count`]
    pub fn set_max_reference_pictures_active_count(&mut self, value: u32) -> &mut Self {
        self.max_reference_pictures_active_count = value;
        self
    }
    ///Sets the value of [`Self::std_header_version`]
    pub fn set_std_header_version(&mut self, value: crate::vulkan1_0::ExtensionProperties) -> &mut Self {
        self.std_header_version = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::capability_flags`]
    pub fn with_capability_flags(mut self, value: crate::extensions::khr_video_queue::VideoCapabilityFlagsKHR) -> Self {
        self.capability_flags = value;
        self
    }
    ///Sets the value of [`Self::min_bitstream_buffer_offset_alignment`]
    pub fn with_min_bitstream_buffer_offset_alignment(mut self, value: crate::vulkan1_0::DeviceSize) -> Self {
        self.min_bitstream_buffer_offset_alignment = value;
        self
    }
    ///Sets the value of [`Self::min_bitstream_buffer_size_alignment`]
    pub fn with_min_bitstream_buffer_size_alignment(mut self, value: crate::vulkan1_0::DeviceSize) -> Self {
        self.min_bitstream_buffer_size_alignment = value;
        self
    }
    ///Sets the value of [`Self::video_picture_extent_granularity`]
    pub fn with_video_picture_extent_granularity(mut self, value: crate::vulkan1_0::Extent2D) -> Self {
        self.video_picture_extent_granularity = value;
        self
    }
    ///Sets the value of [`Self::min_extent`]
    pub fn with_min_extent(mut self, value: crate::vulkan1_0::Extent2D) -> Self {
        self.min_extent = value;
        self
    }
    ///Sets the value of [`Self::max_extent`]
    pub fn with_max_extent(mut self, value: crate::vulkan1_0::Extent2D) -> Self {
        self.max_extent = value;
        self
    }
    ///Sets the value of [`Self::max_reference_pictures_slots_count`]
    pub fn with_max_reference_pictures_slots_count(mut self, value: u32) -> Self {
        self.max_reference_pictures_slots_count = value;
        self
    }
    ///Sets the value of [`Self::max_reference_pictures_active_count`]
    pub fn with_max_reference_pictures_active_count(mut self, value: u32) -> Self {
        self.max_reference_pictures_active_count = value;
        self
    }
    ///Sets the value of [`Self::std_header_version`]
    pub fn with_std_header_version(mut self, value: crate::vulkan1_0::ExtensionProperties) -> Self {
        self.std_header_version = value;
        self
    }
}
#[cfg(feature = "VK_KHR_video_decode_queue")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, VideoDecodeCapabilitiesKHR<'extender>> for VideoCapabilitiesKHR<'this>
{
    type Out = VideoCapabilitiesKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut VideoDecodeCapabilitiesKHR<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut VideoDecodeCapabilitiesKHR<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
    #[must_use]
    #[inline]
    fn chain_opt(self, new: Option<&'other mut VideoDecodeCapabilitiesKHR<'extender>>) -> Self::Out {
        match new {
            Some(new) => self.chain(new),
            None => unsafe { std::mem::transmute(self) },
        }
    }
}
#[cfg(feature = "VK_KHR_video_encode_queue")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, VideoEncodeCapabilitiesKHR<'extender>> for VideoCapabilitiesKHR<'this>
{
    type Out = VideoCapabilitiesKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut VideoEncodeCapabilitiesKHR<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut VideoEncodeCapabilitiesKHR<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
    #[must_use]
    #[inline]
    fn chain_opt(self, new: Option<&'other mut VideoEncodeCapabilitiesKHR<'extender>>) -> Self::Out {
        match new {
            Some(new) => self.chain(new),
            None => unsafe { std::mem::transmute(self) },
        }
    }
}
///[VkVideoGetMemoryPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoGetMemoryPropertiesKHR.html) - Structure specifying video session required memory heap type
///# C Specifications
///The [`VideoGetMemoryPropertiesKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_video_queue
///typedef struct VkVideoGetMemoryPropertiesKHR {
///    VkStructureType           sType;
///    const void*               pNext;
///    uint32_t                  memoryBindIndex;
///    VkMemoryRequirements2*    pMemoryRequirements;
///} VkVideoGetMemoryPropertiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`memory_bind_index`] is the memory bind index of the memory heap type described by the
///   information returned in [`memory_requirements`].
/// - [`memory_requirements`] is a pointer to a [`MemoryRequirements2`] structure in which the
///   requested memory heap requirements for the heap with index [`memory_bind_index`] are returned.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_GET_MEMORY_PROPERTIES_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`memory_requirements`] **must**  be a valid pointer to a [`MemoryRequirements2`] structure
///# Related
/// - [`khr_video_queue`]
/// - [`MemoryRequirements2`]
/// - [`StructureType`]
/// - [`get_video_session_memory_requirements_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoGetMemoryPropertiesKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct VideoGetMemoryPropertiesKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`memory_bind_index`] is the memory bind index of the memory heap type
    ///described by the information returned in [`memory_requirements`].
    pub memory_bind_index: u32,
    ///[`memory_requirements`] is a pointer to a [`MemoryRequirements2`]
    ///structure in which the requested memory heap requirements for the heap
    ///with index [`memory_bind_index`] are returned.
    pub memory_requirements: *mut MemoryRequirements2<'lt>,
}
impl<'lt> Default for VideoGetMemoryPropertiesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_GET_MEMORY_PROPERTIES_KHR,
            p_next: std::ptr::null(),
            memory_bind_index: 0,
            memory_requirements: std::ptr::null_mut(),
        }
    }
}
impl<'lt> VideoGetMemoryPropertiesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::memory_requirements`]
    pub fn memory_requirements_raw(&self) -> *mut MemoryRequirements2<'lt> {
        self.memory_requirements
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::memory_requirements`]
    pub fn set_memory_requirements_raw(&mut self, value: *mut MemoryRequirements2<'lt>) -> &mut Self {
        self.memory_requirements = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::memory_requirements`]
    pub fn with_memory_requirements_raw(mut self, value: *mut MemoryRequirements2<'lt>) -> Self {
        self.memory_requirements = value;
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
    ///Gets the value of [`Self::memory_bind_index`]
    pub fn memory_bind_index(&self) -> u32 {
        self.memory_bind_index
    }
    ///Gets the value of [`Self::memory_requirements`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn memory_requirements(&self) -> &MemoryRequirements2<'lt> {
        &*self.memory_requirements
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::memory_bind_index`]
    pub fn memory_bind_index_mut(&mut self) -> &mut u32 {
        &mut self.memory_bind_index
    }
    ///Gets a mutable reference to the value of [`Self::memory_requirements`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn memory_requirements_mut(&mut self) -> &mut MemoryRequirements2<'lt> {
        &mut *self.memory_requirements
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::memory_bind_index`]
    pub fn set_memory_bind_index(&mut self, value: u32) -> &mut Self {
        self.memory_bind_index = value;
        self
    }
    ///Sets the value of [`Self::memory_requirements`]
    pub fn set_memory_requirements(&mut self, value: &'lt mut crate::vulkan1_1::MemoryRequirements2<'lt>) -> &mut Self {
        self.memory_requirements = value as *mut _;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::memory_bind_index`]
    pub fn with_memory_bind_index(mut self, value: u32) -> Self {
        self.memory_bind_index = value;
        self
    }
    ///Sets the value of [`Self::memory_requirements`]
    pub fn with_memory_requirements(mut self, value: &'lt mut crate::vulkan1_1::MemoryRequirements2<'lt>) -> Self {
        self.memory_requirements = value as *mut _;
        self
    }
}
///[VkVideoBindMemoryKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoBindMemoryKHR.html) - Structure specifying device memory heap entry for video session object
///# C Specifications
///The [`VideoBindMemoryKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_video_queue
///typedef struct VkVideoBindMemoryKHR {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           memoryBindIndex;
///    VkDeviceMemory     memory;
///    VkDeviceSize       memoryOffset;
///    VkDeviceSize       memorySize;
///} VkVideoBindMemoryKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`memory_bind_index`] is the index of the device memory heap returned in
///   [`VideoGetMemoryPropertiesKHR`]::[`memory_bind_index`] from
///   [`get_video_session_memory_requirements_khr`].
/// - [`memory`] is the allocated device memory to be bound to the video session’s heap with index
///   [`memory_bind_index`].
/// - [`memory_offset`] is the start offset of the region of [`memory`] which is to be bound.
/// - [`memory_size`] is the size in bytes of the region of [`memory`], starting from
///   [`memory_offset`] bytes, to be bound.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_BIND_MEMORY_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`memory`] **must**  be a valid [`DeviceMemory`] handle
///# Related
/// - [`khr_video_queue`]
/// - [`DeviceMemory`]
/// - [`DeviceSize`]
/// - [`StructureType`]
/// - [`bind_video_session_memory_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoBindMemoryKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct VideoBindMemoryKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`memory_bind_index`] is the index of the device memory heap returned in
    ///[`VideoGetMemoryPropertiesKHR`]::[`memory_bind_index`] from
    ///[`get_video_session_memory_requirements_khr`].
    pub memory_bind_index: u32,
    ///[`memory`] is the allocated device memory to be bound to the video
    ///session’s heap with index [`memory_bind_index`].
    pub memory: DeviceMemory,
    ///[`memory_offset`] is the start offset of the region of [`memory`]
    ///which is to be bound.
    pub memory_offset: DeviceSize,
    ///[`memory_size`] is the size in bytes of the region of [`memory`],
    ///starting from [`memory_offset`] bytes, to be bound.
    pub memory_size: DeviceSize,
}
impl<'lt> Default for VideoBindMemoryKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_BIND_MEMORY_KHR,
            p_next: std::ptr::null(),
            memory_bind_index: 0,
            memory: Default::default(),
            memory_offset: Default::default(),
            memory_size: Default::default(),
        }
    }
}
impl<'lt> VideoBindMemoryKHR<'lt> {
    ///Creates a static version of this structure
    pub fn make_static(mut self) -> VideoBindMemoryKHR<'static> {
        unsafe {
            self.p_next = std::ptr::null_mut() as _;
            std::mem::transmute(self)
        }
    }
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::memory_bind_index`]
    pub fn memory_bind_index(&self) -> u32 {
        self.memory_bind_index
    }
    ///Gets the value of [`Self::memory`]
    pub fn memory(&self) -> DeviceMemory {
        self.memory
    }
    ///Gets the value of [`Self::memory_offset`]
    pub fn memory_offset(&self) -> DeviceSize {
        self.memory_offset
    }
    ///Gets the value of [`Self::memory_size`]
    pub fn memory_size(&self) -> DeviceSize {
        self.memory_size
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::memory_bind_index`]
    pub fn memory_bind_index_mut(&mut self) -> &mut u32 {
        &mut self.memory_bind_index
    }
    ///Gets a mutable reference to the value of [`Self::memory`]
    pub fn memory_mut(&mut self) -> &mut DeviceMemory {
        &mut self.memory
    }
    ///Gets a mutable reference to the value of [`Self::memory_offset`]
    pub fn memory_offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.memory_offset
    }
    ///Gets a mutable reference to the value of [`Self::memory_size`]
    pub fn memory_size_mut(&mut self) -> &mut DeviceSize {
        &mut self.memory_size
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::memory_bind_index`]
    pub fn set_memory_bind_index(&mut self, value: u32) -> &mut Self {
        self.memory_bind_index = value;
        self
    }
    ///Sets the value of [`Self::memory`]
    pub fn set_memory(&mut self, value: crate::vulkan1_0::DeviceMemory) -> &mut Self {
        self.memory = value;
        self
    }
    ///Sets the value of [`Self::memory_offset`]
    pub fn set_memory_offset(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.memory_offset = value;
        self
    }
    ///Sets the value of [`Self::memory_size`]
    pub fn set_memory_size(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.memory_size = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::memory_bind_index`]
    pub fn with_memory_bind_index(mut self, value: u32) -> Self {
        self.memory_bind_index = value;
        self
    }
    ///Sets the value of [`Self::memory`]
    pub fn with_memory(mut self, value: crate::vulkan1_0::DeviceMemory) -> Self {
        self.memory = value;
        self
    }
    ///Sets the value of [`Self::memory_offset`]
    pub fn with_memory_offset(mut self, value: crate::vulkan1_0::DeviceSize) -> Self {
        self.memory_offset = value;
        self
    }
    ///Sets the value of [`Self::memory_size`]
    pub fn with_memory_size(mut self, value: crate::vulkan1_0::DeviceSize) -> Self {
        self.memory_size = value;
        self
    }
}
///[VkVideoPictureResourceKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoPictureResourceKHR.html) - Structure specifying the picture resources
///# C Specifications
///The [`VideoPictureResourceKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_video_queue
///typedef struct VkVideoPictureResourceKHR {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkOffset2D         codedOffset;
///    VkExtent2D         codedExtent;
///    uint32_t           baseArrayLayer;
///    VkImageView        imageViewBinding;
///} VkVideoPictureResourceKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`coded_offset`] is the offset to be used for the picture resource.
/// - [`coded_extent`] is the extent to be used for the picture resource.
/// - [`base_array_layer`] is the first array layer to be accessed for the Decode or Encode
///   Operations.
/// - [`image_view_binding`] is a [`ImageView`] image view representing this picture resource.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_PICTURE_RESOURCE_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`image_view_binding`] **must**  be a valid [`ImageView`] handle
///# Related
/// - [`khr_video_queue`]
/// - [`Extent2D`]
/// - [`ImageView`]
/// - [`Offset2D`]
/// - [`StructureType`]
/// - [`VideoDecodeInfoKHR`]
/// - [`VideoEncodeInfoKHR`]
/// - [`VideoReferenceSlotKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoPictureResourceKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct VideoPictureResourceKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`coded_offset`] is the offset to be used for the picture resource.
    pub coded_offset: Offset2D,
    ///[`coded_extent`] is the extent to be used for the picture resource.
    pub coded_extent: Extent2D,
    ///[`base_array_layer`] is the first array layer to be accessed for the
    ///Decode or Encode Operations.
    pub base_array_layer: u32,
    ///[`image_view_binding`] is a [`ImageView`] image view representing
    ///this picture resource.
    pub image_view_binding: ImageView,
}
impl<'lt> Default for VideoPictureResourceKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_PICTURE_RESOURCE_KHR,
            p_next: std::ptr::null(),
            coded_offset: Default::default(),
            coded_extent: Default::default(),
            base_array_layer: 0,
            image_view_binding: Default::default(),
        }
    }
}
impl<'lt> VideoPictureResourceKHR<'lt> {
    ///Creates a static version of this structure
    pub fn make_static(mut self) -> VideoPictureResourceKHR<'static> {
        unsafe {
            self.p_next = std::ptr::null_mut() as _;
            std::mem::transmute(self)
        }
    }
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::coded_offset`]
    pub fn coded_offset(&self) -> Offset2D {
        self.coded_offset
    }
    ///Gets the value of [`Self::coded_extent`]
    pub fn coded_extent(&self) -> Extent2D {
        self.coded_extent
    }
    ///Gets the value of [`Self::base_array_layer`]
    pub fn base_array_layer(&self) -> u32 {
        self.base_array_layer
    }
    ///Gets the value of [`Self::image_view_binding`]
    pub fn image_view_binding(&self) -> ImageView {
        self.image_view_binding
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::coded_offset`]
    pub fn coded_offset_mut(&mut self) -> &mut Offset2D {
        &mut self.coded_offset
    }
    ///Gets a mutable reference to the value of [`Self::coded_extent`]
    pub fn coded_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.coded_extent
    }
    ///Gets a mutable reference to the value of [`Self::base_array_layer`]
    pub fn base_array_layer_mut(&mut self) -> &mut u32 {
        &mut self.base_array_layer
    }
    ///Gets a mutable reference to the value of [`Self::image_view_binding`]
    pub fn image_view_binding_mut(&mut self) -> &mut ImageView {
        &mut self.image_view_binding
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::coded_offset`]
    pub fn set_coded_offset(&mut self, value: crate::vulkan1_0::Offset2D) -> &mut Self {
        self.coded_offset = value;
        self
    }
    ///Sets the value of [`Self::coded_extent`]
    pub fn set_coded_extent(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.coded_extent = value;
        self
    }
    ///Sets the value of [`Self::base_array_layer`]
    pub fn set_base_array_layer(&mut self, value: u32) -> &mut Self {
        self.base_array_layer = value;
        self
    }
    ///Sets the value of [`Self::image_view_binding`]
    pub fn set_image_view_binding(&mut self, value: crate::vulkan1_0::ImageView) -> &mut Self {
        self.image_view_binding = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::coded_offset`]
    pub fn with_coded_offset(mut self, value: crate::vulkan1_0::Offset2D) -> Self {
        self.coded_offset = value;
        self
    }
    ///Sets the value of [`Self::coded_extent`]
    pub fn with_coded_extent(mut self, value: crate::vulkan1_0::Extent2D) -> Self {
        self.coded_extent = value;
        self
    }
    ///Sets the value of [`Self::base_array_layer`]
    pub fn with_base_array_layer(mut self, value: u32) -> Self {
        self.base_array_layer = value;
        self
    }
    ///Sets the value of [`Self::image_view_binding`]
    pub fn with_image_view_binding(mut self, value: crate::vulkan1_0::ImageView) -> Self {
        self.image_view_binding = value;
        self
    }
}
///[VkVideoReferenceSlotKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoReferenceSlotKHR.html) - Structure specifying the reference picture slot
///# C Specifications
///The [`VideoReferenceSlotKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_video_queue
///typedef struct VkVideoReferenceSlotKHR {
///    VkStructureType                     sType;
///    const void*                         pNext;
///    int8_t                              slotIndex;
///    const VkVideoPictureResourceKHR*    pPictureResource;
///} VkVideoReferenceSlotKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`slot_index`] is the unique reference slot index used for the encode or decode operation.
/// - [`picture_resource`] is a pointer to a [`VideoPictureResourceKHR`] structure describing the
///   picture resource bound to this slot index.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_REFERENCE_SLOT_KHR`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**
///   be either `NULL` or a pointer to a valid instance of [`VideoDecodeH264DpbSlotInfoEXT`] or
///   [`VideoDecodeH265DpbSlotInfoEXT`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
/// - [`picture_resource`] **must**  be a valid pointer to a valid [`VideoPictureResourceKHR`]
///   structure
///# Related
/// - [`khr_video_queue`]
/// - [`StructureType`]
/// - [`VideoBeginCodingInfoKHR`]
/// - [`VideoDecodeInfoKHR`]
/// - [`VideoEncodeInfoKHR`]
/// - [`VideoPictureResourceKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoReferenceSlotKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct VideoReferenceSlotKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`slot_index`] is the unique reference slot index used for the encode
    ///or decode operation.
    pub slot_index: i8,
    ///[`picture_resource`] is a pointer to a [`VideoPictureResourceKHR`]
    ///structure describing the picture resource bound to this slot index.
    pub picture_resource: *const VideoPictureResourceKHR<'lt>,
}
impl<'lt> Default for VideoReferenceSlotKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_REFERENCE_SLOT_KHR,
            p_next: std::ptr::null(),
            slot_index: 0,
            picture_resource: std::ptr::null(),
        }
    }
}
impl<'lt> VideoReferenceSlotKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::picture_resource`]
    pub fn picture_resource_raw(&self) -> *const VideoPictureResourceKHR<'lt> {
        self.picture_resource
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::picture_resource`]
    pub fn set_picture_resource_raw(&mut self, value: *const VideoPictureResourceKHR<'lt>) -> &mut Self {
        self.picture_resource = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::picture_resource`]
    pub fn with_picture_resource_raw(mut self, value: *const VideoPictureResourceKHR<'lt>) -> Self {
        self.picture_resource = value;
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
    ///Gets the value of [`Self::picture_resource`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn picture_resource(&self) -> &VideoPictureResourceKHR<'lt> {
        &*self.picture_resource
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
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::slot_index`]
    pub fn set_slot_index(&mut self, value: i8) -> &mut Self {
        self.slot_index = value;
        self
    }
    ///Sets the value of [`Self::picture_resource`]
    pub fn set_picture_resource(
        &mut self,
        value: &'lt crate::extensions::khr_video_queue::VideoPictureResourceKHR<'lt>,
    ) -> &mut Self {
        self.picture_resource = value as *const _;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::slot_index`]
    pub fn with_slot_index(mut self, value: i8) -> Self {
        self.slot_index = value;
        self
    }
    ///Sets the value of [`Self::picture_resource`]
    pub fn with_picture_resource(
        mut self,
        value: &'lt crate::extensions::khr_video_queue::VideoPictureResourceKHR<'lt>,
    ) -> Self {
        self.picture_resource = value as *const _;
        self
    }
}
#[cfg(feature = "VK_EXT_video_decode_h264")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, VideoDecodeH264DpbSlotInfoEXT<'extender>> for VideoReferenceSlotKHR<'this>
{
    type Out = VideoReferenceSlotKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut VideoDecodeH264DpbSlotInfoEXT<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut VideoDecodeH264DpbSlotInfoEXT<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
    #[must_use]
    #[inline]
    fn chain_opt(self, new: Option<&'other mut VideoDecodeH264DpbSlotInfoEXT<'extender>>) -> Self::Out {
        match new {
            Some(new) => self.chain(new),
            None => unsafe { std::mem::transmute(self) },
        }
    }
}
#[cfg(feature = "VK_EXT_video_decode_h265")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, VideoDecodeH265DpbSlotInfoEXT<'extender>> for VideoReferenceSlotKHR<'this>
{
    type Out = VideoReferenceSlotKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut VideoDecodeH265DpbSlotInfoEXT<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut VideoDecodeH265DpbSlotInfoEXT<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
    #[must_use]
    #[inline]
    fn chain_opt(self, new: Option<&'other mut VideoDecodeH265DpbSlotInfoEXT<'extender>>) -> Self::Out {
        match new {
            Some(new) => self.chain(new),
            None => unsafe { std::mem::transmute(self) },
        }
    }
}
///[VkVideoSessionCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoSessionCreateInfoKHR.html) - Structure specifying parameters of a newly created video decode session
///# C Specifications
///The [`VideoSessionCreateInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_video_queue
///typedef struct VkVideoSessionCreateInfoKHR {
///    VkStructureType                 sType;
///    const void*                     pNext;
///    uint32_t                        queueFamilyIndex;
///    VkVideoSessionCreateFlagsKHR    flags;
///    const VkVideoProfileKHR*        pVideoProfile;
///    VkFormat                        pictureFormat;
///    VkExtent2D                      maxCodedExtent;
///    VkFormat                        referencePicturesFormat;
///    uint32_t                        maxReferencePicturesSlotsCount;
///    uint32_t                        maxReferencePicturesActiveCount;
///    const VkExtensionProperties*    pStdHeaderVersion;
///} VkVideoSessionCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`queue_family_index`] is the queue family of the created video session.
/// - [`flags`] is a bitmask of [`VideoSessionCreateFlagBitsKHR`] specifying creation flags.
/// - [`video_profile`] is a pointer to a [`VideoProfileKHR`] structure.
/// - [`picture_format`] is the format of the [image views](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views)
///   representing decoded [Output](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#decoded-output-picture)
///   or encoded [Input](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#input-encode-picture)
///   pictures.
/// - [`max_coded_extent`] is the maximum width and height of the coded pictures that this instance
///   will be able to support.
/// - [`reference_pictures_format`] is the format of the [DPB](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#dpb)
///   image views representing the [Reference Pictures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#reference-picture).
/// - [`max_reference_pictures_slots_count`] is the maximum number of [DPB Slots](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#dpb-slot)
///   that can be activated with associated [Video Picture Resources](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-picture-resources)
///   for the created video session.
/// - [`max_reference_pictures_active_count`] is the maximum number of active [DPB Slots](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#dpb-slot)
///   that can be used as Dpb or Reconstructed [Reference Pictures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#reference-picture)
///   within a single decode or encode operation for the created video session.
/// - [`std_header_version`] is a pointer to a [`ExtensionProperties`] structure requesting the
///   Video Std header version to use for `codecOperation` in [`video_profile`].
///# Description
///## Valid Usage
/// - [`video_profile`] **must**  be a pointer to a valid [`VideoProfileKHR`] structure whose
///   [`p_next`] chain  **must**  include a valid codec-specific profile structure
/// - If [Reference Pictures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#reference-picture)
///   are required for use with the created video session, the
///   [`max_reference_pictures_slots_count`] **must**  be set to a value bigger than `0`
/// - [`max_reference_pictures_slots_count`] **cannot**  exceed the implementation reported
///   [`VideoCapabilitiesKHR`]::[`max_reference_pictures_slots_count`]
/// - If [Reference Pictures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#reference-picture)
///   are required for use with the created video session, the
///   [`max_reference_pictures_active_count`] **must**  be set to a value bigger than `0`
/// - [`max_reference_pictures_active_count`] **cannot**  exceed the implementation reported
///   [`VideoCapabilitiesKHR`]::[`max_reference_pictures_active_count`]
/// - [`max_reference_pictures_active_count`] **cannot**  exceed the
///   [`max_reference_pictures_slots_count`]
/// - [`max_coded_extent`] **cannot**  be smaller than [`VideoCapabilitiesKHR::min_extent`] and
///   bigger than [`VideoCapabilitiesKHR::max_extent`]
/// - [`reference_pictures_format`] **must**  be one of the supported formats in
///   [`VideoFormatPropertiesKHR`]`format` returned by the
///   [`get_physical_device_video_format_properties_khr`] when the
///   [`PhysicalDeviceVideoFormatInfoKHR`]`imageUsage` contains
///   `VK_IMAGE_USAGE_VIDEO_DECODE_DPB_BIT_KHR` or `VK_IMAGE_USAGE_VIDEO_ENCODE_DPB_BIT_KHR`
///   depending on the session codec operation
/// - [`picture_format`] for decode output  **must**  be one of the supported formats in
///   [`VideoFormatPropertiesKHR`]`format` returned by the
///   [`get_physical_device_video_format_properties_khr`] when the
///   [`PhysicalDeviceVideoFormatInfoKHR`]`imageUsage` contains
///   `VK_IMAGE_USAGE_VIDEO_DECODE_DST_BIT_KHR`
/// - [`picture_format`] targeting encode operations  **must**  be one of the supported formats in
///   [`VideoFormatPropertiesKHR`]`format` returned by the
///   [`get_physical_device_video_format_properties_khr`] when the
///   [`PhysicalDeviceVideoFormatInfoKHR`]`imageUsage` contains
///   `VK_IMAGE_USAGE_VIDEO_ENCODE_SRC_BIT_KHR`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_SESSION_CREATE_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`flags`] **must**  be a valid combination of [`VideoSessionCreateFlagBitsKHR`] values
/// - [`video_profile`] **must**  be a valid pointer to a valid [`VideoProfileKHR`] structure
/// - [`picture_format`] **must**  be a valid [`Format`] value
/// - [`reference_pictures_format`] **must**  be a valid [`Format`] value
/// - [`std_header_version`] **must**  be a valid pointer to a valid [`ExtensionProperties`]
///   structure
///# Related
/// - [`khr_video_queue`]
/// - [`ExtensionProperties`]
/// - [`Extent2D`]
/// - [`Format`]
/// - [`StructureType`]
/// - [`VideoProfileKHR`]
/// - [`VideoSessionCreateFlagsKHR`]
/// - [`create_video_session_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoSessionCreateInfoKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct VideoSessionCreateInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`queue_family_index`] is the queue family of the created video session.
    pub queue_family_index: u32,
    ///[`flags`] is a bitmask of [`VideoSessionCreateFlagBitsKHR`]
    ///specifying creation flags.
    pub flags: VideoSessionCreateFlagsKHR,
    ///[`video_profile`] is a pointer to a [`VideoProfileKHR`] structure.
    pub video_profile: *const VideoProfileKHR<'lt>,
    ///[`picture_format`] is the format of the [image
    ///views](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views) representing decoded [Output](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#decoded-output-picture) or
    ///encoded [Input](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#input-encode-picture) pictures.
    pub picture_format: Format,
    ///[`max_coded_extent`] is the maximum width and height of the coded
    ///pictures that this instance will be able to support.
    pub max_coded_extent: Extent2D,
    ///[`reference_pictures_format`] is the format of the [DPB](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#dpb) image
    ///views representing the [Reference Pictures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#reference-picture).
    pub reference_pictures_format: Format,
    ///[`max_reference_pictures_slots_count`] is the maximum number of
    ///[DPB Slots](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#dpb-slot) that can be activated with associated
    ///[Video Picture Resources](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-picture-resources) for the created
    ///video session.
    pub max_reference_pictures_slots_count: u32,
    ///[`max_reference_pictures_active_count`] is the maximum number of active
    ///[DPB Slots](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#dpb-slot) that can be used as Dpb or Reconstructed
    ///[Reference Pictures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#reference-picture) within a single decode or
    ///encode operation for the created video session.
    pub max_reference_pictures_active_count: u32,
    ///[`std_header_version`] is a pointer to a [`ExtensionProperties`]
    ///structure requesting the Video Std header version to use for
    ///`codecOperation` in [`video_profile`].
    pub std_header_version: *const ExtensionProperties,
}
impl<'lt> Default for VideoSessionCreateInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_SESSION_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            queue_family_index: 0,
            flags: Default::default(),
            video_profile: std::ptr::null(),
            picture_format: Default::default(),
            max_coded_extent: Default::default(),
            reference_pictures_format: Default::default(),
            max_reference_pictures_slots_count: 0,
            max_reference_pictures_active_count: 0,
            std_header_version: std::ptr::null(),
        }
    }
}
impl<'lt> VideoSessionCreateInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::video_profile`]
    pub fn video_profile_raw(&self) -> *const VideoProfileKHR<'lt> {
        self.video_profile
    }
    ///Gets the raw value of [`Self::std_header_version`]
    pub fn std_header_version_raw(&self) -> *const ExtensionProperties {
        self.std_header_version
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::video_profile`]
    pub fn set_video_profile_raw(&mut self, value: *const VideoProfileKHR<'lt>) -> &mut Self {
        self.video_profile = value;
        self
    }
    ///Sets the raw value of [`Self::std_header_version`]
    pub fn set_std_header_version_raw(&mut self, value: *const ExtensionProperties) -> &mut Self {
        self.std_header_version = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::video_profile`]
    pub fn with_video_profile_raw(mut self, value: *const VideoProfileKHR<'lt>) -> Self {
        self.video_profile = value;
        self
    }
    ///Sets the raw value of [`Self::std_header_version`]
    pub fn with_std_header_version_raw(mut self, value: *const ExtensionProperties) -> Self {
        self.std_header_version = value;
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
    ///Gets the value of [`Self::queue_family_index`]
    pub fn queue_family_index(&self) -> u32 {
        self.queue_family_index
    }
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> VideoSessionCreateFlagsKHR {
        self.flags
    }
    ///Gets the value of [`Self::video_profile`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn video_profile(&self) -> &VideoProfileKHR<'lt> {
        &*self.video_profile
    }
    ///Gets the value of [`Self::picture_format`]
    pub fn picture_format(&self) -> Format {
        self.picture_format
    }
    ///Gets the value of [`Self::max_coded_extent`]
    pub fn max_coded_extent(&self) -> Extent2D {
        self.max_coded_extent
    }
    ///Gets the value of [`Self::reference_pictures_format`]
    pub fn reference_pictures_format(&self) -> Format {
        self.reference_pictures_format
    }
    ///Gets the value of [`Self::max_reference_pictures_slots_count`]
    pub fn max_reference_pictures_slots_count(&self) -> u32 {
        self.max_reference_pictures_slots_count
    }
    ///Gets the value of [`Self::max_reference_pictures_active_count`]
    pub fn max_reference_pictures_active_count(&self) -> u32 {
        self.max_reference_pictures_active_count
    }
    ///Gets the value of [`Self::std_header_version`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn std_header_version(&self) -> &ExtensionProperties {
        &*self.std_header_version
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::queue_family_index`]
    pub fn queue_family_index_mut(&mut self) -> &mut u32 {
        &mut self.queue_family_index
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut VideoSessionCreateFlagsKHR {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::picture_format`]
    pub fn picture_format_mut(&mut self) -> &mut Format {
        &mut self.picture_format
    }
    ///Gets a mutable reference to the value of [`Self::max_coded_extent`]
    pub fn max_coded_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.max_coded_extent
    }
    ///Gets a mutable reference to the value of [`Self::reference_pictures_format`]
    pub fn reference_pictures_format_mut(&mut self) -> &mut Format {
        &mut self.reference_pictures_format
    }
    ///Gets a mutable reference to the value of [`Self::max_reference_pictures_slots_count`]
    pub fn max_reference_pictures_slots_count_mut(&mut self) -> &mut u32 {
        &mut self.max_reference_pictures_slots_count
    }
    ///Gets a mutable reference to the value of [`Self::max_reference_pictures_active_count`]
    pub fn max_reference_pictures_active_count_mut(&mut self) -> &mut u32 {
        &mut self.max_reference_pictures_active_count
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::queue_family_index`]
    pub fn set_queue_family_index(&mut self, value: u32) -> &mut Self {
        self.queue_family_index = value;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn set_flags(&mut self, value: crate::extensions::khr_video_queue::VideoSessionCreateFlagsKHR) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::video_profile`]
    pub fn set_video_profile(
        &mut self,
        value: &'lt crate::extensions::khr_video_queue::VideoProfileKHR<'lt>,
    ) -> &mut Self {
        self.video_profile = value as *const _;
        self
    }
    ///Sets the value of [`Self::picture_format`]
    pub fn set_picture_format(&mut self, value: crate::vulkan1_0::Format) -> &mut Self {
        self.picture_format = value;
        self
    }
    ///Sets the value of [`Self::max_coded_extent`]
    pub fn set_max_coded_extent(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.max_coded_extent = value;
        self
    }
    ///Sets the value of [`Self::reference_pictures_format`]
    pub fn set_reference_pictures_format(&mut self, value: crate::vulkan1_0::Format) -> &mut Self {
        self.reference_pictures_format = value;
        self
    }
    ///Sets the value of [`Self::max_reference_pictures_slots_count`]
    pub fn set_max_reference_pictures_slots_count(&mut self, value: u32) -> &mut Self {
        self.max_reference_pictures_slots_count = value;
        self
    }
    ///Sets the value of [`Self::max_reference_pictures_active_count`]
    pub fn set_max_reference_pictures_active_count(&mut self, value: u32) -> &mut Self {
        self.max_reference_pictures_active_count = value;
        self
    }
    ///Sets the value of [`Self::std_header_version`]
    pub fn set_std_header_version(&mut self, value: &'lt crate::vulkan1_0::ExtensionProperties) -> &mut Self {
        self.std_header_version = value as *const _;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::queue_family_index`]
    pub fn with_queue_family_index(mut self, value: u32) -> Self {
        self.queue_family_index = value;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn with_flags(mut self, value: crate::extensions::khr_video_queue::VideoSessionCreateFlagsKHR) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::video_profile`]
    pub fn with_video_profile(mut self, value: &'lt crate::extensions::khr_video_queue::VideoProfileKHR<'lt>) -> Self {
        self.video_profile = value as *const _;
        self
    }
    ///Sets the value of [`Self::picture_format`]
    pub fn with_picture_format(mut self, value: crate::vulkan1_0::Format) -> Self {
        self.picture_format = value;
        self
    }
    ///Sets the value of [`Self::max_coded_extent`]
    pub fn with_max_coded_extent(mut self, value: crate::vulkan1_0::Extent2D) -> Self {
        self.max_coded_extent = value;
        self
    }
    ///Sets the value of [`Self::reference_pictures_format`]
    pub fn with_reference_pictures_format(mut self, value: crate::vulkan1_0::Format) -> Self {
        self.reference_pictures_format = value;
        self
    }
    ///Sets the value of [`Self::max_reference_pictures_slots_count`]
    pub fn with_max_reference_pictures_slots_count(mut self, value: u32) -> Self {
        self.max_reference_pictures_slots_count = value;
        self
    }
    ///Sets the value of [`Self::max_reference_pictures_active_count`]
    pub fn with_max_reference_pictures_active_count(mut self, value: u32) -> Self {
        self.max_reference_pictures_active_count = value;
        self
    }
    ///Sets the value of [`Self::std_header_version`]
    pub fn with_std_header_version(mut self, value: &'lt crate::vulkan1_0::ExtensionProperties) -> Self {
        self.std_header_version = value as *const _;
        self
    }
}
///[VkVideoSessionParametersCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoSessionParametersCreateInfoKHR.html) - Structure to set video session parameters
///# C Specifications
///The [`VideoSessionParametersCreateInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_video_queue
///typedef struct VkVideoSessionParametersCreateInfoKHR {
///    VkStructureType                sType;
///    const void*                    pNext;
///    VkVideoSessionParametersKHR    videoSessionParametersTemplate;
///    VkVideoSessionKHR              videoSession;
///} VkVideoSessionParametersCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`video_session_parameters_template`] is [`crate::Handle::null`] or a valid handle to a
///   [`VideoSessionParametersKHR`] object. If this parameter represents a valid handle, then the
///   underlying Video Session Parameters object will be used as a template for constructing the new
///   video session parameters object. All of the template object’s current parameters will be
///   inherited by the new object in such a case. Optionally, some of the template’s parameters can
///   be updated or new parameters added to the newly constructed object via the extension-specific
///   parameters.
/// - [`video_session`] is the video session object against which the video session parameters
///   object is going to be created.
///# Description
///## Valid Usage
/// - If [`video_session_parameters_template`] represents a valid handle, it  **must**  have been
///   created against [`video_session`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**
///   be either `NULL` or a pointer to a valid instance of
///   [`VideoDecodeH264SessionParametersCreateInfoEXT`],
///   [`VideoDecodeH265SessionParametersCreateInfoEXT`],
///   [`VideoEncodeH264SessionParametersCreateInfoEXT`], or
///   [`VideoEncodeH265SessionParametersCreateInfoEXT`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
/// - If [`video_session_parameters_template`] is not [`crate::Handle::null`],
///   [`video_session_parameters_template`] **must**  be a valid [`VideoSessionParametersKHR`]
///   handle
/// - [`video_session`] **must**  be a valid [`VideoSessionKHR`] handle
/// - If [`video_session_parameters_template`] is a valid handle, it  **must**  have been created,
///   allocated, or retrieved from [`video_session`]
/// - Both of [`video_session`], and [`video_session_parameters_template`] that are valid handles of
///   non-ignored parameters  **must**  have been created, allocated, or retrieved from the same
///   [`Device`]
///# Related
/// - [`khr_video_queue`]
/// - [`StructureType`]
/// - [`VideoSessionKHR`]
/// - [`VideoSessionParametersKHR`]
/// - [`create_video_session_parameters_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoSessionParametersCreateInfoKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct VideoSessionParametersCreateInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`video_session_parameters_template`] is [`crate::Handle::null`] or a valid
    ///handle to a [`VideoSessionParametersKHR`] object.
    ///If this parameter represents a valid handle, then the underlying Video
    ///Session Parameters object will be used as a template for constructing
    ///the new video session parameters object.
    ///All of the template object’s current parameters will be inherited by the
    ///new object in such a case.
    ///Optionally, some of the template’s parameters can be updated or new
    ///parameters added to the newly constructed object via the
    ///extension-specific parameters.
    pub video_session_parameters_template: VideoSessionParametersKHR,
    ///[`video_session`] is the video session object against which the video
    ///session parameters object is going to be created.
    pub video_session: VideoSessionKHR,
}
impl<'lt> Default for VideoSessionParametersCreateInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            video_session_parameters_template: Default::default(),
            video_session: Default::default(),
        }
    }
}
impl<'lt> VideoSessionParametersCreateInfoKHR<'lt> {
    ///Creates a static version of this structure
    pub fn make_static(mut self) -> VideoSessionParametersCreateInfoKHR<'static> {
        unsafe {
            self.p_next = std::ptr::null_mut() as _;
            std::mem::transmute(self)
        }
    }
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::video_session_parameters_template`]
    pub fn video_session_parameters_template(&self) -> VideoSessionParametersKHR {
        self.video_session_parameters_template
    }
    ///Gets the value of [`Self::video_session`]
    pub fn video_session(&self) -> VideoSessionKHR {
        self.video_session
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::video_session_parameters_template`]
    pub fn video_session_parameters_template_mut(&mut self) -> &mut VideoSessionParametersKHR {
        &mut self.video_session_parameters_template
    }
    ///Gets a mutable reference to the value of [`Self::video_session`]
    pub fn video_session_mut(&mut self) -> &mut VideoSessionKHR {
        &mut self.video_session
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::video_session_parameters_template`]
    pub fn set_video_session_parameters_template(
        &mut self,
        value: crate::extensions::khr_video_queue::VideoSessionParametersKHR,
    ) -> &mut Self {
        self.video_session_parameters_template = value;
        self
    }
    ///Sets the value of [`Self::video_session`]
    pub fn set_video_session(&mut self, value: crate::extensions::khr_video_queue::VideoSessionKHR) -> &mut Self {
        self.video_session = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::video_session_parameters_template`]
    pub fn with_video_session_parameters_template(
        mut self,
        value: crate::extensions::khr_video_queue::VideoSessionParametersKHR,
    ) -> Self {
        self.video_session_parameters_template = value;
        self
    }
    ///Sets the value of [`Self::video_session`]
    pub fn with_video_session(mut self, value: crate::extensions::khr_video_queue::VideoSessionKHR) -> Self {
        self.video_session = value;
        self
    }
}
#[cfg(feature = "VK_EXT_video_decode_h264")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, VideoDecodeH264SessionParametersCreateInfoEXT<'extender>>
    for VideoSessionParametersCreateInfoKHR<'this>
{
    type Out = VideoSessionParametersCreateInfoKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut VideoDecodeH264SessionParametersCreateInfoEXT<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut VideoDecodeH264SessionParametersCreateInfoEXT<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
    #[must_use]
    #[inline]
    fn chain_opt(self, new: Option<&'other mut VideoDecodeH264SessionParametersCreateInfoEXT<'extender>>) -> Self::Out {
        match new {
            Some(new) => self.chain(new),
            None => unsafe { std::mem::transmute(self) },
        }
    }
}
#[cfg(feature = "VK_EXT_video_decode_h265")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, VideoDecodeH265SessionParametersCreateInfoEXT<'extender>>
    for VideoSessionParametersCreateInfoKHR<'this>
{
    type Out = VideoSessionParametersCreateInfoKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut VideoDecodeH265SessionParametersCreateInfoEXT<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut VideoDecodeH265SessionParametersCreateInfoEXT<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
    #[must_use]
    #[inline]
    fn chain_opt(self, new: Option<&'other mut VideoDecodeH265SessionParametersCreateInfoEXT<'extender>>) -> Self::Out {
        match new {
            Some(new) => self.chain(new),
            None => unsafe { std::mem::transmute(self) },
        }
    }
}
#[cfg(feature = "VK_EXT_video_encode_h264")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, VideoEncodeH264SessionParametersCreateInfoEXT<'extender>>
    for VideoSessionParametersCreateInfoKHR<'this>
{
    type Out = VideoSessionParametersCreateInfoKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut VideoEncodeH264SessionParametersCreateInfoEXT<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut VideoEncodeH264SessionParametersCreateInfoEXT<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
    #[must_use]
    #[inline]
    fn chain_opt(self, new: Option<&'other mut VideoEncodeH264SessionParametersCreateInfoEXT<'extender>>) -> Self::Out {
        match new {
            Some(new) => self.chain(new),
            None => unsafe { std::mem::transmute(self) },
        }
    }
}
#[cfg(feature = "VK_EXT_video_encode_h265")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, VideoEncodeH265SessionParametersCreateInfoEXT<'extender>>
    for VideoSessionParametersCreateInfoKHR<'this>
{
    type Out = VideoSessionParametersCreateInfoKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut VideoEncodeH265SessionParametersCreateInfoEXT<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut VideoEncodeH265SessionParametersCreateInfoEXT<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
    #[must_use]
    #[inline]
    fn chain_opt(self, new: Option<&'other mut VideoEncodeH265SessionParametersCreateInfoEXT<'extender>>) -> Self::Out {
        match new {
            Some(new) => self.chain(new),
            None => unsafe { std::mem::transmute(self) },
        }
    }
}
///[VkVideoSessionParametersUpdateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoSessionParametersUpdateInfoKHR.html) - Structure to update video session parameters
///# C Specifications
///The [`VideoSessionParametersUpdateInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_video_queue
///typedef struct VkVideoSessionParametersUpdateInfoKHR {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           updateSequenceCount;
///} VkVideoSessionParametersUpdateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`update_sequence_count`] is the sequence number of the object update with parameters,
///   starting from `1` and incrementing the value by one with each subsequent update.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**
///   be either `NULL` or a pointer to a valid instance of
///   [`VideoDecodeH264SessionParametersAddInfoEXT`],
///   [`VideoDecodeH265SessionParametersAddInfoEXT`],
///   [`VideoEncodeH264SessionParametersAddInfoEXT`], or
///   [`VideoEncodeH265SessionParametersAddInfoEXT`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
///# Related
/// - [`khr_video_queue`]
/// - [`StructureType`]
/// - [`update_video_session_parameters_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoSessionParametersUpdateInfoKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct VideoSessionParametersUpdateInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`update_sequence_count`] is the sequence number of the object update
    ///with parameters, starting from `1` and incrementing the value by one
    ///with each subsequent update.
    pub update_sequence_count: u32,
}
impl<'lt> Default for VideoSessionParametersUpdateInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR,
            p_next: std::ptr::null(),
            update_sequence_count: 0,
        }
    }
}
impl<'lt> VideoSessionParametersUpdateInfoKHR<'lt> {
    ///Creates a static version of this structure
    pub fn make_static(mut self) -> VideoSessionParametersUpdateInfoKHR<'static> {
        unsafe {
            self.p_next = std::ptr::null_mut() as _;
            std::mem::transmute(self)
        }
    }
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::update_sequence_count`]
    pub fn update_sequence_count(&self) -> u32 {
        self.update_sequence_count
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::update_sequence_count`]
    pub fn update_sequence_count_mut(&mut self) -> &mut u32 {
        &mut self.update_sequence_count
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::update_sequence_count`]
    pub fn set_update_sequence_count(&mut self, value: u32) -> &mut Self {
        self.update_sequence_count = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::update_sequence_count`]
    pub fn with_update_sequence_count(mut self, value: u32) -> Self {
        self.update_sequence_count = value;
        self
    }
}
#[cfg(feature = "VK_EXT_video_decode_h264")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, VideoDecodeH264SessionParametersAddInfoEXT<'extender>>
    for VideoSessionParametersUpdateInfoKHR<'this>
{
    type Out = VideoSessionParametersUpdateInfoKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut VideoDecodeH264SessionParametersAddInfoEXT<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut VideoDecodeH264SessionParametersAddInfoEXT<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
    #[must_use]
    #[inline]
    fn chain_opt(self, new: Option<&'other mut VideoDecodeH264SessionParametersAddInfoEXT<'extender>>) -> Self::Out {
        match new {
            Some(new) => self.chain(new),
            None => unsafe { std::mem::transmute(self) },
        }
    }
}
#[cfg(feature = "VK_EXT_video_decode_h265")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, VideoDecodeH265SessionParametersAddInfoEXT<'extender>>
    for VideoSessionParametersUpdateInfoKHR<'this>
{
    type Out = VideoSessionParametersUpdateInfoKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut VideoDecodeH265SessionParametersAddInfoEXT<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut VideoDecodeH265SessionParametersAddInfoEXT<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
    #[must_use]
    #[inline]
    fn chain_opt(self, new: Option<&'other mut VideoDecodeH265SessionParametersAddInfoEXT<'extender>>) -> Self::Out {
        match new {
            Some(new) => self.chain(new),
            None => unsafe { std::mem::transmute(self) },
        }
    }
}
#[cfg(feature = "VK_EXT_video_encode_h264")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, VideoEncodeH264SessionParametersAddInfoEXT<'extender>>
    for VideoSessionParametersUpdateInfoKHR<'this>
{
    type Out = VideoSessionParametersUpdateInfoKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut VideoEncodeH264SessionParametersAddInfoEXT<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut VideoEncodeH264SessionParametersAddInfoEXT<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
    #[must_use]
    #[inline]
    fn chain_opt(self, new: Option<&'other mut VideoEncodeH264SessionParametersAddInfoEXT<'extender>>) -> Self::Out {
        match new {
            Some(new) => self.chain(new),
            None => unsafe { std::mem::transmute(self) },
        }
    }
}
#[cfg(feature = "VK_EXT_video_encode_h265")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, VideoEncodeH265SessionParametersAddInfoEXT<'extender>>
    for VideoSessionParametersUpdateInfoKHR<'this>
{
    type Out = VideoSessionParametersUpdateInfoKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut VideoEncodeH265SessionParametersAddInfoEXT<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut VideoEncodeH265SessionParametersAddInfoEXT<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
    #[must_use]
    #[inline]
    fn chain_opt(self, new: Option<&'other mut VideoEncodeH265SessionParametersAddInfoEXT<'extender>>) -> Self::Out {
        match new {
            Some(new) => self.chain(new),
            None => unsafe { std::mem::transmute(self) },
        }
    }
}
///[VkVideoBeginCodingInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoBeginCodingInfoKHR.html) - Structure specifying parameters of decode starts
///# C Specifications
///The [`VideoBeginCodingInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_video_queue
///typedef struct VkVideoBeginCodingInfoKHR {
///    VkStructureType                       sType;
///    const void*                           pNext;
///    VkVideoBeginCodingFlagsKHR            flags;
///    VkVideoCodingQualityPresetFlagsKHR    codecQualityPreset;
///    VkVideoSessionKHR                     videoSession;
///    VkVideoSessionParametersKHR           videoSessionParameters;
///    uint32_t                              referenceSlotCount;
///    const VkVideoReferenceSlotKHR*        pReferenceSlots;
///} VkVideoBeginCodingInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`codec_quality_preset`] is a bitmask of [`VideoCodingQualityPresetFlagBitsKHR`] specifying
///   the Video Decode or Encode quality preset.
/// - [`video_session`] is the video session object to be bound for the processing of the video
///   commands.
/// - [`video_session_parameters`] is [`crate::Handle::null`] or a handle of a
///   [`VideoSessionParametersKHR`] object to be used for the processing of the video commands. If
///   [`crate::Handle::null`], then no video session parameters apply to this command buffer
///   context.
/// - [`reference_slot_count`] is the number of reference slot entries provided in
///   [`reference_slots`].
/// - [`reference_slots`] is a pointer to an array of [`VideoReferenceSlotKHR`] structures
///   specifying reference slots, used within the video command context between this
///   [`cmd_begin_video_coding_khr`] command and the [`cmd_end_video_coding_khr`] commmand that
///   follows. Each reference slot provides a slot index and the [`VideoPictureResourceKHR`]
///   specifying the reference picture resource bound to this slot index. A slot index  **must**
///   not appear more than once in [`reference_slots`] in a given command.
///# Description
///## Valid Usage
/// - [`VideoBeginCodingInfoKHR`]::[`reference_slot_count`] **must**  not exceed the value specified
///   in [`VideoSessionCreateInfoKHR::max_reference_pictures_slots_count`] when creating the video
///   session object that is being provided in [`video_session`]
/// - If [`video_session_parameters`] is not [`crate::Handle::null`], it  **must**  have been
///   created using [`video_session`] as a parent object
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_BEGIN_CODING_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`flags`] **must**  be `0`
/// - [`codec_quality_preset`] **must**  be a valid combination of
///   [`VideoCodingQualityPresetFlagBitsKHR`] values
/// - [`codec_quality_preset`] **must**  not be `0`
/// - [`video_session`] **must**  be a valid [`VideoSessionKHR`] handle
/// - If [`video_session_parameters`] is not [`crate::Handle::null`], [`video_session_parameters`]
///   **must**  be a valid [`VideoSessionParametersKHR`] handle
/// - If [`reference_slot_count`] is not `0`, [`reference_slots`] **must**  be a valid pointer to an
///   array of [`reference_slot_count`] valid [`VideoReferenceSlotKHR`] structures
/// - If [`video_session_parameters`] is a valid handle, it  **must**  have been created, allocated,
///   or retrieved from [`video_session`]
/// - Both of [`video_session`], and [`video_session_parameters`] that are valid handles of
///   non-ignored parameters  **must**  have been created, allocated, or retrieved from the same
///   [`Device`]
///# Related
/// - [`khr_video_queue`]
/// - [`StructureType`]
/// - [`VideoBeginCodingFlagsKHR`]
/// - [`VideoCodingQualityPresetFlagsKHR`]
/// - [`VideoReferenceSlotKHR`]
/// - [`VideoSessionKHR`]
/// - [`VideoSessionParametersKHR`]
/// - [`cmd_begin_video_coding_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoBeginCodingInfoKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct VideoBeginCodingInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    pub flags: VideoBeginCodingFlagsKHR,
    ///[`codec_quality_preset`] is a bitmask of
    ///[`VideoCodingQualityPresetFlagBitsKHR`] specifying the Video Decode
    ///or Encode quality preset.
    pub codec_quality_preset: VideoCodingQualityPresetFlagsKHR,
    ///[`video_session`] is the video session object to be bound for the
    ///processing of the video commands.
    pub video_session: VideoSessionKHR,
    ///[`video_session_parameters`] is [`crate::Handle::null`] or a handle of a
    ///[`VideoSessionParametersKHR`] object to be used for the processing
    ///of the video commands.
    ///If [`crate::Handle::null`], then no video session parameters apply to this
    ///command buffer context.
    pub video_session_parameters: VideoSessionParametersKHR,
    ///[`reference_slot_count`] is the number of reference slot entries
    ///provided in [`reference_slots`].
    pub reference_slot_count: u32,
    ///[`reference_slots`] is a pointer to an array of
    ///[`VideoReferenceSlotKHR`] structures specifying reference slots,
    ///used within the video command context between this
    ///[`cmd_begin_video_coding_khr`] command and the
    ///[`cmd_end_video_coding_khr`] commmand that follows.
    ///Each reference slot provides a slot index and the
    ///[`VideoPictureResourceKHR`] specifying the reference picture
    ///resource bound to this slot index.
    ///A slot index  **must**  not appear more than once in [`reference_slots`] in
    ///a given command.
    pub reference_slots: *const VideoReferenceSlotKHR<'lt>,
}
impl<'lt> Default for VideoBeginCodingInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_BEGIN_CODING_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            codec_quality_preset: Default::default(),
            video_session: Default::default(),
            video_session_parameters: Default::default(),
            reference_slot_count: 0,
            reference_slots: std::ptr::null(),
        }
    }
}
impl<'lt> VideoBeginCodingInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::reference_slots`]
    pub fn reference_slots_raw(&self) -> *const VideoReferenceSlotKHR<'lt> {
        self.reference_slots
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::reference_slots`]
    pub fn set_reference_slots_raw(&mut self, value: *const VideoReferenceSlotKHR<'lt>) -> &mut Self {
        self.reference_slots = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::reference_slots`]
    pub fn with_reference_slots_raw(mut self, value: *const VideoReferenceSlotKHR<'lt>) -> Self {
        self.reference_slots = value;
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
    pub fn flags(&self) -> VideoBeginCodingFlagsKHR {
        self.flags
    }
    ///Gets the value of [`Self::codec_quality_preset`]
    pub fn codec_quality_preset(&self) -> VideoCodingQualityPresetFlagsKHR {
        self.codec_quality_preset
    }
    ///Gets the value of [`Self::video_session`]
    pub fn video_session(&self) -> VideoSessionKHR {
        self.video_session
    }
    ///Gets the value of [`Self::video_session_parameters`]
    pub fn video_session_parameters(&self) -> VideoSessionParametersKHR {
        self.video_session_parameters
    }
    ///Gets the value of [`Self::reference_slot_count`]
    pub fn reference_slot_count(&self) -> u32 {
        self.reference_slot_count
    }
    ///Gets the value of [`Self::reference_slots`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn reference_slots(&self) -> &[VideoReferenceSlotKHR<'lt>] {
        std::slice::from_raw_parts(self.reference_slots, self.reference_slot_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut VideoBeginCodingFlagsKHR {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::codec_quality_preset`]
    pub fn codec_quality_preset_mut(&mut self) -> &mut VideoCodingQualityPresetFlagsKHR {
        &mut self.codec_quality_preset
    }
    ///Gets a mutable reference to the value of [`Self::video_session`]
    pub fn video_session_mut(&mut self) -> &mut VideoSessionKHR {
        &mut self.video_session
    }
    ///Gets a mutable reference to the value of [`Self::video_session_parameters`]
    pub fn video_session_parameters_mut(&mut self) -> &mut VideoSessionParametersKHR {
        &mut self.video_session_parameters
    }
    ///Gets a mutable reference to the value of [`Self::reference_slot_count`]
    pub fn reference_slot_count_mut(&mut self) -> &mut u32 {
        &mut self.reference_slot_count
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn set_flags(&mut self, value: crate::extensions::khr_video_queue::VideoBeginCodingFlagsKHR) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::codec_quality_preset`]
    pub fn set_codec_quality_preset(
        &mut self,
        value: crate::extensions::khr_video_queue::VideoCodingQualityPresetFlagsKHR,
    ) -> &mut Self {
        self.codec_quality_preset = value;
        self
    }
    ///Sets the value of [`Self::video_session`]
    pub fn set_video_session(&mut self, value: crate::extensions::khr_video_queue::VideoSessionKHR) -> &mut Self {
        self.video_session = value;
        self
    }
    ///Sets the value of [`Self::video_session_parameters`]
    pub fn set_video_session_parameters(
        &mut self,
        value: crate::extensions::khr_video_queue::VideoSessionParametersKHR,
    ) -> &mut Self {
        self.video_session_parameters = value;
        self
    }
    ///Sets the value of [`Self::reference_slot_count`]
    pub fn set_reference_slot_count(&mut self, value: u32) -> &mut Self {
        self.reference_slot_count = value;
        self
    }
    ///Sets the value of [`Self::reference_slots`]
    pub fn set_reference_slots(
        &mut self,
        value: &'lt [crate::extensions::khr_video_queue::VideoReferenceSlotKHR<'lt>],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.reference_slots = value.as_ptr();
        self.reference_slot_count = len_;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn with_flags(mut self, value: crate::extensions::khr_video_queue::VideoBeginCodingFlagsKHR) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::codec_quality_preset`]
    pub fn with_codec_quality_preset(
        mut self,
        value: crate::extensions::khr_video_queue::VideoCodingQualityPresetFlagsKHR,
    ) -> Self {
        self.codec_quality_preset = value;
        self
    }
    ///Sets the value of [`Self::video_session`]
    pub fn with_video_session(mut self, value: crate::extensions::khr_video_queue::VideoSessionKHR) -> Self {
        self.video_session = value;
        self
    }
    ///Sets the value of [`Self::video_session_parameters`]
    pub fn with_video_session_parameters(
        mut self,
        value: crate::extensions::khr_video_queue::VideoSessionParametersKHR,
    ) -> Self {
        self.video_session_parameters = value;
        self
    }
    ///Sets the value of [`Self::reference_slot_count`]
    pub fn with_reference_slot_count(mut self, value: u32) -> Self {
        self.reference_slot_count = value;
        self
    }
    ///Sets the value of [`Self::reference_slots`]
    pub fn with_reference_slots(
        mut self,
        value: &'lt [crate::extensions::khr_video_queue::VideoReferenceSlotKHR<'lt>],
    ) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.reference_slots = value.as_ptr();
        self.reference_slot_count = len_;
        self
    }
}
///[VkVideoEndCodingInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEndCodingInfoKHR.html) - Structure specifying the end of decode encode commands sequence
///# C Specifications
///The [`VideoEndCodingInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_video_queue
///typedef struct VkVideoEndCodingInfoKHR {
///    VkStructureType             sType;
///    const void*                 pNext;
///    VkVideoEndCodingFlagsKHR    flags;
///} VkVideoEndCodingInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_END_CODING_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`flags`] **must**  be `0`
///# Related
/// - [`khr_video_queue`]
/// - [`StructureType`]
/// - [`VideoEndCodingFlagsKHR`]
/// - [`cmd_end_video_coding_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoEndCodingInfoKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct VideoEndCodingInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    pub flags: VideoEndCodingFlagsKHR,
}
impl<'lt> Default for VideoEndCodingInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_END_CODING_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
        }
    }
}
impl<'lt> VideoEndCodingInfoKHR<'lt> {
    ///Creates a static version of this structure
    pub fn make_static(mut self) -> VideoEndCodingInfoKHR<'static> {
        unsafe {
            self.p_next = std::ptr::null_mut() as _;
            std::mem::transmute(self)
        }
    }
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    pub fn flags(&self) -> VideoEndCodingFlagsKHR {
        self.flags
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut VideoEndCodingFlagsKHR {
        &mut self.flags
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn set_flags(&mut self, value: crate::extensions::khr_video_queue::VideoEndCodingFlagsKHR) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn with_flags(mut self, value: crate::extensions::khr_video_queue::VideoEndCodingFlagsKHR) -> Self {
        self.flags = value;
        self
    }
}
///[VkVideoCodingControlInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoCodingControlInfoKHR.html) - Structure specifying parameters of coding control
///# C Specifications
///The [`VideoCodingControlInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_video_queue
///typedef struct VkVideoCodingControlInfoKHR {
///    VkStructureType                 sType;
///    const void*                     pNext;
///    VkVideoCodingControlFlagsKHR    flags;
///} VkVideoCodingControlInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is a bitmask of [`VideoCodingControlFlagsKHR`] specifying control flags.
///# Description
///## Valid Usage
/// - The first command buffer submitted for a newly created video session  **must**  set the
///   `VK_VIDEO_CODING_CONTROL_RESET_BIT_KHR` bit in [`VideoCodingControlInfoKHR`]::[`flags`] to
///   reset the session device context before any video decode or encode operations are performed on
///   the session.
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_CODING_CONTROL_INFO_KHR`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**
///   be either `NULL` or a pointer to a valid instance of [`VideoEncodeRateControlInfoKHR`] or
///   [`VideoEncodeRateControlLayerInfoKHR`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
/// - [`flags`] **must**  be a valid combination of [`VideoCodingControlFlagBitsKHR`] values
///# Related
/// - [`khr_video_queue`]
/// - [`StructureType`]
/// - [`VideoCodingControlFlagsKHR`]
/// - [`cmd_control_video_coding_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoCodingControlInfoKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct VideoCodingControlInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`VideoCodingControlFlagsKHR`]
    ///specifying control flags.
    pub flags: VideoCodingControlFlagsKHR,
}
impl<'lt> Default for VideoCodingControlInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_CODING_CONTROL_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
        }
    }
}
impl<'lt> VideoCodingControlInfoKHR<'lt> {
    ///Creates a static version of this structure
    pub fn make_static(mut self) -> VideoCodingControlInfoKHR<'static> {
        unsafe {
            self.p_next = std::ptr::null_mut() as _;
            std::mem::transmute(self)
        }
    }
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    pub fn flags(&self) -> VideoCodingControlFlagsKHR {
        self.flags
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut VideoCodingControlFlagsKHR {
        &mut self.flags
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn set_flags(&mut self, value: crate::extensions::khr_video_queue::VideoCodingControlFlagsKHR) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn with_flags(mut self, value: crate::extensions::khr_video_queue::VideoCodingControlFlagsKHR) -> Self {
        self.flags = value;
        self
    }
}
#[cfg(feature = "VK_KHR_video_encode_queue")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, VideoEncodeRateControlInfoKHR<'extender>> for VideoCodingControlInfoKHR<'this>
{
    type Out = VideoCodingControlInfoKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut VideoEncodeRateControlInfoKHR<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut VideoEncodeRateControlInfoKHR<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
    #[must_use]
    #[inline]
    fn chain_opt(self, new: Option<&'other mut VideoEncodeRateControlInfoKHR<'extender>>) -> Self::Out {
        match new {
            Some(new) => self.chain(new),
            None => unsafe { std::mem::transmute(self) },
        }
    }
}
#[cfg(feature = "VK_KHR_video_encode_queue")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, VideoEncodeRateControlLayerInfoKHR<'extender>> for VideoCodingControlInfoKHR<'this>
{
    type Out = VideoCodingControlInfoKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut VideoEncodeRateControlLayerInfoKHR<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut VideoEncodeRateControlLayerInfoKHR<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
    #[must_use]
    #[inline]
    fn chain_opt(self, new: Option<&'other mut VideoEncodeRateControlLayerInfoKHR<'extender>>) -> Self::Out {
        match new {
            Some(new) => self.chain(new),
            None => unsafe { std::mem::transmute(self) },
        }
    }
}
impl PhysicalDevice {
    ///[vkGetPhysicalDeviceVideoCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceVideoCapabilitiesKHR.html) - Query video decode or encode capabilities
    ///# C Specifications
    ///To query video decode or encode capabilities for a specific codec, call:
    ///```c
    ///// Provided by VK_KHR_video_queue
    ///VkResult vkGetPhysicalDeviceVideoCapabilitiesKHR(
    ///    VkPhysicalDevice                            physicalDevice,
    ///    const VkVideoProfileKHR*                    pVideoProfile,
    ///    VkVideoCapabilitiesKHR*                     pCapabilities);
    ///```
    ///# Parameters
    /// - [`physical_device`] is the physical device whose video decode or encode capabilities will
    ///   be queried.
    /// - [`p_video_profile`] is a pointer to a [`VideoProfileKHR`] structure with a chained
    ///   codec-operation specific video profile structure.
    /// - [`p_capabilities`] is a pointer to a [`VideoCapabilitiesKHR`] structure in which the
    ///   capabilities are returned.
    ///# Description
    ///## Valid Usage (Implicit)
    /// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
    /// - [`p_video_profile`] **must**  be a valid pointer to a valid [`VideoProfileKHR`] structure
    /// - [`p_capabilities`] **must**  be a valid pointer to a [`VideoCapabilitiesKHR`] structure
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_EXTENSION_NOT_PRESENT`  - `VK_ERROR_INITIALIZATION_FAILED`  -
    ///   `VK_ERROR_FEATURE_NOT_PRESENT`  - `VK_ERROR_FORMAT_NOT_SUPPORTED`
    ///# Related
    /// - [`khr_video_queue`]
    /// - [`PhysicalDevice`]
    /// - [`VideoCapabilitiesKHR`]
    /// - [`VideoProfileKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetPhysicalDeviceVideoCapabilitiesKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_physical_device_video_capabilities_khr<'lt>(
        self: &Unique<PhysicalDevice>,
        p_video_profile: &VideoProfileKHR<'lt>,
        p_capabilities: Option<VideoCapabilitiesKHR<'lt>>,
    ) -> VulkanResult<VideoCapabilitiesKHR<'static>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .instance()
            .vtable()
            .khr_video_queue()
            .and_then(|vtable| vtable.get_physical_device_video_capabilities_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .instance()
            .vtable()
            .khr_video_queue()
            .and_then(|vtable| vtable.get_physical_device_video_capabilities_khr())
            .unwrap_unchecked();
        let mut p_capabilities = p_capabilities.unwrap_or_default();
        let _return = _function(
            self.as_raw(),
            p_video_profile as *const VideoProfileKHR<'lt>,
            &mut p_capabilities,
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, {
                p_capabilities.p_next = std::ptr::null_mut();
                std::mem::transmute(p_capabilities)
            }),
            e => VulkanResult::Err(e),
        }
    }
}
impl PhysicalDevice {
    ///[vkGetPhysicalDeviceVideoFormatPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceVideoFormatPropertiesKHR.html) - Query supported Video Decode and Encode image formats
    ///# C Specifications
    ///To enumerate the supported output, input and DPB image formats for a
    ///specific codec operation and video profile, call:
    ///```c
    ///// Provided by VK_KHR_video_queue
    ///VkResult vkGetPhysicalDeviceVideoFormatPropertiesKHR(
    ///    VkPhysicalDevice                            physicalDevice,
    ///    const VkPhysicalDeviceVideoFormatInfoKHR*   pVideoFormatInfo,
    ///    uint32_t*                                   pVideoFormatPropertyCount,
    ///    VkVideoFormatPropertiesKHR*                 pVideoFormatProperties);
    ///```
    ///# Parameters
    /// - [`physical_device`] is the physical device being queried.
    /// - [`p_video_format_info`] is a pointer to a [`PhysicalDeviceVideoFormatInfoKHR`] structure
    ///   specifying the codec and video profile for which information is returned.
    /// - [`p_video_format_property_count`] is a pointer to an integer related to the number of
    ///   video format properties available or queried, as described below.
    /// - [`p_video_format_properties`] is a pointer to an array of [`VideoFormatPropertiesKHR`]
    ///   structures in which supported formats are returned.
    ///# Description
    ///If [`p_video_format_properties`] is `NULL`, then the number of video format
    ///properties supported for the given [`physical_device`] is returned in
    ///[`p_video_format_property_count`].
    ///Otherwise, [`p_video_format_property_count`] **must**  point to a variable set by
    ///the user to the number of elements in the [`p_video_format_properties`]
    ///array, and on return the variable is overwritten with the number of values
    ///actually written to [`p_video_format_properties`].
    ///If the value of [`p_video_format_property_count`] is less than the number of
    ///video format properties supported, at most [`p_video_format_property_count`]
    ///values will be written to [`p_video_format_properties`], and
    ///`VK_INCOMPLETE` will be returned instead of `VK_SUCCESS`, to
    ///indicate that not all the available values were returned.If an implementation reports
    ///`VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_COINCIDE_BIT_KHR` is
    ///supported but
    ///`VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_DISTINCT_BIT_KHR` is not
    ///supported in [`VideoDecodeCapabilitiesKHR::flags`], then to query
    ///for video format properties for decode DPB or output, `imageUsage` **must**
    ///have both `VK_IMAGE_USAGE_VIDEO_DECODE_DPB_BIT_KHR` and
    ///`VK_IMAGE_USAGE_VIDEO_DECODE_DST_BIT_KHR` set.
    ///Otherwise, the call will fail with `VK_ERROR_FORMAT_NOT_SUPPORTED`.If an implementation
    /// reports
    ///`VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_DISTINCT_BIT_KHR` is
    ///supported but
    ///`VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_COINCIDE_BIT_KHR` is not
    ///supported in [`VideoDecodeCapabilitiesKHR::flags`], then to query
    ///for video format properties for decode DPB, `imageUsage` **must**  have
    ///`VK_IMAGE_USAGE_VIDEO_DECODE_DPB_BIT_KHR` set and
    ///`VK_IMAGE_USAGE_VIDEO_DECODE_DST_BIT_KHR` not set.
    ///Otherwise, the call will fail with `VK_ERROR_FORMAT_NOT_SUPPORTED`.
    ///Similarly, to query for video format properties for decode output,
    ///`imageUsage` **must**  have `VK_IMAGE_USAGE_VIDEO_DECODE_DST_BIT_KHR`
    ///set and `VK_IMAGE_USAGE_VIDEO_DECODE_DPB_BIT_KHR` not set.
    ///Otherwise, the call will fail with `VK_ERROR_FORMAT_NOT_SUPPORTED`.
    ///## Valid Usage
    /// - The `imageUsage` enum of [`PhysicalDeviceVideoFormatInfoKHR`] **must**  contain at least
    ///   one of the following video image usage bit(s): `VK_IMAGE_USAGE_VIDEO_DECODE_DST_BIT_KHR`,
    ///   `VK_IMAGE_USAGE_VIDEO_DECODE_DPB_BIT_KHR`, `VK_IMAGE_USAGE_VIDEO_ENCODE_SRC_BIT_KHR`, or
    ///   `VK_IMAGE_USAGE_VIDEO_ENCODE_DPB_BIT_KHR`
    ///
    ///## Valid Usage (Implicit)
    /// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
    /// - [`p_video_format_info`] **must**  be a valid pointer to a valid
    ///   [`PhysicalDeviceVideoFormatInfoKHR`] structure
    /// - [`p_video_format_property_count`] **must**  be a valid pointer to a `uint32_t` value
    /// - If the value referenced by [`p_video_format_property_count`] is not `0`, and
    ///   [`p_video_format_properties`] is not `NULL`, [`p_video_format_properties`] **must**  be a
    ///   valid pointer to an array of [`p_video_format_property_count`][`VideoFormatPropertiesKHR`]
    ///   structures
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
    /// * - `VK_ERROR_EXTENSION_NOT_PRESENT`  - `VK_ERROR_INITIALIZATION_FAILED`  -
    ///   `VK_ERROR_FORMAT_NOT_SUPPORTED`
    ///# Related
    /// - [`khr_video_queue`]
    /// - [`PhysicalDevice`]
    /// - [`PhysicalDeviceVideoFormatInfoKHR`]
    /// - [`VideoFormatPropertiesKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetPhysicalDeviceVideoFormatPropertiesKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_physical_device_video_format_properties_khr<'lt>(
        self: &Unique<PhysicalDevice>,
        p_video_format_info: &PhysicalDeviceVideoFormatInfoKHR<'lt>,
        p_video_format_property_count: Option<usize>,
    ) -> VulkanResult<SmallVec<VideoFormatPropertiesKHR<'lt>>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .instance()
            .vtable()
            .khr_video_queue()
            .and_then(|vtable| vtable.get_physical_device_video_format_properties_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .instance()
            .vtable()
            .khr_video_queue()
            .and_then(|vtable| vtable.get_physical_device_video_format_properties_khr())
            .unwrap_unchecked();
        let mut p_video_format_property_count = match p_video_format_property_count {
            Some(v) => v as _,
            None => {
                let mut v = 0;
                _function(
                    self.as_raw(),
                    p_video_format_info as *const PhysicalDeviceVideoFormatInfoKHR<'lt>,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            },
        };
        let mut p_video_format_properties = SmallVec::<VideoFormatPropertiesKHR<'lt>>::from_elem(
            Default::default(),
            p_video_format_property_count as usize,
        );
        let _return = _function(
            self.as_raw(),
            p_video_format_info as *const PhysicalDeviceVideoFormatInfoKHR<'lt>,
            &mut p_video_format_property_count,
            p_video_format_properties.as_mut_ptr(),
        );
        match _return {
            VulkanResultCodes::SUCCESS | VulkanResultCodes::INCOMPLETE => {
                VulkanResult::Success(_return, p_video_format_properties)
            },
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkCreateVideoSessionKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateVideoSessionKHR.html) - Creates a video session object
    ///# C Specifications
    ///To create a video session object, call:
    ///```c
    ///// Provided by VK_KHR_video_queue
    ///VkResult vkCreateVideoSessionKHR(
    ///    VkDevice                                    device,
    ///    const VkVideoSessionCreateInfoKHR*          pCreateInfo,
    ///    const VkAllocationCallbacks*                pAllocator,
    ///    VkVideoSessionKHR*                          pVideoSession);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device that creates the decode or encode session object.
    /// - [`p_create_info`] is a pointer to a [`VideoSessionCreateInfoKHR`] structure containing
    ///   parameters specifying the creation of the decode or encode session.
    /// - [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)
    ///   chapter.
    /// - [`p_video_session`] is a pointer to a [`VideoSessionKHR`] structure specifying the decode
    ///   or encode video session object which will be created by this function when it returns
    ///   `VK_SUCCESS`
    ///# Description
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_create_info`] **must**  be a valid pointer to a valid [`VideoSessionCreateInfoKHR`]
    ///   structure
    /// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
    ///   [`AllocationCallbacks`] structure
    /// - [`p_video_session`] **must**  be a valid pointer to a [`VideoSessionKHR`] handle
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
    ///   `VK_ERROR_INITIALIZATION_FAILED`  - `VK_ERROR_INCOMPATIBLE_DRIVER`  -
    ///   `VK_ERROR_FEATURE_NOT_PRESENT`
    ///# Related
    /// - [`khr_video_queue`]
    /// - [`AllocationCallbacks`]
    /// - [`Device`]
    /// - [`VideoSessionCreateInfoKHR`]
    /// - [`VideoSessionKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCreateVideoSessionKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn create_video_session_khr<'lt>(
        self: &Unique<Device>,
        p_create_info: &VideoSessionCreateInfoKHR<'lt>,
        p_allocator: Option<&AllocationCallbacks<'lt>>,
    ) -> VulkanResult<Unique<VideoSessionKHR>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_video_queue()
            .and_then(|vtable| vtable.create_video_session_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_video_queue()
            .and_then(|vtable| vtable.create_video_session_khr())
            .unwrap_unchecked();
        let mut p_video_session = MaybeUninit::<VideoSessionKHR>::uninit();
        let _return = _function(
            self.as_raw(),
            p_create_info as *const VideoSessionCreateInfoKHR<'lt>,
            p_allocator
                .map(|v| v as *const AllocationCallbacks<'lt>)
                .unwrap_or_else(std::ptr::null),
            p_video_session.as_mut_ptr(),
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(
                _return,
                Unique::new(self, p_video_session.assume_init(), AtomicBool::default()),
            ),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkDestroyVideoSessionKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyVideoSessionKHR.html) - Destroy decode session object
    ///# C Specifications
    ///To destroy a decode session object, call:
    ///```c
    ///// Provided by VK_KHR_video_queue
    ///void vkDestroyVideoSessionKHR(
    ///    VkDevice                                    device,
    ///    VkVideoSessionKHR                           videoSession,
    ///    const VkAllocationCallbacks*                pAllocator);
    ///```
    ///# Parameters
    /// - [`device`] is the device that was used for the creation of the video session.
    /// - [`video_session`] is the decode or encode video session to be destroyed.
    /// - [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)
    ///   chapter.
    ///# Description
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`video_session`] **must**  be a valid [`VideoSessionKHR`] handle
    /// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
    ///   [`AllocationCallbacks`] structure
    /// - [`video_session`] **must**  have been created, allocated, or retrieved from [`device`]
    ///# Related
    /// - [`khr_video_queue`]
    /// - [`AllocationCallbacks`]
    /// - [`Device`]
    /// - [`VideoSessionKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkDestroyVideoSessionKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn destroy_video_session_khr<'lt>(
        self: &Unique<Device>,
        video_session: VideoSessionKHR,
        p_allocator: Option<&AllocationCallbacks<'lt>>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_video_queue()
            .and_then(|vtable| vtable.destroy_video_session_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_video_queue()
            .and_then(|vtable| vtable.destroy_video_session_khr())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            video_session,
            p_allocator
                .map(|v| v as *const AllocationCallbacks<'lt>)
                .unwrap_or_else(std::ptr::null),
        );
        ()
    }
}
impl VideoSessionKHR {
    ///[vkCreateVideoSessionParametersKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateVideoSessionParametersKHR.html) - Creates video session video session parameter object
    ///# C Specifications
    ///To create a video session parameters object, call:
    ///```c
    ///// Provided by VK_KHR_video_queue
    ///VkResult vkCreateVideoSessionParametersKHR(
    ///    VkDevice                                    device,
    ///    const VkVideoSessionParametersCreateInfoKHR* pCreateInfo,
    ///    const VkAllocationCallbacks*                pAllocator,
    ///    VkVideoSessionParametersKHR*                pVideoSessionParameters);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device that was used for the creation of the video session
    ///   object.
    /// - [`p_create_info`] is a pointer to [`VideoSessionParametersCreateInfoKHR`] structure
    ///   specifying the video session parameters.
    /// - [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)
    ///   chapter.
    /// - [`p_video_session_parameters`] is a pointer to a [`VideoSessionParametersKHR`] handle in
    ///   which the video session parameters object is returned.
    ///# Description
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_create_info`] **must**  be a valid pointer to a valid
    ///   [`VideoSessionParametersCreateInfoKHR`] structure
    /// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
    ///   [`AllocationCallbacks`] structure
    /// - [`p_video_session_parameters`] **must**  be a valid pointer to a
    ///   [`VideoSessionParametersKHR`] handle
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_INITIALIZATION_FAILED`  - `VK_ERROR_OUT_OF_HOST_MEMORY`  -
    ///   `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_TOO_MANY_OBJECTS`
    ///# Related
    /// - [`khr_video_queue`]
    /// - [`AllocationCallbacks`]
    /// - [`Device`]
    /// - [`VideoSessionParametersCreateInfoKHR`]
    /// - [`VideoSessionParametersKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCreateVideoSessionParametersKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn create_video_session_parameters_khr<'lt>(
        self: &Unique<VideoSessionKHR>,
        p_create_info: &VideoSessionParametersCreateInfoKHR<'lt>,
        p_allocator: Option<&AllocationCallbacks<'lt>>,
    ) -> VulkanResult<Unique<VideoSessionParametersKHR>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .khr_video_queue()
            .and_then(|vtable| vtable.create_video_session_parameters_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .khr_video_queue()
            .and_then(|vtable| vtable.create_video_session_parameters_khr())
            .unwrap_unchecked();
        let mut p_video_session_parameters = MaybeUninit::<VideoSessionParametersKHR>::uninit();
        let _return = _function(
            self.device().as_raw(),
            p_create_info as *const VideoSessionParametersCreateInfoKHR<'lt>,
            p_allocator
                .map(|v| v as *const AllocationCallbacks<'lt>)
                .unwrap_or_else(std::ptr::null),
            p_video_session_parameters.as_mut_ptr(),
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(
                _return,
                Unique::new(self, p_video_session_parameters.assume_init(), AtomicBool::default()),
            ),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkUpdateVideoSessionParametersKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUpdateVideoSessionParametersKHR.html) - Update video session video session parameter object
    ///# C Specifications
    ///To update, add, or remove video session parameters state, call:
    ///```c
    ///// Provided by VK_KHR_video_queue
    ///VkResult vkUpdateVideoSessionParametersKHR(
    ///    VkDevice                                    device,
    ///    VkVideoSessionParametersKHR                 videoSessionParameters,
    ///    const VkVideoSessionParametersUpdateInfoKHR* pUpdateInfo);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device that was used for the creation of the video session
    ///   object.
    /// - [`video_session_parameters`] is the video session object that is going to be updated.
    /// - [`p_update_info`] is a pointer to a [`VideoSessionParametersUpdateInfoKHR`] structure
    ///   containing the session parameters update information.
    ///# Description
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`video_session_parameters`] **must**  be a valid [`VideoSessionParametersKHR`] handle
    /// - [`p_update_info`] **must**  be a valid pointer to a valid
    ///   [`VideoSessionParametersUpdateInfoKHR`] structure
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_INITIALIZATION_FAILED`  - `VK_ERROR_TOO_MANY_OBJECTS`
    ///# Related
    /// - [`khr_video_queue`]
    /// - [`Device`]
    /// - [`VideoSessionParametersKHR`]
    /// - [`VideoSessionParametersUpdateInfoKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkUpdateVideoSessionParametersKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn update_video_session_parameters_khr<'lt>(
        self: &Unique<Device>,
        video_session_parameters: VideoSessionParametersKHR,
        p_update_info: &VideoSessionParametersUpdateInfoKHR<'lt>,
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_video_queue()
            .and_then(|vtable| vtable.update_video_session_parameters_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_video_queue()
            .and_then(|vtable| vtable.update_video_session_parameters_khr())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            video_session_parameters,
            p_update_info as *const VideoSessionParametersUpdateInfoKHR<'lt>,
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkDestroyVideoSessionParametersKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyVideoSessionParametersKHR.html) - Destroy video session parameters object
    ///# C Specifications
    ///To destroy a video session object, call:
    ///```c
    ///// Provided by VK_KHR_video_queue
    ///void vkDestroyVideoSessionParametersKHR(
    ///    VkDevice                                    device,
    ///    VkVideoSessionParametersKHR                 videoSessionParameters,
    ///    const VkAllocationCallbacks*                pAllocator);
    ///```
    ///# Parameters
    /// - [`device`] is the device the video session was created with.
    /// - [`video_session_parameters`] is the video session parameters object to be destroyed.
    /// - [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)
    ///   chapter.
    ///# Description
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`video_session_parameters`] **must**  be a valid [`VideoSessionParametersKHR`] handle
    /// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
    ///   [`AllocationCallbacks`] structure
    ///# Related
    /// - [`khr_video_queue`]
    /// - [`AllocationCallbacks`]
    /// - [`Device`]
    /// - [`VideoSessionParametersKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkDestroyVideoSessionParametersKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn destroy_video_session_parameters_khr<'lt>(
        self: &Unique<Device>,
        video_session_parameters: VideoSessionParametersKHR,
        p_allocator: Option<&AllocationCallbacks<'lt>>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_video_queue()
            .and_then(|vtable| vtable.destroy_video_session_parameters_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_video_queue()
            .and_then(|vtable| vtable.destroy_video_session_parameters_khr())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            video_session_parameters,
            p_allocator
                .map(|v| v as *const AllocationCallbacks<'lt>)
                .unwrap_or_else(std::ptr::null),
        );
        ()
    }
}
impl Device {
    ///[vkGetVideoSessionMemoryRequirementsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetVideoSessionMemoryRequirementsKHR.html) - Get Memory Requirements
    ///# C Specifications
    ///To get memory requirements for a video session, call:
    ///```c
    ///// Provided by VK_KHR_video_queue
    ///VkResult vkGetVideoSessionMemoryRequirementsKHR(
    ///    VkDevice                                    device,
    ///    VkVideoSessionKHR                           videoSession,
    ///    uint32_t*                                   pVideoSessionMemoryRequirementsCount,
    ///    VkVideoGetMemoryPropertiesKHR*              pVideoSessionMemoryRequirements);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device that owns the video session.
    /// - [`video_session`] is the video session to query.
    /// - [`p_video_session_memory_requirements_count`] is a pointer to an integer related to the
    ///   number of memory heap requirements available or queried, as described below.
    /// - [`p_video_session_memory_requirements`] is `NULL` or a pointer to an array of
    ///   [`VideoGetMemoryPropertiesKHR`] structures in which the memory heap requirements of the
    ///   video session are returned.
    ///# Description
    ///If [`p_video_session_memory_requirements`] is `NULL`, then the number of
    ///memory heap types required for the video session is returned in
    ///[`p_video_session_memory_requirements_count`].
    ///Otherwise, [`p_video_session_memory_requirements_count`] **must**  point to a
    ///variable set by the user with the number of elements in the
    ///[`p_video_session_memory_requirements`] array, and on return the variable is
    ///overwritten with the number of formats actually written to
    ///[`p_video_session_memory_requirements`].
    ///If [`p_video_session_memory_requirements_count`] is less than the number of
    ///memory heap types required for the video session, then at most
    ///[`p_video_session_memory_requirements_count`] elements will be written to
    ///[`p_video_session_memory_requirements`], and `VK_INCOMPLETE` will be
    ///returned, instead of `VK_SUCCESS`, to indicate that not all required
    ///memory heap types were returned.
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`video_session`] **must**  be a valid [`VideoSessionKHR`] handle
    /// - [`p_video_session_memory_requirements_count`] **must**  be a valid pointer to a `uint32_t`
    ///   value
    /// - If the value referenced by [`p_video_session_memory_requirements_count`] is not `0`, and
    ///   [`p_video_session_memory_requirements`] is not `NULL`,
    ///   [`p_video_session_memory_requirements`] **must**  be a valid pointer to an array of
    ///   [`p_video_session_memory_requirements_count`][`VideoGetMemoryPropertiesKHR`] structures
    /// - [`video_session`] **must**  have been created, allocated, or retrieved from [`device`]
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
    /// * - `VK_ERROR_INITIALIZATION_FAILED`
    ///# Related
    /// - [`khr_video_queue`]
    /// - [`Device`]
    /// - [`VideoGetMemoryPropertiesKHR`]
    /// - [`VideoSessionKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetVideoSessionMemoryRequirementsKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_video_session_memory_requirements_khr<'lt>(
        self: &Unique<Device>,
        video_session: VideoSessionKHR,
        p_video_session_memory_requirements_count: Option<usize>,
    ) -> VulkanResult<SmallVec<VideoGetMemoryPropertiesKHR<'lt>>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_video_queue()
            .and_then(|vtable| vtable.get_video_session_memory_requirements_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_video_queue()
            .and_then(|vtable| vtable.get_video_session_memory_requirements_khr())
            .unwrap_unchecked();
        let mut p_video_session_memory_requirements_count = match p_video_session_memory_requirements_count {
            Some(v) => v as _,
            None => {
                let mut v = 0;
                _function(self.as_raw(), video_session, &mut v, std::ptr::null_mut());
                v
            },
        };
        let mut p_video_session_memory_requirements = SmallVec::<VideoGetMemoryPropertiesKHR<'lt>>::from_elem(
            Default::default(),
            p_video_session_memory_requirements_count as usize,
        );
        let _return = _function(
            self.as_raw(),
            video_session,
            &mut p_video_session_memory_requirements_count,
            p_video_session_memory_requirements.as_mut_ptr(),
        );
        match _return {
            VulkanResultCodes::SUCCESS | VulkanResultCodes::INCOMPLETE => {
                VulkanResult::Success(_return, p_video_session_memory_requirements)
            },
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkBindVideoSessionMemoryKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindVideoSessionMemoryKHR.html) - Bind Video Memory
    ///# C Specifications
    ///To attach memory to a video session object, call:
    ///```c
    ///// Provided by VK_KHR_video_queue
    ///VkResult vkBindVideoSessionMemoryKHR(
    ///    VkDevice                                    device,
    ///    VkVideoSessionKHR                           videoSession,
    ///    uint32_t                                    videoSessionBindMemoryCount,
    ///    const VkVideoBindMemoryKHR*                 pVideoSessionBindMemories);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device that owns the video session’s memory.
    /// - [`video_session`] is the video session to be bound with device memory.
    /// - [`video_session_bind_memory_count`] is the number of [`p_video_session_bind_memories`] to
    ///   be bound.
    /// - [`p_video_session_bind_memories`] is a pointer to an array of [`VideoBindMemoryKHR`]
    ///   structures specifying memory regions to be bound to a device memory heap.
    ///# Description
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`video_session`] **must**  be a valid [`VideoSessionKHR`] handle
    /// - [`p_video_session_bind_memories`] **must**  be a valid pointer to an array of
    ///   [`video_session_bind_memory_count`] valid [`VideoBindMemoryKHR`] structures
    /// - [`video_session_bind_memory_count`] **must**  be greater than `0`
    /// - [`video_session`] **must**  have been created, allocated, or retrieved from [`device`]
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
    ///   `VK_ERROR_INITIALIZATION_FAILED`
    ///# Related
    /// - [`khr_video_queue`]
    /// - [`Device`]
    /// - [`VideoBindMemoryKHR`]
    /// - [`VideoSessionKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkBindVideoSessionMemoryKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn bind_video_session_memory_khr<'lt>(
        self: &Unique<Device>,
        video_session: VideoSessionKHR,
        p_video_session_bind_memories: &[crate::extensions::khr_video_queue::VideoBindMemoryKHR<'lt>],
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_video_queue()
            .and_then(|vtable| vtable.bind_video_session_memory_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_video_queue()
            .and_then(|vtable| vtable.bind_video_session_memory_khr())
            .unwrap_unchecked();
        let video_session_bind_memory_count = (|len: usize| len)(p_video_session_bind_memories.len()) as _;
        let _return = _function(
            self.as_raw(),
            video_session,
            video_session_bind_memory_count,
            p_video_session_bind_memories.as_ptr(),
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
impl CommandBuffer {
    ///[vkCmdBeginVideoCodingKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginVideoCodingKHR.html) - Start decode jobs
    ///# C Specifications
    ///To start video decode or encode operations, call:
    ///```c
    ///// Provided by VK_KHR_video_queue
    ///void vkCmdBeginVideoCodingKHR(
    ///    VkCommandBuffer                             commandBuffer,
    ///    const VkVideoBeginCodingInfoKHR*            pBeginInfo);
    ///```
    ///# Parameters
    /// - [`command_buffer`] is the command buffer to be used when recording commands for the video
    ///   decode or encode operations.
    /// - [`p_begin_info`] is a pointer to a [`VideoBeginCodingInfoKHR`] structure.
    ///# Description
    ///## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`p_begin_info`] **must**  be a valid pointer to a valid [`VideoBeginCodingInfoKHR`]
    ///   structure
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support decode,
    ///   or encode operations
    /// - This command  **must**  only be called outside of a render pass instance
    /// - [`command_buffer`] **must**  be a primary [`CommandBuffer`]
    ///
    ///## Host Synchronization
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    ///## Command Properties
    ///# Related
    /// - [`khr_video_queue`]
    /// - [`CommandBuffer`]
    /// - [`VideoBeginCodingInfoKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdBeginVideoCodingKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_begin_video_coding_khr<'lt>(
        self: &Unique<CommandBuffer>,
        p_begin_info: &VideoBeginCodingInfoKHR<'lt>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .khr_video_queue()
            .and_then(|vtable| vtable.cmd_begin_video_coding_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .khr_video_queue()
            .and_then(|vtable| vtable.cmd_begin_video_coding_khr())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), p_begin_info as *const VideoBeginCodingInfoKHR<'lt>);
        ()
    }
}
impl CommandBuffer {
    ///[vkCmdControlVideoCodingKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdControlVideoCodingKHR.html) - Set encode rate control parameters
    ///# C Specifications
    ///To apply dynamic controls to video decode or video encode operations, call:
    ///```c
    ///// Provided by VK_KHR_video_queue
    ///void vkCmdControlVideoCodingKHR(
    ///    VkCommandBuffer                             commandBuffer,
    ///    const VkVideoCodingControlInfoKHR*          pCodingControlInfo);
    ///```
    ///# Parameters
    /// - [`command_buffer`] is the command buffer to be filled by this function.
    /// - [`p_coding_control_info`] is a pointer to a [`VideoCodingControlInfoKHR`] structure.
    ///# Description
    ///## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`p_coding_control_info`] **must**  be a valid pointer to a valid
    ///   [`VideoCodingControlInfoKHR`] structure
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support decode,
    ///   or encode operations
    /// - This command  **must**  only be called outside of a render pass instance
    /// - [`command_buffer`] **must**  be a primary [`CommandBuffer`]
    ///
    ///## Host Synchronization
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    ///## Command Properties
    ///# Related
    /// - [`khr_video_queue`]
    /// - [`CommandBuffer`]
    /// - [`VideoCodingControlInfoKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdControlVideoCodingKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_control_video_coding_khr<'lt>(
        self: &Unique<CommandBuffer>,
        p_coding_control_info: &VideoCodingControlInfoKHR<'lt>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .khr_video_queue()
            .and_then(|vtable| vtable.cmd_control_video_coding_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .khr_video_queue()
            .and_then(|vtable| vtable.cmd_control_video_coding_khr())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            p_coding_control_info as *const VideoCodingControlInfoKHR<'lt>,
        );
        ()
    }
}
impl CommandBuffer {
    ///[vkCmdEndVideoCodingKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndVideoCodingKHR.html) - End decode jobs
    ///# C Specifications
    ///To end video decode or encode operations, call:
    ///```c
    ///// Provided by VK_KHR_video_queue
    ///void vkCmdEndVideoCodingKHR(
    ///    VkCommandBuffer                             commandBuffer,
    ///    const VkVideoEndCodingInfoKHR*              pEndCodingInfo);
    ///```
    ///# Parameters
    /// - [`command_buffer`] is the command buffer to be filled by this function.
    /// - [`p_end_coding_info`] is a pointer to a [`VideoEndCodingInfoKHR`] structure.
    ///# Description
    ///## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`p_end_coding_info`] **must**  be a valid pointer to a valid [`VideoEndCodingInfoKHR`]
    ///   structure
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support decode,
    ///   or encode operations
    /// - This command  **must**  only be called outside of a render pass instance
    /// - [`command_buffer`] **must**  be a primary [`CommandBuffer`]
    ///
    ///## Host Synchronization
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    ///## Command Properties
    ///# Related
    /// - [`khr_video_queue`]
    /// - [`CommandBuffer`]
    /// - [`VideoEndCodingInfoKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdEndVideoCodingKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_end_video_coding_khr<'lt>(
        self: &Unique<CommandBuffer>,
        p_end_coding_info: &VideoEndCodingInfoKHR<'lt>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .khr_video_queue()
            .and_then(|vtable| vtable.cmd_end_video_coding_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .khr_video_queue()
            .and_then(|vtable| vtable.cmd_end_video_coding_khr())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), p_end_coding_info as *const VideoEndCodingInfoKHR<'lt>);
        ()
    }
}
///[VkVideoSessionKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoSessionKHR.html) - Opaque handle to a video session object
///# C Specifications
///Video session objects are abstracted and represented by
///[`VideoSessionKHR`] handles:
///```c
///// Provided by VK_KHR_video_queue
///VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkVideoSessionKHR)
///```
///# Related
/// - [`khr_video_queue`]
/// - [`VideoBeginCodingInfoKHR`]
/// - [`VideoSessionParametersCreateInfoKHR`]
/// - [`bind_video_session_memory_khr`]
/// - [`create_video_session_khr`]
/// - [`destroy_video_session_khr`]
/// - [`get_video_session_memory_requirements_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoSessionKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(transparent)]
pub struct VideoSessionKHR(pub u64);
impl VideoSessionKHR {
    ///Creates a new null handle
    #[inline]
    pub const fn null() -> Self {
        Self(0)
    }
    ///Checks if this is a null handle
    #[inline]
    pub fn is_null(&self) -> bool {
        self == &Self::null()
    }
    ///Gets the raw value
    #[inline]
    pub fn raw(&self) -> u64 {
        self.0
    }
}
unsafe impl Send for VideoSessionKHR {}
impl Default for VideoSessionKHR {
    fn default() -> Self {
        Self::null()
    }
}
impl Handle for VideoSessionKHR {
    type Parent = Unique<Device>;
    type VTable = ();
    type Metadata = AtomicBool;
    type Storage = u64;
    #[inline]
    fn as_stored(self) -> Self::Storage {
        self.0
    }
    #[inline]
    unsafe fn from_stored(this: Self::Storage) -> Self {
        Self(this)
    }
    #[inline]
    #[track_caller]
    unsafe fn destroy(self: &mut Unique<Self>) {
        if !self.metadata().load(Ordering::Acquire) {
            self.device().destroy_video_session_khr(self.as_raw().coerce(), None);
        }
    }
    #[inline]
    unsafe fn load_vtable(&self, _: &Self::Parent, _: &Self::Metadata) -> Self::VTable {}
}
impl Unique<VideoSessionKHR> {
    ///Gets the reference to the [`Entry`]
    #[inline]
    pub fn entry(&self) -> &Arc<Entry> {
        self.parent().parent().parent().parent()
    }
    ///Gets the reference to the [`Instance`]
    #[inline]
    pub fn instance(&self) -> &Unique<Instance> {
        self.parent().parent().parent()
    }
    ///Gets the reference to the [`PhysicalDevice`]
    #[inline]
    pub fn physical_device(&self) -> &Unique<PhysicalDevice> {
        self.parent().parent()
    }
    ///Gets the reference to the [`Device`]
    #[inline]
    pub fn device(&self) -> &Unique<Device> {
        self.parent()
    }
    ///Disables the base dropping behaviour of this handle
    #[inline]
    pub fn disable_drop(&self) {
        self.metadata().store(true, Ordering::Relaxed);
    }
}
#[cfg(feature = "VK_EXT_debug_utils")]
impl VideoSessionKHR {
    ///Give a user-friendly name to an object
    pub fn set_name(self: &Unique<Self>, name: &std::ffi::CStr) {
        assert!(
            self.strong_count() == 1,
            "`set_name` requires that the object be synchronized"
        );
        if !self.device().instance().metadata().ext_debug_utils() {
            return;
        }
        let info = crate::generated::extensions::ext_debug_utils::DebugUtilsObjectNameInfoEXT::default()
            .with_object_type(crate::generated::vulkan1_0::ObjectType::VIDEO_SESSION_KHR)
            .with_object_handle(self.as_stored() as u64)
            .with_object_name(name.as_ptr());
        unsafe {
            self.device().set_debug_utils_object_name_ext(&info).unwrap();
        }
    }
    ///Attach arbitrary data to an object
    ///In addition to setting a name for an object, debugging and validation layers may have uses
    /// for additional
    ///binary data on a per-object basis that has no other place in the Vulkan API. For example, a
    /// VkShaderModule
    ///could have additional debugging data attached to it to aid in offline shader tracing.
    pub fn set_tag(self: &Unique<Self>, tag: u64, data: &[u8]) {
        assert!(
            self.strong_count() == 1,
            "`set_name` requires that the object be synchronized"
        );
        if !self.device().instance().metadata().ext_debug_utils() {
            return;
        }
        let info = crate::generated::extensions::ext_debug_utils::DebugUtilsObjectTagInfoEXT::default()
            .with_object_type(crate::generated::vulkan1_0::ObjectType::VIDEO_SESSION_KHR)
            .with_object_handle(self.as_stored() as u64)
            .with_tag_name(tag)
            .with_tag_size(data.len() as _)
            .with_tag_raw(data.as_ptr().cast());
        unsafe {
            self.device().set_debug_utils_object_tag_ext(&info).unwrap();
        }
    }
}
///[VkVideoSessionParametersKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoSessionParametersKHR.html) - Opaque handle to a video video session parameters object
///# C Specifications
///Video session parameter objects are represented by
///[`VideoSessionParametersKHR`] handles:
///```c
///// Provided by VK_KHR_video_queue
///VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkVideoSessionParametersKHR)
///```
///# Related
/// - [`khr_video_queue`]
/// - [`VideoBeginCodingInfoKHR`]
/// - [`VideoSessionParametersCreateInfoKHR`]
/// - [`create_video_session_parameters_khr`]
/// - [`destroy_video_session_parameters_khr`]
/// - [`update_video_session_parameters_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoSessionParametersKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(transparent)]
pub struct VideoSessionParametersKHR(pub u64);
impl VideoSessionParametersKHR {
    ///Creates a new null handle
    #[inline]
    pub const fn null() -> Self {
        Self(0)
    }
    ///Checks if this is a null handle
    #[inline]
    pub fn is_null(&self) -> bool {
        self == &Self::null()
    }
    ///Gets the raw value
    #[inline]
    pub fn raw(&self) -> u64 {
        self.0
    }
}
unsafe impl Send for VideoSessionParametersKHR {}
impl Default for VideoSessionParametersKHR {
    fn default() -> Self {
        Self::null()
    }
}
impl Handle for VideoSessionParametersKHR {
    type Parent = Unique<VideoSessionKHR>;
    type VTable = ();
    type Metadata = AtomicBool;
    type Storage = u64;
    #[inline]
    fn as_stored(self) -> Self::Storage {
        self.0
    }
    #[inline]
    unsafe fn from_stored(this: Self::Storage) -> Self {
        Self(this)
    }
    #[inline]
    #[track_caller]
    unsafe fn destroy(self: &mut Unique<Self>) {
        if !self.metadata().load(Ordering::Acquire) {
            self.device()
                .destroy_video_session_parameters_khr(self.as_raw().coerce(), None);
        }
    }
    #[inline]
    unsafe fn load_vtable(&self, _: &Self::Parent, _: &Self::Metadata) -> Self::VTable {}
}
impl Unique<VideoSessionParametersKHR> {
    ///Gets the reference to the [`Entry`]
    #[inline]
    pub fn entry(&self) -> &Arc<Entry> {
        self.parent().parent().parent().parent().parent()
    }
    ///Gets the reference to the [`Instance`]
    #[inline]
    pub fn instance(&self) -> &Unique<Instance> {
        self.parent().parent().parent().parent()
    }
    ///Gets the reference to the [`PhysicalDevice`]
    #[inline]
    pub fn physical_device(&self) -> &Unique<PhysicalDevice> {
        self.parent().parent().parent()
    }
    ///Gets the reference to the [`Device`]
    #[inline]
    pub fn device(&self) -> &Unique<Device> {
        self.parent().parent()
    }
    ///Gets the reference to the [`VideoSessionKHR`]
    #[inline]
    pub fn video_session_khr(&self) -> &Unique<VideoSessionKHR> {
        self.parent()
    }
    ///Disables the base dropping behaviour of this handle
    #[inline]
    pub fn disable_drop(&self) {
        self.metadata().store(true, Ordering::Relaxed);
    }
}
#[cfg(feature = "VK_EXT_debug_utils")]
impl VideoSessionParametersKHR {
    ///Give a user-friendly name to an object
    pub fn set_name(self: &Unique<Self>, name: &std::ffi::CStr) {
        assert!(
            self.strong_count() == 1,
            "`set_name` requires that the object be synchronized"
        );
        if !self.device().instance().metadata().ext_debug_utils() {
            return;
        }
        let info = crate::generated::extensions::ext_debug_utils::DebugUtilsObjectNameInfoEXT::default()
            .with_object_type(crate::generated::vulkan1_0::ObjectType::VIDEO_SESSION_PARAMETERS_KHR)
            .with_object_handle(self.as_stored() as u64)
            .with_object_name(name.as_ptr());
        unsafe {
            self.device().set_debug_utils_object_name_ext(&info).unwrap();
        }
    }
    ///Attach arbitrary data to an object
    ///In addition to setting a name for an object, debugging and validation layers may have uses
    /// for additional
    ///binary data on a per-object basis that has no other place in the Vulkan API. For example, a
    /// VkShaderModule
    ///could have additional debugging data attached to it to aid in offline shader tracing.
    pub fn set_tag(self: &Unique<Self>, tag: u64, data: &[u8]) {
        assert!(
            self.strong_count() == 1,
            "`set_name` requires that the object be synchronized"
        );
        if !self.device().instance().metadata().ext_debug_utils() {
            return;
        }
        let info = crate::generated::extensions::ext_debug_utils::DebugUtilsObjectTagInfoEXT::default()
            .with_object_type(crate::generated::vulkan1_0::ObjectType::VIDEO_SESSION_PARAMETERS_KHR)
            .with_object_handle(self.as_stored() as u64)
            .with_tag_name(tag)
            .with_tag_size(data.len() as _)
            .with_tag_raw(data.as_ptr().cast());
        unsafe {
            self.device().set_debug_utils_object_tag_ext(&info).unwrap();
        }
    }
}
///The V-table of [`Instance`] for functions from `VK_KHR_video_queue`
pub struct InstanceKhrVideoQueueVTable {
    ///See [`FNGetPhysicalDeviceVideoCapabilitiesKhr`] for more information.
    pub get_physical_device_video_capabilities_khr: FNGetPhysicalDeviceVideoCapabilitiesKhr,
    ///See [`FNGetPhysicalDeviceVideoFormatPropertiesKhr`] for more information.
    pub get_physical_device_video_format_properties_khr: FNGetPhysicalDeviceVideoFormatPropertiesKhr,
}
impl InstanceKhrVideoQueueVTable {
    ///Loads the VTable from the owner and the names
    #[track_caller]
    pub fn load(
        loader_fn: unsafe extern "system" fn(
            Instance,
            *const std::os::raw::c_char,
        ) -> Option<unsafe extern "system" fn()>,
        loader: Instance,
    ) -> Self {
        Self {
            get_physical_device_video_capabilities_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetPhysicalDeviceVideoCapabilitiesKHR").as_ptr(),
                ))
            },
            get_physical_device_video_format_properties_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetPhysicalDeviceVideoFormatPropertiesKHR").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::get_physical_device_video_capabilities_khr`]. See
    /// [`FNGetPhysicalDeviceVideoCapabilitiesKhr`] for more information.
    pub fn get_physical_device_video_capabilities_khr(&self) -> FNGetPhysicalDeviceVideoCapabilitiesKhr {
        self.get_physical_device_video_capabilities_khr
    }
    ///Gets [`Self::get_physical_device_video_format_properties_khr`]. See
    /// [`FNGetPhysicalDeviceVideoFormatPropertiesKhr`] for more information.
    pub fn get_physical_device_video_format_properties_khr(&self) -> FNGetPhysicalDeviceVideoFormatPropertiesKhr {
        self.get_physical_device_video_format_properties_khr
    }
}
///The V-table of [`Device`] for functions from `VK_KHR_video_queue`
pub struct DeviceKhrVideoQueueVTable {
    ///See [`FNCreateVideoSessionKhr`] for more information.
    pub create_video_session_khr: FNCreateVideoSessionKhr,
    ///See [`FNDestroyVideoSessionKhr`] for more information.
    pub destroy_video_session_khr: FNDestroyVideoSessionKhr,
    ///See [`FNCreateVideoSessionParametersKhr`] for more information.
    pub create_video_session_parameters_khr: FNCreateVideoSessionParametersKhr,
    ///See [`FNUpdateVideoSessionParametersKhr`] for more information.
    pub update_video_session_parameters_khr: FNUpdateVideoSessionParametersKhr,
    ///See [`FNDestroyVideoSessionParametersKhr`] for more information.
    pub destroy_video_session_parameters_khr: FNDestroyVideoSessionParametersKhr,
    ///See [`FNGetVideoSessionMemoryRequirementsKhr`] for more information.
    pub get_video_session_memory_requirements_khr: FNGetVideoSessionMemoryRequirementsKhr,
    ///See [`FNBindVideoSessionMemoryKhr`] for more information.
    pub bind_video_session_memory_khr: FNBindVideoSessionMemoryKhr,
    ///See [`FNCmdBeginVideoCodingKhr`] for more information.
    pub cmd_begin_video_coding_khr: FNCmdBeginVideoCodingKhr,
    ///See [`FNCmdControlVideoCodingKhr`] for more information.
    pub cmd_control_video_coding_khr: FNCmdControlVideoCodingKhr,
    ///See [`FNCmdEndVideoCodingKhr`] for more information.
    pub cmd_end_video_coding_khr: FNCmdEndVideoCodingKhr,
}
impl DeviceKhrVideoQueueVTable {
    ///Loads the VTable from the owner and the names
    #[track_caller]
    pub fn load(
        loader_fn: unsafe extern "system" fn(
            Device,
            *const std::os::raw::c_char,
        ) -> Option<unsafe extern "system" fn()>,
        loader: Device,
    ) -> Self {
        Self {
            create_video_session_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCreateVideoSessionKHR").as_ptr()))
            },
            destroy_video_session_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkDestroyVideoSessionKHR").as_ptr()))
            },
            create_video_session_parameters_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCreateVideoSessionParametersKHR").as_ptr(),
                ))
            },
            update_video_session_parameters_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkUpdateVideoSessionParametersKHR").as_ptr(),
                ))
            },
            destroy_video_session_parameters_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkDestroyVideoSessionParametersKHR").as_ptr(),
                ))
            },
            get_video_session_memory_requirements_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetVideoSessionMemoryRequirementsKHR").as_ptr(),
                ))
            },
            bind_video_session_memory_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkBindVideoSessionMemoryKHR").as_ptr()))
            },
            cmd_begin_video_coding_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdBeginVideoCodingKHR").as_ptr()))
            },
            cmd_control_video_coding_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdControlVideoCodingKHR").as_ptr()))
            },
            cmd_end_video_coding_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdEndVideoCodingKHR").as_ptr()))
            },
        }
    }
    ///Gets [`Self::create_video_session_khr`]. See [`FNCreateVideoSessionKhr`] for more
    /// information.
    pub fn create_video_session_khr(&self) -> FNCreateVideoSessionKhr {
        self.create_video_session_khr
    }
    ///Gets [`Self::destroy_video_session_khr`]. See [`FNDestroyVideoSessionKhr`] for more
    /// information.
    pub fn destroy_video_session_khr(&self) -> FNDestroyVideoSessionKhr {
        self.destroy_video_session_khr
    }
    ///Gets [`Self::create_video_session_parameters_khr`]. See
    /// [`FNCreateVideoSessionParametersKhr`] for more information.
    pub fn create_video_session_parameters_khr(&self) -> FNCreateVideoSessionParametersKhr {
        self.create_video_session_parameters_khr
    }
    ///Gets [`Self::update_video_session_parameters_khr`]. See
    /// [`FNUpdateVideoSessionParametersKhr`] for more information.
    pub fn update_video_session_parameters_khr(&self) -> FNUpdateVideoSessionParametersKhr {
        self.update_video_session_parameters_khr
    }
    ///Gets [`Self::destroy_video_session_parameters_khr`]. See
    /// [`FNDestroyVideoSessionParametersKhr`] for more information.
    pub fn destroy_video_session_parameters_khr(&self) -> FNDestroyVideoSessionParametersKhr {
        self.destroy_video_session_parameters_khr
    }
    ///Gets [`Self::get_video_session_memory_requirements_khr`]. See
    /// [`FNGetVideoSessionMemoryRequirementsKhr`] for more information.
    pub fn get_video_session_memory_requirements_khr(&self) -> FNGetVideoSessionMemoryRequirementsKhr {
        self.get_video_session_memory_requirements_khr
    }
    ///Gets [`Self::bind_video_session_memory_khr`]. See [`FNBindVideoSessionMemoryKhr`] for more
    /// information.
    pub fn bind_video_session_memory_khr(&self) -> FNBindVideoSessionMemoryKhr {
        self.bind_video_session_memory_khr
    }
    ///Gets [`Self::cmd_begin_video_coding_khr`]. See [`FNCmdBeginVideoCodingKhr`] for more
    /// information.
    pub fn cmd_begin_video_coding_khr(&self) -> FNCmdBeginVideoCodingKhr {
        self.cmd_begin_video_coding_khr
    }
    ///Gets [`Self::cmd_control_video_coding_khr`]. See [`FNCmdControlVideoCodingKhr`] for more
    /// information.
    pub fn cmd_control_video_coding_khr(&self) -> FNCmdControlVideoCodingKhr {
        self.cmd_control_video_coding_khr
    }
    ///Gets [`Self::cmd_end_video_coding_khr`]. See [`FNCmdEndVideoCodingKhr`] for more
    /// information.
    pub fn cmd_end_video_coding_khr(&self) -> FNCmdEndVideoCodingKhr {
        self.cmd_end_video_coding_khr
    }
}
