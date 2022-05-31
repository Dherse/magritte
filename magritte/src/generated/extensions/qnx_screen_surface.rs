//![VK_QNX_screen_surface](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_QNX_screen_surface.html) - instance extension
//!# Description
//!The [`VK_QNX_screen_surface`] extension is an instance extension.
//!It provides a mechanism to create a [`SurfaceKHR`] object (defined by
//!the `[`khr_surface`]` extension) that refers to a QNX Screen
//!`window`, as well as a query to determine support for rendering to a QNX
//!Screen compositor.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_surface`]`
//!# Contacts
//! - Mike Gorchak [mgorchak-blackberry](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_QNX_screen_surface]
//!   @mgorchak-blackberry%0A<<Here describe the issue or question you have about the
//!   VK_QNX_screen_surface extension>>)
//!# New functions & commands
//! - [`create_screen_surface_qnx`]
//! - [`get_physical_device_screen_presentation_support_qnx`]
//!# New structures
//! - [`ScreenSurfaceCreateInfoQNX`]
//!# New bitmasks
//! - [`ScreenSurfaceCreateFlagsQNX`]
//!# New constants
//! - [`QNX_SCREEN_SURFACE_EXTENSION_NAME`]
//! - [`QNX_SCREEN_SURFACE_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_SCREEN_SURFACE_CREATE_INFO_QNX`
//!# Version History
//! - Revision 1, 2021-01-11 (Mike Gorchak)  - Initial draft.
//!# Other info
//! * 2021-01-11
//! * No known IP claims.
//! * - Mike Gorchak, BlackBerry Limited
//!# Related
//! - [`ScreenSurfaceCreateFlagsQNX`]
//! - [`ScreenSurfaceCreateInfoQNX`]
//! - [`create_screen_surface_qnx`]
//! - [`get_physical_device_screen_presentation_support_qnx`]
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
    native::{_screen_context, _screen_window},
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, Bool32, Instance, PhysicalDevice, StructureType, VulkanResultCodes,
    },
    AsRaw, Unique, VulkanResult,
};
use std::{ffi::CStr, marker::PhantomData, mem::MaybeUninit, sync::atomic::AtomicBool};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_QNX_SCREEN_SURFACE_SPEC_VERSION")]
pub const QNX_SCREEN_SURFACE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_QNX_SCREEN_SURFACE_EXTENSION_NAME")]
pub const QNX_SCREEN_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_QNX_screen_surface");
///[vkCreateScreenSurfaceQNX](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateScreenSurfaceQNX.html) - Create a slink:VkSurfaceKHR object for a QNX Screen window
///# C Specifications
///To create a [`SurfaceKHR`] object for a QNX Screen surface, call:
///```c
///// Provided by VK_QNX_screen_surface
///VkResult vkCreateScreenSurfaceQNX(
///    VkInstance                                  instance,
///    const VkScreenSurfaceCreateInfoQNX*         pCreateInfo,
///    const VkAllocationCallbacks*                pAllocator,
///    VkSurfaceKHR*                               pSurface);
///```
/// # Parameters
/// - [`instance`] is the instance to associate the surface with.
/// - [`p_create_info`] is a pointer to a [`ScreenSurfaceCreateInfoQNX`] structure containing
///   parameters affecting the creation of the surface object.
/// - [`p_allocator`] is the allocator used for host memory allocated for the surface object when there is no more specific allocator available (see [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)).
/// - [`p_surface`] is a pointer to a [`SurfaceKHR`] handle in which the created surface object is
///   returned.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`instance`] **must**  be a valid [`Instance`] handle
/// - [`p_create_info`] **must**  be a valid pointer to a valid [`ScreenSurfaceCreateInfoQNX`]
///   structure
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
/// - [`p_surface`] **must**  be a valid pointer to a [`SurfaceKHR`] handle
///
/// ## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
/// # Related
/// - [`qnx_screen_surface`]
/// - [`AllocationCallbacks`]
/// - [`Instance`]
/// - [`ScreenSurfaceCreateInfoQNX`]
/// - [`SurfaceKHR`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCreateScreenSurfaceQNX")]
pub type FNCreateScreenSurfaceQnx = Option<
    for<'lt> unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const ScreenSurfaceCreateInfoQNX<'lt>,
        p_allocator: *const AllocationCallbacks<'lt>,
        p_surface: *mut SurfaceKHR,
    ) -> VulkanResultCodes,
>;
///[vkGetPhysicalDeviceScreenPresentationSupportQNX](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceScreenPresentationSupportQNX.html) - Query physical device for presentation to QNX Screen
///# C Specifications
///To determine whether a queue family of a physical device supports
///presentation to a QNX Screen compositor, call:
///```c
///// Provided by VK_QNX_screen_surface
///VkBool32 vkGetPhysicalDeviceScreenPresentationSupportQNX(
///    VkPhysicalDevice                            physicalDevice,
///    uint32_t                                    queueFamilyIndex,
///    struct _screen_window*                      window);
///```
/// # Parameters
/// - [`physical_device`] is the physical device.
/// - [`queue_family_index`] is the queue family index.
/// - [`window`] is the QNX Screen [`window`] object.
/// # Description
/// This platform-specific function  **can**  be called prior to creating a surface.
/// ## Valid Usage
/// - [`queue_family_index`] **must**  be less than `pQueueFamilyPropertyCount` returned by
///   [`get_physical_device_queue_family_properties`] for the given [`physical_device`]
///
/// ## Valid Usage (Implicit)
/// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
/// - [`window`] **must**  be a valid pointer to a [`_screen_window`] value
/// # Related
/// - [`qnx_screen_surface`]
/// - [`PhysicalDevice`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetPhysicalDeviceScreenPresentationSupportQNX")]
pub type FNGetPhysicalDeviceScreenPresentationSupportQnx = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        window: *mut _screen_window,
    ) -> Bool32,
>;
///[VkScreenSurfaceCreateFlagsQNX](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkScreenSurfaceCreateFlagsQNX.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_QNX_screen_surface
///typedef VkFlags VkScreenSurfaceCreateFlagsQNX;
///```
/// # Related
/// - [`qnx_screen_surface`]
/// - [`ScreenSurfaceCreateInfoQNX`]
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
pub struct ScreenSurfaceCreateFlagsQNX(u32);
impl const Default for ScreenSurfaceCreateFlagsQNX {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for ScreenSurfaceCreateFlagsQNX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(ScreenSurfaceCreateFlagsQNX))
            .field(&self.0)
            .finish()
    }
}
///[VkScreenSurfaceCreateInfoQNX](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkScreenSurfaceCreateInfoQNX.html) - Structure specifying parameters of a newly created QNX Screen surface object
///# C Specifications
///The [`ScreenSurfaceCreateInfoQNX`] structure is defined as:
///```c
///// Provided by VK_QNX_screen_surface
///typedef struct VkScreenSurfaceCreateInfoQNX {
///    VkStructureType                  sType;
///    const void*                      pNext;
///    VkScreenSurfaceCreateFlagsQNX    flags;
///    struct _screen_context*          context;
///    struct _screen_window*           window;
///} VkScreenSurfaceCreateInfoQNX;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`context`] and [`window`] are QNX Screen [`context`] and [`window`] to associate the surface
///   with.
/// # Description
/// ## Valid Usage
/// - [`context`] **must**  point to a valid QNX Screen `struct` _screen_context
/// - [`window`] **must**  point to a valid QNX Screen `struct` _screen_window
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SCREEN_SURFACE_CREATE_INFO_QNX`
/// - [`p_next`] **must**  be `NULL`
/// - [`flags`] **must**  be `0`
/// # Related
/// - [`qnx_screen_surface`]
/// - [`ScreenSurfaceCreateFlagsQNX`]
/// - [`StructureType`]
/// - [`create_screen_surface_qnx`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkScreenSurfaceCreateInfoQNX")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct ScreenSurfaceCreateInfoQNX<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    pub flags: ScreenSurfaceCreateFlagsQNX,
    ///[`context`] and [`window`] are QNX Screen [`context`] and
    ///[`window`] to associate the surface with.
    pub context: *mut _screen_context,
    ///No documentation found
    pub window: *mut _screen_window,
}
impl<'lt> Default for ScreenSurfaceCreateInfoQNX<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::SCREEN_SURFACE_CREATE_INFO_QNX,
            p_next: std::ptr::null(),
            flags: Default::default(),
            context: std::ptr::null_mut(),
            window: std::ptr::null_mut(),
        }
    }
}
impl<'lt> ScreenSurfaceCreateInfoQNX<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    pub fn flags(&self) -> ScreenSurfaceCreateFlagsQNX {
        self.flags
    }
    ///Gets the value of [`Self::context`]
    pub fn context(&self) -> *mut _screen_context {
        self.context
    }
    ///Gets the value of [`Self::window`]
    pub fn window(&self) -> *mut _screen_window {
        self.window
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut ScreenSurfaceCreateFlagsQNX {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::context`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn context_mut(&mut self) -> &mut _screen_context {
        &mut *self.context
    }
    ///Gets a mutable reference to the value of [`Self::window`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn window_mut(&mut self) -> &mut _screen_window {
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
    pub fn set_flags(
        &mut self,
        value: crate::extensions::qnx_screen_surface::ScreenSurfaceCreateFlagsQNX,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::context`]
    pub fn set_context(&mut self, value: *mut crate::native::_screen_context) -> &mut Self {
        self.context = value;
        self
    }
    ///Sets the value of [`Self::window`]
    pub fn set_window(&mut self, value: *mut crate::native::_screen_window) -> &mut Self {
        self.window = value;
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
    pub fn with_flags(mut self, value: crate::extensions::qnx_screen_surface::ScreenSurfaceCreateFlagsQNX) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::context`]
    pub fn with_context(mut self, value: *mut crate::native::_screen_context) -> Self {
        self.context = value;
        self
    }
    ///Sets the value of [`Self::window`]
    pub fn with_window(mut self, value: *mut crate::native::_screen_window) -> Self {
        self.window = value;
        self
    }
}
impl Instance {
    ///[vkCreateScreenSurfaceQNX](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateScreenSurfaceQNX.html) - Create a slink:VkSurfaceKHR object for a QNX Screen window
    ///# C Specifications
    ///To create a [`SurfaceKHR`] object for a QNX Screen surface, call:
    ///```c
    ///// Provided by VK_QNX_screen_surface
    ///VkResult vkCreateScreenSurfaceQNX(
    ///    VkInstance                                  instance,
    ///    const VkScreenSurfaceCreateInfoQNX*         pCreateInfo,
    ///    const VkAllocationCallbacks*                pAllocator,
    ///    VkSurfaceKHR*                               pSurface);
    ///```
    /// # Parameters
    /// - [`instance`] is the instance to associate the surface with.
    /// - [`p_create_info`] is a pointer to a [`ScreenSurfaceCreateInfoQNX`] structure containing
    ///   parameters affecting the creation of the surface object.
    /// - [`p_allocator`] is the allocator used for host memory allocated for the surface object when there is no more specific allocator available (see [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)).
    /// - [`p_surface`] is a pointer to a [`SurfaceKHR`] handle in which the created surface object
    ///   is returned.
    /// # Description
    /// ## Valid Usage (Implicit)
    /// - [`instance`] **must**  be a valid [`Instance`] handle
    /// - [`p_create_info`] **must**  be a valid pointer to a valid [`ScreenSurfaceCreateInfoQNX`]
    ///   structure
    /// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
    ///   [`AllocationCallbacks`] structure
    /// - [`p_surface`] **must**  be a valid pointer to a [`SurfaceKHR`] handle
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// # Related
    /// - [`qnx_screen_surface`]
    /// - [`AllocationCallbacks`]
    /// - [`Instance`]
    /// - [`ScreenSurfaceCreateInfoQNX`]
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
    #[doc(alias = "vkCreateScreenSurfaceQNX")]
    #[track_caller]
    #[inline]
    pub unsafe fn create_screen_surface_qnx<'lt>(
        self: &Unique<Instance>,
        p_create_info: &ScreenSurfaceCreateInfoQNX<'lt>,
        p_allocator: Option<&AllocationCallbacks<'lt>>,
    ) -> VulkanResult<Unique<SurfaceKHR>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .qnx_screen_surface()
            .and_then(|vtable| vtable.create_screen_surface_qnx())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .qnx_screen_surface()
            .and_then(|vtable| vtable.create_screen_surface_qnx())
            .unwrap_unchecked();
        let mut p_surface = MaybeUninit::<SurfaceKHR>::uninit();
        let _return = _function(
            self.as_raw(),
            p_create_info as *const ScreenSurfaceCreateInfoQNX<'lt>,
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
    ///[vkGetPhysicalDeviceScreenPresentationSupportQNX](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceScreenPresentationSupportQNX.html) - Query physical device for presentation to QNX Screen
    ///# C Specifications
    ///To determine whether a queue family of a physical device supports
    ///presentation to a QNX Screen compositor, call:
    ///```c
    ///// Provided by VK_QNX_screen_surface
    ///VkBool32 vkGetPhysicalDeviceScreenPresentationSupportQNX(
    ///    VkPhysicalDevice                            physicalDevice,
    ///    uint32_t                                    queueFamilyIndex,
    ///    struct _screen_window*                      window);
    ///```
    /// # Parameters
    /// - [`physical_device`] is the physical device.
    /// - [`queue_family_index`] is the queue family index.
    /// - [`window`] is the QNX Screen [`window`] object.
    /// # Description
    /// This platform-specific function  **can**  be called prior to creating a surface.
    /// ## Valid Usage
    /// - [`queue_family_index`] **must**  be less than `pQueueFamilyPropertyCount` returned by
    ///   [`get_physical_device_queue_family_properties`] for the given [`physical_device`]
    ///
    /// ## Valid Usage (Implicit)
    /// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
    /// - [`window`] **must**  be a valid pointer to a [`_screen_window`] value
    /// # Related
    /// - [`qnx_screen_surface`]
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
    #[doc(alias = "vkGetPhysicalDeviceScreenPresentationSupportQNX")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_physical_device_screen_presentation_support_qnx(
        self: &Unique<PhysicalDevice>,
        queue_family_index: Option<u32>,
    ) -> (_screen_window, bool) {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .instance()
            .vtable()
            .qnx_screen_surface()
            .and_then(|vtable| vtable.get_physical_device_screen_presentation_support_qnx())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .instance()
            .vtable()
            .qnx_screen_surface()
            .and_then(|vtable| vtable.get_physical_device_screen_presentation_support_qnx())
            .unwrap_unchecked();
        let mut window = std::mem::zeroed();
        let _return = _function(self.as_raw(), queue_family_index.unwrap_or_default() as _, &mut window);
        (window, std::mem::transmute(_return as u8))
    }
}
///The V-table of [`Instance`] for functions from `VK_QNX_screen_surface`
pub struct InstanceQnxScreenSurfaceVTable {
    ///See [`FNCreateScreenSurfaceQnx`] for more information.
    pub create_screen_surface_qnx: FNCreateScreenSurfaceQnx,
    ///See [`FNGetPhysicalDeviceScreenPresentationSupportQnx`] for more information.
    pub get_physical_device_screen_presentation_support_qnx: FNGetPhysicalDeviceScreenPresentationSupportQnx,
}
impl InstanceQnxScreenSurfaceVTable {
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
            create_screen_surface_qnx: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCreateScreenSurfaceQNX").as_ptr()))
            },
            get_physical_device_screen_presentation_support_qnx: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetPhysicalDeviceScreenPresentationSupportQNX").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::create_screen_surface_qnx`]. See [`FNCreateScreenSurfaceQnx`] for more
    /// information.
    pub fn create_screen_surface_qnx(&self) -> FNCreateScreenSurfaceQnx {
        self.create_screen_surface_qnx
    }
    ///Gets [`Self::get_physical_device_screen_presentation_support_qnx`]. See
    /// [`FNGetPhysicalDeviceScreenPresentationSupportQnx`] for more information.
    pub fn get_physical_device_screen_presentation_support_qnx(
        &self,
    ) -> FNGetPhysicalDeviceScreenPresentationSupportQnx {
        self.get_physical_device_screen_presentation_support_qnx
    }
}
