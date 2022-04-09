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
//! - [`create_wayland_surface_khr`]
//! - [`get_physical_device_wayland_presentation_support_khr`]
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
//!query than [`get_physical_device_surface_support_khr`]: if the
//!Wayland-specific query returned [`TRUE`] for a ([`PhysicalDevice`],
//!`struct wl_display*`) pair, then the physical device could be assumed to
//!support presentation to any [`SurfaceKHR`] for surfaces on the display. **RESOLVED** : Yes.
//![`get_physical_device_wayland_presentation_support_khr`] was added to address
//!this issue.2) Should we require surfaces created with [`create_wayland_surface_khr`]
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
//!   `VK_PRESENT_MODE_MAILBOX_KHR`.  - Added wording about interactions between
//!   [`queue_present_khr`] and the Wayland requests sent to the compositor.
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
//! - [`create_wayland_surface_khr`]
//! - [`get_physical_device_wayland_presentation_support_khr`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    extensions::khr_surface::SurfaceKHR,
    native::{wl_display, wl_surface},
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, Bool32, Instance, PhysicalDevice, StructureType, VulkanResultCodes,
    },
    AsRaw, Unique, VulkanResult,
};
use std::{ffi::CStr, marker::PhantomData, mem::MaybeUninit, sync::atomic::AtomicBool};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_WAYLAND_SURFACE_SPEC_VERSION")]
pub const KHR_WAYLAND_SURFACE_SPEC_VERSION: u32 = 6;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME")]
pub const KHR_WAYLAND_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_wayland_surface");
///[vkCreateWaylandSurfaceKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateWaylandSurfaceKHR.html) - Create a slink:VkSurfaceKHR object for a Wayland window
///# C Specifications
///To create a [`SurfaceKHR`] object for a Wayland surface, call:
///```c
///// Provided by VK_KHR_wayland_surface
///VkResult vkCreateWaylandSurfaceKHR(
///    VkInstance                                  instance,
///    const VkWaylandSurfaceCreateInfoKHR*        pCreateInfo,
///    const VkAllocationCallbacks*                pAllocator,
///    VkSurfaceKHR*                               pSurface);
///```
/// # Parameters
/// - [`instance`] is the instance to associate the surface with.
/// - [`p_create_info`] is a pointer to a [`WaylandSurfaceCreateInfoKHR`] structure containing
///   parameters affecting the creation of the surface object.
/// - [`p_allocator`] is the allocator used for host memory allocated for the surface object when there is no more specific allocator available (see [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)).
/// - [`p_surface`] is a pointer to a [`SurfaceKHR`] handle in which the created surface object is
///   returned.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`instance`] **must**  be a valid [`Instance`] handle
/// - [`p_create_info`] **must**  be a valid pointer to a valid [`WaylandSurfaceCreateInfoKHR`]
///   structure
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
/// - [`p_surface`] **must**  be a valid pointer to a [`SurfaceKHR`] handle
///
/// ## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
/// # Related
/// - [`VK_KHR_wayland_surface`]
/// - [`AllocationCallbacks`]
/// - [`Instance`]
/// - [`SurfaceKHR`]
/// - [`WaylandSurfaceCreateInfoKHR`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCreateWaylandSurfaceKHR")]
pub type FNCreateWaylandSurfaceKhr = Option<
    for<'lt> unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const WaylandSurfaceCreateInfoKHR<'lt>,
        p_allocator: *const AllocationCallbacks<'lt>,
        p_surface: *mut SurfaceKHR,
    ) -> VulkanResultCodes,
>;
///[vkGetPhysicalDeviceWaylandPresentationSupportKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceWaylandPresentationSupportKHR.html) - Query physical device for presentation to Wayland
///# C Specifications
///To determine whether a queue family of a physical device supports
///presentation to a Wayland compositor, call:
///```c
///// Provided by VK_KHR_wayland_surface
///VkBool32 vkGetPhysicalDeviceWaylandPresentationSupportKHR(
///    VkPhysicalDevice                            physicalDevice,
///    uint32_t                                    queueFamilyIndex,
///    struct wl_display*                          display);
///```
/// # Parameters
/// - [`physical_device`] is the physical device.
/// - [`queue_family_index`] is the queue family index.
/// - [`display`] is a pointer to the [`wl_display`] associated with a Wayland compositor.
/// # Description
/// This platform-specific function  **can**  be called prior to creating a surface.
/// ## Valid Usage
/// - [`queue_family_index`] **must**  be less than `pQueueFamilyPropertyCount` returned by
///   [`get_physical_device_queue_family_properties`] for the given [`physical_device`]
///
/// ## Valid Usage (Implicit)
/// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
/// - [`display`] **must**  be a valid pointer to a [`wl_display`] value
/// # Related
/// - [`VK_KHR_wayland_surface`]
/// - [`PhysicalDevice`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetPhysicalDeviceWaylandPresentationSupportKHR")]
pub type FNGetPhysicalDeviceWaylandPresentationSupportKhr = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        display: *mut wl_display,
    ) -> Bool32,
>;
///[VkWaylandSurfaceCreateFlagsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkWaylandSurfaceCreateFlagsKHR.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_KHR_wayland_surface
///typedef VkFlags VkWaylandSurfaceCreateFlagsKHR;
///```
/// # Related
/// - [`VK_KHR_wayland_surface`]
/// - [`WaylandSurfaceCreateInfoKHR`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
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
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`display`] and [`surface`] are pointers to the Wayland [`wl_display`] and [`wl_surface`] to
///   associate the surface with.
/// # Description
/// ## Valid Usage
/// - [`display`] **must**  point to a valid Wayland [`wl_display`]
/// - [`surface`] **must**  point to a valid Wayland [`wl_surface`]
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`flags`] **must**  be `0`
/// # Related
/// - [`VK_KHR_wayland_surface`]
/// - [`StructureType`]
/// - [`WaylandSurfaceCreateFlagsKHR`]
/// - [`create_wayland_surface_khr`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkWaylandSurfaceCreateInfoKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct WaylandSurfaceCreateInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    pub flags: WaylandSurfaceCreateFlagsKHR,
    ///[`display`] and [`surface`] are pointers to the Wayland
    ///[`wl_display`] and [`wl_surface`] to associate the surface with.
    pub display: *mut wl_display,
    ///No documentation found
    pub surface: *mut wl_surface,
}
impl<'lt> Default for WaylandSurfaceCreateInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::WAYLAND_SURFACE_CREATE_INFO_KHR,
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
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    pub fn display(&self) -> *mut wl_display {
        self.display
    }
    ///Gets the value of [`Self::surface`]
    pub fn surface(&self) -> *mut wl_surface {
        self.surface
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
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn set_flags(mut self, value: crate::extensions::khr_wayland_surface::WaylandSurfaceCreateFlagsKHR) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::display`]
    pub fn set_display(mut self, value: *mut crate::native::wl_display) -> Self {
        self.display = value;
        self
    }
    ///Sets the value of [`Self::surface`]
    pub fn set_surface(mut self, value: *mut crate::native::wl_surface) -> Self {
        self.surface = value;
        self
    }
}
impl Instance {
    ///[vkCreateWaylandSurfaceKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateWaylandSurfaceKHR.html) - Create a slink:VkSurfaceKHR object for a Wayland window
    ///# C Specifications
    ///To create a [`SurfaceKHR`] object for a Wayland surface, call:
    ///```c
    ///// Provided by VK_KHR_wayland_surface
    ///VkResult vkCreateWaylandSurfaceKHR(
    ///    VkInstance                                  instance,
    ///    const VkWaylandSurfaceCreateInfoKHR*        pCreateInfo,
    ///    const VkAllocationCallbacks*                pAllocator,
    ///    VkSurfaceKHR*                               pSurface);
    ///```
    /// # Parameters
    /// - [`instance`] is the instance to associate the surface with.
    /// - [`p_create_info`] is a pointer to a [`WaylandSurfaceCreateInfoKHR`] structure containing
    ///   parameters affecting the creation of the surface object.
    /// - [`p_allocator`] is the allocator used for host memory allocated for the surface object when there is no more specific allocator available (see [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)).
    /// - [`p_surface`] is a pointer to a [`SurfaceKHR`] handle in which the created surface object
    ///   is returned.
    /// # Description
    /// ## Valid Usage (Implicit)
    /// - [`instance`] **must**  be a valid [`Instance`] handle
    /// - [`p_create_info`] **must**  be a valid pointer to a valid [`WaylandSurfaceCreateInfoKHR`]
    ///   structure
    /// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
    ///   [`AllocationCallbacks`] structure
    /// - [`p_surface`] **must**  be a valid pointer to a [`SurfaceKHR`] handle
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// # Related
    /// - [`VK_KHR_wayland_surface`]
    /// - [`AllocationCallbacks`]
    /// - [`Instance`]
    /// - [`SurfaceKHR`]
    /// - [`WaylandSurfaceCreateInfoKHR`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCreateWaylandSurfaceKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn create_wayland_surface_khr<'lt>(
        self: &Unique<Instance>,
        p_create_info: &WaylandSurfaceCreateInfoKHR<'lt>,
        p_allocator: Option<&AllocationCallbacks<'lt>>,
    ) -> VulkanResult<Unique<SurfaceKHR>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_wayland_surface()
            .and_then(|vtable| vtable.create_wayland_surface_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_wayland_surface()
            .and_then(|vtable| vtable.create_wayland_surface_khr())
            .unwrap_unchecked();
        let mut p_surface = MaybeUninit::<SurfaceKHR>::uninit();
        let _return = _function(
            self.as_raw(),
            p_create_info as *const WaylandSurfaceCreateInfoKHR<'lt>,
            p_allocator
                .map(|v| v as *const AllocationCallbacks<'lt>)
                .unwrap_or_else(std::ptr::null),
            p_surface.as_mut_ptr(),
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(
                _return,
                Unique::new(self, p_surface.assume_init(), AtomicBool::default()),
            ),
            e => VulkanResult::Err(e),
        }
    }
}
impl PhysicalDevice {
    ///[vkGetPhysicalDeviceWaylandPresentationSupportKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceWaylandPresentationSupportKHR.html) - Query physical device for presentation to Wayland
    ///# C Specifications
    ///To determine whether a queue family of a physical device supports
    ///presentation to a Wayland compositor, call:
    ///```c
    ///// Provided by VK_KHR_wayland_surface
    ///VkBool32 vkGetPhysicalDeviceWaylandPresentationSupportKHR(
    ///    VkPhysicalDevice                            physicalDevice,
    ///    uint32_t                                    queueFamilyIndex,
    ///    struct wl_display*                          display);
    ///```
    /// # Parameters
    /// - [`physical_device`] is the physical device.
    /// - [`queue_family_index`] is the queue family index.
    /// - [`display`] is a pointer to the [`wl_display`] associated with a Wayland compositor.
    /// # Description
    /// This platform-specific function  **can**  be called prior to creating a surface.
    /// ## Valid Usage
    /// - [`queue_family_index`] **must**  be less than `pQueueFamilyPropertyCount` returned by
    ///   [`get_physical_device_queue_family_properties`] for the given [`physical_device`]
    ///
    /// ## Valid Usage (Implicit)
    /// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
    /// - [`display`] **must**  be a valid pointer to a [`wl_display`] value
    /// # Related
    /// - [`VK_KHR_wayland_surface`]
    /// - [`PhysicalDevice`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetPhysicalDeviceWaylandPresentationSupportKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_physical_device_wayland_presentation_support_khr(
        self: &Unique<PhysicalDevice>,
        queue_family_index: Option<u32>,
    ) -> (wl_display, bool) {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .instance()
            .vtable()
            .khr_wayland_surface()
            .and_then(|vtable| vtable.get_physical_device_wayland_presentation_support_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .instance()
            .vtable()
            .khr_wayland_surface()
            .and_then(|vtable| vtable.get_physical_device_wayland_presentation_support_khr())
            .unwrap_unchecked();
        let mut display = std::mem::zeroed();
        let _return = _function(self.as_raw(), queue_family_index.unwrap_or_default() as _, &mut display);
        (display, std::mem::transmute(_return as u8))
    }
}
///The V-table of [`Instance`] for functions from `VK_KHR_wayland_surface`
pub struct InstanceKhrWaylandSurfaceVTable {
    ///See [`FNCreateWaylandSurfaceKhr`] for more information.
    pub create_wayland_surface_khr: FNCreateWaylandSurfaceKhr,
    ///See [`FNGetPhysicalDeviceWaylandPresentationSupportKhr`] for more information.
    pub get_physical_device_wayland_presentation_support_khr: FNGetPhysicalDeviceWaylandPresentationSupportKhr,
}
impl InstanceKhrWaylandSurfaceVTable {
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
            create_wayland_surface_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCreateWaylandSurfaceKHR").as_ptr()))
            },
            get_physical_device_wayland_presentation_support_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetPhysicalDeviceWaylandPresentationSupportKHR").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::create_wayland_surface_khr`]. See [`FNCreateWaylandSurfaceKhr`] for more
    /// information.
    pub fn create_wayland_surface_khr(&self) -> FNCreateWaylandSurfaceKhr {
        self.create_wayland_surface_khr
    }
    ///Gets [`Self::get_physical_device_wayland_presentation_support_khr`]. See
    /// [`FNGetPhysicalDeviceWaylandPresentationSupportKhr`] for more information.
    pub fn get_physical_device_wayland_presentation_support_khr(
        &self,
    ) -> FNGetPhysicalDeviceWaylandPresentationSupportKhr {
        self.get_physical_device_wayland_presentation_support_khr
    }
}
