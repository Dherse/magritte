//![VK_EXT_video_decode_h264](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_video_decode_h264.html) - device extension
//!# Revision
//!4
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_video_decode_queue`]`
//! - **This is a *provisional* extension and  **must**  be used with caution. See the [description](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#boilerplate-provisional-header)
//!   of provisional header files for enablement and stability details.**
//!# Contacts
//! - [peter.fang@amd.com]()
//!# New structures
//! - Extending [`VideoDecodeCapabilitiesKHR`]:  - [`VideoDecodeH264CapabilitiesEXT`]
//! - Extending [`VideoDecodeH264PictureInfoEXT`]:  - [`VideoDecodeH264MvcEXT`]
//! - Extending [`VideoDecodeInfoKHR`]:  - [`VideoDecodeH264PictureInfoEXT`]
//! - Extending [`VideoProfileKHR`], [`QueryPoolCreateInfo`], [`FormatProperties2`],
//!   [`ImageCreateInfo`], [`ImageViewCreateInfo`], [`BufferCreateInfo`]:  -
//!   [`VideoDecodeH264ProfileEXT`]
//! - Extending [`VideoReferenceSlotKHR`]:  - [`VideoDecodeH264DpbSlotInfoEXT`]
//! - Extending [`VideoSessionParametersCreateInfoKHR`]:  -
//!   [`VideoDecodeH264SessionParametersCreateInfoEXT`]
//! - Extending [`VideoSessionParametersUpdateInfoKHR`]:  -
//!   [`VideoDecodeH264SessionParametersAddInfoEXT`]
//!# New enums
//! - [`VideoDecodeH264PictureLayoutFlagBitsEXT`]
//!# New bitmasks
//! - [`VideoDecodeH264PictureLayoutFlagsEXT`]
//!# New constants
//! - [`EXT_VIDEO_DECODE_H264_EXTENSION_NAME`]
//! - [`EXT_VIDEO_DECODE_H264_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_CAPABILITIES_EXT`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_DPB_SLOT_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_MVC_EXT`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_PICTURE_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_PROFILE_EXT`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_SESSION_PARAMETERS_ADD_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_SESSION_PARAMETERS_CREATE_INFO_EXT`
//! - Extending [`VideoCodecOperationFlagBitsKHR`]:  -
//!   `VK_VIDEO_CODEC_OPERATION_DECODE_H264_BIT_EXT`
//!# Version History
//! - Revision 1, 2018-6-11 (Peter Fang)  - Initial draft
//! - Revision 2, March 29 2021 (Tony Zlatinski)  - Spec and API Updates
//! - Revision 3, August 1 2021 (Srinath Kumarapuram)  - Rename
//!   `VkVideoDecodeH264FieldLayoutFlagsEXT` to [`VideoDecodeH264PictureLayoutFlagsEXT`],
//!   `VkVideoDecodeH264FieldLayoutFlagBitsEXT` to [`VideoDecodeH264PictureLayoutFlagBitsEXT`]
//!   (along with the names of enumerants it defines), and `VkVideoDecodeH264ProfileEXT.fieldLayout`
//!   to `VkVideoDecodeH264ProfileEXT.pictureLayout`, following Vulkan naming conventions.
//! - Revision 4, 2022-03-16 (Ahmed Abdelkhalek)  - Relocate Std header version reporting/requesting
//!   from this extension to VK_KHR_video_queue extension.  - Remove the now empty
//!   VkVideoDecodeH264SessionCreateInfoEXT.
//!# Other info
//! * 2022-03-16
//! * No known IP claims.
//! * - Ahmed Abdelkhalek, AMD  - Chunbo Chen, Intel  - HoHin Lau, AMD  - Jake Beju, AMD  - Peter
//!   Fang, AMD  - Ping Liu, Intel  - Srinath Kumarapuram, NVIDIA  - Tony Zlatinski, NVIDIA
//!# Related
//! - [`VideoDecodeH264CapabilitiesEXT`]
//! - [`VideoDecodeH264DpbSlotInfoEXT`]
//! - [`VideoDecodeH264MvcEXT`]
//! - [`VideoDecodeH264PictureInfoEXT`]
//! - [`VideoDecodeH264PictureLayoutFlagBitsEXT`]
//! - [`VideoDecodeH264PictureLayoutFlagsEXT`]
//! - [`VideoDecodeH264ProfileEXT`]
//! - [`VideoDecodeH264SessionParametersAddInfoEXT`]
//! - [`VideoDecodeH264SessionParametersCreateInfoEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    native::{
        StdVideoDecodeH264Mvc, StdVideoDecodeH264PictureInfo, StdVideoDecodeH264ReferenceInfo,
        StdVideoH264PictureParameterSet, StdVideoH264ProfileIdc, StdVideoH264SequenceParameterSet,
    },
    vulkan1_0::{BaseInStructure, BaseOutStructure, Offset2D, StructureType},
};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    ffi::CStr,
    iter::{Extend, FromIterator, IntoIterator},
    marker::PhantomData,
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_VIDEO_DECODE_H264_SPEC_VERSION")]
pub const EXT_VIDEO_DECODE_H264_SPEC_VERSION: u32 = 4;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_VIDEO_DECODE_H264_EXTENSION_NAME")]
pub const EXT_VIDEO_DECODE_H264_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_video_decode_h264");
///[VkVideoDecodeH264PictureLayoutFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH264PictureLayoutFlagBitsEXT.html) - H.264 video decode picture layout flags
///# C Specifications
///The H.264 video decode picture layout flags are defined with the following
///enum:
///```c
///// Provided by VK_EXT_video_decode_h264
///typedef enum VkVideoDecodeH264PictureLayoutFlagBitsEXT {
///    VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_PROGRESSIVE_EXT = 0,
///    VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_INTERLACED_INTERLEAVED_LINES_BIT_EXT = 0x00000001,
///    VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_INTERLACED_SEPARATE_PLANES_BIT_EXT = 0x00000002,
///} VkVideoDecodeH264PictureLayoutFlagBitsEXT;
///```
///# Description
/// - [`PROGRESSIVE`] specifies support for progressive content. This flag has the value `0`.
/// - [`INTERLACED_INTERLEAVED_LINES`] specifies support for or use of a picture layout for
///   interlaced content where all lines belonging to the first field are decoded to the
///   even-numbered lines within the picture resource, and all lines belonging to the second field
///   are decoded to the odd-numbered lines within the picture resource.
/// - [`INTERLACED_SEPARATE_PLANES`] specifies support for or use of a picture layout for interlaced
///   content where all lines belonging to the first field are grouped together in a single plane,
///   followed by another plane containing all lines belonging to the second field.
///# Related
/// - [`VK_EXT_video_decode_h264`]
/// - [`VideoDecodeH264PictureLayoutFlagsEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoDecodeH264PictureLayoutFlagBitsEXT")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoDecodeH264PictureLayoutFlagBitsEXT(u32);
impl const Default for VideoDecodeH264PictureLayoutFlagBitsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl VideoDecodeH264PictureLayoutFlagBitsEXT {
    ///[`PROGRESSIVE`] specifies
    ///support for progressive content.
    ///This flag has the value `0`.
    pub const PROGRESSIVE: Self = Self(0);
    ///[`INTERLACED_INTERLEAVED_LINES`]
    ///specifies support for or use of a picture layout for interlaced content
    ///where all lines belonging to the first field are decoded to the
    ///even-numbered lines within the picture resource, and all lines belonging
    ///to the second field are decoded to the odd-numbered lines within the
    ///picture resource.
    pub const INTERLACED_INTERLEAVED_LINES: Self = Self(1);
    ///[`INTERLACED_SEPARATE_PLANES`]
    ///specifies support for or use of a picture layout for interlaced content
    ///where all lines belonging to the first field are grouped together in a
    ///single plane, followed by another plane containing all lines belonging
    ///to the second field.
    pub const INTERLACED_SEPARATE_PLANES: Self = Self(2);
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
///[VkVideoDecodeH264PictureLayoutFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH264PictureLayoutFlagBitsEXT.html) - H.264 video decode picture layout flags
///# C Specifications
///The H.264 video decode picture layout flags are defined with the following
///enum:
///```c
///// Provided by VK_EXT_video_decode_h264
///typedef enum VkVideoDecodeH264PictureLayoutFlagBitsEXT {
///    VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_PROGRESSIVE_EXT = 0,
///    VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_INTERLACED_INTERLEAVED_LINES_BIT_EXT = 0x00000001,
///    VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_INTERLACED_SEPARATE_PLANES_BIT_EXT = 0x00000002,
///} VkVideoDecodeH264PictureLayoutFlagBitsEXT;
///```
///# Description
/// - [`PROGRESSIVE`] specifies support for progressive content. This flag has the value `0`.
/// - [`INTERLACED_INTERLEAVED_LINES`] specifies support for or use of a picture layout for
///   interlaced content where all lines belonging to the first field are decoded to the
///   even-numbered lines within the picture resource, and all lines belonging to the second field
///   are decoded to the odd-numbered lines within the picture resource.
/// - [`INTERLACED_SEPARATE_PLANES`] specifies support for or use of a picture layout for interlaced
///   content where all lines belonging to the first field are grouped together in a single plane,
///   followed by another plane containing all lines belonging to the second field.
///# Related
/// - [`VK_EXT_video_decode_h264`]
/// - [`VideoDecodeH264PictureLayoutFlagsEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoDecodeH264PictureLayoutFlagsEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoDecodeH264PictureLayoutFlagsEXT(u32);
impl const Default for VideoDecodeH264PictureLayoutFlagsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl From<VideoDecodeH264PictureLayoutFlagBitsEXT> for VideoDecodeH264PictureLayoutFlagsEXT {
    fn from(from: VideoDecodeH264PictureLayoutFlagBitsEXT) -> Self {
        unsafe { Self::from_bits_unchecked(from.bits()) }
    }
}
impl VideoDecodeH264PictureLayoutFlagsEXT {
    ///[`PROGRESSIVE`] specifies
    ///support for progressive content.
    ///This flag has the value `0`.
    pub const PROGRESSIVE: Self = Self(0);
    ///[`INTERLACED_INTERLEAVED_LINES`]
    ///specifies support for or use of a picture layout for interlaced content
    ///where all lines belonging to the first field are decoded to the
    ///even-numbered lines within the picture resource, and all lines belonging
    ///to the second field are decoded to the odd-numbered lines within the
    ///picture resource.
    pub const INTERLACED_INTERLEAVED_LINES: Self = Self(1);
    ///[`INTERLACED_SEPARATE_PLANES`]
    ///specifies support for or use of a picture layout for interlaced content
    ///where all lines belonging to the first field are grouped together in a
    ///single plane, followed by another plane containing all lines belonging
    ///to the second field.
    pub const INTERLACED_SEPARATE_PLANES: Self = Self(2);
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
            all |= Self::PROGRESSIVE;
        }
        {
            all |= Self::INTERLACED_INTERLEAVED_LINES;
        }
        {
            all |= Self::INTERLACED_SEPARATE_PLANES;
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
impl const std::ops::BitOr for VideoDecodeH264PictureLayoutFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for VideoDecodeH264PictureLayoutFlagsEXT {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for VideoDecodeH264PictureLayoutFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for VideoDecodeH264PictureLayoutFlagsEXT {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for VideoDecodeH264PictureLayoutFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for VideoDecodeH264PictureLayoutFlagsEXT {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for VideoDecodeH264PictureLayoutFlagsEXT {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for VideoDecodeH264PictureLayoutFlagsEXT {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for VideoDecodeH264PictureLayoutFlagsEXT {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<VideoDecodeH264PictureLayoutFlagsEXT> for VideoDecodeH264PictureLayoutFlagsEXT {
    fn extend<T: IntoIterator<Item = VideoDecodeH264PictureLayoutFlagsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<VideoDecodeH264PictureLayoutFlagBitsEXT> for VideoDecodeH264PictureLayoutFlagsEXT {
    fn extend<T: IntoIterator<Item = VideoDecodeH264PictureLayoutFlagBitsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<VideoDecodeH264PictureLayoutFlagBitsEXT>>::from(i));
        }
    }
}
impl FromIterator<VideoDecodeH264PictureLayoutFlagsEXT> for VideoDecodeH264PictureLayoutFlagsEXT {
    fn from_iter<T: IntoIterator<Item = VideoDecodeH264PictureLayoutFlagsEXT>>(
        iterator: T,
    ) -> VideoDecodeH264PictureLayoutFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<VideoDecodeH264PictureLayoutFlagsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<VideoDecodeH264PictureLayoutFlagBitsEXT> for VideoDecodeH264PictureLayoutFlagsEXT {
    fn from_iter<T: IntoIterator<Item = VideoDecodeH264PictureLayoutFlagBitsEXT>>(
        iterator: T,
    ) -> VideoDecodeH264PictureLayoutFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<VideoDecodeH264PictureLayoutFlagBitsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for VideoDecodeH264PictureLayoutFlagsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoDecodeH264PictureLayoutFlagsEXT);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoDecodeH264PictureLayoutFlagsEXT::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(VideoDecodeH264PictureLayoutFlagsEXT::PROGRESSIVE) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(PROGRESSIVE))?;
                    }
                    if self
                        .0
                        .contains(VideoDecodeH264PictureLayoutFlagsEXT::INTERLACED_INTERLEAVED_LINES)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(INTERLACED_INTERLEAVED_LINES))?;
                    }
                    if self
                        .0
                        .contains(VideoDecodeH264PictureLayoutFlagsEXT::INTERLACED_SEPARATE_PLANES)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(INTERLACED_SEPARATE_PLANES))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoDecodeH264PictureLayoutFlagsEXT))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoDecodeH264ProfileEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH264ProfileEXT.html) - Structure specifying H.264 decode profile
///# C Specifications
///The [`VideoDecodeH264ProfileEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_video_decode_h264
///typedef struct VkVideoDecodeH264ProfileEXT {
///    VkStructureType                           sType;
///    const void*                               pNext;
///    StdVideoH264ProfileIdc                    stdProfileIdc;
///    VkVideoDecodeH264PictureLayoutFlagsEXT    pictureLayout;
///} VkVideoDecodeH264ProfileEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`std_profile_idc`] is a [`StdVideoH264ProfileIdc`] value specifying the H.264 codec profile
///   IDC
/// - [`picture_layout`] is a bitmask of [`VideoDecodeH264PictureLayoutFlagBitsEXT`] specifying the
///   layout of the decoded picture’s contents depending on the nature (progressive vs. interlaced)
///   of the input content.
///# Description
///## Valid Usage
/// - If the [`VideoDecodeH264ProfileEXT`] structure is included in the [`p_next`] chain of the
///   [`VideoCapabilitiesKHR`] structure passed to [`get_physical_device_video_capabilities_khr`],
///   the value in [`picture_layout`] is treated as a bitmask of requested picture layouts. It is
///   always valid to use the value `VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_PROGRESSIVE_EXT` as the
///   implementation is guaranteed to support decoding of progressive content.
/// - If the [`VideoDecodeH264ProfileEXT`] structure is included in the [`p_next`] chain of the
///   [`VideoSessionCreateInfoKHR`] structure passed to [`create_video_session_khr`], the value in
///   [`picture_layout`] **must**  be exactly one of
///   `VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_PROGRESSIVE_EXT`,
///   `VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_INTERLACED_INTERLEAVED_LINES_BIT_EXT` or
///   `VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_INTERLACED_SEPARATE_PLANES_BIT_EXT`.
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_PROFILE_EXT`
///# Related
/// - [`VK_EXT_video_decode_h264`]
/// - [`StructureType`]
/// - [`VideoDecodeH264PictureLayoutFlagsEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoDecodeH264ProfileEXT")]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoDecodeH264ProfileEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`std_profile_idc`] is a [`StdVideoH264ProfileIdc`] value specifying
    ///the H.264 codec profile IDC
    pub std_profile_idc: StdVideoH264ProfileIdc,
    ///[`picture_layout`] is a bitmask of
    ///[`VideoDecodeH264PictureLayoutFlagBitsEXT`] specifying the layout of
    ///the decoded picture’s contents depending on the nature (progressive vs.
    ///interlaced) of the input content.
    pub picture_layout: VideoDecodeH264PictureLayoutFlagsEXT,
}
impl<'lt> Default for VideoDecodeH264ProfileEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_DECODE_H264_PROFILE_EXT,
            p_next: std::ptr::null(),
            std_profile_idc: unsafe { std::mem::zeroed() },
            picture_layout: Default::default(),
        }
    }
}
impl<'lt> VideoDecodeH264ProfileEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::std_profile_idc`]
    pub fn std_profile_idc_raw(&self) -> &StdVideoH264ProfileIdc {
        &self.std_profile_idc
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::std_profile_idc`]
    pub fn set_std_profile_idc_raw(mut self, value: StdVideoH264ProfileIdc) -> Self {
        self.std_profile_idc = value;
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
    ///Gets the value of [`Self::std_profile_idc`]
    pub fn std_profile_idc(&self) -> StdVideoH264ProfileIdc {
        self.std_profile_idc
    }
    ///Gets the value of [`Self::picture_layout`]
    pub fn picture_layout(&self) -> VideoDecodeH264PictureLayoutFlagsEXT {
        self.picture_layout
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::std_profile_idc`]
    pub fn std_profile_idc_mut(&mut self) -> &mut StdVideoH264ProfileIdc {
        &mut self.std_profile_idc
    }
    ///Gets a mutable reference to the value of [`Self::picture_layout`]
    pub fn picture_layout_mut(&mut self) -> &mut VideoDecodeH264PictureLayoutFlagsEXT {
        &mut self.picture_layout
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::std_profile_idc`]
    pub fn set_std_profile_idc(mut self, value: crate::native::StdVideoH264ProfileIdc) -> Self {
        self.std_profile_idc = value;
        self
    }
    ///Sets the value of [`Self::picture_layout`]
    pub fn set_picture_layout(
        mut self,
        value: crate::extensions::ext_video_decode_h_264::VideoDecodeH264PictureLayoutFlagsEXT,
    ) -> Self {
        self.picture_layout = value;
        self
    }
}
///[VkVideoDecodeH264CapabilitiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH264CapabilitiesEXT.html) - Structure specifying H.264 decode capabilities
///# C Specifications
///The [`VideoDecodeH264CapabilitiesEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_video_decode_h264
///typedef struct VkVideoDecodeH264CapabilitiesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           maxLevel;
///    VkOffset2D         fieldOffsetGranularity;
///} VkVideoDecodeH264CapabilitiesEXT;
///```
///# Members
///When using [`get_physical_device_video_capabilities_khr`] to query the
///capabilities for the input `pVideoProfile` with
///`videoCodecOperation` specified as
///`VK_VIDEO_CODEC_OPERATION_DECODE_H264_BIT_EXT`, a
///[`VideoDecodeH264CapabilitiesEXT`] structure  **must**  be chained to
///[`VideoCapabilitiesKHR`] to get this H.264 decode profile specific
///capabilities.
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`max_level`] is the maximum H.264 level supported by the device.
/// - [`field_offset_granularity`] - if Interlaced Video Content is suported, the maximum field
///   offset granularity supported for the picture resource.
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_CAPABILITIES_EXT`
///# Related
/// - [`VK_EXT_video_decode_h264`]
/// - [`Offset2D`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoDecodeH264CapabilitiesEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoDecodeH264CapabilitiesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`max_level`] is the maximum H.264 level supported by the device.
    pub max_level: u32,
    ///[`field_offset_granularity`] - if Interlaced Video Content is suported,
    ///the maximum field offset granularity supported for the picture resource.
    pub field_offset_granularity: Offset2D,
}
impl<'lt> Default for VideoDecodeH264CapabilitiesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_DECODE_H264_CAPABILITIES_EXT,
            p_next: std::ptr::null_mut(),
            max_level: 0,
            field_offset_granularity: Default::default(),
        }
    }
}
impl<'lt> VideoDecodeH264CapabilitiesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::max_level`]
    pub fn max_level(&self) -> u32 {
        self.max_level
    }
    ///Gets the value of [`Self::field_offset_granularity`]
    pub fn field_offset_granularity(&self) -> Offset2D {
        self.field_offset_granularity
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
    ///Gets a mutable reference to the value of [`Self::max_level`]
    pub fn max_level_mut(&mut self) -> &mut u32 {
        &mut self.max_level
    }
    ///Gets a mutable reference to the value of [`Self::field_offset_granularity`]
    pub fn field_offset_granularity_mut(&mut self) -> &mut Offset2D {
        &mut self.field_offset_granularity
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::max_level`]
    pub fn set_max_level(mut self, value: u32) -> Self {
        self.max_level = value;
        self
    }
    ///Sets the value of [`Self::field_offset_granularity`]
    pub fn set_field_offset_granularity(mut self, value: crate::vulkan1_0::Offset2D) -> Self {
        self.field_offset_granularity = value;
        self
    }
}
///[VkVideoDecodeH264SessionParametersAddInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH264SessionParametersAddInfoEXT.html) - Structure specifies H.264 decoder parameter set information
///# C Specifications
///The [`VideoDecodeH264SessionParametersAddInfoEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_video_decode_h264
///typedef struct VkVideoDecodeH264SessionParametersAddInfoEXT {
///    VkStructureType                            sType;
///    const void*                                pNext;
///    uint32_t                                   spsStdCount;
///    const StdVideoH264SequenceParameterSet*    pSpsStd;
///    uint32_t                                   ppsStdCount;
///    const StdVideoH264PictureParameterSet*     pPpsStd;
///} VkVideoDecodeH264SessionParametersAddInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`sps_std_count`] is the number of SPS elements in [`sps_std`]. Its value  **must**  be less
///   than or equal to the value of `maxSpsStdCount`.
/// - [`sps_std`] is a pointer to an array of [`StdVideoH264SequenceParameterSet`] structures
///   representing H.264 sequence parameter sets. Each element of the array  **must**  have a unique
///   H.264 SPS ID.
/// - [`pps_std_count`] is the number of PPS provided in [`pps_std`]. Its value  **must**  be less
///   than or equal to the value of `maxPpsStdCount`.
/// - [`pps_std`] is a pointer to an array of [`StdVideoH264PictureParameterSet`] structures
///   representing H.264 picture parameter sets. Each element of the array  **must**  have a unique
///   H.264 SPS-PPS ID pair.
///# Description
///## Valid Usage
/// - The values of [`sps_std_count`] and [`pps_std_count`] **must**  be less than or equal to the
///   values of `maxSpsStdCount` and `maxPpsStdCount`, respectively
/// - When the `maxSpsStdCount` number of parameters of type StdVideoH264SequenceParameterSet in the
///   Video Session Parameters object is reached, no additional parameters of that type can be added
///   to this object. `VK_ERROR_TOO_MANY_OBJECTS` will be returned if an attempt is made to add
///   additional data to this object at this point
/// - When the `maxPpsStdCount` number of parameters of type StdVideoH264PictureParameterSet in the
///   Video Session Parameters object is reached, no additional parameters of that type can be added
///   to this object. `VK_ERROR_TOO_MANY_OBJECTS` will be returned if an attempt is made to add
///   additional data to this object at this point
/// - Each entry to be added  **must**  have a unique, to the rest of the parameter array entries
///   and the existing parameters in the Video Session Parameters Object that is being updated,
///   SPS-PPS IDs
/// - Parameter entries that already exist in Video Session Parameters object with a particular
///   SPS-PPS IDs  **cannot**  be replaced nor updated
/// - When creating a new object using a Video Session Parameters as a template, the array’s
///   parameters with the same SPS-PPS IDs as the ones from the template take precedence
/// - SPS/PPS parameters  **must**  comply with the limits specified in
///   [`VideoSessionCreateInfoKHR`] during Video Session creation
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_SESSION_PARAMETERS_ADD_INFO_EXT`
/// - If [`sps_std`] is not `NULL`, [`sps_std`] **must**  be a valid pointer to an array of
///   [`sps_std_count`][`StdVideoH264SequenceParameterSet`] values
/// - If [`pps_std`] is not `NULL`, [`pps_std`] **must**  be a valid pointer to an array of
///   [`pps_std_count`][`StdVideoH264PictureParameterSet`] values
/// - [`sps_std_count`] **must**  be greater than `0`
/// - [`pps_std_count`] **must**  be greater than `0`
///# Related
/// - [`VK_EXT_video_decode_h264`]
/// - [`StructureType`]
/// - [`VideoDecodeH264SessionParametersCreateInfoEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoDecodeH264SessionParametersAddInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoDecodeH264SessionParametersAddInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`sps_std_count`] is the number of SPS elements in [`sps_std`].
    ///Its value  **must**  be less than or equal to the value of
    ///`maxSpsStdCount`.
    pub sps_std_count: u32,
    ///[`sps_std`] is a pointer to an array of
    ///[`StdVideoH264SequenceParameterSet`] structures representing H.264
    ///sequence parameter sets.
    ///Each element of the array  **must**  have a unique H.264 SPS ID.
    pub sps_std: *const StdVideoH264SequenceParameterSet,
    ///[`pps_std_count`] is the number of PPS provided in [`pps_std`].
    ///Its value  **must**  be less than or equal to the value of
    ///`maxPpsStdCount`.
    pub pps_std_count: u32,
    ///[`pps_std`] is a pointer to an array of
    ///[`StdVideoH264PictureParameterSet`] structures representing H.264
    ///picture parameter sets.
    ///Each element of the array  **must**  have a unique H.264 SPS-PPS ID pair.
    pub pps_std: *const StdVideoH264PictureParameterSet,
}
impl<'lt> Default for VideoDecodeH264SessionParametersAddInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_DECODE_H264_SESSION_PARAMETERS_ADD_INFO_EXT,
            p_next: std::ptr::null(),
            sps_std_count: 0,
            sps_std: std::ptr::null(),
            pps_std_count: 0,
            pps_std: std::ptr::null(),
        }
    }
}
impl<'lt> VideoDecodeH264SessionParametersAddInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::sps_std_count`]
    pub fn sps_std_count(&self) -> u32 {
        self.sps_std_count
    }
    ///Gets the value of [`Self::sps_std`]
    pub fn sps_std(&self) -> *const StdVideoH264SequenceParameterSet {
        self.sps_std
    }
    ///Gets the value of [`Self::pps_std_count`]
    pub fn pps_std_count(&self) -> u32 {
        self.pps_std_count
    }
    ///Gets the value of [`Self::pps_std`]
    pub fn pps_std(&self) -> *const StdVideoH264PictureParameterSet {
        self.pps_std
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::sps_std_count`]
    pub fn sps_std_count_mut(&mut self) -> &mut u32 {
        &mut self.sps_std_count
    }
    ///Gets a mutable reference to the value of [`Self::pps_std_count`]
    pub fn pps_std_count_mut(&mut self) -> &mut u32 {
        &mut self.pps_std_count
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::sps_std_count`]
    pub fn set_sps_std_count(mut self, value: u32) -> Self {
        self.sps_std_count = value;
        self
    }
    ///Sets the value of [`Self::sps_std`]
    pub fn set_sps_std(mut self, value: *const crate::native::StdVideoH264SequenceParameterSet) -> Self {
        self.sps_std = value;
        self
    }
    ///Sets the value of [`Self::pps_std_count`]
    pub fn set_pps_std_count(mut self, value: u32) -> Self {
        self.pps_std_count = value;
        self
    }
    ///Sets the value of [`Self::pps_std`]
    pub fn set_pps_std(mut self, value: *const crate::native::StdVideoH264PictureParameterSet) -> Self {
        self.pps_std = value;
        self
    }
}
///[VkVideoDecodeH264SessionParametersCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH264SessionParametersCreateInfoEXT.html) - Structure specifies H.264 decoder parameter set information
///# C Specifications
///The [`VideoDecodeH264SessionParametersCreateInfoEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_video_decode_h264
///typedef struct VkVideoDecodeH264SessionParametersCreateInfoEXT {
///    VkStructureType                                        sType;
///    const void*                                            pNext;
///    uint32_t                                               maxSpsStdCount;
///    uint32_t                                               maxPpsStdCount;
///    const VkVideoDecodeH264SessionParametersAddInfoEXT*    pParametersAddInfo;
///} VkVideoDecodeH264SessionParametersCreateInfoEXT;
///```
///# Members
///A [`VideoDecodeH264SessionParametersCreateInfoEXT`] structure holding
///one H.264 SPS and at least one H.264 PPS paramater set  **must**  be chained to
///[`VideoSessionParametersCreateInfoKHR`] when calling
///[`create_video_session_parameters_khr`] to store these parameter set(s) with
///the decoder parameter set object for later reference.
///The provided H.264 SPS/PPS parameters  **must**  be within the limits specified
///during decoder creation for the decoder specified in
///[`VideoSessionParametersCreateInfoKHR`].
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`max_sps_std_count`] is the maximum number of SPS parameters that the
///   [`VideoSessionParametersKHR`] can contain.
/// - [`max_pps_std_count`] is the maximum number of PPS parameters that the
///   [`VideoSessionParametersKHR`] can contain.
/// - [`parameters_add_info`] is `NULL` or a pointer to a
///   [`VideoDecodeH264SessionParametersAddInfoEXT`] structure specifying H.264 parameters to add
///   upon object creation.
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_SESSION_PARAMETERS_CREATE_INFO_EXT`
/// - If [`parameters_add_info`] is not `NULL`, [`parameters_add_info`] **must**  be a valid pointer
///   to a valid [`VideoDecodeH264SessionParametersAddInfoEXT`] structure
///# Related
/// - [`VK_EXT_video_decode_h264`]
/// - [`StructureType`]
/// - [`VideoDecodeH264SessionParametersAddInfoEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoDecodeH264SessionParametersCreateInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoDecodeH264SessionParametersCreateInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`max_sps_std_count`] is the maximum number of SPS parameters that the
    ///[`VideoSessionParametersKHR`] can contain.
    pub max_sps_std_count: u32,
    ///[`max_pps_std_count`] is the maximum number of PPS parameters that the
    ///[`VideoSessionParametersKHR`] can contain.
    pub max_pps_std_count: u32,
    ///[`parameters_add_info`] is `NULL` or a pointer to a
    ///[`VideoDecodeH264SessionParametersAddInfoEXT`] structure specifying
    ///H.264 parameters to add upon object creation.
    pub parameters_add_info: *const VideoDecodeH264SessionParametersAddInfoEXT<'lt>,
}
impl<'lt> Default for VideoDecodeH264SessionParametersCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_DECODE_H264_SESSION_PARAMETERS_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            max_sps_std_count: 0,
            max_pps_std_count: 0,
            parameters_add_info: std::ptr::null(),
        }
    }
}
impl<'lt> VideoDecodeH264SessionParametersCreateInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::parameters_add_info`]
    pub fn parameters_add_info_raw(&self) -> *const VideoDecodeH264SessionParametersAddInfoEXT<'lt> {
        self.parameters_add_info
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::parameters_add_info`]
    pub fn set_parameters_add_info_raw(
        mut self,
        value: *const VideoDecodeH264SessionParametersAddInfoEXT<'lt>,
    ) -> Self {
        self.parameters_add_info = value;
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
    ///Gets the value of [`Self::max_sps_std_count`]
    pub fn max_sps_std_count(&self) -> u32 {
        self.max_sps_std_count
    }
    ///Gets the value of [`Self::max_pps_std_count`]
    pub fn max_pps_std_count(&self) -> u32 {
        self.max_pps_std_count
    }
    ///Gets the value of [`Self::parameters_add_info`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn parameters_add_info(&self) -> &VideoDecodeH264SessionParametersAddInfoEXT<'lt> {
        &*self.parameters_add_info
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::max_sps_std_count`]
    pub fn max_sps_std_count_mut(&mut self) -> &mut u32 {
        &mut self.max_sps_std_count
    }
    ///Gets a mutable reference to the value of [`Self::max_pps_std_count`]
    pub fn max_pps_std_count_mut(&mut self) -> &mut u32 {
        &mut self.max_pps_std_count
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::max_sps_std_count`]
    pub fn set_max_sps_std_count(mut self, value: u32) -> Self {
        self.max_sps_std_count = value;
        self
    }
    ///Sets the value of [`Self::max_pps_std_count`]
    pub fn set_max_pps_std_count(mut self, value: u32) -> Self {
        self.max_pps_std_count = value;
        self
    }
    ///Sets the value of [`Self::parameters_add_info`]
    pub fn set_parameters_add_info(
        mut self,
        value: &'lt crate::extensions::ext_video_decode_h_264::VideoDecodeH264SessionParametersAddInfoEXT<'lt>,
    ) -> Self {
        self.parameters_add_info = value as *const _;
        self
    }
}
///[VkVideoDecodeH264PictureInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH264PictureInfoEXT.html) - Structure specifies H.264 decode picture parameters when decoding a picture
///# C Specifications
///The [`VideoDecodeH264PictureInfoEXT`] structure represents a picture
///decode operation and is defined as:
///```c
///// Provided by VK_EXT_video_decode_h264
///typedef struct VkVideoDecodeH264PictureInfoEXT {
///    VkStructureType                         sType;
///    const void*                             pNext;
///    const StdVideoDecodeH264PictureInfo*    pStdPictureInfo;
///    uint32_t                                slicesCount;
///    const uint32_t*                         pSlicesDataOffsets;
///} VkVideoDecodeH264PictureInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`std_picture_info`] is a pointer to a [`StdVideoDecodeH264PictureInfo`] structure specifying
///   the codec standard specific picture information from the H.264 specification.
/// - [`slices_count`] is the number of slices in this picture.
/// - [`slices_data_offsets`] is a pointer to an array of [`slices_count`] offsets indicating the
///   start offset of each slice within the bitstream buffer.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_PICTURE_INFO_EXT`
/// - [`std_picture_info`] **must**  be a valid pointer to a valid [`StdVideoDecodeH264PictureInfo`]
///   value
/// - [`slices_data_offsets`] **must**  be a valid pointer to an array of [`slices_count`]`uint32_t`
///   values
/// - [`slices_count`] **must**  be greater than `0`
///# Related
/// - [`VK_EXT_video_decode_h264`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoDecodeH264PictureInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoDecodeH264PictureInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`std_picture_info`] is a pointer to a
    ///[`StdVideoDecodeH264PictureInfo`] structure specifying the codec
    ///standard specific picture information from the H.264 specification.
    pub std_picture_info: *const StdVideoDecodeH264PictureInfo,
    ///[`slices_count`] is the number of slices in this picture.
    pub slices_count: u32,
    ///[`slices_data_offsets`] is a pointer to an array of [`slices_count`]
    ///offsets indicating the start offset of each slice within the bitstream
    ///buffer.
    pub slices_data_offsets: *const u32,
}
impl<'lt> Default for VideoDecodeH264PictureInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_DECODE_H264_PICTURE_INFO_EXT,
            p_next: std::ptr::null(),
            std_picture_info: std::ptr::null(),
            slices_count: 0,
            slices_data_offsets: std::ptr::null(),
        }
    }
}
impl<'lt> VideoDecodeH264PictureInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::slices_data_offsets`]
    pub fn slices_data_offsets_raw(&self) -> *const u32 {
        self.slices_data_offsets
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::slices_data_offsets`]
    pub fn set_slices_data_offsets_raw(mut self, value: *const u32) -> Self {
        self.slices_data_offsets = value;
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
    ///Gets the value of [`Self::std_picture_info`]
    pub fn std_picture_info(&self) -> *const StdVideoDecodeH264PictureInfo {
        self.std_picture_info
    }
    ///Gets the value of [`Self::slices_count`]
    pub fn slices_count(&self) -> u32 {
        self.slices_count
    }
    ///Gets the value of [`Self::slices_data_offsets`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn slices_data_offsets(&self) -> &[u32] {
        std::slice::from_raw_parts(self.slices_data_offsets, self.slices_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::slices_count`]
    pub fn slices_count_mut(&mut self) -> &mut u32 {
        &mut self.slices_count
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::std_picture_info`]
    pub fn set_std_picture_info(mut self, value: *const crate::native::StdVideoDecodeH264PictureInfo) -> Self {
        self.std_picture_info = value;
        self
    }
    ///Sets the value of [`Self::slices_count`]
    pub fn set_slices_count(mut self, value: u32) -> Self {
        self.slices_count = value;
        self
    }
    ///Sets the value of [`Self::slices_data_offsets`]
    pub fn set_slices_data_offsets(mut self, value: &'lt [u32]) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.slices_data_offsets = value.as_ptr();
        self.slices_count = len_;
        self
    }
}
///[VkVideoDecodeH264DpbSlotInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH264DpbSlotInfoEXT.html) - Structure specifies H.264 decode DPB picture information
///# C Specifications
///The [`VideoDecodeH264DpbSlotInfoEXT`] structure correlates a DPB Slot
///index with codec-specific information and is defined as:
///```c
///// Provided by VK_EXT_video_decode_h264
///typedef struct VkVideoDecodeH264DpbSlotInfoEXT {
///    VkStructureType                           sType;
///    const void*                               pNext;
///    const StdVideoDecodeH264ReferenceInfo*    pStdReferenceInfo;
///} VkVideoDecodeH264DpbSlotInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`std_reference_info`] is a pointer to a [`StdVideoDecodeH264ReferenceInfo`] structure
///   specifying the codec standard specific picture reference information from the H.264
///   specification.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_DPB_SLOT_INFO_EXT`
/// - [`std_reference_info`] **must**  be a valid pointer to a valid
///   [`StdVideoDecodeH264ReferenceInfo`] value
///# Related
/// - [`VK_EXT_video_decode_h264`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoDecodeH264DpbSlotInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoDecodeH264DpbSlotInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///No documentation found
    pub p_next: *const BaseInStructure<'lt>,
    ///[`std_reference_info`] is a pointer to a
    ///[`StdVideoDecodeH264ReferenceInfo`] structure specifying the codec
    ///standard specific picture reference information from the H.264
    ///specification.
    pub std_reference_info: *const StdVideoDecodeH264ReferenceInfo,
}
impl<'lt> Default for VideoDecodeH264DpbSlotInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_DECODE_H264_DPB_SLOT_INFO_EXT,
            p_next: std::ptr::null(),
            std_reference_info: std::ptr::null(),
        }
    }
}
impl<'lt> VideoDecodeH264DpbSlotInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::std_reference_info`]
    pub fn std_reference_info(&self) -> *const StdVideoDecodeH264ReferenceInfo {
        self.std_reference_info
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::std_reference_info`]
    pub fn set_std_reference_info(mut self, value: *const crate::native::StdVideoDecodeH264ReferenceInfo) -> Self {
        self.std_reference_info = value;
        self
    }
}
///[VkVideoDecodeH264MvcEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH264MvcEXT.html) - Structure specifies parameters of mvc views
///# C Specifications
///The [`VideoDecodeH264MvcEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_video_decode_h264
///typedef struct VkVideoDecodeH264MvcEXT {
///    VkStructureType                 sType;
///    const void*                     pNext;
///    const StdVideoDecodeH264Mvc*    pStdMvc;
///} VkVideoDecodeH264MvcEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`std_mvc`] is a pointer to a [`StdVideoDecodeH264Mvc`] structure specifying H.264 codec
///   specification information for MVC.
///# Description
///When the content type is H.264 MVC, a [`VideoDecodeH264MvcEXT`]
///structure  **must**  be chained to [`VideoDecodeH264PictureInfoEXT`].
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_MVC_EXT`
/// - [`std_mvc`] **must**  be a valid pointer to a valid [`StdVideoDecodeH264Mvc`] value
///# Related
/// - [`VK_EXT_video_decode_h264`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoDecodeH264MvcEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoDecodeH264MvcEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`std_mvc`] is a pointer to a [`StdVideoDecodeH264Mvc`] structure
    ///specifying H.264 codec specification information for MVC.
    pub std_mvc: *const StdVideoDecodeH264Mvc,
}
impl<'lt> Default for VideoDecodeH264MvcEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_DECODE_H264_MVC_EXT,
            p_next: std::ptr::null(),
            std_mvc: std::ptr::null(),
        }
    }
}
impl<'lt> VideoDecodeH264MvcEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::std_mvc`]
    pub fn std_mvc(&self) -> *const StdVideoDecodeH264Mvc {
        self.std_mvc
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::std_mvc`]
    pub fn set_std_mvc(mut self, value: *const crate::native::StdVideoDecodeH264Mvc) -> Self {
        self.std_mvc = value;
        self
    }
}
