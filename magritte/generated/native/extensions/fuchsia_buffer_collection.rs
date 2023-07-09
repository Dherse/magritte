use crate::native::{
    opaque::zx_handle_t,
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, BaseOutStructure, BufferCreateInfo, ComponentMapping, Device,
        FormatFeatureFlags, ImageCreateInfo, StructureType, VulkanResultCodes,
    },
    vulkan1_1::{ChromaLocation, SamplerYcbcrModelConversion, SamplerYcbcrRange},
};
#[doc(alias = "VkImportMemoryBufferCollectionFUCHSIA")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImportMemoryBufferCollectionFUCHSIA {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub collection: BufferCollectionFUCHSIA,
    pub index: u32,
}
impl Default for ImportMemoryBufferCollectionFUCHSIA {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImportMemoryBufferCollectionFuchsia,
            p_next: unsafe { std::mem::zeroed() },
            collection: unsafe { std::mem::zeroed() },
            index: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkBufferCollectionImageCreateInfoFUCHSIA")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BufferCollectionImageCreateInfoFUCHSIA {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub collection: BufferCollectionFUCHSIA,
    pub index: u32,
}
impl Default for BufferCollectionImageCreateInfoFUCHSIA {
    fn default() -> Self {
        Self {
            s_type: StructureType::BufferCollectionImageCreateInfoFuchsia,
            p_next: unsafe { std::mem::zeroed() },
            collection: unsafe { std::mem::zeroed() },
            index: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkBufferCollectionBufferCreateInfoFUCHSIA")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BufferCollectionBufferCreateInfoFUCHSIA {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub collection: BufferCollectionFUCHSIA,
    pub index: u32,
}
impl Default for BufferCollectionBufferCreateInfoFUCHSIA {
    fn default() -> Self {
        Self {
            s_type: StructureType::BufferCollectionBufferCreateInfoFuchsia,
            p_next: unsafe { std::mem::zeroed() },
            collection: unsafe { std::mem::zeroed() },
            index: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkBufferCollectionCreateInfoFUCHSIA")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BufferCollectionCreateInfoFUCHSIA {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "collectionToken")]
    pub collection_token: zx_handle_t,
}
impl Default for BufferCollectionCreateInfoFUCHSIA {
    fn default() -> Self {
        Self {
            s_type: StructureType::BufferCollectionCreateInfoFuchsia,
            p_next: unsafe { std::mem::zeroed() },
            collection_token: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkBufferCollectionPropertiesFUCHSIA")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BufferCollectionPropertiesFUCHSIA {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "memoryTypeBits")]
    pub memory_type_bits: u32,
    #[doc(alias = "bufferCount")]
    pub buffer_count: u32,
    #[doc(alias = "createInfoIndex")]
    pub create_info_index: u32,
    #[doc(alias = "sysmemPixelFormat")]
    pub sysmem_pixel_format: u64,
    #[doc(alias = "formatFeatures")]
    pub format_features: FormatFeatureFlags,
    #[doc(alias = "sysmemColorSpaceIndex")]
    pub sysmem_color_space_index: SysmemColorSpaceFUCHSIA,
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
impl Default for BufferCollectionPropertiesFUCHSIA {
    fn default() -> Self {
        Self {
            s_type: StructureType::BufferCollectionPropertiesFuchsia,
            p_next: unsafe { std::mem::zeroed() },
            memory_type_bits: unsafe { std::mem::zeroed() },
            buffer_count: unsafe { std::mem::zeroed() },
            create_info_index: unsafe { std::mem::zeroed() },
            sysmem_pixel_format: unsafe { std::mem::zeroed() },
            format_features: unsafe { std::mem::zeroed() },
            sysmem_color_space_index: unsafe { std::mem::zeroed() },
            sampler_ycbcr_conversion_components: unsafe { std::mem::zeroed() },
            suggested_ycbcr_model: unsafe { std::mem::zeroed() },
            suggested_ycbcr_range: unsafe { std::mem::zeroed() },
            suggested_x_chroma_offset: unsafe { std::mem::zeroed() },
            suggested_y_chroma_offset: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkBufferConstraintsInfoFUCHSIA")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BufferConstraintsInfoFUCHSIA {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "createInfo")]
    pub create_info: BufferCreateInfo,
    #[doc(alias = "requiredFormatFeatures")]
    pub required_format_features: FormatFeatureFlags,
    #[doc(alias = "bufferCollectionConstraints")]
    pub buffer_collection_constraints: BufferCollectionConstraintsInfoFUCHSIA,
}
impl Default for BufferConstraintsInfoFUCHSIA {
    fn default() -> Self {
        Self {
            s_type: StructureType::BufferConstraintsInfoFuchsia,
            p_next: unsafe { std::mem::zeroed() },
            create_info: unsafe { std::mem::zeroed() },
            required_format_features: unsafe { std::mem::zeroed() },
            buffer_collection_constraints: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSysmemColorSpaceFUCHSIA")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SysmemColorSpaceFUCHSIA {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "colorSpace")]
    pub color_space: u32,
}
impl Default for SysmemColorSpaceFUCHSIA {
    fn default() -> Self {
        Self {
            s_type: StructureType::SysmemColorSpaceFuchsia,
            p_next: unsafe { std::mem::zeroed() },
            color_space: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkImageFormatConstraintsInfoFUCHSIA")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageFormatConstraintsInfoFUCHSIA {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "imageCreateInfo")]
    pub image_create_info: ImageCreateInfo,
    #[doc(alias = "requiredFormatFeatures")]
    pub required_format_features: FormatFeatureFlags,
    pub flags: ImageFormatConstraintsFlagsFUCHSIA,
    #[doc(alias = "sysmemPixelFormat")]
    pub sysmem_pixel_format: u64,
    #[doc(alias = "colorSpaceCount")]
    pub color_space_count: u32,
    #[doc(alias = "pColorSpaces")]
    pub color_spaces: *const SysmemColorSpaceFUCHSIA,
}
impl Default for ImageFormatConstraintsInfoFUCHSIA {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImageFormatConstraintsInfoFuchsia,
            p_next: unsafe { std::mem::zeroed() },
            image_create_info: unsafe { std::mem::zeroed() },
            required_format_features: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            sysmem_pixel_format: unsafe { std::mem::zeroed() },
            color_space_count: unsafe { std::mem::zeroed() },
            color_spaces: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkImageConstraintsInfoFUCHSIA")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageConstraintsInfoFUCHSIA {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "formatConstraintsCount")]
    pub format_constraints_count: u32,
    #[doc(alias = "pFormatConstraints")]
    pub format_constraints: *const ImageFormatConstraintsInfoFUCHSIA,
    #[doc(alias = "bufferCollectionConstraints")]
    pub buffer_collection_constraints: BufferCollectionConstraintsInfoFUCHSIA,
    pub flags: ImageConstraintsInfoFlagsFUCHSIA,
}
impl Default for ImageConstraintsInfoFUCHSIA {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImageConstraintsInfoFuchsia,
            p_next: unsafe { std::mem::zeroed() },
            format_constraints_count: unsafe { std::mem::zeroed() },
            format_constraints: unsafe { std::mem::zeroed() },
            buffer_collection_constraints: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkBufferCollectionConstraintsInfoFUCHSIA")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BufferCollectionConstraintsInfoFUCHSIA {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "minBufferCount")]
    pub min_buffer_count: u32,
    #[doc(alias = "maxBufferCount")]
    pub max_buffer_count: u32,
    #[doc(alias = "minBufferCountForCamping")]
    pub min_buffer_count_for_camping: u32,
    #[doc(alias = "minBufferCountForDedicatedSlack")]
    pub min_buffer_count_for_dedicated_slack: u32,
    #[doc(alias = "minBufferCountForSharedSlack")]
    pub min_buffer_count_for_shared_slack: u32,
}
impl Default for BufferCollectionConstraintsInfoFUCHSIA {
    fn default() -> Self {
        Self {
            s_type: StructureType::BufferCollectionConstraintsInfoFuchsia,
            p_next: unsafe { std::mem::zeroed() },
            min_buffer_count: unsafe { std::mem::zeroed() },
            max_buffer_count: unsafe { std::mem::zeroed() },
            min_buffer_count_for_camping: unsafe { std::mem::zeroed() },
            min_buffer_count_for_dedicated_slack: unsafe { std::mem::zeroed() },
            min_buffer_count_for_shared_slack: unsafe { std::mem::zeroed() },
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkBufferCollectionFUCHSIA")]
#[repr(transparent)]
pub struct BufferCollectionFUCHSIA(u64);
impl BufferCollectionFUCHSIA {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for BufferCollectionFUCHSIA {
    fn default() -> Self {
        Self::null()
    }
}
pub use crate::common::extensions::fuchsia_buffer_collection::{
    ImageConstraintsInfoFlagBitsFUCHSIA, ImageConstraintsInfoFlagsFUCHSIA, ImageFormatConstraintsFlagsFUCHSIA,
    FUCHSIA_BUFFER_COLLECTION_EXTENSION_NAME, FUCHSIA_BUFFER_COLLECTION_SPEC_VERSION,
};
#[doc(alias = "vkCreateBufferCollectionFUCHSIA")]
pub type FNCreateBufferCollectionFuchsia = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const BufferCollectionCreateInfoFUCHSIA,
    p_allocator: *const AllocationCallbacks,
    p_collection: *mut BufferCollectionFUCHSIA,
) -> VulkanResultCodes;
#[doc(alias = "vkSetBufferCollectionBufferConstraintsFUCHSIA")]
pub type FNSetBufferCollectionBufferConstraintsFuchsia = unsafe extern "system" fn(
    device: Device,
    collection: BufferCollectionFUCHSIA,
    p_buffer_constraints_info: *const BufferConstraintsInfoFUCHSIA,
) -> VulkanResultCodes;
#[doc(alias = "vkSetBufferCollectionImageConstraintsFUCHSIA")]
pub type FNSetBufferCollectionImageConstraintsFuchsia = unsafe extern "system" fn(
    device: Device,
    collection: BufferCollectionFUCHSIA,
    p_image_constraints_info: *const ImageConstraintsInfoFUCHSIA,
) -> VulkanResultCodes;
#[doc(alias = "vkDestroyBufferCollectionFUCHSIA")]
pub type FNDestroyBufferCollectionFuchsia = unsafe extern "system" fn(
    device: Device,
    collection: BufferCollectionFUCHSIA,
    p_allocator: *const AllocationCallbacks,
);
#[doc(alias = "vkGetBufferCollectionPropertiesFUCHSIA")]
pub type FNGetBufferCollectionPropertiesFuchsia = unsafe extern "system" fn(
    device: Device,
    collection: BufferCollectionFUCHSIA,
    p_properties: *mut BufferCollectionPropertiesFUCHSIA,
) -> VulkanResultCodes;
