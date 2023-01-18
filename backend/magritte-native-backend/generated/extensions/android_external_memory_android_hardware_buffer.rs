use crate::{
    cstr,
    vulkan1_0::{
        BaseInStructure, BaseOutStructure, ComponentMapping, Device, DeviceMemory, DeviceSize, Format,
        FormatFeatureFlags, StructureType, VulkanResultCodes,
    },
    vulkan1_1::{ChromaLocation, SamplerYcbcrModelConversion, SamplerYcbcrRange},
    vulkan1_3::FormatFeatureFlags2,
};
use std::ffi::CStr;
#[doc(alias = "VkImportAndroidHardwareBufferInfoANDROID")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImportAndroidHardwareBufferInfoANDROID {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    buffer: *mut AHardwareBuffer,
}
#[doc(alias = "VkAndroidHardwareBufferUsageANDROID")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AndroidHardwareBufferUsageANDROID {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "androidHardwareBufferUsage")]
    android_hardware_buffer_usage: u64,
}
#[doc(alias = "VkAndroidHardwareBufferPropertiesANDROID")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AndroidHardwareBufferPropertiesANDROID {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "allocationSize")]
    allocation_size: DeviceSize,
    #[doc(alias = "memoryTypeBits")]
    memory_type_bits: u32,
}
#[doc(alias = "VkMemoryGetAndroidHardwareBufferInfoANDROID")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryGetAndroidHardwareBufferInfoANDROID {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    memory: DeviceMemory,
}
#[doc(alias = "VkAndroidHardwareBufferFormatPropertiesANDROID")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AndroidHardwareBufferFormatPropertiesANDROID {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    format: Format,
    #[doc(alias = "externalFormat")]
    external_format: u64,
    #[doc(alias = "formatFeatures")]
    format_features: FormatFeatureFlags,
    #[doc(alias = "samplerYcbcrConversionComponents")]
    sampler_ycbcr_conversion_components: ComponentMapping,
    #[doc(alias = "suggestedYcbcrModel")]
    suggested_ycbcr_model: SamplerYcbcrModelConversion,
    #[doc(alias = "suggestedYcbcrRange")]
    suggested_ycbcr_range: SamplerYcbcrRange,
    #[doc(alias = "suggestedXChromaOffset")]
    suggested_x_chroma_offset: ChromaLocation,
    #[doc(alias = "suggestedYChromaOffset")]
    suggested_y_chroma_offset: ChromaLocation,
}
#[doc(alias = "VkExternalFormatANDROID")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExternalFormatANDROID {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "externalFormat")]
    external_format: u64,
}
#[doc(alias = "VkAndroidHardwareBufferFormatProperties2ANDROID")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AndroidHardwareBufferFormatProperties2ANDROID {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    format: Format,
    #[doc(alias = "externalFormat")]
    external_format: u64,
    #[doc(alias = "formatFeatures")]
    format_features: FormatFeatureFlags2,
    #[doc(alias = "samplerYcbcrConversionComponents")]
    sampler_ycbcr_conversion_components: ComponentMapping,
    #[doc(alias = "suggestedYcbcrModel")]
    suggested_ycbcr_model: SamplerYcbcrModelConversion,
    #[doc(alias = "suggestedYcbcrRange")]
    suggested_ycbcr_range: SamplerYcbcrRange,
    #[doc(alias = "suggestedXChromaOffset")]
    suggested_x_chroma_offset: ChromaLocation,
    #[doc(alias = "suggestedYChromaOffset")]
    suggested_y_chroma_offset: ChromaLocation,
}
pub type AHardwareBuffer = std::ffi::c_void;
#[doc(alias = "VK_ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_SPEC_VERSION")]
pub const ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_SPEC_VERSION: u32 = 5;
#[doc(alias = "VK_ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_EXTENSION_NAME")]
pub const ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_EXTENSION_NAME: &'static CStr =
    cstr!("VK_ANDROID_external_memory_android_hardware_buffer");
#[doc(alias = "vkGetAndroidHardwareBufferPropertiesANDROID")]
pub type FNGetAndroidHardwareBufferPropertiesAndroid = unsafe extern "system" fn(
    device: Device,
    buffer: *const AHardwareBuffer,
    p_properties: *mut AndroidHardwareBufferPropertiesANDROID,
) -> VulkanResultCodes;
#[doc(alias = "vkGetMemoryAndroidHardwareBufferANDROID")]
pub type FNGetMemoryAndroidHardwareBufferAndroid = unsafe extern "system" fn(
    device: Device,
    p_info: *const MemoryGetAndroidHardwareBufferInfoANDROID,
    p_buffer: *mut *mut AHardwareBuffer,
) -> VulkanResultCodes;
