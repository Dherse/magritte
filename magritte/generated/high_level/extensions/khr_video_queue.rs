pub use crate::common::extensions::khr_video_queue::{
    QueryResultStatusKHR, VideoBeginCodingFlagsKHR, VideoCapabilityFlagBitsKHR, VideoCapabilityFlagsKHR,
    VideoChromaSubsamplingFlagBitsKHR, VideoChromaSubsamplingFlagsKHR, VideoCodecOperationFlagBitsKHR,
    VideoCodecOperationFlagsKHR, VideoCodingControlFlagBitsKHR, VideoCodingControlFlagsKHR,
    VideoCodingQualityPresetFlagBitsKHR, VideoCodingQualityPresetFlagsKHR, VideoComponentBitDepthFlagBitsKHR,
    VideoComponentBitDepthFlagsKHR, VideoEndCodingFlagsKHR, VideoSessionCreateFlagBitsKHR, VideoSessionCreateFlagsKHR,
    KHR_VIDEO_QUEUE_EXTENSION_NAME, KHR_VIDEO_QUEUE_SPEC_VERSION,
};
#[cfg(feature = "VK_KHR_video_decode_queue")]
use crate::extensions::khr_video_decode_queue::VideoDecodeCapabilitiesKHR;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::extensions::khr_video_encode_queue::VideoEncodeCapabilitiesKHR;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::extensions::khr_video_encode_queue::VideoEncodeRateControlInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::extensions::khr_video_encode_queue::VideoEncodeRateControlLayerInfoKHR;
use crate::{
    context::{Container, Context, Error, ObjectId},
    vulkan1_0::{
        DeviceMemory, DeviceSize, ExtensionProperties, Extent2D, Format, ImageUsageFlags, ImageView, Offset2D,
        StructureType,
    },
    vulkan1_1::MemoryRequirements2,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
#[doc(alias = "VkVideoQueueFamilyProperties2KHR")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoQueueFamilyProperties2KHR {
    #[doc(alias = "videoCodecOperations")]
    pub video_codec_operations: VideoCodecOperationFlagsKHR,
}
impl VideoQueueFamilyProperties2KHR {
    ///Get a reference to the `video_codec_operations` field.
    pub fn video_codec_operations(&self) -> VideoCodecOperationFlagsKHR {
        self.video_codec_operations
    }
    ///Get a mutable reference to the `video_codec_operations` field.
    pub fn video_codec_operations_mut(&mut self) -> &mut VideoCodecOperationFlagsKHR {
        &mut self.video_codec_operations
    }
    ///Sets the `video_codec_operations` field.
    pub fn set_video_codec_operations(&mut self, video_codec_operations: VideoCodecOperationFlagsKHR) -> &mut Self {
        self.video_codec_operations = video_codec_operations;
        self
    }
    ///Sets the `video_codec_operations` field in a builder way.
    pub fn with_video_codec_operations(mut self, video_codec_operations: VideoCodecOperationFlagsKHR) -> Self {
        self.video_codec_operations = video_codec_operations;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for VideoQueueFamilyProperties2KHR {
    type LowLevel = crate::native::extensions::khr_video_queue::VideoQueueFamilyProperties2KHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_video_queue::VideoQueueFamilyProperties2KHR {
            s_type: StructureType::VideoQueueFamilyProperties2Khr,
            p_next: std::ptr::null_mut(),
            video_codec_operations: self.video_codec_operations.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for VideoQueueFamilyProperties2KHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            video_codec_operations: crate::conv::FromLowLevel::from_low_level(context, value.video_codec_operations),
        }
    }
}
#[doc(alias = "VkQueueFamilyQueryResultStatusProperties2KHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueueFamilyQueryResultStatusProperties2KHR {
    pub supported: bool,
}
impl QueueFamilyQueryResultStatusProperties2KHR {
    ///Get a reference to the `supported` field.
    pub fn supported(&self) -> &bool {
        &self.supported
    }
    ///Get a mutable reference to the `supported` field.
    pub fn supported_mut(&mut self) -> &mut bool {
        &mut self.supported
    }
    ///Sets the `supported` field.
    pub fn set_supported(&mut self, supported: bool) -> &mut Self {
        self.supported = supported;
        self
    }
    ///Sets the `supported` field in a builder way.
    pub fn with_supported(mut self, supported: bool) -> Self {
        self.supported = supported;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for QueueFamilyQueryResultStatusProperties2KHR {
    type LowLevel = crate::native::extensions::khr_video_queue::QueueFamilyQueryResultStatusProperties2KHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_video_queue::QueueFamilyQueryResultStatusProperties2KHR {
            s_type: StructureType::QueueFamilyQueryResultStatusProperties2Khr,
            p_next: std::ptr::null_mut(),
            supported: self.supported.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for QueueFamilyQueryResultStatusProperties2KHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            supported: crate::conv::FromLowLevel::from_low_level(context, value.supported),
        }
    }
}
#[doc(alias = "VkVideoProfilesKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoProfilesKHR {
    #[doc(alias = "pProfiles")]
    pub profiles: SmallVec<[VideoProfileKHR; 8]>,
}
impl VideoProfilesKHR {
    ///Get a reference to the `profiles` field.
    pub fn profiles(&self) -> &SmallVec<[VideoProfileKHR; 8]> {
        &self.profiles
    }
    ///Get a mutable reference to the `profiles` field.
    pub fn profiles_mut(&mut self) -> &mut SmallVec<[VideoProfileKHR; 8]> {
        &mut self.profiles
    }
    ///Sets the `profiles` field.
    pub fn set_profiles(&mut self, profiles: SmallVec<[VideoProfileKHR; 8]>) -> &mut Self {
        self.profiles = profiles;
        self
    }
    ///Sets the `profiles` field in a builder way.
    pub fn with_profiles(mut self, profiles: SmallVec<[VideoProfileKHR; 8]>) -> Self {
        self.profiles = profiles;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for VideoProfilesKHR {
    type LowLevel = crate::native::extensions::khr_video_queue::VideoProfilesKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_profiles = self.profiles.len() as u32;
        let profiles = bump
            .alloc_slice_fill_iter(self.profiles.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::extensions::khr_video_queue::VideoProfilesKHR {
            s_type: StructureType::VideoProfilesKhr,
            p_next: std::ptr::null_mut(),
            profile_count: len_profiles,
            profiles: profiles,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for VideoProfilesKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let profiles_len = value.profile_count;
        let mut profiles = SmallVec::with_capacity(profiles_len as usize);
        for i in 0..profiles_len {
            profiles.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.profiles.add(i as usize).read(),
            ));
        }
        Self { profiles: profiles }
    }
}
#[doc(alias = "VkPhysicalDeviceVideoFormatInfoKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceVideoFormatInfoKHR {
    #[doc(alias = "imageUsage")]
    pub image_usage: ImageUsageFlags,
    #[doc(alias = "pVideoProfiles")]
    pub video_profiles: VideoProfilesKHR,
}
impl PhysicalDeviceVideoFormatInfoKHR {
    ///Get a reference to the `image_usage` field.
    pub fn image_usage(&self) -> ImageUsageFlags {
        self.image_usage
    }
    ///Get a reference to the `video_profiles` field.
    pub fn video_profiles(&self) -> &VideoProfilesKHR {
        &self.video_profiles
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceVideoFormatInfoKHR {
    type LowLevel = crate::native::extensions::khr_video_queue::PhysicalDeviceVideoFormatInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_video_queue::PhysicalDeviceVideoFormatInfoKHR {
            s_type: StructureType::PhysicalDeviceVideoFormatInfoKhr,
            p_next: std::ptr::null_mut(),
            image_usage: self.image_usage.into_low_level(context, bump),
            video_profiles: bump.alloc(self.video_profiles.into_low_level(context, bump)) as *const _,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceVideoFormatInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            image_usage: crate::conv::FromLowLevel::from_low_level(context, value.image_usage),
            video_profiles: crate::conv::FromLowLevel::from_low_level(context, *value.video_profiles),
        }
    }
}
#[doc(alias = "VkVideoFormatPropertiesKHR")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoFormatPropertiesKHR {
    pub format: Format,
}
impl VideoFormatPropertiesKHR {
    ///Get a reference to the `format` field.
    pub fn format(&self) -> Format {
        self.format
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for VideoFormatPropertiesKHR {
    type LowLevel = crate::native::extensions::khr_video_queue::VideoFormatPropertiesKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_video_queue::VideoFormatPropertiesKHR {
            s_type: StructureType::VideoFormatPropertiesKhr,
            p_next: std::ptr::null_mut(),
            format: self.format.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for VideoFormatPropertiesKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            format: crate::conv::FromLowLevel::from_low_level(context, value.format),
        }
    }
}
#[doc(alias = "VkVideoProfileKHR")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoProfileKHR {
    #[doc(alias = "videoCodecOperation")]
    pub video_codec_operation: VideoCodecOperationFlagBitsKHR,
    #[doc(alias = "chromaSubsampling")]
    pub chroma_subsampling: VideoChromaSubsamplingFlagsKHR,
    #[doc(alias = "lumaBitDepth")]
    pub luma_bit_depth: VideoComponentBitDepthFlagsKHR,
    #[doc(alias = "chromaBitDepth")]
    pub chroma_bit_depth: VideoComponentBitDepthFlagsKHR,
}
impl VideoProfileKHR {
    ///Get a reference to the `video_codec_operation` field.
    pub fn video_codec_operation(&self) -> VideoCodecOperationFlagBitsKHR {
        self.video_codec_operation
    }
    ///Get a reference to the `chroma_subsampling` field.
    pub fn chroma_subsampling(&self) -> VideoChromaSubsamplingFlagsKHR {
        self.chroma_subsampling
    }
    ///Get a reference to the `luma_bit_depth` field.
    pub fn luma_bit_depth(&self) -> VideoComponentBitDepthFlagsKHR {
        self.luma_bit_depth
    }
    ///Get a reference to the `chroma_bit_depth` field.
    pub fn chroma_bit_depth(&self) -> VideoComponentBitDepthFlagsKHR {
        self.chroma_bit_depth
    }
    ///Get a mutable reference to the `video_codec_operation` field.
    pub fn video_codec_operation_mut(&mut self) -> &mut VideoCodecOperationFlagBitsKHR {
        &mut self.video_codec_operation
    }
    ///Get a mutable reference to the `chroma_subsampling` field.
    pub fn chroma_subsampling_mut(&mut self) -> &mut VideoChromaSubsamplingFlagsKHR {
        &mut self.chroma_subsampling
    }
    ///Get a mutable reference to the `luma_bit_depth` field.
    pub fn luma_bit_depth_mut(&mut self) -> &mut VideoComponentBitDepthFlagsKHR {
        &mut self.luma_bit_depth
    }
    ///Get a mutable reference to the `chroma_bit_depth` field.
    pub fn chroma_bit_depth_mut(&mut self) -> &mut VideoComponentBitDepthFlagsKHR {
        &mut self.chroma_bit_depth
    }
    ///Sets the `video_codec_operation` field.
    pub fn set_video_codec_operation(&mut self, video_codec_operation: VideoCodecOperationFlagBitsKHR) -> &mut Self {
        self.video_codec_operation = video_codec_operation;
        self
    }
    ///Sets the `chroma_subsampling` field.
    pub fn set_chroma_subsampling(&mut self, chroma_subsampling: VideoChromaSubsamplingFlagsKHR) -> &mut Self {
        self.chroma_subsampling = chroma_subsampling;
        self
    }
    ///Sets the `luma_bit_depth` field.
    pub fn set_luma_bit_depth(&mut self, luma_bit_depth: VideoComponentBitDepthFlagsKHR) -> &mut Self {
        self.luma_bit_depth = luma_bit_depth;
        self
    }
    ///Sets the `chroma_bit_depth` field.
    pub fn set_chroma_bit_depth(&mut self, chroma_bit_depth: VideoComponentBitDepthFlagsKHR) -> &mut Self {
        self.chroma_bit_depth = chroma_bit_depth;
        self
    }
    ///Sets the `video_codec_operation` field in a builder way.
    pub fn with_video_codec_operation(mut self, video_codec_operation: VideoCodecOperationFlagBitsKHR) -> Self {
        self.video_codec_operation = video_codec_operation;
        self
    }
    ///Sets the `chroma_subsampling` field in a builder way.
    pub fn with_chroma_subsampling(mut self, chroma_subsampling: VideoChromaSubsamplingFlagsKHR) -> Self {
        self.chroma_subsampling = chroma_subsampling;
        self
    }
    ///Sets the `luma_bit_depth` field in a builder way.
    pub fn with_luma_bit_depth(mut self, luma_bit_depth: VideoComponentBitDepthFlagsKHR) -> Self {
        self.luma_bit_depth = luma_bit_depth;
        self
    }
    ///Sets the `chroma_bit_depth` field in a builder way.
    pub fn with_chroma_bit_depth(mut self, chroma_bit_depth: VideoComponentBitDepthFlagsKHR) -> Self {
        self.chroma_bit_depth = chroma_bit_depth;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for VideoProfileKHR {
    type LowLevel = crate::native::extensions::khr_video_queue::VideoProfileKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_video_queue::VideoProfileKHR {
            s_type: StructureType::VideoProfileKhr,
            p_next: std::ptr::null_mut(),
            video_codec_operation: self.video_codec_operation.into_low_level(context, bump),
            chroma_subsampling: self.chroma_subsampling.into_low_level(context, bump),
            luma_bit_depth: self.luma_bit_depth.into_low_level(context, bump),
            chroma_bit_depth: self.chroma_bit_depth.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for VideoProfileKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            video_codec_operation: crate::conv::FromLowLevel::from_low_level(context, value.video_codec_operation),
            chroma_subsampling: crate::conv::FromLowLevel::from_low_level(context, value.chroma_subsampling),
            luma_bit_depth: crate::conv::FromLowLevel::from_low_level(context, value.luma_bit_depth),
            chroma_bit_depth: crate::conv::FromLowLevel::from_low_level(context, value.chroma_bit_depth),
        }
    }
}
#[doc(alias = "VkVideoCapabilitiesKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoCapabilitiesKHR {
    #[doc(alias = "pNext")]
    pub extensions: SmallVec<[VideoCapabilitiesKHRExtension; 1]>,
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
impl VideoCapabilitiesKHR {
    ///Adds an extension to the struct
    pub fn with_extension(mut self, ext: impl Into<VideoCapabilitiesKHRExtension>) -> Self {
        self.extensions.push(ext.into());
        self
    }
    ///Get a reference to the `extensions` field.
    pub fn extensions(&self) -> &SmallVec<[VideoCapabilitiesKHRExtension; 1]> {
        &self.extensions
    }
    ///Get a reference to the `capability_flags` field.
    pub fn capability_flags(&self) -> VideoCapabilityFlagsKHR {
        self.capability_flags
    }
    ///Get a reference to the `min_bitstream_buffer_offset_alignment` field.
    pub fn min_bitstream_buffer_offset_alignment(&self) -> &DeviceSize {
        &self.min_bitstream_buffer_offset_alignment
    }
    ///Get a reference to the `min_bitstream_buffer_size_alignment` field.
    pub fn min_bitstream_buffer_size_alignment(&self) -> &DeviceSize {
        &self.min_bitstream_buffer_size_alignment
    }
    ///Get a reference to the `video_picture_extent_granularity` field.
    pub fn video_picture_extent_granularity(&self) -> Extent2D {
        self.video_picture_extent_granularity
    }
    ///Get a reference to the `min_extent` field.
    pub fn min_extent(&self) -> Extent2D {
        self.min_extent
    }
    ///Get a reference to the `max_extent` field.
    pub fn max_extent(&self) -> Extent2D {
        self.max_extent
    }
    ///Get a reference to the `max_reference_pictures_slots_count` field.
    pub fn max_reference_pictures_slots_count(&self) -> u32 {
        self.max_reference_pictures_slots_count
    }
    ///Get a reference to the `max_reference_pictures_active_count` field.
    pub fn max_reference_pictures_active_count(&self) -> u32 {
        self.max_reference_pictures_active_count
    }
    ///Get a reference to the `std_header_version` field.
    pub fn std_header_version(&self) -> &ExtensionProperties {
        &self.std_header_version
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for VideoCapabilitiesKHR {
    type LowLevel = crate::native::extensions::khr_video_queue::VideoCapabilitiesKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let mut next = std::ptr::null_mut();
        let mut extensions = self.extensions.iter();
        while let Some(ext) = extensions.next() {
            let ext = ext.into_low_level(context, bump);
            (*ext).next = next;
            next = ext;
        }
        crate::native::extensions::khr_video_queue::VideoCapabilitiesKHR {
            s_type: StructureType::VideoCapabilitiesKhr,
            p_next: next,
            capability_flags: self.capability_flags.into_low_level(context, bump),
            min_bitstream_buffer_offset_alignment: self
                .min_bitstream_buffer_offset_alignment
                .into_low_level(context, bump),
            min_bitstream_buffer_size_alignment: self.min_bitstream_buffer_size_alignment.into_low_level(context, bump),
            video_picture_extent_granularity: self.video_picture_extent_granularity.into_low_level(context, bump),
            min_extent: self.min_extent.into_low_level(context, bump),
            max_extent: self.max_extent.into_low_level(context, bump),
            max_reference_pictures_slots_count: self.max_reference_pictures_slots_count.into_low_level(context, bump),
            max_reference_pictures_active_count: self.max_reference_pictures_active_count.into_low_level(context, bump),
            std_header_version: self.std_header_version.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for VideoCapabilitiesKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let mut next = value.p_next;
        let mut extensions = SmallVec::new();
        while !next.is_null() {
            extensions.push(crate::conv::FromLowLevel::from_low_level(context, next));
            next = std::ptr::read(next).next;
        }
        Self {
            extensions: extensions,
            capability_flags: crate::conv::FromLowLevel::from_low_level(context, value.capability_flags),
            min_bitstream_buffer_offset_alignment: crate::conv::FromLowLevel::from_low_level(
                context,
                value.min_bitstream_buffer_offset_alignment,
            ),
            min_bitstream_buffer_size_alignment: crate::conv::FromLowLevel::from_low_level(
                context,
                value.min_bitstream_buffer_size_alignment,
            ),
            video_picture_extent_granularity: crate::conv::FromLowLevel::from_low_level(
                context,
                value.video_picture_extent_granularity,
            ),
            min_extent: crate::conv::FromLowLevel::from_low_level(context, value.min_extent),
            max_extent: crate::conv::FromLowLevel::from_low_level(context, value.max_extent),
            max_reference_pictures_slots_count: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_reference_pictures_slots_count,
            ),
            max_reference_pictures_active_count: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_reference_pictures_active_count,
            ),
            std_header_version: crate::conv::FromLowLevel::from_low_level(context, value.std_header_version),
        }
    }
}
#[derive(Clone, PartialEq, Debug, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
///Extensions for [`VideoCapabilitiesKHR`]
pub enum VideoCapabilitiesKHRExtension {
    #[cfg(feature = "VK_KHR_video_decode_queue")]
    ///Contains a type [`VideoDecodeCapabilitiesKHR`] for extending [`VideoCapabilitiesKHR`]
    VideoDecodeCapabilitiesKHR(VideoDecodeCapabilitiesKHR),
    #[cfg(feature = "VK_KHR_video_encode_queue")]
    ///Contains a type [`VideoEncodeCapabilitiesKHR`] for extending [`VideoCapabilitiesKHR`]
    VideoEncodeCapabilitiesKHR(VideoEncodeCapabilitiesKHR),
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for VideoCapabilitiesKHRExtension {
    type LowLevel = *mut crate::native::vulkan1_0::BaseOutStructure;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self {
            #[cfg(feature = "VK_KHR_video_decode_queue")]
            Self::VideoDecodeCapabilitiesKHR(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::extensions::khr_video_decode_queue::VideoDecodeCapabilitiesKHR)
                .cast(),
            #[cfg(feature = "VK_KHR_video_encode_queue")]
            Self::VideoEncodeCapabilitiesKHR(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::extensions::khr_video_encode_queue::VideoEncodeCapabilitiesKHR)
                .cast(),
            other => unreachable!("unexpected variant {:?}", other),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for VideoCapabilitiesKHRExtension {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        assert!(!value.is_null());
        match (*value).s_type {
            #[cfg(feature = "VK_KHR_video_decode_queue")]
            crate::native::vulkan1_0::StructureType::VideoDecodeCapabilitiesKhr => {
                Self::VideoDecodeCapabilitiesKHR(VideoDecodeCapabilitiesKHR::from_low_level(
                    context,
                    std::ptr::read(
                        value.cast::<crate::native::extensions::khr_video_decode_queue::VideoDecodeCapabilitiesKHR>(),
                    ),
                ))
            },
            #[cfg(feature = "VK_KHR_video_encode_queue")]
            crate::native::vulkan1_0::StructureType::VideoEncodeCapabilitiesKhr => {
                Self::VideoEncodeCapabilitiesKHR(VideoEncodeCapabilitiesKHR::from_low_level(
                    context,
                    std::ptr::read(
                        value.cast::<crate::native::extensions::khr_video_encode_queue::VideoEncodeCapabilitiesKHR>(),
                    ),
                ))
            },
            other => panic!(
                "Structure type {:?} is not a member of {}",
                other,
                stringify!(VideoCapabilitiesKHR)
            ),
        }
    }
}
#[cfg(feature = "VK_KHR_video_decode_queue")]
impl From<VideoDecodeCapabilitiesKHR> for VideoCapabilitiesKHRExtension {
    fn from(ext: VideoDecodeCapabilitiesKHR) -> Self {
        Self::VideoDecodeCapabilitiesKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_video_decode_queue")]
impl TryInto<VideoDecodeCapabilitiesKHR> for VideoCapabilitiesKHRExtension {
    type Error = VideoCapabilitiesKHRExtension;
    fn try_into(self) -> Result<VideoDecodeCapabilitiesKHR, Self::Error> {
        match self {
            Self::VideoDecodeCapabilitiesKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_video_encode_queue")]
impl From<VideoEncodeCapabilitiesKHR> for VideoCapabilitiesKHRExtension {
    fn from(ext: VideoEncodeCapabilitiesKHR) -> Self {
        Self::VideoEncodeCapabilitiesKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_video_encode_queue")]
impl TryInto<VideoEncodeCapabilitiesKHR> for VideoCapabilitiesKHRExtension {
    type Error = VideoCapabilitiesKHRExtension;
    fn try_into(self) -> Result<VideoEncodeCapabilitiesKHR, Self::Error> {
        match self {
            Self::VideoEncodeCapabilitiesKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[doc(alias = "VkVideoGetMemoryPropertiesKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoGetMemoryPropertiesKHR {
    #[doc(alias = "memoryBindIndex")]
    pub memory_bind_index: u32,
    #[doc(alias = "pMemoryRequirements")]
    pub memory_requirements: MemoryRequirements2,
}
impl VideoGetMemoryPropertiesKHR {
    ///Get a reference to the `memory_bind_index` field.
    pub fn memory_bind_index(&self) -> u32 {
        self.memory_bind_index
    }
    ///Get a reference to the `memory_requirements` field.
    pub fn memory_requirements(&self) -> &MemoryRequirements2 {
        &self.memory_requirements
    }
    ///Get a mutable reference to the `memory_bind_index` field.
    pub fn memory_bind_index_mut(&mut self) -> &mut u32 {
        &mut self.memory_bind_index
    }
    ///Get a mutable reference to the `memory_requirements` field.
    pub fn memory_requirements_mut(&mut self) -> &mut MemoryRequirements2 {
        &mut self.memory_requirements
    }
    ///Sets the `memory_bind_index` field.
    pub fn set_memory_bind_index(&mut self, memory_bind_index: u32) -> &mut Self {
        self.memory_bind_index = memory_bind_index;
        self
    }
    ///Sets the `memory_requirements` field.
    pub fn set_memory_requirements(&mut self, memory_requirements: MemoryRequirements2) -> &mut Self {
        self.memory_requirements = memory_requirements;
        self
    }
    ///Sets the `memory_bind_index` field in a builder way.
    pub fn with_memory_bind_index(mut self, memory_bind_index: u32) -> Self {
        self.memory_bind_index = memory_bind_index;
        self
    }
    ///Sets the `memory_requirements` field in a builder way.
    pub fn with_memory_requirements(mut self, memory_requirements: MemoryRequirements2) -> Self {
        self.memory_requirements = memory_requirements;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for VideoGetMemoryPropertiesKHR {
    type LowLevel = crate::native::extensions::khr_video_queue::VideoGetMemoryPropertiesKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_video_queue::VideoGetMemoryPropertiesKHR {
            s_type: StructureType::VideoGetMemoryPropertiesKhr,
            p_next: std::ptr::null(),
            memory_bind_index: self.memory_bind_index.into_low_level(context, bump),
            memory_requirements: bump.alloc(self.memory_requirements.into_low_level(context, bump)) as *mut _,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for VideoGetMemoryPropertiesKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            memory_bind_index: crate::conv::FromLowLevel::from_low_level(context, value.memory_bind_index),
            memory_requirements: crate::conv::FromLowLevel::from_low_level(context, *value.memory_requirements),
        }
    }
}
#[doc(alias = "VkVideoBindMemoryKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoBindMemoryKHR {
    #[doc(alias = "memoryBindIndex")]
    pub memory_bind_index: u32,
    pub memory: DeviceMemory,
    #[doc(alias = "memoryOffset")]
    pub memory_offset: DeviceSize,
    #[doc(alias = "memorySize")]
    pub memory_size: DeviceSize,
}
impl VideoBindMemoryKHR {
    ///Get a reference to the `memory_bind_index` field.
    pub fn memory_bind_index(&self) -> u32 {
        self.memory_bind_index
    }
    ///Get a reference to the `memory` field.
    pub fn memory(&self) -> &DeviceMemory {
        &self.memory
    }
    ///Get a reference to the `memory_offset` field.
    pub fn memory_offset(&self) -> &DeviceSize {
        &self.memory_offset
    }
    ///Get a reference to the `memory_size` field.
    pub fn memory_size(&self) -> &DeviceSize {
        &self.memory_size
    }
    ///Get a mutable reference to the `memory_bind_index` field.
    pub fn memory_bind_index_mut(&mut self) -> &mut u32 {
        &mut self.memory_bind_index
    }
    ///Get a mutable reference to the `memory` field.
    pub fn memory_mut(&mut self) -> &mut DeviceMemory {
        &mut self.memory
    }
    ///Get a mutable reference to the `memory_offset` field.
    pub fn memory_offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.memory_offset
    }
    ///Get a mutable reference to the `memory_size` field.
    pub fn memory_size_mut(&mut self) -> &mut DeviceSize {
        &mut self.memory_size
    }
    ///Sets the `memory_bind_index` field.
    pub fn set_memory_bind_index(&mut self, memory_bind_index: u32) -> &mut Self {
        self.memory_bind_index = memory_bind_index;
        self
    }
    ///Sets the `memory` field.
    pub fn set_memory(&mut self, memory: DeviceMemory) -> &mut Self {
        self.memory = memory;
        self
    }
    ///Sets the `memory_offset` field.
    pub fn set_memory_offset(&mut self, memory_offset: DeviceSize) -> &mut Self {
        self.memory_offset = memory_offset;
        self
    }
    ///Sets the `memory_size` field.
    pub fn set_memory_size(&mut self, memory_size: DeviceSize) -> &mut Self {
        self.memory_size = memory_size;
        self
    }
    ///Sets the `memory_bind_index` field in a builder way.
    pub fn with_memory_bind_index(mut self, memory_bind_index: u32) -> Self {
        self.memory_bind_index = memory_bind_index;
        self
    }
    ///Sets the `memory` field in a builder way.
    pub fn with_memory(mut self, memory: DeviceMemory) -> Self {
        self.memory = memory;
        self
    }
    ///Sets the `memory_offset` field in a builder way.
    pub fn with_memory_offset(mut self, memory_offset: DeviceSize) -> Self {
        self.memory_offset = memory_offset;
        self
    }
    ///Sets the `memory_size` field in a builder way.
    pub fn with_memory_size(mut self, memory_size: DeviceSize) -> Self {
        self.memory_size = memory_size;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for VideoBindMemoryKHR {
    type LowLevel = crate::native::extensions::khr_video_queue::VideoBindMemoryKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_video_queue::VideoBindMemoryKHR {
            s_type: StructureType::VideoBindMemoryKhr,
            p_next: std::ptr::null(),
            memory_bind_index: self.memory_bind_index.into_low_level(context, bump),
            memory: self.memory.into_low_level(context, bump),
            memory_offset: self.memory_offset.into_low_level(context, bump),
            memory_size: self.memory_size.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for VideoBindMemoryKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            memory_bind_index: crate::conv::FromLowLevel::from_low_level(context, value.memory_bind_index),
            memory: crate::conv::FromLowLevel::from_low_level(context, value.memory),
            memory_offset: crate::conv::FromLowLevel::from_low_level(context, value.memory_offset),
            memory_size: crate::conv::FromLowLevel::from_low_level(context, value.memory_size),
        }
    }
}
#[doc(alias = "VkVideoPictureResourceKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoPictureResourceKHR {
    #[doc(alias = "codedOffset")]
    pub coded_offset: Offset2D,
    #[doc(alias = "codedExtent")]
    pub coded_extent: Extent2D,
    #[doc(alias = "baseArrayLayer")]
    pub base_array_layer: u32,
    #[doc(alias = "imageViewBinding")]
    pub image_view_binding: ImageView,
}
impl VideoPictureResourceKHR {
    ///Get a reference to the `coded_offset` field.
    pub fn coded_offset(&self) -> Offset2D {
        self.coded_offset
    }
    ///Get a reference to the `coded_extent` field.
    pub fn coded_extent(&self) -> Extent2D {
        self.coded_extent
    }
    ///Get a reference to the `base_array_layer` field.
    pub fn base_array_layer(&self) -> u32 {
        self.base_array_layer
    }
    ///Get a reference to the `image_view_binding` field.
    pub fn image_view_binding(&self) -> &ImageView {
        &self.image_view_binding
    }
    ///Get a mutable reference to the `coded_offset` field.
    pub fn coded_offset_mut(&mut self) -> &mut Offset2D {
        &mut self.coded_offset
    }
    ///Get a mutable reference to the `coded_extent` field.
    pub fn coded_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.coded_extent
    }
    ///Get a mutable reference to the `base_array_layer` field.
    pub fn base_array_layer_mut(&mut self) -> &mut u32 {
        &mut self.base_array_layer
    }
    ///Get a mutable reference to the `image_view_binding` field.
    pub fn image_view_binding_mut(&mut self) -> &mut ImageView {
        &mut self.image_view_binding
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
    ///Sets the `base_array_layer` field.
    pub fn set_base_array_layer(&mut self, base_array_layer: u32) -> &mut Self {
        self.base_array_layer = base_array_layer;
        self
    }
    ///Sets the `image_view_binding` field.
    pub fn set_image_view_binding(&mut self, image_view_binding: ImageView) -> &mut Self {
        self.image_view_binding = image_view_binding;
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
    ///Sets the `base_array_layer` field in a builder way.
    pub fn with_base_array_layer(mut self, base_array_layer: u32) -> Self {
        self.base_array_layer = base_array_layer;
        self
    }
    ///Sets the `image_view_binding` field in a builder way.
    pub fn with_image_view_binding(mut self, image_view_binding: ImageView) -> Self {
        self.image_view_binding = image_view_binding;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for VideoPictureResourceKHR {
    type LowLevel = crate::native::extensions::khr_video_queue::VideoPictureResourceKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_video_queue::VideoPictureResourceKHR {
            s_type: StructureType::VideoPictureResourceKhr,
            p_next: std::ptr::null(),
            coded_offset: self.coded_offset.into_low_level(context, bump),
            coded_extent: self.coded_extent.into_low_level(context, bump),
            base_array_layer: self.base_array_layer.into_low_level(context, bump),
            image_view_binding: self.image_view_binding.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for VideoPictureResourceKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            coded_offset: crate::conv::FromLowLevel::from_low_level(context, value.coded_offset),
            coded_extent: crate::conv::FromLowLevel::from_low_level(context, value.coded_extent),
            base_array_layer: crate::conv::FromLowLevel::from_low_level(context, value.base_array_layer),
            image_view_binding: crate::conv::FromLowLevel::from_low_level(context, value.image_view_binding),
        }
    }
}
#[doc(alias = "VkVideoReferenceSlotKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoReferenceSlotKHR {
    #[doc(alias = "slotIndex")]
    pub slot_index: i8,
    #[doc(alias = "pPictureResource")]
    pub picture_resource: VideoPictureResourceKHR,
}
impl VideoReferenceSlotKHR {
    ///Get a reference to the `slot_index` field.
    pub fn slot_index(&self) -> i8 {
        self.slot_index
    }
    ///Get a reference to the `picture_resource` field.
    pub fn picture_resource(&self) -> &VideoPictureResourceKHR {
        &self.picture_resource
    }
    ///Get a mutable reference to the `slot_index` field.
    pub fn slot_index_mut(&mut self) -> &mut i8 {
        &mut self.slot_index
    }
    ///Get a mutable reference to the `picture_resource` field.
    pub fn picture_resource_mut(&mut self) -> &mut VideoPictureResourceKHR {
        &mut self.picture_resource
    }
    ///Sets the `slot_index` field.
    pub fn set_slot_index(&mut self, slot_index: i8) -> &mut Self {
        self.slot_index = slot_index;
        self
    }
    ///Sets the `picture_resource` field.
    pub fn set_picture_resource(&mut self, picture_resource: VideoPictureResourceKHR) -> &mut Self {
        self.picture_resource = picture_resource;
        self
    }
    ///Sets the `slot_index` field in a builder way.
    pub fn with_slot_index(mut self, slot_index: i8) -> Self {
        self.slot_index = slot_index;
        self
    }
    ///Sets the `picture_resource` field in a builder way.
    pub fn with_picture_resource(mut self, picture_resource: VideoPictureResourceKHR) -> Self {
        self.picture_resource = picture_resource;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for VideoReferenceSlotKHR {
    type LowLevel = crate::native::extensions::khr_video_queue::VideoReferenceSlotKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_video_queue::VideoReferenceSlotKHR {
            s_type: StructureType::VideoReferenceSlotKhr,
            p_next: std::ptr::null(),
            slot_index: self.slot_index.into_low_level(context, bump),
            picture_resource: bump.alloc(self.picture_resource.into_low_level(context, bump)) as *const _,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for VideoReferenceSlotKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            slot_index: crate::conv::FromLowLevel::from_low_level(context, value.slot_index),
            picture_resource: crate::conv::FromLowLevel::from_low_level(context, *value.picture_resource),
        }
    }
}
#[doc(alias = "VkVideoSessionCreateInfoKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoSessionCreateInfoKHR {
    #[doc(alias = "queueFamilyIndex")]
    pub queue_family_index: u32,
    pub flags: VideoSessionCreateFlagsKHR,
    #[doc(alias = "pVideoProfile")]
    pub video_profile: VideoProfileKHR,
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
    pub std_header_version: ExtensionProperties,
}
impl VideoSessionCreateInfoKHR {
    ///Get a reference to the `queue_family_index` field.
    pub fn queue_family_index(&self) -> u32 {
        self.queue_family_index
    }
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> VideoSessionCreateFlagsKHR {
        self.flags
    }
    ///Get a reference to the `video_profile` field.
    pub fn video_profile(&self) -> &VideoProfileKHR {
        &self.video_profile
    }
    ///Get a reference to the `picture_format` field.
    pub fn picture_format(&self) -> Format {
        self.picture_format
    }
    ///Get a reference to the `max_coded_extent` field.
    pub fn max_coded_extent(&self) -> Extent2D {
        self.max_coded_extent
    }
    ///Get a reference to the `reference_pictures_format` field.
    pub fn reference_pictures_format(&self) -> Format {
        self.reference_pictures_format
    }
    ///Get a reference to the `max_reference_pictures_slots_count` field.
    pub fn max_reference_pictures_slots_count(&self) -> u32 {
        self.max_reference_pictures_slots_count
    }
    ///Get a reference to the `max_reference_pictures_active_count` field.
    pub fn max_reference_pictures_active_count(&self) -> u32 {
        self.max_reference_pictures_active_count
    }
    ///Get a reference to the `std_header_version` field.
    pub fn std_header_version(&self) -> &ExtensionProperties {
        &self.std_header_version
    }
    ///Get a mutable reference to the `queue_family_index` field.
    pub fn queue_family_index_mut(&mut self) -> &mut u32 {
        &mut self.queue_family_index
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut VideoSessionCreateFlagsKHR {
        &mut self.flags
    }
    ///Get a mutable reference to the `video_profile` field.
    pub fn video_profile_mut(&mut self) -> &mut VideoProfileKHR {
        &mut self.video_profile
    }
    ///Get a mutable reference to the `picture_format` field.
    pub fn picture_format_mut(&mut self) -> &mut Format {
        &mut self.picture_format
    }
    ///Get a mutable reference to the `max_coded_extent` field.
    pub fn max_coded_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.max_coded_extent
    }
    ///Get a mutable reference to the `reference_pictures_format` field.
    pub fn reference_pictures_format_mut(&mut self) -> &mut Format {
        &mut self.reference_pictures_format
    }
    ///Get a mutable reference to the `max_reference_pictures_slots_count` field.
    pub fn max_reference_pictures_slots_count_mut(&mut self) -> &mut u32 {
        &mut self.max_reference_pictures_slots_count
    }
    ///Get a mutable reference to the `max_reference_pictures_active_count` field.
    pub fn max_reference_pictures_active_count_mut(&mut self) -> &mut u32 {
        &mut self.max_reference_pictures_active_count
    }
    ///Get a mutable reference to the `std_header_version` field.
    pub fn std_header_version_mut(&mut self) -> &mut ExtensionProperties {
        &mut self.std_header_version
    }
    ///Sets the `queue_family_index` field.
    pub fn set_queue_family_index(&mut self, queue_family_index: u32) -> &mut Self {
        self.queue_family_index = queue_family_index;
        self
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: VideoSessionCreateFlagsKHR) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `video_profile` field.
    pub fn set_video_profile(&mut self, video_profile: VideoProfileKHR) -> &mut Self {
        self.video_profile = video_profile;
        self
    }
    ///Sets the `picture_format` field.
    pub fn set_picture_format(&mut self, picture_format: Format) -> &mut Self {
        self.picture_format = picture_format;
        self
    }
    ///Sets the `max_coded_extent` field.
    pub fn set_max_coded_extent(&mut self, max_coded_extent: Extent2D) -> &mut Self {
        self.max_coded_extent = max_coded_extent;
        self
    }
    ///Sets the `reference_pictures_format` field.
    pub fn set_reference_pictures_format(&mut self, reference_pictures_format: Format) -> &mut Self {
        self.reference_pictures_format = reference_pictures_format;
        self
    }
    ///Sets the `max_reference_pictures_slots_count` field.
    pub fn set_max_reference_pictures_slots_count(&mut self, max_reference_pictures_slots_count: u32) -> &mut Self {
        self.max_reference_pictures_slots_count = max_reference_pictures_slots_count;
        self
    }
    ///Sets the `max_reference_pictures_active_count` field.
    pub fn set_max_reference_pictures_active_count(&mut self, max_reference_pictures_active_count: u32) -> &mut Self {
        self.max_reference_pictures_active_count = max_reference_pictures_active_count;
        self
    }
    ///Sets the `std_header_version` field.
    pub fn set_std_header_version(&mut self, std_header_version: ExtensionProperties) -> &mut Self {
        self.std_header_version = std_header_version;
        self
    }
    ///Sets the `queue_family_index` field in a builder way.
    pub fn with_queue_family_index(mut self, queue_family_index: u32) -> Self {
        self.queue_family_index = queue_family_index;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: VideoSessionCreateFlagsKHR) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `video_profile` field in a builder way.
    pub fn with_video_profile(mut self, video_profile: VideoProfileKHR) -> Self {
        self.video_profile = video_profile;
        self
    }
    ///Sets the `picture_format` field in a builder way.
    pub fn with_picture_format(mut self, picture_format: Format) -> Self {
        self.picture_format = picture_format;
        self
    }
    ///Sets the `max_coded_extent` field in a builder way.
    pub fn with_max_coded_extent(mut self, max_coded_extent: Extent2D) -> Self {
        self.max_coded_extent = max_coded_extent;
        self
    }
    ///Sets the `reference_pictures_format` field in a builder way.
    pub fn with_reference_pictures_format(mut self, reference_pictures_format: Format) -> Self {
        self.reference_pictures_format = reference_pictures_format;
        self
    }
    ///Sets the `max_reference_pictures_slots_count` field in a builder way.
    pub fn with_max_reference_pictures_slots_count(mut self, max_reference_pictures_slots_count: u32) -> Self {
        self.max_reference_pictures_slots_count = max_reference_pictures_slots_count;
        self
    }
    ///Sets the `max_reference_pictures_active_count` field in a builder way.
    pub fn with_max_reference_pictures_active_count(mut self, max_reference_pictures_active_count: u32) -> Self {
        self.max_reference_pictures_active_count = max_reference_pictures_active_count;
        self
    }
    ///Sets the `std_header_version` field in a builder way.
    pub fn with_std_header_version(mut self, std_header_version: ExtensionProperties) -> Self {
        self.std_header_version = std_header_version;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for VideoSessionCreateInfoKHR {
    type LowLevel = crate::native::extensions::khr_video_queue::VideoSessionCreateInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_video_queue::VideoSessionCreateInfoKHR {
            s_type: StructureType::VideoSessionCreateInfoKhr,
            p_next: std::ptr::null(),
            queue_family_index: self.queue_family_index.into_low_level(context, bump),
            flags: self.flags.into_low_level(context, bump),
            video_profile: bump.alloc(self.video_profile.into_low_level(context, bump)) as *const _,
            picture_format: self.picture_format.into_low_level(context, bump),
            max_coded_extent: self.max_coded_extent.into_low_level(context, bump),
            reference_pictures_format: self.reference_pictures_format.into_low_level(context, bump),
            max_reference_pictures_slots_count: self.max_reference_pictures_slots_count.into_low_level(context, bump),
            max_reference_pictures_active_count: self.max_reference_pictures_active_count.into_low_level(context, bump),
            std_header_version: bump.alloc(self.std_header_version.into_low_level(context, bump)) as *const _,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for VideoSessionCreateInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            queue_family_index: crate::conv::FromLowLevel::from_low_level(context, value.queue_family_index),
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            video_profile: crate::conv::FromLowLevel::from_low_level(context, *value.video_profile),
            picture_format: crate::conv::FromLowLevel::from_low_level(context, value.picture_format),
            max_coded_extent: crate::conv::FromLowLevel::from_low_level(context, value.max_coded_extent),
            reference_pictures_format: crate::conv::FromLowLevel::from_low_level(
                context,
                value.reference_pictures_format,
            ),
            max_reference_pictures_slots_count: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_reference_pictures_slots_count,
            ),
            max_reference_pictures_active_count: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_reference_pictures_active_count,
            ),
            std_header_version: crate::conv::FromLowLevel::from_low_level(context, *value.std_header_version),
        }
    }
}
#[doc(alias = "VkVideoSessionParametersCreateInfoKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoSessionParametersCreateInfoKHR {
    #[doc(alias = "videoSessionParametersTemplate")]
    pub video_session_parameters_template: Option<VideoSessionParametersKHR>,
    #[doc(alias = "videoSession")]
    pub video_session: VideoSessionKHR,
}
impl VideoSessionParametersCreateInfoKHR {
    ///Get a reference to the `video_session_parameters_template` field.
    pub fn video_session_parameters_template(&self) -> &Option<VideoSessionParametersKHR> {
        &self.video_session_parameters_template
    }
    ///Get a reference to the `video_session` field.
    pub fn video_session(&self) -> &VideoSessionKHR {
        &self.video_session
    }
    ///Get a mutable reference to the `video_session_parameters_template` field.
    pub fn video_session_parameters_template_mut(&mut self) -> &mut Option<VideoSessionParametersKHR> {
        &mut self.video_session_parameters_template
    }
    ///Get a mutable reference to the `video_session` field.
    pub fn video_session_mut(&mut self) -> &mut VideoSessionKHR {
        &mut self.video_session
    }
    ///Sets the `video_session_parameters_template` field.
    pub fn set_video_session_parameters_template(
        &mut self,
        video_session_parameters_template: Option<VideoSessionParametersKHR>,
    ) -> &mut Self {
        self.video_session_parameters_template = video_session_parameters_template;
        self
    }
    ///Sets the `video_session` field.
    pub fn set_video_session(&mut self, video_session: VideoSessionKHR) -> &mut Self {
        self.video_session = video_session;
        self
    }
    ///Sets the `video_session_parameters_template` field in a builder way.
    pub fn with_video_session_parameters_template(
        mut self,
        video_session_parameters_template: Option<VideoSessionParametersKHR>,
    ) -> Self {
        self.video_session_parameters_template = video_session_parameters_template;
        self
    }
    ///Sets the `video_session` field in a builder way.
    pub fn with_video_session(mut self, video_session: VideoSessionKHR) -> Self {
        self.video_session = video_session;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for VideoSessionParametersCreateInfoKHR {
    type LowLevel = crate::native::extensions::khr_video_queue::VideoSessionParametersCreateInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_video_queue::VideoSessionParametersCreateInfoKHR {
            s_type: StructureType::VideoSessionParametersCreateInfoKhr,
            p_next: std::ptr::null(),
            video_session_parameters_template: self
                .video_session_parameters_template
                .as_ref()
                .map(|v| v.into_low_level(context, bump))
                .unwrap_or_else(Default::default),
            video_session: self.video_session.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for VideoSessionParametersCreateInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            video_session_parameters_template: if value.video_session_parameters_template
                == crate::native::extensions::khr_video_queue::VideoSessionParametersKHR::null()
            {
                None
            } else {
                Some(crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.video_session_parameters_template,
                ))
            },
            video_session: crate::conv::FromLowLevel::from_low_level(context, value.video_session),
        }
    }
}
#[doc(alias = "VkVideoSessionParametersUpdateInfoKHR")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoSessionParametersUpdateInfoKHR {
    #[doc(alias = "updateSequenceCount")]
    pub update_sequence_count: u32,
}
impl VideoSessionParametersUpdateInfoKHR {
    ///Get a reference to the `update_sequence_count` field.
    pub fn update_sequence_count(&self) -> u32 {
        self.update_sequence_count
    }
    ///Get a mutable reference to the `update_sequence_count` field.
    pub fn update_sequence_count_mut(&mut self) -> &mut u32 {
        &mut self.update_sequence_count
    }
    ///Sets the `update_sequence_count` field.
    pub fn set_update_sequence_count(&mut self, update_sequence_count: u32) -> &mut Self {
        self.update_sequence_count = update_sequence_count;
        self
    }
    ///Sets the `update_sequence_count` field in a builder way.
    pub fn with_update_sequence_count(mut self, update_sequence_count: u32) -> Self {
        self.update_sequence_count = update_sequence_count;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for VideoSessionParametersUpdateInfoKHR {
    type LowLevel = crate::native::extensions::khr_video_queue::VideoSessionParametersUpdateInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_video_queue::VideoSessionParametersUpdateInfoKHR {
            s_type: StructureType::VideoSessionParametersUpdateInfoKhr,
            p_next: std::ptr::null(),
            update_sequence_count: self.update_sequence_count.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for VideoSessionParametersUpdateInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            update_sequence_count: crate::conv::FromLowLevel::from_low_level(context, value.update_sequence_count),
        }
    }
}
#[doc(alias = "VkVideoBeginCodingInfoKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoBeginCodingInfoKHR {
    pub flags: VideoBeginCodingFlagsKHR,
    #[doc(alias = "codecQualityPreset")]
    pub codec_quality_preset: VideoCodingQualityPresetFlagsKHR,
    #[doc(alias = "videoSession")]
    pub video_session: VideoSessionKHR,
    #[doc(alias = "videoSessionParameters")]
    pub video_session_parameters: Option<VideoSessionParametersKHR>,
    #[doc(alias = "pReferenceSlots")]
    pub reference_slots: SmallVec<[VideoReferenceSlotKHR; 8]>,
}
impl VideoBeginCodingInfoKHR {
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> VideoBeginCodingFlagsKHR {
        self.flags
    }
    ///Get a reference to the `codec_quality_preset` field.
    pub fn codec_quality_preset(&self) -> VideoCodingQualityPresetFlagsKHR {
        self.codec_quality_preset
    }
    ///Get a reference to the `video_session` field.
    pub fn video_session(&self) -> &VideoSessionKHR {
        &self.video_session
    }
    ///Get a reference to the `video_session_parameters` field.
    pub fn video_session_parameters(&self) -> &Option<VideoSessionParametersKHR> {
        &self.video_session_parameters
    }
    ///Get a reference to the `reference_slots` field.
    pub fn reference_slots(&self) -> &SmallVec<[VideoReferenceSlotKHR; 8]> {
        &self.reference_slots
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut VideoBeginCodingFlagsKHR {
        &mut self.flags
    }
    ///Get a mutable reference to the `codec_quality_preset` field.
    pub fn codec_quality_preset_mut(&mut self) -> &mut VideoCodingQualityPresetFlagsKHR {
        &mut self.codec_quality_preset
    }
    ///Get a mutable reference to the `video_session` field.
    pub fn video_session_mut(&mut self) -> &mut VideoSessionKHR {
        &mut self.video_session
    }
    ///Get a mutable reference to the `video_session_parameters` field.
    pub fn video_session_parameters_mut(&mut self) -> &mut Option<VideoSessionParametersKHR> {
        &mut self.video_session_parameters
    }
    ///Get a mutable reference to the `reference_slots` field.
    pub fn reference_slots_mut(&mut self) -> &mut SmallVec<[VideoReferenceSlotKHR; 8]> {
        &mut self.reference_slots
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: VideoBeginCodingFlagsKHR) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `codec_quality_preset` field.
    pub fn set_codec_quality_preset(&mut self, codec_quality_preset: VideoCodingQualityPresetFlagsKHR) -> &mut Self {
        self.codec_quality_preset = codec_quality_preset;
        self
    }
    ///Sets the `video_session` field.
    pub fn set_video_session(&mut self, video_session: VideoSessionKHR) -> &mut Self {
        self.video_session = video_session;
        self
    }
    ///Sets the `video_session_parameters` field.
    pub fn set_video_session_parameters(
        &mut self,
        video_session_parameters: Option<VideoSessionParametersKHR>,
    ) -> &mut Self {
        self.video_session_parameters = video_session_parameters;
        self
    }
    ///Sets the `reference_slots` field.
    pub fn set_reference_slots(&mut self, reference_slots: SmallVec<[VideoReferenceSlotKHR; 8]>) -> &mut Self {
        self.reference_slots = reference_slots;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: VideoBeginCodingFlagsKHR) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `codec_quality_preset` field in a builder way.
    pub fn with_codec_quality_preset(mut self, codec_quality_preset: VideoCodingQualityPresetFlagsKHR) -> Self {
        self.codec_quality_preset = codec_quality_preset;
        self
    }
    ///Sets the `video_session` field in a builder way.
    pub fn with_video_session(mut self, video_session: VideoSessionKHR) -> Self {
        self.video_session = video_session;
        self
    }
    ///Sets the `video_session_parameters` field in a builder way.
    pub fn with_video_session_parameters(
        mut self,
        video_session_parameters: Option<VideoSessionParametersKHR>,
    ) -> Self {
        self.video_session_parameters = video_session_parameters;
        self
    }
    ///Sets the `reference_slots` field in a builder way.
    pub fn with_reference_slots(mut self, reference_slots: SmallVec<[VideoReferenceSlotKHR; 8]>) -> Self {
        self.reference_slots = reference_slots;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for VideoBeginCodingInfoKHR {
    type LowLevel = crate::native::extensions::khr_video_queue::VideoBeginCodingInfoKHR;
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
        crate::native::extensions::khr_video_queue::VideoBeginCodingInfoKHR {
            s_type: StructureType::VideoBeginCodingInfoKhr,
            p_next: std::ptr::null(),
            flags: self.flags.into_low_level(context, bump),
            codec_quality_preset: self.codec_quality_preset.into_low_level(context, bump),
            video_session: self.video_session.into_low_level(context, bump),
            video_session_parameters: self
                .video_session_parameters
                .as_ref()
                .map(|v| v.into_low_level(context, bump))
                .unwrap_or_else(Default::default),
            reference_slot_count: len_reference_slots,
            reference_slots: reference_slots,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for VideoBeginCodingInfoKHR {
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
            codec_quality_preset: crate::conv::FromLowLevel::from_low_level(context, value.codec_quality_preset),
            video_session: crate::conv::FromLowLevel::from_low_level(context, value.video_session),
            video_session_parameters: if value.video_session_parameters
                == crate::native::extensions::khr_video_queue::VideoSessionParametersKHR::null()
            {
                None
            } else {
                Some(crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.video_session_parameters,
                ))
            },
            reference_slots: reference_slots,
        }
    }
}
#[doc(alias = "VkVideoEndCodingInfoKHR")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoEndCodingInfoKHR {
    pub flags: VideoEndCodingFlagsKHR,
}
impl VideoEndCodingInfoKHR {
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> VideoEndCodingFlagsKHR {
        self.flags
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut VideoEndCodingFlagsKHR {
        &mut self.flags
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: VideoEndCodingFlagsKHR) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: VideoEndCodingFlagsKHR) -> Self {
        self.flags = flags;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for VideoEndCodingInfoKHR {
    type LowLevel = crate::native::extensions::khr_video_queue::VideoEndCodingInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_video_queue::VideoEndCodingInfoKHR {
            s_type: StructureType::VideoEndCodingInfoKhr,
            p_next: std::ptr::null(),
            flags: self.flags.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for VideoEndCodingInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
        }
    }
}
#[doc(alias = "VkVideoCodingControlInfoKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoCodingControlInfoKHR {
    #[doc(alias = "pNext")]
    pub extensions: SmallVec<[VideoCodingControlInfoKHRExtension; 1]>,
    pub flags: VideoCodingControlFlagsKHR,
}
impl VideoCodingControlInfoKHR {
    ///Adds an extension to the struct
    pub fn with_extension(mut self, ext: impl Into<VideoCodingControlInfoKHRExtension>) -> Self {
        self.extensions.push(ext.into());
        self
    }
    ///Get a reference to the `extensions` field.
    pub fn extensions(&self) -> &SmallVec<[VideoCodingControlInfoKHRExtension; 1]> {
        &self.extensions
    }
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> VideoCodingControlFlagsKHR {
        self.flags
    }
    ///Get a mutable reference to the `extensions` field.
    pub fn extensions_mut(&mut self) -> &mut SmallVec<[VideoCodingControlInfoKHRExtension; 1]> {
        &mut self.extensions
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut VideoCodingControlFlagsKHR {
        &mut self.flags
    }
    ///Sets the `extensions` field.
    pub fn set_extensions(&mut self, extensions: SmallVec<[VideoCodingControlInfoKHRExtension; 1]>) -> &mut Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: VideoCodingControlFlagsKHR) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `extensions` field in a builder way.
    pub fn with_extensions(mut self, extensions: SmallVec<[VideoCodingControlInfoKHRExtension; 1]>) -> Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: VideoCodingControlFlagsKHR) -> Self {
        self.flags = flags;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for VideoCodingControlInfoKHR {
    type LowLevel = crate::native::extensions::khr_video_queue::VideoCodingControlInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let mut next = std::ptr::null();
        let mut extensions = self.extensions.iter();
        while let Some(ext) = extensions.next() {
            let ext = ext.into_low_level(context, bump);
            (*ext).next = next;
            next = ext;
        }
        crate::native::extensions::khr_video_queue::VideoCodingControlInfoKHR {
            s_type: StructureType::VideoCodingControlInfoKhr,
            p_next: next,
            flags: self.flags.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for VideoCodingControlInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let mut next = value.p_next;
        let mut extensions = SmallVec::new();
        while !next.is_null() {
            extensions.push(crate::conv::FromLowLevel::from_low_level(context, next));
            next = std::ptr::read(next).next;
        }
        Self {
            extensions: extensions,
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
        }
    }
}
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
///Extensions for [`VideoCodingControlInfoKHR`]
pub enum VideoCodingControlInfoKHRExtension {
    #[cfg(feature = "VK_KHR_video_encode_queue")]
    ///Contains a type [`VideoEncodeRateControlInfoKHR`] for extending
    /// [`VideoCodingControlInfoKHR`]
    VideoEncodeRateControlInfoKHR(VideoEncodeRateControlInfoKHR),
    #[cfg(feature = "VK_KHR_video_encode_queue")]
    ///Contains a type [`VideoEncodeRateControlLayerInfoKHR`] for extending
    /// [`VideoCodingControlInfoKHR`]
    VideoEncodeRateControlLayerInfoKHR(VideoEncodeRateControlLayerInfoKHR),
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for VideoCodingControlInfoKHRExtension {
    type LowLevel = *const crate::native::vulkan1_0::BaseInStructure;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self {
            #[cfg(feature = "VK_KHR_video_encode_queue")]
            Self::VideoEncodeRateControlInfoKHR(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::extensions::khr_video_encode_queue::VideoEncodeRateControlInfoKHR)
                .cast(),
            #[cfg(feature = "VK_KHR_video_encode_queue")]
            Self::VideoEncodeRateControlLayerInfoKHR(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::extensions::khr_video_encode_queue::VideoEncodeRateControlLayerInfoKHR)
                .cast(),
            other => unreachable!("unexpected variant {:?}", other),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for VideoCodingControlInfoKHRExtension {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        assert!(!value.is_null());
        match (* value) . s_type { # [cfg (feature = "VK_KHR_video_encode_queue")] crate :: native :: vulkan1_0 :: StructureType :: VideoEncodeRateControlInfoKhr => Self :: VideoEncodeRateControlInfoKHR (VideoEncodeRateControlInfoKHR :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: khr_video_encode_queue :: VideoEncodeRateControlInfoKHR > ()))) , # [cfg (feature = "VK_KHR_video_encode_queue")] crate :: native :: vulkan1_0 :: StructureType :: VideoEncodeRateControlLayerInfoKhr => Self :: VideoEncodeRateControlLayerInfoKHR (VideoEncodeRateControlLayerInfoKHR :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: khr_video_encode_queue :: VideoEncodeRateControlLayerInfoKHR > ()))) , other => panic ! ("Structure type {:?} is not a member of {}" , other , stringify ! (VideoCodingControlInfoKHR)) }
    }
}
#[cfg(feature = "VK_KHR_video_encode_queue")]
impl From<VideoEncodeRateControlInfoKHR> for VideoCodingControlInfoKHRExtension {
    fn from(ext: VideoEncodeRateControlInfoKHR) -> Self {
        Self::VideoEncodeRateControlInfoKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_video_encode_queue")]
impl TryInto<VideoEncodeRateControlInfoKHR> for VideoCodingControlInfoKHRExtension {
    type Error = VideoCodingControlInfoKHRExtension;
    fn try_into(self) -> Result<VideoEncodeRateControlInfoKHR, Self::Error> {
        match self {
            Self::VideoEncodeRateControlInfoKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_video_encode_queue")]
impl From<VideoEncodeRateControlLayerInfoKHR> for VideoCodingControlInfoKHRExtension {
    fn from(ext: VideoEncodeRateControlLayerInfoKHR) -> Self {
        Self::VideoEncodeRateControlLayerInfoKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_video_encode_queue")]
impl TryInto<VideoEncodeRateControlLayerInfoKHR> for VideoCodingControlInfoKHRExtension {
    type Error = VideoCodingControlInfoKHRExtension;
    fn try_into(self) -> Result<VideoEncodeRateControlLayerInfoKHR, Self::Error> {
        match self {
            Self::VideoEncodeRateControlLayerInfoKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[doc(alias = "VkVideoSessionKHR")]
#[derive(Debug)]
pub struct VideoSessionKHR {
    context: Arc<Context>,
    id: ObjectId,
}
impl Clone for VideoSessionKHR {
    fn clone(&self) -> Self {
        self.context.clone_video_session_khr(self.id);
        Self {
            context: Arc::clone(&self.context),
            id: self.id,
        }
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for VideoSessionKHR {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.id.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for VideoSessionKHR {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id = ObjectId::deserialize(deserializer)?;
        crate::context::CONTEXT.with(|context| {
            let borrow = context.borrow();
            let context = borrow.as_ref().expect("Context not set.");
            Ok(Self {
                context: Arc::clone(context),
                id,
            })
        })
    }
}
impl Drop for VideoSessionKHR {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.context.drop_video_session_khr(&self.id);
        }
    }
}
impl PartialEq for VideoSessionKHR {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl VideoSessionKHR {
    ///Creates a new instance of this handle from its core components.
    pub(crate) const unsafe fn new(context: Arc<Context>, id: ObjectId) -> Self {
        Self { context, id }
    }
    ///Gets the object id
    pub fn id(&self) -> &ObjectId {
        &self.id
    }
    ///Gets a reference to the context
    pub fn context(&self) -> &Context {
        &self.context
    }
    ///Gets a reference to the context wrapped in an [`Arc`]
    pub fn arc_context(&self) -> &Arc<Context> {
        &self.context
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for VideoSessionKHR {
    type LowLevel = crate::native::extensions::khr_video_queue::VideoSessionKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *context
            .video_session_khr()
            .get(&self.id)
            .expect("unknwon handle")
            .handle()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for VideoSessionKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let object_id = ObjectId::random();
        context.video_session_khr().insert(object_id, Container::new(value));
        Self {
            context: context.clone(),
            id: object_id,
        }
    }
}
#[doc(alias = "VkVideoSessionParametersKHR")]
#[derive(Debug)]
pub struct VideoSessionParametersKHR {
    context: Arc<Context>,
    id: ObjectId,
}
impl Clone for VideoSessionParametersKHR {
    fn clone(&self) -> Self {
        self.context.clone_video_session_parameters_khr(self.id);
        Self {
            context: Arc::clone(&self.context),
            id: self.id,
        }
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for VideoSessionParametersKHR {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.id.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for VideoSessionParametersKHR {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id = ObjectId::deserialize(deserializer)?;
        crate::context::CONTEXT.with(|context| {
            let borrow = context.borrow();
            let context = borrow.as_ref().expect("Context not set.");
            Ok(Self {
                context: Arc::clone(context),
                id,
            })
        })
    }
}
impl Drop for VideoSessionParametersKHR {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.context.drop_video_session_parameters_khr(&self.id);
        }
    }
}
impl PartialEq for VideoSessionParametersKHR {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl VideoSessionParametersKHR {
    ///Creates a new instance of this handle from its core components.
    pub(crate) const unsafe fn new(context: Arc<Context>, id: ObjectId) -> Self {
        Self { context, id }
    }
    ///Gets the object id
    pub fn id(&self) -> &ObjectId {
        &self.id
    }
    ///Gets a reference to the context
    pub fn context(&self) -> &Context {
        &self.context
    }
    ///Gets a reference to the context wrapped in an [`Arc`]
    pub fn arc_context(&self) -> &Arc<Context> {
        &self.context
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for VideoSessionParametersKHR {
    type LowLevel = crate::native::extensions::khr_video_queue::VideoSessionParametersKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *context
            .video_session_parameters_khr()
            .get(&self.id)
            .expect("unknwon handle")
            .handle()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for VideoSessionParametersKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let object_id = ObjectId::random();
        context
            .video_session_parameters_khr()
            .insert(object_id, Container::new(value));
        Self {
            context: context.clone(),
            id: object_id,
        }
    }
}
