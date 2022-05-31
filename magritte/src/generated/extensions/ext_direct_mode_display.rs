//![VK_EXT_direct_mode_display](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_direct_mode_display.html) - instance extension
//!# Description
//!This is extension, along with related platform extensions, allows
//!applications to take exclusive control of displays associated with a native
//!windowing system.
//!This is especially useful for virtual reality applications that wish to hide
//!HMDs (head mounted displays) from the native platform’s display management
//!system, desktop, and/or other applications.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_display`]`
//!# Contacts
//! - James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_direct_mode_display]
//!   @cubanismo%0A<<Here describe the issue or question you have about the
//!   VK_EXT_direct_mode_display extension>>)
//!# New functions & commands
//! - [`release_display_ext`]
//!# New constants
//! - [`EXT_DIRECT_MODE_DISPLAY_EXTENSION_NAME`]
//! - [`EXT_DIRECT_MODE_DISPLAY_SPEC_VERSION`]
//!# Known issues & F.A.Q
//!1) Should this extension and its related platform-specific extensions
//!leverage `[`khr_display`]`, or provide separate equivalent interfaces. **RESOLVED** : Use
//! `[`khr_display`]` concepts and objects.
//!`[`khr_display`]` can be used to enumerate all displays on the system,
//!including those attached to/in use by a window system or native platform,
//!but `[`khr_display_swapchain`]` will fail to create a swapchain on
//!in-use displays.
//!This extension and its platform-specific children will allow applications to
//!grab in-use displays away from window systems and/or native platforms,
//!allowing them to be used with `[`khr_display_swapchain`]`.2) Are separate calls needed to
//! acquire displays and enable direct mode? **RESOLVED** : No, these operations happen in one
//! combined command.
//!Acquiring a display puts it into direct mode.
//!# Version History
//! - Revision 1, 2016-12-13 (James Jones)  - Initial draft
//!# Other info
//! * 2016-12-13
//! * No known IP claims.
//! * - Pierre Boudier, NVIDIA  - James Jones, NVIDIA  - Damien Leone, NVIDIA  - Pierre-Loup
//!   Griffais, Valve  - Liam Middlebrook, NVIDIA
//!# Related
//! - [`release_display_ext`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    extensions::khr_display::DisplayKHR,
    vulkan1_0::{Instance, PhysicalDevice, VulkanResultCodes},
    AsRaw, Unique, VulkanResult,
};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DIRECT_MODE_DISPLAY_SPEC_VERSION")]
pub const EXT_DIRECT_MODE_DISPLAY_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DIRECT_MODE_DISPLAY_EXTENSION_NAME")]
pub const EXT_DIRECT_MODE_DISPLAY_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_direct_mode_display");
///[vkReleaseDisplayEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkReleaseDisplayEXT.html) - Release access to an acquired VkDisplayKHR
///# C Specifications
///To release a previously acquired display, call:
///```c
///// Provided by VK_EXT_direct_mode_display
///VkResult vkReleaseDisplayEXT(
///    VkPhysicalDevice                            physicalDevice,
///    VkDisplayKHR                                display);
///```
///# Parameters
/// - [`physical_device`] The physical device the display is on.
/// - [`display`] The display to release control of.
///# Description
///## Valid Usage (Implicit)
/// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
/// - [`display`] **must**  be a valid [`DisplayKHR`] handle
/// - [`display`] **must**  have been created, allocated, or retrieved from [`physical_device`]
///
///## Return Codes
/// * - `VK_SUCCESS`
///# Related
/// - [`ext_direct_mode_display`]
/// - [`DisplayKHR`]
/// - [`PhysicalDevice`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkReleaseDisplayEXT")]
pub type FNReleaseDisplayExt =
    Option<unsafe extern "system" fn(physical_device: PhysicalDevice, display: DisplayKHR) -> VulkanResultCodes>;
impl PhysicalDevice {
    ///[vkReleaseDisplayEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkReleaseDisplayEXT.html) - Release access to an acquired VkDisplayKHR
    ///# C Specifications
    ///To release a previously acquired display, call:
    ///```c
    ///// Provided by VK_EXT_direct_mode_display
    ///VkResult vkReleaseDisplayEXT(
    ///    VkPhysicalDevice                            physicalDevice,
    ///    VkDisplayKHR                                display);
    ///```
    ///# Parameters
    /// - [`physical_device`] The physical device the display is on.
    /// - [`display`] The display to release control of.
    ///# Description
    ///## Valid Usage (Implicit)
    /// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
    /// - [`display`] **must**  be a valid [`DisplayKHR`] handle
    /// - [`display`] **must**  have been created, allocated, or retrieved from [`physical_device`]
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    ///# Related
    /// - [`ext_direct_mode_display`]
    /// - [`DisplayKHR`]
    /// - [`PhysicalDevice`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkReleaseDisplayEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn release_display_ext(self: &Unique<PhysicalDevice>, display: DisplayKHR) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .instance()
            .vtable()
            .ext_direct_mode_display()
            .and_then(|vtable| vtable.release_display_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .instance()
            .vtable()
            .ext_direct_mode_display()
            .and_then(|vtable| vtable.release_display_ext())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), display);
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
///The V-table of [`Instance`] for functions from `VK_EXT_direct_mode_display`
pub struct InstanceExtDirectModeDisplayVTable {
    ///See [`FNReleaseDisplayExt`] for more information.
    pub release_display_ext: FNReleaseDisplayExt,
}
impl InstanceExtDirectModeDisplayVTable {
    ///Loads the VTable from the owner and the names
    #[track_caller]
    pub fn load(
        loader_fn: unsafe extern "system" fn(
            Instance,
            *const std::os::raw::c_char,
        ) -> Option<unsafe extern "system" fn()>,
        loader: Instance,
    ) -> Self {
        Self {
            release_display_ext: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkReleaseDisplayEXT").as_ptr()))
            },
        }
    }
    ///Gets [`Self::release_display_ext`]. See [`FNReleaseDisplayExt`] for more information.
    pub fn release_display_ext(&self) -> FNReleaseDisplayExt {
        self.release_display_ext
    }
}
