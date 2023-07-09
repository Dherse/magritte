pub use crate::common::extensions::nv_external_memory_capabilities::{
    ExternalImageFormatPropertiesNV, ExternalMemoryFeatureFlagBitsNV, ExternalMemoryFeatureFlagsNV,
    ExternalMemoryHandleTypeFlagBitsNV, ExternalMemoryHandleTypeFlagsNV,
    NV_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME, NV_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION,
};
use crate::native::vulkan1_0::{
    Format, ImageCreateFlags, ImageTiling, ImageType, ImageUsageFlags, PhysicalDevice, VulkanResultCodes,
};
#[doc(alias = "vkGetPhysicalDeviceExternalImageFormatPropertiesNV")]
pub type FNGetPhysicalDeviceExternalImageFormatPropertiesNv = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    format: Format,
    type_: ImageType,
    tiling: ImageTiling,
    usage: ImageUsageFlags,
    flags: ImageCreateFlags,
    external_handle_type: ExternalMemoryHandleTypeFlagsNV,
    p_external_image_format_properties: *mut ExternalImageFormatPropertiesNV,
) -> VulkanResultCodes;
