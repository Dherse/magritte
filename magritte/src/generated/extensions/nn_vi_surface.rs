//![VK_NN_vi_surface](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NN_vi_surface.html) - instance extension
//!# Description
//!The [`VK_NN_vi_surface`] extension is an instance extension.
//!It provides a mechanism to create a [`SurfaceKHR`] object (defined by
//!the `[`khr_surface`]` extension) associated with an
//!`nn`::`vi`::`Layer`.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_surface`]`
//!# Contacts
//! - Mathias Heyer mheyer
//!# New functions & commands
//! - [`create_vi_surface_nn`]
//!# New structures
//! - [`ViSurfaceCreateInfoNN`]
//!# New bitmasks
//! - [`ViSurfaceCreateFlagsNN`]
//!# New constants
//! - [`NN_VI_SURFACE_EXTENSION_NAME`]
//! - [`NN_VI_SURFACE_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_VI_SURFACE_CREATE_INFO_NN`
//!# Known issues & F.A.Q
//!1) Does VI need a way to query for compatibility between a particular
//!physical device (and queue family?) and a specific VI display? **RESOLVED** : No.
//!It is currently always assumed that the device and display will always be
//!compatible.2) [`ViSurfaceCreateInfoNN`]`::pWindow` is intended to store an
//!`nn`::`vi`::`NativeWindowHandle`, but its declared type is a bare
//!`void*` to store the window handle.
//!Why the discrepancy? **RESOLVED** : It is for C compatibility.
//!The definition for the VI native window handle type is defined inside the
//!`nn`::`vi` C++ namespace.
//!This prevents its use in C source files.
//!`nn`::`vi`::`NativeWindowHandle` is always defined to be
//!`void*`, so this extension uses `void*` to match.
//!# Version History
//! - Revision 1, 2016-12-2 (Michael Chock)  - Initial draft.
//!# Other info
//! * 2016-12-02
//! * No known IP claims.
//! * - Mathias Heyer, NVIDIA  - Michael Chock, NVIDIA  - Yasuhiro Yoshioka, Nintendo  - Daniel
//!   Koch, NVIDIA
//!# Related
//! - [`ViSurfaceCreateFlagsNN`]
//! - [`ViSurfaceCreateInfoNN`]
//! - [`create_vi_surface_nn`]
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
#[doc(alias = "VK_NN_VI_SURFACE_SPEC_VERSION")]
pub const NN_VI_SURFACE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NN_VI_SURFACE_EXTENSION_NAME")]
pub const NN_VI_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NN_vi_surface");
///[vkCreateViSurfaceNN](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateViSurfaceNN.html) - Create a slink:VkSurfaceKHR object for a VI layer
///# C Specifications
///To create a [`SurfaceKHR`] object for an `nn`::`vi`::`Layer`,
///query the layer’s native handle using
///`nn`::`vi`::`GetNativeWindow`, and then call:
///```c
///// Provided by VK_NN_vi_surface
///VkResult vkCreateViSurfaceNN(
///    VkInstance                                  instance,
///    const VkViSurfaceCreateInfoNN*              pCreateInfo,
///    const VkAllocationCallbacks*                pAllocator,
///    VkSurfaceKHR*                               pSurface);
///```
/// # Parameters
/// - [`instance`] is the instance with which to associate the surface.
/// - [`p_create_info`] is a pointer to a [`ViSurfaceCreateInfoNN`] structure containing parameters
///   affecting the creation of the surface object.
/// - [`p_allocator`] is the allocator used for host memory allocated for the surface object when there is no more specific allocator available (see [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)).
/// - [`p_surface`] is a pointer to a [`SurfaceKHR`] handle in which the created surface object is
///   returned.
/// # Description
/// During the lifetime of a surface created using a particular
/// `nn`::`vi`::`NativeWindowHandle`, applications  **must**  not attempt to
/// create another surface for the same `nn`::`vi`::`Layer` or attempt
/// to connect to the same `nn`::`vi`::`Layer` through other platform
/// mechanisms.If the native window is created with a specified size, `currentExtent`
/// will reflect that size.
/// In this case, applications should use the same size for the swapchain’s
/// `imageExtent`.
/// Otherwise, the `currentExtent` will have the special value
/// (0xFFFFFFFF, 0xFFFFFFFF), indicating that applications are expected to
/// choose an appropriate size for the swapchain’s `imageExtent` (e.g., by
/// matching the result of a call to
/// `nn`::`vi`::`GetDisplayResolution`).
/// ## Valid Usage (Implicit)
/// - [`instance`] **must**  be a valid [`Instance`] handle
/// - [`p_create_info`] **must**  be a valid pointer to a valid [`ViSurfaceCreateInfoNN`] structure
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
/// - [`p_surface`] **must**  be a valid pointer to a [`SurfaceKHR`] handle
///
/// ## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
///   `VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`
/// # Related
/// - [`nn_vi_surface`]
/// - [`AllocationCallbacks`]
/// - [`Instance`]
/// - [`SurfaceKHR`]
/// - [`ViSurfaceCreateInfoNN`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCreateViSurfaceNN")]
pub type FNCreateViSurfaceNn = Option<
    for<'lt> unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const ViSurfaceCreateInfoNN<'lt>,
        p_allocator: *const AllocationCallbacks<'lt>,
        p_surface: *mut SurfaceKHR,
    ) -> VulkanResultCodes,
>;
///[VkViSurfaceCreateFlagsNN](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkViSurfaceCreateFlagsNN.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_NN_vi_surface
///typedef VkFlags VkViSurfaceCreateFlagsNN;
///```
/// # Related
/// - [`nn_vi_surface`]
/// - [`ViSurfaceCreateInfoNN`]
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
pub struct ViSurfaceCreateFlagsNN(u32);
impl const Default for ViSurfaceCreateFlagsNN {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for ViSurfaceCreateFlagsNN {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(ViSurfaceCreateFlagsNN))
            .field(&self.0)
            .finish()
    }
}
///[VkViSurfaceCreateInfoNN](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkViSurfaceCreateInfoNN.html) - Structure specifying parameters of a newly created VI surface object
///# C Specifications
///The [`ViSurfaceCreateInfoNN`] structure is defined as:
///```c
///// Provided by VK_NN_vi_surface
///typedef struct VkViSurfaceCreateInfoNN {
///    VkStructureType             sType;
///    const void*                 pNext;
///    VkViSurfaceCreateFlagsNN    flags;
///    void*                       window;
///} VkViSurfaceCreateInfoNN;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`window`] is the `nn`::`vi`::`NativeWindowHandle` for the `nn`::`vi`::`Layer` with which to
///   associate the surface.
/// # Description
/// ## Valid Usage
/// - [`window`] **must**  be a valid `nn`::`vi`::`NativeWindowHandle`
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VI_SURFACE_CREATE_INFO_NN`
/// - [`p_next`] **must**  be `NULL`
/// - [`flags`] **must**  be `0`
/// # Related
/// - [`nn_vi_surface`]
/// - [`StructureType`]
/// - [`ViSurfaceCreateFlagsNN`]
/// - [`create_vi_surface_nn`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkViSurfaceCreateInfoNN")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct ViSurfaceCreateInfoNN<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    pub flags: ViSurfaceCreateFlagsNN,
    ///[`window`] is the `nn`::`vi`::`NativeWindowHandle` for the
    ///`nn`::`vi`::`Layer` with which to associate the surface.
    pub window: *mut c_void,
}
impl<'lt> Default for ViSurfaceCreateInfoNN<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VI_SURFACE_CREATE_INFO_NN,
            p_next: std::ptr::null(),
            flags: Default::default(),
            window: std::ptr::null_mut(),
        }
    }
}
impl<'lt> ViSurfaceCreateInfoNN<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::window`]
    pub fn window_raw(&self) -> *mut c_void {
        self.window
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::window`]
    pub fn set_window_raw(&mut self, value: *mut c_void) -> &mut Self {
        self.window = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::window`]
    pub fn with_window_raw(mut self, value: *mut c_void) -> Self {
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
    pub fn flags(&self) -> ViSurfaceCreateFlagsNN {
        self.flags
    }
    ///Gets the value of [`Self::window`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn window(&self) -> &c_void {
        &*self.window
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut ViSurfaceCreateFlagsNN {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::window`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn window_mut(&mut self) -> &mut c_void {
        &mut *self.window
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn set_flags(&mut self, value: crate::extensions::nn_vi_surface::ViSurfaceCreateFlagsNN) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::window`]
    pub fn set_window(&mut self, value: &'lt mut std::ffi::c_void) -> &mut Self {
        self.window = value as *mut _;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn with_flags(mut self, value: crate::extensions::nn_vi_surface::ViSurfaceCreateFlagsNN) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::window`]
    pub fn with_window(mut self, value: &'lt mut std::ffi::c_void) -> Self {
        self.window = value as *mut _;
        self
    }
}
impl Instance {
    ///[vkCreateViSurfaceNN](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateViSurfaceNN.html) - Create a slink:VkSurfaceKHR object for a VI layer
    ///# C Specifications
    ///To create a [`SurfaceKHR`] object for an `nn`::`vi`::`Layer`,
    ///query the layer’s native handle using
    ///`nn`::`vi`::`GetNativeWindow`, and then call:
    ///```c
    ///// Provided by VK_NN_vi_surface
    ///VkResult vkCreateViSurfaceNN(
    ///    VkInstance                                  instance,
    ///    const VkViSurfaceCreateInfoNN*              pCreateInfo,
    ///    const VkAllocationCallbacks*                pAllocator,
    ///    VkSurfaceKHR*                               pSurface);
    ///```
    /// # Parameters
    /// - [`instance`] is the instance with which to associate the surface.
    /// - [`p_create_info`] is a pointer to a [`ViSurfaceCreateInfoNN`] structure containing
    ///   parameters affecting the creation of the surface object.
    /// - [`p_allocator`] is the allocator used for host memory allocated for the surface object when there is no more specific allocator available (see [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)).
    /// - [`p_surface`] is a pointer to a [`SurfaceKHR`] handle in which the created surface object
    ///   is returned.
    /// # Description
    /// During the lifetime of a surface created using a particular
    /// `nn`::`vi`::`NativeWindowHandle`, applications  **must**  not attempt to
    /// create another surface for the same `nn`::`vi`::`Layer` or attempt
    /// to connect to the same `nn`::`vi`::`Layer` through other platform
    /// mechanisms.If the native window is created with a specified size, `currentExtent`
    /// will reflect that size.
    /// In this case, applications should use the same size for the swapchain’s
    /// `imageExtent`.
    /// Otherwise, the `currentExtent` will have the special value
    /// (0xFFFFFFFF, 0xFFFFFFFF), indicating that applications are expected to
    /// choose an appropriate size for the swapchain’s `imageExtent` (e.g., by
    /// matching the result of a call to
    /// `nn`::`vi`::`GetDisplayResolution`).
    /// ## Valid Usage (Implicit)
    /// - [`instance`] **must**  be a valid [`Instance`] handle
    /// - [`p_create_info`] **must**  be a valid pointer to a valid [`ViSurfaceCreateInfoNN`]
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
    /// - [`nn_vi_surface`]
    /// - [`AllocationCallbacks`]
    /// - [`Instance`]
    /// - [`SurfaceKHR`]
    /// - [`ViSurfaceCreateInfoNN`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCreateViSurfaceNN")]
    #[track_caller]
    #[inline]
    pub unsafe fn create_vi_surface_nn<'lt>(
        self: &Unique<Instance>,
        p_create_info: &ViSurfaceCreateInfoNN<'lt>,
        p_allocator: Option<&AllocationCallbacks<'lt>>,
    ) -> VulkanResult<Unique<SurfaceKHR>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .nn_vi_surface()
            .and_then(|vtable| vtable.create_vi_surface_nn())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .nn_vi_surface()
            .and_then(|vtable| vtable.create_vi_surface_nn())
            .unwrap_unchecked();
        let mut p_surface = MaybeUninit::<SurfaceKHR>::uninit();
        let _return = _function(
            self.as_raw(),
            p_create_info as *const ViSurfaceCreateInfoNN<'lt>,
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
///The V-table of [`Instance`] for functions from `VK_NN_vi_surface`
pub struct InstanceNnViSurfaceVTable {
    ///See [`FNCreateViSurfaceNn`] for more information.
    pub create_vi_surface_nn: FNCreateViSurfaceNn,
}
impl InstanceNnViSurfaceVTable {
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
            create_vi_surface_nn: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCreateViSurfaceNN").as_ptr()))
            },
        }
    }
    ///Gets [`Self::create_vi_surface_nn`]. See [`FNCreateViSurfaceNn`] for more information.
    pub fn create_vi_surface_nn(&self) -> FNCreateViSurfaceNn {
        self.create_vi_surface_nn
    }
}
