use crate::{
    native::{xcb_connection_t, xcb_window_t},
    vulkan1_0::{BaseInStructure, StructureType},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_XCB_SURFACE_SPEC_VERSION")]
pub const KHR_XCB_SURFACE_SPEC_VERSION: u32 = 6;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_XCB_SURFACE_EXTENSION_NAME")]
pub const KHR_XCB_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_xcb_surface");
///[VkXcbSurfaceCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkXcbSurfaceCreateInfoKHR.html) - Structure specifying parameters of a newly created Xcb surface object
///# C Specifications
///The [`XcbSurfaceCreateInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_xcb_surface
///typedef struct VkXcbSurfaceCreateInfoKHR {
///    VkStructureType               sType;
///    const void*                   pNext;
///    VkXcbSurfaceCreateFlagsKHR    flags;
///    xcb_connection_t*             connection;
///    xcb_window_t                  window;
///} VkXcbSurfaceCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`connection`] is a pointer to an [`xcb_connection_t`] to the X server.
/// - [`window`] is the [`xcb_window_t`] for the X11 window to associate the surface with.
///# Description
///Valid Usage
/// - [`connection`]**must** point to a valid X11 [`xcb_connection_t`]
/// - [`window`]**must** be a valid X11 [`xcb_window_t`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`flags`]**must** be `0`
///# Related
/// - [`VK_KHR_xcb_surface`]
/// - [`StructureType`]
/// - [`XcbSurfaceCreateFlagsKHR`]
/// - [`CreateXcbSurfaceKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct XcbSurfaceCreateInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: XcbSurfaceCreateFlagsKHR,
    ///[`connection`] is a pointer to an [`xcb_connection_t`] to the X
    ///server.
    connection: *mut xcb_connection_t,
    ///[`window`] is the [`xcb_window_t`] for the X11 window to associate
    ///the surface with.
    window: xcb_window_t,
}
impl<'lt> Default for XcbSurfaceCreateInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
            connection: std::ptr::null_mut(),
            window: Default::default(),
        }
    }
}
impl<'lt> XcbSurfaceCreateInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::window`]
    pub fn window_raw(&self) -> &xcb_window_t {
        &self.window
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::window`]
    pub fn set_window_raw(&mut self, value: xcb_window_t) -> &mut Self {
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
    pub fn flags(&self) -> XcbSurfaceCreateFlagsKHR {
        self.flags
    }
    ///Gets the value of [`Self::connection`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn connection(&self) -> &xcb_connection_t {
        &*self.connection
    }
    ///Gets the value of [`Self::window`]
    pub fn window(&self) -> &xcb_window_t {
        &self.window
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut XcbSurfaceCreateFlagsKHR {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::connection`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn connection_mut(&mut self) -> &mut xcb_connection_t {
        &mut *self.connection
    }
    ///Gets a mutable reference to the value of [`Self::window`]
    pub fn window_mut(&mut self) -> &mut xcb_window_t {
        &mut self.window
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
    pub fn set_flags(&mut self, value: crate::extensions::khr_xcb_surface::XcbSurfaceCreateFlagsKHR) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::connection`]
    pub fn set_connection(&mut self, value: &'lt mut crate::native::xcb_connection_t) -> &mut Self {
        self.connection = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::window`]
    pub fn set_window(&mut self, value: crate::native::xcb_window_t) -> &mut Self {
        self.window = value;
        self
    }
}
