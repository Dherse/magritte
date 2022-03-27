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
///[`s_type`] and [`p_next`] added for extensibility.Additional capabilities of a surface **may**
/// be available to swapchains created
///with different full-screen exclusive settings - particularly if exclusive
///full-screen access is application controlled.
///These additional capabilities **can** be queried by adding a
///[`SurfaceFullScreenExclusiveInfoEXT`] structure to the [`p_next`] chain
///of this structure when used to query surface properties.
///Additionally, for Win32 surfaces with application controlled exclusive
///full-screen access, chaining a
///[`SurfaceFullScreenExclusiveWin32InfoEXT`] structure **may** also report
///additional surface capabilities.
///These additional capabilities only apply to swapchains created with the same
///parameters included in the [`p_next`] chain of
///[`SwapchainCreateInfoKHR`].Valid Usage
/// - If the [`p_next`] chain includes a [`SurfaceFullScreenExclusiveInfoEXT`] structure with its
///   `fullScreenExclusive` member set to `VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT`, and
///   [`surface`] was created using [`CreateWin32SurfaceKHR`], a
///   [`SurfaceFullScreenExclusiveWin32InfoEXT`] structure **must** be included in the [`p_next`]
///   chain
/// - When passed as the `pSurfaceInfo` parameter of [`GetPhysicalDeviceSurfaceCapabilities2KHR`],
///   if the `[`VK_GOOGLE_surfaceless_query`]` extension is enabled and the [`p_next`] chain of the
///   `pSurfaceCapabilities` parameter includes [`SurfaceProtectedCapabilitiesKHR`], then
///   [`surface`]**can** be [`crate::utils::Handle::null`]. Otherwise, [`surface`]**must** be a
///   valid [`SurfaceKHR`] handle
/// - When passed as the `pSurfaceInfo` parameter of [`GetPhysicalDeviceSurfaceFormats2KHR`], if the
///   `[`VK_GOOGLE_surfaceless_query`]` extension is enabled, then [`surface`]**can** be
///   [`crate::utils::Handle::null`]. Otherwise, [`surface`]**must** be a valid [`SurfaceKHR`]
///   handle
/// - When passed as the `pSurfaceInfo` parameter of [`GetPhysicalDeviceSurfacePresentModes2EXT`],
///   if the `[`VK_GOOGLE_surfaceless_query`]` extension is enabled, then [`surface`]**can** be
///   [`crate::utils::Handle::null`]. Otherwise, [`surface`]**must** be a valid [`SurfaceKHR`]
///   handle
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain **must**
///   be either `NULL` or a pointer to a valid instance of [`SurfaceFullScreenExclusiveInfoEXT`] or
///   [`SurfaceFullScreenExclusiveWin32InfoEXT`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
/// - If [`surface`] is not [`crate::utils::Handle::null`], [`surface`]**must** be a valid
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceSurfaceInfo2KHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`surface`] is the surface that will be associated with the swapchain.
    surface: SurfaceKHR,
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_KHR`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain **must**
///   be either `NULL` or a pointer to a valid instance of
///   [`DisplayNativeHdrSurfaceCapabilitiesAMD`], [`SharedPresentSurfaceCapabilitiesKHR`],
///   [`SurfaceCapabilitiesFullScreenExclusiveEXT`], or [`SurfaceProtectedCapabilitiesKHR`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct SurfaceCapabilities2KHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`surface_capabilities`] is a [`SurfaceCapabilitiesKHR`] structure
    ///describing the capabilities of the specified surface.
    surface_capabilities: SurfaceCapabilitiesKHR,
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SURFACE_FORMAT_2_KHR`
/// - [`p_next`]**must** be `NULL`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct SurfaceFormat2KHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`surface_format`] is a [`SurfaceFormatKHR`] structure describing a
    ///format-color space pair that is compatible with the specified surface.
    surface_format: SurfaceFormatKHR,
}
