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
//! - [`AcquireWinrtDisplayNV`]
//! - [`GetWinrtDisplayNV`]
//!# New constants
//! - [`NV_ACQUIRE_WINRT_DISPLAY_EXTENSION_NAME`]
//! - [`NV_ACQUIRE_WINRT_DISPLAY_SPEC_VERSION`]
//!# Known issues & F.A.Q
//!1) What should the platform substring be for this extension:**RESOLVED**: The platform substring
//! is “Winrt”.The substring “Winrt” matches the fact that the OS API exposing the
//!acquire and release functionality is called “WinRT”.The substring “Win32” is wrong because the
//! related “WinRT” API is
//!explicitly **not** a “Win32” API.
//!“WinRT” is a competing API family to the “Win32” API family.The substring “Windows” is
//! suboptimal because there could be more than one
//!relevant API on the Windows platform.
//!There is preference to use the more-specific substring “Winrt”.2) Should
//! [`AcquireWinrtDisplayNV`] take a winRT DisplayTarget, or a
//!Vulkan display handle as input?**RESOLVED**: A Vulkan display handle.
//!This matches the design of [`AcquireXlibDisplayEXT`].3) Should the acquire command be
//! platform-independent named
//!“vkAcquireDisplayNV”, or platform-specific named
//!“vkAcquireWinrtDisplayNV”?**RESOLVED**: Add a platform-specific command.The inputs to the
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
//!platform-specific acquire function.4) Should the [`GetWinrtDisplayNV`] parameter identifying a
//! display be
//!named “deviceRelativeId” or “adapterRelativeId”?**RESOLVED**: The WinRT name is
//! “AdapterRelativeId”.
//!The name “adapter” is the Windows analog to a Vulkan “physical device”.
//!Vulkan already has precedent to use the name `deviceLUID` for the
//!concept that Windows APIs call “AdapterLuid”.
//!Keeping form with this precedent, the name “deviceRelativeId” is chosen.5) Does
//! [`AcquireWinrtDisplayNV`] cause the Windows desktop compositor
//!to release a display?**RESOLVED**: No.
//![`AcquireWinrtDisplayNV`] does not itself cause the Windows desktop
//!compositor to release a display.
//!This action must be performed outside of Vulkan.Beginning with Windows 10 version 2004 it is
//! possible to cause the Windows
//!desktop compositor to release a display by using the “Advanced display
//!settings” sub-page of the “Display settings” control panel.
//!See
//![https://docs.microsoft.com/en-us/windows-hardware/drivers/display/specialized-monitors](https://docs.microsoft.com/en-us/windows-hardware/drivers/display/specialized-monitors)6) Where can one find additional information about custom compositors for
//!Windows 10?**RESOLVED**: Relevant references are as follows.According to Microsoft’s
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
//! - Revision 1, 2020-09-29 (Jeff Juliano)
//! - Initial draft
//!# Other info
//! * 2020-09-29
//! * No known IP claims.
//!*
//! - Jeff Juliano, NVIDIA
//!# Related
//! - [`AcquireWinrtDisplayNV`]
//! - [`GetWinrtDisplayNV`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_ACQUIRE_WINRT_DISPLAY_SPEC_VERSION")]
pub const NV_ACQUIRE_WINRT_DISPLAY_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_ACQUIRE_WINRT_DISPLAY_EXTENSION_NAME")]
pub const NV_ACQUIRE_WINRT_DISPLAY_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_acquire_winrt_display");
