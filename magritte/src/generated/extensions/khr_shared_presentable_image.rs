use crate::vulkan1_0::{BaseOutStructure, ImageUsageFlags, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SHARED_PRESENTABLE_IMAGE_SPEC_VERSION")]
pub const KHR_SHARED_PRESENTABLE_IMAGE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SHARED_PRESENTABLE_IMAGE_EXTENSION_NAME")]
pub const KHR_SHARED_PRESENTABLE_IMAGE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_shared_presentable_image");
///[VkSharedPresentSurfaceCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSharedPresentSurfaceCapabilitiesKHR.html) - Structure describing capabilities of a surface for shared presentation
///# C Specifications
///The [`SharedPresentSurfaceCapabilitiesKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_shared_presentable_image
///typedef struct VkSharedPresentSurfaceCapabilitiesKHR {
///    VkStructureType      sType;
///    void*                pNext;
///    VkImageUsageFlags    sharedPresentSupportedUsageFlags;
///} VkSharedPresentSurfaceCapabilitiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`shared_present_supported_usage_flags`] is a bitmask of [`ImageUsageFlagBits`] representing
///   the ways the application **can** use the shared presentable image from a swapchain created
///   with [`PresentModeKHR`] set to `VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR` or
///   `VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR` for the surface on the specified device.
///   `VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT`**must** be included in the set but implementations
///   **may** support additional usages.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SHARED_PRESENT_SURFACE_CAPABILITIES_KHR`
///# Related
/// - [`VK_KHR_shared_presentable_image`]
/// - [`ImageUsageFlags`]
/// - [`StructureType`]
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
pub struct SharedPresentSurfaceCapabilitiesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`shared_present_supported_usage_flags`] is a bitmask of
    ///[`ImageUsageFlagBits`] representing the ways the application **can**
    ///use the shared presentable image from a swapchain created with
    ///[`PresentModeKHR`] set to
    ///`VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR` or
    ///`VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR` for the surface on
    ///the specified device.
    ///`VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT`**must** be included in the set
    ///but implementations **may** support additional usages.
    shared_present_supported_usage_flags: ImageUsageFlags,
}
