//![VK_KHR_xlib_surface](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_xlib_surface.html) - instance extension
//!# Description
//!The [`VK_KHR_xlib_surface`] extension is an instance extension.
//!It provides a mechanism to create a [`SurfaceKHR`] object (defined by
//!the `[`VK_KHR_surface`]` extension) that refers to an X11 [`Window`],
//!using the Xlib client-side library, as well as a query to determine support
//!for rendering via Xlib.
//!# Revision
//!6
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_surface`]`
//!# Contacts
//! - Jesse Hall [critsec](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_xlib_surface]
//!   @critsec%0A<<Here describe the issue or question you have about the VK_KHR_xlib_surface
//!   extension>>)
//! - Ian Elliott [ianelliottus](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_xlib_surface]
//!   @ianelliottus%0A<<Here describe the issue or question you have about the VK_KHR_xlib_surface
//!   extension>>)
//!# New functions & commands
//! - [`CreateXlibSurfaceKHR`]
//! - [`GetPhysicalDeviceXlibPresentationSupportKHR`]
//!# New structures
//! - [`XlibSurfaceCreateInfoKHR`]
//!# New bitmasks
//! - [`XlibSurfaceCreateFlagsKHR`]
//!# New constants
//! - [`KHR_XLIB_SURFACE_EXTENSION_NAME`]
//! - [`KHR_XLIB_SURFACE_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR`
//!# Known issues & F.A.Q
//!1) Does X11 need a way to query for compatibility between a particular
//!physical device and a specific screen? This would be a more general query
//!than [`GetPhysicalDeviceSurfaceSupportKHR`]; if it returned
//![`TRUE`], then the physical device could be assumed to support
//!presentation to any window on that screen. **RESOLVED** : Yes, this is needed for toolkits that
//! want to create a
//![`Device`] before creating a window.
//!To ensure the query is reliable, it must be made against a particular X
//!visual rather than the screen in general.
//!# Version History
//! - Revision 1, 2015-09-23 (Jesse Hall)  - Initial draft, based on the previous contents of
//!   VK_EXT_KHR_swapchain (later renamed VK_EXT_KHR_surface).
//! - Revision 2, 2015-10-02 (James Jones)  - Added presentation support query for (Display*,
//!   VisualID) pair.  - Removed “root” parameter from CreateXlibSurfaceKHR(), as it is redundant
//!   when a window on the same screen is specified as well.  - Added appropriate X errors.  -
//!   Adjusted wording of issue #1 and added agreed upon resolution.
//! - Revision 3, 2015-10-14 (Ian Elliott)  - Renamed this extension from VK_EXT_KHR_x11_surface to
//!   VK_EXT_KHR_xlib_surface.
//! - Revision 4, 2015-10-26 (Ian Elliott)  - Renamed from VK_EXT_KHR_xlib_surface to
//!   VK_KHR_xlib_surface.
//! - Revision 5, 2015-11-03 (Daniel Rakos)  - Added allocation callbacks to vkCreateXlibSurfaceKHR.
//! - Revision 6, 2015-11-28 (Daniel Rakos)  - Updated the surface create function to take a
//!   pCreateInfo structure.
//!# Other info
//! * 2015-11-28
//! * No known IP claims.
//! * - Patrick Doane, Blizzard  - Jason Ekstrand, Intel  - Ian Elliott, LunarG  - Courtney
//!   Goeltzenleuchter, LunarG  - Jesse Hall, Google  - James Jones, NVIDIA  - Antoine Labour,
//!   Google  - Jon Leech, Khronos  - David Mao, AMD  - Norbert Nopper, Freescale  - Alon Or-bach,
//!   Samsung  - Daniel Rakos, AMD  - Graham Sellers, AMD  - Ray Smith, ARM  - Jeff Vigil, Qualcomm
//!   - Chia-I Wu, LunarG
//!# Related
//! - [`XlibSurfaceCreateFlagsKHR`]
//! - [`XlibSurfaceCreateInfoKHR`]
//! - [`CreateXlibSurfaceKHR`]
//! - [`GetPhysicalDeviceXlibPresentationSupportKHR`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    native::{Display, Window},
    vulkan1_0::{BaseInStructure, StructureType},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_XLIB_SURFACE_SPEC_VERSION")]
pub const KHR_XLIB_SURFACE_SPEC_VERSION: u32 = 6;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_XLIB_SURFACE_EXTENSION_NAME")]
pub const KHR_XLIB_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_xlib_surface");
///[VkXlibSurfaceCreateFlagsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkXlibSurfaceCreateFlagsKHR.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_KHR_xlib_surface
///typedef VkFlags VkXlibSurfaceCreateFlagsKHR;
///```
///# Related
/// - [`VK_KHR_xlib_surface`]
/// - [`XlibSurfaceCreateInfoKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct XlibSurfaceCreateFlagsKHR(u32);
impl const Default for XlibSurfaceCreateFlagsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for XlibSurfaceCreateFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(XlibSurfaceCreateFlagsKHR))
            .field(&self.0)
            .finish()
    }
}
///[VkXlibSurfaceCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkXlibSurfaceCreateInfoKHR.html) - Structure specifying parameters of a newly created Xlib surface object
///# C Specifications
///The [`XlibSurfaceCreateInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_xlib_surface
///typedef struct VkXlibSurfaceCreateInfoKHR {
///    VkStructureType                sType;
///    const void*                    pNext;
///    VkXlibSurfaceCreateFlagsKHR    flags;
///    Display*                       dpy;
///    Window                         window;
///} VkXlibSurfaceCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`dpy`] is a pointer to an Xlib [`Display`] connection to the X server.
/// - [`window`] is an Xlib [`Window`] to associate the surface with.
///# Description
///## Valid Usage
/// - [`dpy`] **must**  point to a valid Xlib [`Display`]
/// - [`window`] **must**  be a valid Xlib [`Window`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`flags`] **must**  be `0`
///# Related
/// - [`VK_KHR_xlib_surface`]
/// - [`StructureType`]
/// - [`XlibSurfaceCreateFlagsKHR`]
/// - [`CreateXlibSurfaceKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkXlibSurfaceCreateInfoKHR")]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct XlibSurfaceCreateInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    pub flags: XlibSurfaceCreateFlagsKHR,
    ///[`dpy`] is a pointer to an Xlib [`Display`] connection to the X
    ///server.
    pub dpy: *mut Display,
    ///[`window`] is an Xlib [`Window`] to associate the surface with.
    pub window: Window,
}
impl<'lt> Default for XlibSurfaceCreateInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::XlibSurfaceCreateInfoKhr,
            p_next: std::ptr::null(),
            flags: Default::default(),
            dpy: std::ptr::null_mut(),
            window: unsafe { std::mem::zeroed() },
        }
    }
}
impl<'lt> XlibSurfaceCreateInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::window`]
    pub fn window_raw(&self) -> &Window {
        &self.window
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::window`]
    pub fn set_window_raw(&mut self, value: Window) -> &mut Self {
        self.window = value;
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
    pub fn flags(&self) -> XlibSurfaceCreateFlagsKHR {
        self.flags
    }
    ///Gets the value of [`Self::dpy`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn dpy(&self) -> &Display {
        &*self.dpy
    }
    ///Gets the value of [`Self::window`]
    pub fn window(&self) -> &Window {
        &self.window
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut XlibSurfaceCreateFlagsKHR {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::dpy`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn dpy_mut(&mut self) -> &mut Display {
        &mut *self.dpy
    }
    ///Gets a mutable reference to the value of [`Self::window`]
    pub fn window_mut(&mut self) -> &mut Window {
        &mut self.window
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
    pub fn set_flags(&mut self, value: crate::extensions::khr_xlib_surface::XlibSurfaceCreateFlagsKHR) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::dpy`]
    pub fn set_dpy(&mut self, value: &'lt mut crate::native::Display) -> &mut Self {
        self.dpy = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::window`]
    pub fn set_window(&mut self, value: crate::native::Window) -> &mut Self {
        self.window = value;
        self
    }
}
