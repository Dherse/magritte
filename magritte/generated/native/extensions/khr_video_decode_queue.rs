use crate::native::{
    extensions::khr_video_queue::{VideoPictureResourceKHR, VideoReferenceSlotKHR},
    vulkan1_0::{
        BaseInStructure, BaseOutStructure, Buffer, CommandBuffer, DeviceSize, Extent2D, Offset2D, StructureType,
    },
};
#[doc(alias = "VkVideoDecodeCapabilitiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoDecodeCapabilitiesKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    pub flags: VideoDecodeCapabilityFlagsKHR,
}
impl Default for VideoDecodeCapabilitiesKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::VideoDecodeCapabilitiesKhr,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkVideoDecodeInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoDecodeInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: VideoDecodeFlagsKHR,
    #[doc(alias = "codedOffset")]
    pub coded_offset: Offset2D,
    #[doc(alias = "codedExtent")]
    pub coded_extent: Extent2D,
    #[doc(alias = "srcBuffer")]
    pub src_buffer: Buffer,
    #[doc(alias = "srcBufferOffset")]
    pub src_buffer_offset: DeviceSize,
    #[doc(alias = "srcBufferRange")]
    pub src_buffer_range: DeviceSize,
    #[doc(alias = "dstPictureResource")]
    pub dst_picture_resource: VideoPictureResourceKHR,
    #[doc(alias = "pSetupReferenceSlot")]
    pub setup_reference_slot: *const VideoReferenceSlotKHR,
    #[doc(alias = "referenceSlotCount")]
    pub reference_slot_count: u32,
    #[doc(alias = "pReferenceSlots")]
    pub reference_slots: *const VideoReferenceSlotKHR,
}
impl Default for VideoDecodeInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::VideoDecodeInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            coded_offset: unsafe { std::mem::zeroed() },
            coded_extent: unsafe { std::mem::zeroed() },
            src_buffer: unsafe { std::mem::zeroed() },
            src_buffer_offset: unsafe { std::mem::zeroed() },
            src_buffer_range: unsafe { std::mem::zeroed() },
            dst_picture_resource: unsafe { std::mem::zeroed() },
            setup_reference_slot: unsafe { std::mem::zeroed() },
            reference_slot_count: unsafe { std::mem::zeroed() },
            reference_slots: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::khr_video_decode_queue::{
    VideoDecodeCapabilityFlagBitsKHR, VideoDecodeCapabilityFlagsKHR, VideoDecodeFlagBitsKHR, VideoDecodeFlagsKHR,
    KHR_VIDEO_DECODE_QUEUE_EXTENSION_NAME, KHR_VIDEO_DECODE_QUEUE_SPEC_VERSION,
};
#[doc(alias = "vkCmdDecodeVideoKHR")]
pub type FNCmdDecodeVideoKhr =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_frame_info: *const VideoDecodeInfoKHR);
