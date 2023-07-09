pub use crate::common::extensions::khr_video_encode_queue::{
    VideoEncodeCapabilityFlagBitsKHR, VideoEncodeCapabilityFlagsKHR, VideoEncodeFlagBitsKHR, VideoEncodeFlagsKHR,
    VideoEncodeRateControlFlagBitsKHR, VideoEncodeRateControlFlagsKHR, VideoEncodeRateControlModeFlagBitsKHR,
    VideoEncodeRateControlModeFlagsKHR, KHR_VIDEO_ENCODE_QUEUE_EXTENSION_NAME, KHR_VIDEO_ENCODE_QUEUE_SPEC_VERSION,
};
use crate::{
    context::Context,
    extensions::khr_video_queue::{VideoPictureResourceKHR, VideoReferenceSlotKHR},
    vulkan1_0::{Buffer, DeviceSize, Extent2D, StructureType},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
#[doc(alias = "VkVideoEncodeInfoKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoEncodeInfoKHR {
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
    pub setup_reference_slot: VideoReferenceSlotKHR,
    #[doc(alias = "pReferenceSlots")]
    pub reference_slots: SmallVec<[VideoReferenceSlotKHR; 8]>,
    #[doc(alias = "precedingExternallyEncodedBytes")]
    pub preceding_externally_encoded_bytes: u32,
}
impl VideoEncodeInfoKHR {
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> VideoEncodeFlagsKHR {
        self.flags
    }
    ///Get a reference to the `quality_level` field.
    pub fn quality_level(&self) -> u32 {
        self.quality_level
    }
    ///Get a reference to the `coded_extent` field.
    pub fn coded_extent(&self) -> Extent2D {
        self.coded_extent
    }
    ///Get a reference to the `dst_bitstream_buffer` field.
    pub fn dst_bitstream_buffer(&self) -> &Buffer {
        &self.dst_bitstream_buffer
    }
    ///Get a reference to the `dst_bitstream_buffer_offset` field.
    pub fn dst_bitstream_buffer_offset(&self) -> &DeviceSize {
        &self.dst_bitstream_buffer_offset
    }
    ///Get a reference to the `dst_bitstream_buffer_max_range` field.
    pub fn dst_bitstream_buffer_max_range(&self) -> &DeviceSize {
        &self.dst_bitstream_buffer_max_range
    }
    ///Get a reference to the `src_picture_resource` field.
    pub fn src_picture_resource(&self) -> &VideoPictureResourceKHR {
        &self.src_picture_resource
    }
    ///Get a reference to the `setup_reference_slot` field.
    pub fn setup_reference_slot(&self) -> &VideoReferenceSlotKHR {
        &self.setup_reference_slot
    }
    ///Get a reference to the `reference_slots` field.
    pub fn reference_slots(&self) -> &SmallVec<[VideoReferenceSlotKHR; 8]> {
        &self.reference_slots
    }
    ///Get a reference to the `preceding_externally_encoded_bytes` field.
    pub fn preceding_externally_encoded_bytes(&self) -> u32 {
        self.preceding_externally_encoded_bytes
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut VideoEncodeFlagsKHR {
        &mut self.flags
    }
    ///Get a mutable reference to the `quality_level` field.
    pub fn quality_level_mut(&mut self) -> &mut u32 {
        &mut self.quality_level
    }
    ///Get a mutable reference to the `coded_extent` field.
    pub fn coded_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.coded_extent
    }
    ///Get a mutable reference to the `dst_bitstream_buffer` field.
    pub fn dst_bitstream_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.dst_bitstream_buffer
    }
    ///Get a mutable reference to the `dst_bitstream_buffer_offset` field.
    pub fn dst_bitstream_buffer_offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.dst_bitstream_buffer_offset
    }
    ///Get a mutable reference to the `dst_bitstream_buffer_max_range` field.
    pub fn dst_bitstream_buffer_max_range_mut(&mut self) -> &mut DeviceSize {
        &mut self.dst_bitstream_buffer_max_range
    }
    ///Get a mutable reference to the `src_picture_resource` field.
    pub fn src_picture_resource_mut(&mut self) -> &mut VideoPictureResourceKHR {
        &mut self.src_picture_resource
    }
    ///Get a mutable reference to the `setup_reference_slot` field.
    pub fn setup_reference_slot_mut(&mut self) -> &mut VideoReferenceSlotKHR {
        &mut self.setup_reference_slot
    }
    ///Get a mutable reference to the `reference_slots` field.
    pub fn reference_slots_mut(&mut self) -> &mut SmallVec<[VideoReferenceSlotKHR; 8]> {
        &mut self.reference_slots
    }
    ///Get a mutable reference to the `preceding_externally_encoded_bytes` field.
    pub fn preceding_externally_encoded_bytes_mut(&mut self) -> &mut u32 {
        &mut self.preceding_externally_encoded_bytes
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: VideoEncodeFlagsKHR) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `quality_level` field.
    pub fn set_quality_level(&mut self, quality_level: u32) -> &mut Self {
        self.quality_level = quality_level;
        self
    }
    ///Sets the `coded_extent` field.
    pub fn set_coded_extent(&mut self, coded_extent: Extent2D) -> &mut Self {
        self.coded_extent = coded_extent;
        self
    }
    ///Sets the `dst_bitstream_buffer` field.
    pub fn set_dst_bitstream_buffer(&mut self, dst_bitstream_buffer: Buffer) -> &mut Self {
        self.dst_bitstream_buffer = dst_bitstream_buffer;
        self
    }
    ///Sets the `dst_bitstream_buffer_offset` field.
    pub fn set_dst_bitstream_buffer_offset(&mut self, dst_bitstream_buffer_offset: DeviceSize) -> &mut Self {
        self.dst_bitstream_buffer_offset = dst_bitstream_buffer_offset;
        self
    }
    ///Sets the `dst_bitstream_buffer_max_range` field.
    pub fn set_dst_bitstream_buffer_max_range(&mut self, dst_bitstream_buffer_max_range: DeviceSize) -> &mut Self {
        self.dst_bitstream_buffer_max_range = dst_bitstream_buffer_max_range;
        self
    }
    ///Sets the `src_picture_resource` field.
    pub fn set_src_picture_resource(&mut self, src_picture_resource: VideoPictureResourceKHR) -> &mut Self {
        self.src_picture_resource = src_picture_resource;
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
    ///Sets the `preceding_externally_encoded_bytes` field.
    pub fn set_preceding_externally_encoded_bytes(&mut self, preceding_externally_encoded_bytes: u32) -> &mut Self {
        self.preceding_externally_encoded_bytes = preceding_externally_encoded_bytes;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: VideoEncodeFlagsKHR) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `quality_level` field in a builder way.
    pub fn with_quality_level(mut self, quality_level: u32) -> Self {
        self.quality_level = quality_level;
        self
    }
    ///Sets the `coded_extent` field in a builder way.
    pub fn with_coded_extent(mut self, coded_extent: Extent2D) -> Self {
        self.coded_extent = coded_extent;
        self
    }
    ///Sets the `dst_bitstream_buffer` field in a builder way.
    pub fn with_dst_bitstream_buffer(mut self, dst_bitstream_buffer: Buffer) -> Self {
        self.dst_bitstream_buffer = dst_bitstream_buffer;
        self
    }
    ///Sets the `dst_bitstream_buffer_offset` field in a builder way.
    pub fn with_dst_bitstream_buffer_offset(mut self, dst_bitstream_buffer_offset: DeviceSize) -> Self {
        self.dst_bitstream_buffer_offset = dst_bitstream_buffer_offset;
        self
    }
    ///Sets the `dst_bitstream_buffer_max_range` field in a builder way.
    pub fn with_dst_bitstream_buffer_max_range(mut self, dst_bitstream_buffer_max_range: DeviceSize) -> Self {
        self.dst_bitstream_buffer_max_range = dst_bitstream_buffer_max_range;
        self
    }
    ///Sets the `src_picture_resource` field in a builder way.
    pub fn with_src_picture_resource(mut self, src_picture_resource: VideoPictureResourceKHR) -> Self {
        self.src_picture_resource = src_picture_resource;
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
    ///Sets the `preceding_externally_encoded_bytes` field in a builder way.
    pub fn with_preceding_externally_encoded_bytes(mut self, preceding_externally_encoded_bytes: u32) -> Self {
        self.preceding_externally_encoded_bytes = preceding_externally_encoded_bytes;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for VideoEncodeInfoKHR {
    type LowLevel = crate::native::extensions::khr_video_encode_queue::VideoEncodeInfoKHR;
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
        crate::native::extensions::khr_video_encode_queue::VideoEncodeInfoKHR {
            s_type: StructureType::VideoEncodeInfoKhr,
            p_next: std::ptr::null(),
            flags: self.flags.into_low_level(context, bump),
            quality_level: self.quality_level.into_low_level(context, bump),
            coded_extent: self.coded_extent.into_low_level(context, bump),
            dst_bitstream_buffer: self.dst_bitstream_buffer.into_low_level(context, bump),
            dst_bitstream_buffer_offset: self.dst_bitstream_buffer_offset.into_low_level(context, bump),
            dst_bitstream_buffer_max_range: self.dst_bitstream_buffer_max_range.into_low_level(context, bump),
            src_picture_resource: self.src_picture_resource.into_low_level(context, bump),
            setup_reference_slot: bump.alloc(self.setup_reference_slot.into_low_level(context, bump)) as *const _,
            reference_slot_count: len_reference_slots,
            reference_slots: reference_slots,
            preceding_externally_encoded_bytes: self.preceding_externally_encoded_bytes.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for VideoEncodeInfoKHR {
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
            quality_level: crate::conv::FromLowLevel::from_low_level(context, value.quality_level),
            coded_extent: crate::conv::FromLowLevel::from_low_level(context, value.coded_extent),
            dst_bitstream_buffer: crate::conv::FromLowLevel::from_low_level(context, value.dst_bitstream_buffer),
            dst_bitstream_buffer_offset: crate::conv::FromLowLevel::from_low_level(
                context,
                value.dst_bitstream_buffer_offset,
            ),
            dst_bitstream_buffer_max_range: crate::conv::FromLowLevel::from_low_level(
                context,
                value.dst_bitstream_buffer_max_range,
            ),
            src_picture_resource: crate::conv::FromLowLevel::from_low_level(context, value.src_picture_resource),
            setup_reference_slot: crate::conv::FromLowLevel::from_low_level(context, *value.setup_reference_slot),
            reference_slots: reference_slots,
            preceding_externally_encoded_bytes: crate::conv::FromLowLevel::from_low_level(
                context,
                value.preceding_externally_encoded_bytes,
            ),
        }
    }
}
#[doc(alias = "VkVideoEncodeRateControlInfoKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoEncodeRateControlInfoKHR {
    pub flags: VideoEncodeRateControlFlagsKHR,
    #[doc(alias = "rateControlMode")]
    pub rate_control_mode: VideoEncodeRateControlModeFlagBitsKHR,
    #[doc(alias = "layerCount")]
    pub layer_count: u8,
    #[doc(alias = "pLayerConfigs")]
    pub layer_configs: SmallVec<[VideoEncodeRateControlLayerInfoKHR; 8]>,
}
impl VideoEncodeRateControlInfoKHR {
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> VideoEncodeRateControlFlagsKHR {
        self.flags
    }
    ///Get a reference to the `rate_control_mode` field.
    pub fn rate_control_mode(&self) -> VideoEncodeRateControlModeFlagBitsKHR {
        self.rate_control_mode
    }
    ///Get a reference to the `layer_count` field.
    pub fn layer_count(&self) -> u8 {
        self.layer_count
    }
    ///Get a reference to the `layer_configs` field.
    pub fn layer_configs(&self) -> &SmallVec<[VideoEncodeRateControlLayerInfoKHR; 8]> {
        &self.layer_configs
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut VideoEncodeRateControlFlagsKHR {
        &mut self.flags
    }
    ///Get a mutable reference to the `rate_control_mode` field.
    pub fn rate_control_mode_mut(&mut self) -> &mut VideoEncodeRateControlModeFlagBitsKHR {
        &mut self.rate_control_mode
    }
    ///Get a mutable reference to the `layer_count` field.
    pub fn layer_count_mut(&mut self) -> &mut u8 {
        &mut self.layer_count
    }
    ///Get a mutable reference to the `layer_configs` field.
    pub fn layer_configs_mut(&mut self) -> &mut SmallVec<[VideoEncodeRateControlLayerInfoKHR; 8]> {
        &mut self.layer_configs
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: VideoEncodeRateControlFlagsKHR) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `rate_control_mode` field.
    pub fn set_rate_control_mode(&mut self, rate_control_mode: VideoEncodeRateControlModeFlagBitsKHR) -> &mut Self {
        self.rate_control_mode = rate_control_mode;
        self
    }
    ///Sets the `layer_count` field.
    pub fn set_layer_count(&mut self, layer_count: u8) -> &mut Self {
        self.layer_count = layer_count;
        self
    }
    ///Sets the `layer_configs` field.
    pub fn set_layer_configs(&mut self, layer_configs: SmallVec<[VideoEncodeRateControlLayerInfoKHR; 8]>) -> &mut Self {
        self.layer_configs = layer_configs;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: VideoEncodeRateControlFlagsKHR) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `rate_control_mode` field in a builder way.
    pub fn with_rate_control_mode(mut self, rate_control_mode: VideoEncodeRateControlModeFlagBitsKHR) -> Self {
        self.rate_control_mode = rate_control_mode;
        self
    }
    ///Sets the `layer_count` field in a builder way.
    pub fn with_layer_count(mut self, layer_count: u8) -> Self {
        self.layer_count = layer_count;
        self
    }
    ///Sets the `layer_configs` field in a builder way.
    pub fn with_layer_configs(mut self, layer_configs: SmallVec<[VideoEncodeRateControlLayerInfoKHR; 8]>) -> Self {
        self.layer_configs = layer_configs;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for VideoEncodeRateControlInfoKHR {
    type LowLevel = crate::native::extensions::khr_video_encode_queue::VideoEncodeRateControlInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let layer_configs = bump
            .alloc_slice_fill_iter(self.layer_configs.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::extensions::khr_video_encode_queue::VideoEncodeRateControlInfoKHR {
            s_type: StructureType::VideoEncodeRateControlInfoKhr,
            p_next: std::ptr::null(),
            flags: self.flags.into_low_level(context, bump),
            rate_control_mode: self.rate_control_mode.into_low_level(context, bump),
            layer_count: self.layer_count.into_low_level(context, bump),
            layer_configs: layer_configs,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for VideoEncodeRateControlInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let layer_configs_len = value.layer_count;
        let mut layer_configs = SmallVec::with_capacity(layer_configs_len as usize);
        for i in 0..layer_configs_len {
            layer_configs.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.layer_configs.add(i as usize).read(),
            ));
        }
        Self {
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            rate_control_mode: crate::conv::FromLowLevel::from_low_level(context, value.rate_control_mode),
            layer_count: crate::conv::FromLowLevel::from_low_level(context, value.layer_count),
            layer_configs: layer_configs,
        }
    }
}
#[doc(alias = "VkVideoEncodeRateControlLayerInfoKHR")]
#[derive(Clone, PartialEq, Debug, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoEncodeRateControlLayerInfoKHR {
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
impl VideoEncodeRateControlLayerInfoKHR {
    ///Get a reference to the `average_bitrate` field.
    pub fn average_bitrate(&self) -> u32 {
        self.average_bitrate
    }
    ///Get a reference to the `max_bitrate` field.
    pub fn max_bitrate(&self) -> u32 {
        self.max_bitrate
    }
    ///Get a reference to the `frame_rate_numerator` field.
    pub fn frame_rate_numerator(&self) -> u32 {
        self.frame_rate_numerator
    }
    ///Get a reference to the `frame_rate_denominator` field.
    pub fn frame_rate_denominator(&self) -> u32 {
        self.frame_rate_denominator
    }
    ///Get a reference to the `virtual_buffer_size_in_ms` field.
    pub fn virtual_buffer_size_in_ms(&self) -> u32 {
        self.virtual_buffer_size_in_ms
    }
    ///Get a reference to the `initial_virtual_buffer_size_in_ms` field.
    pub fn initial_virtual_buffer_size_in_ms(&self) -> u32 {
        self.initial_virtual_buffer_size_in_ms
    }
    ///Get a mutable reference to the `average_bitrate` field.
    pub fn average_bitrate_mut(&mut self) -> &mut u32 {
        &mut self.average_bitrate
    }
    ///Get a mutable reference to the `max_bitrate` field.
    pub fn max_bitrate_mut(&mut self) -> &mut u32 {
        &mut self.max_bitrate
    }
    ///Get a mutable reference to the `frame_rate_numerator` field.
    pub fn frame_rate_numerator_mut(&mut self) -> &mut u32 {
        &mut self.frame_rate_numerator
    }
    ///Get a mutable reference to the `frame_rate_denominator` field.
    pub fn frame_rate_denominator_mut(&mut self) -> &mut u32 {
        &mut self.frame_rate_denominator
    }
    ///Get a mutable reference to the `virtual_buffer_size_in_ms` field.
    pub fn virtual_buffer_size_in_ms_mut(&mut self) -> &mut u32 {
        &mut self.virtual_buffer_size_in_ms
    }
    ///Get a mutable reference to the `initial_virtual_buffer_size_in_ms` field.
    pub fn initial_virtual_buffer_size_in_ms_mut(&mut self) -> &mut u32 {
        &mut self.initial_virtual_buffer_size_in_ms
    }
    ///Sets the `average_bitrate` field.
    pub fn set_average_bitrate(&mut self, average_bitrate: u32) -> &mut Self {
        self.average_bitrate = average_bitrate;
        self
    }
    ///Sets the `max_bitrate` field.
    pub fn set_max_bitrate(&mut self, max_bitrate: u32) -> &mut Self {
        self.max_bitrate = max_bitrate;
        self
    }
    ///Sets the `frame_rate_numerator` field.
    pub fn set_frame_rate_numerator(&mut self, frame_rate_numerator: u32) -> &mut Self {
        self.frame_rate_numerator = frame_rate_numerator;
        self
    }
    ///Sets the `frame_rate_denominator` field.
    pub fn set_frame_rate_denominator(&mut self, frame_rate_denominator: u32) -> &mut Self {
        self.frame_rate_denominator = frame_rate_denominator;
        self
    }
    ///Sets the `virtual_buffer_size_in_ms` field.
    pub fn set_virtual_buffer_size_in_ms(&mut self, virtual_buffer_size_in_ms: u32) -> &mut Self {
        self.virtual_buffer_size_in_ms = virtual_buffer_size_in_ms;
        self
    }
    ///Sets the `initial_virtual_buffer_size_in_ms` field.
    pub fn set_initial_virtual_buffer_size_in_ms(&mut self, initial_virtual_buffer_size_in_ms: u32) -> &mut Self {
        self.initial_virtual_buffer_size_in_ms = initial_virtual_buffer_size_in_ms;
        self
    }
    ///Sets the `average_bitrate` field in a builder way.
    pub fn with_average_bitrate(mut self, average_bitrate: u32) -> Self {
        self.average_bitrate = average_bitrate;
        self
    }
    ///Sets the `max_bitrate` field in a builder way.
    pub fn with_max_bitrate(mut self, max_bitrate: u32) -> Self {
        self.max_bitrate = max_bitrate;
        self
    }
    ///Sets the `frame_rate_numerator` field in a builder way.
    pub fn with_frame_rate_numerator(mut self, frame_rate_numerator: u32) -> Self {
        self.frame_rate_numerator = frame_rate_numerator;
        self
    }
    ///Sets the `frame_rate_denominator` field in a builder way.
    pub fn with_frame_rate_denominator(mut self, frame_rate_denominator: u32) -> Self {
        self.frame_rate_denominator = frame_rate_denominator;
        self
    }
    ///Sets the `virtual_buffer_size_in_ms` field in a builder way.
    pub fn with_virtual_buffer_size_in_ms(mut self, virtual_buffer_size_in_ms: u32) -> Self {
        self.virtual_buffer_size_in_ms = virtual_buffer_size_in_ms;
        self
    }
    ///Sets the `initial_virtual_buffer_size_in_ms` field in a builder way.
    pub fn with_initial_virtual_buffer_size_in_ms(mut self, initial_virtual_buffer_size_in_ms: u32) -> Self {
        self.initial_virtual_buffer_size_in_ms = initial_virtual_buffer_size_in_ms;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for VideoEncodeRateControlLayerInfoKHR {
    type LowLevel = crate::native::extensions::khr_video_encode_queue::VideoEncodeRateControlLayerInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_video_encode_queue::VideoEncodeRateControlLayerInfoKHR {
            s_type: StructureType::VideoEncodeRateControlLayerInfoKhr,
            p_next: std::ptr::null(),
            average_bitrate: self.average_bitrate.into_low_level(context, bump),
            max_bitrate: self.max_bitrate.into_low_level(context, bump),
            frame_rate_numerator: self.frame_rate_numerator.into_low_level(context, bump),
            frame_rate_denominator: self.frame_rate_denominator.into_low_level(context, bump),
            virtual_buffer_size_in_ms: self.virtual_buffer_size_in_ms.into_low_level(context, bump),
            initial_virtual_buffer_size_in_ms: self.initial_virtual_buffer_size_in_ms.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for VideoEncodeRateControlLayerInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            average_bitrate: crate::conv::FromLowLevel::from_low_level(context, value.average_bitrate),
            max_bitrate: crate::conv::FromLowLevel::from_low_level(context, value.max_bitrate),
            frame_rate_numerator: crate::conv::FromLowLevel::from_low_level(context, value.frame_rate_numerator),
            frame_rate_denominator: crate::conv::FromLowLevel::from_low_level(context, value.frame_rate_denominator),
            virtual_buffer_size_in_ms: crate::conv::FromLowLevel::from_low_level(
                context,
                value.virtual_buffer_size_in_ms,
            ),
            initial_virtual_buffer_size_in_ms: crate::conv::FromLowLevel::from_low_level(
                context,
                value.initial_virtual_buffer_size_in_ms,
            ),
        }
    }
}
#[doc(alias = "VkVideoEncodeCapabilitiesKHR")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoEncodeCapabilitiesKHR {
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
impl VideoEncodeCapabilitiesKHR {
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> VideoEncodeCapabilityFlagsKHR {
        self.flags
    }
    ///Get a reference to the `rate_control_modes` field.
    pub fn rate_control_modes(&self) -> VideoEncodeRateControlModeFlagsKHR {
        self.rate_control_modes
    }
    ///Get a reference to the `rate_control_layer_count` field.
    pub fn rate_control_layer_count(&self) -> u8 {
        self.rate_control_layer_count
    }
    ///Get a reference to the `quality_level_count` field.
    pub fn quality_level_count(&self) -> u8 {
        self.quality_level_count
    }
    ///Get a reference to the `input_image_data_fill_alignment` field.
    pub fn input_image_data_fill_alignment(&self) -> Extent2D {
        self.input_image_data_fill_alignment
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut VideoEncodeCapabilityFlagsKHR {
        &mut self.flags
    }
    ///Get a mutable reference to the `rate_control_modes` field.
    pub fn rate_control_modes_mut(&mut self) -> &mut VideoEncodeRateControlModeFlagsKHR {
        &mut self.rate_control_modes
    }
    ///Get a mutable reference to the `rate_control_layer_count` field.
    pub fn rate_control_layer_count_mut(&mut self) -> &mut u8 {
        &mut self.rate_control_layer_count
    }
    ///Get a mutable reference to the `quality_level_count` field.
    pub fn quality_level_count_mut(&mut self) -> &mut u8 {
        &mut self.quality_level_count
    }
    ///Get a mutable reference to the `input_image_data_fill_alignment` field.
    pub fn input_image_data_fill_alignment_mut(&mut self) -> &mut Extent2D {
        &mut self.input_image_data_fill_alignment
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: VideoEncodeCapabilityFlagsKHR) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `rate_control_modes` field.
    pub fn set_rate_control_modes(&mut self, rate_control_modes: VideoEncodeRateControlModeFlagsKHR) -> &mut Self {
        self.rate_control_modes = rate_control_modes;
        self
    }
    ///Sets the `rate_control_layer_count` field.
    pub fn set_rate_control_layer_count(&mut self, rate_control_layer_count: u8) -> &mut Self {
        self.rate_control_layer_count = rate_control_layer_count;
        self
    }
    ///Sets the `quality_level_count` field.
    pub fn set_quality_level_count(&mut self, quality_level_count: u8) -> &mut Self {
        self.quality_level_count = quality_level_count;
        self
    }
    ///Sets the `input_image_data_fill_alignment` field.
    pub fn set_input_image_data_fill_alignment(&mut self, input_image_data_fill_alignment: Extent2D) -> &mut Self {
        self.input_image_data_fill_alignment = input_image_data_fill_alignment;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: VideoEncodeCapabilityFlagsKHR) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `rate_control_modes` field in a builder way.
    pub fn with_rate_control_modes(mut self, rate_control_modes: VideoEncodeRateControlModeFlagsKHR) -> Self {
        self.rate_control_modes = rate_control_modes;
        self
    }
    ///Sets the `rate_control_layer_count` field in a builder way.
    pub fn with_rate_control_layer_count(mut self, rate_control_layer_count: u8) -> Self {
        self.rate_control_layer_count = rate_control_layer_count;
        self
    }
    ///Sets the `quality_level_count` field in a builder way.
    pub fn with_quality_level_count(mut self, quality_level_count: u8) -> Self {
        self.quality_level_count = quality_level_count;
        self
    }
    ///Sets the `input_image_data_fill_alignment` field in a builder way.
    pub fn with_input_image_data_fill_alignment(mut self, input_image_data_fill_alignment: Extent2D) -> Self {
        self.input_image_data_fill_alignment = input_image_data_fill_alignment;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for VideoEncodeCapabilitiesKHR {
    type LowLevel = crate::native::extensions::khr_video_encode_queue::VideoEncodeCapabilitiesKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_video_encode_queue::VideoEncodeCapabilitiesKHR {
            s_type: StructureType::VideoEncodeCapabilitiesKhr,
            p_next: std::ptr::null(),
            flags: self.flags.into_low_level(context, bump),
            rate_control_modes: self.rate_control_modes.into_low_level(context, bump),
            rate_control_layer_count: self.rate_control_layer_count.into_low_level(context, bump),
            quality_level_count: self.quality_level_count.into_low_level(context, bump),
            input_image_data_fill_alignment: self.input_image_data_fill_alignment.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for VideoEncodeCapabilitiesKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            rate_control_modes: crate::conv::FromLowLevel::from_low_level(context, value.rate_control_modes),
            rate_control_layer_count: crate::conv::FromLowLevel::from_low_level(
                context,
                value.rate_control_layer_count,
            ),
            quality_level_count: crate::conv::FromLowLevel::from_low_level(context, value.quality_level_count),
            input_image_data_fill_alignment: crate::conv::FromLowLevel::from_low_level(
                context,
                value.input_image_data_fill_alignment,
            ),
        }
    }
}
