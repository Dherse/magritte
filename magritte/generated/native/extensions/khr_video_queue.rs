use crate::native::{
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, BaseOutStructure, Bool32, CommandBuffer, Device, DeviceMemory,
        DeviceSize, ExtensionProperties, Extent2D, Format, ImageUsageFlags, ImageView, Offset2D, PhysicalDevice,
        StructureType, VulkanResultCodes,
    },
    vulkan1_1::MemoryRequirements2,
};
#[doc(alias = "VkVideoQueueFamilyProperties2KHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoQueueFamilyProperties2KHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "videoCodecOperations")]
    pub video_codec_operations: VideoCodecOperationFlagsKHR,
}
impl Default for VideoQueueFamilyProperties2KHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::VideoQueueFamilyProperties2Khr,
            p_next: unsafe { std::mem::zeroed() },
            video_codec_operations: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkQueueFamilyQueryResultStatusProperties2KHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct QueueFamilyQueryResultStatusProperties2KHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    pub supported: Bool32,
}
impl Default for QueueFamilyQueryResultStatusProperties2KHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::QueueFamilyQueryResultStatusProperties2Khr,
            p_next: unsafe { std::mem::zeroed() },
            supported: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkVideoProfilesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoProfilesKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "profileCount")]
    pub profile_count: u32,
    #[doc(alias = "pProfiles")]
    pub profiles: *const VideoProfileKHR,
}
impl Default for VideoProfilesKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::VideoProfilesKhr,
            p_next: unsafe { std::mem::zeroed() },
            profile_count: unsafe { std::mem::zeroed() },
            profiles: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceVideoFormatInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVideoFormatInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "imageUsage")]
    pub image_usage: ImageUsageFlags,
    #[doc(alias = "pVideoProfiles")]
    pub video_profiles: *const VideoProfilesKHR,
}
impl Default for PhysicalDeviceVideoFormatInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceVideoFormatInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            image_usage: unsafe { std::mem::zeroed() },
            video_profiles: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkVideoFormatPropertiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoFormatPropertiesKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    pub format: Format,
}
impl Default for VideoFormatPropertiesKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::VideoFormatPropertiesKhr,
            p_next: unsafe { std::mem::zeroed() },
            format: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkVideoProfileKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoProfileKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "videoCodecOperation")]
    pub video_codec_operation: VideoCodecOperationFlagBitsKHR,
    #[doc(alias = "chromaSubsampling")]
    pub chroma_subsampling: VideoChromaSubsamplingFlagsKHR,
    #[doc(alias = "lumaBitDepth")]
    pub luma_bit_depth: VideoComponentBitDepthFlagsKHR,
    #[doc(alias = "chromaBitDepth")]
    pub chroma_bit_depth: VideoComponentBitDepthFlagsKHR,
}
impl Default for VideoProfileKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::VideoProfileKhr,
            p_next: unsafe { std::mem::zeroed() },
            video_codec_operation: unsafe { std::mem::zeroed() },
            chroma_subsampling: unsafe { std::mem::zeroed() },
            luma_bit_depth: unsafe { std::mem::zeroed() },
            chroma_bit_depth: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkVideoCapabilitiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoCapabilitiesKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "capabilityFlags")]
    pub capability_flags: VideoCapabilityFlagsKHR,
    #[doc(alias = "minBitstreamBufferOffsetAlignment")]
    pub min_bitstream_buffer_offset_alignment: DeviceSize,
    #[doc(alias = "minBitstreamBufferSizeAlignment")]
    pub min_bitstream_buffer_size_alignment: DeviceSize,
    #[doc(alias = "videoPictureExtentGranularity")]
    pub video_picture_extent_granularity: Extent2D,
    #[doc(alias = "minExtent")]
    pub min_extent: Extent2D,
    #[doc(alias = "maxExtent")]
    pub max_extent: Extent2D,
    #[doc(alias = "maxReferencePicturesSlotsCount")]
    pub max_reference_pictures_slots_count: u32,
    #[doc(alias = "maxReferencePicturesActiveCount")]
    pub max_reference_pictures_active_count: u32,
    #[doc(alias = "stdHeaderVersion")]
    pub std_header_version: ExtensionProperties,
}
impl Default for VideoCapabilitiesKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::VideoCapabilitiesKhr,
            p_next: unsafe { std::mem::zeroed() },
            capability_flags: unsafe { std::mem::zeroed() },
            min_bitstream_buffer_offset_alignment: unsafe { std::mem::zeroed() },
            min_bitstream_buffer_size_alignment: unsafe { std::mem::zeroed() },
            video_picture_extent_granularity: unsafe { std::mem::zeroed() },
            min_extent: unsafe { std::mem::zeroed() },
            max_extent: unsafe { std::mem::zeroed() },
            max_reference_pictures_slots_count: unsafe { std::mem::zeroed() },
            max_reference_pictures_active_count: unsafe { std::mem::zeroed() },
            std_header_version: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkVideoGetMemoryPropertiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoGetMemoryPropertiesKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "memoryBindIndex")]
    pub memory_bind_index: u32,
    #[doc(alias = "pMemoryRequirements")]
    pub memory_requirements: *mut MemoryRequirements2,
}
impl Default for VideoGetMemoryPropertiesKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::VideoGetMemoryPropertiesKhr,
            p_next: unsafe { std::mem::zeroed() },
            memory_bind_index: unsafe { std::mem::zeroed() },
            memory_requirements: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkVideoBindMemoryKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoBindMemoryKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "memoryBindIndex")]
    pub memory_bind_index: u32,
    pub memory: DeviceMemory,
    #[doc(alias = "memoryOffset")]
    pub memory_offset: DeviceSize,
    #[doc(alias = "memorySize")]
    pub memory_size: DeviceSize,
}
impl Default for VideoBindMemoryKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::VideoBindMemoryKhr,
            p_next: unsafe { std::mem::zeroed() },
            memory_bind_index: unsafe { std::mem::zeroed() },
            memory: unsafe { std::mem::zeroed() },
            memory_offset: unsafe { std::mem::zeroed() },
            memory_size: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkVideoPictureResourceKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoPictureResourceKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "codedOffset")]
    pub coded_offset: Offset2D,
    #[doc(alias = "codedExtent")]
    pub coded_extent: Extent2D,
    #[doc(alias = "baseArrayLayer")]
    pub base_array_layer: u32,
    #[doc(alias = "imageViewBinding")]
    pub image_view_binding: ImageView,
}
impl Default for VideoPictureResourceKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::VideoPictureResourceKhr,
            p_next: unsafe { std::mem::zeroed() },
            coded_offset: unsafe { std::mem::zeroed() },
            coded_extent: unsafe { std::mem::zeroed() },
            base_array_layer: unsafe { std::mem::zeroed() },
            image_view_binding: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkVideoReferenceSlotKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoReferenceSlotKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "slotIndex")]
    pub slot_index: i8,
    #[doc(alias = "pPictureResource")]
    pub picture_resource: *const VideoPictureResourceKHR,
}
impl Default for VideoReferenceSlotKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::VideoReferenceSlotKhr,
            p_next: unsafe { std::mem::zeroed() },
            slot_index: unsafe { std::mem::zeroed() },
            picture_resource: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkVideoSessionCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoSessionCreateInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "queueFamilyIndex")]
    pub queue_family_index: u32,
    pub flags: VideoSessionCreateFlagsKHR,
    #[doc(alias = "pVideoProfile")]
    pub video_profile: *const VideoProfileKHR,
    #[doc(alias = "pictureFormat")]
    pub picture_format: Format,
    #[doc(alias = "maxCodedExtent")]
    pub max_coded_extent: Extent2D,
    #[doc(alias = "referencePicturesFormat")]
    pub reference_pictures_format: Format,
    #[doc(alias = "maxReferencePicturesSlotsCount")]
    pub max_reference_pictures_slots_count: u32,
    #[doc(alias = "maxReferencePicturesActiveCount")]
    pub max_reference_pictures_active_count: u32,
    #[doc(alias = "pStdHeaderVersion")]
    pub std_header_version: *const ExtensionProperties,
}
impl Default for VideoSessionCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::VideoSessionCreateInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            queue_family_index: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            video_profile: unsafe { std::mem::zeroed() },
            picture_format: unsafe { std::mem::zeroed() },
            max_coded_extent: unsafe { std::mem::zeroed() },
            reference_pictures_format: unsafe { std::mem::zeroed() },
            max_reference_pictures_slots_count: unsafe { std::mem::zeroed() },
            max_reference_pictures_active_count: unsafe { std::mem::zeroed() },
            std_header_version: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkVideoSessionParametersCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoSessionParametersCreateInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "videoSessionParametersTemplate")]
    pub video_session_parameters_template: VideoSessionParametersKHR,
    #[doc(alias = "videoSession")]
    pub video_session: VideoSessionKHR,
}
impl Default for VideoSessionParametersCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::VideoSessionParametersCreateInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            video_session_parameters_template: unsafe { std::mem::zeroed() },
            video_session: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkVideoSessionParametersUpdateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoSessionParametersUpdateInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "updateSequenceCount")]
    pub update_sequence_count: u32,
}
impl Default for VideoSessionParametersUpdateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::VideoSessionParametersUpdateInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            update_sequence_count: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkVideoBeginCodingInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoBeginCodingInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: VideoBeginCodingFlagsKHR,
    #[doc(alias = "codecQualityPreset")]
    pub codec_quality_preset: VideoCodingQualityPresetFlagsKHR,
    #[doc(alias = "videoSession")]
    pub video_session: VideoSessionKHR,
    #[doc(alias = "videoSessionParameters")]
    pub video_session_parameters: VideoSessionParametersKHR,
    #[doc(alias = "referenceSlotCount")]
    pub reference_slot_count: u32,
    #[doc(alias = "pReferenceSlots")]
    pub reference_slots: *const VideoReferenceSlotKHR,
}
impl Default for VideoBeginCodingInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::VideoBeginCodingInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            codec_quality_preset: unsafe { std::mem::zeroed() },
            video_session: unsafe { std::mem::zeroed() },
            video_session_parameters: unsafe { std::mem::zeroed() },
            reference_slot_count: unsafe { std::mem::zeroed() },
            reference_slots: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkVideoEndCodingInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoEndCodingInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: VideoEndCodingFlagsKHR,
}
impl Default for VideoEndCodingInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::VideoEndCodingInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkVideoCodingControlInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoCodingControlInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: VideoCodingControlFlagsKHR,
}
impl Default for VideoCodingControlInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::VideoCodingControlInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkVideoSessionKHR")]
#[repr(transparent)]
pub struct VideoSessionKHR(u64);
impl VideoSessionKHR {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for VideoSessionKHR {
    fn default() -> Self {
        Self::null()
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkVideoSessionParametersKHR")]
#[repr(transparent)]
pub struct VideoSessionParametersKHR(u64);
impl VideoSessionParametersKHR {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for VideoSessionParametersKHR {
    fn default() -> Self {
        Self::null()
    }
}
pub use crate::common::extensions::khr_video_queue::{
    QueryResultStatusKHR, VideoBeginCodingFlagsKHR, VideoCapabilityFlagBitsKHR, VideoCapabilityFlagsKHR,
    VideoChromaSubsamplingFlagBitsKHR, VideoChromaSubsamplingFlagsKHR, VideoCodecOperationFlagBitsKHR,
    VideoCodecOperationFlagsKHR, VideoCodingControlFlagBitsKHR, VideoCodingControlFlagsKHR,
    VideoCodingQualityPresetFlagBitsKHR, VideoCodingQualityPresetFlagsKHR, VideoComponentBitDepthFlagBitsKHR,
    VideoComponentBitDepthFlagsKHR, VideoEndCodingFlagsKHR, VideoSessionCreateFlagBitsKHR, VideoSessionCreateFlagsKHR,
    KHR_VIDEO_QUEUE_EXTENSION_NAME, KHR_VIDEO_QUEUE_SPEC_VERSION,
};
#[doc(alias = "vkGetPhysicalDeviceVideoCapabilitiesKHR")]
pub type FNGetPhysicalDeviceVideoCapabilitiesKhr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_video_profile: *const VideoProfileKHR,
    p_capabilities: *mut VideoCapabilitiesKHR,
) -> VulkanResultCodes;
#[doc(alias = "vkGetPhysicalDeviceVideoFormatPropertiesKHR")]
pub type FNGetPhysicalDeviceVideoFormatPropertiesKhr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_video_format_info: *const PhysicalDeviceVideoFormatInfoKHR,
    p_video_format_property_count: *mut u32,
    p_video_format_properties: *mut VideoFormatPropertiesKHR,
) -> VulkanResultCodes;
#[doc(alias = "vkCreateVideoSessionKHR")]
pub type FNCreateVideoSessionKhr = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const VideoSessionCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_video_session: *mut VideoSessionKHR,
) -> VulkanResultCodes;
#[doc(alias = "vkDestroyVideoSessionKHR")]
pub type FNDestroyVideoSessionKhr =
    unsafe extern "system" fn(device: Device, video_session: VideoSessionKHR, p_allocator: *const AllocationCallbacks);
#[doc(alias = "vkCreateVideoSessionParametersKHR")]
pub type FNCreateVideoSessionParametersKhr = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const VideoSessionParametersCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_video_session_parameters: *mut VideoSessionParametersKHR,
) -> VulkanResultCodes;
#[doc(alias = "vkUpdateVideoSessionParametersKHR")]
pub type FNUpdateVideoSessionParametersKhr = unsafe extern "system" fn(
    device: Device,
    video_session_parameters: VideoSessionParametersKHR,
    p_update_info: *const VideoSessionParametersUpdateInfoKHR,
) -> VulkanResultCodes;
#[doc(alias = "vkDestroyVideoSessionParametersKHR")]
pub type FNDestroyVideoSessionParametersKhr = unsafe extern "system" fn(
    device: Device,
    video_session_parameters: VideoSessionParametersKHR,
    p_allocator: *const AllocationCallbacks,
);
#[doc(alias = "vkGetVideoSessionMemoryRequirementsKHR")]
pub type FNGetVideoSessionMemoryRequirementsKhr = unsafe extern "system" fn(
    device: Device,
    video_session: VideoSessionKHR,
    p_video_session_memory_requirements_count: *mut u32,
    p_video_session_memory_requirements: *mut VideoGetMemoryPropertiesKHR,
) -> VulkanResultCodes;
#[doc(alias = "vkBindVideoSessionMemoryKHR")]
pub type FNBindVideoSessionMemoryKhr = unsafe extern "system" fn(
    device: Device,
    video_session: VideoSessionKHR,
    video_session_bind_memory_count: u32,
    p_video_session_bind_memories: *const VideoBindMemoryKHR,
) -> VulkanResultCodes;
#[doc(alias = "vkCmdBeginVideoCodingKHR")]
pub type FNCmdBeginVideoCodingKhr =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_begin_info: *const VideoBeginCodingInfoKHR);
#[doc(alias = "vkCmdControlVideoCodingKHR")]
pub type FNCmdControlVideoCodingKhr =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_coding_control_info: *const VideoCodingControlInfoKHR);
#[doc(alias = "vkCmdEndVideoCodingKHR")]
pub type FNCmdEndVideoCodingKhr =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_end_coding_info: *const VideoEndCodingInfoKHR);
