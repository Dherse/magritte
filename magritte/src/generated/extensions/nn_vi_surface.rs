use crate::vulkan1_0::{BaseInStructure, StructureType};
use std::{
    ffi::{c_void, CStr},
    marker::PhantomData,
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NN_VI_SURFACE_SPEC_VERSION")]
pub const NN_VI_SURFACE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NN_VI_SURFACE_EXTENSION_NAME")]
pub const NN_VI_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NN_vi_surface");
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
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`window`] is the `nn`::`vi`::`NativeWindowHandle` for the `nn`::`vi`::`Layer` with which to
///   associate the surface.
///# Description
///Valid Usage
/// - [`window`]**must** be a valid `nn`::`vi`::`NativeWindowHandle`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VI_SURFACE_CREATE_INFO_NN`
/// - [`p_next`]**must** be `NULL`
/// - [`flags`]**must** be `0`
///# Related
/// - [`VK_NN_vi_surface`]
/// - [`StructureType`]
/// - [`ViSurfaceCreateFlagsNN`]
/// - [`CreateViSurfaceNN`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ViSurfaceCreateInfoNN<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: ViSurfaceCreateFlagsNN,
    ///[`window`] is the `nn`::`vi`::`NativeWindowHandle` for the
    ///`nn`::`vi`::`Layer` with which to associate the surface.
    window: *mut c_void,
}
impl<'lt> Default for ViSurfaceCreateInfoNN<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
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
    pub fn window_raw(&self) -> &*mut c_void {
        &self.window
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
    pub fn set_flags(&mut self, value: crate::extensions::nn_vi_surface::ViSurfaceCreateFlagsNN) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::window`]
    pub fn set_window(&mut self, value: &'lt mut std::ffi::c_void) -> &mut Self {
        self.window = value as *mut _;
        self
    }
}
