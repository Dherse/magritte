use crate::native::{
    vulkan1_0::{
        BaseInStructure, BaseOutStructure, ComponentMapping, Device, DeviceMemory, DeviceSize, Format,
        FormatFeatureFlags, StructureType, VulkanResultCodes,
    },
    vulkan1_1::{ChromaLocation, SamplerYcbcrModelConversion, SamplerYcbcrRange},
    vulkan1_3::FormatFeatureFlags2,
};
#[doc(alias = "VkImportAndroidHardwareBufferInfoANDROID")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImportAndroidHardwareBufferInfoANDROID {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub buffer: *mut AHardwareBuffer,
}
impl Default for ImportAndroidHardwareBufferInfoANDROID {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImportAndroidHardwareBufferInfoAndroid,
            p_next: unsafe { std::mem::zeroed() },
            buffer: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkAndroidHardwareBufferUsageANDROID")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AndroidHardwareBufferUsageANDROID {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "androidHardwareBufferUsage")]
    pub android_hardware_buffer_usage: u64,
}
impl Default for AndroidHardwareBufferUsageANDROID {
    fn default() -> Self {
        Self {
            s_type: StructureType::AndroidHardwareBufferUsageAndroid,
            p_next: unsafe { std::mem::zeroed() },
            android_hardware_buffer_usage: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkAndroidHardwareBufferPropertiesANDROID")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AndroidHardwareBufferPropertiesANDROID {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "allocationSize")]
    pub allocation_size: DeviceSize,
    #[doc(alias = "memoryTypeBits")]
    pub memory_type_bits: u32,
}
impl Default for AndroidHardwareBufferPropertiesANDROID {
    fn default() -> Self {
        Self {
            s_type: StructureType::AndroidHardwareBufferPropertiesAndroid,
            p_next: unsafe { std::mem::zeroed() },
            allocation_size: unsafe { std::mem::zeroed() },
            memory_type_bits: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkMemoryGetAndroidHardwareBufferInfoANDROID")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryGetAndroidHardwareBufferInfoANDROID {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub memory: DeviceMemory,
}
impl Default for MemoryGetAndroidHardwareBufferInfoANDROID {
    fn default() -> Self {
        Self {
            s_type: StructureType::MemoryGetAndroidHardwareBufferInfoAndroid,
            p_next: unsafe { std::mem::zeroed() },
            memory: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkAndroidHardwareBufferFormatPropertiesANDROID")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AndroidHardwareBufferFormatPropertiesANDROID {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    pub format: Format,
    #[doc(alias = "externalFormat")]
    pub external_format: u64,
    #[doc(alias = "formatFeatures")]
    pub format_features: FormatFeatureFlags,
    #[doc(alias = "samplerYcbcrConversionComponents")]
    pub sampler_ycbcr_conversion_components: ComponentMapping,
    #[doc(alias = "suggestedYcbcrModel")]
    pub suggested_ycbcr_model: SamplerYcbcrModelConversion,
    #[doc(alias = "suggestedYcbcrRange")]
    pub suggested_ycbcr_range: SamplerYcbcrRange,
    #[doc(alias = "suggestedXChromaOffset")]
    pub suggested_x_chroma_offset: ChromaLocation,
    #[doc(alias = "suggestedYChromaOffset")]
    pub suggested_y_chroma_offset: ChromaLocation,
}
impl Default for AndroidHardwareBufferFormatPropertiesANDROID {
    fn default() -> Self {
        Self {
            s_type: StructureType::AndroidHardwareBufferFormatPropertiesAndroid,
            p_next: unsafe { std::mem::zeroed() },
            format: unsafe { std::mem::zeroed() },
            external_format: unsafe { std::mem::zeroed() },
            format_features: unsafe { std::mem::zeroed() },
            sampler_ycbcr_conversion_components: unsafe { std::mem::zeroed() },
            suggested_ycbcr_model: unsafe { std::mem::zeroed() },
            suggested_ycbcr_range: unsafe { std::mem::zeroed() },
            suggested_x_chroma_offset: unsafe { std::mem::zeroed() },
            suggested_y_chroma_offset: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkExternalFormatANDROID")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExternalFormatANDROID {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "externalFormat")]
    pub external_format: u64,
}
impl Default for ExternalFormatANDROID {
    fn default() -> Self {
        Self {
            s_type: StructureType::ExternalFormatAndroid,
            p_next: unsafe { std::mem::zeroed() },
            external_format: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkAndroidHardwareBufferFormatProperties2ANDROID")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AndroidHardwareBufferFormatProperties2ANDROID {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    pub format: Format,
    #[doc(alias = "externalFormat")]
    pub external_format: u64,
    #[doc(alias = "formatFeatures")]
    pub format_features: FormatFeatureFlags2,
    #[doc(alias = "samplerYcbcrConversionComponents")]
    pub sampler_ycbcr_conversion_components: ComponentMapping,
    #[doc(alias = "suggestedYcbcrModel")]
    pub suggested_ycbcr_model: SamplerYcbcrModelConversion,
    #[doc(alias = "suggestedYcbcrRange")]
    pub suggested_ycbcr_range: SamplerYcbcrRange,
    #[doc(alias = "suggestedXChromaOffset")]
    pub suggested_x_chroma_offset: ChromaLocation,
    #[doc(alias = "suggestedYChromaOffset")]
    pub suggested_y_chroma_offset: ChromaLocation,
}
impl Default for AndroidHardwareBufferFormatProperties2ANDROID {
    fn default() -> Self {
        Self {
            s_type: StructureType::AndroidHardwareBufferFormatProperties2Android,
            p_next: unsafe { std::mem::zeroed() },
            format: unsafe { std::mem::zeroed() },
            external_format: unsafe { std::mem::zeroed() },
            format_features: unsafe { std::mem::zeroed() },
            sampler_ycbcr_conversion_components: unsafe { std::mem::zeroed() },
            suggested_ycbcr_model: unsafe { std::mem::zeroed() },
            suggested_ycbcr_range: unsafe { std::mem::zeroed() },
            suggested_x_chroma_offset: unsafe { std::mem::zeroed() },
            suggested_y_chroma_offset: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::android_external_memory_android_hardware_buffer::{
    AHardwareBuffer, ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_EXTENSION_NAME,
    ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_SPEC_VERSION,
};
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
