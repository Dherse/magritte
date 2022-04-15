//![VK_FUCHSIA_imagepipe_surface](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_FUCHSIA_imagepipe_surface.html) - instance extension
//!# Description
//!The [`VK_FUCHSIA_imagepipe_surface`] extension is an instance extension.
//!It provides a mechanism to create a [`SurfaceKHR`] object (defined by
//!the `[`khr_surface`]` extension) that refers to a Fuchsia
//!`imagePipeHandle`.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_surface`]`
//!# Contacts
//! - Craig Stout [cdotstout](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_FUCHSIA_imagepipe_surface]
//!   @cdotstout%0A<<Here describe the issue or question you have about the
//!   VK_FUCHSIA_imagepipe_surface extension>>)
//!# New functions & commands
//! - [`create_image_pipe_surface_fuchsia`]
//!# New structures
//! - [`ImagePipeSurfaceCreateInfoFUCHSIA`]
//!# New bitmasks
//! - [`ImagePipeSurfaceCreateFlagsFUCHSIA`]
//!# New constants
//! - [`FUCHSIA_IMAGEPIPE_SURFACE_EXTENSION_NAME`]
//! - [`FUCHSIA_IMAGEPIPE_SURFACE_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA`
//!# Version History
//! - Revision 1, 2018-07-27 (Craig Stout)  - Initial draft.
//!# Other info
//! * 2018-07-27
//! * No known IP claims.
//! * - Craig Stout, Google  - Ian Elliott, Google  - Jesse Hall, Google
//!# Related
//! - [`ImagePipeSurfaceCreateFlagsFUCHSIA`]
//! - [`ImagePipeSurfaceCreateInfoFUCHSIA`]
//! - [`create_image_pipe_surface_fuchsia`]
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
    native::zx_handle_t,
    vulkan1_0::{AllocationCallbacks, BaseInStructure, Instance, StructureType, VulkanResultCodes},
    AsRaw, Unique, VulkanResult,
};
use std::{ffi::CStr, marker::PhantomData, mem::MaybeUninit, sync::atomic::AtomicBool};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_FUCHSIA_IMAGEPIPE_SURFACE_SPEC_VERSION")]
pub const FUCHSIA_IMAGEPIPE_SURFACE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_FUCHSIA_IMAGEPIPE_SURFACE_EXTENSION_NAME")]
pub const FUCHSIA_IMAGEPIPE_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_FUCHSIA_imagepipe_surface");
///[vkCreateImagePipeSurfaceFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateImagePipeSurfaceFUCHSIA.html) - Create a slink:VkSurfaceKHR object for a Fuchsia ImagePipe
///# C Specifications
///To create a [`SurfaceKHR`] object for a Fuchsia ImagePipe, call:
///```c
///// Provided by VK_FUCHSIA_imagepipe_surface
///VkResult vkCreateImagePipeSurfaceFUCHSIA(
///    VkInstance                                  instance,
///    const VkImagePipeSurfaceCreateInfoFUCHSIA*  pCreateInfo,
///    const VkAllocationCallbacks*                pAllocator,
///    VkSurfaceKHR*                               pSurface);
///```
/// # Parameters
/// - [`instance`] is the instance to associate with the surface.
/// - [`p_create_info`] is a pointer to a [`ImagePipeSurfaceCreateInfoFUCHSIA`] structure containing
///   parameters affecting the creation of the surface object.
/// - [`p_allocator`] is the allocator used for host memory allocated for the surface object when there is no more specific allocator available (see [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)).
/// - [`p_surface`] is a pointer to a [`SurfaceKHR`] handle in which the created surface object is
///   returned.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`instance`] **must**  be a valid [`Instance`] handle
/// - [`p_create_info`] **must**  be a valid pointer to a valid
///   [`ImagePipeSurfaceCreateInfoFUCHSIA`] structure
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
/// - [`p_surface`] **must**  be a valid pointer to a [`SurfaceKHR`] handle
///
/// ## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
/// # Related
/// - [`fuchsia_imagepipe_surface`]
/// - [`AllocationCallbacks`]
/// - [`ImagePipeSurfaceCreateInfoFUCHSIA`]
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
#[doc(alias = "vkCreateImagePipeSurfaceFUCHSIA")]
pub type FNCreateImagePipeSurfaceFuchsia = Option<
    for<'lt> unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const ImagePipeSurfaceCreateInfoFUCHSIA<'lt>,
        p_allocator: *const AllocationCallbacks<'lt>,
        p_surface: *mut SurfaceKHR,
    ) -> VulkanResultCodes,
>;
///[VkImagePipeSurfaceCreateFlagsFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImagePipeSurfaceCreateFlagsFUCHSIA.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_FUCHSIA_imagepipe_surface
///typedef VkFlags VkImagePipeSurfaceCreateFlagsFUCHSIA;
///```
/// # Related
/// - [`fuchsia_imagepipe_surface`]
/// - [`ImagePipeSurfaceCreateInfoFUCHSIA`]
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
pub struct ImagePipeSurfaceCreateFlagsFUCHSIA(u32);
impl const Default for ImagePipeSurfaceCreateFlagsFUCHSIA {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for ImagePipeSurfaceCreateFlagsFUCHSIA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(ImagePipeSurfaceCreateFlagsFUCHSIA))
            .field(&self.0)
            .finish()
    }
}
///[VkImagePipeSurfaceCreateInfoFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImagePipeSurfaceCreateInfoFUCHSIA.html) - Structure specifying parameters of a newly created ImagePipe surface object
///# C Specifications
///The [`ImagePipeSurfaceCreateInfoFUCHSIA`] structure is defined as:
///```c
///// Provided by VK_FUCHSIA_imagepipe_surface
///typedef struct VkImagePipeSurfaceCreateInfoFUCHSIA {
///    VkStructureType                         sType;
///    const void*                             pNext;
///    VkImagePipeSurfaceCreateFlagsFUCHSIA    flags;
///    zx_handle_t                             imagePipeHandle;
///} VkImagePipeSurfaceCreateInfoFUCHSIA;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`image_pipe_handle`] is a [`zx_handle_t`] referring to the ImagePipe to associate with the
///   surface.
/// # Description
/// ## Valid Usage
/// - [`image_pipe_handle`] **must**  be a valid [`zx_handle_t`]
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA`
/// - [`p_next`] **must**  be `NULL`
/// - [`flags`] **must**  be `0`
/// # Related
/// - [`fuchsia_imagepipe_surface`]
/// - [`ImagePipeSurfaceCreateFlagsFUCHSIA`]
/// - [`StructureType`]
/// - [`create_image_pipe_surface_fuchsia`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkImagePipeSurfaceCreateInfoFUCHSIA")]
#[repr(C)]
pub struct ImagePipeSurfaceCreateInfoFUCHSIA<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    pub flags: ImagePipeSurfaceCreateFlagsFUCHSIA,
    ///[`image_pipe_handle`] is a [`zx_handle_t`] referring to the ImagePipe
    ///to associate with the surface.
    pub image_pipe_handle: zx_handle_t,
}
impl<'lt> Default for ImagePipeSurfaceCreateInfoFUCHSIA<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA,
            p_next: std::ptr::null(),
            flags: Default::default(),
            image_pipe_handle: unsafe { std::mem::zeroed() },
        }
    }
}
impl<'lt> ImagePipeSurfaceCreateInfoFUCHSIA<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::image_pipe_handle`]
    pub fn image_pipe_handle_raw(&self) -> &zx_handle_t {
        &self.image_pipe_handle
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::image_pipe_handle`]
    pub fn set_image_pipe_handle_raw(mut self, value: zx_handle_t) -> Self {
        self.image_pipe_handle = value;
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
    pub fn flags(&self) -> ImagePipeSurfaceCreateFlagsFUCHSIA {
        self.flags
    }
    ///Gets the value of [`Self::image_pipe_handle`]
    pub fn image_pipe_handle(&self) -> zx_handle_t {
        self.image_pipe_handle
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut ImagePipeSurfaceCreateFlagsFUCHSIA {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::image_pipe_handle`]
    pub fn image_pipe_handle_mut(&mut self) -> &mut zx_handle_t {
        &mut self.image_pipe_handle
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
    pub fn set_flags(
        mut self,
        value: crate::extensions::fuchsia_imagepipe_surface::ImagePipeSurfaceCreateFlagsFUCHSIA,
    ) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::image_pipe_handle`]
    pub fn set_image_pipe_handle(mut self, value: crate::native::zx_handle_t) -> Self {
        self.image_pipe_handle = value;
        self
    }
}
impl Instance {
    ///[vkCreateImagePipeSurfaceFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateImagePipeSurfaceFUCHSIA.html) - Create a slink:VkSurfaceKHR object for a Fuchsia ImagePipe
    ///# C Specifications
    ///To create a [`SurfaceKHR`] object for a Fuchsia ImagePipe, call:
    ///```c
    ///// Provided by VK_FUCHSIA_imagepipe_surface
    ///VkResult vkCreateImagePipeSurfaceFUCHSIA(
    ///    VkInstance                                  instance,
    ///    const VkImagePipeSurfaceCreateInfoFUCHSIA*  pCreateInfo,
    ///    const VkAllocationCallbacks*                pAllocator,
    ///    VkSurfaceKHR*                               pSurface);
    ///```
    /// # Parameters
    /// - [`instance`] is the instance to associate with the surface.
    /// - [`p_create_info`] is a pointer to a [`ImagePipeSurfaceCreateInfoFUCHSIA`] structure
    ///   containing parameters affecting the creation of the surface object.
    /// - [`p_allocator`] is the allocator used for host memory allocated for the surface object when there is no more specific allocator available (see [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)).
    /// - [`p_surface`] is a pointer to a [`SurfaceKHR`] handle in which the created surface object
    ///   is returned.
    /// # Description
    /// ## Valid Usage (Implicit)
    /// - [`instance`] **must**  be a valid [`Instance`] handle
    /// - [`p_create_info`] **must**  be a valid pointer to a valid
    ///   [`ImagePipeSurfaceCreateInfoFUCHSIA`] structure
    /// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
    ///   [`AllocationCallbacks`] structure
    /// - [`p_surface`] **must**  be a valid pointer to a [`SurfaceKHR`] handle
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// # Related
    /// - [`fuchsia_imagepipe_surface`]
    /// - [`AllocationCallbacks`]
    /// - [`ImagePipeSurfaceCreateInfoFUCHSIA`]
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
    #[doc(alias = "vkCreateImagePipeSurfaceFUCHSIA")]
    #[track_caller]
    #[inline]
    pub unsafe fn create_image_pipe_surface_fuchsia<'lt>(
        self: &Unique<Instance>,
        p_create_info: &ImagePipeSurfaceCreateInfoFUCHSIA<'lt>,
        p_allocator: Option<&AllocationCallbacks<'lt>>,
    ) -> VulkanResult<Unique<SurfaceKHR>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .fuchsia_imagepipe_surface()
            .and_then(|vtable| vtable.create_image_pipe_surface_fuchsia())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .fuchsia_imagepipe_surface()
            .and_then(|vtable| vtable.create_image_pipe_surface_fuchsia())
            .unwrap_unchecked();
        let mut p_surface = MaybeUninit::<SurfaceKHR>::uninit();
        let _return = _function(
            self.as_raw(),
            p_create_info as *const ImagePipeSurfaceCreateInfoFUCHSIA<'lt>,
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
///The V-table of [`Instance`] for functions from `VK_FUCHSIA_imagepipe_surface`
pub struct InstanceFuchsiaImagepipeSurfaceVTable {
    ///See [`FNCreateImagePipeSurfaceFuchsia`] for more information.
    pub create_image_pipe_surface_fuchsia: FNCreateImagePipeSurfaceFuchsia,
}
impl InstanceFuchsiaImagepipeSurfaceVTable {
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
            create_image_pipe_surface_fuchsia: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCreateImagePipeSurfaceFUCHSIA").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::create_image_pipe_surface_fuchsia`]. See [`FNCreateImagePipeSurfaceFuchsia`]
    /// for more information.
    pub fn create_image_pipe_surface_fuchsia(&self) -> FNCreateImagePipeSurfaceFuchsia {
        self.create_image_pipe_surface_fuchsia
    }
}
