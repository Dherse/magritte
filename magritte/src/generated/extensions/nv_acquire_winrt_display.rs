//![VK_NV_acquire_winrt_display](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_acquire_winrt_display.html) - device extension
//!# Description
//!This extension allows an application to take exclusive control of a display
//!on Windows 10 provided that the display is not already controlled by a
//!compositor.
//!Examples of compositors include the Windows desktop compositor, other
//!applications using this Vulkan extension, and applications that
//![“Acquire”](https://docs.microsoft.com/en-us/uwp/api/windows.devices.display.core.displaymanager.tryacquiretarget)
//!a
//![“DisplayTarget”](https://docs.microsoft.com/en-us/uwp/api/windows.devices.display.core.displaytarget)
//!using a [“WinRT”](https://docs.microsoft.com/en-us/uwp/api/) command such as
//![“winrt::Windows::Devices::Display::Core::DisplayManager.TryAcquireTarget()”](https://docs.microsoft.com/en-us/uwp/api/windows.devices.display.core.displaymanager.tryacquiretarget).When control is acquired the application has exclusive access to the display
//!until control is released or the application terminates.
//!An application’s attempt to acquire is denied if a different application has
//!already acquired the display.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_EXT_direct_mode_display`]`
//!# Contacts
//! - Jeff Juliano [jjuliano](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_acquire_winrt_display]
//!   @jjuliano%0A<<Here describe the issue or question you have about the
//!   VK_NV_acquire_winrt_display extension>>)
//!# New functions & commands
//! - [`acquire_winrt_display_nv`]
//! - [`get_winrt_display_nv`]
//!# New constants
//! - [`NV_ACQUIRE_WINRT_DISPLAY_EXTENSION_NAME`]
//! - [`NV_ACQUIRE_WINRT_DISPLAY_SPEC_VERSION`]
//!# Known issues & F.A.Q
//!1) What should the platform substring be for this extension: **RESOLVED** : The platform
//! substring is “Winrt”.The substring “Winrt” matches the fact that the OS API exposing the
//!acquire and release functionality is called “WinRT”.The substring “Win32” is wrong because the
//! related “WinRT” API is
//!explicitly  **not**  a “Win32” API.
//!“WinRT” is a competing API family to the “Win32” API family.The substring “Windows” is
//! suboptimal because there could be more than one
//!relevant API on the Windows platform.
//!There is preference to use the more-specific substring “Winrt”.2) Should
//! [`acquire_winrt_display_nv`] take a winRT DisplayTarget, or a
//!Vulkan display handle as input? **RESOLVED** : A Vulkan display handle.
//!This matches the design of [`acquire_xlib_display_ext`].3) Should the acquire command be
//! platform-independent named
//!“vkAcquireDisplayNV”, or platform-specific named
//!“vkAcquireWinrtDisplayNV”? **RESOLVED** : Add a platform-specific command.The inputs to the
//! Acquire command are all Vulkan types.
//!None are WinRT types.
//!This opens the possibility of the winrt extension defining a
//!platform-independent acquire command.The X11 acquire command does need to accept a
//! platform-specific parameter.
//!This could be handled by adding to a platform-independent acquire command a
//!params struct to which platform-dependent types can be chained by
//!`pNext` pointer.The prevailing opinion is that it would be odd to create a second
//!platform-independent function that is used on the Windows 10 platform, but
//!that is not used for the X11 platform.
//!Since a Windows 10 platform-specific command is needed anyway for converting
//!between vkDisplayKHR and platform-native handles, opinion was to create a
//!platform-specific acquire function.4) Should the [`get_winrt_display_nv`] parameter identifying
//! a display be
//!named “deviceRelativeId” or “adapterRelativeId”? **RESOLVED** : The WinRT name is
//! “AdapterRelativeId”.
//!The name “adapter” is the Windows analog to a Vulkan “physical device”.
//!Vulkan already has precedent to use the name `deviceLUID` for the
//!concept that Windows APIs call “AdapterLuid”.
//!Keeping form with this precedent, the name “deviceRelativeId” is chosen.5) Does
//! [`acquire_winrt_display_nv`] cause the Windows desktop compositor
//!to release a display? **RESOLVED** : No.
//![`acquire_winrt_display_nv`] does not itself cause the Windows desktop
//!compositor to release a display.
//!This action must be performed outside of Vulkan.Beginning with Windows 10 version 2004 it is
//! possible to cause the Windows
//!desktop compositor to release a display by using the “Advanced display
//!settings” sub-page of the “Display settings” control panel.
//!See
//![https://docs.microsoft.com/en-us/windows-hardware/drivers/display/specialized-monitors](https://docs.microsoft.com/en-us/windows-hardware/drivers/display/specialized-monitors)6) Where can one find additional information about custom compositors for
//!Windows 10? **RESOLVED** : Relevant references are as follows.According to Microsoft’s
//! documentation on
//!["building
//!a custom compositor"](https://docs.microsoft.com/en-us/windows-hardware/drivers/display/specialized-monitors-compositor), the ability to write a custom compositor is not a
//!replacement for a fullscreen desktop window.
//!The feature is for writing compositor apps that drive specialized hardware.Only certain editions
//! of Windows 10 support custom compositors,
//!["documented
//!here"](https://docs.microsoft.com/en-us/windows-hardware/drivers/display/specialized-monitors#windows-10-version-2004).
//!The product type can be queried from Windows 10.
//!See
//![https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getproductinfo](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getproductinfo)
//!# Version History
//! - Revision 1, 2020-09-29 (Jeff Juliano)  - Initial draft
//!# Other info
//! * 2020-09-29
//! * No known IP claims.
//! * - Jeff Juliano, NVIDIA
//!# Related
//! - [`acquire_winrt_display_nv`]
//! - [`get_winrt_display_nv`]
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
use std::{ffi::CStr, mem::MaybeUninit};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_ACQUIRE_WINRT_DISPLAY_SPEC_VERSION")]
pub const NV_ACQUIRE_WINRT_DISPLAY_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_ACQUIRE_WINRT_DISPLAY_EXTENSION_NAME")]
pub const NV_ACQUIRE_WINRT_DISPLAY_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_acquire_winrt_display");
///[vkAcquireWinrtDisplayNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireWinrtDisplayNV.html) - Acquire access to a VkDisplayKHR
///# C Specifications
///To acquire permission to directly access a display in Vulkan on Windows 10,
///call:
///```c
///// Provided by VK_NV_acquire_winrt_display
///VkResult vkAcquireWinrtDisplayNV(
///    VkPhysicalDevice                            physicalDevice,
///    VkDisplayKHR                                display);
///```
///# Parameters
/// - [`physical_device`] The physical device the display is on.
/// - [`display`] The display the caller wishes to control in Vulkan.
///# Description
///All permissions necessary to control the display are granted to the Vulkan
///instance associated with [`physical_device`] until the display is released
///or the application is terminated.
///Permission to access the display  **may**  be revoked by events that cause
///Windows 10 itself to lose access to [`display`].
///If this has happened, operations which require access to the display  **must**
///fail with an appropriate error code.
///If permission to access [`display`] has already been acquired by another
///entity, the call  **must**  return the error code
///`VK_ERROR_INITIALIZATION_FAILED`.
///## Valid Usage (Implicit)
/// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
/// - [`display`] **must**  be a valid [`DisplayKHR`] handle
/// - [`display`] **must**  have been created, allocated, or retrieved from [`physical_device`]
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_DEVICE_LOST`  - `VK_ERROR_INITIALIZATION_FAILED`
///# Related
/// - [`VK_NV_acquire_winrt_display`]
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
#[doc(alias = "vkAcquireWinrtDisplayNV")]
pub type FNAcquireWinrtDisplayNv =
    Option<unsafe extern "system" fn(physical_device: PhysicalDevice, display: DisplayKHR) -> VulkanResultCodes>;
///[vkGetWinrtDisplayNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetWinrtDisplayNV.html) - Query the VkDisplayKHR corresponding to a WinRT DisplayTarget
///# C Specifications
///When acquiring displays on Windows 10, an application may also wish to
///enumerate and identify them using a native handle rather than a
///[`DisplayKHR`] handle.To determine the [`DisplayKHR`] handle corresponding to a
///[“winrt::Windows::Devices::Display::Core::DisplayTarget”](https://docs.microsoft.com/en-us/uwp/api/windows.devices.display.core.displaytarget),
///call:
///```c
///// Provided by VK_NV_acquire_winrt_display
///VkResult vkGetWinrtDisplayNV(
///    VkPhysicalDevice                            physicalDevice,
///    uint32_t                                    deviceRelativeId,
///    VkDisplayKHR*                               pDisplay);
///```
///# Parameters
/// - [`physical_device`] The physical device on which to query the display handle.
/// - [`device_relative_id`] The value of the [“AdapterRelativeId”](https://docs.microsoft.com/en-us/uwp/api/windows.devices.display.core.displaytarget.adapterrelativeid)
///   property of a [“DisplayTarget”](https://docs.microsoft.com/en-us/uwp/api/windows.devices.display.core.displaytarget)
///   that is enumerated by a [“DisplayAdapter”](https://docs.microsoft.com/en-us/uwp/api/windows.devices.display.core.displayadapter)
///   with an [“Id”](https://docs.microsoft.com/en-us/uwp/api/windows.devices.display.core.displayadapter.id)
///   property matching the `deviceLUID` property of a [`PhysicalDeviceIdProperties`] for
///   [`physical_device`].
/// - [`p_display`] The corresponding [`DisplayKHR`] handle will be returned here.
///# Description
///If there is no [`DisplayKHR`] corresponding to [`device_relative_id`] on
///[`physical_device`], [`crate::Handle::null`] **must**  be returned in
///[`p_display`].
///## Valid Usage (Implicit)
/// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
/// - [`p_display`] **must**  be a valid pointer to a [`DisplayKHR`] handle
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_DEVICE_LOST`  - `VK_ERROR_INITIALIZATION_FAILED`
///# Related
/// - [`VK_NV_acquire_winrt_display`]
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
#[doc(alias = "vkGetWinrtDisplayNV")]
pub type FNGetWinrtDisplayNv = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        device_relative_id: u32,
        p_display: *mut DisplayKHR,
    ) -> VulkanResultCodes,
>;
impl PhysicalDevice {
    ///[vkAcquireWinrtDisplayNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireWinrtDisplayNV.html) - Acquire access to a VkDisplayKHR
    ///# C Specifications
    ///To acquire permission to directly access a display in Vulkan on Windows 10,
    ///call:
    ///```c
    ///// Provided by VK_NV_acquire_winrt_display
    ///VkResult vkAcquireWinrtDisplayNV(
    ///    VkPhysicalDevice                            physicalDevice,
    ///    VkDisplayKHR                                display);
    ///```
    ///# Parameters
    /// - [`physical_device`] The physical device the display is on.
    /// - [`display`] The display the caller wishes to control in Vulkan.
    ///# Description
    ///All permissions necessary to control the display are granted to the Vulkan
    ///instance associated with [`physical_device`] until the display is released
    ///or the application is terminated.
    ///Permission to access the display  **may**  be revoked by events that cause
    ///Windows 10 itself to lose access to [`display`].
    ///If this has happened, operations which require access to the display  **must**
    ///fail with an appropriate error code.
    ///If permission to access [`display`] has already been acquired by another
    ///entity, the call  **must**  return the error code
    ///`VK_ERROR_INITIALIZATION_FAILED`.
    ///## Valid Usage (Implicit)
    /// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
    /// - [`display`] **must**  be a valid [`DisplayKHR`] handle
    /// - [`display`] **must**  have been created, allocated, or retrieved from [`physical_device`]
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_DEVICE_LOST`  -
    ///   `VK_ERROR_INITIALIZATION_FAILED`
    ///# Related
    /// - [`VK_NV_acquire_winrt_display`]
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
    #[doc(alias = "vkAcquireWinrtDisplayNV")]
    #[track_caller]
    #[inline]
    pub unsafe fn acquire_winrt_display_nv<'a: 'this, 'this>(
        self: &'this Unique<'a, PhysicalDevice>,
        display: DisplayKHR,
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .instance()
            .vtable()
            .nv_acquire_winrt_display()
            .and_then(|vtable| vtable.acquire_winrt_display_nv())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .instance()
            .vtable()
            .nv_acquire_winrt_display()
            .and_then(|vtable| vtable.acquire_winrt_display_nv())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), display);
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
impl PhysicalDevice {
    ///[vkGetWinrtDisplayNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetWinrtDisplayNV.html) - Query the VkDisplayKHR corresponding to a WinRT DisplayTarget
    ///# C Specifications
    ///When acquiring displays on Windows 10, an application may also wish to
    ///enumerate and identify them using a native handle rather than a
    ///[`DisplayKHR`] handle.To determine the [`DisplayKHR`] handle corresponding to a
    ///[“winrt::Windows::Devices::Display::Core::DisplayTarget”](https://docs.microsoft.com/en-us/uwp/api/windows.devices.display.core.displaytarget),
    ///call:
    ///```c
    ///// Provided by VK_NV_acquire_winrt_display
    ///VkResult vkGetWinrtDisplayNV(
    ///    VkPhysicalDevice                            physicalDevice,
    ///    uint32_t                                    deviceRelativeId,
    ///    VkDisplayKHR*                               pDisplay);
    ///```
    ///# Parameters
    /// - [`physical_device`] The physical device on which to query the display handle.
    /// - [`device_relative_id`] The value of the [“AdapterRelativeId”](https://docs.microsoft.com/en-us/uwp/api/windows.devices.display.core.displaytarget.adapterrelativeid)
    ///   property of a [“DisplayTarget”](https://docs.microsoft.com/en-us/uwp/api/windows.devices.display.core.displaytarget)
    ///   that is enumerated by a [“DisplayAdapter”](https://docs.microsoft.com/en-us/uwp/api/windows.devices.display.core.displayadapter)
    ///   with an [“Id”](https://docs.microsoft.com/en-us/uwp/api/windows.devices.display.core.displayadapter.id)
    ///   property matching the `deviceLUID` property of a [`PhysicalDeviceIdProperties`] for
    ///   [`physical_device`].
    /// - [`p_display`] The corresponding [`DisplayKHR`] handle will be returned here.
    ///# Description
    ///If there is no [`DisplayKHR`] corresponding to [`device_relative_id`] on
    ///[`physical_device`], [`crate::Handle::null`] **must**  be returned in
    ///[`p_display`].
    ///## Valid Usage (Implicit)
    /// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
    /// - [`p_display`] **must**  be a valid pointer to a [`DisplayKHR`] handle
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_DEVICE_LOST`  -
    ///   `VK_ERROR_INITIALIZATION_FAILED`
    ///# Related
    /// - [`VK_NV_acquire_winrt_display`]
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
    #[doc(alias = "vkGetWinrtDisplayNV")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_winrt_display_nv<'a: 'this, 'this>(
        self: &'this Unique<'a, PhysicalDevice>,
        device_relative_id: Option<u32>,
    ) -> VulkanResult<Unique<'this, DisplayKHR>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .instance()
            .vtable()
            .nv_acquire_winrt_display()
            .and_then(|vtable| vtable.get_winrt_display_nv())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .instance()
            .vtable()
            .nv_acquire_winrt_display()
            .and_then(|vtable| vtable.get_winrt_display_nv())
            .unwrap_unchecked();
        let mut p_display = MaybeUninit::<DisplayKHR>::uninit();
        let _return = _function(
            self.as_raw(),
            device_relative_id.unwrap_or_default() as _,
            p_display.as_mut_ptr(),
        );
        match _return {
            VulkanResultCodes::SUCCESS => {
                VulkanResult::Success(_return, Unique::new(self, p_display.assume_init(), true))
            },
            e => VulkanResult::Err(e),
        }
    }
}
///The V-table of [`Instance`] for functions from `VK_NV_acquire_winrt_display`
pub struct InstanceNvAcquireWinrtDisplayVTable {
    ///See [`FNAcquireWinrtDisplayNv`] for more information.
    pub acquire_winrt_display_nv: FNAcquireWinrtDisplayNv,
    ///See [`FNGetWinrtDisplayNv`] for more information.
    pub get_winrt_display_nv: FNGetWinrtDisplayNv,
}
impl InstanceNvAcquireWinrtDisplayVTable {
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
            acquire_winrt_display_nv: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkAcquireWinrtDisplayNV").as_ptr()))
            },
            get_winrt_display_nv: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkGetWinrtDisplayNV").as_ptr()))
            },
        }
    }
    ///Gets [`Self::acquire_winrt_display_nv`]. See [`FNAcquireWinrtDisplayNv`] for more
    /// information.
    pub fn acquire_winrt_display_nv(&self) -> FNAcquireWinrtDisplayNv {
        self.acquire_winrt_display_nv
    }
    ///Gets [`Self::get_winrt_display_nv`]. See [`FNGetWinrtDisplayNv`] for more information.
    pub fn get_winrt_display_nv(&self) -> FNGetWinrtDisplayNv {
        self.get_winrt_display_nv
    }
}
