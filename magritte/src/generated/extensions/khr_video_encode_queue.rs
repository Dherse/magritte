//![VK_KHR_video_encode_queue](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_video_encode_queue.html) - device extension
//!# Revision
//!4
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_video_queue`]`
//! - Requires `[`khr_synchronization2`]`
//! - **This is a *provisional* extension and  **must**  be used with caution. See the [description](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#boilerplate-provisional-header)
//!   of provisional header files for enablement and stability details.**
//!# Contacts
//! - Ahmed Abdelkhalek [aabdelkh](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_video_encode_queue]
//!   @aabdelkh%0A<<Here describe the issue or question you have about the VK_KHR_video_encode_queue
//!   extension>>)
//!# New functions & commands
//! - [`cmd_encode_video_khr`]
//!# New structures
//! - [`VideoEncodeInfoKHR`]
//! - Extending [`VideoCapabilitiesKHR`]:  - [`VideoEncodeCapabilitiesKHR`]
//! - Extending [`VideoCodingControlInfoKHR`]:  - [`VideoEncodeRateControlInfoKHR`]  -
//!   [`VideoEncodeRateControlLayerInfoKHR`]
//!# New enums
//! - [`VideoEncodeCapabilityFlagBitsKHR`]
//! - [`VideoEncodeFlagBitsKHR`]
//! - [`VideoEncodeRateControlFlagBitsKHR`]
//! - [`VideoEncodeRateControlModeFlagBitsKHR`]
//!# New bitmasks
//! - [`VideoEncodeCapabilityFlagsKHR`]
//! - [`VideoEncodeFlagsKHR`]
//! - [`VideoEncodeRateControlFlagsKHR`]
//! - [`VideoEncodeRateControlModeFlagsKHR`]
//!# New constants
//! - [`KHR_VIDEO_ENCODE_QUEUE_EXTENSION_NAME`]
//! - [`KHR_VIDEO_ENCODE_QUEUE_SPEC_VERSION`]
//! - Extending [`AccessFlagBits2`]:  - `VK_ACCESS_2_VIDEO_ENCODE_READ_BIT_KHR`  -
//!   `VK_ACCESS_2_VIDEO_ENCODE_WRITE_BIT_KHR`
//! - Extending [`BufferUsageFlagBits`]:  - `VK_BUFFER_USAGE_VIDEO_ENCODE_DST_BIT_KHR`  -
//!   `VK_BUFFER_USAGE_VIDEO_ENCODE_SRC_BIT_KHR`
//! - Extending [`FormatFeatureFlagBits`]:  - `VK_FORMAT_FEATURE_VIDEO_ENCODE_DPB_BIT_KHR`  -
//!   `VK_FORMAT_FEATURE_VIDEO_ENCODE_INPUT_BIT_KHR`
//! - Extending [`ImageLayout`]:  - `VK_IMAGE_LAYOUT_VIDEO_ENCODE_DPB_KHR`  -
//!   `VK_IMAGE_LAYOUT_VIDEO_ENCODE_DST_KHR`  - `VK_IMAGE_LAYOUT_VIDEO_ENCODE_SRC_KHR`
//! - Extending [`ImageUsageFlagBits`]:  - `VK_IMAGE_USAGE_VIDEO_ENCODE_DPB_BIT_KHR`  -
//!   `VK_IMAGE_USAGE_VIDEO_ENCODE_DST_BIT_KHR`  - `VK_IMAGE_USAGE_VIDEO_ENCODE_SRC_BIT_KHR`
//! - Extending [`PipelineStageFlagBits2`]:  - `VK_PIPELINE_STAGE_2_VIDEO_ENCODE_BIT_KHR`
//! - Extending [`QueryType`]:  - `VK_QUERY_TYPE_VIDEO_ENCODE_BITSTREAM_BUFFER_RANGE_KHR`
//! - Extending [`QueueFlagBits`]:  - `VK_QUEUE_VIDEO_ENCODE_BIT_KHR`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_VIDEO_ENCODE_CAPABILITIES_KHR`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_ENCODE_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_ENCODE_RATE_CONTROL_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_ENCODE_RATE_CONTROL_LAYER_INFO_KHR`
//!If [`khr_format_feature_flags2`] is supported:
//! - Extending [`FormatFeatureFlagBits2`]:  - `VK_FORMAT_FEATURE_2_VIDEO_ENCODE_DPB_BIT_KHR`  -
//!   `VK_FORMAT_FEATURE_2_VIDEO_ENCODE_INPUT_BIT_KHR`
//!# Version History
//! - Revision 1, 2018-07-23 (Ahmed Abdelkhalek)  - Initial draft
//! - Revision 1.1, 10/29/2019 (Tony Zlatinski)  - Updated the reserved spec tokens and renamed
//!   VkVideoEncoderKHR to VkVideoSessionKHR
//! - Revision 1.6, Jan 08 2020 (Tony Zlatinski)  - API unify with the video_decode_queue spec
//! - Revision 2, March 29 2021 (Tony Zlatinski)  - Spec and API updates.
//! - Revision 3, 2021-09-30 (Jon Leech)  - Add interaction with `[`khr_format_feature_flags2`]` to
//!   `vk.xml`
//! - Revision 4, 2022-02-10 (Ahmed Abdelkhalek)  - Updates to encode capability interface
//!# Other info
//! * 2022-02-10
//! * No known IP claims.
//! * - Ahmed Abdelkhalek, AMD  - Damien Kessler, NVIDIA  - Daniel Rakos, AMD  - George Hao, AMD  -
//!   Jake Beju, AMD  - Peter Fang, AMD  - Piers Daniell, NVIDIA  - Srinath Kumarapuram, NVIDIA  -
//!   Thomas J. Meier, NVIDIA  - Tony Zlatinski, NVIDIA  - Yang Liu, AMD
//!# Related
//! - [`VideoEncodeCapabilitiesKHR`]
//! - [`VideoEncodeCapabilityFlagBitsKHR`]
//! - [`VideoEncodeCapabilityFlagsKHR`]
//! - [`VideoEncodeFlagBitsKHR`]
//! - [`VideoEncodeFlagsKHR`]
//! - [`VideoEncodeInfoKHR`]
//! - [`VideoEncodeRateControlFlagBitsKHR`]
//! - [`VideoEncodeRateControlFlagsKHR`]
//! - [`VideoEncodeRateControlInfoKHR`]
//! - [`VideoEncodeRateControlLayerInfoKHR`]
//! - [`VideoEncodeRateControlModeFlagBitsKHR`]
//! - [`VideoEncodeRateControlModeFlagsKHR`]
//! - [`cmd_encode_video_khr`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
#[cfg(feature = "VK_EXT_video_encode_h264")]
pub use crate::extensions::ext_video_encode_h264::VideoEncodeH264CapabilitiesEXT;
#[cfg(feature = "VK_EXT_video_encode_h264")]
pub use crate::extensions::ext_video_encode_h264::VideoEncodeH264EmitPictureParametersEXT;
#[cfg(feature = "VK_EXT_video_encode_h264")]
pub use crate::extensions::ext_video_encode_h264::VideoEncodeH264RateControlInfoEXT;
#[cfg(feature = "VK_EXT_video_encode_h264")]
pub use crate::extensions::ext_video_encode_h264::VideoEncodeH264RateControlLayerInfoEXT;
#[cfg(feature = "VK_EXT_video_encode_h264")]
pub use crate::extensions::ext_video_encode_h264::VideoEncodeH264VclFrameInfoEXT;
#[cfg(feature = "VK_EXT_video_encode_h265")]
pub use crate::extensions::ext_video_encode_h265::VideoEncodeH265CapabilitiesEXT;
#[cfg(feature = "VK_EXT_video_encode_h265")]
pub use crate::extensions::ext_video_encode_h265::VideoEncodeH265EmitPictureParametersEXT;
#[cfg(feature = "VK_EXT_video_encode_h265")]
pub use crate::extensions::ext_video_encode_h265::VideoEncodeH265RateControlInfoEXT;
#[cfg(feature = "VK_EXT_video_encode_h265")]
pub use crate::extensions::ext_video_encode_h265::VideoEncodeH265RateControlLayerInfoEXT;
#[cfg(feature = "VK_EXT_video_encode_h265")]
pub use crate::extensions::ext_video_encode_h265::VideoEncodeH265VclFrameInfoEXT;
use crate::{
    extensions::khr_video_queue::{VideoPictureResourceKHR, VideoReferenceSlotKHR},
    vulkan1_0::{
        BaseInStructure, BaseOutStructure, Buffer, CommandBuffer, Device, DeviceSize, Extent2D, StructureType,
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
#[doc(alias = "VK_KHR_VIDEO_ENCODE_QUEUE_SPEC_VERSION")]
pub const KHR_VIDEO_ENCODE_QUEUE_SPEC_VERSION: u32 = 4;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_VIDEO_ENCODE_QUEUE_EXTENSION_NAME")]
pub const KHR_VIDEO_ENCODE_QUEUE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_video_encode_queue");
///[vkCmdEncodeVideoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEncodeVideoKHR.html) - Encode operation for bitstream generation
///# C Specifications
///To launch an encode operation that results in bitstream generation, call:
///```c
///// Provided by VK_KHR_video_encode_queue
///void vkCmdEncodeVideoKHR(
///    VkCommandBuffer                             commandBuffer,
///    const VkVideoEncodeInfoKHR*                 pEncodeInfo);
///```
///# Parameters
/// - [`command_buffer`] is the command buffer to be filled with this function for encoding to
///   generate a bitstream.
/// - [`p_encode_info`] is a pointer to a [`VideoEncodeInfoKHR`] structure.
///# Description
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`p_encode_info`] **must**  be a valid pointer to a valid [`VideoEncodeInfoKHR`] structure
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support encode
///   operations
/// - This command  **must**  only be called outside of a render pass instance
/// - [`command_buffer`] **must**  be a primary [`CommandBuffer`]
///
///## Host Synchronization
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
///## Command Properties
///# Related
/// - [`khr_video_encode_queue`]
/// - [`CommandBuffer`]
/// - [`VideoEncodeInfoKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdEncodeVideoKHR")]
pub type FNCmdEncodeVideoKhr = Option<
    for<'lt> unsafe extern "system" fn(command_buffer: CommandBuffer, p_encode_info: *const VideoEncodeInfoKHR<'lt>),
>;
///[VkVideoEncodeFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeFlagBitsKHR.html) - Video Encode Command Flags
///# C Specifications
///The [`cmd_encode_video_khr`] flags are defined with the following
///enumeration:
///```c
///// Provided by VK_KHR_video_encode_queue
///typedef enum VkVideoEncodeFlagBitsKHR {
///    VK_VIDEO_ENCODE_DEFAULT_KHR = 0,
///    VK_VIDEO_ENCODE_RESERVED_0_BIT_KHR = 0x00000001,
///} VkVideoEncodeFlagBitsKHR;
///```
///# Description
/// - [`RESERVED0`] The current version of the specification has reserved this value for future use.
///# Related
/// - [`khr_video_encode_queue`]
/// - [`VideoEncodeFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoEncodeFlagBitsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoEncodeFlagBitsKHR(u32);
impl const Default for VideoEncodeFlagBitsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl VideoEncodeFlagBitsKHR {
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
impl std::fmt::Debug for VideoEncodeFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoEncodeFlagBitsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoEncodeFlagBitsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    match self.0 {
                        VideoEncodeFlagBitsKHR::DEFAULT => f.write_str("DEFAULT")?,
                        VideoEncodeFlagBitsKHR::RESERVED0 => f.write_str("RESERVED0")?,
                        _ => f.write_str("invalid")?,
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoEncodeFlagBitsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoEncodeCapabilityFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeCapabilityFlagBitsKHR.html) - Video encode capability flags
///# C Specifications
///Bits which  **may**  be set in [`VideoEncodeCapabilitiesKHR::flags`],
///indicating the encoding tools supported, are:
///```c
///// Provided by VK_KHR_video_encode_queue
///typedef enum VkVideoEncodeCapabilityFlagBitsKHR {
///    VK_VIDEO_ENCODE_CAPABILITY_DEFAULT_KHR = 0,
///    VK_VIDEO_ENCODE_CAPABILITY_PRECEDING_EXTERNALLY_ENCODED_BYTES_BIT_KHR = 0x00000001,
///} VkVideoEncodeCapabilityFlagBitsKHR;
///```
///# Description
/// - [`PRECEDING_EXTERNALLY_ENCODED_BYTES`] reports that the implementation supports use of
///   [`VideoEncodeInfoKHR::preceding_externally_encoded_bytes`].
///# Related
/// - [`khr_video_encode_queue`]
/// - [`VideoEncodeCapabilityFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoEncodeCapabilityFlagBitsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoEncodeCapabilityFlagBitsKHR(u32);
impl const Default for VideoEncodeCapabilityFlagBitsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl VideoEncodeCapabilityFlagBitsKHR {
    ///No documentation found
    pub const DEFAULT: Self = Self(0);
    ///[`PRECEDING_EXTERNALLY_ENCODED_BYTES`]
    ///reports that the implementation supports use of
    ///[`VideoEncodeInfoKHR`]::`precedingExternallyEncodedBytes`.
    pub const PRECEDING_EXTERNALLY_ENCODED_BYTES: Self = Self(1);
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
impl std::fmt::Debug for VideoEncodeCapabilityFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoEncodeCapabilityFlagBitsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoEncodeCapabilityFlagBitsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    match self.0 {
                        VideoEncodeCapabilityFlagBitsKHR::DEFAULT => f.write_str("DEFAULT")?,
                        VideoEncodeCapabilityFlagBitsKHR::PRECEDING_EXTERNALLY_ENCODED_BYTES => {
                            f.write_str("PRECEDING_EXTERNALLY_ENCODED_BYTES")?
                        },
                        _ => f.write_str("invalid")?,
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoEncodeCapabilityFlagBitsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoEncodeRateControlFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeRateControlFlagBitsKHR.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_KHR_video_encode_queue
///typedef enum VkVideoEncodeRateControlFlagBitsKHR {
///    VK_VIDEO_ENCODE_RATE_CONTROL_DEFAULT_KHR = 0,
///    VK_VIDEO_ENCODE_RATE_CONTROL_RESERVED_0_BIT_KHR = 0x00000001,
///} VkVideoEncodeRateControlFlagBitsKHR;
///```
///# Description
///[`VideoEncodeRateControlFlagBitsKHR`] defines bits which may be set in a
///[`VideoEncodeRateControlFlagsKHR`] value, but is currently unused.
///# Related
/// - [`khr_video_encode_queue`]
/// - [`VideoEncodeRateControlFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoEncodeRateControlFlagBitsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoEncodeRateControlFlagBitsKHR(u32);
impl const Default for VideoEncodeRateControlFlagBitsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl VideoEncodeRateControlFlagBitsKHR {
    ///No documentation found
    pub const DEFAULT: Self = Self(0);
    ///No documentation found
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
impl std::fmt::Debug for VideoEncodeRateControlFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoEncodeRateControlFlagBitsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoEncodeRateControlFlagBitsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    match self.0 {
                        VideoEncodeRateControlFlagBitsKHR::DEFAULT => f.write_str("DEFAULT")?,
                        VideoEncodeRateControlFlagBitsKHR::RESERVED0 => f.write_str("RESERVED0")?,
                        _ => f.write_str("invalid")?,
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoEncodeRateControlFlagBitsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoEncodeRateControlModeFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeRateControlModeFlagBitsKHR.html) - Video encode rate control modes
///# C Specifications
///The rate control modes are defined with the following enums:
///```c
///// Provided by VK_KHR_video_encode_queue
///typedef enum VkVideoEncodeRateControlModeFlagBitsKHR {
///    VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR = 0,
///    VK_VIDEO_ENCODE_RATE_CONTROL_MODE_CBR_BIT_KHR = 1,
///    VK_VIDEO_ENCODE_RATE_CONTROL_MODE_VBR_BIT_KHR = 2,
///} VkVideoEncodeRateControlModeFlagBitsKHR;
///```
///# Description
/// - [`NONE`] for disabling rate control.
/// - [`CBR`] for constant bitrate rate control mode.
/// - [`VBR`] for variable bitrate rate control mode.
///# Related
/// - [`khr_video_encode_queue`]
/// - [`VideoEncodeRateControlInfoKHR`]
/// - [`VideoEncodeRateControlModeFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoEncodeRateControlModeFlagBitsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoEncodeRateControlModeFlagBitsKHR(u32);
impl const Default for VideoEncodeRateControlModeFlagBitsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl VideoEncodeRateControlModeFlagBitsKHR {
    ///[`NONE`] for disabling rate
    ///control.
    pub const NONE: Self = Self(0);
    ///[`CBR`] for constant bitrate
    ///rate control mode.
    pub const CBR: Self = Self(1);
    ///[`VBR`] for variable bitrate
    ///rate control mode.
    pub const VBR: Self = Self(2);
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
impl std::fmt::Debug for VideoEncodeRateControlModeFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoEncodeRateControlModeFlagBitsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoEncodeRateControlModeFlagBitsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    match self.0 {
                        VideoEncodeRateControlModeFlagBitsKHR::NONE => f.write_str("NONE")?,
                        VideoEncodeRateControlModeFlagBitsKHR::CBR => f.write_str("CBR")?,
                        VideoEncodeRateControlModeFlagBitsKHR::VBR => f.write_str("VBR")?,
                        _ => f.write_str("invalid")?,
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoEncodeRateControlModeFlagBitsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoEncodeFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeFlagBitsKHR.html) - Video Encode Command Flags
///# C Specifications
///The [`cmd_encode_video_khr`] flags are defined with the following
///enumeration:
///```c
///// Provided by VK_KHR_video_encode_queue
///typedef enum VkVideoEncodeFlagBitsKHR {
///    VK_VIDEO_ENCODE_DEFAULT_KHR = 0,
///    VK_VIDEO_ENCODE_RESERVED_0_BIT_KHR = 0x00000001,
///} VkVideoEncodeFlagBitsKHR;
///```
///# Description
/// - [`RESERVED0`] The current version of the specification has reserved this value for future use.
///# Related
/// - [`khr_video_encode_queue`]
/// - [`VideoEncodeFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoEncodeFlagsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoEncodeFlagsKHR(u32);
impl const Default for VideoEncodeFlagsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl From<VideoEncodeFlagBitsKHR> for VideoEncodeFlagsKHR {
    fn from(from: VideoEncodeFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(from.bits()) }
    }
}
impl VideoEncodeFlagsKHR {
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
impl const std::ops::BitOr for VideoEncodeFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for VideoEncodeFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for VideoEncodeFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for VideoEncodeFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for VideoEncodeFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for VideoEncodeFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for VideoEncodeFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for VideoEncodeFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for VideoEncodeFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<VideoEncodeFlagsKHR> for VideoEncodeFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoEncodeFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<VideoEncodeFlagBitsKHR> for VideoEncodeFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoEncodeFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<VideoEncodeFlagBitsKHR>>::from(i));
        }
    }
}
impl FromIterator<VideoEncodeFlagsKHR> for VideoEncodeFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoEncodeFlagsKHR>>(iterator: T) -> VideoEncodeFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<VideoEncodeFlagBitsKHR> for VideoEncodeFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoEncodeFlagBitsKHR>>(iterator: T) -> VideoEncodeFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for VideoEncodeFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoEncodeFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoEncodeFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(VideoEncodeFlagsKHR::DEFAULT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DEFAULT))?;
                    }
                    if self.0.contains(VideoEncodeFlagsKHR::RESERVED0) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(RESERVED0))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoEncodeFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoEncodeCapabilityFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeCapabilityFlagBitsKHR.html) - Video encode capability flags
///# C Specifications
///Bits which  **may**  be set in [`VideoEncodeCapabilitiesKHR::flags`],
///indicating the encoding tools supported, are:
///```c
///// Provided by VK_KHR_video_encode_queue
///typedef enum VkVideoEncodeCapabilityFlagBitsKHR {
///    VK_VIDEO_ENCODE_CAPABILITY_DEFAULT_KHR = 0,
///    VK_VIDEO_ENCODE_CAPABILITY_PRECEDING_EXTERNALLY_ENCODED_BYTES_BIT_KHR = 0x00000001,
///} VkVideoEncodeCapabilityFlagBitsKHR;
///```
///# Description
/// - [`PRECEDING_EXTERNALLY_ENCODED_BYTES`] reports that the implementation supports use of
///   [`VideoEncodeInfoKHR::preceding_externally_encoded_bytes`].
///# Related
/// - [`khr_video_encode_queue`]
/// - [`VideoEncodeCapabilityFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoEncodeCapabilityFlagsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoEncodeCapabilityFlagsKHR(u32);
impl const Default for VideoEncodeCapabilityFlagsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl From<VideoEncodeCapabilityFlagBitsKHR> for VideoEncodeCapabilityFlagsKHR {
    fn from(from: VideoEncodeCapabilityFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(from.bits()) }
    }
}
impl VideoEncodeCapabilityFlagsKHR {
    ///No documentation found
    pub const DEFAULT: Self = Self(0);
    ///[`PRECEDING_EXTERNALLY_ENCODED_BYTES`]
    ///reports that the implementation supports use of
    ///[`VideoEncodeInfoKHR`]::`precedingExternallyEncodedBytes`.
    pub const PRECEDING_EXTERNALLY_ENCODED_BYTES: Self = Self(1);
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
            all |= Self::PRECEDING_EXTERNALLY_ENCODED_BYTES;
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
impl const std::ops::BitOr for VideoEncodeCapabilityFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for VideoEncodeCapabilityFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for VideoEncodeCapabilityFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for VideoEncodeCapabilityFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for VideoEncodeCapabilityFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for VideoEncodeCapabilityFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for VideoEncodeCapabilityFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for VideoEncodeCapabilityFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for VideoEncodeCapabilityFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<VideoEncodeCapabilityFlagsKHR> for VideoEncodeCapabilityFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoEncodeCapabilityFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<VideoEncodeCapabilityFlagBitsKHR> for VideoEncodeCapabilityFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoEncodeCapabilityFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<VideoEncodeCapabilityFlagBitsKHR>>::from(i));
        }
    }
}
impl FromIterator<VideoEncodeCapabilityFlagsKHR> for VideoEncodeCapabilityFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoEncodeCapabilityFlagsKHR>>(iterator: T) -> VideoEncodeCapabilityFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeCapabilityFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<VideoEncodeCapabilityFlagBitsKHR> for VideoEncodeCapabilityFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoEncodeCapabilityFlagBitsKHR>>(
        iterator: T,
    ) -> VideoEncodeCapabilityFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeCapabilityFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for VideoEncodeCapabilityFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoEncodeCapabilityFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoEncodeCapabilityFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(VideoEncodeCapabilityFlagsKHR::DEFAULT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DEFAULT))?;
                    }
                    if self
                        .0
                        .contains(VideoEncodeCapabilityFlagsKHR::PRECEDING_EXTERNALLY_ENCODED_BYTES)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PRECEDING_EXTERNALLY_ENCODED_BYTES))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoEncodeCapabilityFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoEncodeRateControlFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeRateControlFlagBitsKHR.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_KHR_video_encode_queue
///typedef enum VkVideoEncodeRateControlFlagBitsKHR {
///    VK_VIDEO_ENCODE_RATE_CONTROL_DEFAULT_KHR = 0,
///    VK_VIDEO_ENCODE_RATE_CONTROL_RESERVED_0_BIT_KHR = 0x00000001,
///} VkVideoEncodeRateControlFlagBitsKHR;
///```
///# Description
///[`VideoEncodeRateControlFlagBitsKHR`] defines bits which may be set in a
///[`VideoEncodeRateControlFlagsKHR`] value, but is currently unused.
///# Related
/// - [`khr_video_encode_queue`]
/// - [`VideoEncodeRateControlFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoEncodeRateControlFlagsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoEncodeRateControlFlagsKHR(u32);
impl const Default for VideoEncodeRateControlFlagsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl From<VideoEncodeRateControlFlagBitsKHR> for VideoEncodeRateControlFlagsKHR {
    fn from(from: VideoEncodeRateControlFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(from.bits()) }
    }
}
impl VideoEncodeRateControlFlagsKHR {
    ///No documentation found
    pub const DEFAULT: Self = Self(0);
    ///No documentation found
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
impl const std::ops::BitOr for VideoEncodeRateControlFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for VideoEncodeRateControlFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for VideoEncodeRateControlFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for VideoEncodeRateControlFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for VideoEncodeRateControlFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for VideoEncodeRateControlFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for VideoEncodeRateControlFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for VideoEncodeRateControlFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for VideoEncodeRateControlFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<VideoEncodeRateControlFlagsKHR> for VideoEncodeRateControlFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoEncodeRateControlFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<VideoEncodeRateControlFlagBitsKHR> for VideoEncodeRateControlFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoEncodeRateControlFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<VideoEncodeRateControlFlagBitsKHR>>::from(i));
        }
    }
}
impl FromIterator<VideoEncodeRateControlFlagsKHR> for VideoEncodeRateControlFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoEncodeRateControlFlagsKHR>>(
        iterator: T,
    ) -> VideoEncodeRateControlFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeRateControlFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<VideoEncodeRateControlFlagBitsKHR> for VideoEncodeRateControlFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoEncodeRateControlFlagBitsKHR>>(
        iterator: T,
    ) -> VideoEncodeRateControlFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeRateControlFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for VideoEncodeRateControlFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoEncodeRateControlFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoEncodeRateControlFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(VideoEncodeRateControlFlagsKHR::DEFAULT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DEFAULT))?;
                    }
                    if self.0.contains(VideoEncodeRateControlFlagsKHR::RESERVED0) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(RESERVED0))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoEncodeRateControlFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoEncodeRateControlModeFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeRateControlModeFlagBitsKHR.html) - Video encode rate control modes
///# C Specifications
///The rate control modes are defined with the following enums:
///```c
///// Provided by VK_KHR_video_encode_queue
///typedef enum VkVideoEncodeRateControlModeFlagBitsKHR {
///    VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR = 0,
///    VK_VIDEO_ENCODE_RATE_CONTROL_MODE_CBR_BIT_KHR = 1,
///    VK_VIDEO_ENCODE_RATE_CONTROL_MODE_VBR_BIT_KHR = 2,
///} VkVideoEncodeRateControlModeFlagBitsKHR;
///```
///# Description
/// - [`NONE`] for disabling rate control.
/// - [`CBR`] for constant bitrate rate control mode.
/// - [`VBR`] for variable bitrate rate control mode.
///# Related
/// - [`khr_video_encode_queue`]
/// - [`VideoEncodeRateControlInfoKHR`]
/// - [`VideoEncodeRateControlModeFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoEncodeRateControlModeFlagsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VideoEncodeRateControlModeFlagsKHR(u32);
impl const Default for VideoEncodeRateControlModeFlagsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl From<VideoEncodeRateControlModeFlagBitsKHR> for VideoEncodeRateControlModeFlagsKHR {
    fn from(from: VideoEncodeRateControlModeFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(from.bits()) }
    }
}
impl VideoEncodeRateControlModeFlagsKHR {
    ///[`NONE`] for disabling rate
    ///control.
    pub const NONE: Self = Self(0);
    ///[`CBR`] for constant bitrate
    ///rate control mode.
    pub const CBR: Self = Self(1);
    ///[`VBR`] for variable bitrate
    ///rate control mode.
    pub const VBR: Self = Self(2);
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
            all |= Self::NONE;
        }
        {
            all |= Self::CBR;
        }
        {
            all |= Self::VBR;
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
impl const std::ops::BitOr for VideoEncodeRateControlModeFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for VideoEncodeRateControlModeFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for VideoEncodeRateControlModeFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for VideoEncodeRateControlModeFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for VideoEncodeRateControlModeFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for VideoEncodeRateControlModeFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for VideoEncodeRateControlModeFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for VideoEncodeRateControlModeFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for VideoEncodeRateControlModeFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<VideoEncodeRateControlModeFlagsKHR> for VideoEncodeRateControlModeFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoEncodeRateControlModeFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<VideoEncodeRateControlModeFlagBitsKHR> for VideoEncodeRateControlModeFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoEncodeRateControlModeFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<VideoEncodeRateControlModeFlagBitsKHR>>::from(i));
        }
    }
}
impl FromIterator<VideoEncodeRateControlModeFlagsKHR> for VideoEncodeRateControlModeFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoEncodeRateControlModeFlagsKHR>>(
        iterator: T,
    ) -> VideoEncodeRateControlModeFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeRateControlModeFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<VideoEncodeRateControlModeFlagBitsKHR> for VideoEncodeRateControlModeFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoEncodeRateControlModeFlagBitsKHR>>(
        iterator: T,
    ) -> VideoEncodeRateControlModeFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeRateControlModeFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for VideoEncodeRateControlModeFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoEncodeRateControlModeFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoEncodeRateControlModeFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(VideoEncodeRateControlModeFlagsKHR::NONE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(NONE))?;
                    }
                    if self.0.contains(VideoEncodeRateControlModeFlagsKHR::CBR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(CBR))?;
                    }
                    if self.0.contains(VideoEncodeRateControlModeFlagsKHR::VBR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(VBR))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoEncodeRateControlModeFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkVideoEncodeInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeInfoKHR.html) - Structure to chain codec-specific structures to
///# C Specifications
///The [`VideoEncodeInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_video_encode_queue
///typedef struct VkVideoEncodeInfoKHR {
///    VkStructureType                   sType;
///    const void*                       pNext;
///    VkVideoEncodeFlagsKHR             flags;
///    uint32_t                          qualityLevel;
///    VkExtent2D                        codedExtent;
///    VkBuffer                          dstBitstreamBuffer;
///    VkDeviceSize                      dstBitstreamBufferOffset;
///    VkDeviceSize                      dstBitstreamBufferMaxRange;
///    VkVideoPictureResourceKHR         srcPictureResource;
///    const VkVideoReferenceSlotKHR*    pSetupReferenceSlot;
///    uint32_t                          referenceSlotCount;
///    const VkVideoReferenceSlotKHR*    pReferenceSlots;
///    uint32_t                          precedingExternallyEncodedBytes;
///} VkVideoEncodeInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is a pointer to a structure extending this structure. A codec-specific extension
///   structure  **must**  be chained to specify what bitstream unit to generate with this encode
///   operation.
/// - [`flags`] is a bitmask of [`VideoEncodeFlagBitsKHR`] specifying encode flags, and is reserved
///   for future versions of this specification.
/// - [`quality_level`] is the coding quality level of the encoding. It is defined by the
///   codec-specific extensions.
/// - [`coded_extent`] is the coded size of the encode operations.
/// - [`dst_bitstream_buffer`] is the buffer where the encoded bitstream output will be produced.
/// - [`dst_bitstream_buffer_offset`] is the offset in the [`dst_bitstream_buffer`] where the
///   encoded bitstream output will start. [`dst_bitstream_buffer_offset`]’s value  **must**  be
///   aligned to [`VideoCapabilitiesKHR::min_bitstream_buffer_offset_alignment`], as reported by the
///   implementation.
/// - [`dst_bitstream_buffer_max_range`] is the maximum size of the [`dst_bitstream_buffer`] that
///   can be used while the encoded bitstream output is produced.
///   [`dst_bitstream_buffer_max_range`]’s value  **must**  be aligned to
///   [`VideoCapabilitiesKHR::min_bitstream_buffer_size_alignment`], as reported by the
///   implementation.
/// - [`src_picture_resource`] is the Picture Resource of the [Input Picture](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#input-encode-picture)
///   to be encoded by the operation.
/// - [`setup_reference_slot`] is a pointer to a [`VideoReferenceSlotKHR`] structure used for
///   generating a reconstructed reference slot and Picture Resource.
///   `pSetupReferenceSlot->slotIndex` specifies the slot index number to use as a target for
///   producing the Reconstructed (DPB) data. [`setup_reference_slot`] **must**  be one of the
///   entries provided in [`VideoBeginCodingInfoKHR`] via the [`reference_slots`] within the
///   [`cmd_begin_video_coding_khr`] command that established the Vulkan Video Encode Context for
///   this command.
/// - [`reference_slot_count`] is the number of Reconstructed Reference Pictures that will be used
///   when this encoding operation is executing.
/// - [`reference_slots`] is `NULL` or a pointer to an array of [`VideoReferenceSlotKHR`] structures
///   that will be used when this encoding operation is executing. Each entry in [`reference_slots`]
///   **must**  be one of the entries provided in [`VideoBeginCodingInfoKHR`] via the
///   [`reference_slots`] within the [`cmd_begin_video_coding_khr`] command that established the
///   Vulkan Video Encode Context for this command.
/// - [`preceding_externally_encoded_bytes`] is the number of bytes externally encoded for insertion
///   in the active video encode session overall bitstream prior to the bitstream that will be
///   generated by the implementation for this instance of [`VideoEncodeInfoKHR`]. Valid when
///   [`VideoEncodeRateControlInfoKHR::rate_control_mode`] is not
///   `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`. The value provided is used to update the
///   implementation’s rate control algorithm for the rate control layer this instance of
///   [`VideoEncodeInfoKHR`] belongs to, by accounting for the bitrate budget consumed by these
///   externally encoded bytes. See [`VideoEncodeRateControlInfoKHR`] for additional information
///   about encode rate control.
///# Description
///Multiple [`cmd_encode_video_khr`] commands  **may**  be recorded within a Vulkan
///Video Encode Context.
///The execution of each [`cmd_encode_video_khr`] command will result in
///generating codec-specific bitstream units.
///These bitstream units are generated consecutively into the bitstream buffer
///specified in [`dst_bitstream_buffer`] of a [`VideoEncodeInfoKHR`]
///structure within the [`cmd_begin_video_coding_khr`] command.
///The produced bitstream is the sum of all these bitstream units, including
///any padding between the bitstream units.
///Any bitstream padding  **must**  be filled with data compliant to the codec
///standard so as not to cause any syntax errors during decoding of the
///bitstream units with the padding included.
///The range of the bitstream buffer written  **can**  be queried via
///[video encode bitstream buffer
///range queries](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#queries-video-encode-bitstream-buffer-range).
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_INFO_KHR`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**
///   be either `NULL` or a pointer to a valid instance of
///   [`VideoEncodeH264EmitPictureParametersEXT`], [`VideoEncodeH264VclFrameInfoEXT`],
///   [`VideoEncodeH265EmitPictureParametersEXT`], or [`VideoEncodeH265VclFrameInfoEXT`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
/// - [`flags`] **must**  be a valid combination of [`VideoEncodeFlagBitsKHR`] values
/// - [`dst_bitstream_buffer`] **must**  be a valid [`Buffer`] handle
/// - [`src_picture_resource`] **must**  be a valid [`VideoPictureResourceKHR`] structure
/// - [`setup_reference_slot`] **must**  be a valid pointer to a valid [`VideoReferenceSlotKHR`]
///   structure
/// - If [`reference_slot_count`] is not `0`, [`reference_slots`] **must**  be a valid pointer to an
///   array of [`reference_slot_count`] valid [`VideoReferenceSlotKHR`] structures
///# Related
/// - [`khr_video_encode_queue`]
/// - [`Buffer`]
/// - [`DeviceSize`]
/// - [`Extent2D`]
/// - [`StructureType`]
/// - [`VideoEncodeFlagsKHR`]
/// - [`VideoPictureResourceKHR`]
/// - [`VideoReferenceSlotKHR`]
/// - [`cmd_encode_video_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoEncodeInfoKHR")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct VideoEncodeInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is a pointer to a structure extending this structure.
    ///A codec-specific extension structure  **must**  be chained to specify what
    ///bitstream unit to generate with this encode operation.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`VideoEncodeFlagBitsKHR`] specifying
    ///encode flags, and is reserved for future versions of this specification.
    pub flags: VideoEncodeFlagsKHR,
    ///[`quality_level`] is the coding quality level of the encoding.
    ///It is defined by the codec-specific extensions.
    pub quality_level: u32,
    ///[`coded_extent`] is the coded size of the encode operations.
    pub coded_extent: Extent2D,
    ///[`dst_bitstream_buffer`] is the buffer where the encoded bitstream
    ///output will be produced.
    pub dst_bitstream_buffer: Buffer,
    ///[`dst_bitstream_buffer_offset`] is the offset in the
    ///[`dst_bitstream_buffer`] where the encoded bitstream output will start.
    ///[`dst_bitstream_buffer_offset`]’s value  **must**  be aligned to
    ///[`VideoCapabilitiesKHR`]::`minBitstreamBufferOffsetAlignment`,
    ///as reported by the implementation.
    pub dst_bitstream_buffer_offset: DeviceSize,
    ///[`dst_bitstream_buffer_max_range`] is the maximum size of the
    ///[`dst_bitstream_buffer`] that can be used while the encoded bitstream
    ///output is produced.
    ///[`dst_bitstream_buffer_max_range`]’s value  **must**  be aligned to
    ///[`VideoCapabilitiesKHR`]::`minBitstreamBufferSizeAlignment`, as
    ///reported by the implementation.
    pub dst_bitstream_buffer_max_range: DeviceSize,
    ///[`src_picture_resource`] is the Picture Resource of the
    ///[Input Picture](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#input-encode-picture) to be encoded by the operation.
    pub src_picture_resource: VideoPictureResourceKHR<'lt>,
    ///[`setup_reference_slot`] is a pointer to a
    ///[`VideoReferenceSlotKHR`] structure used for generating a
    ///reconstructed reference slot and Picture Resource.
    ///`pSetupReferenceSlot->slotIndex` specifies the slot index number to
    ///use as a target for producing the Reconstructed (DPB) data.
    ///[`setup_reference_slot`] **must**  be one of the entries provided in
    ///[`VideoBeginCodingInfoKHR`] via the [`reference_slots`] within the
    ///[`cmd_begin_video_coding_khr`] command that established the Vulkan Video
    ///Encode Context for this command.
    pub setup_reference_slot: *const VideoReferenceSlotKHR<'lt>,
    ///[`reference_slot_count`] is the number of Reconstructed Reference
    ///Pictures that will be used when this encoding operation is executing.
    pub reference_slot_count: u32,
    ///[`reference_slots`] is `NULL` or a pointer to an array of
    ///[`VideoReferenceSlotKHR`] structures that will be used when this
    ///encoding operation is executing.
    ///Each entry in [`reference_slots`] **must**  be one of the entries provided
    ///in [`VideoBeginCodingInfoKHR`] via the [`reference_slots`] within
    ///the [`cmd_begin_video_coding_khr`] command that established the Vulkan
    ///Video Encode Context for this command.
    pub reference_slots: *const VideoReferenceSlotKHR<'lt>,
    ///[`preceding_externally_encoded_bytes`] is the number of bytes externally
    ///encoded for insertion in the active video encode session overall
    ///bitstream prior to the bitstream that will be generated by the
    ///implementation for this instance of [`VideoEncodeInfoKHR`].
    ///Valid when [`VideoEncodeRateControlInfoKHR`]::`rateControlMode`
    ///is not `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`.
    ///The value provided is used to update the implementation’s rate control
    ///algorithm for the rate control layer this instance of
    ///[`VideoEncodeInfoKHR`] belongs to, by accounting for the bitrate
    ///budget consumed by these externally encoded bytes.
    ///See [`VideoEncodeRateControlInfoKHR`] for additional information
    ///about encode rate control.
    pub preceding_externally_encoded_bytes: u32,
}
impl<'lt> Default for VideoEncodeInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_ENCODE_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            quality_level: 0,
            coded_extent: Default::default(),
            dst_bitstream_buffer: Default::default(),
            dst_bitstream_buffer_offset: Default::default(),
            dst_bitstream_buffer_max_range: Default::default(),
            src_picture_resource: Default::default(),
            setup_reference_slot: std::ptr::null(),
            reference_slot_count: 0,
            reference_slots: std::ptr::null(),
            preceding_externally_encoded_bytes: 0,
        }
    }
}
impl<'lt> VideoEncodeInfoKHR<'lt> {
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
    pub fn flags(&self) -> VideoEncodeFlagsKHR {
        self.flags
    }
    ///Gets the value of [`Self::quality_level`]
    pub fn quality_level(&self) -> u32 {
        self.quality_level
    }
    ///Gets the value of [`Self::coded_extent`]
    pub fn coded_extent(&self) -> Extent2D {
        self.coded_extent
    }
    ///Gets the value of [`Self::dst_bitstream_buffer`]
    pub fn dst_bitstream_buffer(&self) -> Buffer {
        self.dst_bitstream_buffer
    }
    ///Gets the value of [`Self::dst_bitstream_buffer_offset`]
    pub fn dst_bitstream_buffer_offset(&self) -> DeviceSize {
        self.dst_bitstream_buffer_offset
    }
    ///Gets the value of [`Self::dst_bitstream_buffer_max_range`]
    pub fn dst_bitstream_buffer_max_range(&self) -> DeviceSize {
        self.dst_bitstream_buffer_max_range
    }
    ///Gets the value of [`Self::src_picture_resource`]
    pub fn src_picture_resource(&self) -> &VideoPictureResourceKHR<'lt> {
        &self.src_picture_resource
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
    ///Gets the value of [`Self::preceding_externally_encoded_bytes`]
    pub fn preceding_externally_encoded_bytes(&self) -> u32 {
        self.preceding_externally_encoded_bytes
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut VideoEncodeFlagsKHR {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::quality_level`]
    pub fn quality_level_mut(&mut self) -> &mut u32 {
        &mut self.quality_level
    }
    ///Gets a mutable reference to the value of [`Self::coded_extent`]
    pub fn coded_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.coded_extent
    }
    ///Gets a mutable reference to the value of [`Self::dst_bitstream_buffer`]
    pub fn dst_bitstream_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.dst_bitstream_buffer
    }
    ///Gets a mutable reference to the value of [`Self::dst_bitstream_buffer_offset`]
    pub fn dst_bitstream_buffer_offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.dst_bitstream_buffer_offset
    }
    ///Gets a mutable reference to the value of [`Self::dst_bitstream_buffer_max_range`]
    pub fn dst_bitstream_buffer_max_range_mut(&mut self) -> &mut DeviceSize {
        &mut self.dst_bitstream_buffer_max_range
    }
    ///Gets a mutable reference to the value of [`Self::src_picture_resource`]
    pub fn src_picture_resource_mut(&mut self) -> &mut VideoPictureResourceKHR<'lt> {
        &mut self.src_picture_resource
    }
    ///Gets a mutable reference to the value of [`Self::reference_slot_count`]
    pub fn reference_slot_count_mut(&mut self) -> &mut u32 {
        &mut self.reference_slot_count
    }
    ///Gets a mutable reference to the value of [`Self::preceding_externally_encoded_bytes`]
    pub fn preceding_externally_encoded_bytes_mut(&mut self) -> &mut u32 {
        &mut self.preceding_externally_encoded_bytes
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
    pub fn set_flags(&mut self, value: crate::extensions::khr_video_encode_queue::VideoEncodeFlagsKHR) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::quality_level`]
    pub fn set_quality_level(&mut self, value: u32) -> &mut Self {
        self.quality_level = value;
        self
    }
    ///Sets the value of [`Self::coded_extent`]
    pub fn set_coded_extent(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.coded_extent = value;
        self
    }
    ///Sets the value of [`Self::dst_bitstream_buffer`]
    pub fn set_dst_bitstream_buffer(&mut self, value: crate::vulkan1_0::Buffer) -> &mut Self {
        self.dst_bitstream_buffer = value;
        self
    }
    ///Sets the value of [`Self::dst_bitstream_buffer_offset`]
    pub fn set_dst_bitstream_buffer_offset(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.dst_bitstream_buffer_offset = value;
        self
    }
    ///Sets the value of [`Self::dst_bitstream_buffer_max_range`]
    pub fn set_dst_bitstream_buffer_max_range(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.dst_bitstream_buffer_max_range = value;
        self
    }
    ///Sets the value of [`Self::src_picture_resource`]
    pub fn set_src_picture_resource(
        &mut self,
        value: crate::extensions::khr_video_queue::VideoPictureResourceKHR<'lt>,
    ) -> &mut Self {
        self.src_picture_resource = value;
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
    ///Sets the value of [`Self::preceding_externally_encoded_bytes`]
    pub fn set_preceding_externally_encoded_bytes(&mut self, value: u32) -> &mut Self {
        self.preceding_externally_encoded_bytes = value;
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
    pub fn with_flags(mut self, value: crate::extensions::khr_video_encode_queue::VideoEncodeFlagsKHR) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::quality_level`]
    pub fn with_quality_level(mut self, value: u32) -> Self {
        self.quality_level = value;
        self
    }
    ///Sets the value of [`Self::coded_extent`]
    pub fn with_coded_extent(mut self, value: crate::vulkan1_0::Extent2D) -> Self {
        self.coded_extent = value;
        self
    }
    ///Sets the value of [`Self::dst_bitstream_buffer`]
    pub fn with_dst_bitstream_buffer(mut self, value: crate::vulkan1_0::Buffer) -> Self {
        self.dst_bitstream_buffer = value;
        self
    }
    ///Sets the value of [`Self::dst_bitstream_buffer_offset`]
    pub fn with_dst_bitstream_buffer_offset(mut self, value: crate::vulkan1_0::DeviceSize) -> Self {
        self.dst_bitstream_buffer_offset = value;
        self
    }
    ///Sets the value of [`Self::dst_bitstream_buffer_max_range`]
    pub fn with_dst_bitstream_buffer_max_range(mut self, value: crate::vulkan1_0::DeviceSize) -> Self {
        self.dst_bitstream_buffer_max_range = value;
        self
    }
    ///Sets the value of [`Self::src_picture_resource`]
    pub fn with_src_picture_resource(
        mut self,
        value: crate::extensions::khr_video_queue::VideoPictureResourceKHR<'lt>,
    ) -> Self {
        self.src_picture_resource = value;
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
    ///Sets the value of [`Self::preceding_externally_encoded_bytes`]
    pub fn with_preceding_externally_encoded_bytes(mut self, value: u32) -> Self {
        self.preceding_externally_encoded_bytes = value;
        self
    }
}
#[cfg(feature = "VK_EXT_video_encode_h264")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, VideoEncodeH264VclFrameInfoEXT<'extender>> for VideoEncodeInfoKHR<'this>
{
    type Out = VideoEncodeInfoKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut VideoEncodeH264VclFrameInfoEXT<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut VideoEncodeH264VclFrameInfoEXT<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
}
#[cfg(feature = "VK_EXT_video_encode_h264")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, VideoEncodeH264EmitPictureParametersEXT<'extender>> for VideoEncodeInfoKHR<'this>
{
    type Out = VideoEncodeInfoKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut VideoEncodeH264EmitPictureParametersEXT<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut VideoEncodeH264EmitPictureParametersEXT<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
}
#[cfg(feature = "VK_EXT_video_encode_h265")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, VideoEncodeH265VclFrameInfoEXT<'extender>> for VideoEncodeInfoKHR<'this>
{
    type Out = VideoEncodeInfoKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut VideoEncodeH265VclFrameInfoEXT<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut VideoEncodeH265VclFrameInfoEXT<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
}
#[cfg(feature = "VK_EXT_video_encode_h265")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, VideoEncodeH265EmitPictureParametersEXT<'extender>> for VideoEncodeInfoKHR<'this>
{
    type Out = VideoEncodeInfoKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut VideoEncodeH265EmitPictureParametersEXT<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut VideoEncodeH265EmitPictureParametersEXT<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
}
///[VkVideoEncodeRateControlInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeRateControlInfoKHR.html) - Structure to set encode stream rate control parameters
///# C Specifications
///The [`VideoEncodeRateControlInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_video_encode_queue
///typedef struct VkVideoEncodeRateControlInfoKHR {
///    VkStructureType                                sType;
///    const void*                                    pNext;
///    VkVideoEncodeRateControlFlagsKHR               flags;
///    VkVideoEncodeRateControlModeFlagBitsKHR        rateControlMode;
///    uint8_t                                        layerCount;
///    const VkVideoEncodeRateControlLayerInfoKHR*    pLayerConfigs;
///} VkVideoEncodeRateControlInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is a bitmask of [`VideoEncodeRateControlFlagBitsKHR`] specifying encode rate control
///   flags.
/// - [`rate_control_mode`] is a [`VideoEncodeRateControlModeFlagBitsKHR`] value specifying the
///   encode stream rate control mode.
/// - [`layer_count`] specifies the number of rate control layers in the video encode stream.
/// - [`layer_configs`] is a pointer to an array of [`VideoEncodeRateControlLayerInfoKHR`]
///   structures specifying the rate control configurations of [`layer_count`] rate control layers.
///# Description
///In order to provide video encode stream rate control settings, add a
///[`VideoEncodeRateControlInfoKHR`] structure to the [`p_next`] chain of
///the [`VideoCodingControlInfoKHR`] structure passed to the
///[`cmd_control_video_coding_khr`] command.A codec-specific extension structure for further encode
/// stream rate control
///parameter settings  **may**  be chained to [`VideoEncodeRateControlInfoKHR`].To ensure that the
/// video session is properly initalized with stream-level
///rate control settings, the application  **must**  call
///[`cmd_control_video_coding_khr`] with stream-level rate control settings at
///least once in execution order before the first [`cmd_encode_video_khr`]
///command that is executed after video session reset.
///If not provided, default implementation-specific stream rate control
///settings will be used.Stream rate control settings  **can**  also be re-initialized during an
/// active
///video encoding session.
///The re-initialization takes effect whenever the
///[`VideoEncodeRateControlInfoKHR`] structure is included in the
///[`p_next`] chain of the [`VideoCodingControlInfoKHR`] structure in the
///call to [`cmd_control_video_coding_khr`], and only impacts
///[`cmd_encode_video_khr`] operations that follow in execution order.
///## Valid Usage
/// - [`VideoEncodeH264RateControlInfoEXT`] **must**  be included in the [`p_next`] chain of
///   [`VideoEncodeRateControlInfoKHR`] if and only if
///   [`VideoEncodeRateControlInfoKHR`]::[`rate_control_mode`] is not
///   `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR` and the bound video session was created with
///   [`VideoProfileKHR::video_codec_operation`] set to
///   `VK_VIDEO_CODEC_OPERATION_ENCODE_H264_BIT_EXT`.
/// - If [`VideoEncodeRateControlInfoKHR`]::[`layer_count`] is greater than `1`, then
///   [`VideoEncodeH264RateControlInfoEXT::temporal_layer_count`] **must**  be equal to
///   [`layer_count`].
/// - [`VideoEncodeH265RateControlInfoEXT`] **must**  be included in the [`p_next`] chain of
///   [`VideoEncodeRateControlInfoKHR`] if and only if
///   [`VideoEncodeRateControlInfoKHR`]::[`rate_control_mode`] is not
///   `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR` and the bound video session was created with
///   [`VideoProfileKHR::video_codec_operation`] set to
///   `VK_VIDEO_CODEC_OPERATION_ENCODE_H265_BIT_EXT`.
/// - If [`VideoEncodeRateControlInfoKHR`]::[`layer_count`] is greater than `1`, then
///   [`VideoEncodeH265RateControlInfoEXT::sub_layer_count`] **must**  be equal to [`layer_count`].
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_RATE_CONTROL_INFO_KHR`
/// - [`rate_control_mode`] **must**  be a valid [`VideoEncodeRateControlModeFlagBitsKHR`] value
/// - [`layer_configs`] **must**  be a valid pointer to an array of [`layer_count`] valid
///   [`VideoEncodeRateControlLayerInfoKHR`] structures
/// - [`layer_count`] **must**  be greater than `0`
///# Related
/// - [`khr_video_encode_queue`]
/// - [`StructureType`]
/// - [`VideoEncodeRateControlFlagsKHR`]
/// - [`VideoEncodeRateControlLayerInfoKHR`]
/// - [`VideoEncodeRateControlModeFlagBitsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoEncodeRateControlInfoKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct VideoEncodeRateControlInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`VideoEncodeRateControlFlagBitsKHR`]
    ///specifying encode rate control flags.
    pub flags: VideoEncodeRateControlFlagsKHR,
    ///[`rate_control_mode`] is a [`VideoEncodeRateControlModeFlagBitsKHR`]
    ///value specifying the encode stream rate control mode.
    pub rate_control_mode: VideoEncodeRateControlModeFlagBitsKHR,
    ///[`layer_count`] specifies the number of rate control layers in the
    ///video encode stream.
    pub layer_count: u8,
    ///[`layer_configs`] is a pointer to an array of
    ///[`VideoEncodeRateControlLayerInfoKHR`] structures specifying the
    ///rate control configurations of [`layer_count`] rate control layers.
    pub layer_configs: *const VideoEncodeRateControlLayerInfoKHR<'lt>,
}
impl<'lt> Default for VideoEncodeRateControlInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_ENCODE_RATE_CONTROL_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            rate_control_mode: Default::default(),
            layer_count: 0,
            layer_configs: std::ptr::null(),
        }
    }
}
impl<'lt> VideoEncodeRateControlInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::layer_configs`]
    pub fn layer_configs_raw(&self) -> *const VideoEncodeRateControlLayerInfoKHR<'lt> {
        self.layer_configs
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::layer_configs`]
    pub fn set_layer_configs_raw(&mut self, value: *const VideoEncodeRateControlLayerInfoKHR<'lt>) -> &mut Self {
        self.layer_configs = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::layer_configs`]
    pub fn with_layer_configs_raw(mut self, value: *const VideoEncodeRateControlLayerInfoKHR<'lt>) -> Self {
        self.layer_configs = value;
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
    pub fn flags(&self) -> VideoEncodeRateControlFlagsKHR {
        self.flags
    }
    ///Gets the value of [`Self::rate_control_mode`]
    pub fn rate_control_mode(&self) -> VideoEncodeRateControlModeFlagBitsKHR {
        self.rate_control_mode
    }
    ///Gets the value of [`Self::layer_count`]
    pub fn layer_count(&self) -> u8 {
        self.layer_count
    }
    ///Gets the value of [`Self::layer_configs`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn layer_configs(&self) -> &[VideoEncodeRateControlLayerInfoKHR<'lt>] {
        std::slice::from_raw_parts(self.layer_configs, self.layer_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut VideoEncodeRateControlFlagsKHR {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::rate_control_mode`]
    pub fn rate_control_mode_mut(&mut self) -> &mut VideoEncodeRateControlModeFlagBitsKHR {
        &mut self.rate_control_mode
    }
    ///Gets a mutable reference to the value of [`Self::layer_count`]
    pub fn layer_count_mut(&mut self) -> &mut u8 {
        &mut self.layer_count
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
    pub fn set_flags(
        &mut self,
        value: crate::extensions::khr_video_encode_queue::VideoEncodeRateControlFlagsKHR,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::rate_control_mode`]
    pub fn set_rate_control_mode(
        &mut self,
        value: crate::extensions::khr_video_encode_queue::VideoEncodeRateControlModeFlagBitsKHR,
    ) -> &mut Self {
        self.rate_control_mode = value;
        self
    }
    ///Sets the value of [`Self::layer_count`]
    pub fn set_layer_count(&mut self, value: u8) -> &mut Self {
        self.layer_count = value;
        self
    }
    ///Sets the value of [`Self::layer_configs`]
    pub fn set_layer_configs(
        &mut self,
        value: &'lt [crate::extensions::khr_video_encode_queue::VideoEncodeRateControlLayerInfoKHR<'lt>],
    ) -> &mut Self {
        let len_ = value.len() as u8;
        let len_ = len_;
        self.layer_configs = value.as_ptr();
        self.layer_count = len_;
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
    pub fn with_flags(
        mut self,
        value: crate::extensions::khr_video_encode_queue::VideoEncodeRateControlFlagsKHR,
    ) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::rate_control_mode`]
    pub fn with_rate_control_mode(
        mut self,
        value: crate::extensions::khr_video_encode_queue::VideoEncodeRateControlModeFlagBitsKHR,
    ) -> Self {
        self.rate_control_mode = value;
        self
    }
    ///Sets the value of [`Self::layer_count`]
    pub fn with_layer_count(mut self, value: u8) -> Self {
        self.layer_count = value;
        self
    }
    ///Sets the value of [`Self::layer_configs`]
    pub fn with_layer_configs(
        mut self,
        value: &'lt [crate::extensions::khr_video_encode_queue::VideoEncodeRateControlLayerInfoKHR<'lt>],
    ) -> Self {
        let len_ = value.len() as u8;
        let len_ = len_;
        self.layer_configs = value.as_ptr();
        self.layer_count = len_;
        self
    }
}
#[cfg(feature = "VK_EXT_video_encode_h264")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, VideoEncodeH264RateControlInfoEXT<'extender>> for VideoEncodeRateControlInfoKHR<'this>
{
    type Out = VideoEncodeRateControlInfoKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut VideoEncodeH264RateControlInfoEXT<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut VideoEncodeH264RateControlInfoEXT<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
}
#[cfg(feature = "VK_EXT_video_encode_h265")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, VideoEncodeH265RateControlInfoEXT<'extender>> for VideoEncodeRateControlInfoKHR<'this>
{
    type Out = VideoEncodeRateControlInfoKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut VideoEncodeH265RateControlInfoEXT<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut VideoEncodeH265RateControlInfoEXT<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
}
///[VkVideoEncodeRateControlLayerInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeRateControlLayerInfoKHR.html) - Structure to set encode per-layer rate control parameters
///# C Specifications
///The [`VideoEncodeRateControlLayerInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_video_encode_queue
///typedef struct VkVideoEncodeRateControlLayerInfoKHR {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           averageBitrate;
///    uint32_t           maxBitrate;
///    uint32_t           frameRateNumerator;
///    uint32_t           frameRateDenominator;
///    uint32_t           virtualBufferSizeInMs;
///    uint32_t           initialVirtualBufferSizeInMs;
///} VkVideoEncodeRateControlLayerInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is a pointer to a structure extending this structure.
/// - [`average_bitrate`] is the average bitrate in bits/second. Valid when rate control mode is not
///   `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`.
/// - [`max_bitrate`] is the peak bitrate in bits/second. Valid when rate control mode is
///   `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_VBR_BIT_KHR`.
/// - [`frame_rate_numerator`] is the numerator of the frame rate. Valid when rate control mode is
///   not `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`.
/// - [`frame_rate_denominator`] is the denominator of the frame rate. Valid when rate control mode
///   is not `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`.
/// - [`virtual_buffer_size_in_ms`] is the leaky bucket model virtual buffer size in milliseconds,
///   with respect to peak bitrate. Valid when rate control mode is not
///   `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`. For example, virtual buffer size is
///   ([`virtual_buffer_size_in_ms`] * [`max_bitrate`] / 1000).
/// - [`initial_virtual_buffer_size_in_ms`] is the initial occupancy in milliseconds of the virtual
///   buffer in the leaky bucket model. Valid when the rate control mode is not
///   `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`.
///# Description
///A codec-specific structure specifying additional per-layer rate control
///settings  **must**  be chained to [`VideoEncodeRateControlLayerInfoKHR`].
///If multiple rate control layers are enabled
///([`VideoEncodeRateControlInfoKHR::layer_count`] is greater than 1),
///then the chained codec-specific extension structure also identifies the
///specific video coding layer its parent
///[`VideoEncodeRateControlLayerInfoKHR`] applies to.
///If multiple rate control layers are enabled, the number of rate control
///layers  **must**  match the number of video coding layers.
///The specification for an encode codec-specific extension would describe how
///multiple video coding layers are enabled for the corresponding codec.Per-layer rate control
/// settings for all enabled rate control layers  **must**  be
///initialized or re-initialized whenever stream rate control settings are
///provided via [`VideoEncodeRateControlInfoKHR`].
///This is done by specifying settings for all enabled rate control layers in
///[`VideoEncodeRateControlInfoKHR::layer_configs`].To adjust rate control settings for an
/// individual layer at runtime, add a
///[`VideoEncodeRateControlLayerInfoKHR`] structure to the [`p_next`]
///chain of the [`VideoCodingControlInfoKHR`] structure passed to the
///[`cmd_control_video_coding_khr`] command.
///This adjustment only impacts the specified layer without impacting the rate
///control settings or implementation rate control algorithm behavior for any
///other enabled rate control layers.
///The adjustment takes effect whenever the corresponding
///[`cmd_control_video_coding_khr`] is executed, and only impacts
///[`cmd_encode_video_khr`] operations pertaining to the corresponding video
///coding layer that follow in execution order.It is possible for an application to enable multiple
/// video coding layers
///(via codec-specific extensions to encoding operations) while only enabling a
///single layer of rate control for the entire video stream.
///To achieve this, `layerCount` in [`VideoEncodeRateControlInfoKHR`] **must**  be set to 1, and
/// the single [`VideoEncodeRateControlLayerInfoKHR`]
///provided in `pLayerConfigs` would apply to all encoded segments of the
///video stream, regardless of which codec-defined video coding layer they
///belong to.
///In this case, the implementation decides bitrate distribution across video
///coding layers (if applicable to the specified stream rate control mode).
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_RATE_CONTROL_LAYER_INFO_KHR`
///# Related
/// - [`khr_video_encode_queue`]
/// - [`StructureType`]
/// - [`VideoEncodeRateControlInfoKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoEncodeRateControlLayerInfoKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct VideoEncodeRateControlLayerInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is a pointer to a structure extending this structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`average_bitrate`] is the average bitrate in bits/second.
    ///Valid when rate control mode is not
    ///`VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`.
    pub average_bitrate: u32,
    ///[`max_bitrate`] is the peak bitrate in bits/second.
    ///Valid when rate control mode is
    ///`VK_VIDEO_ENCODE_RATE_CONTROL_MODE_VBR_BIT_KHR`.
    pub max_bitrate: u32,
    ///[`frame_rate_numerator`] is the numerator of the frame rate.
    ///Valid when rate control mode is not
    ///`VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`.
    pub frame_rate_numerator: u32,
    ///[`frame_rate_denominator`] is the denominator of the frame rate.
    ///Valid when rate control mode is not
    ///`VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`.
    pub frame_rate_denominator: u32,
    ///[`virtual_buffer_size_in_ms`] is the leaky bucket model virtual buffer
    ///size in milliseconds, with respect to peak bitrate.
    ///Valid when rate control mode is not
    ///`VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`.
    ///For example, virtual buffer size is ([`virtual_buffer_size_in_ms`] *
    ///[`max_bitrate`] / 1000).
    pub virtual_buffer_size_in_ms: u32,
    ///[`initial_virtual_buffer_size_in_ms`] is the initial occupancy in
    ///milliseconds of the virtual buffer in the leaky bucket model.
    ///Valid when the rate control mode is not
    ///`VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`.
    pub initial_virtual_buffer_size_in_ms: u32,
}
impl<'lt> Default for VideoEncodeRateControlLayerInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_ENCODE_RATE_CONTROL_LAYER_INFO_KHR,
            p_next: std::ptr::null(),
            average_bitrate: 0,
            max_bitrate: 0,
            frame_rate_numerator: 0,
            frame_rate_denominator: 0,
            virtual_buffer_size_in_ms: 0,
            initial_virtual_buffer_size_in_ms: 0,
        }
    }
}
impl<'lt> VideoEncodeRateControlLayerInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::average_bitrate`]
    pub fn average_bitrate(&self) -> u32 {
        self.average_bitrate
    }
    ///Gets the value of [`Self::max_bitrate`]
    pub fn max_bitrate(&self) -> u32 {
        self.max_bitrate
    }
    ///Gets the value of [`Self::frame_rate_numerator`]
    pub fn frame_rate_numerator(&self) -> u32 {
        self.frame_rate_numerator
    }
    ///Gets the value of [`Self::frame_rate_denominator`]
    pub fn frame_rate_denominator(&self) -> u32 {
        self.frame_rate_denominator
    }
    ///Gets the value of [`Self::virtual_buffer_size_in_ms`]
    pub fn virtual_buffer_size_in_ms(&self) -> u32 {
        self.virtual_buffer_size_in_ms
    }
    ///Gets the value of [`Self::initial_virtual_buffer_size_in_ms`]
    pub fn initial_virtual_buffer_size_in_ms(&self) -> u32 {
        self.initial_virtual_buffer_size_in_ms
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::average_bitrate`]
    pub fn average_bitrate_mut(&mut self) -> &mut u32 {
        &mut self.average_bitrate
    }
    ///Gets a mutable reference to the value of [`Self::max_bitrate`]
    pub fn max_bitrate_mut(&mut self) -> &mut u32 {
        &mut self.max_bitrate
    }
    ///Gets a mutable reference to the value of [`Self::frame_rate_numerator`]
    pub fn frame_rate_numerator_mut(&mut self) -> &mut u32 {
        &mut self.frame_rate_numerator
    }
    ///Gets a mutable reference to the value of [`Self::frame_rate_denominator`]
    pub fn frame_rate_denominator_mut(&mut self) -> &mut u32 {
        &mut self.frame_rate_denominator
    }
    ///Gets a mutable reference to the value of [`Self::virtual_buffer_size_in_ms`]
    pub fn virtual_buffer_size_in_ms_mut(&mut self) -> &mut u32 {
        &mut self.virtual_buffer_size_in_ms
    }
    ///Gets a mutable reference to the value of [`Self::initial_virtual_buffer_size_in_ms`]
    pub fn initial_virtual_buffer_size_in_ms_mut(&mut self) -> &mut u32 {
        &mut self.initial_virtual_buffer_size_in_ms
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
    ///Sets the value of [`Self::average_bitrate`]
    pub fn set_average_bitrate(&mut self, value: u32) -> &mut Self {
        self.average_bitrate = value;
        self
    }
    ///Sets the value of [`Self::max_bitrate`]
    pub fn set_max_bitrate(&mut self, value: u32) -> &mut Self {
        self.max_bitrate = value;
        self
    }
    ///Sets the value of [`Self::frame_rate_numerator`]
    pub fn set_frame_rate_numerator(&mut self, value: u32) -> &mut Self {
        self.frame_rate_numerator = value;
        self
    }
    ///Sets the value of [`Self::frame_rate_denominator`]
    pub fn set_frame_rate_denominator(&mut self, value: u32) -> &mut Self {
        self.frame_rate_denominator = value;
        self
    }
    ///Sets the value of [`Self::virtual_buffer_size_in_ms`]
    pub fn set_virtual_buffer_size_in_ms(&mut self, value: u32) -> &mut Self {
        self.virtual_buffer_size_in_ms = value;
        self
    }
    ///Sets the value of [`Self::initial_virtual_buffer_size_in_ms`]
    pub fn set_initial_virtual_buffer_size_in_ms(&mut self, value: u32) -> &mut Self {
        self.initial_virtual_buffer_size_in_ms = value;
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
    ///Sets the value of [`Self::average_bitrate`]
    pub fn with_average_bitrate(mut self, value: u32) -> Self {
        self.average_bitrate = value;
        self
    }
    ///Sets the value of [`Self::max_bitrate`]
    pub fn with_max_bitrate(mut self, value: u32) -> Self {
        self.max_bitrate = value;
        self
    }
    ///Sets the value of [`Self::frame_rate_numerator`]
    pub fn with_frame_rate_numerator(mut self, value: u32) -> Self {
        self.frame_rate_numerator = value;
        self
    }
    ///Sets the value of [`Self::frame_rate_denominator`]
    pub fn with_frame_rate_denominator(mut self, value: u32) -> Self {
        self.frame_rate_denominator = value;
        self
    }
    ///Sets the value of [`Self::virtual_buffer_size_in_ms`]
    pub fn with_virtual_buffer_size_in_ms(mut self, value: u32) -> Self {
        self.virtual_buffer_size_in_ms = value;
        self
    }
    ///Sets the value of [`Self::initial_virtual_buffer_size_in_ms`]
    pub fn with_initial_virtual_buffer_size_in_ms(mut self, value: u32) -> Self {
        self.initial_virtual_buffer_size_in_ms = value;
        self
    }
}
#[cfg(feature = "VK_EXT_video_encode_h264")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, VideoEncodeH264RateControlLayerInfoEXT<'extender>>
    for VideoEncodeRateControlLayerInfoKHR<'this>
{
    type Out = VideoEncodeRateControlLayerInfoKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut VideoEncodeH264RateControlLayerInfoEXT<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut VideoEncodeH264RateControlLayerInfoEXT<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
}
#[cfg(feature = "VK_EXT_video_encode_h265")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, VideoEncodeH265RateControlLayerInfoEXT<'extender>>
    for VideoEncodeRateControlLayerInfoKHR<'this>
{
    type Out = VideoEncodeRateControlLayerInfoKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut VideoEncodeH265RateControlLayerInfoEXT<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut VideoEncodeH265RateControlLayerInfoEXT<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
}
///[VkVideoEncodeCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeCapabilitiesKHR.html) - Structure specifying encode capabilities
///# C Specifications
///When calling [`get_physical_device_video_capabilities_khr`] with
///`pVideoProfile->videoCodecOperation` specified as one of the encode
///operation bits, the [`VideoEncodeCapabilitiesKHR`] structure  **must**  be
///included in the [`p_next`] chain of the [`VideoCapabilitiesKHR`]
///structure to retrieve capabilities specific to video encoding.The [`VideoEncodeCapabilitiesKHR`]
/// structure is defined as:
///```c
///// Provided by VK_KHR_video_encode_queue
///typedef struct VkVideoEncodeCapabilitiesKHR {
///    VkStructureType                         sType;
///    const void*                             pNext;
///    VkVideoEncodeCapabilityFlagsKHR         flags;
///    VkVideoEncodeRateControlModeFlagsKHR    rateControlModes;
///    uint8_t                                 rateControlLayerCount;
///    uint8_t                                 qualityLevelCount;
///    VkExtent2D                              inputImageDataFillAlignment;
///} VkVideoEncodeCapabilitiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is a bitmask of [`VideoEncodeCapabilityFlagBitsKHR`] describing supported encoding
///   features.
/// - [`rate_control_modes`] is a bitmask of [`VideoEncodeRateControlModeFlagBitsKHR`] describing
///   supported rate control modes. All implementations  **must**  support
///   `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`.
/// - [`rate_control_layer_count`] reports the maximum number of rate control layers supported.
///   Implementations  **must**  report at least 1.
/// - [`quality_level_count`] is the number of discrete quality levels supported. Implementations
///   **must**  report at least 1.
/// - [`input_image_data_fill_alignment`] reports alignment of data that should be filled in the
///   input image horizontally and vertically in pixels before encode operations are performed on
///   the input image.
///# Description
///The input content and encode resolution (specified in
///[`VideoEncodeInfoKHR::coded_extent`]) may not be aligned with the
///codec-specific coding block size.
///For example, the input content may be 1920x1080 and the coding block size
///may be 16x16 pixel blocks.
///In this example, the content is horizontally aligned with the coding block
///size, but not vertically aligned with the coding block size.
///Encoding of the last row of blocks may be impacted by contents of the input
///image in pixel rows 1081 to 1088 (the next vertical alignment with the
///coding block size).
///In general, to ensure efficient encoding for the last row/column of blocks,
///and/or to ensure consistent encoding results between repeated encoding of
///the same input content, these extra pixel rows/columns should be filled to
///known values up to the coding block size alignment before encoding
///operations are performed.
///Some implementations support performing auto-fill of unaligned pixels beyond
///a specific alignment, which is reported in
///[`input_image_data_fill_alignment`].
///For example, if an implementation reports 1x1 in
///[`input_image_data_fill_alignment`], then the implementation will perform
///auto-fill for any unaligned pixels beyond the encode resolution up to the
///next coding block size.
///For a coding block size of 16x16, if the implementation reports 16x16 in
///[`input_image_data_fill_alignment`], then it is the application’s
///responsibility to fill any unaligned pixels, if desired.
///If not, it may impact the encoding efficiency, but it will not affect the
///validity of the generated bitstream.
///If the implementation reports 8x8 in [`input_image_data_fill_alignment`], then
///for the 1920x1080 example, since the content is aligned to 8 pixels
///vertically, the implementation will auto-fill pixel rows 1081 to 1088 (up to
///the 16x16 coding block size in the example).
///The auto-fill value(s) are implementation-specific.
///The auto-fill value(s) are not written to the input image memory, but are
///used as part of the encoding operation on the input image.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_CAPABILITIES_KHR`
/// - [`rate_control_modes`] **must**  be a valid combination of
///   [`VideoEncodeRateControlModeFlagBitsKHR`] values
/// - [`rate_control_modes`] **must**  not be `0`
///# Related
/// - [`khr_video_encode_queue`]
/// - [`Extent2D`]
/// - [`StructureType`]
/// - [`VideoEncodeCapabilityFlagsKHR`]
/// - [`VideoEncodeRateControlModeFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVideoEncodeCapabilitiesKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct VideoEncodeCapabilitiesKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`VideoEncodeCapabilityFlagBitsKHR`]
    ///describing supported encoding features.
    pub flags: VideoEncodeCapabilityFlagsKHR,
    ///[`rate_control_modes`] is a bitmask of
    ///[`VideoEncodeRateControlModeFlagBitsKHR`] describing supported rate
    ///control modes.
    ///All implementations  **must**  support
    ///`VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`.
    pub rate_control_modes: VideoEncodeRateControlModeFlagsKHR,
    ///[`rate_control_layer_count`] reports the maximum number of rate control
    ///layers supported.
    ///Implementations  **must**  report at least 1.
    pub rate_control_layer_count: u8,
    ///[`quality_level_count`] is the number of discrete quality levels
    ///supported.
    ///Implementations  **must**  report at least 1.
    pub quality_level_count: u8,
    ///[`input_image_data_fill_alignment`] reports alignment of data that should
    ///be filled in the input image horizontally and vertically in pixels
    ///before encode operations are performed on the input image.
    pub input_image_data_fill_alignment: Extent2D,
}
impl<'lt> Default for VideoEncodeCapabilitiesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_ENCODE_CAPABILITIES_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            rate_control_modes: Default::default(),
            rate_control_layer_count: 0,
            quality_level_count: 0,
            input_image_data_fill_alignment: Default::default(),
        }
    }
}
impl<'lt> VideoEncodeCapabilitiesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> VideoEncodeCapabilityFlagsKHR {
        self.flags
    }
    ///Gets the value of [`Self::rate_control_modes`]
    pub fn rate_control_modes(&self) -> VideoEncodeRateControlModeFlagsKHR {
        self.rate_control_modes
    }
    ///Gets the value of [`Self::rate_control_layer_count`]
    pub fn rate_control_layer_count(&self) -> u8 {
        self.rate_control_layer_count
    }
    ///Gets the value of [`Self::quality_level_count`]
    pub fn quality_level_count(&self) -> u8 {
        self.quality_level_count
    }
    ///Gets the value of [`Self::input_image_data_fill_alignment`]
    pub fn input_image_data_fill_alignment(&self) -> Extent2D {
        self.input_image_data_fill_alignment
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut VideoEncodeCapabilityFlagsKHR {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::rate_control_modes`]
    pub fn rate_control_modes_mut(&mut self) -> &mut VideoEncodeRateControlModeFlagsKHR {
        &mut self.rate_control_modes
    }
    ///Gets a mutable reference to the value of [`Self::rate_control_layer_count`]
    pub fn rate_control_layer_count_mut(&mut self) -> &mut u8 {
        &mut self.rate_control_layer_count
    }
    ///Gets a mutable reference to the value of [`Self::quality_level_count`]
    pub fn quality_level_count_mut(&mut self) -> &mut u8 {
        &mut self.quality_level_count
    }
    ///Gets a mutable reference to the value of [`Self::input_image_data_fill_alignment`]
    pub fn input_image_data_fill_alignment_mut(&mut self) -> &mut Extent2D {
        &mut self.input_image_data_fill_alignment
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
    pub fn set_flags(
        &mut self,
        value: crate::extensions::khr_video_encode_queue::VideoEncodeCapabilityFlagsKHR,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::rate_control_modes`]
    pub fn set_rate_control_modes(
        &mut self,
        value: crate::extensions::khr_video_encode_queue::VideoEncodeRateControlModeFlagsKHR,
    ) -> &mut Self {
        self.rate_control_modes = value;
        self
    }
    ///Sets the value of [`Self::rate_control_layer_count`]
    pub fn set_rate_control_layer_count(&mut self, value: u8) -> &mut Self {
        self.rate_control_layer_count = value;
        self
    }
    ///Sets the value of [`Self::quality_level_count`]
    pub fn set_quality_level_count(&mut self, value: u8) -> &mut Self {
        self.quality_level_count = value;
        self
    }
    ///Sets the value of [`Self::input_image_data_fill_alignment`]
    pub fn set_input_image_data_fill_alignment(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.input_image_data_fill_alignment = value;
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
    pub fn with_flags(
        mut self,
        value: crate::extensions::khr_video_encode_queue::VideoEncodeCapabilityFlagsKHR,
    ) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::rate_control_modes`]
    pub fn with_rate_control_modes(
        mut self,
        value: crate::extensions::khr_video_encode_queue::VideoEncodeRateControlModeFlagsKHR,
    ) -> Self {
        self.rate_control_modes = value;
        self
    }
    ///Sets the value of [`Self::rate_control_layer_count`]
    pub fn with_rate_control_layer_count(mut self, value: u8) -> Self {
        self.rate_control_layer_count = value;
        self
    }
    ///Sets the value of [`Self::quality_level_count`]
    pub fn with_quality_level_count(mut self, value: u8) -> Self {
        self.quality_level_count = value;
        self
    }
    ///Sets the value of [`Self::input_image_data_fill_alignment`]
    pub fn with_input_image_data_fill_alignment(mut self, value: crate::vulkan1_0::Extent2D) -> Self {
        self.input_image_data_fill_alignment = value;
        self
    }
}
#[cfg(feature = "VK_EXT_video_encode_h264")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, VideoEncodeH264CapabilitiesEXT<'extender>> for VideoEncodeCapabilitiesKHR<'this>
{
    type Out = VideoEncodeCapabilitiesKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut VideoEncodeH264CapabilitiesEXT<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut VideoEncodeH264CapabilitiesEXT<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
}
#[cfg(feature = "VK_EXT_video_encode_h265")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, VideoEncodeH265CapabilitiesEXT<'extender>> for VideoEncodeCapabilitiesKHR<'this>
{
    type Out = VideoEncodeCapabilitiesKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut VideoEncodeH265CapabilitiesEXT<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut VideoEncodeH265CapabilitiesEXT<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
}
impl CommandBuffer {
    ///[vkCmdEncodeVideoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEncodeVideoKHR.html) - Encode operation for bitstream generation
    ///# C Specifications
    ///To launch an encode operation that results in bitstream generation, call:
    ///```c
    ///// Provided by VK_KHR_video_encode_queue
    ///void vkCmdEncodeVideoKHR(
    ///    VkCommandBuffer                             commandBuffer,
    ///    const VkVideoEncodeInfoKHR*                 pEncodeInfo);
    ///```
    ///# Parameters
    /// - [`command_buffer`] is the command buffer to be filled with this function for encoding to
    ///   generate a bitstream.
    /// - [`p_encode_info`] is a pointer to a [`VideoEncodeInfoKHR`] structure.
    ///# Description
    ///## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`p_encode_info`] **must**  be a valid pointer to a valid [`VideoEncodeInfoKHR`] structure
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support encode
    ///   operations
    /// - This command  **must**  only be called outside of a render pass instance
    /// - [`command_buffer`] **must**  be a primary [`CommandBuffer`]
    ///
    ///## Host Synchronization
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    ///## Command Properties
    ///# Related
    /// - [`khr_video_encode_queue`]
    /// - [`CommandBuffer`]
    /// - [`VideoEncodeInfoKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdEncodeVideoKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_encode_video_khr<'lt>(
        self: &Unique<CommandBuffer>,
        p_encode_info: &VideoEncodeInfoKHR<'lt>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .khr_video_encode_queue()
            .and_then(|vtable| vtable.cmd_encode_video_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .khr_video_encode_queue()
            .and_then(|vtable| vtable.cmd_encode_video_khr())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), p_encode_info as *const VideoEncodeInfoKHR<'lt>);
        ()
    }
}
///The V-table of [`Device`] for functions from `VK_KHR_video_encode_queue`
pub struct DeviceKhrVideoEncodeQueueVTable {
    ///See [`FNCmdEncodeVideoKhr`] for more information.
    pub cmd_encode_video_khr: FNCmdEncodeVideoKhr,
}
impl DeviceKhrVideoEncodeQueueVTable {
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
            cmd_encode_video_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdEncodeVideoKHR").as_ptr()))
            },
        }
    }
    ///Gets [`Self::cmd_encode_video_khr`]. See [`FNCmdEncodeVideoKhr`] for more information.
    pub fn cmd_encode_video_khr(&self) -> FNCmdEncodeVideoKhr {
        self.cmd_encode_video_khr
    }
}
