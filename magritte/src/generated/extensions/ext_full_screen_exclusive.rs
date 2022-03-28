//![VK_EXT_full_screen_exclusive](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_full_screen_exclusive.html) - device extension
//!# Description
//!This extension allows applications to set the policy for swapchain creation
//!and presentation mechanisms relating to full-screen access.
//!Implementations may be able to acquire exclusive access to a particular
//!display for an application window that covers the whole screen.
//!This can increase performance on some systems by bypassing composition,
//!however it can also result in disruptive or expensive transitions in the
//!underlying windowing system when a change occurs.Applications can choose between explicitly
//! disallowing or allowing this
//!behavior, letting the implementation decide, or managing this mode of
//!operation directly using the new [`AcquireFullScreenExclusiveModeEXT`]
//!and [`ReleaseFullScreenExclusiveModeEXT`] commands.
//!# Revision
//!4
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//! - Requires `[`VK_KHR_surface`]`
//! - Requires `[`VK_KHR_get_surface_capabilities2`]`
//! - Requires `[`VK_KHR_swapchain`]`
//!# Contacts
//! - James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_full_screen_exclusive]
//!   @cubanismo%0A<<Here describe the issue or question you have about the
//!   VK_EXT_full_screen_exclusive extension>>)
//!# New functions & commands
//! - [`AcquireFullScreenExclusiveModeEXT`]
//! - [`GetPhysicalDeviceSurfacePresentModes2EXT`]
//! - [`ReleaseFullScreenExclusiveModeEXT`]
//!If [`VK_KHR_device_group`] is supported:
//! - [`GetDeviceGroupSurfacePresentModes2EXT`]
//!If [Version 1.1]() is supported:
//! - [`GetDeviceGroupSurfacePresentModes2EXT`]
//!# New structures
//! - Extending [`PhysicalDeviceSurfaceInfo2KHR`], [`SwapchainCreateInfoKHR`]:  -
//!   [`SurfaceFullScreenExclusiveInfoEXT`]
//! - Extending [`SurfaceCapabilities2KHR`]:  - [`SurfaceCapabilitiesFullScreenExclusiveEXT`]
//!If [`VK_KHR_win32_surface`] is supported:
//! - Extending [`PhysicalDeviceSurfaceInfo2KHR`], [`SwapchainCreateInfoKHR`]:  -
//!   [`SurfaceFullScreenExclusiveWin32InfoEXT`]
//!# New enums
//! - [`FullScreenExclusiveEXT`]
//!# New constants
//! - [`EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME`]
//! - [`EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION`]
//! - Extending [`VulkanResultCodes`]:  - `VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT`  -
//!   `VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT`
//!If [`VK_KHR_win32_surface`] is supported:
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT`
//!# Known issues & F.A.Q
//!1) What should the extension & flag be called? **RESOLVED** : VK_EXT_full_screen_exclusive.Other
//! options considered (prior to the app-controlled mode) were:
//! - VK_EXT_smooth_fullscreen_transition
//! - VK_EXT_fullscreen_behavior
//! - VK_EXT_fullscreen_preference
//! - VK_EXT_fullscreen_hint
//! - VK_EXT_fast_fullscreen_transition
//! - VK_EXT_avoid_fullscreen_exclusive
//!2) Do we need more than a boolean toggle? **RESOLVED** : Yes.Using an enum with
//! default/allowed/disallowed/app-controlled enables
//!applications to accept driver default behavior, specifically override it in
//!either direction without implying the driver is ever required to use
//!full-screen exclusive mechanisms, or manage this mode explicitly.3) Should this be a KHR or EXT
//! extension? **RESOLVED** : EXT, in order to allow it to be shipped faster.4) Can the fullscreen
//! hint affect the surface capabilities, and if so,
//!should the hint also be specified as input when querying the surface
//!capabilities? **RESOLVED** : Yes on both accounts.While the hint does not guarantee a particular
//! fullscreen mode will be used
//!when the swapchain is created, it can sometimes imply particular modes will
//!NOT be used.
//!If the driver determines that it will opt-out of using a particular mode
//!based on the policy, and knows it can only support certain capabilities if
//!that mode is used, it would be confusing at best to the application to
//!report those capabilities in such cases.
//!Not allowing implementations to report this state to applications could
//!result in situations where applications are unable to determine why
//!swapchain creation fails when they specify certain hint values, which could
//!result in never- terminating surface creation loops.5) Should full-screen be one word or two?
//! **RESOLVED** : Two words."Fullscreen" is not in my dictionary, and web searches did not turn up
//!definitive proof that it is a colloquially accepted compound word.
//!Documentation for the corresponding Windows API mechanisms dithers.
//!The text consistently uses a hyphen, but none-the-less, there is a
//!SetFullscreenState method in the DXGI swapchain object.
//!Given this inconclusive external guidance, it is best to adhere to the
//!Vulkan style guidelines and avoid inventing new compound words.
//!# Version History
//! - Revision 4, 2019-03-12 (Tobias Hector)  - Added application-controlled mode, and related
//!   functions  - Tidied up appendix
//! - Revision 3, 2019-01-03 (James Jones)  - Renamed to VK_EXT_full_screen_exclusive  - Made
//!   related adjustments to the tri-state enumerant names.
//! - Revision 2, 2018-11-27 (James Jones)  - Renamed to VK_KHR_fullscreen_behavior  - Switched from
//!   boolean flag to tri-state enum
//! - Revision 1, 2018-11-06 (James Jones)  - Internal revision
//!# Other info
//! * 2019-03-12
//! * No known IP claims.
//! * - Interacts with Vulkan 1.1  - Interacts with `[`VK_KHR_device_group`]`  - Interacts with
//!   `[`VK_KHR_win32_surface`]`
//! * - Hans-Kristian Arntzen, ARM  - Slawomir Grajewski, Intel  - Tobias Hector, AMD  - James
//!   Jones, NVIDIA  - Daniel Rakos, AMD  - Jeff Juliano, NVIDIA  - Joshua Schnarr, NVIDIA  - Aaron
//!   Hagan, AMD
//!# Related
//! - [`FullScreenExclusiveEXT`]
//! - [`SurfaceCapabilitiesFullScreenExclusiveEXT`]
//! - [`SurfaceFullScreenExclusiveInfoEXT`]
//! - [`AcquireFullScreenExclusiveModeEXT`]
//! - [`GetPhysicalDeviceSurfacePresentModes2EXT`]
//! - [`ReleaseFullScreenExclusiveModeEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    native::HMONITOR,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType},
};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION")]
pub const EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION: u32 = 4;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME")]
pub const EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_full_screen_exclusive");
///[VkFullScreenExclusiveEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFullScreenExclusiveEXT.html) - Hint values an application can specify affecting full-screen transition behavior
///# C Specifications
///Possible values of
///[`SurfaceFullScreenExclusiveInfoEXT::full_screen_exclusive`] are:
///```c
///// Provided by VK_EXT_full_screen_exclusive
///typedef enum VkFullScreenExclusiveEXT {
///    VK_FULL_SCREEN_EXCLUSIVE_DEFAULT_EXT = 0,
///    VK_FULL_SCREEN_EXCLUSIVE_ALLOWED_EXT = 1,
///    VK_FULL_SCREEN_EXCLUSIVE_DISALLOWED_EXT = 2,
///    VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT = 3,
///} VkFullScreenExclusiveEXT;
///```
///# Description
/// - [`FullScreenExclusiveDefaultExt`] indicates the implementation  **should**  determine the
///   appropriate full-screen method by whatever means it deems appropriate.
/// - [`FullScreenExclusiveAllowedExt`] indicates the implementation  **may**  use full-screen
///   exclusive mechanisms when available. Such mechanisms  **may**  result in better performance
///   and/or the availability of different presentation capabilities, but  **may**  require a more
///   disruptive transition during swapchain initialization, first presentation and/or destruction.
/// - [`FullScreenExclusiveDisallowedExt`] indicates the implementation  **should**  avoid using
///   full-screen mechanisms which rely on disruptive transitions.
/// - [`FullScreenExclusiveApplicationControlledExt`] indicates the application will manage
///   full-screen exclusive mode by using the [`AcquireFullScreenExclusiveModeEXT`] and
///   [`ReleaseFullScreenExclusiveModeEXT`] commands.
///# Related
/// - [`VK_EXT_full_screen_exclusive`]
/// - [`SurfaceFullScreenExclusiveInfoEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkFullScreenExclusiveEXT")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(i32)]
pub enum FullScreenExclusiveEXT {
    ///[`FullScreenExclusiveDefaultExt`] indicates the implementation
    /// **should**  determine the appropriate full-screen method by whatever means
    ///it deems appropriate.
    FullScreenExclusiveDefaultExt = 0,
    ///[`FullScreenExclusiveAllowedExt`] indicates the implementation
    /// **may**  use full-screen exclusive mechanisms when available.
    ///Such mechanisms  **may**  result in better performance and/or the
    ///availability of different presentation capabilities, but  **may**  require a
    ///more disruptive transition during swapchain initialization, first
    ///presentation and/or destruction.
    FullScreenExclusiveAllowedExt = 1,
    ///[`FullScreenExclusiveDisallowedExt`] indicates the
    ///implementation  **should**  avoid using full-screen mechanisms which rely on
    ///disruptive transitions.
    FullScreenExclusiveDisallowedExt = 2,
    ///[`FullScreenExclusiveApplicationControlledExt`] indicates the
    ///application will manage full-screen exclusive mode by using the
    ///[`AcquireFullScreenExclusiveModeEXT`] and
    ///[`ReleaseFullScreenExclusiveModeEXT`] commands.
    FullScreenExclusiveApplicationControlledExt = 3,
}
impl const Default for FullScreenExclusiveEXT {
    fn default() -> Self {
        Self::FullScreenExclusiveDefaultExt
    }
}
impl FullScreenExclusiveEXT {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
    }
}
///[VkSurfaceFullScreenExclusiveInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceFullScreenExclusiveInfoEXT.html) - Structure specifying the preferred full-screen transition behavior
///# C Specifications
///If the [`p_next`] chain of [`SwapchainCreateInfoKHR`] includes a
///[`SurfaceFullScreenExclusiveInfoEXT`] structure, then that structure
///specifies the application’s preferred full-screen transition behavior.The
/// [`SurfaceFullScreenExclusiveInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_full_screen_exclusive
///typedef struct VkSurfaceFullScreenExclusiveInfoEXT {
///    VkStructureType             sType;
///    void*                       pNext;
///    VkFullScreenExclusiveEXT    fullScreenExclusive;
///} VkSurfaceFullScreenExclusiveInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`full_screen_exclusive`] is a [`FullScreenExclusiveEXT`] value specifying the preferred
///   full-screen transition behavior.
///# Description
///If this structure is not present, [`full_screen_exclusive`] is considered to
///be `VK_FULL_SCREEN_EXCLUSIVE_DEFAULT_EXT`.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT`
/// - [`full_screen_exclusive`] **must**  be a valid [`FullScreenExclusiveEXT`] value
///# Related
/// - [`VK_EXT_full_screen_exclusive`]
/// - [`FullScreenExclusiveEXT`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct SurfaceFullScreenExclusiveInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`full_screen_exclusive`] is a [`FullScreenExclusiveEXT`] value
    ///specifying the preferred full-screen transition behavior.
    full_screen_exclusive: FullScreenExclusiveEXT,
}
impl<'lt> Default for SurfaceFullScreenExclusiveInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            full_screen_exclusive: Default::default(),
        }
    }
}
impl<'lt> SurfaceFullScreenExclusiveInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
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
    ///Gets the value of [`Self::full_screen_exclusive`]
    pub fn full_screen_exclusive(&self) -> FullScreenExclusiveEXT {
        self.full_screen_exclusive
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
    ///Gets a mutable reference to the value of [`Self::full_screen_exclusive`]
    pub fn full_screen_exclusive_mut(&mut self) -> &mut FullScreenExclusiveEXT {
        &mut self.full_screen_exclusive
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::full_screen_exclusive`]
    pub fn set_full_screen_exclusive(
        &mut self,
        value: crate::extensions::ext_full_screen_exclusive::FullScreenExclusiveEXT,
    ) -> &mut Self {
        self.full_screen_exclusive = value;
        self
    }
}
///[VkSurfaceFullScreenExclusiveWin32InfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceFullScreenExclusiveWin32InfoEXT.html) - Structure specifying additional creation parameters specific to Win32 fullscreen exclusive mode
///# C Specifications
///The [`SurfaceFullScreenExclusiveWin32InfoEXT`] structure is defined as:
///```c
///// Provided by VK_KHR_win32_surface with VK_EXT_full_screen_exclusive
///typedef struct VkSurfaceFullScreenExclusiveWin32InfoEXT {
///    VkStructureType    sType;
///    const void*        pNext;
///    HMONITOR           hmonitor;
///} VkSurfaceFullScreenExclusiveWin32InfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`hmonitor`] is the Win32 [`HMONITOR`] handle identifying the display to create the surface
///   with.
///# Description
///## Valid Usage
/// - [`hmonitor`] **must**  be a valid [`HMONITOR`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT`
///# Related
/// - [`VK_EXT_full_screen_exclusive`]
/// - [`VK_KHR_win32_surface`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct SurfaceFullScreenExclusiveWin32InfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`hmonitor`] is the Win32 [`HMONITOR`] handle identifying the display
    ///to create the surface with.
    hmonitor: HMONITOR,
}
impl<'lt> Default for SurfaceFullScreenExclusiveWin32InfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            hmonitor: Default::default(),
        }
    }
}
impl<'lt> SurfaceFullScreenExclusiveWin32InfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::hmonitor`]
    pub fn hmonitor_raw(&self) -> &HMONITOR {
        &self.hmonitor
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::hmonitor`]
    pub fn set_hmonitor_raw(&mut self, value: HMONITOR) -> &mut Self {
        self.hmonitor = value;
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
    ///Gets the value of [`Self::hmonitor`]
    pub fn hmonitor(&self) -> &HMONITOR {
        &self.hmonitor
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::hmonitor`]
    pub fn hmonitor_mut(&mut self) -> &mut HMONITOR {
        &mut self.hmonitor
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
    ///Sets the raw value of [`Self::hmonitor`]
    pub fn set_hmonitor(&mut self, value: crate::native::HMONITOR) -> &mut Self {
        self.hmonitor = value;
        self
    }
}
///[VkSurfaceCapabilitiesFullScreenExclusiveEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceCapabilitiesFullScreenExclusiveEXT.html) - Structure describing full screen exclusive capabilities of a surface
///# C Specifications
///The [`SurfaceCapabilitiesFullScreenExclusiveEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_full_screen_exclusive
///typedef struct VkSurfaceCapabilitiesFullScreenExclusiveEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           fullScreenExclusiveSupported;
///} VkSurfaceCapabilitiesFullScreenExclusiveEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - `fullScreenExclusiveControlSupported` is a boolean describing whether the surface is able to
///   make use of exclusive full-screen access.
///# Description
///This structure  **can**  be included in the [`p_next`] chain of
///[`SurfaceCapabilities2KHR`] to determine support for exclusive
///full-screen access.
///If [`full_screen_exclusive_supported`] is [`FALSE`], it indicates that
///exclusive full-screen access is not obtainable for this surface.Applications  **must**  not
/// attempt to create swapchains with
///`VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT` set if
///[`full_screen_exclusive_supported`] is [`FALSE`].
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT`
///# Related
/// - [`VK_EXT_full_screen_exclusive`]
/// - [`Bool32`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct SurfaceCapabilitiesFullScreenExclusiveEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///No documentation found
    full_screen_exclusive_supported: Bool32,
}
impl<'lt> Default for SurfaceCapabilitiesFullScreenExclusiveEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            full_screen_exclusive_supported: 0,
        }
    }
}
impl<'lt> SurfaceCapabilitiesFullScreenExclusiveEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::full_screen_exclusive_supported`]
    pub fn full_screen_exclusive_supported_raw(&self) -> Bool32 {
        self.full_screen_exclusive_supported
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::full_screen_exclusive_supported`]
    pub fn set_full_screen_exclusive_supported_raw(&mut self, value: Bool32) -> &mut Self {
        self.full_screen_exclusive_supported = value;
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
    ///Gets the value of [`Self::full_screen_exclusive_supported`]
    pub fn full_screen_exclusive_supported(&self) -> bool {
        unsafe { std::mem::transmute(self.full_screen_exclusive_supported as u8) }
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
    ///Gets a mutable reference to the value of [`Self::full_screen_exclusive_supported`]
    pub fn full_screen_exclusive_supported_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.full_screen_exclusive_supported as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.full_screen_exclusive_supported as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::full_screen_exclusive_supported`]
    pub fn set_full_screen_exclusive_supported(&mut self, value: bool) -> &mut Self {
        self.full_screen_exclusive_supported = value as u8 as u32;
        self
    }
}
