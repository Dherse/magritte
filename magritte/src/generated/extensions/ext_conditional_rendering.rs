//![VK_EXT_conditional_rendering](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_conditional_rendering.html) - device extension
//!# Description
//!This extension allows the execution of one or more rendering commands to be
//!conditional on a value in buffer memory.
//!This may help an application reduce the latency by conditionally discarding
//!rendering commands without application intervention.
//!The conditional rendering commands are limited to draws, compute dispatches
//!and clearing attachments within a conditional rendering block.
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Vikram Kushwaha [vkushwaha](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_conditional_rendering]
//!   @vkushwaha%0A<<Here describe the issue or question you have about the
//!   VK_EXT_conditional_rendering extension>>)
//!# New functions & commands
//! - [`cmd_begin_conditional_rendering_ext`]
//! - [`cmd_end_conditional_rendering_ext`]
//!# New structures
//! - [`ConditionalRenderingBeginInfoEXT`]
//! - Extending [`CommandBufferInheritanceInfo`]:  -
//!   [`CommandBufferInheritanceConditionalRenderingInfoEXT`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceConditionalRenderingFeaturesEXT`]
//!# New enums
//! - [`ConditionalRenderingFlagBitsEXT`]
//!# New bitmasks
//! - [`ConditionalRenderingFlagsEXT`]
//!# New constants
//! - [`EXT_CONDITIONAL_RENDERING_EXTENSION_NAME`]
//! - [`EXT_CONDITIONAL_RENDERING_SPEC_VERSION`]
//! - Extending [`AccessFlagBits`]:  - `VK_ACCESS_CONDITIONAL_RENDERING_READ_BIT_EXT`
//! - Extending [`BufferUsageFlagBits`]:  - `VK_BUFFER_USAGE_CONDITIONAL_RENDERING_BIT_EXT`
//! - Extending [`PipelineStageFlagBits`]:  - `VK_PIPELINE_STAGE_CONDITIONAL_RENDERING_BIT_EXT`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_CONDITIONAL_RENDERING_BEGIN_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT`
//!# Known issues & F.A.Q
//!1) Should conditional rendering affect copy and blit commands? **RESOLVED** : Conditional
//! rendering should not affect copies and blits.2) Should secondary command buffers be allowed to
//! execute while conditional
//!rendering is active in the primary command buffer? **RESOLVED** : The rendering commands in
//! secondary command buffer will be
//!affected by an active conditional rendering in primary command buffer if the
//!`conditionalRenderingEnable` is set to [`TRUE`].
//!Conditional rendering  **must**  not be active in the primary command buffer if
//!`conditionalRenderingEnable` is [`FALSE`].
//!# Version History
//! - Revision 1, 2018-04-19 (Vikram Kushwaha)  - First Version
//! - Revision 2, 2018-05-21 (Vikram Kushwaha)  - Add new pipeline stage, access flags and limit
//!   conditional rendering to a subpass or entire render pass.
//!# Other info
//! * 2018-05-21
//! * No known IP claims.
//! * - Vikram Kushwaha, NVIDIA  - Daniel Rakos, AMD  - Jesse Hall, Google  - Jeff Bolz, NVIDIA  -
//!   Piers Daniell, NVIDIA  - Stuart Smith, Imagination Technologies
//!# Related
//! - [`CommandBufferInheritanceConditionalRenderingInfoEXT`]
//! - [`ConditionalRenderingBeginInfoEXT`]
//! - [`ConditionalRenderingFlagBitsEXT`]
//! - [`ConditionalRenderingFlagsEXT`]
//! - [`PhysicalDeviceConditionalRenderingFeaturesEXT`]
//! - [`cmd_begin_conditional_rendering_ext`]
//! - [`cmd_end_conditional_rendering_ext`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, Buffer, CommandBuffer, Device, DeviceSize, StructureType},
    AsRaw, Unique,
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
#[doc(alias = "VK_EXT_CONDITIONAL_RENDERING_SPEC_VERSION")]
pub const EXT_CONDITIONAL_RENDERING_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_CONDITIONAL_RENDERING_EXTENSION_NAME")]
pub const EXT_CONDITIONAL_RENDERING_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_conditional_rendering");
///[vkCmdBeginConditionalRenderingEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginConditionalRenderingEXT.html) - Define the beginning of a conditional rendering block
///# C Specifications
///To begin conditional rendering, call:
///```c
///// Provided by VK_EXT_conditional_rendering
///void vkCmdBeginConditionalRenderingEXT(
///    VkCommandBuffer                             commandBuffer,
///    const VkConditionalRenderingBeginInfoEXT*   pConditionalRenderingBegin);
///```
/// # Parameters
/// - [`command_buffer`] is the command buffer into which this command will be recorded.
/// - [`p_conditional_rendering_begin`] is a pointer to a [`ConditionalRenderingBeginInfoEXT`]
///   structure specifying parameters of conditional rendering.
/// # Description
/// ## Valid Usage
/// - Conditional rendering  **must**  not already be [active](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#active-conditional-rendering)
///
/// ## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`p_conditional_rendering_begin`] **must**  be a valid pointer to a valid
///   [`ConditionalRenderingBeginInfoEXT`] structure
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics, or
///   compute operations
///
/// ## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
/// ## Command Properties
/// # Related
/// - [`VK_EXT_conditional_rendering`]
/// - [`CommandBuffer`]
/// - [`ConditionalRenderingBeginInfoEXT`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdBeginConditionalRenderingEXT")]
pub type FNCmdBeginConditionalRenderingExt = Option<
    for<'lt> unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_conditional_rendering_begin: *const ConditionalRenderingBeginInfoEXT<'lt>,
    ),
>;
///[vkCmdEndConditionalRenderingEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndConditionalRenderingEXT.html) - Define the end of a conditional rendering block
///# C Specifications
///To end conditional rendering, call:
///```c
///// Provided by VK_EXT_conditional_rendering
///void vkCmdEndConditionalRenderingEXT(
///    VkCommandBuffer                             commandBuffer);
///```
/// # Parameters
/// - [`command_buffer`] is the command buffer into which this command will be recorded.
/// # Description
/// Once ended, conditional rendering becomes inactive.
/// ## Valid Usage
/// - Conditional rendering  **must**  be [active](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#active-conditional-rendering)
/// - If conditional rendering was made [active](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#active-conditional-rendering)
///   outside of a render pass instance, it  **must**  not be ended inside a render pass instance
/// - If conditional rendering was made [active](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#active-conditional-rendering)
///   within a subpass it  **must**  be ended in the same subpass
///
/// ## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics, or
///   compute operations
///
/// ## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
/// ## Command Properties
/// # Related
/// - [`VK_EXT_conditional_rendering`]
/// - [`CommandBuffer`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdEndConditionalRenderingEXT")]
pub type FNCmdEndConditionalRenderingExt = Option<unsafe extern "system" fn(command_buffer: CommandBuffer)>;
///[VkConditionalRenderingFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkConditionalRenderingFlagBitsEXT.html) - Specify the behavior of conditional rendering
///# C Specifications
///Bits which  **can**  be set in
///[`cmd_begin_conditional_rendering_ext`]`::flags`, specifying the
///behavior of conditional rendering, are:
///```c
///// Provided by VK_EXT_conditional_rendering
///typedef enum VkConditionalRenderingFlagBitsEXT {
///    VK_CONDITIONAL_RENDERING_INVERTED_BIT_EXT = 0x00000001,
///} VkConditionalRenderingFlagBitsEXT;
///```
/// # Description
/// - [`INVERTED`] specifies the condition used to determine whether to discard rendering commands
///   or not. That is, if the 32-bit predicate read from `buffer` memory at `offset` is zero, the
///   rendering commands are not discarded, and if non zero, then they are discarded.
/// # Related
/// - [`VK_EXT_conditional_rendering`]
/// - [`ConditionalRenderingFlagsEXT`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkConditionalRenderingFlagBitsEXT")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct ConditionalRenderingFlagBitsEXT(u32);
impl const Default for ConditionalRenderingFlagBitsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl ConditionalRenderingFlagBitsEXT {
    ///[`INVERTED`] specifies the condition
    ///used to determine whether to discard rendering commands or not.
    ///That is, if the 32-bit predicate read from `buffer` memory at
    ///`offset` is zero, the rendering commands are not discarded, and if
    ///non zero, then they are discarded.
    pub const INVERTED: Self = Self(1);
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
///[VkConditionalRenderingFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkConditionalRenderingFlagBitsEXT.html) - Specify the behavior of conditional rendering
///# C Specifications
///Bits which  **can**  be set in
///[`cmd_begin_conditional_rendering_ext`]`::flags`, specifying the
///behavior of conditional rendering, are:
///```c
///// Provided by VK_EXT_conditional_rendering
///typedef enum VkConditionalRenderingFlagBitsEXT {
///    VK_CONDITIONAL_RENDERING_INVERTED_BIT_EXT = 0x00000001,
///} VkConditionalRenderingFlagBitsEXT;
///```
/// # Description
/// - [`INVERTED`] specifies the condition used to determine whether to discard rendering commands
///   or not. That is, if the 32-bit predicate read from `buffer` memory at `offset` is zero, the
///   rendering commands are not discarded, and if non zero, then they are discarded.
/// # Related
/// - [`VK_EXT_conditional_rendering`]
/// - [`ConditionalRenderingFlagsEXT`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkConditionalRenderingFlagsEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct ConditionalRenderingFlagsEXT(u32);
impl const Default for ConditionalRenderingFlagsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl From<ConditionalRenderingFlagBitsEXT> for ConditionalRenderingFlagsEXT {
    fn from(from: ConditionalRenderingFlagBitsEXT) -> Self {
        unsafe { Self::from_bits_unchecked(from.bits()) }
    }
}
impl ConditionalRenderingFlagsEXT {
    ///[`INVERTED`] specifies the condition
    ///used to determine whether to discard rendering commands or not.
    ///That is, if the 32-bit predicate read from `buffer` memory at
    ///`offset` is zero, the rendering commands are not discarded, and if
    ///non zero, then they are discarded.
    pub const INVERTED: Self = Self(1);
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
            all |= Self::INVERTED;
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
impl const std::ops::BitOr for ConditionalRenderingFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for ConditionalRenderingFlagsEXT {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for ConditionalRenderingFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for ConditionalRenderingFlagsEXT {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for ConditionalRenderingFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for ConditionalRenderingFlagsEXT {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for ConditionalRenderingFlagsEXT {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for ConditionalRenderingFlagsEXT {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for ConditionalRenderingFlagsEXT {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<ConditionalRenderingFlagsEXT> for ConditionalRenderingFlagsEXT {
    fn extend<T: IntoIterator<Item = ConditionalRenderingFlagsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<ConditionalRenderingFlagBitsEXT> for ConditionalRenderingFlagsEXT {
    fn extend<T: IntoIterator<Item = ConditionalRenderingFlagBitsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<ConditionalRenderingFlagBitsEXT>>::from(i));
        }
    }
}
impl FromIterator<ConditionalRenderingFlagsEXT> for ConditionalRenderingFlagsEXT {
    fn from_iter<T: IntoIterator<Item = ConditionalRenderingFlagsEXT>>(iterator: T) -> ConditionalRenderingFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<ConditionalRenderingFlagsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<ConditionalRenderingFlagBitsEXT> for ConditionalRenderingFlagsEXT {
    fn from_iter<T: IntoIterator<Item = ConditionalRenderingFlagBitsEXT>>(iterator: T) -> ConditionalRenderingFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<ConditionalRenderingFlagBitsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for ConditionalRenderingFlagsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(ConditionalRenderingFlagsEXT);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == ConditionalRenderingFlagsEXT::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(ConditionalRenderingFlagsEXT::INVERTED) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(INVERTED))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(ConditionalRenderingFlagsEXT))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkConditionalRenderingBeginInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkConditionalRenderingBeginInfoEXT.html) - Structure specifying conditional rendering begin information
///# C Specifications
///The [`ConditionalRenderingBeginInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_conditional_rendering
///typedef struct VkConditionalRenderingBeginInfoEXT {
///    VkStructureType                   sType;
///    const void*                       pNext;
///    VkBuffer                          buffer;
///    VkDeviceSize                      offset;
///    VkConditionalRenderingFlagsEXT    flags;
///} VkConditionalRenderingBeginInfoEXT;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`buffer`] is a buffer containing the predicate for conditional rendering.
/// - [`offset`] is the byte offset into [`buffer`] where the predicate is located.
/// - [`flags`] is a bitmask of [`ConditionalRenderingFlagsEXT`] specifying the behavior of
///   conditional rendering.
/// # Description
/// If the 32-bit value at [`offset`] in [`buffer`] memory is zero, then the
/// rendering commands are discarded, otherwise they are executed as normal.
/// If the value of the predicate in buffer memory changes while conditional
/// rendering is active, the rendering commands  **may**  be discarded in an
/// implementation-dependent way.
/// Some implementations may latch the value of the predicate upon beginning
/// conditional rendering while others may read it before every rendering
/// command.
/// ## Valid Usage
/// - If [`buffer`] is non-sparse then it  **must**  be bound completely and contiguously to a
///   single [`DeviceMemory`] object
/// - [`buffer`] **must**  have been created with the
///   `VK_BUFFER_USAGE_CONDITIONAL_RENDERING_BIT_EXT` bit set
/// - [`offset`] **must**  be less than the size of [`buffer`] by at least 32 bits
/// - [`offset`] **must**  be a multiple of 4
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_CONDITIONAL_RENDERING_BEGIN_INFO_EXT`
/// - [`p_next`] **must**  be `NULL`
/// - [`buffer`] **must**  be a valid [`Buffer`] handle
/// - [`flags`] **must**  be a valid combination of [`ConditionalRenderingFlagBitsEXT`] values
/// # Related
/// - [`VK_EXT_conditional_rendering`]
/// - [`Buffer`]
/// - [`ConditionalRenderingFlagsEXT`]
/// - [`DeviceSize`]
/// - [`StructureType`]
/// - [`cmd_begin_conditional_rendering_ext`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkConditionalRenderingBeginInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ConditionalRenderingBeginInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`buffer`] is a buffer containing the predicate for conditional
    ///rendering.
    pub buffer: Buffer,
    ///[`offset`] is the byte offset into [`buffer`] where the predicate is
    ///located.
    pub offset: DeviceSize,
    ///[`flags`] is a bitmask of [`ConditionalRenderingFlagsEXT`]
    ///specifying the behavior of conditional rendering.
    pub flags: ConditionalRenderingFlagsEXT,
}
impl<'lt> Default for ConditionalRenderingBeginInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::CONDITIONAL_RENDERING_BEGIN_INFO_EXT,
            p_next: std::ptr::null(),
            buffer: Default::default(),
            offset: Default::default(),
            flags: Default::default(),
        }
    }
}
impl<'lt> ConditionalRenderingBeginInfoEXT<'lt> {
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
    ///Gets the value of [`Self::buffer`]
    pub fn buffer(&self) -> Buffer {
        self.buffer
    }
    ///Gets the value of [`Self::offset`]
    pub fn offset(&self) -> DeviceSize {
        self.offset
    }
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> ConditionalRenderingFlagsEXT {
        self.flags
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::buffer`]
    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }
    ///Gets a mutable reference to the value of [`Self::offset`]
    pub fn offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.offset
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut ConditionalRenderingFlagsEXT {
        &mut self.flags
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
    ///Sets the value of [`Self::buffer`]
    pub fn set_buffer(mut self, value: crate::vulkan1_0::Buffer) -> Self {
        self.buffer = value;
        self
    }
    ///Sets the value of [`Self::offset`]
    pub fn set_offset(mut self, value: crate::vulkan1_0::DeviceSize) -> Self {
        self.offset = value;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn set_flags(
        mut self,
        value: crate::extensions::ext_conditional_rendering::ConditionalRenderingFlagsEXT,
    ) -> Self {
        self.flags = value;
        self
    }
}
///[VkCommandBufferInheritanceConditionalRenderingInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferInheritanceConditionalRenderingInfoEXT.html) - Structure specifying command buffer inheritance information
///# C Specifications
///If the [`p_next`] chain of [`CommandBufferInheritanceInfo`] includes a
///[`CommandBufferInheritanceConditionalRenderingInfoEXT`] structure, then
///that structure controls whether a command buffer  **can**  be executed while
///conditional rendering is [active](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#active-conditional-rendering) in the
///primary command buffer.The [`CommandBufferInheritanceConditionalRenderingInfoEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_conditional_rendering
///typedef struct VkCommandBufferInheritanceConditionalRenderingInfoEXT {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkBool32           conditionalRenderingEnable;
///} VkCommandBufferInheritanceConditionalRenderingInfoEXT;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`conditional_rendering_enable`] specifies whether the command buffer  **can**  be executed
///   while conditional rendering is active in the primary command buffer. If this is [`TRUE`], then
///   this command buffer  **can**  be executed whether the primary command buffer has active
///   conditional rendering or not. If this is [`FALSE`], then the primary command buffer  **must**
///   not have conditional rendering active.
/// # Description
/// If this structure is not present, the behavior is as if
/// [`conditional_rendering_enable`] is [`FALSE`].
/// ## Valid Usage
/// - If the [inherited conditional rendering](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-inheritedConditionalRendering)
///   feature is not enabled, [`conditional_rendering_enable`] **must**  be [`FALSE`]
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT`
/// # Related
/// - [`VK_EXT_conditional_rendering`]
/// - [`Bool32`]
/// - [`StructureType`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkCommandBufferInheritanceConditionalRenderingInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct CommandBufferInheritanceConditionalRenderingInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`conditional_rendering_enable`] specifies whether the command buffer
    /// **can**  be executed while conditional rendering is active in the primary
    ///command buffer.
    ///If this is [`TRUE`], then this command buffer  **can**  be executed
    ///whether the primary command buffer has active conditional rendering or
    ///not.
    ///If this is [`FALSE`], then the primary command buffer  **must**  not
    ///have conditional rendering active.
    pub conditional_rendering_enable: Bool32,
}
impl<'lt> Default for CommandBufferInheritanceConditionalRenderingInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT,
            p_next: std::ptr::null(),
            conditional_rendering_enable: 0,
        }
    }
}
impl<'lt> CommandBufferInheritanceConditionalRenderingInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::conditional_rendering_enable`]
    pub fn conditional_rendering_enable_raw(&self) -> Bool32 {
        self.conditional_rendering_enable
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::conditional_rendering_enable`]
    pub fn set_conditional_rendering_enable_raw(mut self, value: Bool32) -> Self {
        self.conditional_rendering_enable = value;
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
    ///Gets the value of [`Self::conditional_rendering_enable`]
    pub fn conditional_rendering_enable(&self) -> bool {
        unsafe { std::mem::transmute(self.conditional_rendering_enable as u8) }
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::conditional_rendering_enable`]
    pub fn conditional_rendering_enable_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.conditional_rendering_enable as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.conditional_rendering_enable as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
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
    ///Sets the value of [`Self::conditional_rendering_enable`]
    pub fn set_conditional_rendering_enable(mut self, value: bool) -> Self {
        self.conditional_rendering_enable = value as u8 as u32;
        self
    }
}
///[VkPhysicalDeviceConditionalRenderingFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceConditionalRenderingFeaturesEXT.html) - Structure describing if a secondary command buffer can be executed if conditional rendering is active in the primary command buffer
///# C Specifications
///The [`PhysicalDeviceConditionalRenderingFeaturesEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_conditional_rendering
///typedef struct VkPhysicalDeviceConditionalRenderingFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           conditionalRendering;
///    VkBool32           inheritedConditionalRendering;
///} VkPhysicalDeviceConditionalRenderingFeaturesEXT;
///```
/// # Members
/// This structure describes the following features:
/// # Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`conditional_rendering`] specifies whether conditional rendering is supported.
/// - [`inherited_conditional_rendering`] specifies whether a secondary command buffer  **can**  be
///   executed while conditional rendering is active in the primary command buffer.
/// If the [`PhysicalDeviceConditionalRenderingFeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
/// [`PhysicalDeviceFeatures2`] structure passed to
/// [`get_physical_device_features2`], it is filled in to indicate whether each
/// corresponding feature is supported.
/// [`PhysicalDeviceConditionalRenderingFeaturesEXT`] **can**  also be used in the [`p_next`] chain
/// of
/// [`DeviceCreateInfo`] to selectively enable these features.
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT`
/// # Related
/// - [`VK_EXT_conditional_rendering`]
/// - [`Bool32`]
/// - [`StructureType`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceConditionalRenderingFeaturesEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceConditionalRenderingFeaturesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`conditional_rendering`] specifies
    ///whether conditional rendering is supported.
    pub conditional_rendering: Bool32,
    ///[`inherited_conditional_rendering`] specifies whether a secondary
    ///command buffer  **can**  be executed while conditional rendering is active in
    ///the primary command buffer.
    pub inherited_conditional_rendering: Bool32,
}
impl<'lt> Default for PhysicalDeviceConditionalRenderingFeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            conditional_rendering: 0,
            inherited_conditional_rendering: 0,
        }
    }
}
impl<'lt> PhysicalDeviceConditionalRenderingFeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::conditional_rendering`]
    pub fn conditional_rendering_raw(&self) -> Bool32 {
        self.conditional_rendering
    }
    ///Gets the raw value of [`Self::inherited_conditional_rendering`]
    pub fn inherited_conditional_rendering_raw(&self) -> Bool32 {
        self.inherited_conditional_rendering
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::conditional_rendering`]
    pub fn set_conditional_rendering_raw(mut self, value: Bool32) -> Self {
        self.conditional_rendering = value;
        self
    }
    ///Sets the raw value of [`Self::inherited_conditional_rendering`]
    pub fn set_inherited_conditional_rendering_raw(mut self, value: Bool32) -> Self {
        self.inherited_conditional_rendering = value;
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
    ///Gets the value of [`Self::conditional_rendering`]
    pub fn conditional_rendering(&self) -> bool {
        unsafe { std::mem::transmute(self.conditional_rendering as u8) }
    }
    ///Gets the value of [`Self::inherited_conditional_rendering`]
    pub fn inherited_conditional_rendering(&self) -> bool {
        unsafe { std::mem::transmute(self.inherited_conditional_rendering as u8) }
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
    ///Gets a mutable reference to the value of [`Self::conditional_rendering`]
    pub fn conditional_rendering_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.conditional_rendering as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.conditional_rendering as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::inherited_conditional_rendering`]
    pub fn inherited_conditional_rendering_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.inherited_conditional_rendering as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.inherited_conditional_rendering as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
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
    ///Sets the value of [`Self::conditional_rendering`]
    pub fn set_conditional_rendering(mut self, value: bool) -> Self {
        self.conditional_rendering = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::inherited_conditional_rendering`]
    pub fn set_inherited_conditional_rendering(mut self, value: bool) -> Self {
        self.inherited_conditional_rendering = value as u8 as u32;
        self
    }
}
impl CommandBuffer {
    ///[vkCmdBeginConditionalRenderingEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginConditionalRenderingEXT.html) - Define the beginning of a conditional rendering block
    ///# C Specifications
    ///To begin conditional rendering, call:
    ///```c
    ///// Provided by VK_EXT_conditional_rendering
    ///void vkCmdBeginConditionalRenderingEXT(
    ///    VkCommandBuffer                             commandBuffer,
    ///    const VkConditionalRenderingBeginInfoEXT*   pConditionalRenderingBegin);
    ///```
    /// # Parameters
    /// - [`command_buffer`] is the command buffer into which this command will be recorded.
    /// - [`p_conditional_rendering_begin`] is a pointer to a [`ConditionalRenderingBeginInfoEXT`]
    ///   structure specifying parameters of conditional rendering.
    /// # Description
    /// ## Valid Usage
    /// - Conditional rendering  **must**  not already be [active](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#active-conditional-rendering)
    ///
    /// ## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`p_conditional_rendering_begin`] **must**  be a valid pointer to a valid
    ///   [`ConditionalRenderingBeginInfoEXT`] structure
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support
    ///   graphics, or compute operations
    ///
    /// ## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    /// ## Command Properties
    /// # Related
    /// - [`VK_EXT_conditional_rendering`]
    /// - [`CommandBuffer`]
    /// - [`ConditionalRenderingBeginInfoEXT`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdBeginConditionalRenderingEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_begin_conditional_rendering_ext<'lt>(
        self: &Unique<CommandBuffer>,
        p_conditional_rendering_begin: &ConditionalRenderingBeginInfoEXT<'lt>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .ext_conditional_rendering()
            .and_then(|vtable| vtable.cmd_begin_conditional_rendering_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .ext_conditional_rendering()
            .and_then(|vtable| vtable.cmd_begin_conditional_rendering_ext())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            p_conditional_rendering_begin as *const ConditionalRenderingBeginInfoEXT<'lt>,
        );
        ()
    }
}
impl CommandBuffer {
    ///[vkCmdEndConditionalRenderingEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndConditionalRenderingEXT.html) - Define the end of a conditional rendering block
    ///# C Specifications
    ///To end conditional rendering, call:
    ///```c
    ///// Provided by VK_EXT_conditional_rendering
    ///void vkCmdEndConditionalRenderingEXT(
    ///    VkCommandBuffer                             commandBuffer);
    ///```
    /// # Parameters
    /// - [`command_buffer`] is the command buffer into which this command will be recorded.
    /// # Description
    /// Once ended, conditional rendering becomes inactive.
    /// ## Valid Usage
    /// - Conditional rendering  **must**  be [active](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#active-conditional-rendering)
    /// - If conditional rendering was made [active](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#active-conditional-rendering)
    ///   outside of a render pass instance, it  **must**  not be ended inside a render pass
    ///   instance
    /// - If conditional rendering was made [active](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#active-conditional-rendering)
    ///   within a subpass it  **must**  be ended in the same subpass
    ///
    /// ## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support
    ///   graphics, or compute operations
    ///
    /// ## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    /// ## Command Properties
    /// # Related
    /// - [`VK_EXT_conditional_rendering`]
    /// - [`CommandBuffer`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdEndConditionalRenderingEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_end_conditional_rendering_ext(self: &Unique<CommandBuffer>) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .ext_conditional_rendering()
            .and_then(|vtable| vtable.cmd_end_conditional_rendering_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .ext_conditional_rendering()
            .and_then(|vtable| vtable.cmd_end_conditional_rendering_ext())
            .unwrap_unchecked();
        let _return = _function(self.as_raw());
        ()
    }
}
///The V-table of [`Device`] for functions from `VK_EXT_conditional_rendering`
pub struct DeviceExtConditionalRenderingVTable {
    ///See [`FNCmdBeginConditionalRenderingExt`] for more information.
    pub cmd_begin_conditional_rendering_ext: FNCmdBeginConditionalRenderingExt,
    ///See [`FNCmdEndConditionalRenderingExt`] for more information.
    pub cmd_end_conditional_rendering_ext: FNCmdEndConditionalRenderingExt,
}
impl DeviceExtConditionalRenderingVTable {
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
            cmd_begin_conditional_rendering_ext: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCmdBeginConditionalRenderingEXT").as_ptr(),
                ))
            },
            cmd_end_conditional_rendering_ext: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCmdEndConditionalRenderingEXT").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::cmd_begin_conditional_rendering_ext`]. See
    /// [`FNCmdBeginConditionalRenderingExt`] for more information.
    pub fn cmd_begin_conditional_rendering_ext(&self) -> FNCmdBeginConditionalRenderingExt {
        self.cmd_begin_conditional_rendering_ext
    }
    ///Gets [`Self::cmd_end_conditional_rendering_ext`]. See [`FNCmdEndConditionalRenderingExt`]
    /// for more information.
    pub fn cmd_end_conditional_rendering_ext(&self) -> FNCmdEndConditionalRenderingExt {
        self.cmd_end_conditional_rendering_ext
    }
}
