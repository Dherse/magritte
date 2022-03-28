//![VK_KHR_wayland_surface](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_wayland_surface.html) - instance extension
//!# Description
//!The [`VK_KHR_wayland_surface`] extension is an instance extension.
//!It provides a mechanism to create a [`SurfaceKHR`] object (defined by
//!the `[`VK_KHR_surface`]` extension) that refers to a Wayland
//![`wl_surface`], as well as a query to determine support for rendering to a
//!Wayland compositor.
//!# Revision
//!6
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_surface`]`
//!# Contacts
//! - Jesse Hall [critsec](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_wayland_surface]
//!   @critsec%0A<<Here describe the issue or question you have about the VK_KHR_wayland_surface
//!   extension>>)
//! - Ian Elliott [ianelliottus](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_wayland_surface]
//!   @ianelliottus%0A<<Here describe the issue or question you have about the
//!   VK_KHR_wayland_surface extension>>)
//!# New functions & commands
//! - [`CreateWaylandSurfaceKHR`]
//! - [`GetPhysicalDeviceWaylandPresentationSupportKHR`]
//!# New structures
//! - [`WaylandSurfaceCreateInfoKHR`]
//!# New bitmasks
//! - [`WaylandSurfaceCreateFlagsKHR`]
//!# New constants
//! - [`KHR_WAYLAND_SURFACE_EXTENSION_NAME`]
//! - [`KHR_WAYLAND_SURFACE_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR`
//!# Known issues & F.A.Q
//!1) Does Wayland need a way to query for compatibility between a particular
//!physical device and a specific Wayland display? This would be a more general
//!query than [`GetPhysicalDeviceSurfaceSupportKHR`]: if the
//!Wayland-specific query returned [`TRUE`] for a ([`PhysicalDevice`],
//!`struct wl_display*`) pair, then the physical device could be assumed to
//!support presentation to any [`SurfaceKHR`] for surfaces on the display. **RESOLVED** : Yes.
//![`GetPhysicalDeviceWaylandPresentationSupportKHR`] was added to address
//!this issue.2) Should we require surfaces created with [`CreateWaylandSurfaceKHR`]
//!to support the `VK_PRESENT_MODE_MAILBOX_KHR` present mode? **RESOLVED** : Yes.
//!Wayland is an inherently mailbox window system and mailbox support is
//!required for some Wayland compositor interactions to work as expected.
//!While handling these interactions may be possible with
//!`VK_PRESENT_MODE_FIFO_KHR`, it is much more difficult to do without
//!deadlock and requiring all Wayland applications to be able to support
//!implementations which only support `VK_PRESENT_MODE_FIFO_KHR` would be
//!an onerous restriction on application developers.
//!# Version History
//! - Revision 1, 2015-09-23 (Jesse Hall)  - Initial draft, based on the previous contents of
//!   VK_EXT_KHR_swapchain (later renamed VK_EXT_KHR_surface).
//! - Revision 2, 2015-10-02 (James Jones)  - Added
//!   vkGetPhysicalDeviceWaylandPresentationSupportKHR() to resolve issue #1.  - Adjusted wording of
//!   issue #1 to match the agreed-upon solution.  - Renamed “window” parameters to “surface” to
//!   match Wayland conventions.
//! - Revision 3, 2015-10-26 (Ian Elliott)  - Renamed from VK_EXT_KHR_wayland_surface to
//!   VK_KHR_wayland_surface.
//! - Revision 4, 2015-11-03 (Daniel Rakos)  - Added allocation callbacks to
//!   vkCreateWaylandSurfaceKHR.
//! - Revision 5, 2015-11-28 (Daniel Rakos)  - Updated the surface create function to take a
//!   pCreateInfo structure.
//! - Revision 6, 2017-02-08 (Jason Ekstrand)  - Added the requirement that implementations support
//!   `VK_PRESENT_MODE_MAILBOX_KHR`.  - Added wording about interactions between [`QueuePresentKHR`]
//!   and the Wayland requests sent to the compositor.
//!# Other info
//! * 2015-11-28
//! * No known IP claims.
//! * - Patrick Doane, Blizzard  - Jason Ekstrand, Intel  - Ian Elliott, LunarG  - Courtney
//!   Goeltzenleuchter, LunarG  - Jesse Hall, Google  - James Jones, NVIDIA  - Antoine Labour,
//!   Google  - Jon Leech, Khronos  - David Mao, AMD  - Norbert Nopper, Freescale  - Alon Or-bach,
//!   Samsung  - Daniel Rakos, AMD  - Graham Sellers, AMD  - Ray Smith, ARM  - Jeff Vigil, Qualcomm
//!   - Chia-I Wu, LunarG
//!# Related
//! - [`WaylandSurfaceCreateFlagsKHR`]
//! - [`WaylandSurfaceCreateInfoKHR`]
//! - [`CreateWaylandSurfaceKHR`]
//! - [`GetPhysicalDeviceWaylandPresentationSupportKHR`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    native::{wl_display, wl_surface},
    vulkan1_0::{BaseInStructure, StructureType},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_WAYLAND_SURFACE_SPEC_VERSION")]
pub const KHR_WAYLAND_SURFACE_SPEC_VERSION: u32 = 6;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME")]
pub const KHR_WAYLAND_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_wayland_surface");
///[VkWaylandSurfaceCreateFlagsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkWaylandSurfaceCreateFlagsKHR.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_KHR_wayland_surface
///typedef VkFlags VkWaylandSurfaceCreateFlagsKHR;
///```
///# Related
/// - [`VK_KHR_wayland_surface`]
/// - [`WaylandSurfaceCreateInfoKHR`]
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
pub struct WaylandSurfaceCreateFlagsKHR(u32);
impl const Default for WaylandSurfaceCreateFlagsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for WaylandSurfaceCreateFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(WaylandSurfaceCreateFlagsKHR))
            .field(&self.0)
            .finish()
    }
}
///[VkWaylandSurfaceCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkWaylandSurfaceCreateInfoKHR.html) - Structure specifying parameters of a newly created Wayland surface object
///# C Specifications
///The [`WaylandSurfaceCreateInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_wayland_surface
///typedef struct VkWaylandSurfaceCreateInfoKHR {
///    VkStructureType                   sType;
///    const void*                       pNext;
///    VkWaylandSurfaceCreateFlagsKHR    flags;
///    struct wl_display*                display;
///    struct wl_surface*                surface;
///} VkWaylandSurfaceCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`display`] and [`surface`] are pointers to the Wayland [`wl_display`] and [`wl_surface`] to
///   associate the surface with.
///# Description
///## Valid Usage
/// - [`display`] **must**  point to a valid Wayland [`wl_display`]
/// - [`surface`] **must**  point to a valid Wayland [`wl_surface`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`flags`] **must**  be `0`
///# Related
/// - [`VK_KHR_wayland_surface`]
/// - [`StructureType`]
/// - [`WaylandSurfaceCreateFlagsKHR`]
/// - [`CreateWaylandSurfaceKHR`]
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
pub struct WaylandSurfaceCreateInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: WaylandSurfaceCreateFlagsKHR,
    ///[`display`] and [`surface`] are pointers to the Wayland
    ///[`wl_display`] and [`wl_surface`] to associate the surface with.
    display: *mut wl_display,
    ///No documentation found
    surface: *mut wl_surface,
}
impl<'lt> Default for WaylandSurfaceCreateInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
            display: std::ptr::null_mut(),
            surface: std::ptr::null_mut(),
        }
    }
}
impl<'lt> WaylandSurfaceCreateInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
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
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> WaylandSurfaceCreateFlagsKHR {
        self.flags
    }
    ///Gets the value of [`Self::display`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn display(&self) -> &wl_display {
        &*self.display
    }
    ///Gets the value of [`Self::surface`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn surface(&self) -> &wl_surface {
        &*self.surface
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut WaylandSurfaceCreateFlagsKHR {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::display`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn display_mut(&mut self) -> &mut wl_display {
        &mut *self.display
    }
    ///Gets a mutable reference to the value of [`Self::surface`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn surface_mut(&mut self) -> &mut wl_surface {
        &mut *self.surface
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
    pub fn set_flags(
        &mut self,
        value: crate::extensions::khr_wayland_surface::WaylandSurfaceCreateFlagsKHR,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::display`]
    pub fn set_display(&mut self, value: &'lt mut crate::native::wl_display) -> &mut Self {
        self.display = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::surface`]
    pub fn set_surface(&mut self, value: &'lt mut crate::native::wl_surface) -> &mut Self {
        self.surface = value as *mut _;
        self
    }
}
