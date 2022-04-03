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
//! - [`get_physical_device_surface_capabilities2_khr`]
//! - [`get_physical_device_surface_formats2_khr`]
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
//! - [`get_physical_device_surface_capabilities_khr`]: Yes. The need for this motivated the
//!   extension.
//! - [`get_physical_device_surface_support_khr`]: No. Currently only has boolean output. Extensions
//!   should instead extend [`get_physical_device_surface_capabilities2_khr`].
//! - [`get_physical_device_surface_formats_khr`]: Yes.
//! - [`get_physical_device_surface_present_modes_khr`]: No. Recent discussion concluded this
//!   introduced too much variability for applications to deal with. Extensions should instead
//!   extend [`get_physical_device_surface_capabilities2_khr`].
//! - [`get_physical_device_xlib_presentation_support_khr`]: Not in this extension.
//! - [`get_physical_device_xcb_presentation_support_khr`]: Not in this extension.
//! - [`get_physical_device_wayland_presentation_support_khr`]: Not in this extension.
//! - [`get_physical_device_win32_presentation_support_khr`]: Not in this extension.
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
//! - [`get_physical_device_surface_capabilities2_khr`]
//! - [`get_physical_device_surface_formats2_khr`]
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
    vulkan1_0::{BaseInStructure, BaseOutStructure, Instance, PhysicalDevice, StructureType, VulkanResultCodes},
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
///[vkGetPhysicalDeviceSurfaceCapabilities2KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilities2KHR.html) - Reports capabilities of a surface on a physical device
///# C Specifications
///To query the basic capabilities of a surface defined by the core or
///extensions, call:
///```c
///// Provided by VK_KHR_get_surface_capabilities2
///VkResult vkGetPhysicalDeviceSurfaceCapabilities2KHR(
///    VkPhysicalDevice                            physicalDevice,
///    const VkPhysicalDeviceSurfaceInfo2KHR*      pSurfaceInfo,
///    VkSurfaceCapabilities2KHR*                  pSurfaceCapabilities);
///```
///# Parameters
/// - [`physical_device`] is the physical device that will be associated with the swapchain to be
///   created, as described for [`create_swapchain_khr`].
/// - [`p_surface_info`] is a pointer to a [`PhysicalDeviceSurfaceInfo2KHR`] structure describing
///   the surface and other fixed parameters that would be consumed by [`create_swapchain_khr`].
/// - [`p_surface_capabilities`] is a pointer to a [`SurfaceCapabilities2KHR`] structure in which
///   the capabilities are returned.
///# Description
///[`get_physical_device_surface_capabilities2_khr`] behaves similarly to
///[`get_physical_device_surface_capabilities_khr`], with the ability to specify
///extended inputs via chained input structures, and to return extended
///information via chained output structures.
///## Valid Usage
/// - `pSurfaceInfo->surface` **must**  be a valid [`SurfaceKHR`] handle
/// - `pSurfaceInfo->surface` **must**  be supported by [`physical_device`], as reported by
///   [`get_physical_device_surface_support_khr`] or an equivalent platform-specific mechanism
///
/// - If a [`SurfaceCapabilitiesFullScreenExclusiveEXT`] structure is included in the `pNext` chain
///   of [`p_surface_capabilities`], a [`SurfaceFullScreenExclusiveWin32InfoEXT`] structure
///   **must**  be included in the `pNext` chain of [`p_surface_info`]
///
///## Valid Usage (Implicit)
/// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
/// - [`p_surface_info`] **must**  be a valid pointer to a valid [`PhysicalDeviceSurfaceInfo2KHR`]
///   structure
/// - [`p_surface_capabilities`] **must**  be a valid pointer to a [`SurfaceCapabilities2KHR`]
///   structure
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
///   `VK_ERROR_SURFACE_LOST_KHR`
///# Related
/// - [`VK_KHR_get_surface_capabilities2`]
/// - [`PhysicalDevice`]
/// - [`PhysicalDeviceSurfaceInfo2KHR`]
/// - [`SurfaceCapabilities2KHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilities2KHR")]
pub type FNGetPhysicalDeviceSurfaceCapabilities2Khr = Option<
    for<'lt> unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'lt>,
        p_surface_capabilities: *mut SurfaceCapabilities2KHR<'lt>,
    ) -> VulkanResultCodes,
>;
///[vkGetPhysicalDeviceSurfaceFormats2KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceFormats2KHR.html) - Query color formats supported by surface
///# C Specifications
///To query the supported swapchain format tuples for a surface, call:
///```c
///// Provided by VK_KHR_get_surface_capabilities2
///VkResult vkGetPhysicalDeviceSurfaceFormats2KHR(
///    VkPhysicalDevice                            physicalDevice,
///    const VkPhysicalDeviceSurfaceInfo2KHR*      pSurfaceInfo,
///    uint32_t*                                   pSurfaceFormatCount,
///    VkSurfaceFormat2KHR*                        pSurfaceFormats);
///```
///# Parameters
/// - [`physical_device`] is the physical device that will be associated with the swapchain to be
///   created, as described for [`create_swapchain_khr`].
/// - [`p_surface_info`] is a pointer to a [`PhysicalDeviceSurfaceInfo2KHR`] structure describing
///   the surface and other fixed parameters that would be consumed by [`create_swapchain_khr`].
/// - [`p_surface_format_count`] is a pointer to an integer related to the number of format tuples
///   available or queried, as described below.
/// - [`p_surface_formats`] is either `NULL` or a pointer to an array of [`SurfaceFormat2KHR`]
///   structures.
///# Description
///[`get_physical_device_surface_formats2_khr`] behaves similarly to
///[`get_physical_device_surface_formats_khr`], with the ability to be extended
///via `pNext` chains.If [`p_surface_formats`] is `NULL`, then the number of format tuples
///supported for the given `surface` is returned in
///[`p_surface_format_count`].
///Otherwise, [`p_surface_format_count`] **must**  point to a variable set by the
///user to the number of elements in the [`p_surface_formats`] array, and on
///return the variable is overwritten with the number of structures actually
///written to [`p_surface_formats`].
///If the value of [`p_surface_format_count`] is less than the number of format
///tuples supported, at most [`p_surface_format_count`] structures will be
///written, and `VK_INCOMPLETE` will be returned instead of
///`VK_SUCCESS`, to indicate that not all the available values were
///returned.
///## Valid Usage
/// - If the `[`VK_GOOGLE_surfaceless_query`]` extension is not enabled, `pSurfaceInfo->surface`
///   **must**  be a valid [`SurfaceKHR`] handle
/// - If `pSurfaceInfo->surface` is not [`crate::utils::Handle::null`], it  **must**  be supported
///   by [`physical_device`], as reported by [`get_physical_device_surface_support_khr`] or an
///   equivalent platform-specific mechanism
///
///## Valid Usage (Implicit)
/// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
/// - [`p_surface_info`] **must**  be a valid pointer to a valid [`PhysicalDeviceSurfaceInfo2KHR`]
///   structure
/// - [`p_surface_format_count`] **must**  be a valid pointer to a `uint32_t` value
/// - If the value referenced by [`p_surface_format_count`] is not `0`, and [`p_surface_formats`] is
///   not `NULL`, [`p_surface_formats`] **must**  be a valid pointer to an array of
///   [`p_surface_format_count`][`SurfaceFormat2KHR`] structures
///
///## Return Codes
/// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
///   `VK_ERROR_SURFACE_LOST_KHR`
///# Related
/// - [`VK_KHR_get_surface_capabilities2`]
/// - [`PhysicalDevice`]
/// - [`PhysicalDeviceSurfaceInfo2KHR`]
/// - [`SurfaceFormat2KHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetPhysicalDeviceSurfaceFormats2KHR")]
pub type FNGetPhysicalDeviceSurfaceFormats2Khr = Option<
    for<'lt> unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'lt>,
        p_surface_format_count: *mut u32,
        p_surface_formats: *mut SurfaceFormat2KHR<'lt>,
    ) -> VulkanResultCodes,
>;
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
///arguments to [`get_physical_device_surface_capabilities_khr`], with
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
///   [`surface`] was created using [`create_win32_surface_khr`], a
///   [`SurfaceFullScreenExclusiveWin32InfoEXT`] structure  **must**  be included in the [`p_next`]
///   chain
/// - When passed as the `pSurfaceInfo` parameter of
///   [`get_physical_device_surface_capabilities2_khr`], if the `[`VK_GOOGLE_surfaceless_query`]`
///   extension is enabled and the [`p_next`] chain of the `pSurfaceCapabilities` parameter includes
///   [`SurfaceProtectedCapabilitiesKHR`], then [`surface`] **can**  be
///   [`crate::utils::Handle::null`]. Otherwise, [`surface`] **must**  be a valid [`SurfaceKHR`]
///   handle
/// - When passed as the `pSurfaceInfo` parameter of [`get_physical_device_surface_formats2_khr`],
///   if the `[`VK_GOOGLE_surfaceless_query`]` extension is enabled, then [`surface`] **can**  be
///   [`crate::utils::Handle::null`]. Otherwise, [`surface`] **must**  be a valid [`SurfaceKHR`]
///   handle
/// - When passed as the `pSurfaceInfo` parameter of
///   [`get_physical_device_surface_present_modes2_ext`], if the `[`VK_GOOGLE_surfaceless_query`]`
///   extension is enabled, then [`surface`] **can**  be [`crate::utils::Handle::null`]. Otherwise,
///   [`surface`] **must**  be a valid [`SurfaceKHR`] handle
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
/// - [`get_device_group_surface_present_modes2_ext`]
/// - [`get_physical_device_surface_capabilities2_khr`]
/// - [`get_physical_device_surface_formats2_khr`]
/// - [`get_physical_device_surface_present_modes2_ext`]
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
            s_type: StructureType::PhysicalDeviceSurfaceInfo2Khr,
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
/// - [`get_physical_device_surface_capabilities2_khr`]
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
            s_type: StructureType::SurfaceCapabilities2Khr,
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
/// - [`get_physical_device_surface_formats2_khr`]
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
            s_type: StructureType::SurfaceFormat2Khr,
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
///The V-table of [`Instance`] for functions from VK_KHR_get_surface_capabilities2
pub struct InstanceKhrGetSurfaceCapabilities2VTable {
    ///See [`FNGetPhysicalDeviceSurfaceCapabilities2Khr`] for more information.
    pub get_physical_device_surface_capabilities2_khr: FNGetPhysicalDeviceSurfaceCapabilities2Khr,
    ///See [`FNGetPhysicalDeviceSurfaceFormats2Khr`] for more information.
    pub get_physical_device_surface_formats2_khr: FNGetPhysicalDeviceSurfaceFormats2Khr,
}
impl InstanceKhrGetSurfaceCapabilities2VTable {
    ///Loads the VTable from the owner and the names
    pub fn load<F>(loader_fn: F, loader: Instance) -> Self
    where
        F: Fn(Instance, &'static CStr) -> Option<extern "system" fn()>,
    {
        Self {
            get_physical_device_surface_capabilities2_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetPhysicalDeviceSurfaceCapabilities2KHR"),
                ))
            },
            get_physical_device_surface_formats2_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkGetPhysicalDeviceSurfaceFormats2KHR")))
            },
        }
    }
    ///Gets [`Self::get_physical_device_surface_capabilities2_khr`]. See
    /// [`FNGetPhysicalDeviceSurfaceCapabilities2Khr`] for more information.
    pub fn get_physical_device_surface_capabilities2_khr(&self) -> FNGetPhysicalDeviceSurfaceCapabilities2Khr {
        self.get_physical_device_surface_capabilities2_khr
    }
    ///Gets [`Self::get_physical_device_surface_formats2_khr`]. See
    /// [`FNGetPhysicalDeviceSurfaceFormats2Khr`] for more information.
    pub fn get_physical_device_surface_formats2_khr(&self) -> FNGetPhysicalDeviceSurfaceFormats2Khr {
        self.get_physical_device_surface_formats2_khr
    }
}
