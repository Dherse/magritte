//![VK_MVK_ios_surface](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MVK_ios_surface.html) - instance extension
//!# Description
//!The [`VK_MVK_ios_surface`] extension is an instance extension.
//!It provides a mechanism to create a [`SurfaceKHR`] object (defined by
//!the `[`VK_KHR_surface`]` extension) based on a `UIView`, the native
//!surface type of iOS, which is underpinned by a [`CaMetalLayer`], to
//!support rendering to the surface using Apple’s Metal framework.
//!# Revision
//!3
//!# Dependencies
//! - *Deprecated* by `[`VK_EXT_metal_surface`]` extension
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_surface`]`
//!# Contacts
//! - Bill Hollings [billhollings](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_MVK_ios_surface]
//!   @billhollings%0A<<Here describe the issue or question you have about the VK_MVK_ios_surface
//!   extension>>)
//!# New functions & commands
//! - [`CreateIosSurfaceMVK`]
//!# New structures
//! - [`IosSurfaceCreateInfoMVK`]
//!# New bitmasks
//! - [`IosSurfaceCreateFlagsMVK`]
//!# New constants
//! - [`MVK_IOS_SURFACE_EXTENSION_NAME`]
//! - [`MVK_IOS_SURFACE_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_IOS_SURFACE_CREATE_INFO_MVK`
//!# Version History
//! - Revision 1, 2017-02-15 (Bill Hollings)  - Initial draft.
//! - Revision 2, 2017-02-24 (Bill Hollings)  - Minor syntax fix to emphasize firm requirement for
//!   `UIView` to be backed by a [`CaMetalLayer`].
//! - Revision 3, 2020-07-31 (Bill Hollings)  - Update documentation on requirements for UIView.  -
//!   Mark as deprecated by [`VK_EXT_metal_surface`].
//!# Other info
//! * 2020-07-31
//! * No known IP claims.
//! * - Bill Hollings, The Brenwill Workshop Ltd.
//!# Related
//! - [`IosSurfaceCreateFlagsMVK`]
//! - [`IosSurfaceCreateInfoMVK`]
//! - [`CreateIosSurfaceMVK`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{BaseInStructure, StructureType};
use std::{
    ffi::{c_void, CStr},
    marker::PhantomData,
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_MVK_IOS_SURFACE_SPEC_VERSION")]
pub const MVK_IOS_SURFACE_SPEC_VERSION: u32 = 3;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_MVK_IOS_SURFACE_EXTENSION_NAME")]
pub const MVK_IOS_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_MVK_ios_surface");
///[VkIOSSurfaceCreateFlagsMVK](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIOSSurfaceCreateFlagsMVK.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_MVK_ios_surface
///typedef VkFlags VkIOSSurfaceCreateFlagsMVK;
///```
///# Related
/// - [`VK_MVK_ios_surface`]
/// - [`IosSurfaceCreateInfoMVK`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct IosSurfaceCreateFlagsMVK(u32);
impl const Default for IosSurfaceCreateFlagsMVK {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for IosSurfaceCreateFlagsMVK {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(IosSurfaceCreateFlagsMVK))
            .field(&self.0)
            .finish()
    }
}
///[VkIOSSurfaceCreateInfoMVK](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIOSSurfaceCreateInfoMVK.html) - Structure specifying parameters of a newly created iOS surface object
///# C Specifications
///The [`IosSurfaceCreateInfoMVK`] structure is defined as:
///```c
///// Provided by VK_MVK_ios_surface
///typedef struct VkIOSSurfaceCreateInfoMVK {
///    VkStructureType               sType;
///    const void*                   pNext;
///    VkIOSSurfaceCreateFlagsMVK    flags;
///    const void*                   pView;
///} VkIOSSurfaceCreateInfoMVK;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`view`] is a reference to either a [`CaMetalLayer`] object or a `UIView` object.
///# Description
///## Valid Usage
/// - If [`view`] is a [`CaMetalLayer`] object, it  **must**  be a valid [`CaMetalLayer`]
/// - If [`view`] is a `UIView` object, it  **must**  be a valid `UIView`,  **must**  be backed by a
///   `CALayer` object of type [`CaMetalLayer`], and [`CreateIosSurfaceMVK`] **must**  be called on
///   the main thread
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IOS_SURFACE_CREATE_INFO_MVK`
/// - [`p_next`] **must**  be `NULL`
/// - [`flags`] **must**  be `0`
///# Related
/// - [`VK_MVK_ios_surface`]
/// - [`IosSurfaceCreateFlagsMVK`]
/// - [`StructureType`]
/// - [`CreateIosSurfaceMVK`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkIOSSurfaceCreateInfoMVK")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct IosSurfaceCreateInfoMVK<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    pub flags: IosSurfaceCreateFlagsMVK,
    ///[`view`] is a reference to either a [`CaMetalLayer`] object or a
    ///`UIView` object.
    pub view: *const c_void,
}
impl<'lt> Default for IosSurfaceCreateInfoMVK<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
            view: std::ptr::null(),
        }
    }
}
impl<'lt> IosSurfaceCreateInfoMVK<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::view`]
    pub fn view_raw(&self) -> *const c_void {
        self.view
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::view`]
    pub fn set_view_raw(&mut self, value: *const c_void) -> &mut Self {
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
    pub fn flags(&self) -> IosSurfaceCreateFlagsMVK {
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
    pub fn flags_mut(&mut self) -> &mut IosSurfaceCreateFlagsMVK {
        &mut self.flags
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
    pub fn set_flags(&mut self, value: crate::extensions::mvk_ios_surface::IosSurfaceCreateFlagsMVK) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::view`]
    pub fn set_view(&mut self, value: &'lt std::ffi::c_void) -> &mut Self {
        self.view = value as *const _;
        self
    }
}
