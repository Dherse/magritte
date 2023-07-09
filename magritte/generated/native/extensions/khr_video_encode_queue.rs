use crate::native::{
    extensions::khr_video_queue::{VideoPictureResourceKHR, VideoReferenceSlotKHR},
    vulkan1_0::{BaseInStructure, Buffer, CommandBuffer, DeviceSize, Extent2D, StructureType},
};
#[doc(alias = "VkVideoEncodeInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: VideoEncodeFlagsKHR,
    #[doc(alias = "qualityLevel")]
    pub quality_level: u32,
    #[doc(alias = "codedExtent")]
    pub coded_extent: Extent2D,
    #[doc(alias = "dstBitstreamBuffer")]
    pub dst_bitstream_buffer: Buffer,
    #[doc(alias = "dstBitstreamBufferOffset")]
    pub dst_bitstream_buffer_offset: DeviceSize,
    #[doc(alias = "dstBitstreamBufferMaxRange")]
    pub dst_bitstream_buffer_max_range: DeviceSize,
    #[doc(alias = "srcPictureResource")]
    pub src_picture_resource: VideoPictureResourceKHR,
    #[doc(alias = "pSetupReferenceSlot")]
    pub setup_reference_slot: *const VideoReferenceSlotKHR,
    #[doc(alias = "referenceSlotCount")]
    pub reference_slot_count: u32,
    #[doc(alias = "pReferenceSlots")]
    pub reference_slots: *const VideoReferenceSlotKHR,
    #[doc(alias = "precedingExternallyEncodedBytes")]
    pub preceding_externally_encoded_bytes: u32,
}
impl Default for VideoEncodeInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::VideoEncodeInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            quality_level: unsafe { std::mem::zeroed() },
            coded_extent: unsafe { std::mem::zeroed() },
            dst_bitstream_buffer: unsafe { std::mem::zeroed() },
            dst_bitstream_buffer_offset: unsafe { std::mem::zeroed() },
            dst_bitstream_buffer_max_range: unsafe { std::mem::zeroed() },
            src_picture_resource: unsafe { std::mem::zeroed() },
            setup_reference_slot: unsafe { std::mem::zeroed() },
            reference_slot_count: unsafe { std::mem::zeroed() },
            reference_slots: unsafe { std::mem::zeroed() },
            preceding_externally_encoded_bytes: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkVideoEncodeRateControlInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeRateControlInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: VideoEncodeRateControlFlagsKHR,
    #[doc(alias = "rateControlMode")]
    pub rate_control_mode: VideoEncodeRateControlModeFlagBitsKHR,
    #[doc(alias = "layerCount")]
    pub layer_count: u8,
    #[doc(alias = "pLayerConfigs")]
    pub layer_configs: *const VideoEncodeRateControlLayerInfoKHR,
}
impl Default for VideoEncodeRateControlInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::VideoEncodeRateControlInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            rate_control_mode: unsafe { std::mem::zeroed() },
            layer_count: unsafe { std::mem::zeroed() },
            layer_configs: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkVideoEncodeRateControlLayerInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeRateControlLayerInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "averageBitrate")]
    pub average_bitrate: u32,
    #[doc(alias = "maxBitrate")]
    pub max_bitrate: u32,
    #[doc(alias = "frameRateNumerator")]
    pub frame_rate_numerator: u32,
    #[doc(alias = "frameRateDenominator")]
    pub frame_rate_denominator: u32,
    #[doc(alias = "virtualBufferSizeInMs")]
    pub virtual_buffer_size_in_ms: u32,
    #[doc(alias = "initialVirtualBufferSizeInMs")]
    pub initial_virtual_buffer_size_in_ms: u32,
}
impl Default for VideoEncodeRateControlLayerInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::VideoEncodeRateControlLayerInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            average_bitrate: unsafe { std::mem::zeroed() },
            max_bitrate: unsafe { std::mem::zeroed() },
            frame_rate_numerator: unsafe { std::mem::zeroed() },
            frame_rate_denominator: unsafe { std::mem::zeroed() },
            virtual_buffer_size_in_ms: unsafe { std::mem::zeroed() },
            initial_virtual_buffer_size_in_ms: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkVideoEncodeCapabilitiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeCapabilitiesKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: VideoEncodeCapabilityFlagsKHR,
    #[doc(alias = "rateControlModes")]
    pub rate_control_modes: VideoEncodeRateControlModeFlagsKHR,
    #[doc(alias = "rateControlLayerCount")]
    pub rate_control_layer_count: u8,
    #[doc(alias = "qualityLevelCount")]
    pub quality_level_count: u8,
    #[doc(alias = "inputImageDataFillAlignment")]
    pub input_image_data_fill_alignment: Extent2D,
}
impl Default for VideoEncodeCapabilitiesKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::VideoEncodeCapabilitiesKhr,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            rate_control_modes: unsafe { std::mem::zeroed() },
            rate_control_layer_count: unsafe { std::mem::zeroed() },
            quality_level_count: unsafe { std::mem::zeroed() },
            input_image_data_fill_alignment: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::khr_video_encode_queue::{
    VideoEncodeCapabilityFlagBitsKHR, VideoEncodeCapabilityFlagsKHR, VideoEncodeFlagBitsKHR, VideoEncodeFlagsKHR,
    VideoEncodeRateControlFlagBitsKHR, VideoEncodeRateControlFlagsKHR, VideoEncodeRateControlModeFlagBitsKHR,
    VideoEncodeRateControlModeFlagsKHR, KHR_VIDEO_ENCODE_QUEUE_EXTENSION_NAME, KHR_VIDEO_ENCODE_QUEUE_SPEC_VERSION,
};
#[doc(alias = "vkCmdEncodeVideoKHR")]
pub type FNCmdEncodeVideoKhr =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_encode_info: *const VideoEncodeInfoKHR);
