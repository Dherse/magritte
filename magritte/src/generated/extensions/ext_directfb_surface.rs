//![VK_EXT_directfb_surface](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_directfb_surface.html) - instance extension
//!# Description
//!The [`VK_EXT_directfb_surface`] extension is an instance extension.
//!It provides a mechanism to create a [`SurfaceKHR`] object (defined by
//!the `[`VK_KHR_surface`]` extension) that refers to a DirectFB
//![`IDirectFBSurface`], as well as a query to determine support for rendering
//!via DirectFB.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_surface`]`
//!# Contacts
//! - Nicolas Caramelli [caramelli](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_directfb_surface]
//!   @caramelli%0A<<Here describe the issue or question you have about the VK_EXT_directfb_surface
//!   extension>>)
//!# New functions & commands
//! - [`create_direct_fb_surface_ext`]
//! - [`get_physical_device_direct_fb_presentation_support_ext`]
//!# New structures
//! - [`DirectFBSurfaceCreateInfoEXT`]
//!# New bitmasks
//! - [`DirectFBSurfaceCreateFlagsEXT`]
//!# New constants
//! - [`EXT_DIRECTFB_SURFACE_EXTENSION_NAME`]
//! - [`EXT_DIRECTFB_SURFACE_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_DIRECTFB_SURFACE_CREATE_INFO_EXT`
//!# Version History
//! - Revision 1, 2020-06-16 (Nicolas Caramelli)  - Initial version
//!# Other info
//! * 2020-06-16
//! * No known IP claims.
//! * - Nicolas Caramelli
//!# Related
//! - [`DirectFBSurfaceCreateFlagsEXT`]
//! - [`DirectFBSurfaceCreateInfoEXT`]
//! - [`create_direct_fb_surface_ext`]
//! - [`get_physical_device_direct_fb_presentation_support_ext`]
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
    native::{IDirectFB, IDirectFBSurface},
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, Bool32, Instance, PhysicalDevice, StructureType, VulkanResultCodes,
    },
    AsRaw, Unique, VulkanResult,
};
use std::{ffi::CStr, marker::PhantomData, mem::MaybeUninit, sync::atomic::AtomicBool};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DIRECTFB_SURFACE_SPEC_VERSION")]
pub const EXT_DIRECTFB_SURFACE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DIRECTFB_SURFACE_EXTENSION_NAME")]
pub const EXT_DIRECTFB_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_directfb_surface");
///[vkCreateDirectFBSurfaceEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDirectFBSurfaceEXT.html) - Create a slink:VkSurfaceKHR object for a DirectFB surface
///# C Specifications
///To create a [`SurfaceKHR`] object for a DirectFB surface, call:
///```c
///// Provided by VK_EXT_directfb_surface
///VkResult vkCreateDirectFBSurfaceEXT(
///    VkInstance                                  instance,
///    const VkDirectFBSurfaceCreateInfoEXT*       pCreateInfo,
///    const VkAllocationCallbacks*                pAllocator,
///    VkSurfaceKHR*                               pSurface);
///```
/// # Parameters
/// - [`instance`] is the instance to associate the surface with.
/// - [`p_create_info`] is a pointer to a [`DirectFBSurfaceCreateInfoEXT`] structure containing
///   parameters affecting the creation of the surface object.
/// - [`p_allocator`] is the allocator used for host memory allocated for the surface object when there is no more specific allocator available (see [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)).
/// - [`p_surface`] is a pointer to a [`SurfaceKHR`] handle in which the created surface object is
///   returned.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`instance`] **must**  be a valid [`Instance`] handle
/// - [`p_create_info`] **must**  be a valid pointer to a valid [`DirectFBSurfaceCreateInfoEXT`]
///   structure
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
/// - [`p_surface`] **must**  be a valid pointer to a [`SurfaceKHR`] handle
///
/// ## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
/// # Related
/// - [`VK_EXT_directfb_surface`]
/// - [`AllocationCallbacks`]
/// - [`DirectFBSurfaceCreateInfoEXT`]
/// - [`Instance`]
/// - [`SurfaceKHR`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCreateDirectFBSurfaceEXT")]
pub type FNCreateDirectFbSurfaceExt = Option<
    for<'lt> unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const DirectFBSurfaceCreateInfoEXT<'lt>,
        p_allocator: *const AllocationCallbacks<'lt>,
        p_surface: *mut SurfaceKHR,
    ) -> VulkanResultCodes,
>;
///[vkGetPhysicalDeviceDirectFBPresentationSupportEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDirectFBPresentationSupportEXT.html) - Query physical device for presentation with DirectFB
///# C Specifications
///To determine whether a queue family of a physical device supports
///presentation with DirectFB library, call:
///```c
///// Provided by VK_EXT_directfb_surface
///VkBool32 vkGetPhysicalDeviceDirectFBPresentationSupportEXT(
///    VkPhysicalDevice                            physicalDevice,
///    uint32_t                                    queueFamilyIndex,
///    IDirectFB*                                  dfb);
///```
/// # Parameters
/// - [`physical_device`] is the physical device.
/// - [`queue_family_index`] is the queue family index.
/// - [`dfb`] is a pointer to the [`IDirectFB`] main interface of DirectFB.
/// # Description
/// This platform-specific function  **can**  be called prior to creating a surface.
/// ## Valid Usage
/// - [`queue_family_index`] **must**  be less than `pQueueFamilyPropertyCount` returned by
///   [`get_physical_device_queue_family_properties`] for the given [`physical_device`]
///
/// ## Valid Usage (Implicit)
/// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
/// - [`dfb`] **must**  be a valid pointer to an [`IDirectFB`] value
/// # Related
/// - [`VK_EXT_directfb_surface`]
/// - [`PhysicalDevice`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetPhysicalDeviceDirectFBPresentationSupportEXT")]
pub type FNGetPhysicalDeviceDirectFbPresentationSupportExt = Option<
    unsafe extern "system" fn(physical_device: PhysicalDevice, queue_family_index: u32, dfb: *mut IDirectFB) -> Bool32,
>;
///[VkDirectFBSurfaceCreateFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDirectFBSurfaceCreateFlagsEXT.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_EXT_directfb_surface
///typedef VkFlags VkDirectFBSurfaceCreateFlagsEXT;
///```
/// # Related
/// - [`VK_EXT_directfb_surface`]
/// - [`DirectFBSurfaceCreateInfoEXT`]
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
pub struct DirectFBSurfaceCreateFlagsEXT(u32);
impl const Default for DirectFBSurfaceCreateFlagsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for DirectFBSurfaceCreateFlagsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(DirectFBSurfaceCreateFlagsEXT))
            .field(&self.0)
            .finish()
    }
}
///[VkDirectFBSurfaceCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDirectFBSurfaceCreateInfoEXT.html) - Structure specifying parameters of a newly created DirectFB surface object
///# C Specifications
///The [`DirectFBSurfaceCreateInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_directfb_surface
///typedef struct VkDirectFBSurfaceCreateInfoEXT {
///    VkStructureType                    sType;
///    const void*                        pNext;
///    VkDirectFBSurfaceCreateFlagsEXT    flags;
///    IDirectFB*                         dfb;
///    IDirectFBSurface*                  surface;
///} VkDirectFBSurfaceCreateInfoEXT;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`dfb`] is a pointer to the [`IDirectFB`] main interface of DirectFB.
/// - [`surface`] is a pointer to a [`IDirectFBSurface`] surface interface.
/// # Description
/// ## Valid Usage
/// - [`dfb`] **must**  point to a valid DirectFB [`IDirectFB`]
/// - [`surface`] **must**  point to a valid DirectFB [`IDirectFBSurface`]
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DIRECTFB_SURFACE_CREATE_INFO_EXT`
/// - [`p_next`] **must**  be `NULL`
/// - [`flags`] **must**  be `0`
/// # Related
/// - [`VK_EXT_directfb_surface`]
/// - [`DirectFBSurfaceCreateFlagsEXT`]
/// - [`StructureType`]
/// - [`create_direct_fb_surface_ext`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDirectFBSurfaceCreateInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DirectFBSurfaceCreateInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    pub flags: DirectFBSurfaceCreateFlagsEXT,
    ///[`dfb`] is a pointer to the [`IDirectFB`] main interface of DirectFB.
    pub dfb: *mut IDirectFB,
    ///[`surface`] is a pointer to a [`IDirectFBSurface`] surface interface.
    pub surface: *mut IDirectFBSurface,
}
impl<'lt> Default for DirectFBSurfaceCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::DIRECTFB_SURFACE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            flags: Default::default(),
            dfb: std::ptr::null_mut(),
            surface: std::ptr::null_mut(),
        }
    }
}
impl<'lt> DirectFBSurfaceCreateInfoEXT<'lt> {
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
    pub fn flags(&self) -> DirectFBSurfaceCreateFlagsEXT {
        self.flags
    }
    ///Gets the value of [`Self::dfb`]
    pub fn dfb(&self) -> *mut IDirectFB {
        self.dfb
    }
    ///Gets the value of [`Self::surface`]
    pub fn surface(&self) -> *mut IDirectFBSurface {
        self.surface
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut DirectFBSurfaceCreateFlagsEXT {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::dfb`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn dfb_mut(&mut self) -> &mut IDirectFB {
        &mut *self.dfb
    }
    ///Gets a mutable reference to the value of [`Self::surface`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn surface_mut(&mut self) -> &mut IDirectFBSurface {
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
    pub fn set_flags(mut self, value: crate::extensions::ext_directfb_surface::DirectFBSurfaceCreateFlagsEXT) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::dfb`]
    pub fn set_dfb(mut self, value: *mut crate::native::IDirectFB) -> Self {
        self.dfb = value;
        self
    }
    ///Sets the value of [`Self::surface`]
    pub fn set_surface(mut self, value: *mut crate::native::IDirectFBSurface) -> Self {
        self.surface = value;
        self
    }
}
impl Instance {
    ///[vkCreateDirectFBSurfaceEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDirectFBSurfaceEXT.html) - Create a slink:VkSurfaceKHR object for a DirectFB surface
    ///# C Specifications
    ///To create a [`SurfaceKHR`] object for a DirectFB surface, call:
    ///```c
    ///// Provided by VK_EXT_directfb_surface
    ///VkResult vkCreateDirectFBSurfaceEXT(
    ///    VkInstance                                  instance,
    ///    const VkDirectFBSurfaceCreateInfoEXT*       pCreateInfo,
    ///    const VkAllocationCallbacks*                pAllocator,
    ///    VkSurfaceKHR*                               pSurface);
    ///```
    /// # Parameters
    /// - [`instance`] is the instance to associate the surface with.
    /// - [`p_create_info`] is a pointer to a [`DirectFBSurfaceCreateInfoEXT`] structure containing
    ///   parameters affecting the creation of the surface object.
    /// - [`p_allocator`] is the allocator used for host memory allocated for the surface object when there is no more specific allocator available (see [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)).
    /// - [`p_surface`] is a pointer to a [`SurfaceKHR`] handle in which the created surface object
    ///   is returned.
    /// # Description
    /// ## Valid Usage (Implicit)
    /// - [`instance`] **must**  be a valid [`Instance`] handle
    /// - [`p_create_info`] **must**  be a valid pointer to a valid [`DirectFBSurfaceCreateInfoEXT`]
    ///   structure
    /// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
    ///   [`AllocationCallbacks`] structure
    /// - [`p_surface`] **must**  be a valid pointer to a [`SurfaceKHR`] handle
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// # Related
    /// - [`VK_EXT_directfb_surface`]
    /// - [`AllocationCallbacks`]
    /// - [`DirectFBSurfaceCreateInfoEXT`]
    /// - [`Instance`]
    /// - [`SurfaceKHR`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCreateDirectFBSurfaceEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn create_direct_fb_surface_ext<'lt>(
        self: &Unique<Instance>,
        p_create_info: &DirectFBSurfaceCreateInfoEXT<'lt>,
        p_allocator: Option<&AllocationCallbacks<'lt>>,
    ) -> VulkanResult<Unique<SurfaceKHR>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .ext_directfb_surface()
            .and_then(|vtable| vtable.create_direct_fb_surface_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .ext_directfb_surface()
            .and_then(|vtable| vtable.create_direct_fb_surface_ext())
            .unwrap_unchecked();
        let mut p_surface = MaybeUninit::<SurfaceKHR>::uninit();
        let _return = _function(
            self.as_raw(),
            p_create_info as *const DirectFBSurfaceCreateInfoEXT<'lt>,
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
    ///[vkGetPhysicalDeviceDirectFBPresentationSupportEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDirectFBPresentationSupportEXT.html) - Query physical device for presentation with DirectFB
    ///# C Specifications
    ///To determine whether a queue family of a physical device supports
    ///presentation with DirectFB library, call:
    ///```c
    ///// Provided by VK_EXT_directfb_surface
    ///VkBool32 vkGetPhysicalDeviceDirectFBPresentationSupportEXT(
    ///    VkPhysicalDevice                            physicalDevice,
    ///    uint32_t                                    queueFamilyIndex,
    ///    IDirectFB*                                  dfb);
    ///```
    /// # Parameters
    /// - [`physical_device`] is the physical device.
    /// - [`queue_family_index`] is the queue family index.
    /// - [`dfb`] is a pointer to the [`IDirectFB`] main interface of DirectFB.
    /// # Description
    /// This platform-specific function  **can**  be called prior to creating a surface.
    /// ## Valid Usage
    /// - [`queue_family_index`] **must**  be less than `pQueueFamilyPropertyCount` returned by
    ///   [`get_physical_device_queue_family_properties`] for the given [`physical_device`]
    ///
    /// ## Valid Usage (Implicit)
    /// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
    /// - [`dfb`] **must**  be a valid pointer to an [`IDirectFB`] value
    /// # Related
    /// - [`VK_EXT_directfb_surface`]
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
    #[doc(alias = "vkGetPhysicalDeviceDirectFBPresentationSupportEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_physical_device_direct_fb_presentation_support_ext(
        self: &Unique<PhysicalDevice>,
        queue_family_index: Option<u32>,
    ) -> (IDirectFB, bool) {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .instance()
            .vtable()
            .ext_directfb_surface()
            .and_then(|vtable| vtable.get_physical_device_direct_fb_presentation_support_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .instance()
            .vtable()
            .ext_directfb_surface()
            .and_then(|vtable| vtable.get_physical_device_direct_fb_presentation_support_ext())
            .unwrap_unchecked();
        let mut dfb = std::mem::zeroed();
        let _return = _function(self.as_raw(), queue_family_index.unwrap_or_default() as _, &mut dfb);
        (dfb, std::mem::transmute(_return as u8))
    }
}
///The V-table of [`Instance`] for functions from `VK_EXT_directfb_surface`
pub struct InstanceExtDirectfbSurfaceVTable {
    ///See [`FNCreateDirectFbSurfaceExt`] for more information.
    pub create_direct_fb_surface_ext: FNCreateDirectFbSurfaceExt,
    ///See [`FNGetPhysicalDeviceDirectFbPresentationSupportExt`] for more information.
    pub get_physical_device_direct_fb_presentation_support_ext: FNGetPhysicalDeviceDirectFbPresentationSupportExt,
}
impl InstanceExtDirectfbSurfaceVTable {
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
            create_direct_fb_surface_ext: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCreateDirectFBSurfaceEXT").as_ptr()))
            },
            get_physical_device_direct_fb_presentation_support_ext: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetPhysicalDeviceDirectFBPresentationSupportEXT").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::create_direct_fb_surface_ext`]. See [`FNCreateDirectFbSurfaceExt`] for more
    /// information.
    pub fn create_direct_fb_surface_ext(&self) -> FNCreateDirectFbSurfaceExt {
        self.create_direct_fb_surface_ext
    }
    ///Gets [`Self::get_physical_device_direct_fb_presentation_support_ext`]. See
    /// [`FNGetPhysicalDeviceDirectFbPresentationSupportExt`] for more information.
    pub fn get_physical_device_direct_fb_presentation_support_ext(
        &self,
    ) -> FNGetPhysicalDeviceDirectFbPresentationSupportExt {
        self.get_physical_device_direct_fb_presentation_support_ext
    }
}
