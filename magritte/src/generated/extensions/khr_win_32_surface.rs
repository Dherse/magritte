//![VK_KHR_win32_surface](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_win32_surface.html) - instance extension
//!# Description
//!The [`VK_KHR_win32_surface`] extension is an instance extension.
//!It provides a mechanism to create a [`SurfaceKHR`] object (defined by
//!the `[`VK_KHR_surface`]` extension) that refers to a Win32 [`HWND`], as
//!well as a query to determine support for rendering to the windows desktop.
//!# Revision
//!6
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_surface`]`
//!# Contacts
//! - Jesse Hall [critsec](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_win32_surface]
//!   @critsec%0A<<Here describe the issue or question you have about the VK_KHR_win32_surface
//!   extension>>)
//! - Ian Elliott [ianelliottus](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_win32_surface]
//!   @ianelliottus%0A<<Here describe the issue or question you have about the VK_KHR_win32_surface
//!   extension>>)
//!# New functions & commands
//! - [`CreateWin32SurfaceKHR`]
//! - [`GetPhysicalDeviceWin32PresentationSupportKHR`]
//!# New structures
//! - [`Win32SurfaceCreateInfoKHR`]
//!# New bitmasks
//! - [`Win32SurfaceCreateFlagsKHR`]
//!# New constants
//! - [`KHR_WIN32_SURFACE_EXTENSION_NAME`]
//! - [`KHR_WIN32_SURFACE_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR`
//!# Known issues & F.A.Q
//!1) Does Win32 need a way to query for compatibility between a particular
//!physical device and a specific screen? Compatibility between a physical
//!device and a window generally only depends on what screen the window is on.
//!However, there is not an obvious way to identify a screen without already
//!having a window on the screen. **RESOLVED** : No.
//!While it may be useful, there is not a clear way to do this on Win32.
//!However, a method was added to query support for presenting to the windows
//!desktop as a whole.2) If a native window object ([`HWND`]) is used by one graphics API, and
//!then is later used by a different graphics API (one of which is Vulkan), can
//!these uses interfere with each other? **RESOLVED** : Yes.Uses of a window object by multiple
//! graphics APIs results in undefined
//!behavior.
//!Such behavior may succeed when using one Vulkan implementation but fail when
//!using a different Vulkan implementation.
//!Potential failures include:
//! - Creating then destroying a flip presentation model DXGI swapchain on a window object can
//!   prevent [`CreateSwapchainKHR`] from succeeding on the same window object.
//! - Creating then destroying a [`SwapchainKHR`] on a window object can prevent creation of a
//!   bitblt model DXGI swapchain on the same window object.
//! - Creating then destroying a [`SwapchainKHR`] on a window object can effectively
//!   `SetPixelFormat` to a different format than the format chosen by an OpenGL application.
//! - Creating then destroying a [`SwapchainKHR`] on a window object on one [`PhysicalDevice`] can
//!   prevent [`CreateSwapchainKHR`] from succeeding on the same window object, but on a different
//!   [`PhysicalDevice`] that is associated with a different Vulkan ICD.
//!In all cases the problem can be worked around by creating a new window
//!object.Technical details include:
//! - Creating a DXGI swapchain over a window object can alter the object for the remainder of its
//!   lifetime. The alteration persists even after the DXGI swapchain has been destroyed. This
//!   alteration can make it impossible for a conformant Vulkan implementation to create a
//!   [`SwapchainKHR`] over the same window object. Mention of this alteration can be found in the
//!   remarks section of the MSDN documentation for `DXGI_SWAP_EFFECT`.
//! - Calling GDI’s `SetPixelFormat` (needed by OpenGL’s WGL layer) on a window object alters the
//!   object for the remainder of its lifetime. The MSDN documentation for `SetPixelFormat` explains
//!   that a window object’s pixel format can be set only one time.
//! - Creating a [`SwapchainKHR`] over a window object can alter the object for its remaining
//!   lifetime. Either of the above alterations may occur as a side effect of
//!   [`CreateSwapchainKHR`].
//!# Version History
//! - Revision 1, 2015-09-23 (Jesse Hall)  - Initial draft, based on the previous contents of
//!   VK_EXT_KHR_swapchain (later renamed VK_EXT_KHR_surface).
//! - Revision 2, 2015-10-02 (James Jones)  - Added presentation support query for win32 desktops.
//! - Revision 3, 2015-10-26 (Ian Elliott)  - Renamed from VK_EXT_KHR_win32_surface to
//!   VK_KHR_win32_surface.
//! - Revision 4, 2015-11-03 (Daniel Rakos)  - Added allocation callbacks to
//!   vkCreateWin32SurfaceKHR.
//! - Revision 5, 2015-11-28 (Daniel Rakos)  - Updated the surface create function to take a
//!   pCreateInfo structure.
//! - Revision 6, 2017-04-24 (Jeff Juliano)  - Add issue 2 addressing reuse of a native window
//!   object in a different Graphics API, or by a different Vulkan ICD.
//!# Other info
//! * 2017-04-24
//! * No known IP claims.
//! * - Patrick Doane, Blizzard  - Jason Ekstrand, Intel  - Ian Elliott, LunarG  - Courtney
//!   Goeltzenleuchter, LunarG  - Jesse Hall, Google  - James Jones, NVIDIA  - Antoine Labour,
//!   Google  - Jon Leech, Khronos  - David Mao, AMD  - Norbert Nopper, Freescale  - Alon Or-bach,
//!   Samsung  - Daniel Rakos, AMD  - Graham Sellers, AMD  - Ray Smith, ARM  - Jeff Vigil, Qualcomm
//!   - Chia-I Wu, LunarG
//!# Related
//! - [`Win32SurfaceCreateFlagsKHR`]
//! - [`Win32SurfaceCreateInfoKHR`]
//! - [`CreateWin32SurfaceKHR`]
//! - [`GetPhysicalDeviceWin32PresentationSupportKHR`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    native::{HINSTANCE, HWND},
    vulkan1_0::{BaseInStructure, StructureType},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_WIN32_SURFACE_SPEC_VERSION")]
pub const KHR_WIN32_SURFACE_SPEC_VERSION: u32 = 6;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_WIN32_SURFACE_EXTENSION_NAME")]
pub const KHR_WIN32_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_win32_surface");
///[VkWin32SurfaceCreateFlagsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkWin32SurfaceCreateFlagsKHR.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_KHR_win32_surface
///typedef VkFlags VkWin32SurfaceCreateFlagsKHR;
///```
///# Related
/// - [`VK_KHR_win32_surface`]
/// - [`Win32SurfaceCreateInfoKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct Win32SurfaceCreateFlagsKHR(u32);
impl const Default for Win32SurfaceCreateFlagsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for Win32SurfaceCreateFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(Win32SurfaceCreateFlagsKHR))
            .field(&self.0)
            .finish()
    }
}
///[VkWin32SurfaceCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkWin32SurfaceCreateInfoKHR.html) - Structure specifying parameters of a newly created Win32 surface object
///# C Specifications
///The [`Win32SurfaceCreateInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_win32_surface
///typedef struct VkWin32SurfaceCreateInfoKHR {
///    VkStructureType                 sType;
///    const void*                     pNext;
///    VkWin32SurfaceCreateFlagsKHR    flags;
///    HINSTANCE                       hinstance;
///    HWND                            hwnd;
///} VkWin32SurfaceCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`hinstance`] is the Win32 [`HINSTANCE`] for the window to associate the surface with.
/// - [`hwnd`] is the Win32 [`HWND`] for the window to associate the surface with.
///# Description
///## Valid Usage
/// - [`hinstance`] **must**  be a valid Win32 [`HINSTANCE`]
/// - [`hwnd`] **must**  be a valid Win32 [`HWND`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`flags`] **must**  be `0`
///# Related
/// - [`VK_KHR_win32_surface`]
/// - [`StructureType`]
/// - [`Win32SurfaceCreateFlagsKHR`]
/// - [`CreateWin32SurfaceKHR`]
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
pub struct Win32SurfaceCreateInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: Win32SurfaceCreateFlagsKHR,
    ///[`hinstance`] is the Win32 [`HINSTANCE`] for the window to associate
    ///the surface with.
    hinstance: HINSTANCE,
    ///[`hwnd`] is the Win32 [`HWND`] for the window to associate the
    ///surface with.
    hwnd: HWND,
}
impl<'lt> Default for Win32SurfaceCreateInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
            hinstance: Default::default(),
            hwnd: Default::default(),
        }
    }
}
impl<'lt> Win32SurfaceCreateInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::hinstance`]
    pub fn hinstance_raw(&self) -> &HINSTANCE {
        &self.hinstance
    }
    ///Gets the raw value of [`Self::hwnd`]
    pub fn hwnd_raw(&self) -> &HWND {
        &self.hwnd
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::hinstance`]
    pub fn set_hinstance_raw(&mut self, value: HINSTANCE) -> &mut Self {
        self.hinstance = value;
        self
    }
    ///Sets the raw value of [`Self::hwnd`]
    pub fn set_hwnd_raw(&mut self, value: HWND) -> &mut Self {
        self.hwnd = value;
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
    pub fn flags(&self) -> Win32SurfaceCreateFlagsKHR {
        self.flags
    }
    ///Gets the value of [`Self::hinstance`]
    pub fn hinstance(&self) -> &HINSTANCE {
        &self.hinstance
    }
    ///Gets the value of [`Self::hwnd`]
    pub fn hwnd(&self) -> &HWND {
        &self.hwnd
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut Win32SurfaceCreateFlagsKHR {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::hinstance`]
    pub fn hinstance_mut(&mut self) -> &mut HINSTANCE {
        &mut self.hinstance
    }
    ///Gets a mutable reference to the value of [`Self::hwnd`]
    pub fn hwnd_mut(&mut self) -> &mut HWND {
        &mut self.hwnd
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
    pub fn set_flags(&mut self, value: crate::extensions::khr_win_32_surface::Win32SurfaceCreateFlagsKHR) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::hinstance`]
    pub fn set_hinstance(&mut self, value: crate::native::HINSTANCE) -> &mut Self {
        self.hinstance = value;
        self
    }
    ///Sets the raw value of [`Self::hwnd`]
    pub fn set_hwnd(&mut self, value: crate::native::HWND) -> &mut Self {
        self.hwnd = value;
        self
    }
}
