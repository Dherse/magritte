//![VK_EXT_directfb_surface](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_directfb_surface.html) - instance extension
//!# Description
//!The [`VK_EXT_directfb_surface`] extension is an instance extension.
//!It provides a mechanism to create a [`SurfaceKHR`] object (defined by
//!the `[`VK_KHR_surface`]` extension) that refers to a DirectFB
//![`IDirectFBSurface`], as well as a query to determine support for rendering
//!via DirectFB.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_surface`]`
//!# Contacts
//! - Nicolas Caramelli [caramelli](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_directfb_surface]
//!   @caramelli%0A<<Here describe the issue or question you have about the VK_EXT_directfb_surface
//!   extension>>)
//!# New functions & commands
//! - [`CreateDirectFBSurfaceEXT`]
//! - [`GetPhysicalDeviceDirectFBPresentationSupportEXT`]
//!# New structures
//! - [`DirectFBSurfaceCreateInfoEXT`]
//!# New bitmasks
//! - [`DirectFBSurfaceCreateFlagsEXT`]
//!# New constants
//! - [`EXT_DIRECTFB_SURFACE_EXTENSION_NAME`]
//! - [`EXT_DIRECTFB_SURFACE_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_DIRECTFB_SURFACE_CREATE_INFO_EXT`
//!# Version History
//! - Revision 1, 2020-06-16 (Nicolas Caramelli)  - Initial version
//!# Other info
//! * 2020-06-16
//! * No known IP claims.
//! * - Nicolas Caramelli
//!# Related
//! - [`DirectFBSurfaceCreateFlagsEXT`]
//! - [`DirectFBSurfaceCreateInfoEXT`]
//! - [`CreateDirectFBSurfaceEXT`]
//! - [`GetPhysicalDeviceDirectFBPresentationSupportEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    native::{IDirectFB, IDirectFBSurface},
    vulkan1_0::{BaseInStructure, StructureType},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DIRECTFB_SURFACE_SPEC_VERSION")]
pub const EXT_DIRECTFB_SURFACE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DIRECTFB_SURFACE_EXTENSION_NAME")]
pub const EXT_DIRECTFB_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_directfb_surface");
///[VkDirectFBSurfaceCreateFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDirectFBSurfaceCreateFlagsEXT.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_EXT_directfb_surface
///typedef VkFlags VkDirectFBSurfaceCreateFlagsEXT;
///```
///# Related
/// - [`VK_EXT_directfb_surface`]
/// - [`DirectFBSurfaceCreateInfoEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct DirectFBSurfaceCreateFlagsEXT(u32);
impl const Default for DirectFBSurfaceCreateFlagsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for DirectFBSurfaceCreateFlagsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(DirectFBSurfaceCreateFlagsEXT))
            .field(&self.0)
            .finish()
    }
}
///[VkDirectFBSurfaceCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDirectFBSurfaceCreateInfoEXT.html) - Structure specifying parameters of a newly created DirectFB surface object
///# C Specifications
///The [`DirectFBSurfaceCreateInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_directfb_surface
///typedef struct VkDirectFBSurfaceCreateInfoEXT {
///    VkStructureType                    sType;
///    const void*                        pNext;
///    VkDirectFBSurfaceCreateFlagsEXT    flags;
///    IDirectFB*                         dfb;
///    IDirectFBSurface*                  surface;
///} VkDirectFBSurfaceCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`dfb`] is a pointer to the [`IDirectFB`] main interface of DirectFB.
/// - [`surface`] is a pointer to a [`IDirectFBSurface`] surface interface.
///# Description
///## Valid Usage
/// - [`dfb`] **must**  point to a valid DirectFB [`IDirectFB`]
/// - [`surface`] **must**  point to a valid DirectFB [`IDirectFBSurface`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DIRECTFB_SURFACE_CREATE_INFO_EXT`
/// - [`p_next`] **must**  be `NULL`
/// - [`flags`] **must**  be `0`
///# Related
/// - [`VK_EXT_directfb_surface`]
/// - [`DirectFBSurfaceCreateFlagsEXT`]
/// - [`StructureType`]
/// - [`CreateDirectFBSurfaceEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDirectFBSurfaceCreateInfoEXT")]
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DirectFBSurfaceCreateInfoEXT<'lt> {
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    pub flags: DirectFBSurfaceCreateFlagsEXT,
    ///[`dfb`] is a pointer to the [`IDirectFB`] main interface of DirectFB.
    pub dfb: *mut IDirectFB,
    ///[`surface`] is a pointer to a [`IDirectFBSurface`] surface interface.
    pub surface: *mut IDirectFBSurface,
}
impl<'lt> Default for DirectFBSurfaceCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
            dfb: std::ptr::null_mut(),
            surface: std::ptr::null_mut(),
        }
    }
}
impl<'lt> DirectFBSurfaceCreateInfoEXT<'lt> {
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
    pub fn flags(&self) -> DirectFBSurfaceCreateFlagsEXT {
        self.flags
    }
    ///Gets the value of [`Self::dfb`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn dfb(&self) -> &IDirectFB {
        &*self.dfb
    }
    ///Gets the value of [`Self::surface`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn surface(&self) -> &IDirectFBSurface {
        &*self.surface
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut DirectFBSurfaceCreateFlagsEXT {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::dfb`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn dfb_mut(&mut self) -> &mut IDirectFB {
        &mut *self.dfb
    }
    ///Gets a mutable reference to the value of [`Self::surface`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn surface_mut(&mut self) -> &mut IDirectFBSurface {
        &mut *self.surface
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
        value: crate::extensions::ext_directfb_surface::DirectFBSurfaceCreateFlagsEXT,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::dfb`]
    pub fn set_dfb(&mut self, value: &'lt mut crate::native::IDirectFB) -> &mut Self {
        self.dfb = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::surface`]
    pub fn set_surface(&mut self, value: &'lt mut crate::native::IDirectFBSurface) -> &mut Self {
        self.surface = value as *mut _;
        self
    }
}
