use crate::{cstr, vulkan1_2::ImageFormatListCreateInfo};
use std::ffi::CStr;
///See [`ImageFormatListCreateInfo`]
#[doc(alias = "VkImageFormatListCreateInfoKHR")]
pub type ImageFormatListCreateInfoKHR = ImageFormatListCreateInfo;
#[doc(alias = "VK_KHR_IMAGE_FORMAT_LIST_SPEC_VERSION")]
pub const KHR_IMAGE_FORMAT_LIST_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_IMAGE_FORMAT_LIST_EXTENSION_NAME")]
pub const KHR_IMAGE_FORMAT_LIST_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_image_format_list");
