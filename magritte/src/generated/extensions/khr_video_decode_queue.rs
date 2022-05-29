//![VK_KHR_video_decode_queue](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_video_decode_queue.html) - device extension
//!# Revision
//!3
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_video_queue`]`
//! - Requires `[`khr_synchronization2`]`
//! - **This is a *provisional* extension and  **must**  be used with caution. See the [description](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#boilerplate-provisional-header)
//!   of provisional header files for enablement and stability details.**
//!# Contacts
//! - [jake.beju@amd.com]()
//!# New functions & commands
//! - [`cmd_decode_video_khr`]
//!# New structures
//! - [`VideoDecodeInfoKHR`]
//! - Extending [`VideoCapabilitiesKHR`]:  - [`VideoDecodeCapabilitiesKHR`]
//!# New enums
//! - [`VideoDecodeCapabilityFlagBitsKHR`]
//! - [`VideoDecodeFlagBitsKHR`]
//!# New bitmasks
//! - [`VideoDecodeCapabilityFlagsKHR`]
//! - [`VideoDecodeFlagsKHR`]
//!# New constants
//! - [`KHR_VIDEO_DECODE_QUEUE_EXTENSION_NAME`]
//! - [`KHR_VIDEO_DECODE_QUEUE_SPEC_VERSION`]
//! - Extending [`AccessFlagBits2`]:  - `VK_ACCESS_2_VIDEO_DECODE_READ_BIT_KHR`  -
//!   `VK_ACCESS_2_VIDEO_DECODE_WRITE_BIT_KHR`
//! - Extending [`BufferUsageFlagBits`]:  - `VK_BUFFER_USAGE_VIDEO_DECODE_DST_BIT_KHR`  -
//!   `VK_BUFFER_USAGE_VIDEO_DECODE_SRC_BIT_KHR`
//! - Extending [`FormatFeatureFlagBits`]:  - `VK_FORMAT_FEATURE_VIDEO_DECODE_DPB_BIT_KHR`  -
//!   `VK_FORMAT_FEATURE_VIDEO_DECODE_OUTPUT_BIT_KHR`
//! - Extending [`ImageLayout`]:  - `VK_IMAGE_LAYOUT_VIDEO_DECODE_DPB_KHR`  -
//!   `VK_IMAGE_LAYOUT_VIDEO_DECODE_DST_KHR`  - `VK_IMAGE_LAYOUT_VIDEO_DECODE_SRC_KHR`
//! - Extending [`ImageUsageFlagBits`]:  - `VK_IMAGE_USAGE_VIDEO_DECODE_DPB_BIT_KHR`  -
//!   `VK_IMAGE_USAGE_VIDEO_DECODE_DST_BIT_KHR`  - `VK_IMAGE_USAGE_VIDEO_DECODE_SRC_BIT_KHR`
//! - Extending [`PipelineStageFlagBits2`]:  - `VK_PIPELINE_STAGE_2_VIDEO_DECODE_BIT_KHR`
//! - Extending [`QueueFlagBits`]:  - `VK_QUEUE_VIDEO_DECODE_BIT_KHR`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_VIDEO_DECODE_CAPABILITIES_KHR`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_DECODE_INFO_KHR`
//!If [`khr_format_feature_flags2`] is supported:
//! - Extending [`FormatFeatureFlagBits2`]:  - `VK_FORMAT_FEATURE_2_VIDEO_DECODE_DPB_BIT_KHR`  -
//!   `VK_FORMAT_FEATURE_2_VIDEO_DECODE_OUTPUT_BIT_KHR`
//!# Version History
//! - Revision 1, 2018-6-11 (Peter Fang)  - Initial draft
//! - Revision 1.5, Nov 09 2018 (Tony Zlatinski)  - API Updates
//! - Revision 1.6, Jan 08 2020 (Tony Zlatinski)  - API unify with the video_encode_queue spec
//! - Revision 1.7, March 29 2021 (Tony Zlatinski)  - Spec and API updates.
//! - Revision 2, September 30 2021 (Jon Leech)  - Add interaction with
//!   `[`khr_format_feature_flags2`]` to `vk.xml`
//! - Revision 3, 2022-02-25 (Ahmed Abdelkhalek)  - Add VkVideoDecodeCapabilitiesKHR with new flags
//!   to report support for decode DPB and output coinciding in the same image, or in distinct
//!   images.
//!# Other info
//! * 2022-02-25
//! * No known IP claims.
//! * - Ahmed Abdelkhalek, AMD  - Jake Beju, AMD  - Olivier Lapicque, NVIDIA  - Peter Fang, AMD  -
//!   Piers Daniell, NVIDIA  - Srinath Kumarapuram, NVIDIA  - Tony Zlatinski, NVIDIA
//!# Related
//! - [`VideoDecodeCapabilitiesKHR`]
//! - [`VideoDecodeCapabilityFlagBitsKHR`]
//! - [`VideoDecodeCapabilityFlagsKHR`]
//! - [`VideoDecodeFlagBitsKHR`]
//! - [`VideoDecodeFlagsKHR`]
//! - [`VideoDecodeInfoKHR`]
//! - [`cmd_decode_video_khr`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
#[cfg(feature = "VK_EXT_video_decode_h264")]
pub use crate::extensions::ext_video_decode_h264::VideoDecodeH264CapabilitiesEXT;
#[cfg(feature = "VK_EXT_video_decode_h264")]
pub use crate::extensions::ext_video_decode_h264::VideoDecodeH264PictureInfoEXT;
#[cfg(feature = "VK_EXT_video_decode_h265")]
pub use crate::extensions::ext_video_decode_h265::VideoDecodeH265CapabilitiesEXT;
#[cfg(feature = "VK_EXT_video_decode_h265")]
pub use crate::extensions::ext_video_decode_h265::VideoDecodeH265PictureInfoEXT;
use crate::{
    extensions::khr_video_queue::{VideoPictureResourceKHR, VideoReferenceSlotKHR},
    vulkan1_0::{
        BaseInStructure, BaseOutStructure, Buffer, CommandBuffer, Device, DeviceSize, Extent2D, Offset2D, StructureType,
    },
    AsRaw, Unique,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    ffi::CStr,
    iter::{Extend, FromIterator, IntoIterator},
    marker::PhantomData,
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_VIDEO_DECODE_QUEUE_SPEC_VERSION")]
pub const KHR_VIDEO_DECODE_QUEUE_SPEC_VERSION: u32 = 3;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_VIDEO_DECODE_QUEUE_EXTENSION_NAME")]
pub const KHR_VIDEO_DECODE_QUEUE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_video_decode_queue");
///[vkCmdDecodeVideoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDecodeVideoKHR.html) - Decode a frame
///# C Specifications
///To decode a frame, call:
///```c
///// Provided by VK_KHR_video_decode_queue
///void vkCmdDecodeVideoKHR(
///    VkCommandBuffer                             commandBuffer,
///    const VkVideoDecodeInfoKHR*                 pFrameInfo);
///```
/// # Parameters
/// - [`command_buffer`] is the command buffer to be filled with this function for decode frame
///   command.
/// - [`p_frame_info`] is a pointer to a [`VideoDecodeInfoKHR`] structure.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`p_frame_info`] **must**  be a valid pointer to a valid [`VideoDecodeInfoKHR`] structure
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support decode
///   operations
/// - This command  **must**  only be called outside of a render pass instance
/// - [`command_buffer`] **must**  be a primary [`CommandBuffer`]
///
/// ## Host Synchronization
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
/// ## Command Properties
/// # Related
/// - [`khr_video_decode_queue`]
/// - [`CommandBuffer`]
/// - [`VideoDecodeInfoKHR`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdDecodeVideoKHR")]
pub type FNCmdDecodeVideoKhr = Option<
    for<'lt> unsafe extern "system" fn(command_buffer: CommandBuffer, p_frame_info: *const VideoDecodeInfoKHR<'lt>),
>;
///[VkVideoDecodeCapabilityFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeCapabilityFlagBitsKHR.html) - Video decode capability flags
///# C Specifications
///Bits which  **may**  be set in [`VideoDecodeCapabilitiesKHR::flags`],
///indicating the decoding features supported, are:
///```c
///// Provided by VK_KHR_video_decode_queue
///typedef enum VkVideoDecodeCapabilityFlagBitsKHR {
///    VK_VIDEO_DECODE_CAPABILITY_DEFAULT_KHR = 0,
///    VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_COINCIDE_BIT_KHR = 0x00000001,
///    VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_DISTINCT_BIT_KHR = 0x00000002,
///} VkVideoDecodeCapabilityFlagBitsKHR;
///```
/// # Description
/// - [`DPB_AND_OUTPUT_COINCIDE`] - reports the implementation supports using the same [Video Picture Resource](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-picture-resources) for decode DPB and decode output.
/// - [`DPB_AND_OUTPUT_DISTINCT`] - reports the implementation supports using distinct [Video Picture Resources](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-picture-resources) for decode DPB and decode output.
/// An implementation  **must**  report at least one of
/// [`DPB_AND_OUTPUT_COINCIDE`] or
/// [`DPB_AND_OUTPUT_DISTINCT`] as
/// supported.
/// # Related
/// - [`khr_video_decode_queue`]
/// - [`VideoDecodeCapabilityFlagsKHR`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoDecodeCapabilityFlagBitsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoDecodeCapabilityFlagBitsKHR(u32);
impl const Default for VideoDecodeCapabilityFlagBitsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl VideoDecodeCapabilityFlagBitsKHR {
    ///No documentation found
    pub const DEFAULT: Self = Self(0);
    ///[`DPB_AND_OUTPUT_COINCIDE`] -
    ///reports the implementation supports using the same
    ///[Video Picture Resource](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-picture-resources) for decode DPB and
    ///decode output.
    pub const DPB_AND_OUTPUT_COINCIDE: Self = Self(1);
    ///[`DPB_AND_OUTPUT_DISTINCT`] -
    ///reports the implementation supports using distinct
    ///[Video Picture Resources](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-picture-resources) for decode DPB and
    ///decode output.
    pub const DPB_AND_OUTPUT_DISTINCT: Self = Self(2);
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
impl std::fmt::Debug for VideoDecodeCapabilityFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoDecodeCapabilityFlagBitsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoDecodeCapabilityFlagBitsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    match self.0 {
                        VideoDecodeCapabilityFlagBitsKHR::DEFAULT => f.write_str("DEFAULT")?,
                        VideoDecodeCapabilityFlagBitsKHR::DPB_AND_OUTPUT_COINCIDE => {
                            f.write_str("DPB_AND_OUTPUT_COINCIDE")?
                        },
                        VideoDecodeCapabilityFlagBitsKHR::DPB_AND_OUTPUT_DISTINCT => {
                            f.write_str("DPB_AND_OUTPUT_DISTINCT")?
                        },
                        _ => f.write_str("invalid")?,
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoDecodeCapabilityFlagBitsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoDecodeFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeFlagBitsKHR.html) - Video Decode Command Flags
///# C Specifications
///The [`cmd_decode_video_khr`] flags are defined with the following
///enumeration:
///```c
///// Provided by VK_KHR_video_decode_queue
///typedef enum VkVideoDecodeFlagBitsKHR {
///    VK_VIDEO_DECODE_DEFAULT_KHR = 0,
///    VK_VIDEO_DECODE_RESERVED_0_BIT_KHR = 0x00000001,
///} VkVideoDecodeFlagBitsKHR;
///```
/// # Description
/// - [`RESERVED0`] The current version of the specification has reserved this value for future use.
/// # Related
/// - [`khr_video_decode_queue`]
/// - [`VideoDecodeFlagsKHR`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoDecodeFlagBitsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoDecodeFlagBitsKHR(u32);
impl const Default for VideoDecodeFlagBitsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl VideoDecodeFlagBitsKHR {
    ///No documentation found
    pub const DEFAULT: Self = Self(0);
    ///[`RESERVED0`] The current version of the
    ///specification has reserved this value for future use.
    pub const RESERVED0: Self = Self(1);
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
impl std::fmt::Debug for VideoDecodeFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoDecodeFlagBitsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoDecodeFlagBitsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    match self.0 {
                        VideoDecodeFlagBitsKHR::DEFAULT => f.write_str("DEFAULT")?,
                        VideoDecodeFlagBitsKHR::RESERVED0 => f.write_str("RESERVED0")?,
                        _ => f.write_str("invalid")?,
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoDecodeFlagBitsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoDecodeCapabilityFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeCapabilityFlagBitsKHR.html) - Video decode capability flags
///# C Specifications
///Bits which  **may**  be set in [`VideoDecodeCapabilitiesKHR::flags`],
///indicating the decoding features supported, are:
///```c
///// Provided by VK_KHR_video_decode_queue
///typedef enum VkVideoDecodeCapabilityFlagBitsKHR {
///    VK_VIDEO_DECODE_CAPABILITY_DEFAULT_KHR = 0,
///    VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_COINCIDE_BIT_KHR = 0x00000001,
///    VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_DISTINCT_BIT_KHR = 0x00000002,
///} VkVideoDecodeCapabilityFlagBitsKHR;
///```
/// # Description
/// - [`DPB_AND_OUTPUT_COINCIDE`] - reports the implementation supports using the same [Video Picture Resource](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-picture-resources) for decode DPB and decode output.
/// - [`DPB_AND_OUTPUT_DISTINCT`] - reports the implementation supports using distinct [Video Picture Resources](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-picture-resources) for decode DPB and decode output.
/// An implementation  **must**  report at least one of
/// [`DPB_AND_OUTPUT_COINCIDE`] or
/// [`DPB_AND_OUTPUT_DISTINCT`] as
/// supported.
/// # Related
/// - [`khr_video_decode_queue`]
/// - [`VideoDecodeCapabilityFlagsKHR`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoDecodeCapabilityFlagsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoDecodeCapabilityFlagsKHR(u32);
impl const Default for VideoDecodeCapabilityFlagsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl From<VideoDecodeCapabilityFlagBitsKHR> for VideoDecodeCapabilityFlagsKHR {
    fn from(from: VideoDecodeCapabilityFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(from.bits()) }
    }
}
impl VideoDecodeCapabilityFlagsKHR {
    ///No documentation found
    pub const DEFAULT: Self = Self(0);
    ///[`DPB_AND_OUTPUT_COINCIDE`] -
    ///reports the implementation supports using the same
    ///[Video Picture Resource](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-picture-resources) for decode DPB and
    ///decode output.
    pub const DPB_AND_OUTPUT_COINCIDE: Self = Self(1);
    ///[`DPB_AND_OUTPUT_DISTINCT`] -
    ///reports the implementation supports using distinct
    ///[Video Picture Resources](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-picture-resources) for decode DPB and
    ///decode output.
    pub const DPB_AND_OUTPUT_DISTINCT: Self = Self(2);
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
            all |= Self::DEFAULT;
        }
        {
            all |= Self::DPB_AND_OUTPUT_COINCIDE;
        }
        {
            all |= Self::DPB_AND_OUTPUT_DISTINCT;
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
impl const std::ops::BitOr for VideoDecodeCapabilityFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for VideoDecodeCapabilityFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for VideoDecodeCapabilityFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for VideoDecodeCapabilityFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for VideoDecodeCapabilityFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for VideoDecodeCapabilityFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for VideoDecodeCapabilityFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for VideoDecodeCapabilityFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for VideoDecodeCapabilityFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<VideoDecodeCapabilityFlagsKHR> for VideoDecodeCapabilityFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoDecodeCapabilityFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<VideoDecodeCapabilityFlagBitsKHR> for VideoDecodeCapabilityFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoDecodeCapabilityFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<VideoDecodeCapabilityFlagBitsKHR>>::from(i));
        }
    }
}
impl FromIterator<VideoDecodeCapabilityFlagsKHR> for VideoDecodeCapabilityFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoDecodeCapabilityFlagsKHR>>(iterator: T) -> VideoDecodeCapabilityFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoDecodeCapabilityFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<VideoDecodeCapabilityFlagBitsKHR> for VideoDecodeCapabilityFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoDecodeCapabilityFlagBitsKHR>>(
        iterator: T,
    ) -> VideoDecodeCapabilityFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoDecodeCapabilityFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for VideoDecodeCapabilityFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoDecodeCapabilityFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoDecodeCapabilityFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(VideoDecodeCapabilityFlagsKHR::DEFAULT) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(DEFAULT))?;
                    }
                    if self.0.contains(VideoDecodeCapabilityFlagsKHR::DPB_AND_OUTPUT_COINCIDE) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(DPB_AND_OUTPUT_COINCIDE))?;
                    }
                    if self.0.contains(VideoDecodeCapabilityFlagsKHR::DPB_AND_OUTPUT_DISTINCT) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(DPB_AND_OUTPUT_DISTINCT))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoDecodeCapabilityFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoDecodeFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeFlagBitsKHR.html) - Video Decode Command Flags
///# C Specifications
///The [`cmd_decode_video_khr`] flags are defined with the following
///enumeration:
///```c
///// Provided by VK_KHR_video_decode_queue
///typedef enum VkVideoDecodeFlagBitsKHR {
///    VK_VIDEO_DECODE_DEFAULT_KHR = 0,
///    VK_VIDEO_DECODE_RESERVED_0_BIT_KHR = 0x00000001,
///} VkVideoDecodeFlagBitsKHR;
///```
/// # Description
/// - [`RESERVED0`] The current version of the specification has reserved this value for future use.
/// # Related
/// - [`khr_video_decode_queue`]
/// - [`VideoDecodeFlagsKHR`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoDecodeFlagsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoDecodeFlagsKHR(u32);
impl const Default for VideoDecodeFlagsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl From<VideoDecodeFlagBitsKHR> for VideoDecodeFlagsKHR {
    fn from(from: VideoDecodeFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(from.bits()) }
    }
}
impl VideoDecodeFlagsKHR {
    ///No documentation found
    pub const DEFAULT: Self = Self(0);
    ///[`RESERVED0`] The current version of the
    ///specification has reserved this value for future use.
    pub const RESERVED0: Self = Self(1);
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
            all |= Self::DEFAULT;
        }
        {
            all |= Self::RESERVED0;
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
impl const std::ops::BitOr for VideoDecodeFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for VideoDecodeFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for VideoDecodeFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for VideoDecodeFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for VideoDecodeFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for VideoDecodeFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for VideoDecodeFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for VideoDecodeFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for VideoDecodeFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<VideoDecodeFlagsKHR> for VideoDecodeFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoDecodeFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<VideoDecodeFlagBitsKHR> for VideoDecodeFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoDecodeFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<VideoDecodeFlagBitsKHR>>::from(i));
        }
    }
}
impl FromIterator<VideoDecodeFlagsKHR> for VideoDecodeFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoDecodeFlagsKHR>>(iterator: T) -> VideoDecodeFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoDecodeFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<VideoDecodeFlagBitsKHR> for VideoDecodeFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoDecodeFlagBitsKHR>>(iterator: T) -> VideoDecodeFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoDecodeFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for VideoDecodeFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoDecodeFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoDecodeFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(VideoDecodeFlagsKHR::DEFAULT) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(DEFAULT))?;
                    }
                    if self.0.contains(VideoDecodeFlagsKHR::RESERVED0) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(RESERVED0))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoDecodeFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoDecodeCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeCapabilitiesKHR.html) - Structure specifying decode capabilities
///# C Specifications
///When calling [`get_physical_device_video_capabilities_khr`] with
///`pVideoProfile->videoCodecOperation` specified as one of the decode
///operation bits, the [`VideoDecodeCapabilitiesKHR`] structure  **must**  be
///included in the [`p_next`] chain of the [`VideoCapabilitiesKHR`]
///structure to retrieve capabilities specific to video decoding.The [`VideoDecodeCapabilitiesKHR`]
/// structure is defined as:
///```c
///// Provided by VK_KHR_video_decode_queue
///typedef struct VkVideoDecodeCapabilitiesKHR {
///    VkStructureType                    sType;
///    void*                              pNext;
///    VkVideoDecodeCapabilityFlagsKHR    flags;
///} VkVideoDecodeCapabilitiesKHR;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is a bitmask of [`VideoDecodeCapabilityFlagBitsKHR`] describing supported decoding
///   features.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_DECODE_CAPABILITIES_KHR`
/// # Related
/// - [`khr_video_decode_queue`]
/// - [`StructureType`]
/// - [`VideoDecodeCapabilityFlagsKHR`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoDecodeCapabilitiesKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct VideoDecodeCapabilitiesKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`flags`] is a bitmask of [`VideoDecodeCapabilityFlagBitsKHR`]
    ///describing supported decoding features.
    pub flags: VideoDecodeCapabilityFlagsKHR,
}
impl<'lt> Default for VideoDecodeCapabilitiesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_DECODE_CAPABILITIES_KHR,
            p_next: std::ptr::null_mut(),
            flags: Default::default(),
        }
    }
}
impl<'lt> VideoDecodeCapabilitiesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> VideoDecodeCapabilityFlagsKHR {
        self.flags
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
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut VideoDecodeCapabilityFlagsKHR {
        &mut self.flags
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn set_flags(
        &mut self,
        value: crate::extensions::khr_video_decode_queue::VideoDecodeCapabilityFlagsKHR,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn with_flags(
        mut self,
        value: crate::extensions::khr_video_decode_queue::VideoDecodeCapabilityFlagsKHR,
    ) -> Self {
        self.flags = value;
        self
    }
}
#[cfg(feature = "VK_EXT_video_decode_h264")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, VideoDecodeH264CapabilitiesEXT<'extender>> for VideoDecodeCapabilitiesKHR<'this>
{
    type Out = VideoDecodeCapabilitiesKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut VideoDecodeH264CapabilitiesEXT<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut VideoDecodeH264CapabilitiesEXT<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
}
#[cfg(feature = "VK_EXT_video_decode_h265")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, VideoDecodeH265CapabilitiesEXT<'extender>> for VideoDecodeCapabilitiesKHR<'this>
{
    type Out = VideoDecodeCapabilitiesKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut VideoDecodeH265CapabilitiesEXT<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut VideoDecodeH265CapabilitiesEXT<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
}
///[VkVideoDecodeInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeInfoKHR.html) - Structure specifying parameters of decoding a frame
///# C Specifications
///The [`VideoDecodeInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_video_decode_queue
///typedef struct VkVideoDecodeInfoKHR {
///    VkStructureType                   sType;
///    const void*                       pNext;
///    VkVideoDecodeFlagsKHR             flags;
///    VkOffset2D                        codedOffset;
///    VkExtent2D                        codedExtent;
///    VkBuffer                          srcBuffer;
///    VkDeviceSize                      srcBufferOffset;
///    VkDeviceSize                      srcBufferRange;
///    VkVideoPictureResourceKHR         dstPictureResource;
///    const VkVideoReferenceSlotKHR*    pSetupReferenceSlot;
///    uint32_t                          referenceSlotCount;
///    const VkVideoReferenceSlotKHR*    pReferenceSlots;
///} VkVideoDecodeInfoKHR;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure. All the codec
///   specific structures related to each frame(picture parameters, quantization matrix, etc.)
///   **must**  be chained here and pass to decode session with the function call
///   [`cmd_decode_video_khr`].
/// - [`flags`] is a bitmask of [`VideoDecodeFlagBitsKHR`] specifying decode flags, reserved for
///   future versions of this specification.
/// - [`coded_offset`] is the coded offset of the decode operations. The purpose of this field is
///   interpreted based on the codec extension. When decoding content in H.264 field mode, the
///   [`coded_offset`] specifies the line or picture field’s offset within the image.
/// - [`coded_extent`] is the coded size of the decode operations.
/// - [`src_buffer`] is the source buffer that holds the encoded bitstream.
/// - [`src_buffer_offset`] is the buffer offset where the valid encoded bitstream starts in
///   srcBuffer. It  **must**  meet the alignment requirement `minBitstreamBufferOffsetAlignment`
///   within [`VideoCapabilitiesKHR`] queried with the
///   [`get_physical_device_video_capabilities_khr`] function.
/// - [`src_buffer_range`] is the size of the srcBuffer with valid encoded bitstream, starting from
///   [`src_buffer_offset`]. It  **must**  meet the alignment requirement
///   `minBitstreamBufferSizeAlignment` within [`VideoCapabilitiesKHR`] queried with the
///   [`get_physical_device_video_capabilities_khr`] function.
/// - [`dst_picture_resource`] is the destination [Decoded Output Picture](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#decoded-output-picture)
///   Resource.
/// - [`setup_reference_slot`] is `NULL` or a pointer to a [`VideoReferenceSlotKHR`] structure used
///   for generating a DPB reference slot and Picture Resource. `pSetupReferenceSlot->slotIndex`
///   specifies the slot index number to use as a target for producing the DPB data. `slotIndex`
///   **must**  reference a valid entry as specified in [`VideoBeginCodingInfoKHR`] via the
///   [`reference_slots`] within the [`cmd_begin_video_coding_khr`] command that established the
///   Vulkan Video Decode Context for this command.
/// - [`reference_slot_count`] is the number of the DPB Reference Pictures that will be used when
///   this decoding operation is executing.
/// - [`reference_slots`] is a pointer to an array of [`VideoReferenceSlotKHR`] structures
///   specifying the DPB Reference pictures that will be used when this decoding operation is
///   executing.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_DECODE_INFO_KHR`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**
///   be either `NULL` or a pointer to a valid instance of [`VideoDecodeH264PictureInfoEXT`] or
///   [`VideoDecodeH265PictureInfoEXT`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
/// - [`flags`] **must**  be a valid combination of [`VideoDecodeFlagBitsKHR`] values
/// - [`src_buffer`] **must**  be a valid [`Buffer`] handle
/// - [`dst_picture_resource`] **must**  be a valid [`VideoPictureResourceKHR`] structure
/// - [`setup_reference_slot`] **must**  be a valid pointer to a valid [`VideoReferenceSlotKHR`]
///   structure
/// - If [`reference_slot_count`] is not `0`, [`reference_slots`] **must**  be a valid pointer to an
///   array of [`reference_slot_count`] valid [`VideoReferenceSlotKHR`] structures
/// # Related
/// - [`khr_video_decode_queue`]
/// - [`Buffer`]
/// - [`DeviceSize`]
/// - [`Extent2D`]
/// - [`Offset2D`]
/// - [`StructureType`]
/// - [`VideoDecodeFlagsKHR`]
/// - [`VideoPictureResourceKHR`]
/// - [`VideoReferenceSlotKHR`]
/// - [`cmd_decode_video_khr`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoDecodeInfoKHR")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct VideoDecodeInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    ///All the codec specific structures related to each frame(picture
    ///parameters, quantization matrix, etc.)  **must**  be chained here and pass to
    ///decode session with the function call [`cmd_decode_video_khr`].
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`VideoDecodeFlagBitsKHR`] specifying
    ///decode flags, reserved for future versions of this specification.
    pub flags: VideoDecodeFlagsKHR,
    ///[`coded_offset`] is the coded offset of the decode operations.
    ///The purpose of this field is interpreted based on the codec extension.
    ///When decoding content in H.264 field mode, the [`coded_offset`]
    ///specifies the line or picture field’s offset within the image.
    pub coded_offset: Offset2D,
    ///[`coded_extent`] is the coded size of the decode operations.
    pub coded_extent: Extent2D,
    ///[`src_buffer`] is the source buffer that holds the encoded bitstream.
    pub src_buffer: Buffer,
    ///[`src_buffer_offset`] is the buffer offset where the valid encoded
    ///bitstream starts in srcBuffer.
    ///It  **must**  meet the alignment requirement
    ///`minBitstreamBufferOffsetAlignment` within
    ///[`VideoCapabilitiesKHR`] queried with the
    ///[`get_physical_device_video_capabilities_khr`] function.
    pub src_buffer_offset: DeviceSize,
    ///[`src_buffer_range`] is the size of the srcBuffer with valid encoded
    ///bitstream, starting from [`src_buffer_offset`].
    ///It  **must**  meet the alignment requirement
    ///`minBitstreamBufferSizeAlignment` within
    ///[`VideoCapabilitiesKHR`] queried with the
    ///[`get_physical_device_video_capabilities_khr`] function.
    pub src_buffer_range: DeviceSize,
    ///[`dst_picture_resource`] is the destination
    ///[Decoded Output Picture](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#decoded-output-picture) Resource.
    pub dst_picture_resource: VideoPictureResourceKHR<'lt>,
    ///[`setup_reference_slot`] is `NULL` or a pointer to a
    ///[`VideoReferenceSlotKHR`] structure used for generating a DPB
    ///reference slot and Picture Resource.
    ///`pSetupReferenceSlot->slotIndex` specifies the slot index number to
    ///use as a target for producing the DPB data.
    ///`slotIndex` **must**  reference a valid entry as specified in
    ///[`VideoBeginCodingInfoKHR`] via the [`reference_slots`] within the
    ///[`cmd_begin_video_coding_khr`] command that established the Vulkan Video
    ///Decode Context for this command.
    pub setup_reference_slot: *const VideoReferenceSlotKHR<'lt>,
    ///[`reference_slot_count`] is the number of the DPB Reference Pictures
    ///that will be used when this decoding operation is executing.
    pub reference_slot_count: u32,
    ///[`reference_slots`] is a pointer to an array of
    ///[`VideoReferenceSlotKHR`] structures specifying the DPB Reference
    ///pictures that will be used when this decoding operation is executing.
    pub reference_slots: *const VideoReferenceSlotKHR<'lt>,
}
impl<'lt> Default for VideoDecodeInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_DECODE_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            coded_offset: Default::default(),
            coded_extent: Default::default(),
            src_buffer: Default::default(),
            src_buffer_offset: Default::default(),
            src_buffer_range: Default::default(),
            dst_picture_resource: Default::default(),
            setup_reference_slot: std::ptr::null(),
            reference_slot_count: 0,
            reference_slots: std::ptr::null(),
        }
    }
}
impl<'lt> VideoDecodeInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::setup_reference_slot`]
    pub fn setup_reference_slot_raw(&self) -> *const VideoReferenceSlotKHR<'lt> {
        self.setup_reference_slot
    }
    ///Gets the raw value of [`Self::reference_slots`]
    pub fn reference_slots_raw(&self) -> *const VideoReferenceSlotKHR<'lt> {
        self.reference_slots
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::setup_reference_slot`]
    pub fn set_setup_reference_slot_raw(&mut self, value: *const VideoReferenceSlotKHR<'lt>) -> &mut Self {
        self.setup_reference_slot = value;
        self
    }
    ///Sets the raw value of [`Self::reference_slots`]
    pub fn set_reference_slots_raw(&mut self, value: *const VideoReferenceSlotKHR<'lt>) -> &mut Self {
        self.reference_slots = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::setup_reference_slot`]
    pub fn with_setup_reference_slot_raw(mut self, value: *const VideoReferenceSlotKHR<'lt>) -> Self {
        self.setup_reference_slot = value;
        self
    }
    ///Sets the raw value of [`Self::reference_slots`]
    pub fn with_reference_slots_raw(mut self, value: *const VideoReferenceSlotKHR<'lt>) -> Self {
        self.reference_slots = value;
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
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> VideoDecodeFlagsKHR {
        self.flags
    }
    ///Gets the value of [`Self::coded_offset`]
    pub fn coded_offset(&self) -> Offset2D {
        self.coded_offset
    }
    ///Gets the value of [`Self::coded_extent`]
    pub fn coded_extent(&self) -> Extent2D {
        self.coded_extent
    }
    ///Gets the value of [`Self::src_buffer`]
    pub fn src_buffer(&self) -> Buffer {
        self.src_buffer
    }
    ///Gets the value of [`Self::src_buffer_offset`]
    pub fn src_buffer_offset(&self) -> DeviceSize {
        self.src_buffer_offset
    }
    ///Gets the value of [`Self::src_buffer_range`]
    pub fn src_buffer_range(&self) -> DeviceSize {
        self.src_buffer_range
    }
    ///Gets the value of [`Self::dst_picture_resource`]
    pub fn dst_picture_resource(&self) -> &VideoPictureResourceKHR<'lt> {
        &self.dst_picture_resource
    }
    ///Gets the value of [`Self::setup_reference_slot`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn setup_reference_slot(&self) -> &VideoReferenceSlotKHR<'lt> {
        &*self.setup_reference_slot
    }
    ///Gets the value of [`Self::reference_slot_count`]
    pub fn reference_slot_count(&self) -> u32 {
        self.reference_slot_count
    }
    ///Gets the value of [`Self::reference_slots`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn reference_slots(&self) -> &[VideoReferenceSlotKHR<'lt>] {
        std::slice::from_raw_parts(self.reference_slots, self.reference_slot_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut VideoDecodeFlagsKHR {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::coded_offset`]
    pub fn coded_offset_mut(&mut self) -> &mut Offset2D {
        &mut self.coded_offset
    }
    ///Gets a mutable reference to the value of [`Self::coded_extent`]
    pub fn coded_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.coded_extent
    }
    ///Gets a mutable reference to the value of [`Self::src_buffer`]
    pub fn src_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.src_buffer
    }
    ///Gets a mutable reference to the value of [`Self::src_buffer_offset`]
    pub fn src_buffer_offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.src_buffer_offset
    }
    ///Gets a mutable reference to the value of [`Self::src_buffer_range`]
    pub fn src_buffer_range_mut(&mut self) -> &mut DeviceSize {
        &mut self.src_buffer_range
    }
    ///Gets a mutable reference to the value of [`Self::dst_picture_resource`]
    pub fn dst_picture_resource_mut(&mut self) -> &mut VideoPictureResourceKHR<'lt> {
        &mut self.dst_picture_resource
    }
    ///Gets a mutable reference to the value of [`Self::reference_slot_count`]
    pub fn reference_slot_count_mut(&mut self) -> &mut u32 {
        &mut self.reference_slot_count
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn set_flags(&mut self, value: crate::extensions::khr_video_decode_queue::VideoDecodeFlagsKHR) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::coded_offset`]
    pub fn set_coded_offset(&mut self, value: crate::vulkan1_0::Offset2D) -> &mut Self {
        self.coded_offset = value;
        self
    }
    ///Sets the value of [`Self::coded_extent`]
    pub fn set_coded_extent(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.coded_extent = value;
        self
    }
    ///Sets the value of [`Self::src_buffer`]
    pub fn set_src_buffer(&mut self, value: crate::vulkan1_0::Buffer) -> &mut Self {
        self.src_buffer = value;
        self
    }
    ///Sets the value of [`Self::src_buffer_offset`]
    pub fn set_src_buffer_offset(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.src_buffer_offset = value;
        self
    }
    ///Sets the value of [`Self::src_buffer_range`]
    pub fn set_src_buffer_range(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.src_buffer_range = value;
        self
    }
    ///Sets the value of [`Self::dst_picture_resource`]
    pub fn set_dst_picture_resource(
        &mut self,
        value: crate::extensions::khr_video_queue::VideoPictureResourceKHR<'lt>,
    ) -> &mut Self {
        self.dst_picture_resource = value;
        self
    }
    ///Sets the value of [`Self::setup_reference_slot`]
    pub fn set_setup_reference_slot(
        &mut self,
        value: &'lt crate::extensions::khr_video_queue::VideoReferenceSlotKHR<'lt>,
    ) -> &mut Self {
        self.setup_reference_slot = value as *const _;
        self
    }
    ///Sets the value of [`Self::reference_slot_count`]
    pub fn set_reference_slot_count(&mut self, value: u32) -> &mut Self {
        self.reference_slot_count = value;
        self
    }
    ///Sets the value of [`Self::reference_slots`]
    pub fn set_reference_slots(
        &mut self,
        value: &'lt [crate::extensions::khr_video_queue::VideoReferenceSlotKHR<'lt>],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.reference_slots = value.as_ptr();
        self.reference_slot_count = len_;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn with_flags(mut self, value: crate::extensions::khr_video_decode_queue::VideoDecodeFlagsKHR) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::coded_offset`]
    pub fn with_coded_offset(mut self, value: crate::vulkan1_0::Offset2D) -> Self {
        self.coded_offset = value;
        self
    }
    ///Sets the value of [`Self::coded_extent`]
    pub fn with_coded_extent(mut self, value: crate::vulkan1_0::Extent2D) -> Self {
        self.coded_extent = value;
        self
    }
    ///Sets the value of [`Self::src_buffer`]
    pub fn with_src_buffer(mut self, value: crate::vulkan1_0::Buffer) -> Self {
        self.src_buffer = value;
        self
    }
    ///Sets the value of [`Self::src_buffer_offset`]
    pub fn with_src_buffer_offset(mut self, value: crate::vulkan1_0::DeviceSize) -> Self {
        self.src_buffer_offset = value;
        self
    }
    ///Sets the value of [`Self::src_buffer_range`]
    pub fn with_src_buffer_range(mut self, value: crate::vulkan1_0::DeviceSize) -> Self {
        self.src_buffer_range = value;
        self
    }
    ///Sets the value of [`Self::dst_picture_resource`]
    pub fn with_dst_picture_resource(
        mut self,
        value: crate::extensions::khr_video_queue::VideoPictureResourceKHR<'lt>,
    ) -> Self {
        self.dst_picture_resource = value;
        self
    }
    ///Sets the value of [`Self::setup_reference_slot`]
    pub fn with_setup_reference_slot(
        mut self,
        value: &'lt crate::extensions::khr_video_queue::VideoReferenceSlotKHR<'lt>,
    ) -> Self {
        self.setup_reference_slot = value as *const _;
        self
    }
    ///Sets the value of [`Self::reference_slot_count`]
    pub fn with_reference_slot_count(mut self, value: u32) -> Self {
        self.reference_slot_count = value;
        self
    }
    ///Sets the value of [`Self::reference_slots`]
    pub fn with_reference_slots(
        mut self,
        value: &'lt [crate::extensions::khr_video_queue::VideoReferenceSlotKHR<'lt>],
    ) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.reference_slots = value.as_ptr();
        self.reference_slot_count = len_;
        self
    }
}
#[cfg(feature = "VK_EXT_video_decode_h264")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, VideoDecodeH264PictureInfoEXT<'extender>> for VideoDecodeInfoKHR<'this>
{
    type Out = VideoDecodeInfoKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut VideoDecodeH264PictureInfoEXT<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut VideoDecodeH264PictureInfoEXT<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
}
#[cfg(feature = "VK_EXT_video_decode_h265")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, VideoDecodeH265PictureInfoEXT<'extender>> for VideoDecodeInfoKHR<'this>
{
    type Out = VideoDecodeInfoKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut VideoDecodeH265PictureInfoEXT<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut VideoDecodeH265PictureInfoEXT<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
}
impl CommandBuffer {
    ///[vkCmdDecodeVideoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDecodeVideoKHR.html) - Decode a frame
    ///# C Specifications
    ///To decode a frame, call:
    ///```c
    ///// Provided by VK_KHR_video_decode_queue
    ///void vkCmdDecodeVideoKHR(
    ///    VkCommandBuffer                             commandBuffer,
    ///    const VkVideoDecodeInfoKHR*                 pFrameInfo);
    ///```
    /// # Parameters
    /// - [`command_buffer`] is the command buffer to be filled with this function for decode frame
    ///   command.
    /// - [`p_frame_info`] is a pointer to a [`VideoDecodeInfoKHR`] structure.
    /// # Description
    /// ## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`p_frame_info`] **must**  be a valid pointer to a valid [`VideoDecodeInfoKHR`] structure
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support decode
    ///   operations
    /// - This command  **must**  only be called outside of a render pass instance
    /// - [`command_buffer`] **must**  be a primary [`CommandBuffer`]
    ///
    /// ## Host Synchronization
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    /// ## Command Properties
    /// # Related
    /// - [`khr_video_decode_queue`]
    /// - [`CommandBuffer`]
    /// - [`VideoDecodeInfoKHR`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdDecodeVideoKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_decode_video_khr<'lt>(
        self: &Unique<CommandBuffer>,
        p_frame_info: &VideoDecodeInfoKHR<'lt>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .khr_video_decode_queue()
            .and_then(|vtable| vtable.cmd_decode_video_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .khr_video_decode_queue()
            .and_then(|vtable| vtable.cmd_decode_video_khr())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), p_frame_info as *const VideoDecodeInfoKHR<'lt>);
        ()
    }
}
///The V-table of [`Device`] for functions from `VK_KHR_video_decode_queue`
pub struct DeviceKhrVideoDecodeQueueVTable {
    ///See [`FNCmdDecodeVideoKhr`] for more information.
    pub cmd_decode_video_khr: FNCmdDecodeVideoKhr,
}
impl DeviceKhrVideoDecodeQueueVTable {
    ///Loads the VTable from the owner and the names
    #[track_caller]
    pub fn load(
        loader_fn: unsafe extern "system" fn(
            Device,
            *const std::os::raw::c_char,
        ) -> Option<unsafe extern "system" fn()>,
        loader: Device,
    ) -> Self {
        Self {
            cmd_decode_video_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdDecodeVideoKHR").as_ptr()))
            },
        }
    }
    ///Gets [`Self::cmd_decode_video_khr`]. See [`FNCmdDecodeVideoKhr`] for more information.
    pub fn cmd_decode_video_khr(&self) -> FNCmdDecodeVideoKhr {
        self.cmd_decode_video_khr
    }
}
