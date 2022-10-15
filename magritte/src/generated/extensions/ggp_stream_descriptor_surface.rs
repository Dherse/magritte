//![VK_GGP_stream_descriptor_surface](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_GGP_stream_descriptor_surface.html) - instance extension
//!# Description
//!The [`VK_GGP_stream_descriptor_surface`] extension is an instance extension.
//!It provides a mechanism to create a [`SurfaceKHR`] object (defined by
//!the `[`khr_surface`]` extension) that refers to a Google Games
//!Platform [`GgpStreamDescriptor`].
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_surface`]`
//!# Contacts
//! - Jean-Francois Roy [jfroy](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_GGP_stream_descriptor_surface]
//!   @jfroy%0A<<Here describe the issue or question you have about the
//!   VK_GGP_stream_descriptor_surface extension>>)
//!# New commands
//! - [`create_stream_descriptor_surface_ggp`]
//!# New structures
//! - [`StreamDescriptorSurfaceCreateInfoGGP`]
//!# New bitmasks
//! - [`StreamDescriptorSurfaceCreateFlagsGGP`]
//!# New constants
//! - [`GGP_STREAM_DESCRIPTOR_SURFACE_EXTENSION_NAME`]
//! - [`GGP_STREAM_DESCRIPTOR_SURFACE_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP`
//!# Version history
//! - Revision 1, 2018-11-26 (Jean-Francois Roy)  - Initial revision.
//!# Other information
//! * 2019-01-28
//! * No known IP claims.
//! * - Jean-Francois Roy, Google  - Brad Grantham, Google  - Connor Smith, Google  - Cort Stratton,
//!   Google  - Hai Nguyen, Google  - Ian Elliott, Google  - Jesse Hall, Google  - Jim Ray, Google
//!   - Katherine Wu, Google  - Kaye Mason, Google  - Kuangye Guo, Google  - Mark Segal, Google  -
//!   Nicholas Vining, Google  - Paul Lalonde, Google  - Richard O’Grady, Google
//!# Related
//! - [`StreamDescriptorSurfaceCreateFlagsGGP`]
//! - [`StreamDescriptorSurfaceCreateInfoGGP`]
//! - [`create_stream_descriptor_surface_ggp`]
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
    native::GgpStreamDescriptor,
    vulkan1_0::{AllocationCallbacks, BaseInStructure, Instance, StructureType, VulkanResultCodes},
    AsRaw, Unique, VulkanResult,
};
use std::{ffi::CStr, marker::PhantomData, mem::MaybeUninit, sync::atomic::AtomicBool};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_GGP_STREAM_DESCRIPTOR_SURFACE_SPEC_VERSION")]
pub const GGP_STREAM_DESCRIPTOR_SURFACE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_GGP_STREAM_DESCRIPTOR_SURFACE_EXTENSION_NAME")]
pub const GGP_STREAM_DESCRIPTOR_SURFACE_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_GGP_stream_descriptor_surface");
///[vkCreateStreamDescriptorSurfaceGGP](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateStreamDescriptorSurfaceGGP.html) - Create a slink:VkSurfaceKHR object for a Google Games Platform stream
///# C Specifications
///To create a [`SurfaceKHR`] object for a Google Games Platform stream
///descriptor, call:
///```c
///// Provided by VK_GGP_stream_descriptor_surface
///VkResult vkCreateStreamDescriptorSurfaceGGP(
///    VkInstance                                  instance,
///    const VkStreamDescriptorSurfaceCreateInfoGGP* pCreateInfo,
///    const VkAllocationCallbacks*                pAllocator,
///    VkSurfaceKHR*                               pSurface);
///```
/// # Parameters
/// - [`instance`] is the instance to associate with the surface.
/// - [`p_create_info`] is a pointer to a [`StreamDescriptorSurfaceCreateInfoGGP`] structure
///   containing parameters that affect the creation of the surface object.
/// - [`p_allocator`] is the allocator used for host memory allocated for the surface object when there is no more specific allocator available (see [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)).
/// - [`p_surface`] is a pointer to a [`SurfaceKHR`] handle in which the created surface object is
///   returned.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`instance`] **must**  be a valid [`Instance`] handle
/// - [`p_create_info`] **must**  be a valid pointer to a valid
///   [`StreamDescriptorSurfaceCreateInfoGGP`] structure
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
/// - [`p_surface`] **must**  be a valid pointer to a [`SurfaceKHR`] handle
///
/// ## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
///   `VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`
/// # Related
/// - [`ggp_stream_descriptor_surface`]
/// - [`AllocationCallbacks`]
/// - [`Instance`]
/// - [`StreamDescriptorSurfaceCreateInfoGGP`]
/// - [`SurfaceKHR`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCreateStreamDescriptorSurfaceGGP")]
pub type FNCreateStreamDescriptorSurfaceGgp = Option<
    for<'lt> unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const StreamDescriptorSurfaceCreateInfoGGP<'lt>,
        p_allocator: *const AllocationCallbacks<'lt>,
        p_surface: *mut SurfaceKHR,
    ) -> VulkanResultCodes,
>;
///[VkStreamDescriptorSurfaceCreateFlagsGGP](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkStreamDescriptorSurfaceCreateFlagsGGP.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_GGP_stream_descriptor_surface
///typedef VkFlags VkStreamDescriptorSurfaceCreateFlagsGGP;
///```
/// # Related
/// - [`ggp_stream_descriptor_surface`]
/// - [`StreamDescriptorSurfaceCreateInfoGGP`]
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
pub struct StreamDescriptorSurfaceCreateFlagsGGP(u32);
impl const Default for StreamDescriptorSurfaceCreateFlagsGGP {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for StreamDescriptorSurfaceCreateFlagsGGP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(StreamDescriptorSurfaceCreateFlagsGGP))
            .field(&self.0)
            .finish()
    }
}
///[VkStreamDescriptorSurfaceCreateInfoGGP](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkStreamDescriptorSurfaceCreateInfoGGP.html) - Structure specifying parameters of a newly created Google Games Platform stream surface object
///# C Specifications
///The [`StreamDescriptorSurfaceCreateInfoGGP`] structure is defined as:
///```c
///// Provided by VK_GGP_stream_descriptor_surface
///typedef struct VkStreamDescriptorSurfaceCreateInfoGGP {
///    VkStructureType                            sType;
///    const void*                                pNext;
///    VkStreamDescriptorSurfaceCreateFlagsGGP    flags;
///    GgpStreamDescriptor                        streamDescriptor;
///} VkStreamDescriptorSurfaceCreateInfoGGP;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`stream_descriptor`] is a [`GgpStreamDescriptor`] referring to the GGP stream descriptor to
///   associate with the surface.
/// # Description
/// ## Valid Usage
/// - [`stream_descriptor`] **must**  be a valid [`GgpStreamDescriptor`]
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP`
/// - [`p_next`] **must**  be `NULL`
/// - [`flags`] **must**  be `0`
/// # Related
/// - [`ggp_stream_descriptor_surface`]
/// - [`StreamDescriptorSurfaceCreateFlagsGGP`]
/// - [`StructureType`]
/// - [`create_stream_descriptor_surface_ggp`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkStreamDescriptorSurfaceCreateInfoGGP")]
#[repr(C)]
pub struct StreamDescriptorSurfaceCreateInfoGGP<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    pub flags: StreamDescriptorSurfaceCreateFlagsGGP,
    ///[`stream_descriptor`] is a [`GgpStreamDescriptor`] referring to the
    ///GGP stream descriptor to associate with the surface.
    pub stream_descriptor: GgpStreamDescriptor,
}
impl<'lt> Default for StreamDescriptorSurfaceCreateInfoGGP<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP,
            p_next: std::ptr::null(),
            flags: Default::default(),
            stream_descriptor: unsafe { std::mem::zeroed() },
        }
    }
}
impl<'lt> StreamDescriptorSurfaceCreateInfoGGP<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::stream_descriptor`]
    pub fn stream_descriptor_raw(&self) -> &GgpStreamDescriptor {
        &self.stream_descriptor
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::stream_descriptor`]
    pub fn set_stream_descriptor_raw(&mut self, value: GgpStreamDescriptor) -> &mut Self {
        self.stream_descriptor = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::stream_descriptor`]
    pub fn with_stream_descriptor_raw(mut self, value: GgpStreamDescriptor) -> Self {
        self.stream_descriptor = value;
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
    pub fn flags(&self) -> StreamDescriptorSurfaceCreateFlagsGGP {
        self.flags
    }
    ///Gets the value of [`Self::stream_descriptor`]
    pub fn stream_descriptor(&self) -> GgpStreamDescriptor {
        self.stream_descriptor
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut StreamDescriptorSurfaceCreateFlagsGGP {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::stream_descriptor`]
    pub fn stream_descriptor_mut(&mut self) -> &mut GgpStreamDescriptor {
        &mut self.stream_descriptor
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
        value: crate::extensions::ggp_stream_descriptor_surface::StreamDescriptorSurfaceCreateFlagsGGP,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::stream_descriptor`]
    pub fn set_stream_descriptor(&mut self, value: crate::native::GgpStreamDescriptor) -> &mut Self {
        self.stream_descriptor = value;
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
    pub fn with_flags(
        mut self,
        value: crate::extensions::ggp_stream_descriptor_surface::StreamDescriptorSurfaceCreateFlagsGGP,
    ) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::stream_descriptor`]
    pub fn with_stream_descriptor(mut self, value: crate::native::GgpStreamDescriptor) -> Self {
        self.stream_descriptor = value;
        self
    }
}
impl Instance {
    ///[vkCreateStreamDescriptorSurfaceGGP](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateStreamDescriptorSurfaceGGP.html) - Create a slink:VkSurfaceKHR object for a Google Games Platform stream
    ///# C Specifications
    ///To create a [`SurfaceKHR`] object for a Google Games Platform stream
    ///descriptor, call:
    ///```c
    ///// Provided by VK_GGP_stream_descriptor_surface
    ///VkResult vkCreateStreamDescriptorSurfaceGGP(
    ///    VkInstance                                  instance,
    ///    const VkStreamDescriptorSurfaceCreateInfoGGP* pCreateInfo,
    ///    const VkAllocationCallbacks*                pAllocator,
    ///    VkSurfaceKHR*                               pSurface);
    ///```
    /// # Parameters
    /// - [`instance`] is the instance to associate with the surface.
    /// - [`p_create_info`] is a pointer to a [`StreamDescriptorSurfaceCreateInfoGGP`] structure
    ///   containing parameters that affect the creation of the surface object.
    /// - [`p_allocator`] is the allocator used for host memory allocated for the surface object when there is no more specific allocator available (see [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)).
    /// - [`p_surface`] is a pointer to a [`SurfaceKHR`] handle in which the created surface object
    ///   is returned.
    /// # Description
    /// ## Valid Usage (Implicit)
    /// - [`instance`] **must**  be a valid [`Instance`] handle
    /// - [`p_create_info`] **must**  be a valid pointer to a valid
    ///   [`StreamDescriptorSurfaceCreateInfoGGP`] structure
    /// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
    ///   [`AllocationCallbacks`] structure
    /// - [`p_surface`] **must**  be a valid pointer to a [`SurfaceKHR`] handle
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
    ///   `VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`
    /// # Related
    /// - [`ggp_stream_descriptor_surface`]
    /// - [`AllocationCallbacks`]
    /// - [`Instance`]
    /// - [`StreamDescriptorSurfaceCreateInfoGGP`]
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
    #[doc(alias = "vkCreateStreamDescriptorSurfaceGGP")]
    #[track_caller]
    #[inline]
    pub unsafe fn create_stream_descriptor_surface_ggp<'lt>(
        self: &Unique<Instance>,
        p_create_info: &StreamDescriptorSurfaceCreateInfoGGP<'lt>,
        p_allocator: Option<&AllocationCallbacks<'lt>>,
    ) -> VulkanResult<Unique<SurfaceKHR>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .ggp_stream_descriptor_surface()
            .and_then(|vtable| vtable.create_stream_descriptor_surface_ggp())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .ggp_stream_descriptor_surface()
            .and_then(|vtable| vtable.create_stream_descriptor_surface_ggp())
            .unwrap_unchecked();
        let mut p_surface = MaybeUninit::<SurfaceKHR>::uninit();
        let _return = _function(
            self.as_raw(),
            p_create_info as *const StreamDescriptorSurfaceCreateInfoGGP<'lt>,
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
///The V-table of [`Instance`] for functions from `VK_GGP_stream_descriptor_surface`
pub struct InstanceGgpStreamDescriptorSurfaceVTable {
    ///See [`FNCreateStreamDescriptorSurfaceGgp`] for more information.
    pub create_stream_descriptor_surface_ggp: FNCreateStreamDescriptorSurfaceGgp,
}
impl InstanceGgpStreamDescriptorSurfaceVTable {
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
            create_stream_descriptor_surface_ggp: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCreateStreamDescriptorSurfaceGGP").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::create_stream_descriptor_surface_ggp`]. See
    /// [`FNCreateStreamDescriptorSurfaceGgp`] for more information.
    pub fn create_stream_descriptor_surface_ggp(&self) -> FNCreateStreamDescriptorSurfaceGgp {
        self.create_stream_descriptor_surface_ggp
    }
}
