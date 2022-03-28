//![VK_EXT_debug_report](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_debug_report.html) - instance extension
//!# Description
//!Due to the nature of the Vulkan interface, there is very little error
//!information available to the developer and application.
//!By enabling optional validation layers and using the [`VK_EXT_debug_report`]
//!extension, developers  **can**  obtain much more detailed feedback on the
//!application’s use of Vulkan.
//!This extension defines a way for layers and the implementation to call back
//!to the application for events of interest to the application.
//!# Revision
//!10
//!# Dependencies
//! - *Deprecated* by `[`VK_EXT_debug_utils`]` extension
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Courtney Goeltzenleuchter [courtney-g](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_debug_report]
//!   @courtney-g%0A<<Here describe the issue or question you have about the VK_EXT_debug_report
//!   extension>>)
//!# New handles
//! - [`DebugReportCallbackEXT`]
//!# New functions & commands
//! - [`CreateDebugReportCallbackEXT`]
//! - [`DebugReportMessageEXT`]
//! - [`DestroyDebugReportCallbackEXT`]
//!# New structures
//! - Extending [`InstanceCreateInfo`]:  - [`DebugReportCallbackCreateInfoEXT`]
//!# New enums
//! - [`DebugReportFlagBitsEXT`]
//! - [`DebugReportObjectTypeEXT`]
//!# New bitmasks
//! - [`DebugReportFlagsEXT`]
//!# New constants
//! - [`EXT_DEBUG_REPORT_EXTENSION_NAME`]
//! - [`EXT_DEBUG_REPORT_SPEC_VERSION`]
//! - Extending [`ObjectType`]:  - `VK_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT`
//! - Extending [`VulkanResultCodes`]:  - `VK_ERROR_VALIDATION_FAILED_EXT`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_DEBUG_REPORT_CREATE_INFO_EXT`
//!If [Version 1.1]() is supported:
//! - Extending [`DebugReportObjectTypeEXT`]:  -
//!   `VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_EXT`  -
//!   `VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_EXT`
//!# Known issues & F.A.Q
//!1) What is the hierarchy / seriousness of the message flags? E.g.
//!`ERROR` > `WARN` > `PERF_WARN` …​ **RESOLVED** : There is no specific hierarchy.
//!Each bit is independent and should be checked via bitwise AND.
//!For example:
//!```c
//!    if (localFlags & VK_DEBUG_REPORT_ERROR_BIT_EXT) {
//!        process error message
//!    }
//!    if (localFlags & VK_DEBUG_REPORT_DEBUG_BIT_EXT) {
//!        process debug message
//!    }
//!```
//!The validation layers do use them in a hierarchical way (`ERROR` >
//!`WARN` > `PERF`, `WARN` > `DEBUG` > `INFO`) and they (at
//!least at the time of this writing) only set one bit at a time.
//!But it is not a requirement of this extension.It is possible that a layer may intercept and
//! change, or augment the flags
//!with extension values the application’s debug report handler may not be
//!familiar with, so it is important to treat each flag independently.2) Should there be a VU
//! requiring
//![`DebugReportCallbackCreateInfoEXT::flags`] to be non-zero? **RESOLVED** : It may not be very
//! useful, but we do not need VU statement
//!requiring the [`DebugReportCallbackCreateInfoEXT`]`::msgFlags` at
//!create-time to be non-zero.
//!One can imagine that apps may prefer it as it allows them to set the mask as
//!desired - including nothing - at runtime without having to check.3) What is the difference
//! between `VK_DEBUG_REPORT_DEBUG_BIT_EXT` and
//!`VK_DEBUG_REPORT_INFORMATION_BIT_EXT`? **RESOLVED** : `VK_DEBUG_REPORT_DEBUG_BIT_EXT` specifies
//! information that
//!could be useful debugging the Vulkan implementation itself.4) How do you compare handles
//! returned by the debug_report callback to the
//!application’s handles? **RESOLVED** : Due to the different nature of dispatchable and
//! nondispatchable
//!handles there is no generic way (that we know of) that works for common
//!compilers with 32bit, 64bit, C and C++.
//!We recommend applications use the same cast that the validation layers use:+
//!```c
//!reinterpret_cast<uint64_t &>(dispatchableHandle)
//!(uint64_t)(nondispatchableHandle)
//!```
//!+
//!This does require that the app treat dispatchable and nondispatchable
//!handles differently.
//!# Version History
//! - Revision 1, 2015-05-20 (Courtney Goetzenleuchter)  - Initial draft, based on LunarG KHR spec,
//!   other KHR specs
//! - Revision 2, 2016-02-16 (Courtney Goetzenleuchter)  - Update usage, documentation
//! - Revision 3, 2016-06-14 (Courtney Goetzenleuchter)  - Update VK_EXT_DEBUG_REPORT_SPEC_VERSION
//!   to indicate added support for vkCreateInstance and vkDestroyInstance
//! - Revision 4, 2016-12-08 (Mark Lobodzinski)  - Added Display_KHR, DisplayModeKHR extension
//!   objects  - Added ObjectTable_NVX, IndirectCommandsLayout_NVX extension objects  - Bumped spec
//!   revision  - Retroactively added version history
//! - Revision 5, 2017-01-31 (Baldur Karlsson)  - Moved definition of [`DebugReportObjectTypeEXT`]
//!   from debug marker chapter
//! - Revision 6, 2017-01-31 (Baldur Karlsson)  - Added
//!   VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR_EXT
//! - Revision 7, 2017-04-20 (Courtney Goeltzenleuchter)  - Clarify wording and address questions
//!   from developers.
//! - Revision 8, 2017-04-21 (Courtney Goeltzenleuchter)  - Remove unused enum VkDebugReportErrorEXT
//! - Revision 9, 2017-09-12 (Tobias Hector)  - Added interactions with Vulkan 1.1
//! - Revision 10, 2020-12-14 (Courtney Goetzenleuchter)  - Add issue 4 discussing matching handles
//!   returned by the extension, based on suggestion in public issue 368.
//!# Other info
//! * 2020-12-14
//! * No known IP claims.
//! * - Courtney Goeltzenleuchter, LunarG  - Dan Ginsburg, Valve  - Jon Ashburn, LunarG  - Mark
//!   Lobodzinski, LunarG
//!# Related
//! - [`PFNDebugReportCallbackEXT`]
//! - [`DebugReportCallbackCreateInfoEXT`]
//! - [`DebugReportCallbackEXT`]
//! - [`DebugReportFlagBitsEXT`]
//! - [`DebugReportFlagsEXT`]
//! - [`DebugReportObjectTypeEXT`]
//! - [`CreateDebugReportCallbackEXT`]
//! - [`DebugReportMessageEXT`]
//! - [`DestroyDebugReportCallbackEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    extensions::ext_debug_marker::DebugReportObjectTypeEXT,
    vulkan1_0::{BaseInStructure, Bool32, StructureType},
};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    ffi::{c_void, CStr},
    iter::{Extend, FromIterator, IntoIterator},
    marker::PhantomData,
    os::raw::c_char,
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DEBUG_REPORT_SPEC_VERSION")]
pub const EXT_DEBUG_REPORT_SPEC_VERSION: u32 = 10;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DEBUG_REPORT_EXTENSION_NAME")]
pub const EXT_DEBUG_REPORT_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_debug_report");
///[PFN_vkDebugReportCallbackEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/PFN_vkDebugReportCallbackEXT.html) - Application-defined debug report callback function
///# C Specifications
///The prototype for the
///[`DebugReportCallbackCreateInfoEXT::pfn_callback`] function
///implemented by the application is:
///```c
///// Provided by VK_EXT_debug_report
///typedef VkBool32 (VKAPI_PTR *PFN_vkDebugReportCallbackEXT)(
///    VkDebugReportFlagsEXT                       flags,
///    VkDebugReportObjectTypeEXT                  objectType,
///    uint64_t                                    object,
///    size_t                                      location,
///    int32_t                                     messageCode,
///    const char*                                 pLayerPrefix,
///    const char*                                 pMessage,
///    void*                                       pUserData);
///```
///# Parameters
/// - [`flags`] specifies the [`DebugReportFlagBitsEXT`] that triggered this callback.
/// - [`object_type`] is a [`DebugReportObjectTypeEXT`] value specifying the type of object being
///   used or created at the time the event was triggered.
/// - [`object`] is the object where the issue was detected. If [`object_type`] is
///   `VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT`, [`object`] is undefined.
/// - [`location`] is a component (layer, driver, loader) defined value specifying the *location* of
///   the trigger. This is an  **optional**  value.
/// - [`message_code`] is a layer-defined value indicating what test triggered this callback.
/// - [`p_layer_prefix`] is a null-terminated string that is an abbreviation of the name of the
///   component making the callback. [`p_layer_prefix`] is only valid for the duration of the
///   callback.
/// - [`p_message`] is a null-terminated string detailing the trigger conditions. [`p_message`] is
///   only valid for the duration of the callback.
/// - [`p_user_data`] is the user data given when the [`DebugReportCallbackEXT`] was created.
///# Description
///The callback  **must**  not call [`DestroyDebugReportCallbackEXT`].The callback returns a
/// [`Bool32`], which is interpreted in a
///layer-specified manner.
///The application  **should**  always return [`FALSE`].
///The [`TRUE`] value is reserved for use in layer development.[`object`] **must**  be a Vulkan
/// object or [`crate::utils::Handle::null`].
///If [`object_type`] is not `VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT` and
///[`object`] is not [`crate::utils::Handle::null`], [`object`] **must**  be a Vulkan
///object of the corresponding type associated with [`object_type`] as defined
///in [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debug-report-object-types](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debug-report-object-types).
///# Related
/// - [`VK_EXT_debug_report`]
/// - [`DebugReportCallbackCreateInfoEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "PFN_vkDebugReportCallbackEXT")]
pub type PFNDebugReportCallbackEXT = Option<
    unsafe extern "system" fn(
        flags: DebugReportFlagsEXT,
        object_type: DebugReportObjectTypeEXT,
        object: u64,
        location: usize,
        message_code: i32,
        p_layer_prefix: *const c_char,
        p_message: *const c_char,
        p_user_data: *mut c_void,
    ) -> Bool32,
>;
///[VkDebugReportFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugReportFlagBitsEXT.html) - Bitmask specifying events which cause a debug report callback
///# C Specifications
///Bits which  **can**  be set in
///[`DebugReportCallbackCreateInfoEXT::flags`], specifying events
///which cause a debug report, are:
///```c
///// Provided by VK_EXT_debug_report
///typedef enum VkDebugReportFlagBitsEXT {
///    VK_DEBUG_REPORT_INFORMATION_BIT_EXT = 0x00000001,
///    VK_DEBUG_REPORT_WARNING_BIT_EXT = 0x00000002,
///    VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT = 0x00000004,
///    VK_DEBUG_REPORT_ERROR_BIT_EXT = 0x00000008,
///    VK_DEBUG_REPORT_DEBUG_BIT_EXT = 0x00000010,
///} VkDebugReportFlagBitsEXT;
///```
///# Description
/// - [`DebugReportErrorExt`] specifies that the application has violated a valid usage condition of
///   the specification.
/// - [`DebugReportWarningExt`] specifies use of Vulkan that  **may**  expose an app bug. Such cases
///   may not be immediately harmful, such as a fragment shader outputting to a location with no
///   attachment. Other cases  **may**  point to behavior that is almost certainly bad when
///   unintended such as using an image whose memory has not been filled. In general if you see a
///   warning but you know that the behavior is intended/desired, then simply ignore the warning.
/// - [`DebugReportPerformanceWarningExt`] specifies a potentially non-optimal use of Vulkan, e.g.
///   using [`CmdClearColorImage`] when setting [`AttachmentDescription::load_op`] to
///   `VK_ATTACHMENT_LOAD_OP_CLEAR` would have worked.
/// - [`DebugReportInformationExt`] specifies an informational message such as resource details that
///   may be handy when debugging an application.
/// - [`DebugReportDebugExt`] specifies diagnostic information from the implementation and layers.
///# Related
/// - [`VK_EXT_debug_report`]
/// - [`DebugReportFlagsEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDebugReportFlagBitsEXT")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(u32)]
pub enum DebugReportFlagBitsEXT {
    #[doc(hidden)]
    Empty = 0,
    ///[`DebugReportInformationExt`] specifies an informational
    ///message such as resource details that may be handy when debugging an
    ///application.
    DebugReportInformationExt = 1,
    ///[`DebugReportWarningExt`] specifies use of Vulkan that  **may**
    ///expose an app bug.
    ///Such cases may not be immediately harmful, such as a fragment shader
    ///outputting to a location with no attachment.
    ///Other cases  **may**  point to behavior that is almost certainly bad when
    ///unintended such as using an image whose memory has not been filled.
    ///In general if you see a warning but you know that the behavior is
    ///intended/desired, then simply ignore the warning.
    DebugReportWarningExt = 2,
    ///[`DebugReportPerformanceWarningExt`] specifies a
    ///potentially non-optimal use of Vulkan, e.g. using
    ///[`CmdClearColorImage`] when setting
    ///[`AttachmentDescription`]::`loadOp` to
    ///`VK_ATTACHMENT_LOAD_OP_CLEAR` would have worked.
    DebugReportPerformanceWarningExt = 4,
    ///[`DebugReportErrorExt`] specifies that the application has
    ///violated a valid usage condition of the specification.
    DebugReportErrorExt = 8,
    ///[`DebugReportDebugExt`] specifies diagnostic information
    ///from the implementation and layers.
    DebugReportDebugExt = 16,
}
impl const Default for DebugReportFlagBitsEXT {
    fn default() -> Self {
        Self::Empty
    }
}
impl DebugReportFlagBitsEXT {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> u32 {
        *self as u32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: u32) -> u32 {
        std::mem::transmute(bits)
    }
}
///[VkDebugReportFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugReportFlagBitsEXT.html) - Bitmask specifying events which cause a debug report callback
///# C Specifications
///Bits which  **can**  be set in
///[`DebugReportCallbackCreateInfoEXT::flags`], specifying events
///which cause a debug report, are:
///```c
///// Provided by VK_EXT_debug_report
///typedef enum VkDebugReportFlagBitsEXT {
///    VK_DEBUG_REPORT_INFORMATION_BIT_EXT = 0x00000001,
///    VK_DEBUG_REPORT_WARNING_BIT_EXT = 0x00000002,
///    VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT = 0x00000004,
///    VK_DEBUG_REPORT_ERROR_BIT_EXT = 0x00000008,
///    VK_DEBUG_REPORT_DEBUG_BIT_EXT = 0x00000010,
///} VkDebugReportFlagBitsEXT;
///```
///# Description
/// - [`DebugReportErrorExt`] specifies that the application has violated a valid usage condition of
///   the specification.
/// - [`DebugReportWarningExt`] specifies use of Vulkan that  **may**  expose an app bug. Such cases
///   may not be immediately harmful, such as a fragment shader outputting to a location with no
///   attachment. Other cases  **may**  point to behavior that is almost certainly bad when
///   unintended such as using an image whose memory has not been filled. In general if you see a
///   warning but you know that the behavior is intended/desired, then simply ignore the warning.
/// - [`DebugReportPerformanceWarningExt`] specifies a potentially non-optimal use of Vulkan, e.g.
///   using [`CmdClearColorImage`] when setting [`AttachmentDescription::load_op`] to
///   `VK_ATTACHMENT_LOAD_OP_CLEAR` would have worked.
/// - [`DebugReportInformationExt`] specifies an informational message such as resource details that
///   may be handy when debugging an application.
/// - [`DebugReportDebugExt`] specifies diagnostic information from the implementation and layers.
///# Related
/// - [`VK_EXT_debug_report`]
/// - [`DebugReportFlagsEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDebugReportFlagsEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct DebugReportFlagsEXT(u32);
impl const Default for DebugReportFlagsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl From<DebugReportFlagBitsEXT> for DebugReportFlagsEXT {
    fn from(from: DebugReportFlagBitsEXT) -> Self {
        unsafe { Self::from_bits_unchecked(from as u32) }
    }
}
impl DebugReportFlagsEXT {
    ///[`DebugReportInformationExt`] specifies an informational
    ///message such as resource details that may be handy when debugging an
    ///application.
    pub const DEBUG_REPORT_INFORMATION_EXT: Self = Self(1);
    ///[`DebugReportWarningExt`] specifies use of Vulkan that  **may**
    ///expose an app bug.
    ///Such cases may not be immediately harmful, such as a fragment shader
    ///outputting to a location with no attachment.
    ///Other cases  **may**  point to behavior that is almost certainly bad when
    ///unintended such as using an image whose memory has not been filled.
    ///In general if you see a warning but you know that the behavior is
    ///intended/desired, then simply ignore the warning.
    pub const DEBUG_REPORT_WARNING_EXT: Self = Self(2);
    ///[`DebugReportPerformanceWarningExt`] specifies a
    ///potentially non-optimal use of Vulkan, e.g. using
    ///[`CmdClearColorImage`] when setting
    ///[`AttachmentDescription`]::`loadOp` to
    ///`VK_ATTACHMENT_LOAD_OP_CLEAR` would have worked.
    pub const DEBUG_REPORT_PERFORMANCE_WARNING_EXT: Self = Self(4);
    ///[`DebugReportErrorExt`] specifies that the application has
    ///violated a valid usage condition of the specification.
    pub const DEBUG_REPORT_ERROR_EXT: Self = Self(8);
    ///[`DebugReportDebugExt`] specifies diagnostic information
    ///from the implementation and layers.
    pub const DEBUG_REPORT_DEBUG_EXT: Self = Self(16);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    pub const fn all() -> Self {
        Self::empty()
            | Self::DEBUG_REPORT_INFORMATION_EXT
            | Self::DEBUG_REPORT_WARNING_EXT
            | Self::DEBUG_REPORT_PERFORMANCE_WARNING_EXT
            | Self::DEBUG_REPORT_ERROR_EXT
            | Self::DEBUG_REPORT_DEBUG_EXT
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
impl const std::ops::BitOr for DebugReportFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for DebugReportFlagsEXT {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for DebugReportFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for DebugReportFlagsEXT {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for DebugReportFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for DebugReportFlagsEXT {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for DebugReportFlagsEXT {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for DebugReportFlagsEXT {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for DebugReportFlagsEXT {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<DebugReportFlagsEXT> for DebugReportFlagsEXT {
    fn extend<T: IntoIterator<Item = DebugReportFlagsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<DebugReportFlagBitsEXT> for DebugReportFlagsEXT {
    fn extend<T: IntoIterator<Item = DebugReportFlagBitsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<DebugReportFlagBitsEXT>>::from(i));
        }
    }
}
impl FromIterator<DebugReportFlagsEXT> for DebugReportFlagsEXT {
    fn from_iter<T: IntoIterator<Item = DebugReportFlagsEXT>>(iterator: T) -> DebugReportFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<DebugReportFlagsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<DebugReportFlagBitsEXT> for DebugReportFlagsEXT {
    fn from_iter<T: IntoIterator<Item = DebugReportFlagBitsEXT>>(iterator: T) -> DebugReportFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<DebugReportFlagBitsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for DebugReportFlagsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(DebugReportFlagsEXT);
        impl std::fmt::Debug for Flags {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == DebugReportFlagsEXT::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(DebugReportFlagsEXT::DEBUG_REPORT_INFORMATION_EXT) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(DEBUG_REPORT_INFORMATION_EXT))?;
                    }
                    if self.0.contains(DebugReportFlagsEXT::DEBUG_REPORT_WARNING_EXT) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(DEBUG_REPORT_WARNING_EXT))?;
                    }
                    if self
                        .0
                        .contains(DebugReportFlagsEXT::DEBUG_REPORT_PERFORMANCE_WARNING_EXT)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(DEBUG_REPORT_PERFORMANCE_WARNING_EXT))?;
                    }
                    if self.0.contains(DebugReportFlagsEXT::DEBUG_REPORT_ERROR_EXT) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(DEBUG_REPORT_ERROR_EXT))?;
                    }
                    if self.0.contains(DebugReportFlagsEXT::DEBUG_REPORT_DEBUG_EXT) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(DEBUG_REPORT_DEBUG_EXT))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(DebugReportFlagsEXT))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkDebugReportCallbackCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugReportCallbackCreateInfoEXT.html) - Structure specifying parameters of a newly created debug report callback
///# C Specifications
///The definition of [`DebugReportCallbackCreateInfoEXT`] is:
///```c
///// Provided by VK_EXT_debug_report
///typedef struct VkDebugReportCallbackCreateInfoEXT {
///    VkStructureType                 sType;
///    const void*                     pNext;
///    VkDebugReportFlagsEXT           flags;
///    PFN_vkDebugReportCallbackEXT    pfnCallback;
///    void*                           pUserData;
///} VkDebugReportCallbackCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is a bitmask of [`DebugReportFlagBitsEXT`] specifying which event(s) will cause this
///   callback to be called.
/// - [`pfn_callback`] is the application callback function to call.
/// - [`user_data`] is user data to be passed to the callback.
///# Description
///For each [`DebugReportCallbackEXT`] that is created the
///[`DebugReportCallbackCreateInfoEXT`]::[`flags`] determine when that
///[`DebugReportCallbackCreateInfoEXT`]::[`pfn_callback`] is called.
///When an event happens, the implementation will do a bitwise AND of the
///event’s [`DebugReportFlagBitsEXT`] flags to each
///[`DebugReportCallbackEXT`] object’s flags.
///For each non-zero result the corresponding callback will be called.
///The callback will come directly from the component that detected the event,
///unless some other layer intercepts the calls for its own purposes (filter
///them in a different way, log to a system error log, etc.).An application  **may**  receive
/// multiple callbacks if multiple
///[`DebugReportCallbackEXT`] objects were created.
///A callback will always be executed in the same thread as the originating
///Vulkan call.A callback may be called from multiple threads simultaneously (if the
///application is making Vulkan calls from multiple threads).
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT`
/// - [`flags`] **must**  be a valid combination of [`DebugReportFlagBitsEXT`] values
/// - [`pfn_callback`] **must**  be a valid [`PFNDebugReportCallbackEXT`] value
///# Related
/// - [`PFNDebugReportCallbackEXT`]
/// - [`VK_EXT_debug_report`]
/// - [`DebugReportFlagsEXT`]
/// - [`StructureType`]
/// - [`CreateDebugReportCallbackEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDebugReportCallbackCreateInfoEXT")]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DebugReportCallbackCreateInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`DebugReportFlagBitsEXT`] specifying
    ///which event(s) will cause this callback to be called.
    pub flags: DebugReportFlagsEXT,
    ///[`pfn_callback`] is the application callback function to call.
    pub pfn_callback: PFNDebugReportCallbackEXT,
    ///[`user_data`] is user data to be passed to the callback.
    pub user_data: *mut c_void,
}
impl<'lt> Default for DebugReportCallbackCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
            pfn_callback: Default::default(),
            user_data: std::ptr::null_mut(),
        }
    }
}
impl<'lt> DebugReportCallbackCreateInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::user_data`]
    pub fn user_data_raw(&self) -> &*mut c_void {
        &self.user_data
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::user_data`]
    pub fn set_user_data_raw(&mut self, value: *mut c_void) -> &mut Self {
        self.user_data = value;
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
    pub fn flags(&self) -> DebugReportFlagsEXT {
        self.flags
    }
    ///Gets the value of [`Self::pfn_callback`]
    pub fn pfn_callback(&self) -> &PFNDebugReportCallbackEXT {
        &self.pfn_callback
    }
    ///Gets the value of [`Self::user_data`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn user_data(&self) -> &c_void {
        &*self.user_data
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut DebugReportFlagsEXT {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::pfn_callback`]
    pub fn pfn_callback_mut(&mut self) -> &mut PFNDebugReportCallbackEXT {
        &mut self.pfn_callback
    }
    ///Gets a mutable reference to the value of [`Self::user_data`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn user_data_mut(&mut self) -> &mut c_void {
        &mut *self.user_data
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::flags`]
    pub fn set_flags(&mut self, value: crate::extensions::ext_debug_report::DebugReportFlagsEXT) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::pfn_callback`]
    pub fn set_pfn_callback(
        &mut self,
        value: crate::extensions::ext_debug_report::PFNDebugReportCallbackEXT,
    ) -> &mut Self {
        self.pfn_callback = value;
        self
    }
    ///Sets the raw value of [`Self::user_data`]
    pub fn set_user_data(&mut self, value: &'lt mut std::ffi::c_void) -> &mut Self {
        self.user_data = value as *mut _;
        self
    }
}
///[VkDebugReportCallbackEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugReportCallbackEXT.html) - Opaque handle to a debug report callback object
///# C Specifications
///Debug report callbacks are represented by [`DebugReportCallbackEXT`]
///handles:
///```c
///// Provided by VK_EXT_debug_report
///VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkDebugReportCallbackEXT)
///```
///# Related
/// - [`VK_EXT_debug_report`]
/// - [`CreateDebugReportCallbackEXT`]
/// - [`DestroyDebugReportCallbackEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDebugReportCallbackEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(transparent)]
pub struct DebugReportCallbackEXT(pub u64);
impl DebugReportCallbackEXT {
    ///Creates a new null handle
    #[inline]
    pub const fn null() -> Self {
        Self(0)
    }
    ///Checks if this is a null handle
    #[inline]
    pub fn is_null(&self) -> bool {
        self == &Self::null()
    }
    ///Gets the raw value
    #[inline]
    pub fn raw(&self) -> u64 {
        self.0
    }
}
unsafe impl Send for DebugReportCallbackEXT {}
impl Default for DebugReportCallbackEXT {
    fn default() -> Self {
        Self::null()
    }
}
