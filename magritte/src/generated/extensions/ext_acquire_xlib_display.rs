//![VK_EXT_acquire_xlib_display](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_acquire_xlib_display.html) - instance extension
//!# Description
//!This extension allows an application to take exclusive control on a display
//!currently associated with an X11 screen.
//!When control is acquired, the display will be deassociated from the X11
//!screen until control is released or the specified display connection is
//!closed.
//!Essentially, the X11 screen will behave as if the monitor has been unplugged
//!until control is released.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_EXT_direct_mode_display`]`
//!# Contacts
//! - James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_acquire_xlib_display]
//!   @cubanismo%0A<<Here describe the issue or question you have about the
//!   VK_EXT_acquire_xlib_display extension>>)
//!# New functions & commands
//! - [`acquire_xlib_display_ext`]
//! - [`get_rand_r_output_display_ext`]
//!# New constants
//! - [`EXT_ACQUIRE_XLIB_DISPLAY_EXTENSION_NAME`]
//! - [`EXT_ACQUIRE_XLIB_DISPLAY_SPEC_VERSION`]
//!# Known issues & F.A.Q
//!1) Should [`acquire_xlib_display_ext`] take an RandR display ID, or a
//!Vulkan display handle as input? **RESOLVED** : A Vulkan display handle.
//!Otherwise there would be no way to specify handles to displays that had been
//!prevented from being included in the X11 display list by some native
//!platform or vendor-specific mechanism.2) How does an application figure out which RandR display
//! corresponds to a
//!Vulkan display? **RESOLVED** : A new function, [`get_rand_r_output_display_ext`], is introduced
//!for this purpose.3) Should [`get_rand_r_output_display_ext`] be part of this extension, or a
//!general Vulkan / RandR or Vulkan / Xlib extension? **RESOLVED** : To avoid yet another
//! extension, include it in this extension.
//!# Version History
//! - Revision 1, 2016-12-13 (James Jones)  - Initial draft
//!# Other info
//! * 2016-12-13
//! * No known IP claims.
//! * - Dave Airlie, Red Hat  - Pierre Boudier, NVIDIA  - James Jones, NVIDIA  - Damien Leone,
//!   NVIDIA  - Pierre-Loup Griffais, Valve  - Liam Middlebrook, NVIDIA  - Daniel Vetter, Intel
//!# Related
//! - [`acquire_xlib_display_ext`]
//! - [`get_rand_r_output_display_ext`]
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
    native::{Display, RROutput},
    vulkan1_0::{Instance, PhysicalDevice, VulkanResultCodes},
};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_ACQUIRE_XLIB_DISPLAY_SPEC_VERSION")]
pub const EXT_ACQUIRE_XLIB_DISPLAY_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_ACQUIRE_XLIB_DISPLAY_EXTENSION_NAME")]
pub const EXT_ACQUIRE_XLIB_DISPLAY_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_acquire_xlib_display");
///[vkAcquireXlibDisplayEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireXlibDisplayEXT.html) - Acquire access to a VkDisplayKHR using Xlib
///# C Specifications
///To acquire permission to directly access a display in Vulkan from an X11
///server, call:
///```c
///// Provided by VK_EXT_acquire_xlib_display
///VkResult vkAcquireXlibDisplayEXT(
///    VkPhysicalDevice                            physicalDevice,
///    Display*                                    dpy,
///    VkDisplayKHR                                display);
///```
///# Parameters
/// - [`physical_device`] The physical device the display is on.
/// - [`dpy`] A connection to the X11 server that currently owns [`display`].
/// - [`display`] The display the caller wishes to control in Vulkan.
///# Description
///All permissions necessary to control the display are granted to the Vulkan
///instance associated with [`physical_device`] until the display is released
///or the X11 connection specified by [`dpy`] is terminated.
///Permission to access the display  **may**  be temporarily revoked during periods
///when the X11 server from which control was acquired itself loses access to
///[`display`].
///During such periods, operations which require access to the display  **must**
///fail with an approriate error code.
///If the X11 server associated with [`dpy`] does not own [`display`], or
///if permission to access it has already been acquired by another entity, the
///call  **must**  return the error code `VK_ERROR_INITIALIZATION_FAILED`.
///## Valid Usage (Implicit)
/// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
/// - [`dpy`] **must**  be a valid pointer to a [`Display`] value
/// - [`display`] **must**  be a valid [`DisplayKHR`] handle
/// - [`display`] **must**  have been created, allocated, or retrieved from [`physical_device`]
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_INITIALIZATION_FAILED`
///# Related
/// - [`VK_EXT_acquire_xlib_display`]
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
#[doc(alias = "vkAcquireXlibDisplayEXT")]
pub type FNAcquireXlibDisplayExt = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        dpy: *mut Display,
        display: DisplayKHR,
    ) -> VulkanResultCodes,
>;
///[vkGetRandROutputDisplayEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRandROutputDisplayEXT.html) - Query the VkDisplayKHR corresponding to an X11 RandR Output
///# C Specifications
///When acquiring displays from an X11 server, an application may also wish to
///enumerate and identify them using a native handle rather than a
///[`DisplayKHR`] handle.
///To determine the [`DisplayKHR`] handle corresponding to an X11 RandR
///Output, call:
///```c
///// Provided by VK_EXT_acquire_xlib_display
///VkResult vkGetRandROutputDisplayEXT(
///    VkPhysicalDevice                            physicalDevice,
///    Display*                                    dpy,
///    RROutput                                    rrOutput,
///    VkDisplayKHR*                               pDisplay);
///```
///# Parameters
/// - [`physical_device`] The physical device to query the display handle on.
/// - [`dpy`] A connection to the X11 server from which [`rr_output`] was queried.
/// - [`rr_output`] An X11 RandR output ID.
/// - [`p_display`] The corresponding [`DisplayKHR`] handle will be returned here.
///# Description
///If there is no [`DisplayKHR`] corresponding to [`rr_output`] on
///[`physical_device`], [`crate::utils::Handle::null`] **must**  be returned in
///[`p_display`].
///## Valid Usage (Implicit)
/// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
/// - [`dpy`] **must**  be a valid pointer to a [`Display`] value
/// - [`p_display`] **must**  be a valid pointer to a [`DisplayKHR`] handle
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`
///# Related
/// - [`VK_EXT_acquire_xlib_display`]
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
#[doc(alias = "vkGetRandROutputDisplayEXT")]
pub type FNGetRandROutputDisplayExt = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        dpy: *mut Display,
        rr_output: RROutput,
        p_display: *mut DisplayKHR,
    ) -> VulkanResultCodes,
>;
///The V-table of [`Instance`] for functions from VK_EXT_acquire_xlib_display
pub struct InstanceExtAcquireXlibDisplayVTable {
    ///See [`FNAcquireXlibDisplayExt`] for more information.
    pub acquire_xlib_display_ext: FNAcquireXlibDisplayExt,
    ///See [`FNGetRandROutputDisplayExt`] for more information.
    pub get_rand_r_output_display_ext: FNGetRandROutputDisplayExt,
}
impl InstanceExtAcquireXlibDisplayVTable {
    ///Loads the VTable from the owner and the names
    pub fn load<F>(loader_fn: F, loader: Instance) -> Self
    where
        F: Fn(Instance, &'static CStr) -> Option<extern "system" fn()>,
    {
        Self {
            acquire_xlib_display_ext: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkAcquireXlibDisplayEXT")))
            },
            get_rand_r_output_display_ext: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkGetRandROutputDisplayEXT")))
            },
        }
    }
    ///Gets [`Self::acquire_xlib_display_ext`]. See [`FNAcquireXlibDisplayExt`] for more
    /// information.
    pub fn acquire_xlib_display_ext(&self) -> FNAcquireXlibDisplayExt {
        self.acquire_xlib_display_ext
    }
    ///Gets [`Self::get_rand_r_output_display_ext`]. See [`FNGetRandROutputDisplayExt`] for more
    /// information.
    pub fn get_rand_r_output_display_ext(&self) -> FNGetRandROutputDisplayExt {
        self.get_rand_r_output_display_ext
    }
}
