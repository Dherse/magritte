pub use crate::common::extensions::khr_video_decode_queue::{
    VideoDecodeCapabilityFlagBitsKHR, VideoDecodeCapabilityFlagsKHR, VideoDecodeFlagBitsKHR, VideoDecodeFlagsKHR,
    KHR_VIDEO_DECODE_QUEUE_EXTENSION_NAME, KHR_VIDEO_DECODE_QUEUE_SPEC_VERSION,
};
use crate::{
    context::Context,
    extensions::khr_video_queue::{VideoPictureResourceKHR, VideoReferenceSlotKHR},
    vulkan1_0::{Buffer, DeviceSize, Extent2D, Offset2D, StructureType},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
#[doc(alias = "VkVideoDecodeCapabilitiesKHR")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoDecodeCapabilitiesKHR {
    pub flags: VideoDecodeCapabilityFlagsKHR,
}
impl VideoDecodeCapabilitiesKHR {
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> VideoDecodeCapabilityFlagsKHR {
        self.flags
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for VideoDecodeCapabilitiesKHR {
    type LowLevel = crate::native::extensions::khr_video_decode_queue::VideoDecodeCapabilitiesKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_video_decode_queue::VideoDecodeCapabilitiesKHR {
            s_type: StructureType::VideoDecodeCapabilitiesKhr,
            p_next: std::ptr::null_mut(),
            flags: self.flags.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for VideoDecodeCapabilitiesKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
        }
    }
}
#[doc(alias = "VkVideoDecodeInfoKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoDecodeInfoKHR {
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
    pub setup_reference_slot: VideoReferenceSlotKHR,
    #[doc(alias = "pReferenceSlots")]
    pub reference_slots: SmallVec<[VideoReferenceSlotKHR; 8]>,
}
impl VideoDecodeInfoKHR {
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> VideoDecodeFlagsKHR {
        self.flags
    }
    ///Get a reference to the `coded_offset` field.
    pub fn coded_offset(&self) -> Offset2D {
        self.coded_offset
    }
    ///Get a reference to the `coded_extent` field.
    pub fn coded_extent(&self) -> Extent2D {
        self.coded_extent
    }
    ///Get a reference to the `src_buffer` field.
    pub fn src_buffer(&self) -> &Buffer {
        &self.src_buffer
    }
    ///Get a reference to the `src_buffer_offset` field.
    pub fn src_buffer_offset(&self) -> &DeviceSize {
        &self.src_buffer_offset
    }
    ///Get a reference to the `src_buffer_range` field.
    pub fn src_buffer_range(&self) -> &DeviceSize {
        &self.src_buffer_range
    }
    ///Get a reference to the `dst_picture_resource` field.
    pub fn dst_picture_resource(&self) -> &VideoPictureResourceKHR {
        &self.dst_picture_resource
    }
    ///Get a reference to the `setup_reference_slot` field.
    pub fn setup_reference_slot(&self) -> &VideoReferenceSlotKHR {
        &self.setup_reference_slot
    }
    ///Get a reference to the `reference_slots` field.
    pub fn reference_slots(&self) -> &SmallVec<[VideoReferenceSlotKHR; 8]> {
        &self.reference_slots
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut VideoDecodeFlagsKHR {
        &mut self.flags
    }
    ///Get a mutable reference to the `coded_offset` field.
    pub fn coded_offset_mut(&mut self) -> &mut Offset2D {
        &mut self.coded_offset
    }
    ///Get a mutable reference to the `coded_extent` field.
    pub fn coded_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.coded_extent
    }
    ///Get a mutable reference to the `src_buffer` field.
    pub fn src_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.src_buffer
    }
    ///Get a mutable reference to the `src_buffer_offset` field.
    pub fn src_buffer_offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.src_buffer_offset
    }
    ///Get a mutable reference to the `src_buffer_range` field.
    pub fn src_buffer_range_mut(&mut self) -> &mut DeviceSize {
        &mut self.src_buffer_range
    }
    ///Get a mutable reference to the `dst_picture_resource` field.
    pub fn dst_picture_resource_mut(&mut self) -> &mut VideoPictureResourceKHR {
        &mut self.dst_picture_resource
    }
    ///Get a mutable reference to the `setup_reference_slot` field.
    pub fn setup_reference_slot_mut(&mut self) -> &mut VideoReferenceSlotKHR {
        &mut self.setup_reference_slot
    }
    ///Get a mutable reference to the `reference_slots` field.
    pub fn reference_slots_mut(&mut self) -> &mut SmallVec<[VideoReferenceSlotKHR; 8]> {
        &mut self.reference_slots
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: VideoDecodeFlagsKHR) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `coded_offset` field.
    pub fn set_coded_offset(&mut self, coded_offset: Offset2D) -> &mut Self {
        self.coded_offset = coded_offset;
        self
    }
    ///Sets the `coded_extent` field.
    pub fn set_coded_extent(&mut self, coded_extent: Extent2D) -> &mut Self {
        self.coded_extent = coded_extent;
        self
    }
    ///Sets the `src_buffer` field.
    pub fn set_src_buffer(&mut self, src_buffer: Buffer) -> &mut Self {
        self.src_buffer = src_buffer;
        self
    }
    ///Sets the `src_buffer_offset` field.
    pub fn set_src_buffer_offset(&mut self, src_buffer_offset: DeviceSize) -> &mut Self {
        self.src_buffer_offset = src_buffer_offset;
        self
    }
    ///Sets the `src_buffer_range` field.
    pub fn set_src_buffer_range(&mut self, src_buffer_range: DeviceSize) -> &mut Self {
        self.src_buffer_range = src_buffer_range;
        self
    }
    ///Sets the `dst_picture_resource` field.
    pub fn set_dst_picture_resource(&mut self, dst_picture_resource: VideoPictureResourceKHR) -> &mut Self {
        self.dst_picture_resource = dst_picture_resource;
        self
    }
    ///Sets the `setup_reference_slot` field.
    pub fn set_setup_reference_slot(&mut self, setup_reference_slot: VideoReferenceSlotKHR) -> &mut Self {
        self.setup_reference_slot = setup_reference_slot;
        self
    }
    ///Sets the `reference_slots` field.
    pub fn set_reference_slots(&mut self, reference_slots: SmallVec<[VideoReferenceSlotKHR; 8]>) -> &mut Self {
        self.reference_slots = reference_slots;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: VideoDecodeFlagsKHR) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `coded_offset` field in a builder way.
    pub fn with_coded_offset(mut self, coded_offset: Offset2D) -> Self {
        self.coded_offset = coded_offset;
        self
    }
    ///Sets the `coded_extent` field in a builder way.
    pub fn with_coded_extent(mut self, coded_extent: Extent2D) -> Self {
        self.coded_extent = coded_extent;
        self
    }
    ///Sets the `src_buffer` field in a builder way.
    pub fn with_src_buffer(mut self, src_buffer: Buffer) -> Self {
        self.src_buffer = src_buffer;
        self
    }
    ///Sets the `src_buffer_offset` field in a builder way.
    pub fn with_src_buffer_offset(mut self, src_buffer_offset: DeviceSize) -> Self {
        self.src_buffer_offset = src_buffer_offset;
        self
    }
    ///Sets the `src_buffer_range` field in a builder way.
    pub fn with_src_buffer_range(mut self, src_buffer_range: DeviceSize) -> Self {
        self.src_buffer_range = src_buffer_range;
        self
    }
    ///Sets the `dst_picture_resource` field in a builder way.
    pub fn with_dst_picture_resource(mut self, dst_picture_resource: VideoPictureResourceKHR) -> Self {
        self.dst_picture_resource = dst_picture_resource;
        self
    }
    ///Sets the `setup_reference_slot` field in a builder way.
    pub fn with_setup_reference_slot(mut self, setup_reference_slot: VideoReferenceSlotKHR) -> Self {
        self.setup_reference_slot = setup_reference_slot;
        self
    }
    ///Sets the `reference_slots` field in a builder way.
    pub fn with_reference_slots(mut self, reference_slots: SmallVec<[VideoReferenceSlotKHR; 8]>) -> Self {
        self.reference_slots = reference_slots;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for VideoDecodeInfoKHR {
    type LowLevel = crate::native::extensions::khr_video_decode_queue::VideoDecodeInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_reference_slots = self.reference_slots.len() as u32;
        let reference_slots = bump
            .alloc_slice_fill_iter(self.reference_slots.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::extensions::khr_video_decode_queue::VideoDecodeInfoKHR {
            s_type: StructureType::VideoDecodeInfoKhr,
            p_next: std::ptr::null(),
            flags: self.flags.into_low_level(context, bump),
            coded_offset: self.coded_offset.into_low_level(context, bump),
            coded_extent: self.coded_extent.into_low_level(context, bump),
            src_buffer: self.src_buffer.into_low_level(context, bump),
            src_buffer_offset: self.src_buffer_offset.into_low_level(context, bump),
            src_buffer_range: self.src_buffer_range.into_low_level(context, bump),
            dst_picture_resource: self.dst_picture_resource.into_low_level(context, bump),
            setup_reference_slot: bump.alloc(self.setup_reference_slot.into_low_level(context, bump)) as *const _,
            reference_slot_count: len_reference_slots,
            reference_slots: reference_slots,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for VideoDecodeInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let reference_slots_len = value.reference_slot_count;
        let mut reference_slots = SmallVec::with_capacity(reference_slots_len as usize);
        for i in 0..reference_slots_len {
            reference_slots.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.reference_slots.add(i as usize).read(),
            ));
        }
        Self {
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            coded_offset: crate::conv::FromLowLevel::from_low_level(context, value.coded_offset),
            coded_extent: crate::conv::FromLowLevel::from_low_level(context, value.coded_extent),
            src_buffer: crate::conv::FromLowLevel::from_low_level(context, value.src_buffer),
            src_buffer_offset: crate::conv::FromLowLevel::from_low_level(context, value.src_buffer_offset),
            src_buffer_range: crate::conv::FromLowLevel::from_low_level(context, value.src_buffer_range),
            dst_picture_resource: crate::conv::FromLowLevel::from_low_level(context, value.dst_picture_resource),
            setup_reference_slot: crate::conv::FromLowLevel::from_low_level(context, *value.setup_reference_slot),
            reference_slots: reference_slots,
        }
    }
}
