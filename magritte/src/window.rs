#![warn(trivial_casts, trivial_numeric_casts)]

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

use crate::{
    extensions::{self, khr_surface::SurfaceKHR},
    vulkan1_0::{AllocationCallbacks, Instance, VulkanResultCodes},
    Extensions, Unique, VulkanResult,
};

/// Create a surface from a raw surface handle.
///
/// `instance` must have created with platform specific surface extensions enabled.
///
/// # Safety
///
/// In order for the created [`SurfaceKHR`] to be valid for the duration of its
/// usage, the [`Instance`] this was called on must be dropped later than the
/// resulting [`SurfaceKHR`].
pub unsafe fn create_surface(
    instance: &Unique<Instance>,
    window_handle: &dyn HasRawWindowHandle,
    allocation_callbacks: Option<&AllocationCallbacks>,
) -> VulkanResult<Unique<SurfaceKHR>> {
    match window_handle.raw_window_handle() {
        #[cfg(target_os = "windows")]
        RawWindowHandle::Win32(handle) => {
            let surface_desc = extensions::khr_win_32_surface::Win32SurfaceCreateInfoKHR::default()
                .set_hinstance(std::mem::transmute(handle.hinstance))
                .set_hwnd(std::mem::transmute(handle.hwnd));

            instance.create_win32_surface_khr(&surface_desc, allocation_callbacks)
        },

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Wayland(handle) => {
            let surface_desc = extensions::khr_wayland_surface::WaylandSurfaceCreateInfoKHR::default()
                .set_display(handle.display)
                .set_surface(handle.surface);

            instance.create_wayland_surface_khr(&surface_desc, allocation_callbacks)
        },

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Xlib(handle) => {
            let surface_desc = extensions::khr_xlib_surface::XlibSurfaceCreateInfoKHR::default()
                .set_dpy(handle.display)
                .set_window(handle.window);

            instance.create_xlib_surface_khr(&surface_desc, allocation_callbacks)
        },

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Xcb(handle) => {
            let surface_desc = extensions::khr_xcb_surface::XcbSurfaceCreateInfoKHR::default()
                .set_connection(handle.connection)
                .set_window(handle.window);

            instance.create_xcb_surface_khr(&surface_desc, allocation_callbacks)
        },

        #[cfg(any(target_os = "android"))]
        RawWindowHandle::Android(handle) => {
            let surface_desc =
                extensions::khr_android_surface::AndroidSurfaceCreateInfoKHR::default().window(handle.a_native_window);

            instance.create_android_surface(&surface_desc, allocation_callbacks)
        },

        #[cfg(any(target_os = "macos"))]
        RawWindowHandle::MacOS(handle) => {
            todo!()
        },

        #[cfg(any(target_os = "ios"))]
        RawWindowHandle::IOS(handle) => {
            todo!()
        },

        _ => VulkanResult::Err(VulkanResultCodes::ERROR_EXTENSION_NOT_PRESENT), // not supported
    }
}

/// Query the required instance extensions for creating a surface from a window handle.
///
/// The returned extensions will include all extension dependencies.
pub fn enable_required_extensions<W: HasRawWindowHandle>(
    window_handle: W,
    extensions: Extensions,
) -> Result<Extensions, VulkanResultCodes> {
    let extensions = match window_handle.raw_window_handle() {
        #[cfg(target_os = "windows")]
        RawWindowHandle::Win32(_) => extensions.enable_khr_surface().enable_khr_win_32_surface(),

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Wayland(_) => extensions.enable_khr_surface().enable_khr_wayland_surface(),

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Xlib(_) => extensions.enable_khr_surface().enable_khr_xlib_surface(),

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Xcb(_) => extensions.enable_khr_surface().enable_khr_xcb_surface(),

        #[cfg(any(target_os = "android"))]
        RawWindowHandle::Android(_) => extensions.enable_khr_surface().enable_khr_android_surface(),

        #[cfg(any(target_os = "macos"))]
        RawWindowHandle::MacOS(_) => extensions.enable_khr_surface().enable_mvk_macos_surface(),

        #[cfg(any(target_os = "ios"))]
        RawWindowHandle::IOS(_) => extensions.enable_khr_surface().enable_ext_metal_surface(),

        _ => return Err(VulkanResultCodes::ERROR_EXTENSION_NOT_PRESENT),
    };

    Ok(extensions)
}
