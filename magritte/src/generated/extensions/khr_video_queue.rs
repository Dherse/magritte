use crate::{
    vulkan1_0::{
        BaseInStructure, BaseOutStructure, Bool32, DeviceMemory, DeviceSize, Extent2D, Format, ImageUsageFlags,
        ImageView, Offset2D, StructureType,
    },
    vulkan1_1::MemoryRequirements2,
};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{ffi::CStr, marker::PhantomData};
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
/// - [`QueryResultStatusNotReadyKhr`] specifies that the query result is not yet available.
/// - [`QueryResultStatusErrorKhr`] specifies that operations did not complete successfully.
/// - [`QueryResultStatusCompleteKhr`] specifies that operations completed successfully and the
///   query result is available.
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum QueryResultStatusKHR {
    ///[`QueryResultStatusErrorKhr`] specifies that operations did not
    ///complete successfully.
    QueryResultStatusErrorKhr = -1,
    ///[`QueryResultStatusNotReadyKhr`] specifies that the query
    ///result is not yet available.
    QueryResultStatusNotReadyKhr = 0,
    ///[`QueryResultStatusCompleteKhr`] specifies that operations
    ///completed successfully and the query result is available.
    QueryResultStatusCompleteKhr = 1,
}
impl const Default for QueryResultStatusKHR {
    fn default() -> Self {
        QueryResultStatusNotReadyKhr
    }
}
impl QueryResultStatusKHR {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_QUEUE_FAMILY_PROPERTIES_2_KHR`
/// - [`video_codec_operations`]**must** be a valid combination of
///   [`VideoCodecOperationFlagBitsKHR`] values
/// - [`video_codec_operations`]**must** not be `0`
///# Related
/// - [`VK_KHR_video_queue`]
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VideoQueueFamilyProperties2KHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`video_codec_operations`] is a bitmask of
    ///[`VideoCodecOperationFlagBitsKHR`] specifying supported video codec
    ///operation(s).
    video_codec_operations: VideoCodecOperationFlagsKHR,
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_2_KHR`
///# Related
/// - [`VK_KHR_video_queue`]
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct QueueFamilyQueryResultStatusProperties2KHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`supported`] reports [`TRUE`] if query type
    ///`VK_QUERY_TYPE_RESULT_STATUS_ONLY_KHR` and use of
    ///`VK_QUERY_RESULT_WITH_STATUS_BIT_KHR` are supported.
    supported: Bool32,
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
///   [`p_profiles`].
/// - [`p_profiles`] is a pointer to an array of [`VideoProfileKHR`] structures. Each
///   [`VideoProfileKHR`] structure **must** chain the corresponding codec-operation specific
///   extension video profile structure.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_PROFILES_KHR`
/// - [`p_profiles`]**must** be a valid pointer to an array of [`profile_count`] valid
///   [`VideoProfileKHR`] structures
/// - [`profile_count`]**must** be greater than `0`
///# Related
/// - [`VK_KHR_video_queue`]
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VideoProfilesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`profile_count`] is an integer which holds the number of video
    ///profiles included in [`p_profiles`].
    profile_count: u32,
    ///[`p_profiles`] is a pointer to an array of [`VideoProfileKHR`]
    ///structures.
    ///Each [`VideoProfileKHR`] structure **must** chain the corresponding
    ///codec-operation specific extension video profile structure.
    p_profiles: *mut VideoProfileKHR<'lt>,
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
/// - [`p_video_profiles`] is a pointer to a [`VideoProfilesKHR`] structure providing the video
///   profile(s) of video session(s) that will use the image. For most use cases, the image is used
///   by a single video session and a single video profile is provided. For a use case such as
///   transcode, where a decode session output image **may** be used as encode input for one or more
///   encode sessions, multiple video profiles representing the video sessions that will share the
///   image **may** be provided.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
///# Related
/// - [`VK_KHR_video_queue`]
/// - [`ImageUsageFlags`]
/// - [`StructureType`]
/// - [`VideoProfilesKHR`]
/// - [`GetPhysicalDeviceVideoFormatPropertiesKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceVideoFormatInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`image_usage`] is a bitmask of [`ImageUsageFlagBits`] specifying
    ///intended video image usages.
    image_usage: ImageUsageFlags,
    ///[`p_video_profiles`] is a pointer to a [`VideoProfilesKHR`]
    ///structure providing the video profile(s) of video session(s) that will
    ///use the image.
    ///For most use cases, the image is used by a single video session and a
    ///single video profile is provided.
    ///For a use case such as transcode, where a decode session output image
    ///**may** be used as encode input for one or more encode sessions, multiple
    ///video profiles representing the video sessions that will share the image
    ///**may** be provided.
    p_video_profiles: *mut VideoProfilesKHR<'lt>,
}
///[VkVideoFormatPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoFormatPropertiesKHR.html) - Structure enumerating the video image formats
///# C Specifications
///The [`VideoFormatPropertiesKHR`] output structure for
///[`GetPhysicalDeviceVideoFormatPropertiesKHR`] is defined as:
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
///`VK_ERROR_FORMAT_NOT_SUPPORTED` is returned.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_FORMAT_PROPERTIES_KHR`
/// - [`p_next`]**must** be `NULL`
///# Related
/// - [`VK_KHR_video_queue`]
/// - [`Format`]
/// - [`StructureType`]
/// - [`GetPhysicalDeviceVideoFormatPropertiesKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VideoFormatPropertiesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`format`] is one of the supported formats reported by the
    ///implementation.
    format: Format,
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_PROFILE_KHR`
/// - [`video_codec_operation`]**must** be a valid [`VideoCodecOperationFlagBitsKHR`] value
/// - [`chroma_subsampling`]**must** be a valid combination of [`VideoChromaSubsamplingFlagBitsKHR`]
///   values
/// - [`chroma_subsampling`]**must** not be `0`
/// - [`luma_bit_depth`]**must** be a valid combination of [`VideoComponentBitDepthFlagBitsKHR`]
///   values
/// - [`luma_bit_depth`]**must** not be `0`
/// - [`chroma_bit_depth`]**must** be a valid combination of [`VideoComponentBitDepthFlagBitsKHR`]
///   values
/// - [`chroma_bit_depth`]**must** not be `0`
///# Related
/// - [`VK_KHR_video_queue`]
/// - [`StructureType`]
/// - [`VideoChromaSubsamplingFlagsKHR`]
/// - [`VideoCodecOperationFlagBitsKHR`]
/// - [`VideoComponentBitDepthFlagsKHR`]
/// - [`VideoProfilesKHR`]
/// - [`VideoSessionCreateInfoKHR`]
/// - [`GetPhysicalDeviceVideoCapabilitiesKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VideoProfileKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`video_codec_operation`] is a [`VideoCodecOperationFlagBitsKHR`]
    ///value specifying a video codec operation.
    video_codec_operation: VideoCodecOperationFlagBitsKHR,
    ///[`chroma_subsampling`] is a bitmask of
    ///[`VideoChromaSubsamplingFlagBitsKHR`] specifying video chroma
    ///subsampling information.
    chroma_subsampling: VideoChromaSubsamplingFlagsKHR,
    ///[`luma_bit_depth`] is a bitmask of
    ///[`VideoComponentBitDepthFlagBitsKHR`] specifying video luma bit
    ///depth information.
    luma_bit_depth: VideoComponentBitDepthFlagsKHR,
    ///[`chroma_bit_depth`] is a bitmask of
    ///[`VideoComponentBitDepthFlagBitsKHR`] specifying video chroma bit
    ///depth information.
    chroma_bit_depth: VideoComponentBitDepthFlagsKHR,
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
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_CAPABILITIES_KHR`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain **must**
///   be either `NULL` or a pointer to a valid instance of [`VideoDecodeCapabilitiesKHR`] or
///   [`VideoEncodeCapabilitiesKHR`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
///# Related
/// - [`VK_KHR_video_queue`]
/// - [`DeviceSize`]
/// - [`Extent2D`]
/// - [`StructureType`]
/// - [`VideoCapabilityFlagsKHR`]
/// - [`GetPhysicalDeviceVideoCapabilitiesKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VideoCapabilitiesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`capability_flags`] is a bitmask of [`VideoCapabilityFlagBitsKHR`]
    ///specifying capability flags.
    capability_flags: VideoCapabilityFlagsKHR,
    ///[`min_bitstream_buffer_offset_alignment`] is the minimum alignment for the
    ///input or output bitstream buffer offset.
    min_bitstream_buffer_offset_alignment: DeviceSize,
    ///[`min_bitstream_buffer_size_alignment`] is the minimum alignment for the
    ///input or output bitstream buffer size
    min_bitstream_buffer_size_alignment: DeviceSize,
    ///[`video_picture_extent_granularity`] is the minimum size alignment of the
    ///extent with the required padding for the decoded or encoded video
    ///images.
    video_picture_extent_granularity: Extent2D,
    ///[`min_extent`] is the minimum width and height of the decoded or
    ///encoded video.
    min_extent: Extent2D,
    ///[`max_extent`] is the maximum width and height of the decoded or
    ///encoded video.
    max_extent: Extent2D,
    ///[`max_reference_pictures_slots_count`] is the maximum number of DPB Slots
    ///supported by the implementation for a single video session instance.
    max_reference_pictures_slots_count: u32,
    ///[`max_reference_pictures_active_count`] is the maximum slots that can be
    ///used as [Reference Pictures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#reference-picture) with a single decode or
    ///encode operation.
    max_reference_pictures_active_count: u32,
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
///   information returned in [`p_memory_requirements`].
/// - [`p_memory_requirements`] is a pointer to a [`MemoryRequirements2`] structure in which the
///   requested memory heap requirements for the heap with index [`memory_bind_index`] are returned.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_GET_MEMORY_PROPERTIES_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`p_memory_requirements`]**must** be a valid pointer to a [`MemoryRequirements2`] structure
///# Related
/// - [`VK_KHR_video_queue`]
/// - [`MemoryRequirements2`]
/// - [`StructureType`]
/// - [`GetVideoSessionMemoryRequirementsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VideoGetMemoryPropertiesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`memory_bind_index`] is the memory bind index of the memory heap type
    ///described by the information returned in [`p_memory_requirements`].
    memory_bind_index: u32,
    ///[`p_memory_requirements`] is a pointer to a [`MemoryRequirements2`]
    ///structure in which the requested memory heap requirements for the heap
    ///with index [`memory_bind_index`] are returned.
    p_memory_requirements: *const MemoryRequirements2<'lt>,
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
///   [`GetVideoSessionMemoryRequirementsKHR`].
/// - [`memory`] is the allocated device memory to be bound to the video session’s heap with index
///   [`memory_bind_index`].
/// - [`memory_offset`] is the start offset of the region of [`memory`] which is to be bound.
/// - [`memory_size`] is the size in bytes of the region of [`memory`], starting from
///   [`memory_offset`] bytes, to be bound.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_BIND_MEMORY_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`memory`]**must** be a valid [`DeviceMemory`] handle
///# Related
/// - [`VK_KHR_video_queue`]
/// - [`DeviceMemory`]
/// - [`DeviceSize`]
/// - [`StructureType`]
/// - [`BindVideoSessionMemoryKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VideoBindMemoryKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`memory_bind_index`] is the index of the device memory heap returned in
    ///[`VideoGetMemoryPropertiesKHR`]::[`memory_bind_index`] from
    ///[`GetVideoSessionMemoryRequirementsKHR`].
    memory_bind_index: u32,
    ///[`memory`] is the allocated device memory to be bound to the video
    ///session’s heap with index [`memory_bind_index`].
    memory: DeviceMemory,
    ///[`memory_offset`] is the start offset of the region of [`memory`]
    ///which is to be bound.
    memory_offset: DeviceSize,
    ///[`memory_size`] is the size in bytes of the region of [`memory`],
    ///starting from [`memory_offset`] bytes, to be bound.
    memory_size: DeviceSize,
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_PICTURE_RESOURCE_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`image_view_binding`]**must** be a valid [`ImageView`] handle
///# Related
/// - [`VK_KHR_video_queue`]
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VideoPictureResourceKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`coded_offset`] is the offset to be used for the picture resource.
    coded_offset: Offset2D,
    ///[`coded_extent`] is the extent to be used for the picture resource.
    coded_extent: Extent2D,
    ///[`base_array_layer`] is the first array layer to be accessed for the
    ///Decode or Encode Operations.
    base_array_layer: u32,
    ///[`image_view_binding`] is a [`ImageView`] image view representing
    ///this picture resource.
    image_view_binding: ImageView,
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
/// - [`p_picture_resource`] is a pointer to a [`VideoPictureResourceKHR`] structure describing the
///   picture resource bound to this slot index.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_REFERENCE_SLOT_KHR`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain **must**
///   be either `NULL` or a pointer to a valid instance of [`VideoDecodeH264DpbSlotInfoEXT`] or
///   [`VideoDecodeH265DpbSlotInfoEXT`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
/// - [`p_picture_resource`]**must** be a valid pointer to a valid [`VideoPictureResourceKHR`]
///   structure
///# Related
/// - [`VK_KHR_video_queue`]
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VideoReferenceSlotKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`slot_index`] is the unique reference slot index used for the encode
    ///or decode operation.
    slot_index: i8,
    ///[`p_picture_resource`] is a pointer to a [`VideoPictureResourceKHR`]
    ///structure describing the picture resource bound to this slot index.
    p_picture_resource: *mut VideoPictureResourceKHR<'lt>,
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
///} VkVideoSessionCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`queue_family_index`] is the queue family of the created video session.
/// - [`flags`] is a bitmask of [`VideoSessionCreateFlagBitsKHR`] specifying creation flags.
/// - [`p_video_profile`] is a pointer to a [`VideoProfileKHR`] structure.
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
///# Description
///Valid Usage
/// - [`p_video_profile`]**must** be a pointer to a valid [`VideoProfileKHR`] structure whose
///   [`p_next`] chain **must** include a valid codec-specific profile structure
/// - If [Reference Pictures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#reference-picture)
///   are required for use with the created video session, the
///   [`max_reference_pictures_slots_count`]**must** be set to a value bigger than `0`
/// - [`max_reference_pictures_slots_count`]**cannot** exceed the implementation reported
///   [`VideoCapabilitiesKHR`]::[`max_reference_pictures_slots_count`]
/// - If [Reference Pictures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#reference-picture)
///   are required for use with the created video session, the
///   [`max_reference_pictures_active_count`]**must** be set to a value bigger than `0`
/// - [`max_reference_pictures_active_count`]**cannot** exceed the implementation reported
///   [`VideoCapabilitiesKHR`]::[`max_reference_pictures_active_count`]
/// - [`max_reference_pictures_active_count`]**cannot** exceed the
///   [`max_reference_pictures_slots_count`]
/// - [`max_coded_extent`]**cannot** be smaller than [`VideoCapabilitiesKHR::min_extent`] and bigger
///   than [`VideoCapabilitiesKHR::max_extent`]
/// - [`reference_pictures_format`]**must** be one of the supported formats in
///   [`VideoFormatPropertiesKHR`]`format` returned by the
///   [`GetPhysicalDeviceVideoFormatPropertiesKHR`] when the
///   [`PhysicalDeviceVideoFormatInfoKHR`]`imageUsage` contains
///   `VK_IMAGE_USAGE_VIDEO_DECODE_DPB_BIT_KHR` or `VK_IMAGE_USAGE_VIDEO_ENCODE_DPB_BIT_KHR`
///   depending on the session codec operation
/// - [`picture_format`] for decode output **must** be one of the supported formats in
///   [`VideoFormatPropertiesKHR`]`format` returned by the
///   [`GetPhysicalDeviceVideoFormatPropertiesKHR`] when the
///   [`PhysicalDeviceVideoFormatInfoKHR`]`imageUsage` contains
///   `VK_IMAGE_USAGE_VIDEO_DECODE_DST_BIT_KHR`
/// - [`picture_format`] targeting encode operations **must** be one of the supported formats in
///   [`VideoFormatPropertiesKHR`]`format` returned by the
///   [`GetPhysicalDeviceVideoFormatPropertiesKHR`] when the
///   [`PhysicalDeviceVideoFormatInfoKHR`]`imageUsage` contains
///   `VK_IMAGE_USAGE_VIDEO_ENCODE_SRC_BIT_KHR`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_SESSION_CREATE_INFO_KHR`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain **must**
///   be either `NULL` or a pointer to a valid instance of [`VideoDecodeH264SessionCreateInfoEXT`],
///   [`VideoDecodeH265SessionCreateInfoEXT`], [`VideoEncodeH264SessionCreateInfoEXT`], or
///   [`VideoEncodeH265SessionCreateInfoEXT`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
/// - [`flags`]**must** be a valid combination of [`VideoSessionCreateFlagBitsKHR`] values
/// - [`p_video_profile`]**must** be a valid pointer to a valid [`VideoProfileKHR`] structure
/// - [`picture_format`]**must** be a valid [`Format`] value
/// - [`reference_pictures_format`]**must** be a valid [`Format`] value
///# Related
/// - [`VK_KHR_video_queue`]
/// - [`Extent2D`]
/// - [`Format`]
/// - [`StructureType`]
/// - [`VideoProfileKHR`]
/// - [`VideoSessionCreateFlagsKHR`]
/// - [`CreateVideoSessionKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VideoSessionCreateInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`queue_family_index`] is the queue family of the created video session.
    queue_family_index: u32,
    ///[`flags`] is a bitmask of [`VideoSessionCreateFlagBitsKHR`]
    ///specifying creation flags.
    flags: VideoSessionCreateFlagsKHR,
    ///[`p_video_profile`] is a pointer to a [`VideoProfileKHR`] structure.
    p_video_profile: *mut VideoProfileKHR<'lt>,
    ///[`picture_format`] is the format of the [image
    ///views](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views) representing decoded [Output](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#decoded-output-picture) or
    ///encoded [Input](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#input-encode-picture) pictures.
    picture_format: Format,
    ///[`max_coded_extent`] is the maximum width and height of the coded
    ///pictures that this instance will be able to support.
    max_coded_extent: Extent2D,
    ///[`reference_pictures_format`] is the format of the [DPB](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#dpb) image
    ///views representing the [Reference Pictures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#reference-picture).
    reference_pictures_format: Format,
    ///[`max_reference_pictures_slots_count`] is the maximum number of
    ///[DPB Slots](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#dpb-slot) that can be activated with associated
    ///[Video Picture Resources](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-picture-resources) for the created
    ///video session.
    max_reference_pictures_slots_count: u32,
    ///[`max_reference_pictures_active_count`] is the maximum number of active
    ///[DPB Slots](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#dpb-slot) that can be used as Dpb or Reconstructed
    ///[Reference Pictures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#reference-picture) within a single decode or
    ///encode operation for the created video session.
    max_reference_pictures_active_count: u32,
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
/// - [`video_session_parameters_template`] is [`crate::utils::Handle::null`] or a valid handle to a
///   [`VideoSessionParametersKHR`] object. If this parameter represents a valid handle, then the
///   underlying Video Session Parameters object will be used as a template for constructing the new
///   video session parameters object. All of the template object’s current parameters will be
///   inherited by the new object in such a case. Optionally, some of the template’s parameters can
///   be updated or new parameters added to the newly constructed object via the extension-specific
///   parameters.
/// - [`video_session`] is the video session object against which the video session parameters
///   object is going to be created.
///# Description
///Valid Usage
/// - If [`video_session_parameters_template`] represents a valid handle, it **must** have been
///   created against [`video_session`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain **must**
///   be either `NULL` or a pointer to a valid instance of
///   [`VideoDecodeH264SessionParametersCreateInfoEXT`],
///   [`VideoDecodeH265SessionParametersCreateInfoEXT`],
///   [`VideoEncodeH264SessionParametersCreateInfoEXT`], or
///   [`VideoEncodeH265SessionParametersCreateInfoEXT`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
/// - If [`video_session_parameters_template`] is not [`crate::utils::Handle::null`],
///   [`video_session_parameters_template`]**must** be a valid [`VideoSessionParametersKHR`] handle
/// - [`video_session`]**must** be a valid [`VideoSessionKHR`] handle
/// - If [`video_session_parameters_template`] is a valid handle, it **must** have been created,
///   allocated, or retrieved from [`video_session`]
/// - Both of [`video_session`], and [`video_session_parameters_template`] that are valid handles of
///   non-ignored parameters **must** have been created, allocated, or retrieved from the same
///   [`Device`]
///# Related
/// - [`VK_KHR_video_queue`]
/// - [`StructureType`]
/// - [`VideoSessionKHR`]
/// - [`VideoSessionParametersKHR`]
/// - [`CreateVideoSessionParametersKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VideoSessionParametersCreateInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`video_session_parameters_template`] is [`crate::utils::Handle::null`] or a valid
    ///handle to a [`VideoSessionParametersKHR`] object.
    ///If this parameter represents a valid handle, then the underlying Video
    ///Session Parameters object will be used as a template for constructing
    ///the new video session parameters object.
    ///All of the template object’s current parameters will be inherited by the
    ///new object in such a case.
    ///Optionally, some of the template’s parameters can be updated or new
    ///parameters added to the newly constructed object via the
    ///extension-specific parameters.
    video_session_parameters_template: VideoSessionParametersKHR,
    ///[`video_session`] is the video session object against which the video
    ///session parameters object is going to be created.
    video_session: VideoSessionKHR,
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain **must**
///   be either `NULL` or a pointer to a valid instance of
///   [`VideoDecodeH264SessionParametersAddInfoEXT`],
///   [`VideoDecodeH265SessionParametersAddInfoEXT`],
///   [`VideoEncodeH264SessionParametersAddInfoEXT`], or
///   [`VideoEncodeH265SessionParametersAddInfoEXT`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
///# Related
/// - [`VK_KHR_video_queue`]
/// - [`StructureType`]
/// - [`UpdateVideoSessionParametersKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VideoSessionParametersUpdateInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`update_sequence_count`] is the sequence number of the object update
    ///with parameters, starting from `1` and incrementing the value by one
    ///with each subsequent update.
    update_sequence_count: u32,
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
/// - [`video_session_parameters`] is [`crate::utils::Handle::null`] or a handle of a
///   [`VideoSessionParametersKHR`] object to be used for the processing of the video commands. If
///   [`crate::utils::Handle::null`], then no video session parameters apply to this command buffer
///   context.
/// - [`reference_slot_count`] is the number of reference slot entries provided in
///   [`p_reference_slots`].
/// - [`p_reference_slots`] is a pointer to an array of [`VideoReferenceSlotKHR`] structures
///   specifying reference slots, used within the video command context between this
///   [`CmdBeginVideoCodingKHR`] command and the [`CmdEndVideoCodingKHR`] commmand that follows.
///   Each reference slot provides a slot index and the [`VideoPictureResourceKHR`] specifying the
///   reference picture resource bound to this slot index. A slot index **must** not appear more
///   than once in [`p_reference_slots`] in a given command.
///# Description
///Valid Usage
/// - [`VideoBeginCodingInfoKHR`]::[`reference_slot_count`]**must** not exceed the value specified
///   in [`VideoSessionCreateInfoKHR::max_reference_pictures_slots_count`] when creating the video
///   session object that is being provided in [`video_session`]
/// - If [`video_session_parameters`] is not [`crate::utils::Handle::null`], it **must** have been
///   created using [`video_session`] as a parent object
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_BEGIN_CODING_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`flags`]**must** be `0`
/// - [`codec_quality_preset`]**must** be a valid combination of
///   [`VideoCodingQualityPresetFlagBitsKHR`] values
/// - [`codec_quality_preset`]**must** not be `0`
/// - [`video_session`]**must** be a valid [`VideoSessionKHR`] handle
/// - If [`video_session_parameters`] is not [`crate::utils::Handle::null`],
///   [`video_session_parameters`]**must** be a valid [`VideoSessionParametersKHR`] handle
/// - If [`reference_slot_count`] is not `0`, [`p_reference_slots`]**must** be a valid pointer to an
///   array of [`reference_slot_count`] valid [`VideoReferenceSlotKHR`] structures
/// - If [`video_session_parameters`] is a valid handle, it **must** have been created, allocated,
///   or retrieved from [`video_session`]
/// - Both of [`video_session`], and [`video_session_parameters`] that are valid handles of
///   non-ignored parameters **must** have been created, allocated, or retrieved from the same
///   [`Device`]
///# Related
/// - [`VK_KHR_video_queue`]
/// - [`StructureType`]
/// - [`VideoBeginCodingFlagsKHR`]
/// - [`VideoCodingQualityPresetFlagsKHR`]
/// - [`VideoReferenceSlotKHR`]
/// - [`VideoSessionKHR`]
/// - [`VideoSessionParametersKHR`]
/// - [`CmdBeginVideoCodingKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VideoBeginCodingInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: VideoBeginCodingFlagsKHR,
    ///[`codec_quality_preset`] is a bitmask of
    ///[`VideoCodingQualityPresetFlagBitsKHR`] specifying the Video Decode
    ///or Encode quality preset.
    codec_quality_preset: VideoCodingQualityPresetFlagsKHR,
    ///[`video_session`] is the video session object to be bound for the
    ///processing of the video commands.
    video_session: VideoSessionKHR,
    ///[`video_session_parameters`] is [`crate::utils::Handle::null`] or a handle of a
    ///[`VideoSessionParametersKHR`] object to be used for the processing
    ///of the video commands.
    ///If [`crate::utils::Handle::null`], then no video session parameters apply to this
    ///command buffer context.
    video_session_parameters: VideoSessionParametersKHR,
    ///[`reference_slot_count`] is the number of reference slot entries
    ///provided in [`p_reference_slots`].
    reference_slot_count: u32,
    ///[`p_reference_slots`] is a pointer to an array of
    ///[`VideoReferenceSlotKHR`] structures specifying reference slots,
    ///used within the video command context between this
    ///[`CmdBeginVideoCodingKHR`] command and the
    ///[`CmdEndVideoCodingKHR`] commmand that follows.
    ///Each reference slot provides a slot index and the
    ///[`VideoPictureResourceKHR`] specifying the reference picture
    ///resource bound to this slot index.
    ///A slot index **must** not appear more than once in [`p_reference_slots`] in
    ///a given command.
    p_reference_slots: *mut VideoReferenceSlotKHR<'lt>,
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_END_CODING_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`flags`]**must** be `0`
///# Related
/// - [`VK_KHR_video_queue`]
/// - [`StructureType`]
/// - [`VideoEndCodingFlagsKHR`]
/// - [`CmdEndVideoCodingKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VideoEndCodingInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: VideoEndCodingFlagsKHR,
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
///Valid Usage
/// - The first command buffer submitted for a newly created video session **must** set the
///   `VK_VIDEO_CODING_CONTROL_RESET_BIT_KHR` bit in [`VideoCodingControlInfoKHR`]::[`flags`] to
///   reset the session device context before any video decode or encode operations are performed on
///   the session.
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_CODING_CONTROL_INFO_KHR`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain **must**
///   be either `NULL` or a pointer to a valid instance of [`VideoEncodeRateControlInfoKHR`] or
///   [`VideoEncodeRateControlLayerInfoKHR`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
/// - [`flags`]**must** be a valid combination of [`VideoCodingControlFlagBitsKHR`] values
///# Related
/// - [`VK_KHR_video_queue`]
/// - [`StructureType`]
/// - [`VideoCodingControlFlagsKHR`]
/// - [`CmdControlVideoCodingKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VideoCodingControlInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`VideoCodingControlFlagsKHR`]
    ///specifying control flags.
    flags: VideoCodingControlFlagsKHR,
}
