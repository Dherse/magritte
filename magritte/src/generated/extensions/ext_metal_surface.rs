//![VK_EXT_metal_surface](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_metal_surface.html) - instance extension
//!# Description
//!The [`VK_EXT_metal_surface`] extension is an instance extension.
//!It provides a mechanism to create a [`SurfaceKHR`] object (defined by
//!the `[`khr_surface`]` extension) from [`CaMetalLayer`], which is
//!the native rendering surface of Apple’s Metal framework.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_surface`]`
//!# Contacts
//! - Dzmitry Malyshau [kvark](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_metal_surface]
//!   @kvark%0A<<Here describe the issue or question you have about the VK_EXT_metal_surface
//!   extension>>)
//!# New base types
//! - [`CaMetalLayer`]
//!# New commands
//! - [`create_metal_surface_ext`]
//!# New structures
//! - [`MetalSurfaceCreateInfoEXT`]
//!# New bitmasks
//! - [`MetalSurfaceCreateFlagsEXT`]
//!# New constants
//! - [`EXT_METAL_SURFACE_EXTENSION_NAME`]
//! - [`EXT_METAL_SURFACE_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_METAL_SURFACE_CREATE_INFO_EXT`
//!# Version history
//! - Revision 1, 2018-10-01 (Dzmitry Malyshau)  - Initial version
//!# Other information
//! * 2018-10-01
//! * No known IP claims.
//! * - Dzmitry Malyshau, Mozilla Corp.
//!# Related
//! - [`CaMetalLayer`]
//! - [`MetalSurfaceCreateFlagsEXT`]
//! - [`MetalSurfaceCreateInfoEXT`]
//! - [`create_metal_surface_ext`]
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
#[doc(alias = "VK_EXT_METAL_SURFACE_SPEC_VERSION")]
pub const EXT_METAL_SURFACE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_METAL_SURFACE_EXTENSION_NAME")]
pub const EXT_METAL_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_metal_surface");
///[CAMetalLayer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/CAMetalLayer.html) - CoreAnimation native layer type for Metal
///# C Specifications
///To remove an unnecessary compile-time dependency, an incomplete type
///definition of [`CaMetalLayer`] is provided in the Vulkan headers:
///```c
///// Provided by VK_EXT_metal_surface
///
///#ifdef __OBJC__
///@class CAMetalLayer;
///#else
///typedef void CAMetalLayer;
///#endif
///```
///# Related
/// - [`ext_metal_surface`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "CAMetalLayer")]
pub type CaMetalLayer = c_void;
///[vkCreateMetalSurfaceEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateMetalSurfaceEXT.html) - Create a VkSurfaceKHR object for CAMetalLayer
///# C Specifications
///To create a [`SurfaceKHR`] object for a [`CaMetalLayer`], call:
///```c
///// Provided by VK_EXT_metal_surface
///VkResult vkCreateMetalSurfaceEXT(
///    VkInstance                                  instance,
///    const VkMetalSurfaceCreateInfoEXT*          pCreateInfo,
///    const VkAllocationCallbacks*                pAllocator,
///    VkSurfaceKHR*                               pSurface);
///```
///# Parameters
/// - [`instance`] is the instance with which to associate the surface.
/// - [`p_create_info`] is a pointer to a [`MetalSurfaceCreateInfoEXT`] structure specifying
///   parameters affecting the creation of the surface object.
/// - [`p_allocator`] is the allocator used for host memory allocated for the surface object when there is no more specific allocator available (see [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)).
/// - [`p_surface`] is a pointer to a [`SurfaceKHR`] handle in which the created surface object is
///   returned.
///# Description
///## Valid Usage (Implicit)
/// - [`instance`] **must**  be a valid [`Instance`] handle
/// - [`p_create_info`] **must**  be a valid pointer to a valid [`MetalSurfaceCreateInfoEXT`]
///   structure
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
/// - [`p_surface`] **must**  be a valid pointer to a [`SurfaceKHR`] handle
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
///   `VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`
///# Related
/// - [`ext_metal_surface`]
/// - [`AllocationCallbacks`]
/// - [`Instance`]
/// - [`MetalSurfaceCreateInfoEXT`]
/// - [`SurfaceKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCreateMetalSurfaceEXT")]
pub type FNCreateMetalSurfaceExt = Option<
    for<'lt> unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const MetalSurfaceCreateInfoEXT<'lt>,
        p_allocator: *const AllocationCallbacks<'lt>,
        p_surface: *mut SurfaceKHR,
    ) -> VulkanResultCodes,
>;
///[VkMetalSurfaceCreateFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMetalSurfaceCreateFlagsEXT.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_EXT_metal_surface
///typedef VkFlags VkMetalSurfaceCreateFlagsEXT;
///```
///# Related
/// - [`ext_metal_surface`]
/// - [`MetalSurfaceCreateInfoEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct MetalSurfaceCreateFlagsEXT(u32);
impl const Default for MetalSurfaceCreateFlagsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for MetalSurfaceCreateFlagsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(MetalSurfaceCreateFlagsEXT))
            .field(&self.0)
            .finish()
    }
}
///[VkMetalSurfaceCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMetalSurfaceCreateInfoEXT.html) - Structure specifying parameters of a newly created Metal surface object
///# C Specifications
///The [`MetalSurfaceCreateInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_metal_surface
///typedef struct VkMetalSurfaceCreateInfoEXT {
///    VkStructureType                 sType;
///    const void*                     pNext;
///    VkMetalSurfaceCreateFlagsEXT    flags;
///    const CAMetalLayer*             pLayer;
///} VkMetalSurfaceCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`layer`] is a reference to a [`CaMetalLayer`] object representing a renderable surface.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_METAL_SURFACE_CREATE_INFO_EXT`
/// - [`p_next`] **must**  be `NULL`
/// - [`flags`] **must**  be `0`
///# Related
/// - [`ext_metal_surface`]
/// - [`MetalSurfaceCreateFlagsEXT`]
/// - [`StructureType`]
/// - [`create_metal_surface_ext`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkMetalSurfaceCreateInfoEXT")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct MetalSurfaceCreateInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    pub flags: MetalSurfaceCreateFlagsEXT,
    ///[`layer`] is a reference to a [`CaMetalLayer`] object
    ///representing a renderable surface.
    pub layer: *const CaMetalLayer,
}
impl<'lt> Default for MetalSurfaceCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::METAL_SURFACE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            flags: Default::default(),
            layer: std::ptr::null(),
        }
    }
}
impl<'lt> MetalSurfaceCreateInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::layer`]
    pub fn layer_raw(&self) -> *const CaMetalLayer {
        self.layer
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::layer`]
    pub fn set_layer_raw(&mut self, value: *const CaMetalLayer) -> &mut Self {
        self.layer = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::layer`]
    pub fn with_layer_raw(mut self, value: *const CaMetalLayer) -> Self {
        self.layer = value;
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
    pub fn flags(&self) -> MetalSurfaceCreateFlagsEXT {
        self.flags
    }
    ///Gets the value of [`Self::layer`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn layer(&self) -> &CaMetalLayer {
        &*self.layer
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut MetalSurfaceCreateFlagsEXT {
        &mut self.flags
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
    pub fn set_flags(&mut self, value: crate::extensions::ext_metal_surface::MetalSurfaceCreateFlagsEXT) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::layer`]
    pub fn set_layer(&mut self, value: &'lt crate::extensions::ext_metal_surface::CaMetalLayer) -> &mut Self {
        self.layer = value as *const _;
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
    pub fn with_flags(mut self, value: crate::extensions::ext_metal_surface::MetalSurfaceCreateFlagsEXT) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::layer`]
    pub fn with_layer(mut self, value: &'lt crate::extensions::ext_metal_surface::CaMetalLayer) -> Self {
        self.layer = value as *const _;
        self
    }
}
impl Instance {
    ///[vkCreateMetalSurfaceEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateMetalSurfaceEXT.html) - Create a VkSurfaceKHR object for CAMetalLayer
    ///# C Specifications
    ///To create a [`SurfaceKHR`] object for a [`CaMetalLayer`], call:
    ///```c
    ///// Provided by VK_EXT_metal_surface
    ///VkResult vkCreateMetalSurfaceEXT(
    ///    VkInstance                                  instance,
    ///    const VkMetalSurfaceCreateInfoEXT*          pCreateInfo,
    ///    const VkAllocationCallbacks*                pAllocator,
    ///    VkSurfaceKHR*                               pSurface);
    ///```
    ///# Parameters
    /// - [`instance`] is the instance with which to associate the surface.
    /// - [`p_create_info`] is a pointer to a [`MetalSurfaceCreateInfoEXT`] structure specifying
    ///   parameters affecting the creation of the surface object.
    /// - [`p_allocator`] is the allocator used for host memory allocated for the surface object when there is no more specific allocator available (see [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)).
    /// - [`p_surface`] is a pointer to a [`SurfaceKHR`] handle in which the created surface object
    ///   is returned.
    ///# Description
    ///## Valid Usage (Implicit)
    /// - [`instance`] **must**  be a valid [`Instance`] handle
    /// - [`p_create_info`] **must**  be a valid pointer to a valid [`MetalSurfaceCreateInfoEXT`]
    ///   structure
    /// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
    ///   [`AllocationCallbacks`] structure
    /// - [`p_surface`] **must**  be a valid pointer to a [`SurfaceKHR`] handle
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
    ///   `VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`
    ///# Related
    /// - [`ext_metal_surface`]
    /// - [`AllocationCallbacks`]
    /// - [`Instance`]
    /// - [`MetalSurfaceCreateInfoEXT`]
    /// - [`SurfaceKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCreateMetalSurfaceEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn create_metal_surface_ext<'lt>(
        self: &Unique<Instance>,
        p_create_info: &MetalSurfaceCreateInfoEXT<'lt>,
        p_allocator: Option<&AllocationCallbacks<'lt>>,
    ) -> VulkanResult<Unique<SurfaceKHR>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .ext_metal_surface()
            .and_then(|vtable| vtable.create_metal_surface_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .ext_metal_surface()
            .and_then(|vtable| vtable.create_metal_surface_ext())
            .unwrap_unchecked();
        let mut p_surface = MaybeUninit::<SurfaceKHR>::uninit();
        let _return = _function(
            self.as_raw(),
            p_create_info as *const MetalSurfaceCreateInfoEXT<'lt>,
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
///The V-table of [`Instance`] for functions from `VK_EXT_metal_surface`
pub struct InstanceExtMetalSurfaceVTable {
    ///See [`FNCreateMetalSurfaceExt`] for more information.
    pub create_metal_surface_ext: FNCreateMetalSurfaceExt,
}
impl InstanceExtMetalSurfaceVTable {
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
            create_metal_surface_ext: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCreateMetalSurfaceEXT").as_ptr()))
            },
        }
    }
    ///Gets [`Self::create_metal_surface_ext`]. See [`FNCreateMetalSurfaceExt`] for more
    /// information.
    pub fn create_metal_surface_ext(&self) -> FNCreateMetalSurfaceExt {
        self.create_metal_surface_ext
    }
}
