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
//! - [`CreateMacOsSurfaceMVK`]
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
//! - [`CreateMacOsSurfaceMVK`]
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
#[doc(alias = "VK_MVK_MACOS_SURFACE_SPEC_VERSION")]
pub const MVK_MACOS_SURFACE_SPEC_VERSION: u32 = 3;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_MVK_MACOS_SURFACE_EXTENSION_NAME")]
pub const MVK_MACOS_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_MVK_macos_surface");
///[VkMacOSSurfaceCreateFlagsMVK](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMacOSSurfaceCreateFlagsMVK.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_MVK_macos_surface
///typedef VkFlags VkMacOSSurfaceCreateFlagsMVK;
///```
///# Related
/// - [`VK_MVK_macos_surface`]
/// - [`MacOsSurfaceCreateInfoMVK`]
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
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`view`] is a reference to either a [`CaMetalLayer`] object or an `NSView` object.
///# Description
///## Valid Usage
/// - If [`view`] is a [`CaMetalLayer`] object, it  **must**  be a valid [`CaMetalLayer`]
/// - If [`view`] is an `NSView` object, it  **must**  be a valid `NSView`,  **must**  be backed by
///   a `CALayer` object of type [`CaMetalLayer`], and [`CreateMacOsSurfaceMVK`] **must**  be called
///   on the main thread
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK`
/// - [`p_next`] **must**  be `NULL`
/// - [`flags`] **must**  be `0`
///# Related
/// - [`VK_MVK_macos_surface`]
/// - [`MacOsSurfaceCreateFlagsMVK`]
/// - [`StructureType`]
/// - [`CreateMacOsSurfaceMVK`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkMacOSSurfaceCreateInfoMVK")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
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
            s_type: Default::default(),
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
    pub fn set_flags(&mut self, value: crate::extensions::mvk_macos_surface::MacOsSurfaceCreateFlagsMVK) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::view`]
    pub fn set_view(&mut self, value: &'lt std::ffi::c_void) -> &mut Self {
        self.view = value as *const _;
        self
    }
}
