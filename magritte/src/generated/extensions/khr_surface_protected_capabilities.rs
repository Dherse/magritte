//![VK_KHR_surface_protected_capabilities](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_surface_protected_capabilities.html) - instance extension
//!# Description
//!This extension extends [`SurfaceCapabilities2KHR`], providing
//!applications a way to query whether swapchains  **can**  be created with the
//!`VK_SWAPCHAIN_CREATE_PROTECTED_BIT_KHR` flag set.Vulkan 1.1 added (optional) support for protect
//! memory and protected
//!resources including buffers (`VK_BUFFER_CREATE_PROTECTED_BIT`), images
//!(`VK_IMAGE_CREATE_PROTECTED_BIT`), and swapchains
//!(`VK_SWAPCHAIN_CREATE_PROTECTED_BIT_KHR`).
//!However, on implementations which support multiple windowing systems, not
//!all window systems  **may**  be able to provide a protected display path.This extension provides
//! a way to query if a protected swapchain created for
//!a surface (and thus a specific windowing system)  **can**  be displayed on
//!screen.
//!It extends the existing [`SurfaceCapabilities2KHR`] structure with a new
//![`SurfaceProtectedCapabilitiesKHR`] structure from which the application
//! **can**  obtain information about support for protected swapchain creation
//!through [`get_physical_device_surface_capabilities2_khr`].
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.1
//! - Requires `[`VK_KHR_get_surface_capabilities2`]`
//!# Contacts
//! - Sandeep Shinde [sashinde](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_surface_protected_capabilities]
//!   @sashinde%0A<<Here describe the issue or question you have about the
//!   VK_KHR_surface_protected_capabilities extension>>)
//!# New structures
//! - Extending [`SurfaceCapabilities2KHR`]:  - [`SurfaceProtectedCapabilitiesKHR`]
//!# New constants
//! - [`KHR_SURFACE_PROTECTED_CAPABILITIES_EXTENSION_NAME`]
//! - [`KHR_SURFACE_PROTECTED_CAPABILITIES_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_SURFACE_PROTECTED_CAPABILITIES_KHR`
//!# Version History
//! - Revision 1, 2018-12-18 (Sandeep Shinde, Daniel Koch)  - Internal revisions.
//!# Other info
//! * 2018-12-18
//! * No known IP claims.
//! * - Sandeep Shinde, NVIDIA  - James Jones, NVIDIA  - Daniel Koch, NVIDIA
//!# Related
//! - [`SurfaceProtectedCapabilitiesKHR`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{BaseInStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SURFACE_PROTECTED_CAPABILITIES_SPEC_VERSION")]
pub const KHR_SURFACE_PROTECTED_CAPABILITIES_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SURFACE_PROTECTED_CAPABILITIES_EXTENSION_NAME")]
pub const KHR_SURFACE_PROTECTED_CAPABILITIES_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_KHR_surface_protected_capabilities");
///[VkSurfaceProtectedCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceProtectedCapabilitiesKHR.html) - Structure describing capability of a surface to be protected
///# C Specifications
///An application queries if a protected [`SurfaceKHR`] is displayable on a
///specific windowing system using [`SurfaceProtectedCapabilitiesKHR`],
///which  **can**  be passed in [`p_next`] parameter of
///[`SurfaceCapabilities2KHR`].The [`SurfaceProtectedCapabilitiesKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_surface_protected_capabilities
///typedef struct VkSurfaceProtectedCapabilitiesKHR {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkBool32           supportsProtected;
///} VkSurfaceProtectedCapabilitiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`supports_protected`] specifies whether a protected swapchain created from
///   [`PhysicalDeviceSurfaceInfo2KHR::surface`] for a particular windowing system  **can**  be
///   displayed on screen or not. If [`supports_protected`] is [`TRUE`], then creation of swapchains
///   with the `VK_SWAPCHAIN_CREATE_PROTECTED_BIT_KHR` flag set  **must**  be supported for
///   `surface`.
///# Description
///If the `[`VK_GOOGLE_surfaceless_query`]` extension is enabled, the value
///returned in [`supports_protected`] will be identical for every valid
///surface created on this physical device, and so in the
///[`get_physical_device_surface_capabilities2_khr`] call,
///[`PhysicalDeviceSurfaceInfo2KHR::surface`] **can**  be
///[`crate::Handle::null`].
///In that case, the contents of
///[`SurfaceCapabilities2KHR::surface_capabilities`] as well as any
///other struct chained to it will be undefined.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SURFACE_PROTECTED_CAPABILITIES_KHR`
///# Related
/// - [`VK_KHR_surface_protected_capabilities`]
/// - [`Bool32`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSurfaceProtectedCapabilitiesKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct SurfaceProtectedCapabilitiesKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`supports_protected`] specifies whether a protected swapchain created
    ///from [`PhysicalDeviceSurfaceInfo2KHR`]::`surface` for a
    ///particular windowing system  **can**  be displayed on screen or not.
    ///If [`supports_protected`] is [`TRUE`], then creation of swapchains
    ///with the `VK_SWAPCHAIN_CREATE_PROTECTED_BIT_KHR` flag set  **must**  be
    ///supported for `surface`.
    pub supports_protected: Bool32,
}
impl<'lt> Default for SurfaceProtectedCapabilitiesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::SURFACE_PROTECTED_CAPABILITIES_KHR,
            p_next: std::ptr::null(),
            supports_protected: 0,
        }
    }
}
impl<'lt> SurfaceProtectedCapabilitiesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::supports_protected`]
    pub fn supports_protected_raw(&self) -> Bool32 {
        self.supports_protected
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::supports_protected`]
    pub fn set_supports_protected_raw(mut self, value: Bool32) -> Self {
        self.supports_protected = value;
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
    ///Gets the value of [`Self::supports_protected`]
    pub fn supports_protected(&self) -> bool {
        unsafe { std::mem::transmute(self.supports_protected as u8) }
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::supports_protected`]
    pub fn supports_protected_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.supports_protected as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.supports_protected as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
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
    ///Sets the value of [`Self::supports_protected`]
    pub fn set_supports_protected(mut self, value: bool) -> Self {
        self.supports_protected = value as u8 as u32;
        self
    }
}
