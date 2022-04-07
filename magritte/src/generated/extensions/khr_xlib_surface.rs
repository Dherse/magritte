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
//! - [`create_xlib_surface_khr`]
//! - [`get_physical_device_xlib_presentation_support_khr`]
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
//!than [`get_physical_device_surface_support_khr`]; if it returned
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
//! - [`create_xlib_surface_khr`]
//! - [`get_physical_device_xlib_presentation_support_khr`]
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
    native::{Display, VisualID, Window},
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, Bool32, Instance, PhysicalDevice, StructureType, VulkanResultCodes,
    },
    AsRaw, Unique, VulkanResult,
};
use std::{ffi::CStr, marker::PhantomData, mem::MaybeUninit, sync::atomic::AtomicBool};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_XLIB_SURFACE_SPEC_VERSION")]
pub const KHR_XLIB_SURFACE_SPEC_VERSION: u32 = 6;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_XLIB_SURFACE_EXTENSION_NAME")]
pub const KHR_XLIB_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_xlib_surface");
///[vkCreateXlibSurfaceKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateXlibSurfaceKHR.html) - Create a slink:VkSurfaceKHR object for an X11 window, using the Xlib client-side library
///# C Specifications
///To create a [`SurfaceKHR`] object for an X11 window, using the Xlib
///client-side library, call:
///```c
///// Provided by VK_KHR_xlib_surface
///VkResult vkCreateXlibSurfaceKHR(
///    VkInstance                                  instance,
///    const VkXlibSurfaceCreateInfoKHR*           pCreateInfo,
///    const VkAllocationCallbacks*                pAllocator,
///    VkSurfaceKHR*                               pSurface);
///```
/// # Parameters
/// - [`instance`] is the instance to associate the surface with.
/// - [`p_create_info`] is a pointer to a [`XlibSurfaceCreateInfoKHR`] structure containing the
///   parameters affecting the creation of the surface object.
/// - [`p_allocator`] is the allocator used for host memory allocated for the surface object when there is no more specific allocator available (see [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)).
/// - [`p_surface`] is a pointer to a [`SurfaceKHR`] handle in which the created surface object is
///   returned.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`instance`] **must**  be a valid [`Instance`] handle
/// - [`p_create_info`] **must**  be a valid pointer to a valid [`XlibSurfaceCreateInfoKHR`]
///   structure
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
/// - [`p_surface`] **must**  be a valid pointer to a [`SurfaceKHR`] handle
///
/// ## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
/// # Related
/// - [`VK_KHR_xlib_surface`]
/// - [`AllocationCallbacks`]
/// - [`Instance`]
/// - [`SurfaceKHR`]
/// - [`XlibSurfaceCreateInfoKHR`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCreateXlibSurfaceKHR")]
pub type FNCreateXlibSurfaceKhr = Option<
    for<'lt> unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const XlibSurfaceCreateInfoKHR<'lt>,
        p_allocator: *const AllocationCallbacks<'lt>,
        p_surface: *mut SurfaceKHR,
    ) -> VulkanResultCodes,
>;
///[vkGetPhysicalDeviceXlibPresentationSupportKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceXlibPresentationSupportKHR.html) - Query physical device for presentation to X11 server using Xlib
///# C Specifications
///To determine whether a queue family of a physical device supports
///presentation to an X11 server, using the Xlib client-side library, call:
///```c
///// Provided by VK_KHR_xlib_surface
///VkBool32 vkGetPhysicalDeviceXlibPresentationSupportKHR(
///    VkPhysicalDevice                            physicalDevice,
///    uint32_t                                    queueFamilyIndex,
///    Display*                                    dpy,
///    VisualID                                    visualID);
///```
/// # Parameters
/// - [`physical_device`] is the physical device.
/// - [`queue_family_index`] is the queue family index.
/// - [`dpy`] is a pointer to an Xlib [`Display`] connection to the server.
/// - `visualId` is an X11 visual ([`VisualID`]).
/// # Description
/// This platform-specific function  **can**  be called prior to creating a surface.
/// ## Valid Usage
/// - [`queue_family_index`] **must**  be less than `pQueueFamilyPropertyCount` returned by
///   [`get_physical_device_queue_family_properties`] for the given [`physical_device`]
///
/// ## Valid Usage (Implicit)
/// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
/// - [`dpy`] **must**  be a valid pointer to a [`Display`] value
/// # Related
/// - [`VK_KHR_xlib_surface`]
/// - [`PhysicalDevice`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetPhysicalDeviceXlibPresentationSupportKHR")]
pub type FNGetPhysicalDeviceXlibPresentationSupportKhr = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        dpy: *mut Display,
        visual_id: VisualID,
    ) -> Bool32,
>;
///[VkXlibSurfaceCreateFlagsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkXlibSurfaceCreateFlagsKHR.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_KHR_xlib_surface
///typedef VkFlags VkXlibSurfaceCreateFlagsKHR;
///```
/// # Related
/// - [`VK_KHR_xlib_surface`]
/// - [`XlibSurfaceCreateInfoKHR`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
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
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`dpy`] is a pointer to an Xlib [`Display`] connection to the X server.
/// - [`window`] is an Xlib [`Window`] to associate the surface with.
/// # Description
/// ## Valid Usage
/// - [`dpy`] **must**  point to a valid Xlib [`Display`]
/// - [`window`] **must**  be a valid Xlib [`Window`]
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`flags`] **must**  be `0`
/// # Related
/// - [`VK_KHR_xlib_surface`]
/// - [`StructureType`]
/// - [`XlibSurfaceCreateFlagsKHR`]
/// - [`create_xlib_surface_khr`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
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
            s_type: StructureType::XLIB_SURFACE_CREATE_INFO_KHR,
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
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::window`]
    pub fn set_window_raw(mut self, value: Window) -> Self {
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
    pub fn dpy(&self) -> *mut Display {
        self.dpy
    }
    ///Gets the value of [`Self::window`]
    pub fn window(&self) -> Window {
        self.window
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
    pub fn set_flags(mut self, value: crate::extensions::khr_xlib_surface::XlibSurfaceCreateFlagsKHR) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::dpy`]
    pub fn set_dpy(mut self, value: *mut crate::native::Display) -> Self {
        self.dpy = value;
        self
    }
    ///Sets the value of [`Self::window`]
    pub fn set_window(mut self, value: crate::native::Window) -> Self {
        self.window = value;
        self
    }
}
impl Instance {
    ///[vkCreateXlibSurfaceKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateXlibSurfaceKHR.html) - Create a slink:VkSurfaceKHR object for an X11 window, using the Xlib client-side library
    ///# C Specifications
    ///To create a [`SurfaceKHR`] object for an X11 window, using the Xlib
    ///client-side library, call:
    ///```c
    ///// Provided by VK_KHR_xlib_surface
    ///VkResult vkCreateXlibSurfaceKHR(
    ///    VkInstance                                  instance,
    ///    const VkXlibSurfaceCreateInfoKHR*           pCreateInfo,
    ///    const VkAllocationCallbacks*                pAllocator,
    ///    VkSurfaceKHR*                               pSurface);
    ///```
    /// # Parameters
    /// - [`instance`] is the instance to associate the surface with.
    /// - [`p_create_info`] is a pointer to a [`XlibSurfaceCreateInfoKHR`] structure containing the
    ///   parameters affecting the creation of the surface object.
    /// - [`p_allocator`] is the allocator used for host memory allocated for the surface object when there is no more specific allocator available (see [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)).
    /// - [`p_surface`] is a pointer to a [`SurfaceKHR`] handle in which the created surface object
    ///   is returned.
    /// # Description
    /// ## Valid Usage (Implicit)
    /// - [`instance`] **must**  be a valid [`Instance`] handle
    /// - [`p_create_info`] **must**  be a valid pointer to a valid [`XlibSurfaceCreateInfoKHR`]
    ///   structure
    /// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
    ///   [`AllocationCallbacks`] structure
    /// - [`p_surface`] **must**  be a valid pointer to a [`SurfaceKHR`] handle
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// # Related
    /// - [`VK_KHR_xlib_surface`]
    /// - [`AllocationCallbacks`]
    /// - [`Instance`]
    /// - [`SurfaceKHR`]
    /// - [`XlibSurfaceCreateInfoKHR`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCreateXlibSurfaceKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn create_xlib_surface_khr<'lt>(
        self: &Unique<Instance>,
        p_create_info: &XlibSurfaceCreateInfoKHR<'lt>,
        p_allocator: Option<&AllocationCallbacks<'lt>>,
    ) -> VulkanResult<Unique<SurfaceKHR>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_xlib_surface()
            .and_then(|vtable| vtable.create_xlib_surface_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_xlib_surface()
            .and_then(|vtable| vtable.create_xlib_surface_khr())
            .unwrap_unchecked();
        let mut p_surface = MaybeUninit::<SurfaceKHR>::uninit();
        let _return = _function(
            self.as_raw(),
            p_create_info as *const XlibSurfaceCreateInfoKHR<'lt>,
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
    ///[vkGetPhysicalDeviceXlibPresentationSupportKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceXlibPresentationSupportKHR.html) - Query physical device for presentation to X11 server using Xlib
    ///# C Specifications
    ///To determine whether a queue family of a physical device supports
    ///presentation to an X11 server, using the Xlib client-side library, call:
    ///```c
    ///// Provided by VK_KHR_xlib_surface
    ///VkBool32 vkGetPhysicalDeviceXlibPresentationSupportKHR(
    ///    VkPhysicalDevice                            physicalDevice,
    ///    uint32_t                                    queueFamilyIndex,
    ///    Display*                                    dpy,
    ///    VisualID                                    visualID);
    ///```
    /// # Parameters
    /// - [`physical_device`] is the physical device.
    /// - [`queue_family_index`] is the queue family index.
    /// - [`dpy`] is a pointer to an Xlib [`Display`] connection to the server.
    /// - `visualId` is an X11 visual ([`VisualID`]).
    /// # Description
    /// This platform-specific function  **can**  be called prior to creating a surface.
    /// ## Valid Usage
    /// - [`queue_family_index`] **must**  be less than `pQueueFamilyPropertyCount` returned by
    ///   [`get_physical_device_queue_family_properties`] for the given [`physical_device`]
    ///
    /// ## Valid Usage (Implicit)
    /// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
    /// - [`dpy`] **must**  be a valid pointer to a [`Display`] value
    /// # Related
    /// - [`VK_KHR_xlib_surface`]
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
    #[doc(alias = "vkGetPhysicalDeviceXlibPresentationSupportKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_physical_device_xlib_presentation_support_khr(
        self: &Unique<PhysicalDevice>,
        queue_family_index: Option<u32>,
        visual_id: VisualID,
    ) -> (Display, bool) {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .instance()
            .vtable()
            .khr_xlib_surface()
            .and_then(|vtable| vtable.get_physical_device_xlib_presentation_support_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .instance()
            .vtable()
            .khr_xlib_surface()
            .and_then(|vtable| vtable.get_physical_device_xlib_presentation_support_khr())
            .unwrap_unchecked();
        let mut dpy = std::mem::zeroed();
        let _return = _function(
            self.as_raw(),
            queue_family_index.unwrap_or_default() as _,
            &mut dpy,
            visual_id,
        );
        (dpy, std::mem::transmute(_return as u8))
    }
}
///The V-table of [`Instance`] for functions from `VK_KHR_xlib_surface`
pub struct InstanceKhrXlibSurfaceVTable {
    ///See [`FNCreateXlibSurfaceKhr`] for more information.
    pub create_xlib_surface_khr: FNCreateXlibSurfaceKhr,
    ///See [`FNGetPhysicalDeviceXlibPresentationSupportKhr`] for more information.
    pub get_physical_device_xlib_presentation_support_khr: FNGetPhysicalDeviceXlibPresentationSupportKhr,
}
impl InstanceKhrXlibSurfaceVTable {
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
            create_xlib_surface_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCreateXlibSurfaceKHR").as_ptr()))
            },
            get_physical_device_xlib_presentation_support_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetPhysicalDeviceXlibPresentationSupportKHR").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::create_xlib_surface_khr`]. See [`FNCreateXlibSurfaceKhr`] for more information.
    pub fn create_xlib_surface_khr(&self) -> FNCreateXlibSurfaceKhr {
        self.create_xlib_surface_khr
    }
    ///Gets [`Self::get_physical_device_xlib_presentation_support_khr`]. See
    /// [`FNGetPhysicalDeviceXlibPresentationSupportKhr`] for more information.
    pub fn get_physical_device_xlib_presentation_support_khr(&self) -> FNGetPhysicalDeviceXlibPresentationSupportKhr {
        self.get_physical_device_xlib_presentation_support_khr
    }
}
