//![VK_MVK_macos_surface](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MVK_macos_surface.html) - instance extension
//!# Description
//!The [`VK_MVK_macos_surface`] extension is an instance extension.
//!It provides a mechanism to create a [`SurfaceKHR`] object (defined by
//!the `[`VK_KHR_surface`]` extension) based on an `NSView`, the native
//!surface type of macOS, which is underpinned by a [`CaMetalLayer`], to
//!support rendering to the surface using Apple’s Metal framework.
//!# Revision
//!3
//!# Dependencies
//! - *Deprecated* by `[`VK_EXT_metal_surface`]` extension
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_surface`]`
//!# Contacts
//! - Bill Hollings [billhollings](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_MVK_macos_surface]
//!   @billhollings%0A<<Here describe the issue or question you have about the VK_MVK_macos_surface
//!   extension>>)
//!# New functions & commands
//! - [`create_mac_os_surface_mvk`]
//!# New structures
//! - [`MacOsSurfaceCreateInfoMVK`]
//!# New bitmasks
//! - [`MacOsSurfaceCreateFlagsMVK`]
//!# New constants
//! - [`MVK_MACOS_SURFACE_EXTENSION_NAME`]
//! - [`MVK_MACOS_SURFACE_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK`
//!# Version History
//! - Revision 1, 2017-02-15 (Bill Hollings)  - Initial draft.
//! - Revision 2, 2017-02-24 (Bill Hollings)  - Minor syntax fix to emphasize firm requirement for
//!   `NSView` to be backed by a [`CaMetalLayer`].
//! - Revision 3, 2020-07-31 (Bill Hollings)  - Update documentation on requirements for `NSView`.
//!   - Mark as deprecated by [`VK_EXT_metal_surface`].
//!# Other info
//! * 2020-07-31
//! * No known IP claims.
//! * - Bill Hollings, The Brenwill Workshop Ltd.
//!# Related
//! - [`MacOsSurfaceCreateFlagsMVK`]
//! - [`MacOsSurfaceCreateInfoMVK`]
//! - [`create_mac_os_surface_mvk`]
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
    vulkan1_0::{AllocationCallbacks, BaseInStructure, Instance, StructureType, VulkanResultCodes},
    AsRaw, Unique, VulkanResult,
};
use std::{
    ffi::{c_void, CStr},
    marker::PhantomData,
    mem::MaybeUninit,
    sync::atomic::AtomicBool,
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_MVK_MACOS_SURFACE_SPEC_VERSION")]
pub const MVK_MACOS_SURFACE_SPEC_VERSION: u32 = 3;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_MVK_MACOS_SURFACE_EXTENSION_NAME")]
pub const MVK_MACOS_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_MVK_macos_surface");
///[vkCreateMacOSSurfaceMVK](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateMacOSSurfaceMVK.html) - Create a VkSurfaceKHR object for a macOS NSView
///# C Specifications
///To create a [`SurfaceKHR`] object for a macOS `NSView` or
///[`CaMetalLayer`], call:
///```c
///// Provided by VK_MVK_macos_surface
///VkResult vkCreateMacOSSurfaceMVK(
///    VkInstance                                  instance,
///    const VkMacOSSurfaceCreateInfoMVK*          pCreateInfo,
///    const VkAllocationCallbacks*                pAllocator,
///    VkSurfaceKHR*                               pSurface);
///```
/// # Description
/// - [`instance`] is the instance with which to associate the surface.
/// - [`p_create_info`] is a pointer to a [`MacOsSurfaceCreateInfoMVK`] structure containing
///   parameters affecting the creation of the surface object.
/// - [`p_allocator`] is the allocator used for host memory allocated for the surface object when there is no more specific allocator available (see [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)).
/// - [`p_surface`] is a pointer to a [`SurfaceKHR`] handle in which the created surface object is
///   returned.
///
/// ## Valid Usage (Implicit)
/// - [`instance`] **must**  be a valid [`Instance`] handle
/// - [`p_create_info`] **must**  be a valid pointer to a valid [`MacOsSurfaceCreateInfoMVK`]
///   structure
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
/// - [`p_surface`] **must**  be a valid pointer to a [`SurfaceKHR`] handle
///
/// ## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
///   `VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`
/// # Related
/// - [`VK_MVK_macos_surface`]
/// - [`AllocationCallbacks`]
/// - [`Instance`]
/// - [`MacOsSurfaceCreateInfoMVK`]
/// - [`SurfaceKHR`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCreateMacOSSurfaceMVK")]
pub type FNCreateMacOsSurfaceMvk = Option<
    for<'lt> unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const MacOsSurfaceCreateInfoMVK<'lt>,
        p_allocator: *const AllocationCallbacks<'lt>,
        p_surface: *mut SurfaceKHR,
    ) -> VulkanResultCodes,
>;
///[VkMacOSSurfaceCreateFlagsMVK](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMacOSSurfaceCreateFlagsMVK.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_MVK_macos_surface
///typedef VkFlags VkMacOSSurfaceCreateFlagsMVK;
///```
/// # Related
/// - [`VK_MVK_macos_surface`]
/// - [`MacOsSurfaceCreateInfoMVK`]
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
pub struct MacOsSurfaceCreateFlagsMVK(u32);
impl const Default for MacOsSurfaceCreateFlagsMVK {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for MacOsSurfaceCreateFlagsMVK {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(MacOsSurfaceCreateFlagsMVK))
            .field(&self.0)
            .finish()
    }
}
///[VkMacOSSurfaceCreateInfoMVK](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMacOSSurfaceCreateInfoMVK.html) - Structure specifying parameters of a newly created macOS surface object
///# C Specifications
///The [`MacOsSurfaceCreateInfoMVK`] structure is defined as:
///```c
///// Provided by VK_MVK_macos_surface
///typedef struct VkMacOSSurfaceCreateInfoMVK {
///    VkStructureType                 sType;
///    const void*                     pNext;
///    VkMacOSSurfaceCreateFlagsMVK    flags;
///    const void*                     pView;
///} VkMacOSSurfaceCreateInfoMVK;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`view`] is a reference to either a [`CaMetalLayer`] object or an `NSView` object.
/// # Description
/// ## Valid Usage
/// - If [`view`] is a [`CaMetalLayer`] object, it  **must**  be a valid [`CaMetalLayer`]
/// - If [`view`] is an `NSView` object, it  **must**  be a valid `NSView`,  **must**  be backed by
///   a `CALayer` object of type [`CaMetalLayer`], and [`create_mac_os_surface_mvk`] **must**  be
///   called on the main thread
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK`
/// - [`p_next`] **must**  be `NULL`
/// - [`flags`] **must**  be `0`
/// # Related
/// - [`VK_MVK_macos_surface`]
/// - [`MacOsSurfaceCreateFlagsMVK`]
/// - [`StructureType`]
/// - [`create_mac_os_surface_mvk`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkMacOSSurfaceCreateInfoMVK")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct MacOsSurfaceCreateInfoMVK<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    pub flags: MacOsSurfaceCreateFlagsMVK,
    ///[`view`] is a reference to either a [`CaMetalLayer`] object or
    ///an `NSView` object.
    pub view: *const c_void,
}
impl<'lt> Default for MacOsSurfaceCreateInfoMVK<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::MACOS_SURFACE_CREATE_INFO_MVK,
            p_next: std::ptr::null(),
            flags: Default::default(),
            view: std::ptr::null(),
        }
    }
}
impl<'lt> MacOsSurfaceCreateInfoMVK<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::view`]
    pub fn view_raw(&self) -> *const c_void {
        self.view
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::view`]
    pub fn set_view_raw(mut self, value: *const c_void) -> Self {
        self.view = value;
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
    pub fn flags(&self) -> MacOsSurfaceCreateFlagsMVK {
        self.flags
    }
    ///Gets the value of [`Self::view`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn view(&self) -> &c_void {
        &*self.view
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut MacOsSurfaceCreateFlagsMVK {
        &mut self.flags
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
    pub fn set_flags(mut self, value: crate::extensions::mvk_macos_surface::MacOsSurfaceCreateFlagsMVK) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::view`]
    pub fn set_view(mut self, value: &'lt std::ffi::c_void) -> Self {
        self.view = value as *const _;
        self
    }
}
impl Instance {
    ///[vkCreateMacOSSurfaceMVK](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateMacOSSurfaceMVK.html) - Create a VkSurfaceKHR object for a macOS NSView
    ///# C Specifications
    ///To create a [`SurfaceKHR`] object for a macOS `NSView` or
    ///[`CaMetalLayer`], call:
    ///```c
    ///// Provided by VK_MVK_macos_surface
    ///VkResult vkCreateMacOSSurfaceMVK(
    ///    VkInstance                                  instance,
    ///    const VkMacOSSurfaceCreateInfoMVK*          pCreateInfo,
    ///    const VkAllocationCallbacks*                pAllocator,
    ///    VkSurfaceKHR*                               pSurface);
    ///```
    /// # Description
    /// - [`instance`] is the instance with which to associate the surface.
    /// - [`p_create_info`] is a pointer to a [`MacOsSurfaceCreateInfoMVK`] structure containing
    ///   parameters affecting the creation of the surface object.
    /// - [`p_allocator`] is the allocator used for host memory allocated for the surface object when there is no more specific allocator available (see [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)).
    /// - [`p_surface`] is a pointer to a [`SurfaceKHR`] handle in which the created surface object
    ///   is returned.
    ///
    /// ## Valid Usage (Implicit)
    /// - [`instance`] **must**  be a valid [`Instance`] handle
    /// - [`p_create_info`] **must**  be a valid pointer to a valid [`MacOsSurfaceCreateInfoMVK`]
    ///   structure
    /// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
    ///   [`AllocationCallbacks`] structure
    /// - [`p_surface`] **must**  be a valid pointer to a [`SurfaceKHR`] handle
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
    ///   `VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`
    /// # Related
    /// - [`VK_MVK_macos_surface`]
    /// - [`AllocationCallbacks`]
    /// - [`Instance`]
    /// - [`MacOsSurfaceCreateInfoMVK`]
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
    #[doc(alias = "vkCreateMacOSSurfaceMVK")]
    #[track_caller]
    #[inline]
    pub unsafe fn create_mac_os_surface_mvk<'lt>(
        self: &Unique<Instance>,
        p_create_info: &MacOsSurfaceCreateInfoMVK<'lt>,
        p_allocator: Option<&AllocationCallbacks<'lt>>,
    ) -> VulkanResult<Unique<SurfaceKHR>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .mvk_macos_surface()
            .and_then(|vtable| vtable.create_mac_os_surface_mvk())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .mvk_macos_surface()
            .and_then(|vtable| vtable.create_mac_os_surface_mvk())
            .unwrap_unchecked();
        let mut p_surface = MaybeUninit::<SurfaceKHR>::uninit();
        let _return = _function(
            self.as_raw(),
            p_create_info as *const MacOsSurfaceCreateInfoMVK<'lt>,
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
///The V-table of [`Instance`] for functions from `VK_MVK_macos_surface`
pub struct InstanceMvkMacosSurfaceVTable {
    ///See [`FNCreateMacOsSurfaceMvk`] for more information.
    pub create_mac_os_surface_mvk: FNCreateMacOsSurfaceMvk,
}
impl InstanceMvkMacosSurfaceVTable {
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
            create_mac_os_surface_mvk: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCreateMacOSSurfaceMVK").as_ptr()))
            },
        }
    }
    ///Gets [`Self::create_mac_os_surface_mvk`]. See [`FNCreateMacOsSurfaceMvk`] for more
    /// information.
    pub fn create_mac_os_surface_mvk(&self) -> FNCreateMacOsSurfaceMvk {
        self.create_mac_os_surface_mvk
    }
}
