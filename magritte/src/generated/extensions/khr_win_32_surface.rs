use crate::{
    native::{HINSTANCE, HWND},
    vulkan1_0::{BaseInStructure, StructureType},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_WIN32_SURFACE_SPEC_VERSION")]
pub const KHR_WIN32_SURFACE_SPEC_VERSION: u32 = 6;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_WIN32_SURFACE_EXTENSION_NAME")]
pub const KHR_WIN32_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_win32_surface");
///[VkWin32SurfaceCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkWin32SurfaceCreateInfoKHR.html) - Structure specifying parameters of a newly created Win32 surface object
///# C Specifications
///The [`Win32SurfaceCreateInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_win32_surface
///typedef struct VkWin32SurfaceCreateInfoKHR {
///    VkStructureType                 sType;
///    const void*                     pNext;
///    VkWin32SurfaceCreateFlagsKHR    flags;
///    HINSTANCE                       hinstance;
///    HWND                            hwnd;
///} VkWin32SurfaceCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`hinstance`] is the Win32 [`HINSTANCE`] for the window to associate the surface with.
/// - [`hwnd`] is the Win32 [`HWND`] for the window to associate the surface with.
///# Description
///Valid Usage
/// - [`hinstance`]**must** be a valid Win32 [`HINSTANCE`]
/// - [`hwnd`]**must** be a valid Win32 [`HWND`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`flags`]**must** be `0`
///# Related
/// - [`VK_KHR_win32_surface`]
/// - [`StructureType`]
/// - [`Win32SurfaceCreateFlagsKHR`]
/// - [`CreateWin32SurfaceKHR`]
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
pub struct Win32SurfaceCreateInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: Win32SurfaceCreateFlagsKHR,
    ///[`hinstance`] is the Win32 [`HINSTANCE`] for the window to associate
    ///the surface with.
    hinstance: HINSTANCE,
    ///[`hwnd`] is the Win32 [`HWND`] for the window to associate the
    ///surface with.
    hwnd: HWND,
}
impl<'lt> Default for Win32SurfaceCreateInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
            hinstance: Default::default(),
            hwnd: Default::default(),
        }
    }
}
impl<'lt> Win32SurfaceCreateInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::hinstance`]
    pub fn hinstance_raw(&self) -> &HINSTANCE {
        &self.hinstance
    }
    ///Gets the raw value of [`Self::hwnd`]
    pub fn hwnd_raw(&self) -> &HWND {
        &self.hwnd
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::hinstance`]
    pub fn set_hinstance_raw(&mut self, value: HINSTANCE) -> &mut Self {
        self.hinstance = value;
        self
    }
    ///Sets the raw value of [`Self::hwnd`]
    pub fn set_hwnd_raw(&mut self, value: HWND) -> &mut Self {
        self.hwnd = value;
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
    pub fn flags(&self) -> Win32SurfaceCreateFlagsKHR {
        self.flags
    }
    ///Gets the value of [`Self::hinstance`]
    pub fn hinstance(&self) -> &HINSTANCE {
        &self.hinstance
    }
    ///Gets the value of [`Self::hwnd`]
    pub fn hwnd(&self) -> &HWND {
        &self.hwnd
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut Win32SurfaceCreateFlagsKHR {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::hinstance`]
    pub fn hinstance_mut(&mut self) -> &mut HINSTANCE {
        &mut self.hinstance
    }
    ///Gets a mutable reference to the value of [`Self::hwnd`]
    pub fn hwnd_mut(&mut self) -> &mut HWND {
        &mut self.hwnd
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
    pub fn set_flags(&mut self, value: crate::extensions::khr_win_32_surface::Win32SurfaceCreateFlagsKHR) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::hinstance`]
    pub fn set_hinstance(&mut self, value: crate::native::HINSTANCE) -> &mut Self {
        self.hinstance = value;
        self
    }
    ///Sets the raw value of [`Self::hwnd`]
    pub fn set_hwnd(&mut self, value: crate::native::HWND) -> &mut Self {
        self.hwnd = value;
        self
    }
}
