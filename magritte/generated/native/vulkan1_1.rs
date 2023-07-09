pub use crate::common::vulkan1_1::{
    DescriptorUpdateTemplateEntry, ExternalMemoryProperties, InputAttachmentAspectReference,
};
use crate::native::vulkan1_0::{
    AllocationCallbacks, BaseInStructure, BaseOutStructure, Bool32, Buffer, BufferCreateFlags, BufferUsageFlags,
    CommandBuffer, CommandPool, ComponentMapping, DescriptorSet, DescriptorSetLayout, DescriptorSetLayoutCreateInfo,
    Device, DeviceMemory, DeviceQueueCreateFlags, DeviceSize, Filter, Format, FormatProperties, Image,
    ImageAspectFlagBits, ImageCreateFlags, ImageFormatProperties, ImageTiling, ImageType, ImageUsageFlags, Instance,
    MemoryRequirements, PhysicalDevice, PhysicalDeviceFeatures, PhysicalDeviceMemoryProperties,
    PhysicalDeviceProperties, PipelineBindPoint, PipelineLayout, Queue, QueueFamilyProperties, Rect2D,
    SampleCountFlagBits, ShaderStageFlags, SparseImageFormatProperties, SparseImageMemoryRequirements, StructureType,
    VulkanResultCodes, LUID_SIZE, MAX_DEVICE_GROUP_SIZE,
};
use uuid::Uuid;
///See [`PhysicalDeviceVariablePointersFeatures`]
#[doc(alias = "VkPhysicalDeviceVariablePointerFeatures")]
pub type PhysicalDeviceVariablePointerFeatures = PhysicalDeviceVariablePointersFeatures;
///See [`PhysicalDeviceShaderDrawParametersFeatures`]
#[doc(alias = "VkPhysicalDeviceShaderDrawParameterFeatures")]
pub type PhysicalDeviceShaderDrawParameterFeatures = PhysicalDeviceShaderDrawParametersFeatures;
#[doc(alias = "VkPhysicalDeviceFeatures2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFeatures2 {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    pub features: PhysicalDeviceFeatures,
}
impl Default for PhysicalDeviceFeatures2 {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceFeatures2,
            p_next: unsafe { std::mem::zeroed() },
            features: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceProperties2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceProperties2 {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    pub properties: PhysicalDeviceProperties,
}
impl Default for PhysicalDeviceProperties2 {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceProperties2,
            p_next: unsafe { std::mem::zeroed() },
            properties: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkFormatProperties2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct FormatProperties2 {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "formatProperties")]
    pub format_properties: FormatProperties,
}
impl Default for FormatProperties2 {
    fn default() -> Self {
        Self {
            s_type: StructureType::FormatProperties2,
            p_next: unsafe { std::mem::zeroed() },
            format_properties: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkImageFormatProperties2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageFormatProperties2 {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "imageFormatProperties")]
    pub image_format_properties: ImageFormatProperties,
}
impl Default for ImageFormatProperties2 {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImageFormatProperties2,
            p_next: unsafe { std::mem::zeroed() },
            image_format_properties: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceImageFormatInfo2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceImageFormatInfo2 {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub format: Format,
    #[doc(alias = "type")]
    pub type_: ImageType,
    pub tiling: ImageTiling,
    pub usage: ImageUsageFlags,
    pub flags: ImageCreateFlags,
}
impl Default for PhysicalDeviceImageFormatInfo2 {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceImageFormatInfo2,
            p_next: unsafe { std::mem::zeroed() },
            format: unsafe { std::mem::zeroed() },
            type_: unsafe { std::mem::zeroed() },
            tiling: unsafe { std::mem::zeroed() },
            usage: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkQueueFamilyProperties2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct QueueFamilyProperties2 {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "queueFamilyProperties")]
    pub queue_family_properties: QueueFamilyProperties,
}
impl Default for QueueFamilyProperties2 {
    fn default() -> Self {
        Self {
            s_type: StructureType::QueueFamilyProperties2,
            p_next: unsafe { std::mem::zeroed() },
            queue_family_properties: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceMemoryProperties2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMemoryProperties2 {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "memoryProperties")]
    pub memory_properties: PhysicalDeviceMemoryProperties,
}
impl Default for PhysicalDeviceMemoryProperties2 {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceMemoryProperties2,
            p_next: unsafe { std::mem::zeroed() },
            memory_properties: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSparseImageFormatProperties2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SparseImageFormatProperties2 {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    pub properties: SparseImageFormatProperties,
}
impl Default for SparseImageFormatProperties2 {
    fn default() -> Self {
        Self {
            s_type: StructureType::SparseImageFormatProperties2,
            p_next: unsafe { std::mem::zeroed() },
            properties: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceSparseImageFormatInfo2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSparseImageFormatInfo2 {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub format: Format,
    #[doc(alias = "type")]
    pub type_: ImageType,
    pub samples: SampleCountFlagBits,
    pub usage: ImageUsageFlags,
    pub tiling: ImageTiling,
}
impl Default for PhysicalDeviceSparseImageFormatInfo2 {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceSparseImageFormatInfo2,
            p_next: unsafe { std::mem::zeroed() },
            format: unsafe { std::mem::zeroed() },
            type_: unsafe { std::mem::zeroed() },
            samples: unsafe { std::mem::zeroed() },
            usage: unsafe { std::mem::zeroed() },
            tiling: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceVariablePointersFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVariablePointersFeatures {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "variablePointersStorageBuffer")]
    pub variable_pointers_storage_buffer: Bool32,
    #[doc(alias = "variablePointers")]
    pub variable_pointers: Bool32,
}
impl Default for PhysicalDeviceVariablePointersFeatures {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceVariablePointersFeatures,
            p_next: unsafe { std::mem::zeroed() },
            variable_pointers_storage_buffer: unsafe { std::mem::zeroed() },
            variable_pointers: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceExternalImageFormatInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceExternalImageFormatInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "handleType")]
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
}
impl Default for PhysicalDeviceExternalImageFormatInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceExternalImageFormatInfo,
            p_next: unsafe { std::mem::zeroed() },
            handle_type: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkExternalImageFormatProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExternalImageFormatProperties {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "externalMemoryProperties")]
    pub external_memory_properties: ExternalMemoryProperties,
}
impl Default for ExternalImageFormatProperties {
    fn default() -> Self {
        Self {
            s_type: StructureType::ExternalImageFormatProperties,
            p_next: unsafe { std::mem::zeroed() },
            external_memory_properties: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceExternalBufferInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceExternalBufferInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: BufferCreateFlags,
    pub usage: BufferUsageFlags,
    #[doc(alias = "handleType")]
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
}
impl Default for PhysicalDeviceExternalBufferInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceExternalBufferInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            usage: unsafe { std::mem::zeroed() },
            handle_type: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkExternalBufferProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExternalBufferProperties {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "externalMemoryProperties")]
    pub external_memory_properties: ExternalMemoryProperties,
}
impl Default for ExternalBufferProperties {
    fn default() -> Self {
        Self {
            s_type: StructureType::ExternalBufferProperties,
            p_next: unsafe { std::mem::zeroed() },
            external_memory_properties: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceIDProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceIdProperties {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "deviceUUID")]
    pub device_uuid: Uuid,
    #[doc(alias = "driverUUID")]
    pub driver_uuid: Uuid,
    #[doc(alias = "deviceLUID")]
    pub device_luid: [u8; LUID_SIZE as usize],
    #[doc(alias = "deviceNodeMask")]
    pub device_node_mask: u32,
    #[doc(alias = "deviceLUIDValid")]
    pub device_luid_valid: Bool32,
}
impl Default for PhysicalDeviceIdProperties {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceIdProperties,
            p_next: unsafe { std::mem::zeroed() },
            device_uuid: unsafe { std::mem::zeroed() },
            driver_uuid: unsafe { std::mem::zeroed() },
            device_luid: unsafe { std::mem::zeroed() },
            device_node_mask: unsafe { std::mem::zeroed() },
            device_luid_valid: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkExternalMemoryImageCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExternalMemoryImageCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "handleTypes")]
    pub handle_types: ExternalMemoryHandleTypeFlags,
}
impl Default for ExternalMemoryImageCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::ExternalMemoryImageCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            handle_types: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkExternalMemoryBufferCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExternalMemoryBufferCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "handleTypes")]
    pub handle_types: ExternalMemoryHandleTypeFlags,
}
impl Default for ExternalMemoryBufferCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::ExternalMemoryBufferCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            handle_types: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkExportMemoryAllocateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExportMemoryAllocateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "handleTypes")]
    pub handle_types: ExternalMemoryHandleTypeFlags,
}
impl Default for ExportMemoryAllocateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::ExportMemoryAllocateInfo,
            p_next: unsafe { std::mem::zeroed() },
            handle_types: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceExternalSemaphoreInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceExternalSemaphoreInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "handleType")]
    pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
}
impl Default for PhysicalDeviceExternalSemaphoreInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceExternalSemaphoreInfo,
            p_next: unsafe { std::mem::zeroed() },
            handle_type: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkExternalSemaphoreProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExternalSemaphoreProperties {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "exportFromImportedHandleTypes")]
    pub export_from_imported_handle_types: ExternalSemaphoreHandleTypeFlags,
    #[doc(alias = "compatibleHandleTypes")]
    pub compatible_handle_types: ExternalSemaphoreHandleTypeFlags,
    #[doc(alias = "externalSemaphoreFeatures")]
    pub external_semaphore_features: ExternalSemaphoreFeatureFlags,
}
impl Default for ExternalSemaphoreProperties {
    fn default() -> Self {
        Self {
            s_type: StructureType::ExternalSemaphoreProperties,
            p_next: unsafe { std::mem::zeroed() },
            export_from_imported_handle_types: unsafe { std::mem::zeroed() },
            compatible_handle_types: unsafe { std::mem::zeroed() },
            external_semaphore_features: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkExportSemaphoreCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExportSemaphoreCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "handleTypes")]
    pub handle_types: ExternalSemaphoreHandleTypeFlags,
}
impl Default for ExportSemaphoreCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::ExportSemaphoreCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            handle_types: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceExternalFenceInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceExternalFenceInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "handleType")]
    pub handle_type: ExternalFenceHandleTypeFlagBits,
}
impl Default for PhysicalDeviceExternalFenceInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceExternalFenceInfo,
            p_next: unsafe { std::mem::zeroed() },
            handle_type: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkExternalFenceProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExternalFenceProperties {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "exportFromImportedHandleTypes")]
    pub export_from_imported_handle_types: ExternalFenceHandleTypeFlags,
    #[doc(alias = "compatibleHandleTypes")]
    pub compatible_handle_types: ExternalFenceHandleTypeFlags,
    #[doc(alias = "externalFenceFeatures")]
    pub external_fence_features: ExternalFenceFeatureFlags,
}
impl Default for ExternalFenceProperties {
    fn default() -> Self {
        Self {
            s_type: StructureType::ExternalFenceProperties,
            p_next: unsafe { std::mem::zeroed() },
            export_from_imported_handle_types: unsafe { std::mem::zeroed() },
            compatible_handle_types: unsafe { std::mem::zeroed() },
            external_fence_features: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkExportFenceCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExportFenceCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "handleTypes")]
    pub handle_types: ExternalFenceHandleTypeFlags,
}
impl Default for ExportFenceCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::ExportFenceCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            handle_types: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceMultiviewFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMultiviewFeatures {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    pub multiview: Bool32,
    #[doc(alias = "multiviewGeometryShader")]
    pub multiview_geometry_shader: Bool32,
    #[doc(alias = "multiviewTessellationShader")]
    pub multiview_tessellation_shader: Bool32,
}
impl Default for PhysicalDeviceMultiviewFeatures {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceMultiviewFeatures,
            p_next: unsafe { std::mem::zeroed() },
            multiview: unsafe { std::mem::zeroed() },
            multiview_geometry_shader: unsafe { std::mem::zeroed() },
            multiview_tessellation_shader: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceMultiviewProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMultiviewProperties {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "maxMultiviewViewCount")]
    pub max_multiview_view_count: u32,
    #[doc(alias = "maxMultiviewInstanceIndex")]
    pub max_multiview_instance_index: u32,
}
impl Default for PhysicalDeviceMultiviewProperties {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceMultiviewProperties,
            p_next: unsafe { std::mem::zeroed() },
            max_multiview_view_count: unsafe { std::mem::zeroed() },
            max_multiview_instance_index: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkRenderPassMultiviewCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RenderPassMultiviewCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "subpassCount")]
    pub subpass_count: u32,
    #[doc(alias = "pViewMasks")]
    pub view_masks: *const u32,
    #[doc(alias = "dependencyCount")]
    pub dependency_count: u32,
    #[doc(alias = "pViewOffsets")]
    pub view_offsets: *const i32,
    #[doc(alias = "correlationMaskCount")]
    pub correlation_mask_count: u32,
    #[doc(alias = "pCorrelationMasks")]
    pub correlation_masks: *const u32,
}
impl Default for RenderPassMultiviewCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::RenderPassMultiviewCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            subpass_count: unsafe { std::mem::zeroed() },
            view_masks: unsafe { std::mem::zeroed() },
            dependency_count: unsafe { std::mem::zeroed() },
            view_offsets: unsafe { std::mem::zeroed() },
            correlation_mask_count: unsafe { std::mem::zeroed() },
            correlation_masks: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceGroupProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceGroupProperties {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "physicalDeviceCount")]
    pub physical_device_count: u32,
    #[doc(alias = "physicalDevices")]
    pub physical_devices: [PhysicalDevice; MAX_DEVICE_GROUP_SIZE as usize],
    #[doc(alias = "subsetAllocation")]
    pub subset_allocation: Bool32,
}
impl Default for PhysicalDeviceGroupProperties {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceGroupProperties,
            p_next: unsafe { std::mem::zeroed() },
            physical_device_count: unsafe { std::mem::zeroed() },
            physical_devices: unsafe { std::mem::zeroed() },
            subset_allocation: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkMemoryAllocateFlagsInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryAllocateFlagsInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: MemoryAllocateFlags,
    #[doc(alias = "deviceMask")]
    pub device_mask: u32,
}
impl Default for MemoryAllocateFlagsInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::MemoryAllocateFlagsInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            device_mask: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkBindBufferMemoryInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BindBufferMemoryInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub buffer: Buffer,
    pub memory: DeviceMemory,
    #[doc(alias = "memoryOffset")]
    pub memory_offset: DeviceSize,
}
impl Default for BindBufferMemoryInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::BindBufferMemoryInfo,
            p_next: unsafe { std::mem::zeroed() },
            buffer: unsafe { std::mem::zeroed() },
            memory: unsafe { std::mem::zeroed() },
            memory_offset: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkBindBufferMemoryDeviceGroupInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BindBufferMemoryDeviceGroupInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "deviceIndexCount")]
    pub device_index_count: u32,
    #[doc(alias = "pDeviceIndices")]
    pub device_indices: *const u32,
}
impl Default for BindBufferMemoryDeviceGroupInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::BindBufferMemoryDeviceGroupInfo,
            p_next: unsafe { std::mem::zeroed() },
            device_index_count: unsafe { std::mem::zeroed() },
            device_indices: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkBindImageMemoryInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BindImageMemoryInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub image: Image,
    pub memory: DeviceMemory,
    #[doc(alias = "memoryOffset")]
    pub memory_offset: DeviceSize,
}
impl Default for BindImageMemoryInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::BindImageMemoryInfo,
            p_next: unsafe { std::mem::zeroed() },
            image: unsafe { std::mem::zeroed() },
            memory: unsafe { std::mem::zeroed() },
            memory_offset: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkBindImageMemoryDeviceGroupInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BindImageMemoryDeviceGroupInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "deviceIndexCount")]
    pub device_index_count: u32,
    #[doc(alias = "pDeviceIndices")]
    pub device_indices: *const u32,
    #[doc(alias = "splitInstanceBindRegionCount")]
    pub split_instance_bind_region_count: u32,
    #[doc(alias = "pSplitInstanceBindRegions")]
    pub split_instance_bind_regions: *const Rect2D,
}
impl Default for BindImageMemoryDeviceGroupInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::BindImageMemoryDeviceGroupInfo,
            p_next: unsafe { std::mem::zeroed() },
            device_index_count: unsafe { std::mem::zeroed() },
            device_indices: unsafe { std::mem::zeroed() },
            split_instance_bind_region_count: unsafe { std::mem::zeroed() },
            split_instance_bind_regions: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDeviceGroupRenderPassBeginInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceGroupRenderPassBeginInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "deviceMask")]
    pub device_mask: u32,
    #[doc(alias = "deviceRenderAreaCount")]
    pub device_render_area_count: u32,
    #[doc(alias = "pDeviceRenderAreas")]
    pub device_render_areas: *const Rect2D,
}
impl Default for DeviceGroupRenderPassBeginInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::DeviceGroupRenderPassBeginInfo,
            p_next: unsafe { std::mem::zeroed() },
            device_mask: unsafe { std::mem::zeroed() },
            device_render_area_count: unsafe { std::mem::zeroed() },
            device_render_areas: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDeviceGroupCommandBufferBeginInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceGroupCommandBufferBeginInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "deviceMask")]
    pub device_mask: u32,
}
impl Default for DeviceGroupCommandBufferBeginInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::DeviceGroupCommandBufferBeginInfo,
            p_next: unsafe { std::mem::zeroed() },
            device_mask: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDeviceGroupSubmitInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceGroupSubmitInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "waitSemaphoreCount")]
    pub wait_semaphore_count: u32,
    #[doc(alias = "pWaitSemaphoreDeviceIndices")]
    pub wait_semaphore_device_indices: *const u32,
    #[doc(alias = "commandBufferCount")]
    pub command_buffer_count: u32,
    #[doc(alias = "pCommandBufferDeviceMasks")]
    pub command_buffer_device_masks: *const u32,
    #[doc(alias = "signalSemaphoreCount")]
    pub signal_semaphore_count: u32,
    #[doc(alias = "pSignalSemaphoreDeviceIndices")]
    pub signal_semaphore_device_indices: *const u32,
}
impl Default for DeviceGroupSubmitInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::DeviceGroupSubmitInfo,
            p_next: unsafe { std::mem::zeroed() },
            wait_semaphore_count: unsafe { std::mem::zeroed() },
            wait_semaphore_device_indices: unsafe { std::mem::zeroed() },
            command_buffer_count: unsafe { std::mem::zeroed() },
            command_buffer_device_masks: unsafe { std::mem::zeroed() },
            signal_semaphore_count: unsafe { std::mem::zeroed() },
            signal_semaphore_device_indices: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDeviceGroupBindSparseInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceGroupBindSparseInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "resourceDeviceIndex")]
    pub resource_device_index: u32,
    #[doc(alias = "memoryDeviceIndex")]
    pub memory_device_index: u32,
}
impl Default for DeviceGroupBindSparseInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::DeviceGroupBindSparseInfo,
            p_next: unsafe { std::mem::zeroed() },
            resource_device_index: unsafe { std::mem::zeroed() },
            memory_device_index: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDeviceGroupDeviceCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceGroupDeviceCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "physicalDeviceCount")]
    pub physical_device_count: u32,
    #[doc(alias = "pPhysicalDevices")]
    pub physical_devices: *const PhysicalDevice,
}
impl Default for DeviceGroupDeviceCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::DeviceGroupDeviceCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            physical_device_count: unsafe { std::mem::zeroed() },
            physical_devices: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDescriptorUpdateTemplateCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DescriptorUpdateTemplateCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: DescriptorUpdateTemplateCreateFlags,
    #[doc(alias = "descriptorUpdateEntryCount")]
    pub descriptor_update_entry_count: u32,
    #[doc(alias = "pDescriptorUpdateEntries")]
    pub descriptor_update_entries: *const DescriptorUpdateTemplateEntry,
    #[doc(alias = "templateType")]
    pub template_type: DescriptorUpdateTemplateType,
    #[doc(alias = "descriptorSetLayout")]
    pub descriptor_set_layout: DescriptorSetLayout,
    #[doc(alias = "pipelineBindPoint")]
    pub pipeline_bind_point: PipelineBindPoint,
    #[doc(alias = "pipelineLayout")]
    pub pipeline_layout: PipelineLayout,
    pub set: u32,
}
impl Default for DescriptorUpdateTemplateCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::DescriptorUpdateTemplateCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            descriptor_update_entry_count: unsafe { std::mem::zeroed() },
            descriptor_update_entries: unsafe { std::mem::zeroed() },
            template_type: unsafe { std::mem::zeroed() },
            descriptor_set_layout: unsafe { std::mem::zeroed() },
            pipeline_bind_point: unsafe { std::mem::zeroed() },
            pipeline_layout: unsafe { std::mem::zeroed() },
            set: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkRenderPassInputAttachmentAspectCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RenderPassInputAttachmentAspectCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "aspectReferenceCount")]
    pub aspect_reference_count: u32,
    #[doc(alias = "pAspectReferences")]
    pub aspect_references: *const InputAttachmentAspectReference,
}
impl Default for RenderPassInputAttachmentAspectCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::RenderPassInputAttachmentAspectCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            aspect_reference_count: unsafe { std::mem::zeroed() },
            aspect_references: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDevice16BitStorageFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevice16BitStorageFeatures {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "storageBuffer16BitAccess")]
    pub storage_buffer16_bit_access: Bool32,
    #[doc(alias = "uniformAndStorageBuffer16BitAccess")]
    pub uniform_and_storage_buffer16_bit_access: Bool32,
    #[doc(alias = "storagePushConstant16")]
    pub storage_push_constant16: Bool32,
    #[doc(alias = "storageInputOutput16")]
    pub storage_input_output16: Bool32,
}
impl Default for PhysicalDevice16BitStorageFeatures {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDevice16bitStorageFeatures,
            p_next: unsafe { std::mem::zeroed() },
            storage_buffer16_bit_access: unsafe { std::mem::zeroed() },
            uniform_and_storage_buffer16_bit_access: unsafe { std::mem::zeroed() },
            storage_push_constant16: unsafe { std::mem::zeroed() },
            storage_input_output16: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceSubgroupProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSubgroupProperties {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "subgroupSize")]
    pub subgroup_size: u32,
    #[doc(alias = "supportedStages")]
    pub supported_stages: ShaderStageFlags,
    #[doc(alias = "supportedOperations")]
    pub supported_operations: SubgroupFeatureFlags,
    #[doc(alias = "quadOperationsInAllStages")]
    pub quad_operations_in_all_stages: Bool32,
}
impl Default for PhysicalDeviceSubgroupProperties {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceSubgroupProperties,
            p_next: unsafe { std::mem::zeroed() },
            subgroup_size: unsafe { std::mem::zeroed() },
            supported_stages: unsafe { std::mem::zeroed() },
            supported_operations: unsafe { std::mem::zeroed() },
            quad_operations_in_all_stages: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkBufferMemoryRequirementsInfo2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BufferMemoryRequirementsInfo2 {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub buffer: Buffer,
}
impl Default for BufferMemoryRequirementsInfo2 {
    fn default() -> Self {
        Self {
            s_type: StructureType::BufferMemoryRequirementsInfo2,
            p_next: unsafe { std::mem::zeroed() },
            buffer: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkImageMemoryRequirementsInfo2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageMemoryRequirementsInfo2 {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub image: Image,
}
impl Default for ImageMemoryRequirementsInfo2 {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImageMemoryRequirementsInfo2,
            p_next: unsafe { std::mem::zeroed() },
            image: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkImageSparseMemoryRequirementsInfo2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageSparseMemoryRequirementsInfo2 {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub image: Image,
}
impl Default for ImageSparseMemoryRequirementsInfo2 {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImageSparseMemoryRequirementsInfo2,
            p_next: unsafe { std::mem::zeroed() },
            image: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkMemoryRequirements2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryRequirements2 {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "memoryRequirements")]
    pub memory_requirements: MemoryRequirements,
}
impl Default for MemoryRequirements2 {
    fn default() -> Self {
        Self {
            s_type: StructureType::MemoryRequirements2,
            p_next: unsafe { std::mem::zeroed() },
            memory_requirements: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSparseImageMemoryRequirements2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SparseImageMemoryRequirements2 {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "memoryRequirements")]
    pub memory_requirements: SparseImageMemoryRequirements,
}
impl Default for SparseImageMemoryRequirements2 {
    fn default() -> Self {
        Self {
            s_type: StructureType::SparseImageMemoryRequirements2,
            p_next: unsafe { std::mem::zeroed() },
            memory_requirements: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDevicePointClippingProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePointClippingProperties {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "pointClippingBehavior")]
    pub point_clipping_behavior: PointClippingBehavior,
}
impl Default for PhysicalDevicePointClippingProperties {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDevicePointClippingProperties,
            p_next: unsafe { std::mem::zeroed() },
            point_clipping_behavior: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkMemoryDedicatedRequirements")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryDedicatedRequirements {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "prefersDedicatedAllocation")]
    pub prefers_dedicated_allocation: Bool32,
    #[doc(alias = "requiresDedicatedAllocation")]
    pub requires_dedicated_allocation: Bool32,
}
impl Default for MemoryDedicatedRequirements {
    fn default() -> Self {
        Self {
            s_type: StructureType::MemoryDedicatedRequirements,
            p_next: unsafe { std::mem::zeroed() },
            prefers_dedicated_allocation: unsafe { std::mem::zeroed() },
            requires_dedicated_allocation: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkMemoryDedicatedAllocateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryDedicatedAllocateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub image: Image,
    pub buffer: Buffer,
}
impl Default for MemoryDedicatedAllocateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::MemoryDedicatedAllocateInfo,
            p_next: unsafe { std::mem::zeroed() },
            image: unsafe { std::mem::zeroed() },
            buffer: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkImageViewUsageCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageViewUsageCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub usage: ImageUsageFlags,
}
impl Default for ImageViewUsageCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImageViewUsageCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            usage: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineTessellationDomainOriginStateCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineTessellationDomainOriginStateCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "domainOrigin")]
    pub domain_origin: TessellationDomainOrigin,
}
impl Default for PipelineTessellationDomainOriginStateCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineTessellationDomainOriginStateCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            domain_origin: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSamplerYcbcrConversionInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SamplerYcbcrConversionInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub conversion: SamplerYcbcrConversion,
}
impl Default for SamplerYcbcrConversionInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::SamplerYcbcrConversionInfo,
            p_next: unsafe { std::mem::zeroed() },
            conversion: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSamplerYcbcrConversionCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SamplerYcbcrConversionCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub format: Format,
    #[doc(alias = "ycbcrModel")]
    pub ycbcr_model: SamplerYcbcrModelConversion,
    #[doc(alias = "ycbcrRange")]
    pub ycbcr_range: SamplerYcbcrRange,
    pub components: ComponentMapping,
    #[doc(alias = "xChromaOffset")]
    pub x_chroma_offset: ChromaLocation,
    #[doc(alias = "yChromaOffset")]
    pub y_chroma_offset: ChromaLocation,
    #[doc(alias = "chromaFilter")]
    pub chroma_filter: Filter,
    #[doc(alias = "forceExplicitReconstruction")]
    pub force_explicit_reconstruction: Bool32,
}
impl Default for SamplerYcbcrConversionCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::SamplerYcbcrConversionCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            format: unsafe { std::mem::zeroed() },
            ycbcr_model: unsafe { std::mem::zeroed() },
            ycbcr_range: unsafe { std::mem::zeroed() },
            components: unsafe { std::mem::zeroed() },
            x_chroma_offset: unsafe { std::mem::zeroed() },
            y_chroma_offset: unsafe { std::mem::zeroed() },
            chroma_filter: unsafe { std::mem::zeroed() },
            force_explicit_reconstruction: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkBindImagePlaneMemoryInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BindImagePlaneMemoryInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "planeAspect")]
    pub plane_aspect: ImageAspectFlagBits,
}
impl Default for BindImagePlaneMemoryInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::BindImagePlaneMemoryInfo,
            p_next: unsafe { std::mem::zeroed() },
            plane_aspect: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkImagePlaneMemoryRequirementsInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImagePlaneMemoryRequirementsInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "planeAspect")]
    pub plane_aspect: ImageAspectFlagBits,
}
impl Default for ImagePlaneMemoryRequirementsInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImagePlaneMemoryRequirementsInfo,
            p_next: unsafe { std::mem::zeroed() },
            plane_aspect: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceSamplerYcbcrConversionFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSamplerYcbcrConversionFeatures {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "samplerYcbcrConversion")]
    pub sampler_ycbcr_conversion: Bool32,
}
impl Default for PhysicalDeviceSamplerYcbcrConversionFeatures {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceSamplerYcbcrConversionFeatures,
            p_next: unsafe { std::mem::zeroed() },
            sampler_ycbcr_conversion: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSamplerYcbcrConversionImageFormatProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SamplerYcbcrConversionImageFormatProperties {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "combinedImageSamplerDescriptorCount")]
    pub combined_image_sampler_descriptor_count: u32,
}
impl Default for SamplerYcbcrConversionImageFormatProperties {
    fn default() -> Self {
        Self {
            s_type: StructureType::SamplerYcbcrConversionImageFormatProperties,
            p_next: unsafe { std::mem::zeroed() },
            combined_image_sampler_descriptor_count: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkProtectedSubmitInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ProtectedSubmitInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "protectedSubmit")]
    pub protected_submit: Bool32,
}
impl Default for ProtectedSubmitInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::ProtectedSubmitInfo,
            p_next: unsafe { std::mem::zeroed() },
            protected_submit: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceProtectedMemoryFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceProtectedMemoryFeatures {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "protectedMemory")]
    pub protected_memory: Bool32,
}
impl Default for PhysicalDeviceProtectedMemoryFeatures {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceProtectedMemoryFeatures,
            p_next: unsafe { std::mem::zeroed() },
            protected_memory: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceProtectedMemoryProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceProtectedMemoryProperties {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "protectedNoFault")]
    pub protected_no_fault: Bool32,
}
impl Default for PhysicalDeviceProtectedMemoryProperties {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceProtectedMemoryProperties,
            p_next: unsafe { std::mem::zeroed() },
            protected_no_fault: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDeviceQueueInfo2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceQueueInfo2 {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: DeviceQueueCreateFlags,
    #[doc(alias = "queueFamilyIndex")]
    pub queue_family_index: u32,
    #[doc(alias = "queueIndex")]
    pub queue_index: u32,
}
impl Default for DeviceQueueInfo2 {
    fn default() -> Self {
        Self {
            s_type: StructureType::DeviceQueueInfo2,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            queue_family_index: unsafe { std::mem::zeroed() },
            queue_index: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceMaintenance3Properties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMaintenance3Properties {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "maxPerSetDescriptors")]
    pub max_per_set_descriptors: u32,
    #[doc(alias = "maxMemoryAllocationSize")]
    pub max_memory_allocation_size: DeviceSize,
}
impl Default for PhysicalDeviceMaintenance3Properties {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceMaintenance3Properties,
            p_next: unsafe { std::mem::zeroed() },
            max_per_set_descriptors: unsafe { std::mem::zeroed() },
            max_memory_allocation_size: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDescriptorSetLayoutSupport")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DescriptorSetLayoutSupport {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    pub supported: Bool32,
}
impl Default for DescriptorSetLayoutSupport {
    fn default() -> Self {
        Self {
            s_type: StructureType::DescriptorSetLayoutSupport,
            p_next: unsafe { std::mem::zeroed() },
            supported: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceShaderDrawParametersFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderDrawParametersFeatures {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderDrawParameters")]
    pub shader_draw_parameters: Bool32,
}
impl Default for PhysicalDeviceShaderDrawParametersFeatures {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceShaderDrawParametersFeatures,
            p_next: unsafe { std::mem::zeroed() },
            shader_draw_parameters: unsafe { std::mem::zeroed() },
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkDescriptorUpdateTemplate")]
#[repr(transparent)]
pub struct DescriptorUpdateTemplate(u64);
impl DescriptorUpdateTemplate {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for DescriptorUpdateTemplate {
    fn default() -> Self {
        Self::null()
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkSamplerYcbcrConversion")]
#[repr(transparent)]
pub struct SamplerYcbcrConversion(u64);
impl SamplerYcbcrConversion {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for SamplerYcbcrConversion {
    fn default() -> Self {
        Self::null()
    }
}
pub use crate::common::vulkan1_1::{
    ChromaLocation, CommandPoolTrimFlags, DescriptorUpdateTemplateCreateFlags, DescriptorUpdateTemplateType,
    DeviceQueueCreateFlagBits, ExternalFenceFeatureFlagBits, ExternalFenceFeatureFlags,
    ExternalFenceHandleTypeFlagBits, ExternalFenceHandleTypeFlags, ExternalMemoryFeatureFlagBits,
    ExternalMemoryFeatureFlags, ExternalMemoryHandleTypeFlagBits, ExternalMemoryHandleTypeFlags,
    ExternalSemaphoreFeatureFlagBits, ExternalSemaphoreFeatureFlags, ExternalSemaphoreHandleTypeFlagBits,
    ExternalSemaphoreHandleTypeFlags, FenceImportFlagBits, FenceImportFlags, MemoryAllocateFlagBits,
    MemoryAllocateFlags, PeerMemoryFeatureFlagBits, PeerMemoryFeatureFlags, PointClippingBehavior,
    SamplerYcbcrModelConversion, SamplerYcbcrRange, SemaphoreImportFlagBits, SemaphoreImportFlags,
    SubgroupFeatureFlagBits, SubgroupFeatureFlags, TessellationDomainOrigin,
};
#[doc(alias = "vkEnumerateInstanceVersion")]
pub type FNEnumerateInstanceVersion = unsafe extern "system" fn(p_api_version: *mut u32) -> VulkanResultCodes;
#[doc(alias = "vkGetPhysicalDeviceFeatures2")]
pub type FNGetPhysicalDeviceFeatures2 =
    unsafe extern "system" fn(physical_device: PhysicalDevice, p_features: *mut PhysicalDeviceFeatures2);
#[doc(alias = "vkGetPhysicalDeviceProperties2")]
pub type FNGetPhysicalDeviceProperties2 =
    unsafe extern "system" fn(physical_device: PhysicalDevice, p_properties: *mut PhysicalDeviceProperties2);
#[doc(alias = "vkGetPhysicalDeviceFormatProperties2")]
pub type FNGetPhysicalDeviceFormatProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    format: Format,
    p_format_properties: *mut FormatProperties2,
);
#[doc(alias = "vkGetPhysicalDeviceImageFormatProperties2")]
pub type FNGetPhysicalDeviceImageFormatProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_image_format_info: *const PhysicalDeviceImageFormatInfo2,
    p_image_format_properties: *mut ImageFormatProperties2,
) -> VulkanResultCodes;
#[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties2")]
pub type FNGetPhysicalDeviceQueueFamilyProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_queue_family_property_count: *mut u32,
    p_queue_family_properties: *mut QueueFamilyProperties2,
);
#[doc(alias = "vkGetPhysicalDeviceMemoryProperties2")]
pub type FNGetPhysicalDeviceMemoryProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_memory_properties: *mut PhysicalDeviceMemoryProperties2,
);
#[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties2")]
pub type FNGetPhysicalDeviceSparseImageFormatProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_format_info: *const PhysicalDeviceSparseImageFormatInfo2,
    p_property_count: *mut u32,
    p_properties: *mut SparseImageFormatProperties2,
);
#[doc(alias = "vkTrimCommandPool")]
pub type FNTrimCommandPool =
    unsafe extern "system" fn(device: Device, command_pool: CommandPool, flags: CommandPoolTrimFlags);
#[doc(alias = "vkGetPhysicalDeviceExternalBufferProperties")]
pub type FNGetPhysicalDeviceExternalBufferProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_external_buffer_info: *const PhysicalDeviceExternalBufferInfo,
    p_external_buffer_properties: *mut ExternalBufferProperties,
);
#[doc(alias = "vkGetPhysicalDeviceExternalSemaphoreProperties")]
pub type FNGetPhysicalDeviceExternalSemaphoreProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_external_semaphore_info: *const PhysicalDeviceExternalSemaphoreInfo,
    p_external_semaphore_properties: *mut ExternalSemaphoreProperties,
);
#[doc(alias = "vkGetPhysicalDeviceExternalFenceProperties")]
pub type FNGetPhysicalDeviceExternalFenceProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_external_fence_info: *const PhysicalDeviceExternalFenceInfo,
    p_external_fence_properties: *mut ExternalFenceProperties,
);
#[doc(alias = "vkEnumeratePhysicalDeviceGroups")]
pub type FNEnumeratePhysicalDeviceGroups = unsafe extern "system" fn(
    instance: Instance,
    p_physical_device_group_count: *mut u32,
    p_physical_device_group_properties: *mut PhysicalDeviceGroupProperties,
) -> VulkanResultCodes;
#[doc(alias = "vkGetDeviceGroupPeerMemoryFeatures")]
pub type FNGetDeviceGroupPeerMemoryFeatures = unsafe extern "system" fn(
    device: Device,
    heap_index: u32,
    local_device_index: u32,
    remote_device_index: u32,
    p_peer_memory_features: *mut PeerMemoryFeatureFlags,
);
#[doc(alias = "vkBindBufferMemory2")]
pub type FNBindBufferMemory2 = unsafe extern "system" fn(
    device: Device,
    bind_info_count: u32,
    p_bind_infos: *const BindBufferMemoryInfo,
) -> VulkanResultCodes;
#[doc(alias = "vkBindImageMemory2")]
pub type FNBindImageMemory2 = unsafe extern "system" fn(
    device: Device,
    bind_info_count: u32,
    p_bind_infos: *const BindImageMemoryInfo,
) -> VulkanResultCodes;
#[doc(alias = "vkCreateDescriptorUpdateTemplate")]
pub type FNCreateDescriptorUpdateTemplate = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const DescriptorUpdateTemplateCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_descriptor_update_template: *mut DescriptorUpdateTemplate,
) -> VulkanResultCodes;
#[doc(alias = "vkDestroyDescriptorUpdateTemplate")]
pub type FNDestroyDescriptorUpdateTemplate = unsafe extern "system" fn(
    device: Device,
    descriptor_update_template: DescriptorUpdateTemplate,
    p_allocator: *const AllocationCallbacks,
);
#[doc(alias = "vkUpdateDescriptorSetWithTemplate")]
pub type FNUpdateDescriptorSetWithTemplate = unsafe extern "system" fn(
    device: Device,
    descriptor_set: DescriptorSet,
    descriptor_update_template: DescriptorUpdateTemplate,
    p_data: *const std::ffi::c_void,
);
#[doc(alias = "vkGetBufferMemoryRequirements2")]
pub type FNGetBufferMemoryRequirements2 = unsafe extern "system" fn(
    device: Device,
    p_info: *const BufferMemoryRequirementsInfo2,
    p_memory_requirements: *mut MemoryRequirements2,
);
#[doc(alias = "vkGetImageMemoryRequirements2")]
pub type FNGetImageMemoryRequirements2 = unsafe extern "system" fn(
    device: Device,
    p_info: *const ImageMemoryRequirementsInfo2,
    p_memory_requirements: *mut MemoryRequirements2,
);
#[doc(alias = "vkGetImageSparseMemoryRequirements2")]
pub type FNGetImageSparseMemoryRequirements2 = unsafe extern "system" fn(
    device: Device,
    p_info: *const ImageSparseMemoryRequirementsInfo2,
    p_sparse_memory_requirement_count: *mut u32,
    p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
);
#[doc(alias = "vkCreateSamplerYcbcrConversion")]
pub type FNCreateSamplerYcbcrConversion = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const SamplerYcbcrConversionCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_ycbcr_conversion: *mut SamplerYcbcrConversion,
) -> VulkanResultCodes;
#[doc(alias = "vkDestroySamplerYcbcrConversion")]
pub type FNDestroySamplerYcbcrConversion = unsafe extern "system" fn(
    device: Device,
    ycbcr_conversion: SamplerYcbcrConversion,
    p_allocator: *const AllocationCallbacks,
);
#[doc(alias = "vkGetDeviceQueue2")]
pub type FNGetDeviceQueue2 =
    unsafe extern "system" fn(device: Device, p_queue_info: *const DeviceQueueInfo2, p_queue: *mut Queue);
#[doc(alias = "vkGetDescriptorSetLayoutSupport")]
pub type FNGetDescriptorSetLayoutSupport = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const DescriptorSetLayoutCreateInfo,
    p_support: *mut DescriptorSetLayoutSupport,
);
#[doc(alias = "vkCmdSetDeviceMask")]
pub type FNCmdSetDeviceMask = unsafe extern "system" fn(command_buffer: CommandBuffer, device_mask: u32);
#[doc(alias = "vkCmdDispatchBase")]
pub type FNCmdDispatchBase = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    base_group_x: u32,
    base_group_y: u32,
    base_group_z: u32,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
);
