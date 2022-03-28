use crate::{
    native::{_screen_context, _screen_window},
    vulkan1_0::{BaseInStructure, StructureType},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_QNX_SCREEN_SURFACE_SPEC_VERSION")]
pub const QNX_SCREEN_SURFACE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_QNX_SCREEN_SURFACE_EXTENSION_NAME")]
pub const QNX_SCREEN_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_QNX_screen_surface");
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
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`context`] and [`window`] are QNX Screen [`context`] and [`window`] to associate the surface
///   with.
///# Description
///## Valid Usage
/// - [`context`] **must**  point to a valid QNX Screen `struct` _screen_context
/// - [`window`] **must**  point to a valid QNX Screen `struct` _screen_window
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SCREEN_SURFACE_CREATE_INFO_QNX`
/// - [`p_next`] **must**  be `NULL`
/// - [`flags`] **must**  be `0`
///# Related
/// - [`VK_QNX_screen_surface`]
/// - [`ScreenSurfaceCreateFlagsQNX`]
/// - [`StructureType`]
/// - [`CreateScreenSurfaceQNX`]
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
pub struct ScreenSurfaceCreateInfoQNX<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: ScreenSurfaceCreateFlagsQNX,
    ///[`context`] and [`window`] are QNX Screen [`context`] and
    ///[`window`] to associate the surface with.
    context: *mut _screen_context,
    ///No documentation found
    window: *mut _screen_window,
}
impl<'lt> Default for ScreenSurfaceCreateInfoQNX<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
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
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn context(&self) -> &_screen_context {
        &*self.context
    }
    ///Gets the value of [`Self::window`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn window(&self) -> &_screen_window {
        &*self.window
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
    pub fn set_flags(
        &mut self,
        value: crate::extensions::qnx_screen_surface::ScreenSurfaceCreateFlagsQNX,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::context`]
    pub fn set_context(&mut self, value: &'lt mut crate::native::_screen_context) -> &mut Self {
        self.context = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::window`]
    pub fn set_window(&mut self, value: &'lt mut crate::native::_screen_window) -> &mut Self {
        self.window = value as *mut _;
        self
    }
}
