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
///   the ways the application  **can**  use the shared presentable image from a swapchain created
///   with [`PresentModeKHR`] set to `VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR` or
///   `VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR` for the surface on the specified device.
///   `VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT` **must**  be included in the set but implementations
///   **may**  support additional usages.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SHARED_PRESENT_SURFACE_CAPABILITIES_KHR`
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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct SharedPresentSurfaceCapabilitiesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`shared_present_supported_usage_flags`] is a bitmask of
    ///[`ImageUsageFlagBits`] representing the ways the application  **can**
    ///use the shared presentable image from a swapchain created with
    ///[`PresentModeKHR`] set to
    ///`VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR` or
    ///`VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR` for the surface on
    ///the specified device.
    ///`VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT` **must**  be included in the set
    ///but implementations  **may**  support additional usages.
    shared_present_supported_usage_flags: ImageUsageFlags,
}
impl<'lt> Default for SharedPresentSurfaceCapabilitiesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            shared_present_supported_usage_flags: Default::default(),
        }
    }
}
impl<'lt> SharedPresentSurfaceCapabilitiesKHR<'lt> {
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
    ///Gets the value of [`Self::shared_present_supported_usage_flags`]
    pub fn shared_present_supported_usage_flags(&self) -> ImageUsageFlags {
        self.shared_present_supported_usage_flags
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
    ///Gets a mutable reference to the value of [`Self::shared_present_supported_usage_flags`]
    pub fn shared_present_supported_usage_flags_mut(&mut self) -> &mut ImageUsageFlags {
        &mut self.shared_present_supported_usage_flags
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
    ///Sets the raw value of [`Self::shared_present_supported_usage_flags`]
    pub fn set_shared_present_supported_usage_flags(&mut self, value: crate::vulkan1_0::ImageUsageFlags) -> &mut Self {
        self.shared_present_supported_usage_flags = value;
        self
    }
}
