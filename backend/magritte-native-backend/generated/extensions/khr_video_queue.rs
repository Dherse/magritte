use crate::{
    cstr,
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, BaseOutStructure, Bool32, CommandBuffer, Device, DeviceMemory,
        DeviceSize, ExtensionProperties, Extent2D, Format, ImageUsageFlags, ImageView, Offset2D, PhysicalDevice,
        StructureType, VulkanResultCodes,
    },
    vulkan1_1::MemoryRequirements2,
};
use std::ffi::CStr;
#[doc(alias = "VkVideoQueueFamilyProperties2KHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoQueueFamilyProperties2KHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "videoCodecOperations")]
    video_codec_operations: VideoCodecOperationFlagsKHR,
}
#[doc(alias = "VkQueueFamilyQueryResultStatusProperties2KHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct QueueFamilyQueryResultStatusProperties2KHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    supported: Bool32,
}
#[doc(alias = "VkVideoProfilesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoProfilesKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "profileCount")]
    profile_count: u32,
    #[doc(alias = "pProfiles")]
    profiles: *const VideoProfileKHR,
}
#[doc(alias = "VkPhysicalDeviceVideoFormatInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVideoFormatInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "imageUsage")]
    image_usage: ImageUsageFlags,
    #[doc(alias = "pVideoProfiles")]
    video_profiles: *const VideoProfilesKHR,
}
#[doc(alias = "VkVideoFormatPropertiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoFormatPropertiesKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    format: Format,
}
#[doc(alias = "VkVideoProfileKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoProfileKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "videoCodecOperation")]
    video_codec_operation: VideoCodecOperationFlagBitsKHR,
    #[doc(alias = "chromaSubsampling")]
    chroma_subsampling: VideoChromaSubsamplingFlagsKHR,
    #[doc(alias = "lumaBitDepth")]
    luma_bit_depth: VideoComponentBitDepthFlagsKHR,
    #[doc(alias = "chromaBitDepth")]
    chroma_bit_depth: VideoComponentBitDepthFlagsKHR,
}
#[doc(alias = "VkVideoCapabilitiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoCapabilitiesKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "capabilityFlags")]
    capability_flags: VideoCapabilityFlagsKHR,
    #[doc(alias = "minBitstreamBufferOffsetAlignment")]
    min_bitstream_buffer_offset_alignment: DeviceSize,
    #[doc(alias = "minBitstreamBufferSizeAlignment")]
    min_bitstream_buffer_size_alignment: DeviceSize,
    #[doc(alias = "videoPictureExtentGranularity")]
    video_picture_extent_granularity: Extent2D,
    #[doc(alias = "minExtent")]
    min_extent: Extent2D,
    #[doc(alias = "maxExtent")]
    max_extent: Extent2D,
    #[doc(alias = "maxReferencePicturesSlotsCount")]
    max_reference_pictures_slots_count: u32,
    #[doc(alias = "maxReferencePicturesActiveCount")]
    max_reference_pictures_active_count: u32,
    #[doc(alias = "stdHeaderVersion")]
    std_header_version: ExtensionProperties,
}
#[doc(alias = "VkVideoGetMemoryPropertiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoGetMemoryPropertiesKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "memoryBindIndex")]
    memory_bind_index: u32,
    #[doc(alias = "pMemoryRequirements")]
    memory_requirements: *mut MemoryRequirements2,
}
#[doc(alias = "VkVideoBindMemoryKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoBindMemoryKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "memoryBindIndex")]
    memory_bind_index: u32,
    memory: DeviceMemory,
    #[doc(alias = "memoryOffset")]
    memory_offset: DeviceSize,
    #[doc(alias = "memorySize")]
    memory_size: DeviceSize,
}
#[doc(alias = "VkVideoPictureResourceKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoPictureResourceKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "codedOffset")]
    coded_offset: Offset2D,
    #[doc(alias = "codedExtent")]
    coded_extent: Extent2D,
    #[doc(alias = "baseArrayLayer")]
    base_array_layer: u32,
    #[doc(alias = "imageViewBinding")]
    image_view_binding: ImageView,
}
#[doc(alias = "VkVideoReferenceSlotKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoReferenceSlotKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "slotIndex")]
    slot_index: i8,
    #[doc(alias = "pPictureResource")]
    picture_resource: *const VideoPictureResourceKHR,
}
#[doc(alias = "VkVideoSessionCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoSessionCreateInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "queueFamilyIndex")]
    queue_family_index: u32,
    flags: VideoSessionCreateFlagsKHR,
    #[doc(alias = "pVideoProfile")]
    video_profile: *const VideoProfileKHR,
    #[doc(alias = "pictureFormat")]
    picture_format: Format,
    #[doc(alias = "maxCodedExtent")]
    max_coded_extent: Extent2D,
    #[doc(alias = "referencePicturesFormat")]
    reference_pictures_format: Format,
    #[doc(alias = "maxReferencePicturesSlotsCount")]
    max_reference_pictures_slots_count: u32,
    #[doc(alias = "maxReferencePicturesActiveCount")]
    max_reference_pictures_active_count: u32,
    #[doc(alias = "pStdHeaderVersion")]
    std_header_version: *const ExtensionProperties,
}
#[doc(alias = "VkVideoSessionParametersCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoSessionParametersCreateInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "videoSessionParametersTemplate")]
    video_session_parameters_template: VideoSessionParametersKHR,
    #[doc(alias = "videoSession")]
    video_session: VideoSessionKHR,
}
#[doc(alias = "VkVideoSessionParametersUpdateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoSessionParametersUpdateInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "updateSequenceCount")]
    update_sequence_count: u32,
}
#[doc(alias = "VkVideoBeginCodingInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoBeginCodingInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: VideoBeginCodingFlagsKHR,
    #[doc(alias = "codecQualityPreset")]
    codec_quality_preset: VideoCodingQualityPresetFlagsKHR,
    #[doc(alias = "videoSession")]
    video_session: VideoSessionKHR,
    #[doc(alias = "videoSessionParameters")]
    video_session_parameters: VideoSessionParametersKHR,
    #[doc(alias = "referenceSlotCount")]
    reference_slot_count: u32,
    #[doc(alias = "pReferenceSlots")]
    reference_slots: *const VideoReferenceSlotKHR,
}
#[doc(alias = "VkVideoEndCodingInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoEndCodingInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: VideoEndCodingFlagsKHR,
}
#[doc(alias = "VkVideoCodingControlInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoCodingControlInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: VideoCodingControlFlagsKHR,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkVideoSessionKHR")]
#[repr(transparent)]
pub struct VideoSessionKHR(u64);
impl VideoSessionKHR {
    pub const fn null() -> Self {
        Self(0)
    }
}
impl const Default for VideoSessionKHR {
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
}
impl const Default for VideoSessionParametersKHR {
    fn default() -> Self {
        Self::null()
    }
}
#[doc(alias = "VkVideoCodecOperationFlagsKHR")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoCodecOperationFlagsKHR(u32);
impl VideoCodecOperationFlagsKHR {
    #[doc(alias = "VK_VIDEO_CODEC_OPERATION_INVALID_BIT_KHR")]
    pub const INVALID: Self = Self(0);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        {
            all |= Self::INVALID;
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
impl FromIterator<VideoCodecOperationFlagsKHR> for VideoCodecOperationFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoCodecOperationFlagsKHR>>(iterator: T) -> VideoCodecOperationFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoCodecOperationFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for VideoCodecOperationFlagsKHR {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<VideoCodecOperationFlagBitsKHR> for VideoCodecOperationFlagsKHR {
    fn from(bit: VideoCodecOperationFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<VideoCodecOperationFlagBitsKHR> for VideoCodecOperationFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoCodecOperationFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
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
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoCodecOperationFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
#[doc(alias = "VkVideoCapabilityFlagsKHR")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoCapabilityFlagsKHR(u32);
impl VideoCapabilityFlagsKHR {
    #[doc(alias = "VK_VIDEO_CAPABILITY_PROTECTED_CONTENT_BIT_KHR")]
    pub const PROTECTED_CONTENT: Self = Self(1);
    #[doc(alias = "VK_VIDEO_CAPABILITY_SEPARATE_REFERENCE_IMAGES_BIT_KHR")]
    pub const SEPARATE_REFERENCE_IMAGES: Self = Self(2);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
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
impl FromIterator<VideoCapabilityFlagsKHR> for VideoCapabilityFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoCapabilityFlagsKHR>>(iterator: T) -> VideoCapabilityFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoCapabilityFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for VideoCapabilityFlagsKHR {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<VideoCapabilityFlagBitsKHR> for VideoCapabilityFlagsKHR {
    fn from(bit: VideoCapabilityFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<VideoCapabilityFlagBitsKHR> for VideoCapabilityFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoCapabilityFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
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
#[doc(alias = "VkVideoSessionCreateFlagsKHR")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoSessionCreateFlagsKHR(u32);
impl VideoSessionCreateFlagsKHR {
    #[doc(alias = "VK_VIDEO_SESSION_CREATE_DEFAULT_KHR")]
    pub const DEFAULT: Self = Self(0);
    #[doc(alias = "VK_VIDEO_SESSION_CREATE_PROTECTED_CONTENT_BIT_KHR")]
    pub const PROTECTED_CONTENT: Self = Self(1);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
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
impl FromIterator<VideoSessionCreateFlagsKHR> for VideoSessionCreateFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoSessionCreateFlagsKHR>>(iterator: T) -> VideoSessionCreateFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoSessionCreateFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for VideoSessionCreateFlagsKHR {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<VideoSessionCreateFlagBitsKHR> for VideoSessionCreateFlagsKHR {
    fn from(bit: VideoSessionCreateFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<VideoSessionCreateFlagBitsKHR> for VideoSessionCreateFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoSessionCreateFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
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
#[doc(alias = "VkVideoBeginCodingFlagsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoBeginCodingFlagsKHR(u32);
impl VideoBeginCodingFlagsKHR {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
#[doc(alias = "VkVideoEndCodingFlagsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoEndCodingFlagsKHR(u32);
impl VideoEndCodingFlagsKHR {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
#[doc(alias = "VkVideoCodingQualityPresetFlagsKHR")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoCodingQualityPresetFlagsKHR(u32);
impl VideoCodingQualityPresetFlagsKHR {
    #[doc(alias = "VK_VIDEO_CODING_QUALITY_PRESET_NORMAL_BIT_KHR")]
    pub const NORMAL: Self = Self(1);
    #[doc(alias = "VK_VIDEO_CODING_QUALITY_PRESET_POWER_BIT_KHR")]
    pub const POWER: Self = Self(2);
    #[doc(alias = "VK_VIDEO_CODING_QUALITY_PRESET_QUALITY_BIT_KHR")]
    pub const QUALITY: Self = Self(4);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
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
impl FromIterator<VideoCodingQualityPresetFlagsKHR> for VideoCodingQualityPresetFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoCodingQualityPresetFlagsKHR>>(
        iterator: T,
    ) -> VideoCodingQualityPresetFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoCodingQualityPresetFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for VideoCodingQualityPresetFlagsKHR {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<VideoCodingQualityPresetFlagBitsKHR> for VideoCodingQualityPresetFlagsKHR {
    fn from(bit: VideoCodingQualityPresetFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<VideoCodingQualityPresetFlagBitsKHR> for VideoCodingQualityPresetFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoCodingQualityPresetFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
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
#[doc(alias = "VkVideoCodingControlFlagsKHR")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoCodingControlFlagsKHR(u32);
impl VideoCodingControlFlagsKHR {
    #[doc(alias = "VK_VIDEO_CODING_CONTROL_DEFAULT_KHR")]
    pub const DEFAULT: Self = Self(0);
    #[doc(alias = "VK_VIDEO_CODING_CONTROL_RESET_BIT_KHR")]
    pub const RESET: Self = Self(1);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
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
impl FromIterator<VideoCodingControlFlagsKHR> for VideoCodingControlFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoCodingControlFlagsKHR>>(iterator: T) -> VideoCodingControlFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoCodingControlFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for VideoCodingControlFlagsKHR {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<VideoCodingControlFlagBitsKHR> for VideoCodingControlFlagsKHR {
    fn from(bit: VideoCodingControlFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<VideoCodingControlFlagBitsKHR> for VideoCodingControlFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoCodingControlFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
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
#[doc(alias = "VkVideoChromaSubsamplingFlagsKHR")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoChromaSubsamplingFlagsKHR(u32);
impl VideoChromaSubsamplingFlagsKHR {
    #[doc(alias = "VK_VIDEO_CHROMA_SUBSAMPLING_INVALID_BIT_KHR")]
    pub const INVALID: Self = Self(0);
    #[doc(alias = "VK_VIDEO_CHROMA_SUBSAMPLING_MONOCHROME_BIT_KHR")]
    pub const MONOCHROME: Self = Self(1);
    #[doc(alias = "VK_VIDEO_CHROMA_SUBSAMPLING_420_BIT_KHR")]
    pub const N420: Self = Self(2);
    #[doc(alias = "VK_VIDEO_CHROMA_SUBSAMPLING_422_BIT_KHR")]
    pub const N422: Self = Self(4);
    #[doc(alias = "VK_VIDEO_CHROMA_SUBSAMPLING_444_BIT_KHR")]
    pub const N444: Self = Self(8);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
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
            all |= Self::N420;
        }
        {
            all |= Self::N422;
        }
        {
            all |= Self::N444;
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
impl FromIterator<VideoChromaSubsamplingFlagsKHR> for VideoChromaSubsamplingFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoChromaSubsamplingFlagsKHR>>(
        iterator: T,
    ) -> VideoChromaSubsamplingFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoChromaSubsamplingFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for VideoChromaSubsamplingFlagsKHR {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<VideoChromaSubsamplingFlagBitsKHR> for VideoChromaSubsamplingFlagsKHR {
    fn from(bit: VideoChromaSubsamplingFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<VideoChromaSubsamplingFlagBitsKHR> for VideoChromaSubsamplingFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoChromaSubsamplingFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
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
                    if self.0.contains(VideoChromaSubsamplingFlagsKHR::N420) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(N420))?;
                    }
                    if self.0.contains(VideoChromaSubsamplingFlagsKHR::N422) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(N422))?;
                    }
                    if self.0.contains(VideoChromaSubsamplingFlagsKHR::N444) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(N444))?;
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
#[doc(alias = "VkVideoComponentBitDepthFlagsKHR")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoComponentBitDepthFlagsKHR(u32);
impl VideoComponentBitDepthFlagsKHR {
    #[doc(alias = "VK_VIDEO_COMPONENT_BIT_DEPTH_INVALID_KHR")]
    pub const VIDEO_COMPONENT_DEPTH_INVALID: Self = Self(0);
    #[doc(alias = "VK_VIDEO_COMPONENT_BIT_DEPTH_8_BIT_KHR")]
    pub const VIDEO_COMPONENT_DEPTH8: Self = Self(1);
    #[doc(alias = "VK_VIDEO_COMPONENT_BIT_DEPTH_10_BIT_KHR")]
    pub const VIDEO_COMPONENT_DEPTH10: Self = Self(4);
    #[doc(alias = "VK_VIDEO_COMPONENT_BIT_DEPTH_12_BIT_KHR")]
    pub const VIDEO_COMPONENT_DEPTH12: Self = Self(16);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
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
impl FromIterator<VideoComponentBitDepthFlagsKHR> for VideoComponentBitDepthFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoComponentBitDepthFlagsKHR>>(
        iterator: T,
    ) -> VideoComponentBitDepthFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoComponentBitDepthFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for VideoComponentBitDepthFlagsKHR {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<VideoComponentBitDepthFlagBitsKHR> for VideoComponentBitDepthFlagsKHR {
    fn from(bit: VideoComponentBitDepthFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<VideoComponentBitDepthFlagBitsKHR> for VideoComponentBitDepthFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoComponentBitDepthFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
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
#[doc(alias = "VK_KHR_VIDEO_QUEUE_SPEC_VERSION")]
pub const KHR_VIDEO_QUEUE_SPEC_VERSION: u32 = 3;
#[doc(alias = "VK_KHR_VIDEO_QUEUE_EXTENSION_NAME")]
pub const KHR_VIDEO_QUEUE_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_video_queue");
#[doc(alias = "VkVideoCodecOperationFlagBitsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct VideoCodecOperationFlagBitsKHR(u32);
impl VideoCodecOperationFlagBitsKHR {
    #[doc(alias = "VK_VIDEO_CODEC_OPERATION_INVALID_BIT_KHR")]
    pub const INVALID: Self = Self(0);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        match bits {
            x if x == Self::INVALID.bits() => Some(Self(x)),
            #[cfg(feature = "VK_EXT_video_encode_h264")]
            x if x == Self::ENCODE_H264_EXT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_EXT_video_encode_h265")]
            x if x == Self::ENCODE_H265_EXT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_EXT_video_decode_h264")]
            x if x == Self::DECODE_H264_EXT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_EXT_video_decode_h265")]
            x if x == Self::DECODE_H265_EXT.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
#[doc(alias = "VkVideoChromaSubsamplingFlagBitsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct VideoChromaSubsamplingFlagBitsKHR(u32);
impl VideoChromaSubsamplingFlagBitsKHR {
    #[doc(alias = "VK_VIDEO_CHROMA_SUBSAMPLING_INVALID_BIT_KHR")]
    pub const INVALID: Self = Self(0);
    #[doc(alias = "VK_VIDEO_CHROMA_SUBSAMPLING_MONOCHROME_BIT_KHR")]
    pub const MONOCHROME: Self = Self(1);
    #[doc(alias = "VK_VIDEO_CHROMA_SUBSAMPLING_420_BIT_KHR")]
    pub const N420: Self = Self(2);
    #[doc(alias = "VK_VIDEO_CHROMA_SUBSAMPLING_422_BIT_KHR")]
    pub const N422: Self = Self(4);
    #[doc(alias = "VK_VIDEO_CHROMA_SUBSAMPLING_444_BIT_KHR")]
    pub const N444: Self = Self(8);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        match bits {
            x if x == Self::INVALID.bits() => Some(Self(x)),
            x if x == Self::MONOCHROME.bits() => Some(Self(x)),
            x if x == Self::N420.bits() => Some(Self(x)),
            x if x == Self::N422.bits() => Some(Self(x)),
            x if x == Self::N444.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
#[doc(alias = "VkVideoComponentBitDepthFlagBitsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct VideoComponentBitDepthFlagBitsKHR(u32);
impl VideoComponentBitDepthFlagBitsKHR {
    #[doc(alias = "VK_VIDEO_COMPONENT_BIT_DEPTH_INVALID_KHR")]
    pub const VIDEO_COMPONENT_DEPTH_INVALID: Self = Self(0);
    #[doc(alias = "VK_VIDEO_COMPONENT_BIT_DEPTH_8_BIT_KHR")]
    pub const VIDEO_COMPONENT_DEPTH8: Self = Self(1);
    #[doc(alias = "VK_VIDEO_COMPONENT_BIT_DEPTH_10_BIT_KHR")]
    pub const VIDEO_COMPONENT_DEPTH10: Self = Self(4);
    #[doc(alias = "VK_VIDEO_COMPONENT_BIT_DEPTH_12_BIT_KHR")]
    pub const VIDEO_COMPONENT_DEPTH12: Self = Self(16);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        match bits {
            x if x == Self::VIDEO_COMPONENT_DEPTH_INVALID.bits() => Some(Self(x)),
            x if x == Self::VIDEO_COMPONENT_DEPTH8.bits() => Some(Self(x)),
            x if x == Self::VIDEO_COMPONENT_DEPTH10.bits() => Some(Self(x)),
            x if x == Self::VIDEO_COMPONENT_DEPTH12.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
#[doc(alias = "VkVideoCapabilityFlagBitsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct VideoCapabilityFlagBitsKHR(u32);
impl VideoCapabilityFlagBitsKHR {
    #[doc(alias = "VK_VIDEO_CAPABILITY_PROTECTED_CONTENT_BIT_KHR")]
    pub const PROTECTED_CONTENT: Self = Self(1);
    #[doc(alias = "VK_VIDEO_CAPABILITY_SEPARATE_REFERENCE_IMAGES_BIT_KHR")]
    pub const SEPARATE_REFERENCE_IMAGES: Self = Self(2);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        match bits {
            x if x == Self::PROTECTED_CONTENT.bits() => Some(Self(x)),
            x if x == Self::SEPARATE_REFERENCE_IMAGES.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
#[doc(alias = "VkVideoSessionCreateFlagBitsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct VideoSessionCreateFlagBitsKHR(u32);
impl VideoSessionCreateFlagBitsKHR {
    #[doc(alias = "VK_VIDEO_SESSION_CREATE_DEFAULT_KHR")]
    pub const DEFAULT: Self = Self(0);
    #[doc(alias = "VK_VIDEO_SESSION_CREATE_PROTECTED_CONTENT_BIT_KHR")]
    pub const PROTECTED_CONTENT: Self = Self(1);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        match bits {
            x if x == Self::DEFAULT.bits() => Some(Self(x)),
            x if x == Self::PROTECTED_CONTENT.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
#[doc(alias = "VkVideoCodingQualityPresetFlagBitsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct VideoCodingQualityPresetFlagBitsKHR(u32);
impl VideoCodingQualityPresetFlagBitsKHR {
    #[doc(alias = "VK_VIDEO_CODING_QUALITY_PRESET_NORMAL_BIT_KHR")]
    pub const NORMAL: Self = Self(1);
    #[doc(alias = "VK_VIDEO_CODING_QUALITY_PRESET_POWER_BIT_KHR")]
    pub const POWER: Self = Self(2);
    #[doc(alias = "VK_VIDEO_CODING_QUALITY_PRESET_QUALITY_BIT_KHR")]
    pub const QUALITY: Self = Self(4);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        match bits {
            x if x == Self::NORMAL.bits() => Some(Self(x)),
            x if x == Self::POWER.bits() => Some(Self(x)),
            x if x == Self::QUALITY.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
#[doc(alias = "VkVideoCodingControlFlagBitsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct VideoCodingControlFlagBitsKHR(u32);
impl VideoCodingControlFlagBitsKHR {
    #[doc(alias = "VK_VIDEO_CODING_CONTROL_DEFAULT_KHR")]
    pub const DEFAULT: Self = Self(0);
    #[doc(alias = "VK_VIDEO_CODING_CONTROL_RESET_BIT_KHR")]
    pub const RESET: Self = Self(1);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        match bits {
            x if x == Self::DEFAULT.bits() => Some(Self(x)),
            x if x == Self::RESET.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
#[doc(alias = "VkQueryResultStatusKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct QueryResultStatusKHR(i32);
impl QueryResultStatusKHR {
    #[doc(alias = "VK_QUERY_RESULT_STATUS_ERROR_KHR")]
    pub const ERROR: Self = Self(-1);
    #[doc(alias = "VK_QUERY_RESULT_STATUS_NOT_READY_KHR")]
    pub const NOT_READY: Self = Self(0);
    #[doc(alias = "VK_QUERY_RESULT_STATUS_COMPLETE_KHR")]
    pub const COMPLETE: Self = Self(1);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::ERROR.bits() => Some(Self(x)),
            x if x == Self::NOT_READY.bits() => Some(Self(x)),
            x if x == Self::COMPLETE.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
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
