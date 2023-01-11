#![warn(trivial_casts, trivial_numeric_casts)]

pub use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle};

use crate::{
    extensions::{self, khr_surface::SurfaceKHR},
    vulkan1_0::{AllocationCallbacks, Instance, VulkanResultCodes},
    InstanceExtensions, Unique, VulkanResult,
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
    window_handle: &impl HasRawWindowHandle,
    display_handle: &impl HasRawDisplayHandle,
    allocation_callbacks: Option<&AllocationCallbacks>,
) -> VulkanResult<Unique<SurfaceKHR>> {
    match (window_handle.raw_window_handle(), display_handle.raw_display_handle()) {
        #[cfg(target_os = "windows")]
        (RawWindowHandle::Win32(handle), RawDisplayHandle::Windows(_)) => {
            let surface_desc = extensions::khr_win32_surface::Win32SurfaceCreateInfoKHR::default()
                .with_hinstance(std::mem::transmute(handle.hinstance))
                .with_hwnd(std::mem::transmute(handle.hwnd));

            instance.create_win32_surface_khr(&surface_desc, allocation_callbacks)
        },

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        (RawWindowHandle::Wayland(window), RawDisplayHandle::Wayland(display)) => {
            let surface_desc = extensions::khr_wayland_surface::WaylandSurfaceCreateInfoKHR::default()
                .with_display(display.display)
                .with_surface(window.surface);

            instance.create_wayland_surface_khr(&surface_desc, allocation_callbacks)
        },

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        (RawWindowHandle::Xlib(window), RawDisplayHandle::Xlib(display)) => {
            let surface_desc = extensions::khr_xlib_surface::XlibSurfaceCreateInfoKHR::default()
                .with_dpy(display.display)
                .with_window(window.window);

            instance.create_xlib_surface_khr(&surface_desc, allocation_callbacks)
        },

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        (RawWindowHandle::Xcb(window), RawDisplayHandle::Xcb(display)) => {
            let surface_desc = extensions::khr_xcb_surface::XcbSurfaceCreateInfoKHR::default()
                .with_connection(display.connection)
                .with_window(window.window);

            instance.create_xcb_surface_khr(&surface_desc, allocation_callbacks)
        },

        #[cfg(any(target_os = "android"))]
        (RawWindowHandle::AndroidNdk(window), RawDisplayHandle::Android(_)) => {
            let surface_desc =
                extensions::khr_android_surface::AndroidSurfaceCreateInfoKHR::default().with_window(&mut *window.a_native_window);

            instance.create_android_surface_khr(&surface_desc, allocation_callbacks)
        },

        #[cfg(any(target_os = "macos"))]
        (RawWindowHandle::AppKit(window), RawDisplayHandle::AppKit(_)) => {
            todo!()
        },

        #[cfg(any(target_os = "ios"))]
        (RawWindowHandle::UiKit(window), RawDisplayHandle::UiKit(_)) => {
            todo!()
        },

        _ => VulkanResult::Err(VulkanResultCodes::ERROR_EXTENSION_NOT_PRESENT), // not supported
    }
}

/// Query the required instance extensions for creating a surface from a window handle.
///
/// The returned extensions will include all extension dependencies.
pub fn enable_required_extensions(
    window_handle: impl HasRawWindowHandle,
    extensions: InstanceExtensions,
) -> Result<InstanceExtensions, VulkanResultCodes> {
    let extensions = match window_handle.raw_window_handle() {
        #[cfg(target_os = "windows")]
        RawWindowHandle::Win32(_) => extensions.enable_khr_surface().enable_khr_win32_surface(),

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
