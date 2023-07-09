use crate::native::vulkan1_0::{
    BaseInStructure, BaseOutStructure, DescriptorType, Device, DeviceAddress, DeviceSize, ImageView, Sampler,
    StructureType, VulkanResultCodes,
};
#[doc(alias = "VkImageViewHandleInfoNVX")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageViewHandleInfoNVX {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "imageView")]
    pub image_view: ImageView,
    #[doc(alias = "descriptorType")]
    pub descriptor_type: DescriptorType,
    pub sampler: Sampler,
}
impl Default for ImageViewHandleInfoNVX {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImageViewHandleInfoNvx,
            p_next: unsafe { std::mem::zeroed() },
            image_view: unsafe { std::mem::zeroed() },
            descriptor_type: unsafe { std::mem::zeroed() },
            sampler: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkImageViewAddressPropertiesNVX")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageViewAddressPropertiesNVX {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "deviceAddress")]
    pub device_address: DeviceAddress,
    pub size: DeviceSize,
}
impl Default for ImageViewAddressPropertiesNVX {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImageViewAddressPropertiesNvx,
            p_next: unsafe { std::mem::zeroed() },
            device_address: unsafe { std::mem::zeroed() },
            size: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::nvx_image_view_handle::{
    NVX_IMAGE_VIEW_HANDLE_EXTENSION_NAME, NVX_IMAGE_VIEW_HANDLE_SPEC_VERSION,
};
#[doc(alias = "vkGetImageViewHandleNVX")]
pub type FNGetImageViewHandleNvx =
    unsafe extern "system" fn(device: Device, p_info: *const ImageViewHandleInfoNVX) -> u32;
#[doc(alias = "vkGetImageViewAddressNVX")]
pub type FNGetImageViewAddressNvx = unsafe extern "system" fn(
    device: Device,
    image_view: ImageView,
    p_properties: *mut ImageViewAddressPropertiesNVX,
) -> VulkanResultCodes;
