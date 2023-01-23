//!# [VK_NVX_image_view_handle](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NVX_image_view_handle.html)
# ! [doc = include_str ! ("../../../../doc/extensions/nvx_image_view_handle/VK_NVX_image_view_handle.md")]
use crate::{
    cstr,
    vulkan1_0::{
        BaseInStructure, BaseOutStructure, DescriptorType, Device, DeviceAddress, DeviceSize, ImageView, Sampler,
        StructureType, VulkanResultCodes,
    },
};
use std::ffi::CStr;
///# [VkImageViewHandleInfoNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewHandleInfoNVX.html)
# [doc = include_str ! ("../../../../doc/extensions/nvx_image_view_handle/VkImageViewHandleInfoNVX.md")]
#[doc(alias = "VkImageViewHandleInfoNVX")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageViewHandleInfoNVX {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "imageView")]
    image_view: ImageView,
    #[doc(alias = "descriptorType")]
    descriptor_type: DescriptorType,
    sampler: Sampler,
}
///# [VkImageViewAddressPropertiesNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewAddressPropertiesNVX.html)
# [doc = include_str ! ("../../../../doc/extensions/nvx_image_view_handle/VkImageViewAddressPropertiesNVX.md")]
#[doc(alias = "VkImageViewAddressPropertiesNVX")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageViewAddressPropertiesNVX {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "deviceAddress")]
    device_address: DeviceAddress,
    size: DeviceSize,
}
#[doc(alias = "VK_NVX_IMAGE_VIEW_HANDLE_SPEC_VERSION")]
pub const NVX_IMAGE_VIEW_HANDLE_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_NVX_IMAGE_VIEW_HANDLE_EXTENSION_NAME")]
pub const NVX_IMAGE_VIEW_HANDLE_EXTENSION_NAME: &'static CStr = cstr!("VK_NVX_image_view_handle");
///# [vkGetImageViewHandleNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageViewHandleNVX.html)
# [doc = include_str ! ("../../../../doc/extensions/nvx_image_view_handle/vkGetImageViewHandleNVX.md")]
#[doc(alias = "vkGetImageViewHandleNVX")]
pub type FNGetImageViewHandleNvx =
    unsafe extern "system" fn(device: Device, p_info: *const ImageViewHandleInfoNVX) -> u32;
///# [vkGetImageViewAddressNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageViewAddressNVX.html)
# [doc = include_str ! ("../../../../doc/extensions/nvx_image_view_handle/vkGetImageViewAddressNVX.md")]
#[doc(alias = "vkGetImageViewAddressNVX")]
pub type FNGetImageViewAddressNvx = unsafe extern "system" fn(
    device: Device,
    image_view: ImageView,
    p_properties: *mut ImageViewAddressPropertiesNVX,
) -> VulkanResultCodes;
