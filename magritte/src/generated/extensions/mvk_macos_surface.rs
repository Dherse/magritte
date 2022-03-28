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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct MacOsSurfaceCreateInfoMVK<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: MacOsSurfaceCreateFlagsMVK,
    ///[`view`] is a reference to either a [`CaMetalLayer`] object or
    ///an `NSView` object.
    view: *const c_void,
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
