//![VK_KHR_get_surface_capabilities2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_get_surface_capabilities2.html) - instance extension
//!# Description
//!This extension provides new entry points to query device surface
//!capabilities in a way that can be easily extended by other extensions,
//!without introducing any further entry points.
//!This extension can be considered the `[`VK_KHR_surface`]` equivalent of
//!the `[`VK_KHR_get_physical_device_properties2`]` extension.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_surface`]`
//!# Contacts
//! - James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_get_surface_capabilities2]
//!   @cubanismo%0A<<Here describe the issue or question you have about the
//!   VK_KHR_get_surface_capabilities2 extension>>)
//!# New functions & commands
//! - [`GetPhysicalDeviceSurfaceCapabilities2KHR`]
//! - [`GetPhysicalDeviceSurfaceFormats2KHR`]
//!# New structures
//! - [`PhysicalDeviceSurfaceInfo2KHR`]
//! - [`SurfaceCapabilities2KHR`]
//! - [`SurfaceFormat2KHR`]
//!# New constants
//! - [`KHR_GET_SURFACE_CAPABILITIES_2_EXTENSION_NAME`]
//! - [`KHR_GET_SURFACE_CAPABILITIES_2_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR`  -
//!   `VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_KHR`  - `VK_STRUCTURE_TYPE_SURFACE_FORMAT_2_KHR`
//!# Known issues & F.A.Q
//!1) What should this extension be named? **RESOLVED** : [`VK_KHR_get_surface_capabilities2`].
//!Other alternatives:
//! - `VK_KHR_surface2`
//! - One extension, combining a separate display-specific query extension.
//!2) Should additional WSI query functions be extended? **RESOLVED** :
//! - [`GetPhysicalDeviceSurfaceCapabilitiesKHR`]: Yes. The need for this motivated the extension.
//! - [`GetPhysicalDeviceSurfaceSupportKHR`]: No. Currently only has boolean output. Extensions
//!   should instead extend [`GetPhysicalDeviceSurfaceCapabilities2KHR`].
//! - [`GetPhysicalDeviceSurfaceFormatsKHR`]: Yes.
//! - [`GetPhysicalDeviceSurfacePresentModesKHR`]: No. Recent discussion concluded this introduced
//!   too much variability for applications to deal with. Extensions should instead extend
//!   [`GetPhysicalDeviceSurfaceCapabilities2KHR`].
//! - [`GetPhysicalDeviceXlibPresentationSupportKHR`]: Not in this extension.
//! - [`GetPhysicalDeviceXcbPresentationSupportKHR`]: Not in this extension.
//! - [`GetPhysicalDeviceWaylandPresentationSupportKHR`]: Not in this extension.
//! - [`GetPhysicalDeviceWin32PresentationSupportKHR`]: Not in this extension.
//!# Version History
//! - Revision 1, 2017-02-27 (James Jones)  - Initial draft.
//!# Other info
//! * 2017-02-27
//! * No known IP claims.
//! * - Ian Elliott, Google  - James Jones, NVIDIA  - Alon Or-bach, Samsung
//!# Related
//! - [`PhysicalDeviceSurfaceInfo2KHR`]
//! - [`SurfaceCapabilities2KHR`]
//! - [`SurfaceFormat2KHR`]
//! - [`GetPhysicalDeviceSurfaceCapabilities2KHR`]
//! - [`GetPhysicalDeviceSurfaceFormats2KHR`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    extensions::khr_surface::{SurfaceCapabilitiesKHR, SurfaceFormatKHR, SurfaceKHR},
    vulkan1_0::{BaseInStructure, BaseOutStructure, StructureType},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_GET_SURFACE_CAPABILITIES_2_SPEC_VERSION")]
pub const KHR_GET_SURFACE_CAPABILITIES_2_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_GET_SURFACE_CAPABILITIES_2_EXTENSION_NAME")]
pub const KHR_GET_SURFACE_CAPABILITIES_2_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_KHR_get_surface_capabilities2");
///[VkPhysicalDeviceSurfaceInfo2KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSurfaceInfo2KHR.html) - Structure specifying a surface and related swapchain creation parameters
///# C Specifications
///The [`PhysicalDeviceSurfaceInfo2KHR`] structure is defined as:
///```c
///// Provided by VK_KHR_get_surface_capabilities2
///typedef struct VkPhysicalDeviceSurfaceInfo2KHR {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkSurfaceKHR       surface;
///} VkPhysicalDeviceSurfaceInfo2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`surface`] is the surface that will be associated with the swapchain.
///# Description
///The members of [`PhysicalDeviceSurfaceInfo2KHR`] correspond to the
///arguments to [`GetPhysicalDeviceSurfaceCapabilitiesKHR`], with
///[`s_type`] and [`p_next`] added for extensibility.Additional capabilities of a surface  **may**
/// be available to swapchains created
///with different full-screen exclusive settings - particularly if exclusive
///full-screen access is application controlled.
///These additional capabilities  **can**  be queried by adding a
///[`SurfaceFullScreenExclusiveInfoEXT`] structure to the [`p_next`] chain
///of this structure when used to query surface properties.
///Additionally, for Win32 surfaces with application controlled exclusive
///full-screen access, chaining a
///[`SurfaceFullScreenExclusiveWin32InfoEXT`] structure  **may**  also report
///additional surface capabilities.
///These additional capabilities only apply to swapchains created with the same
///parameters included in the [`p_next`] chain of
///[`SwapchainCreateInfoKHR`].
///## Valid Usage
/// - If the [`p_next`] chain includes a [`SurfaceFullScreenExclusiveInfoEXT`] structure with its
///   `fullScreenExclusive` member set to `VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT`, and
///   [`surface`] was created using [`CreateWin32SurfaceKHR`], a
///   [`SurfaceFullScreenExclusiveWin32InfoEXT`] structure  **must**  be included in the [`p_next`]
///   chain
/// - When passed as the `pSurfaceInfo` parameter of [`GetPhysicalDeviceSurfaceCapabilities2KHR`],
///   if the `[`VK_GOOGLE_surfaceless_query`]` extension is enabled and the [`p_next`] chain of the
///   `pSurfaceCapabilities` parameter includes [`SurfaceProtectedCapabilitiesKHR`], then
///   [`surface`] **can**  be [`crate::utils::Handle::null`]. Otherwise, [`surface`] **must**  be a
///   valid [`SurfaceKHR`] handle
/// - When passed as the `pSurfaceInfo` parameter of [`GetPhysicalDeviceSurfaceFormats2KHR`], if the
///   `[`VK_GOOGLE_surfaceless_query`]` extension is enabled, then [`surface`] **can**  be
///   [`crate::utils::Handle::null`]. Otherwise, [`surface`] **must**  be a valid [`SurfaceKHR`]
///   handle
/// - When passed as the `pSurfaceInfo` parameter of [`GetPhysicalDeviceSurfacePresentModes2EXT`],
///   if the `[`VK_GOOGLE_surfaceless_query`]` extension is enabled, then [`surface`] **can**  be
///   [`crate::utils::Handle::null`]. Otherwise, [`surface`] **must**  be a valid [`SurfaceKHR`]
///   handle
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**
///   be either `NULL` or a pointer to a valid instance of [`SurfaceFullScreenExclusiveInfoEXT`] or
///   [`SurfaceFullScreenExclusiveWin32InfoEXT`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
/// - If [`surface`] is not [`crate::utils::Handle::null`], [`surface`] **must**  be a valid
///   [`SurfaceKHR`] handle
///# Related
/// - [`VK_KHR_get_surface_capabilities2`]
/// - [`StructureType`]
/// - [`SurfaceKHR`]
/// - [`GetDeviceGroupSurfacePresentModes2EXT`]
/// - [`GetPhysicalDeviceSurfaceCapabilities2KHR`]
/// - [`GetPhysicalDeviceSurfaceFormats2KHR`]
/// - [`GetPhysicalDeviceSurfacePresentModes2EXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceSurfaceInfo2KHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceSurfaceInfo2KHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`surface`] is the surface that will be associated with the swapchain.
    pub surface: SurfaceKHR,
}
impl<'lt> Default for PhysicalDeviceSurfaceInfo2KHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            surface: Default::default(),
        }
    }
}
impl<'lt> PhysicalDeviceSurfaceInfo2KHR<'lt> {
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
    ///Gets the value of [`Self::surface`]
    pub fn surface(&self) -> SurfaceKHR {
        self.surface
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::surface`]
    pub fn surface_mut(&mut self) -> &mut SurfaceKHR {
        &mut self.surface
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
    ///Sets the raw value of [`Self::surface`]
    pub fn set_surface(&mut self, value: crate::extensions::khr_surface::SurfaceKHR) -> &mut Self {
        self.surface = value;
        self
    }
}
///[VkSurfaceCapabilities2KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceCapabilities2KHR.html) - Structure describing capabilities of a surface
///# C Specifications
///The [`SurfaceCapabilities2KHR`] structure is defined as:
///```c
///// Provided by VK_KHR_get_surface_capabilities2
///typedef struct VkSurfaceCapabilities2KHR {
///    VkStructureType             sType;
///    void*                       pNext;
///    VkSurfaceCapabilitiesKHR    surfaceCapabilities;
///} VkSurfaceCapabilities2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`surface_capabilities`] is a [`SurfaceCapabilitiesKHR`] structure describing the capabilities
///   of the specified surface.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_KHR`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**
///   be either `NULL` or a pointer to a valid instance of
///   [`DisplayNativeHdrSurfaceCapabilitiesAMD`], [`SharedPresentSurfaceCapabilitiesKHR`],
///   [`SurfaceCapabilitiesFullScreenExclusiveEXT`], or [`SurfaceProtectedCapabilitiesKHR`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
///# Related
/// - [`VK_KHR_get_surface_capabilities2`]
/// - [`StructureType`]
/// - [`SurfaceCapabilitiesKHR`]
/// - [`GetPhysicalDeviceSurfaceCapabilities2KHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSurfaceCapabilities2KHR")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct SurfaceCapabilities2KHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`surface_capabilities`] is a [`SurfaceCapabilitiesKHR`] structure
    ///describing the capabilities of the specified surface.
    pub surface_capabilities: SurfaceCapabilitiesKHR,
}
impl<'lt> Default for SurfaceCapabilities2KHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            surface_capabilities: Default::default(),
        }
    }
}
impl<'lt> SurfaceCapabilities2KHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
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
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::surface_capabilities`]
    pub fn surface_capabilities(&self) -> SurfaceCapabilitiesKHR {
        self.surface_capabilities
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::surface_capabilities`]
    pub fn surface_capabilities_mut(&mut self) -> &mut SurfaceCapabilitiesKHR {
        &mut self.surface_capabilities
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::surface_capabilities`]
    pub fn set_surface_capabilities(
        &mut self,
        value: crate::extensions::khr_surface::SurfaceCapabilitiesKHR,
    ) -> &mut Self {
        self.surface_capabilities = value;
        self
    }
}
///[VkSurfaceFormat2KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceFormat2KHR.html) - Structure describing a supported swapchain format tuple
///# C Specifications
///The [`SurfaceFormat2KHR`] structure is defined as:
///```c
///// Provided by VK_KHR_get_surface_capabilities2
///typedef struct VkSurfaceFormat2KHR {
///    VkStructureType       sType;
///    void*                 pNext;
///    VkSurfaceFormatKHR    surfaceFormat;
///} VkSurfaceFormat2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`surface_format`] is a [`SurfaceFormatKHR`] structure describing a format-color space pair
///   that is compatible with the specified surface.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SURFACE_FORMAT_2_KHR`
/// - [`p_next`] **must**  be `NULL`
///# Related
/// - [`VK_KHR_get_surface_capabilities2`]
/// - [`StructureType`]
/// - [`SurfaceFormatKHR`]
/// - [`GetPhysicalDeviceSurfaceFormats2KHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSurfaceFormat2KHR")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct SurfaceFormat2KHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`surface_format`] is a [`SurfaceFormatKHR`] structure describing a
    ///format-color space pair that is compatible with the specified surface.
    pub surface_format: SurfaceFormatKHR,
}
impl<'lt> Default for SurfaceFormat2KHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            surface_format: Default::default(),
        }
    }
}
impl<'lt> SurfaceFormat2KHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
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
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::surface_format`]
    pub fn surface_format(&self) -> SurfaceFormatKHR {
        self.surface_format
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::surface_format`]
    pub fn surface_format_mut(&mut self) -> &mut SurfaceFormatKHR {
        &mut self.surface_format
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::surface_format`]
    pub fn set_surface_format(&mut self, value: crate::extensions::khr_surface::SurfaceFormatKHR) -> &mut Self {
        self.surface_format = value;
        self
    }
}
