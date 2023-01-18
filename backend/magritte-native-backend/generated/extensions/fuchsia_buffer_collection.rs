use crate::{
    cstr,
    opaque::zx_handle_t,
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, BaseOutStructure, BufferCreateInfo, ComponentMapping, Device,
        FormatFeatureFlags, ImageCreateInfo, StructureType, VulkanResultCodes,
    },
    vulkan1_1::{ChromaLocation, SamplerYcbcrModelConversion, SamplerYcbcrRange},
};
use std::ffi::CStr;
#[doc(alias = "VkImportMemoryBufferCollectionFUCHSIA")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImportMemoryBufferCollectionFUCHSIA {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    collection: BufferCollectionFUCHSIA,
    index: u32,
}
#[doc(alias = "VkBufferCollectionImageCreateInfoFUCHSIA")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BufferCollectionImageCreateInfoFUCHSIA {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    collection: BufferCollectionFUCHSIA,
    index: u32,
}
#[doc(alias = "VkBufferCollectionBufferCreateInfoFUCHSIA")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BufferCollectionBufferCreateInfoFUCHSIA {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    collection: BufferCollectionFUCHSIA,
    index: u32,
}
#[doc(alias = "VkBufferCollectionCreateInfoFUCHSIA")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BufferCollectionCreateInfoFUCHSIA {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "collectionToken")]
    collection_token: zx_handle_t,
}
#[doc(alias = "VkBufferCollectionPropertiesFUCHSIA")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BufferCollectionPropertiesFUCHSIA {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "memoryTypeBits")]
    memory_type_bits: u32,
    #[doc(alias = "bufferCount")]
    buffer_count: u32,
    #[doc(alias = "createInfoIndex")]
    create_info_index: u32,
    #[doc(alias = "sysmemPixelFormat")]
    sysmem_pixel_format: u64,
    #[doc(alias = "formatFeatures")]
    format_features: FormatFeatureFlags,
    #[doc(alias = "sysmemColorSpaceIndex")]
    sysmem_color_space_index: SysmemColorSpaceFUCHSIA,
    #[doc(alias = "samplerYcbcrConversionComponents")]
    sampler_ycbcr_conversion_components: ComponentMapping,
    #[doc(alias = "suggestedYcbcrModel")]
    suggested_ycbcr_model: SamplerYcbcrModelConversion,
    #[doc(alias = "suggestedYcbcrRange")]
    suggested_ycbcr_range: SamplerYcbcrRange,
    #[doc(alias = "suggestedXChromaOffset")]
    suggested_x_chroma_offset: ChromaLocation,
    #[doc(alias = "suggestedYChromaOffset")]
    suggested_y_chroma_offset: ChromaLocation,
}
#[doc(alias = "VkBufferConstraintsInfoFUCHSIA")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BufferConstraintsInfoFUCHSIA {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "createInfo")]
    create_info: BufferCreateInfo,
    #[doc(alias = "requiredFormatFeatures")]
    required_format_features: FormatFeatureFlags,
    #[doc(alias = "bufferCollectionConstraints")]
    buffer_collection_constraints: BufferCollectionConstraintsInfoFUCHSIA,
}
#[doc(alias = "VkSysmemColorSpaceFUCHSIA")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SysmemColorSpaceFUCHSIA {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "colorSpace")]
    color_space: u32,
}
#[doc(alias = "VkImageFormatConstraintsInfoFUCHSIA")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageFormatConstraintsInfoFUCHSIA {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "imageCreateInfo")]
    image_create_info: ImageCreateInfo,
    #[doc(alias = "requiredFormatFeatures")]
    required_format_features: FormatFeatureFlags,
    flags: ImageFormatConstraintsFlagsFUCHSIA,
    #[doc(alias = "sysmemPixelFormat")]
    sysmem_pixel_format: u64,
    #[doc(alias = "colorSpaceCount")]
    color_space_count: u32,
    #[doc(alias = "pColorSpaces")]
    color_spaces: *const SysmemColorSpaceFUCHSIA,
}
#[doc(alias = "VkImageConstraintsInfoFUCHSIA")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageConstraintsInfoFUCHSIA {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "formatConstraintsCount")]
    format_constraints_count: u32,
    #[doc(alias = "pFormatConstraints")]
    format_constraints: *const ImageFormatConstraintsInfoFUCHSIA,
    #[doc(alias = "bufferCollectionConstraints")]
    buffer_collection_constraints: BufferCollectionConstraintsInfoFUCHSIA,
    flags: ImageConstraintsInfoFlagsFUCHSIA,
}
#[doc(alias = "VkBufferCollectionConstraintsInfoFUCHSIA")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BufferCollectionConstraintsInfoFUCHSIA {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "minBufferCount")]
    min_buffer_count: u32,
    #[doc(alias = "maxBufferCount")]
    max_buffer_count: u32,
    #[doc(alias = "minBufferCountForCamping")]
    min_buffer_count_for_camping: u32,
    #[doc(alias = "minBufferCountForDedicatedSlack")]
    min_buffer_count_for_dedicated_slack: u32,
    #[doc(alias = "minBufferCountForSharedSlack")]
    min_buffer_count_for_shared_slack: u32,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkBufferCollectionFUCHSIA")]
#[repr(transparent)]
pub struct BufferCollectionFUCHSIA(u64);
impl BufferCollectionFUCHSIA {
    pub const fn null() -> Self {
        Self(0)
    }
}
impl const Default for BufferCollectionFUCHSIA {
    fn default() -> Self {
        Self::null()
    }
}
#[doc(alias = "VkImageFormatConstraintsFlagsFUCHSIA")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ImageFormatConstraintsFlagsFUCHSIA(u32);
impl ImageFormatConstraintsFlagsFUCHSIA {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
#[doc(alias = "VkImageConstraintsInfoFlagsFUCHSIA")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ImageConstraintsInfoFlagsFUCHSIA(u32);
impl ImageConstraintsInfoFlagsFUCHSIA {
    #[doc(alias = "VK_IMAGE_CONSTRAINTS_INFO_CPU_READ_RARELY_FUCHSIA")]
    pub const CPU_READ_RARELY: Self = Self(1);
    #[doc(alias = "VK_IMAGE_CONSTRAINTS_INFO_CPU_READ_OFTEN_FUCHSIA")]
    pub const CPU_READ_OFTEN: Self = Self(2);
    #[doc(alias = "VK_IMAGE_CONSTRAINTS_INFO_CPU_WRITE_RARELY_FUCHSIA")]
    pub const CPU_WRITE_RARELY: Self = Self(4);
    #[doc(alias = "VK_IMAGE_CONSTRAINTS_INFO_CPU_WRITE_OFTEN_FUCHSIA")]
    pub const CPU_WRITE_OFTEN: Self = Self(8);
    #[doc(alias = "VK_IMAGE_CONSTRAINTS_INFO_PROTECTED_OPTIONAL_FUCHSIA")]
    pub const PROTECTED_OPTIONAL: Self = Self(16);
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
            all |= Self::CPU_READ_RARELY;
        }
        {
            all |= Self::CPU_READ_OFTEN;
        }
        {
            all |= Self::CPU_WRITE_RARELY;
        }
        {
            all |= Self::CPU_WRITE_OFTEN;
        }
        {
            all |= Self::PROTECTED_OPTIONAL;
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
impl const std::ops::BitOr for ImageConstraintsInfoFlagsFUCHSIA {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for ImageConstraintsInfoFlagsFUCHSIA {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for ImageConstraintsInfoFlagsFUCHSIA {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for ImageConstraintsInfoFlagsFUCHSIA {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for ImageConstraintsInfoFlagsFUCHSIA {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for ImageConstraintsInfoFlagsFUCHSIA {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for ImageConstraintsInfoFlagsFUCHSIA {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for ImageConstraintsInfoFlagsFUCHSIA {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for ImageConstraintsInfoFlagsFUCHSIA {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<ImageConstraintsInfoFlagsFUCHSIA> for ImageConstraintsInfoFlagsFUCHSIA {
    fn extend<T: IntoIterator<Item = ImageConstraintsInfoFlagsFUCHSIA>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<ImageConstraintsInfoFlagsFUCHSIA> for ImageConstraintsInfoFlagsFUCHSIA {
    fn from_iter<T: IntoIterator<Item = ImageConstraintsInfoFlagsFUCHSIA>>(
        iterator: T,
    ) -> ImageConstraintsInfoFlagsFUCHSIA {
        let mut out = Self::empty();
        <Self as Extend<ImageConstraintsInfoFlagsFUCHSIA>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for ImageConstraintsInfoFlagsFUCHSIA {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<ImageConstraintsInfoFlagBitsFUCHSIA> for ImageConstraintsInfoFlagsFUCHSIA {
    fn from(bit: ImageConstraintsInfoFlagBitsFUCHSIA) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<ImageConstraintsInfoFlagBitsFUCHSIA> for ImageConstraintsInfoFlagsFUCHSIA {
    fn extend<T: IntoIterator<Item = ImageConstraintsInfoFlagBitsFUCHSIA>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<ImageConstraintsInfoFlagBitsFUCHSIA> for ImageConstraintsInfoFlagsFUCHSIA {
    fn from_iter<T: IntoIterator<Item = ImageConstraintsInfoFlagBitsFUCHSIA>>(
        iterator: T,
    ) -> ImageConstraintsInfoFlagsFUCHSIA {
        let mut out = Self::empty();
        <Self as Extend<ImageConstraintsInfoFlagBitsFUCHSIA>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for ImageConstraintsInfoFlagsFUCHSIA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(ImageConstraintsInfoFlagsFUCHSIA);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == ImageConstraintsInfoFlagsFUCHSIA::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(ImageConstraintsInfoFlagsFUCHSIA::CPU_READ_RARELY) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(CPU_READ_RARELY))?;
                    }
                    if self.0.contains(ImageConstraintsInfoFlagsFUCHSIA::CPU_READ_OFTEN) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(CPU_READ_OFTEN))?;
                    }
                    if self.0.contains(ImageConstraintsInfoFlagsFUCHSIA::CPU_WRITE_RARELY) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(CPU_WRITE_RARELY))?;
                    }
                    if self.0.contains(ImageConstraintsInfoFlagsFUCHSIA::CPU_WRITE_OFTEN) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(CPU_WRITE_OFTEN))?;
                    }
                    if self.0.contains(ImageConstraintsInfoFlagsFUCHSIA::PROTECTED_OPTIONAL) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PROTECTED_OPTIONAL))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(ImageConstraintsInfoFlagsFUCHSIA))
            .field(&Flags(*self))
            .finish()
    }
}
#[doc(alias = "VK_FUCHSIA_BUFFER_COLLECTION_SPEC_VERSION")]
pub const FUCHSIA_BUFFER_COLLECTION_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_FUCHSIA_BUFFER_COLLECTION_EXTENSION_NAME")]
pub const FUCHSIA_BUFFER_COLLECTION_EXTENSION_NAME: &'static CStr = cstr!("VK_FUCHSIA_buffer_collection");
#[doc(alias = "VkImageConstraintsInfoFlagBitsFUCHSIA")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ImageConstraintsInfoFlagBitsFUCHSIA(u32);
impl ImageConstraintsInfoFlagBitsFUCHSIA {
    #[doc(alias = "VK_IMAGE_CONSTRAINTS_INFO_CPU_READ_RARELY_FUCHSIA")]
    pub const CPU_READ_RARELY: Self = Self(1);
    #[doc(alias = "VK_IMAGE_CONSTRAINTS_INFO_CPU_READ_OFTEN_FUCHSIA")]
    pub const CPU_READ_OFTEN: Self = Self(2);
    #[doc(alias = "VK_IMAGE_CONSTRAINTS_INFO_CPU_WRITE_RARELY_FUCHSIA")]
    pub const CPU_WRITE_RARELY: Self = Self(4);
    #[doc(alias = "VK_IMAGE_CONSTRAINTS_INFO_CPU_WRITE_OFTEN_FUCHSIA")]
    pub const CPU_WRITE_OFTEN: Self = Self(8);
    #[doc(alias = "VK_IMAGE_CONSTRAINTS_INFO_PROTECTED_OPTIONAL_FUCHSIA")]
    pub const PROTECTED_OPTIONAL: Self = Self(16);
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
            x if x == Self::CPU_READ_RARELY.bits() => Some(Self(x)),
            x if x == Self::CPU_READ_OFTEN.bits() => Some(Self(x)),
            x if x == Self::CPU_WRITE_RARELY.bits() => Some(Self(x)),
            x if x == Self::CPU_WRITE_OFTEN.bits() => Some(Self(x)),
            x if x == Self::PROTECTED_OPTIONAL.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
#[doc(alias = "vkCreateBufferCollectionFUCHSIA")]
pub type FNCreateBufferCollectionFuchsia = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const BufferCollectionCreateInfoFUCHSIA,
    p_allocator: *const AllocationCallbacks,
    p_collection: *mut BufferCollectionFUCHSIA,
) -> VulkanResultCodes;
#[doc(alias = "vkSetBufferCollectionBufferConstraintsFUCHSIA")]
pub type FNSetBufferCollectionBufferConstraintsFuchsia = unsafe extern "system" fn(
    device: Device,
    collection: BufferCollectionFUCHSIA,
    p_buffer_constraints_info: *const BufferConstraintsInfoFUCHSIA,
) -> VulkanResultCodes;
#[doc(alias = "vkSetBufferCollectionImageConstraintsFUCHSIA")]
pub type FNSetBufferCollectionImageConstraintsFuchsia = unsafe extern "system" fn(
    device: Device,
    collection: BufferCollectionFUCHSIA,
    p_image_constraints_info: *const ImageConstraintsInfoFUCHSIA,
) -> VulkanResultCodes;
#[doc(alias = "vkDestroyBufferCollectionFUCHSIA")]
pub type FNDestroyBufferCollectionFuchsia = unsafe extern "system" fn(
    device: Device,
    collection: BufferCollectionFUCHSIA,
    p_allocator: *const AllocationCallbacks,
);
#[doc(alias = "vkGetBufferCollectionPropertiesFUCHSIA")]
pub type FNGetBufferCollectionPropertiesFuchsia = unsafe extern "system" fn(
    device: Device,
    collection: BufferCollectionFUCHSIA,
    p_properties: *mut BufferCollectionPropertiesFUCHSIA,
) -> VulkanResultCodes;
