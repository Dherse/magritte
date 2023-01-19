//!# [VK_VERSION_1_1](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html)
# ! [doc = include_str ! ("../../../doc/VK_VERSION_1_1.md")]
use crate::vulkan1_0::{
    AllocationCallbacks, BaseInStructure, BaseOutStructure, Bool32, Buffer, BufferCreateFlags, BufferUsageFlags,
    CommandBuffer, CommandPool, ComponentMapping, DescriptorSet, DescriptorSetLayout, DescriptorSetLayoutCreateInfo,
    DescriptorType, Device, DeviceMemory, DeviceQueueCreateFlags, DeviceSize, Filter, Format, FormatProperties, Image,
    ImageAspectFlagBits, ImageAspectFlags, ImageCreateFlags, ImageFormatProperties, ImageTiling, ImageType,
    ImageUsageFlags, Instance, MemoryRequirements, PhysicalDevice, PhysicalDeviceFeatures,
    PhysicalDeviceMemoryProperties, PhysicalDeviceProperties, PipelineBindPoint, PipelineLayout, Queue,
    QueueFamilyProperties, Rect2D, SampleCountFlagBits, ShaderStageFlags, SparseImageFormatProperties,
    SparseImageMemoryRequirements, StructureType, VulkanResultCodes, LUID_SIZE, MAX_DEVICE_GROUP_SIZE, UUID_SIZE,
};
///See [`PhysicalDeviceVariablePointersFeatures`]
#[doc(alias = "VkPhysicalDeviceVariablePointerFeatures")]
pub type PhysicalDeviceVariablePointerFeatures = PhysicalDeviceVariablePointersFeatures;
///See [`PhysicalDeviceShaderDrawParametersFeatures`]
#[doc(alias = "VkPhysicalDeviceShaderDrawParameterFeatures")]
pub type PhysicalDeviceShaderDrawParameterFeatures = PhysicalDeviceShaderDrawParametersFeatures;
///# [VkPhysicalDeviceFeatures2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFeatures2.html)
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceFeatures2.md")]
#[doc(alias = "VkPhysicalDeviceFeatures2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFeatures2 {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    # [doc = include_str ! ("../../../doc/VkPhysicalDeviceFeatures2$features.md")]
    features: PhysicalDeviceFeatures,
}
///# [VkPhysicalDeviceProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProperties2.html)
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceProperties2.md")]
#[doc(alias = "VkPhysicalDeviceProperties2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceProperties2 {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    # [doc = include_str ! ("../../../doc/VkPhysicalDeviceProperties2$properties.md")]
    properties: PhysicalDeviceProperties,
}
///# [VkFormatProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFormatProperties2.html)
# [doc = include_str ! ("../../../doc/VkFormatProperties2.md")]
#[doc(alias = "VkFormatProperties2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct FormatProperties2 {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "formatProperties")]
    format_properties: FormatProperties,
}
///# [VkImageFormatProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageFormatProperties2.html)
# [doc = include_str ! ("../../../doc/VkImageFormatProperties2.md")]
#[doc(alias = "VkImageFormatProperties2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageFormatProperties2 {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "imageFormatProperties")]
    image_format_properties: ImageFormatProperties,
}
///# [VkPhysicalDeviceImageFormatInfo2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageFormatInfo2.html)
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceImageFormatInfo2.md")]
#[doc(alias = "VkPhysicalDeviceImageFormatInfo2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceImageFormatInfo2 {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    # [doc = include_str ! ("../../../doc/VkPhysicalDeviceImageFormatInfo2$format.md")]
    format: Format,
    #[doc(alias = "type")]
    type_: ImageType,
    # [doc = include_str ! ("../../../doc/VkPhysicalDeviceImageFormatInfo2$tiling.md")]
    tiling: ImageTiling,
    # [doc = include_str ! ("../../../doc/VkPhysicalDeviceImageFormatInfo2$usage.md")]
    usage: ImageUsageFlags,
    # [doc = include_str ! ("../../../doc/VkPhysicalDeviceImageFormatInfo2$flags.md")]
    flags: ImageCreateFlags,
}
///# [VkQueueFamilyProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyProperties2.html)
# [doc = include_str ! ("../../../doc/VkQueueFamilyProperties2.md")]
#[doc(alias = "VkQueueFamilyProperties2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct QueueFamilyProperties2 {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "queueFamilyProperties")]
    queue_family_properties: QueueFamilyProperties,
}
///# [VkPhysicalDeviceMemoryProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMemoryProperties2.html)
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceMemoryProperties2.md")]
#[doc(alias = "VkPhysicalDeviceMemoryProperties2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMemoryProperties2 {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "memoryProperties")]
    memory_properties: PhysicalDeviceMemoryProperties,
}
///# [VkSparseImageFormatProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageFormatProperties2.html)
# [doc = include_str ! ("../../../doc/VkSparseImageFormatProperties2.md")]
#[doc(alias = "VkSparseImageFormatProperties2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SparseImageFormatProperties2 {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    # [doc = include_str ! ("../../../doc/VkSparseImageFormatProperties2$properties.md")]
    properties: SparseImageFormatProperties,
}
///# [VkPhysicalDeviceSparseImageFormatInfo2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSparseImageFormatInfo2.html)
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceSparseImageFormatInfo2.md")]
#[doc(alias = "VkPhysicalDeviceSparseImageFormatInfo2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSparseImageFormatInfo2 {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    # [doc = include_str ! ("../../../doc/VkPhysicalDeviceSparseImageFormatInfo2$format.md")]
    format: Format,
    #[doc(alias = "type")]
    type_: ImageType,
    # [doc = include_str ! ("../../../doc/VkPhysicalDeviceSparseImageFormatInfo2$samples.md")]
    samples: SampleCountFlagBits,
    # [doc = include_str ! ("../../../doc/VkPhysicalDeviceSparseImageFormatInfo2$usage.md")]
    usage: ImageUsageFlags,
    # [doc = include_str ! ("../../../doc/VkPhysicalDeviceSparseImageFormatInfo2$tiling.md")]
    tiling: ImageTiling,
}
///# [VkPhysicalDeviceVariablePointersFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVariablePointersFeatures.html)
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceVariablePointersFeatures.md")]
#[doc(alias = "VkPhysicalDeviceVariablePointersFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVariablePointersFeatures {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "variablePointersStorageBuffer")]
    variable_pointers_storage_buffer: Bool32,
    #[doc(alias = "variablePointers")]
    variable_pointers: Bool32,
}
///# [VkExternalMemoryProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryProperties.html)
# [doc = include_str ! ("../../../doc/VkExternalMemoryProperties.md")]
#[doc(alias = "VkExternalMemoryProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExternalMemoryProperties {
    #[doc(alias = "externalMemoryFeatures")]
    external_memory_features: ExternalMemoryFeatureFlags,
    #[doc(alias = "exportFromImportedHandleTypes")]
    export_from_imported_handle_types: ExternalMemoryHandleTypeFlags,
    #[doc(alias = "compatibleHandleTypes")]
    compatible_handle_types: ExternalMemoryHandleTypeFlags,
}
///# [VkPhysicalDeviceExternalImageFormatInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalImageFormatInfo.html)
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceExternalImageFormatInfo.md")]
#[doc(alias = "VkPhysicalDeviceExternalImageFormatInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceExternalImageFormatInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "handleType")]
    handle_type: ExternalMemoryHandleTypeFlagBits,
}
///# [VkExternalImageFormatProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalImageFormatProperties.html)
# [doc = include_str ! ("../../../doc/VkExternalImageFormatProperties.md")]
#[doc(alias = "VkExternalImageFormatProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExternalImageFormatProperties {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "externalMemoryProperties")]
    external_memory_properties: ExternalMemoryProperties,
}
///# [VkPhysicalDeviceExternalBufferInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalBufferInfo.html)
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceExternalBufferInfo.md")]
#[doc(alias = "VkPhysicalDeviceExternalBufferInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceExternalBufferInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    # [doc = include_str ! ("../../../doc/VkPhysicalDeviceExternalBufferInfo$flags.md")]
    flags: BufferCreateFlags,
    # [doc = include_str ! ("../../../doc/VkPhysicalDeviceExternalBufferInfo$usage.md")]
    usage: BufferUsageFlags,
    #[doc(alias = "handleType")]
    handle_type: ExternalMemoryHandleTypeFlagBits,
}
///# [VkExternalBufferProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalBufferProperties.html)
# [doc = include_str ! ("../../../doc/VkExternalBufferProperties.md")]
#[doc(alias = "VkExternalBufferProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExternalBufferProperties {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "externalMemoryProperties")]
    external_memory_properties: ExternalMemoryProperties,
}
///# [VkPhysicalDeviceIDProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceIDProperties.html)
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceIDProperties.md")]
#[doc(alias = "VkPhysicalDeviceIDProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceIdProperties {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "deviceUUID")]
    device_uuid: [u8; UUID_SIZE as usize],
    #[doc(alias = "driverUUID")]
    driver_uuid: [u8; UUID_SIZE as usize],
    #[doc(alias = "deviceLUID")]
    device_luid: [u8; LUID_SIZE as usize],
    #[doc(alias = "deviceNodeMask")]
    device_node_mask: u32,
    #[doc(alias = "deviceLUIDValid")]
    device_luid_valid: Bool32,
}
///# [VkExternalMemoryImageCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryImageCreateInfo.html)
# [doc = include_str ! ("../../../doc/VkExternalMemoryImageCreateInfo.md")]
#[doc(alias = "VkExternalMemoryImageCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExternalMemoryImageCreateInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "handleTypes")]
    handle_types: ExternalMemoryHandleTypeFlags,
}
///# [VkExternalMemoryBufferCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryBufferCreateInfo.html)
# [doc = include_str ! ("../../../doc/VkExternalMemoryBufferCreateInfo.md")]
#[doc(alias = "VkExternalMemoryBufferCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExternalMemoryBufferCreateInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "handleTypes")]
    handle_types: ExternalMemoryHandleTypeFlags,
}
///# [VkExportMemoryAllocateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportMemoryAllocateInfo.html)
# [doc = include_str ! ("../../../doc/VkExportMemoryAllocateInfo.md")]
#[doc(alias = "VkExportMemoryAllocateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExportMemoryAllocateInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "handleTypes")]
    handle_types: ExternalMemoryHandleTypeFlags,
}
///# [VkPhysicalDeviceExternalSemaphoreInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalSemaphoreInfo.html)
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceExternalSemaphoreInfo.md")]
#[doc(alias = "VkPhysicalDeviceExternalSemaphoreInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceExternalSemaphoreInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "handleType")]
    handle_type: ExternalSemaphoreHandleTypeFlagBits,
}
///# [VkExternalSemaphoreProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalSemaphoreProperties.html)
# [doc = include_str ! ("../../../doc/VkExternalSemaphoreProperties.md")]
#[doc(alias = "VkExternalSemaphoreProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExternalSemaphoreProperties {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "exportFromImportedHandleTypes")]
    export_from_imported_handle_types: ExternalSemaphoreHandleTypeFlags,
    #[doc(alias = "compatibleHandleTypes")]
    compatible_handle_types: ExternalSemaphoreHandleTypeFlags,
    #[doc(alias = "externalSemaphoreFeatures")]
    external_semaphore_features: ExternalSemaphoreFeatureFlags,
}
///# [VkExportSemaphoreCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportSemaphoreCreateInfo.html)
# [doc = include_str ! ("../../../doc/VkExportSemaphoreCreateInfo.md")]
#[doc(alias = "VkExportSemaphoreCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExportSemaphoreCreateInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "handleTypes")]
    handle_types: ExternalSemaphoreHandleTypeFlags,
}
///# [VkPhysicalDeviceExternalFenceInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalFenceInfo.html)
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceExternalFenceInfo.md")]
#[doc(alias = "VkPhysicalDeviceExternalFenceInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceExternalFenceInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "handleType")]
    handle_type: ExternalFenceHandleTypeFlagBits,
}
///# [VkExternalFenceProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalFenceProperties.html)
# [doc = include_str ! ("../../../doc/VkExternalFenceProperties.md")]
#[doc(alias = "VkExternalFenceProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExternalFenceProperties {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "exportFromImportedHandleTypes")]
    export_from_imported_handle_types: ExternalFenceHandleTypeFlags,
    #[doc(alias = "compatibleHandleTypes")]
    compatible_handle_types: ExternalFenceHandleTypeFlags,
    #[doc(alias = "externalFenceFeatures")]
    external_fence_features: ExternalFenceFeatureFlags,
}
///# [VkExportFenceCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportFenceCreateInfo.html)
# [doc = include_str ! ("../../../doc/VkExportFenceCreateInfo.md")]
#[doc(alias = "VkExportFenceCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExportFenceCreateInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "handleTypes")]
    handle_types: ExternalFenceHandleTypeFlags,
}
///# [VkPhysicalDeviceMultiviewFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMultiviewFeatures.html)
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceMultiviewFeatures.md")]
#[doc(alias = "VkPhysicalDeviceMultiviewFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMultiviewFeatures {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    # [doc = include_str ! ("../../../doc/VkPhysicalDeviceMultiviewFeatures$multiview.md")]
    multiview: Bool32,
    #[doc(alias = "multiviewGeometryShader")]
    multiview_geometry_shader: Bool32,
    #[doc(alias = "multiviewTessellationShader")]
    multiview_tessellation_shader: Bool32,
}
///# [VkPhysicalDeviceMultiviewProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMultiviewProperties.html)
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceMultiviewProperties.md")]
#[doc(alias = "VkPhysicalDeviceMultiviewProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMultiviewProperties {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "maxMultiviewViewCount")]
    max_multiview_view_count: u32,
    #[doc(alias = "maxMultiviewInstanceIndex")]
    max_multiview_instance_index: u32,
}
///# [VkRenderPassMultiviewCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassMultiviewCreateInfo.html)
# [doc = include_str ! ("../../../doc/VkRenderPassMultiviewCreateInfo.md")]
#[doc(alias = "VkRenderPassMultiviewCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RenderPassMultiviewCreateInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "subpassCount")]
    subpass_count: u32,
    #[doc(alias = "pViewMasks")]
    view_masks: *const u32,
    #[doc(alias = "dependencyCount")]
    dependency_count: u32,
    #[doc(alias = "pViewOffsets")]
    view_offsets: *const i32,
    #[doc(alias = "correlationMaskCount")]
    correlation_mask_count: u32,
    #[doc(alias = "pCorrelationMasks")]
    correlation_masks: *const u32,
}
///# [VkPhysicalDeviceGroupProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceGroupProperties.html)
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceGroupProperties.md")]
#[doc(alias = "VkPhysicalDeviceGroupProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceGroupProperties {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "physicalDeviceCount")]
    physical_device_count: u32,
    #[doc(alias = "physicalDevices")]
    physical_devices: [PhysicalDevice; MAX_DEVICE_GROUP_SIZE as usize],
    #[doc(alias = "subsetAllocation")]
    subset_allocation: Bool32,
}
///# [VkMemoryAllocateFlagsInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryAllocateFlagsInfo.html)
# [doc = include_str ! ("../../../doc/VkMemoryAllocateFlagsInfo.md")]
#[doc(alias = "VkMemoryAllocateFlagsInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryAllocateFlagsInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    # [doc = include_str ! ("../../../doc/VkMemoryAllocateFlagsInfo$flags.md")]
    flags: MemoryAllocateFlags,
    #[doc(alias = "deviceMask")]
    device_mask: u32,
}
///# [VkBindBufferMemoryInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindBufferMemoryInfo.html)
# [doc = include_str ! ("../../../doc/VkBindBufferMemoryInfo.md")]
#[doc(alias = "VkBindBufferMemoryInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BindBufferMemoryInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    # [doc = include_str ! ("../../../doc/VkBindBufferMemoryInfo$buffer.md")]
    buffer: Buffer,
    # [doc = include_str ! ("../../../doc/VkBindBufferMemoryInfo$memory.md")]
    memory: DeviceMemory,
    #[doc(alias = "memoryOffset")]
    memory_offset: DeviceSize,
}
///# [VkBindBufferMemoryDeviceGroupInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindBufferMemoryDeviceGroupInfo.html)
# [doc = include_str ! ("../../../doc/VkBindBufferMemoryDeviceGroupInfo.md")]
#[doc(alias = "VkBindBufferMemoryDeviceGroupInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BindBufferMemoryDeviceGroupInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "deviceIndexCount")]
    device_index_count: u32,
    #[doc(alias = "pDeviceIndices")]
    device_indices: *const u32,
}
///# [VkBindImageMemoryInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindImageMemoryInfo.html)
# [doc = include_str ! ("../../../doc/VkBindImageMemoryInfo.md")]
#[doc(alias = "VkBindImageMemoryInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BindImageMemoryInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    # [doc = include_str ! ("../../../doc/VkBindImageMemoryInfo$image.md")]
    image: Image,
    # [doc = include_str ! ("../../../doc/VkBindImageMemoryInfo$memory.md")]
    memory: DeviceMemory,
    #[doc(alias = "memoryOffset")]
    memory_offset: DeviceSize,
}
///# [VkBindImageMemoryDeviceGroupInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindImageMemoryDeviceGroupInfo.html)
# [doc = include_str ! ("../../../doc/VkBindImageMemoryDeviceGroupInfo.md")]
#[doc(alias = "VkBindImageMemoryDeviceGroupInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BindImageMemoryDeviceGroupInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "deviceIndexCount")]
    device_index_count: u32,
    #[doc(alias = "pDeviceIndices")]
    device_indices: *const u32,
    #[doc(alias = "splitInstanceBindRegionCount")]
    split_instance_bind_region_count: u32,
    #[doc(alias = "pSplitInstanceBindRegions")]
    split_instance_bind_regions: *const Rect2D,
}
///# [VkDeviceGroupRenderPassBeginInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupRenderPassBeginInfo.html)
# [doc = include_str ! ("../../../doc/VkDeviceGroupRenderPassBeginInfo.md")]
#[doc(alias = "VkDeviceGroupRenderPassBeginInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceGroupRenderPassBeginInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "deviceMask")]
    device_mask: u32,
    #[doc(alias = "deviceRenderAreaCount")]
    device_render_area_count: u32,
    #[doc(alias = "pDeviceRenderAreas")]
    device_render_areas: *const Rect2D,
}
///# [VkDeviceGroupCommandBufferBeginInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupCommandBufferBeginInfo.html)
# [doc = include_str ! ("../../../doc/VkDeviceGroupCommandBufferBeginInfo.md")]
#[doc(alias = "VkDeviceGroupCommandBufferBeginInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceGroupCommandBufferBeginInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "deviceMask")]
    device_mask: u32,
}
///# [VkDeviceGroupSubmitInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupSubmitInfo.html)
# [doc = include_str ! ("../../../doc/VkDeviceGroupSubmitInfo.md")]
#[doc(alias = "VkDeviceGroupSubmitInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceGroupSubmitInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "waitSemaphoreCount")]
    wait_semaphore_count: u32,
    #[doc(alias = "pWaitSemaphoreDeviceIndices")]
    wait_semaphore_device_indices: *const u32,
    #[doc(alias = "commandBufferCount")]
    command_buffer_count: u32,
    #[doc(alias = "pCommandBufferDeviceMasks")]
    command_buffer_device_masks: *const u32,
    #[doc(alias = "signalSemaphoreCount")]
    signal_semaphore_count: u32,
    #[doc(alias = "pSignalSemaphoreDeviceIndices")]
    signal_semaphore_device_indices: *const u32,
}
///# [VkDeviceGroupBindSparseInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupBindSparseInfo.html)
# [doc = include_str ! ("../../../doc/VkDeviceGroupBindSparseInfo.md")]
#[doc(alias = "VkDeviceGroupBindSparseInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceGroupBindSparseInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "resourceDeviceIndex")]
    resource_device_index: u32,
    #[doc(alias = "memoryDeviceIndex")]
    memory_device_index: u32,
}
///# [VkDeviceGroupDeviceCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupDeviceCreateInfo.html)
# [doc = include_str ! ("../../../doc/VkDeviceGroupDeviceCreateInfo.md")]
#[doc(alias = "VkDeviceGroupDeviceCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceGroupDeviceCreateInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "physicalDeviceCount")]
    physical_device_count: u32,
    #[doc(alias = "pPhysicalDevices")]
    physical_devices: *const PhysicalDevice,
}
///# [VkDescriptorUpdateTemplateEntry](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplateEntry.html)
# [doc = include_str ! ("../../../doc/VkDescriptorUpdateTemplateEntry.md")]
#[doc(alias = "VkDescriptorUpdateTemplateEntry")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DescriptorUpdateTemplateEntry {
    #[doc(alias = "dstBinding")]
    dst_binding: u32,
    #[doc(alias = "dstArrayElement")]
    dst_array_element: u32,
    #[doc(alias = "descriptorCount")]
    descriptor_count: u32,
    #[doc(alias = "descriptorType")]
    descriptor_type: DescriptorType,
    # [doc = include_str ! ("../../../doc/VkDescriptorUpdateTemplateEntry$offset.md")]
    offset: usize,
    # [doc = include_str ! ("../../../doc/VkDescriptorUpdateTemplateEntry$stride.md")]
    stride: usize,
}
///# [VkDescriptorUpdateTemplateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplateCreateInfo.html)
# [doc = include_str ! ("../../../doc/VkDescriptorUpdateTemplateCreateInfo.md")]
#[doc(alias = "VkDescriptorUpdateTemplateCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DescriptorUpdateTemplateCreateInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    # [doc = include_str ! ("../../../doc/VkDescriptorUpdateTemplateCreateInfo$flags.md")]
    flags: DescriptorUpdateTemplateCreateFlags,
    #[doc(alias = "descriptorUpdateEntryCount")]
    descriptor_update_entry_count: u32,
    #[doc(alias = "pDescriptorUpdateEntries")]
    descriptor_update_entries: *const DescriptorUpdateTemplateEntry,
    #[doc(alias = "templateType")]
    template_type: DescriptorUpdateTemplateType,
    #[doc(alias = "descriptorSetLayout")]
    descriptor_set_layout: DescriptorSetLayout,
    #[doc(alias = "pipelineBindPoint")]
    pipeline_bind_point: PipelineBindPoint,
    #[doc(alias = "pipelineLayout")]
    pipeline_layout: PipelineLayout,
    # [doc = include_str ! ("../../../doc/VkDescriptorUpdateTemplateCreateInfo$set.md")]
    set: u32,
}
///# [VkInputAttachmentAspectReference](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkInputAttachmentAspectReference.html)
# [doc = include_str ! ("../../../doc/VkInputAttachmentAspectReference.md")]
#[doc(alias = "VkInputAttachmentAspectReference")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct InputAttachmentAspectReference {
    # [doc = include_str ! ("../../../doc/VkInputAttachmentAspectReference$subpass.md")]
    subpass: u32,
    #[doc(alias = "inputAttachmentIndex")]
    input_attachment_index: u32,
    #[doc(alias = "aspectMask")]
    aspect_mask: ImageAspectFlags,
}
///# [VkRenderPassInputAttachmentAspectCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassInputAttachmentAspectCreateInfo.html)
# [doc = include_str ! ("../../../doc/VkRenderPassInputAttachmentAspectCreateInfo.md")]
#[doc(alias = "VkRenderPassInputAttachmentAspectCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RenderPassInputAttachmentAspectCreateInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "aspectReferenceCount")]
    aspect_reference_count: u32,
    #[doc(alias = "pAspectReferences")]
    aspect_references: *const InputAttachmentAspectReference,
}
///# [VkPhysicalDevice16BitStorageFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevice16BitStorageFeatures.html)
# [doc = include_str ! ("../../../doc/VkPhysicalDevice16BitStorageFeatures.md")]
#[doc(alias = "VkPhysicalDevice16BitStorageFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevice16BitStorageFeatures {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "storageBuffer16BitAccess")]
    storage_buffer16_bit_access: Bool32,
    #[doc(alias = "uniformAndStorageBuffer16BitAccess")]
    uniform_and_storage_buffer16_bit_access: Bool32,
    #[doc(alias = "storagePushConstant16")]
    storage_push_constant16: Bool32,
    #[doc(alias = "storageInputOutput16")]
    storage_input_output16: Bool32,
}
///# [VkPhysicalDeviceSubgroupProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSubgroupProperties.html)
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceSubgroupProperties.md")]
#[doc(alias = "VkPhysicalDeviceSubgroupProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSubgroupProperties {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "subgroupSize")]
    subgroup_size: u32,
    #[doc(alias = "supportedStages")]
    supported_stages: ShaderStageFlags,
    #[doc(alias = "supportedOperations")]
    supported_operations: SubgroupFeatureFlags,
    #[doc(alias = "quadOperationsInAllStages")]
    quad_operations_in_all_stages: Bool32,
}
///# [VkBufferMemoryRequirementsInfo2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferMemoryRequirementsInfo2.html)
# [doc = include_str ! ("../../../doc/VkBufferMemoryRequirementsInfo2.md")]
#[doc(alias = "VkBufferMemoryRequirementsInfo2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BufferMemoryRequirementsInfo2 {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    # [doc = include_str ! ("../../../doc/VkBufferMemoryRequirementsInfo2$buffer.md")]
    buffer: Buffer,
}
///# [VkImageMemoryRequirementsInfo2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageMemoryRequirementsInfo2.html)
# [doc = include_str ! ("../../../doc/VkImageMemoryRequirementsInfo2.md")]
#[doc(alias = "VkImageMemoryRequirementsInfo2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageMemoryRequirementsInfo2 {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    # [doc = include_str ! ("../../../doc/VkImageMemoryRequirementsInfo2$image.md")]
    image: Image,
}
///# [VkImageSparseMemoryRequirementsInfo2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageSparseMemoryRequirementsInfo2.html)
# [doc = include_str ! ("../../../doc/VkImageSparseMemoryRequirementsInfo2.md")]
#[doc(alias = "VkImageSparseMemoryRequirementsInfo2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageSparseMemoryRequirementsInfo2 {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    # [doc = include_str ! ("../../../doc/VkImageSparseMemoryRequirementsInfo2$image.md")]
    image: Image,
}
///# [VkMemoryRequirements2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryRequirements2.html)
# [doc = include_str ! ("../../../doc/VkMemoryRequirements2.md")]
#[doc(alias = "VkMemoryRequirements2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryRequirements2 {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "memoryRequirements")]
    memory_requirements: MemoryRequirements,
}
///# [VkSparseImageMemoryRequirements2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageMemoryRequirements2.html)
# [doc = include_str ! ("../../../doc/VkSparseImageMemoryRequirements2.md")]
#[doc(alias = "VkSparseImageMemoryRequirements2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SparseImageMemoryRequirements2 {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "memoryRequirements")]
    memory_requirements: SparseImageMemoryRequirements,
}
///# [VkPhysicalDevicePointClippingProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePointClippingProperties.html)
# [doc = include_str ! ("../../../doc/VkPhysicalDevicePointClippingProperties.md")]
#[doc(alias = "VkPhysicalDevicePointClippingProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePointClippingProperties {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "pointClippingBehavior")]
    point_clipping_behavior: PointClippingBehavior,
}
///# [VkMemoryDedicatedRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryDedicatedRequirements.html)
# [doc = include_str ! ("../../../doc/VkMemoryDedicatedRequirements.md")]
#[doc(alias = "VkMemoryDedicatedRequirements")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryDedicatedRequirements {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "prefersDedicatedAllocation")]
    prefers_dedicated_allocation: Bool32,
    #[doc(alias = "requiresDedicatedAllocation")]
    requires_dedicated_allocation: Bool32,
}
///# [VkMemoryDedicatedAllocateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryDedicatedAllocateInfo.html)
# [doc = include_str ! ("../../../doc/VkMemoryDedicatedAllocateInfo.md")]
#[doc(alias = "VkMemoryDedicatedAllocateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryDedicatedAllocateInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    # [doc = include_str ! ("../../../doc/VkMemoryDedicatedAllocateInfo$image.md")]
    image: Image,
    # [doc = include_str ! ("../../../doc/VkMemoryDedicatedAllocateInfo$buffer.md")]
    buffer: Buffer,
}
///# [VkImageViewUsageCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewUsageCreateInfo.html)
# [doc = include_str ! ("../../../doc/VkImageViewUsageCreateInfo.md")]
#[doc(alias = "VkImageViewUsageCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageViewUsageCreateInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    # [doc = include_str ! ("../../../doc/VkImageViewUsageCreateInfo$usage.md")]
    usage: ImageUsageFlags,
}
///# [VkPipelineTessellationDomainOriginStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineTessellationDomainOriginStateCreateInfo.html)
# [doc = include_str ! ("../../../doc/VkPipelineTessellationDomainOriginStateCreateInfo.md")]
#[doc(alias = "VkPipelineTessellationDomainOriginStateCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineTessellationDomainOriginStateCreateInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "domainOrigin")]
    domain_origin: TessellationDomainOrigin,
}
///# [VkSamplerYcbcrConversionInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrConversionInfo.html)
# [doc = include_str ! ("../../../doc/VkSamplerYcbcrConversionInfo.md")]
#[doc(alias = "VkSamplerYcbcrConversionInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SamplerYcbcrConversionInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    # [doc = include_str ! ("../../../doc/VkSamplerYcbcrConversionInfo$conversion.md")]
    conversion: SamplerYcbcrConversion,
}
///# [VkSamplerYcbcrConversionCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrConversionCreateInfo.html)
# [doc = include_str ! ("../../../doc/VkSamplerYcbcrConversionCreateInfo.md")]
#[doc(alias = "VkSamplerYcbcrConversionCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SamplerYcbcrConversionCreateInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    # [doc = include_str ! ("../../../doc/VkSamplerYcbcrConversionCreateInfo$format.md")]
    format: Format,
    #[doc(alias = "ycbcrModel")]
    ycbcr_model: SamplerYcbcrModelConversion,
    #[doc(alias = "ycbcrRange")]
    ycbcr_range: SamplerYcbcrRange,
    # [doc = include_str ! ("../../../doc/VkSamplerYcbcrConversionCreateInfo$components.md")]
    components: ComponentMapping,
    #[doc(alias = "xChromaOffset")]
    x_chroma_offset: ChromaLocation,
    #[doc(alias = "yChromaOffset")]
    y_chroma_offset: ChromaLocation,
    #[doc(alias = "chromaFilter")]
    chroma_filter: Filter,
    #[doc(alias = "forceExplicitReconstruction")]
    force_explicit_reconstruction: Bool32,
}
///# [VkBindImagePlaneMemoryInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindImagePlaneMemoryInfo.html)
# [doc = include_str ! ("../../../doc/VkBindImagePlaneMemoryInfo.md")]
#[doc(alias = "VkBindImagePlaneMemoryInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BindImagePlaneMemoryInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "planeAspect")]
    plane_aspect: ImageAspectFlagBits,
}
///# [VkImagePlaneMemoryRequirementsInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImagePlaneMemoryRequirementsInfo.html)
# [doc = include_str ! ("../../../doc/VkImagePlaneMemoryRequirementsInfo.md")]
#[doc(alias = "VkImagePlaneMemoryRequirementsInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImagePlaneMemoryRequirementsInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "planeAspect")]
    plane_aspect: ImageAspectFlagBits,
}
///# [VkPhysicalDeviceSamplerYcbcrConversionFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSamplerYcbcrConversionFeatures.html)
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceSamplerYcbcrConversionFeatures.md")]
#[doc(alias = "VkPhysicalDeviceSamplerYcbcrConversionFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSamplerYcbcrConversionFeatures {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "samplerYcbcrConversion")]
    sampler_ycbcr_conversion: Bool32,
}
///# [VkSamplerYcbcrConversionImageFormatProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrConversionImageFormatProperties.html)
# [doc = include_str ! ("../../../doc/VkSamplerYcbcrConversionImageFormatProperties.md")]
#[doc(alias = "VkSamplerYcbcrConversionImageFormatProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SamplerYcbcrConversionImageFormatProperties {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "combinedImageSamplerDescriptorCount")]
    combined_image_sampler_descriptor_count: u32,
}
///# [VkProtectedSubmitInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkProtectedSubmitInfo.html)
# [doc = include_str ! ("../../../doc/VkProtectedSubmitInfo.md")]
#[doc(alias = "VkProtectedSubmitInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ProtectedSubmitInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "protectedSubmit")]
    protected_submit: Bool32,
}
///# [VkPhysicalDeviceProtectedMemoryFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProtectedMemoryFeatures.html)
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceProtectedMemoryFeatures.md")]
#[doc(alias = "VkPhysicalDeviceProtectedMemoryFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceProtectedMemoryFeatures {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "protectedMemory")]
    protected_memory: Bool32,
}
///# [VkPhysicalDeviceProtectedMemoryProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProtectedMemoryProperties.html)
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceProtectedMemoryProperties.md")]
#[doc(alias = "VkPhysicalDeviceProtectedMemoryProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceProtectedMemoryProperties {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "protectedNoFault")]
    protected_no_fault: Bool32,
}
///# [VkDeviceQueueInfo2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueInfo2.html)
# [doc = include_str ! ("../../../doc/VkDeviceQueueInfo2.md")]
#[doc(alias = "VkDeviceQueueInfo2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceQueueInfo2 {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    # [doc = include_str ! ("../../../doc/VkDeviceQueueInfo2$flags.md")]
    flags: DeviceQueueCreateFlags,
    #[doc(alias = "queueFamilyIndex")]
    queue_family_index: u32,
    #[doc(alias = "queueIndex")]
    queue_index: u32,
}
///# [VkPhysicalDeviceMaintenance3Properties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMaintenance3Properties.html)
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceMaintenance3Properties.md")]
#[doc(alias = "VkPhysicalDeviceMaintenance3Properties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMaintenance3Properties {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "maxPerSetDescriptors")]
    max_per_set_descriptors: u32,
    #[doc(alias = "maxMemoryAllocationSize")]
    max_memory_allocation_size: DeviceSize,
}
///# [VkDescriptorSetLayoutSupport](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutSupport.html)
# [doc = include_str ! ("../../../doc/VkDescriptorSetLayoutSupport.md")]
#[doc(alias = "VkDescriptorSetLayoutSupport")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DescriptorSetLayoutSupport {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    # [doc = include_str ! ("../../../doc/VkDescriptorSetLayoutSupport$supported.md")]
    supported: Bool32,
}
///# [VkPhysicalDeviceShaderDrawParametersFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderDrawParametersFeatures.html)
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceShaderDrawParametersFeatures.md")]
#[doc(alias = "VkPhysicalDeviceShaderDrawParametersFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderDrawParametersFeatures {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderDrawParameters")]
    shader_draw_parameters: Bool32,
}
///# [VkDescriptorUpdateTemplate](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplate.html)
# [doc = include_str ! ("../../../doc/VkDescriptorUpdateTemplate.md")]
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkDescriptorUpdateTemplate")]
#[repr(transparent)]
pub struct DescriptorUpdateTemplate(u64);
impl DescriptorUpdateTemplate {
    pub const fn null() -> Self {
        Self(0)
    }
}
impl const Default for DescriptorUpdateTemplate {
    fn default() -> Self {
        Self::null()
    }
}
///# [VkSamplerYcbcrConversion](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrConversion.html)
# [doc = include_str ! ("../../../doc/VkSamplerYcbcrConversion.md")]
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkSamplerYcbcrConversion")]
#[repr(transparent)]
pub struct SamplerYcbcrConversion(u64);
impl SamplerYcbcrConversion {
    pub const fn null() -> Self {
        Self(0)
    }
}
impl const Default for SamplerYcbcrConversion {
    fn default() -> Self {
        Self::null()
    }
}
///# [VkSubgroupFeatureFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubgroupFeatureFlagBits.html)
# [doc = include_str ! ("../../../doc/VkSubgroupFeatureFlagBits.md")]
#[doc(alias = "VkSubgroupFeatureFlags")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SubgroupFeatureFlags(u32);
impl SubgroupFeatureFlags {
    #[doc(alias = "VK_SUBGROUP_FEATURE_BASIC_BIT")]
    pub const BASIC: Self = Self(1);
    #[doc(alias = "VK_SUBGROUP_FEATURE_VOTE_BIT")]
    pub const VOTE: Self = Self(2);
    #[doc(alias = "VK_SUBGROUP_FEATURE_ARITHMETIC_BIT")]
    pub const ARITHMETIC: Self = Self(4);
    #[doc(alias = "VK_SUBGROUP_FEATURE_BALLOT_BIT")]
    pub const BALLOT: Self = Self(8);
    #[doc(alias = "VK_SUBGROUP_FEATURE_SHUFFLE_BIT")]
    pub const SHUFFLE: Self = Self(16);
    #[doc(alias = "VK_SUBGROUP_FEATURE_SHUFFLE_RELATIVE_BIT")]
    pub const SHUFFLE_RELATIVE: Self = Self(32);
    #[doc(alias = "VK_SUBGROUP_FEATURE_CLUSTERED_BIT")]
    pub const CLUSTERED: Self = Self(64);
    #[doc(alias = "VK_SUBGROUP_FEATURE_QUAD_BIT")]
    pub const QUAD: Self = Self(128);
    #[doc(alias = "VK_SUBGROUP_FEATURE_PARTITIONED_BIT_NV")]
    #[cfg(feature = "VK_NV_shader_subgroup_partitioned")]
    pub const PARTITIONED_NV: Self = Self(256);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        {
            all |= Self::BASIC;
        }
        {
            all |= Self::VOTE;
        }
        {
            all |= Self::ARITHMETIC;
        }
        {
            all |= Self::BALLOT;
        }
        {
            all |= Self::SHUFFLE;
        }
        {
            all |= Self::SHUFFLE_RELATIVE;
        }
        {
            all |= Self::CLUSTERED;
        }
        {
            all |= Self::QUAD;
        }
        #[cfg(feature = "VK_NV_shader_subgroup_partitioned")]
        {
            all |= Self::PARTITIONED_NV;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for SubgroupFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for SubgroupFeatureFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for SubgroupFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for SubgroupFeatureFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for SubgroupFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for SubgroupFeatureFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for SubgroupFeatureFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for SubgroupFeatureFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for SubgroupFeatureFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<SubgroupFeatureFlags> for SubgroupFeatureFlags {
    fn extend<T: IntoIterator<Item = SubgroupFeatureFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<SubgroupFeatureFlags> for SubgroupFeatureFlags {
    fn from_iter<T: IntoIterator<Item = SubgroupFeatureFlags>>(iterator: T) -> SubgroupFeatureFlags {
        let mut out = Self::empty();
        <Self as Extend<SubgroupFeatureFlags>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for SubgroupFeatureFlags {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<SubgroupFeatureFlagBits> for SubgroupFeatureFlags {
    fn from(bit: SubgroupFeatureFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<SubgroupFeatureFlagBits> for SubgroupFeatureFlags {
    fn extend<T: IntoIterator<Item = SubgroupFeatureFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<SubgroupFeatureFlagBits> for SubgroupFeatureFlags {
    fn from_iter<T: IntoIterator<Item = SubgroupFeatureFlagBits>>(iterator: T) -> SubgroupFeatureFlags {
        let mut out = Self::empty();
        <Self as Extend<SubgroupFeatureFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for SubgroupFeatureFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(SubgroupFeatureFlags);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == SubgroupFeatureFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(SubgroupFeatureFlags::BASIC) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(BASIC))?;
                    }
                    if self.0.contains(SubgroupFeatureFlags::VOTE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(VOTE))?;
                    }
                    if self.0.contains(SubgroupFeatureFlags::ARITHMETIC) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ARITHMETIC))?;
                    }
                    if self.0.contains(SubgroupFeatureFlags::BALLOT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(BALLOT))?;
                    }
                    if self.0.contains(SubgroupFeatureFlags::SHUFFLE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(SHUFFLE))?;
                    }
                    if self.0.contains(SubgroupFeatureFlags::SHUFFLE_RELATIVE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(SHUFFLE_RELATIVE))?;
                    }
                    if self.0.contains(SubgroupFeatureFlags::CLUSTERED) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(CLUSTERED))?;
                    }
                    if self.0.contains(SubgroupFeatureFlags::QUAD) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(QUAD))?;
                    }
                    #[cfg(feature = "VK_NV_shader_subgroup_partitioned")]
                    if self.0.contains(SubgroupFeatureFlags::PARTITIONED_NV) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PARTITIONED_NV))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(SubgroupFeatureFlags))
            .field(&Flags(*self))
            .finish()
    }
}
#[doc(alias = "VkDescriptorUpdateTemplateCreateFlags")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DescriptorUpdateTemplateCreateFlags(u32);
impl DescriptorUpdateTemplateCreateFlags {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
///# [VkPeerMemoryFeatureFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPeerMemoryFeatureFlagBits.html)
# [doc = include_str ! ("../../../doc/VkPeerMemoryFeatureFlagBits.md")]
#[doc(alias = "VkPeerMemoryFeatureFlags")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PeerMemoryFeatureFlags(u32);
impl PeerMemoryFeatureFlags {
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT")]
    pub const COPY_SRC: Self = Self(1);
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_COPY_DST_BIT")]
    pub const COPY_DST: Self = Self(2);
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT")]
    pub const GENERIC_SRC: Self = Self(4);
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT")]
    pub const GENERIC_DST: Self = Self(8);
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT_KHR")]
    #[cfg(feature = "VK_KHR_device_group")]
    pub const COPY_SRC_KHR: Self = Self::COPY_SRC;
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_COPY_DST_BIT_KHR")]
    #[cfg(feature = "VK_KHR_device_group")]
    pub const COPY_DST_KHR: Self = Self::COPY_DST;
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT_KHR")]
    #[cfg(feature = "VK_KHR_device_group")]
    pub const GENERIC_SRC_KHR: Self = Self::GENERIC_SRC;
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT_KHR")]
    #[cfg(feature = "VK_KHR_device_group")]
    pub const GENERIC_DST_KHR: Self = Self::GENERIC_DST;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        {
            all |= Self::COPY_SRC;
        }
        {
            all |= Self::COPY_DST;
        }
        {
            all |= Self::GENERIC_SRC;
        }
        {
            all |= Self::GENERIC_DST;
        }
        #[cfg(feature = "VK_KHR_device_group")]
        {
            all |= Self::COPY_SRC_KHR;
        }
        #[cfg(feature = "VK_KHR_device_group")]
        {
            all |= Self::COPY_DST_KHR;
        }
        #[cfg(feature = "VK_KHR_device_group")]
        {
            all |= Self::GENERIC_SRC_KHR;
        }
        #[cfg(feature = "VK_KHR_device_group")]
        {
            all |= Self::GENERIC_DST_KHR;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for PeerMemoryFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for PeerMemoryFeatureFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for PeerMemoryFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for PeerMemoryFeatureFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for PeerMemoryFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for PeerMemoryFeatureFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for PeerMemoryFeatureFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for PeerMemoryFeatureFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for PeerMemoryFeatureFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<PeerMemoryFeatureFlags> for PeerMemoryFeatureFlags {
    fn extend<T: IntoIterator<Item = PeerMemoryFeatureFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<PeerMemoryFeatureFlags> for PeerMemoryFeatureFlags {
    fn from_iter<T: IntoIterator<Item = PeerMemoryFeatureFlags>>(iterator: T) -> PeerMemoryFeatureFlags {
        let mut out = Self::empty();
        <Self as Extend<PeerMemoryFeatureFlags>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for PeerMemoryFeatureFlags {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<PeerMemoryFeatureFlagBits> for PeerMemoryFeatureFlags {
    fn from(bit: PeerMemoryFeatureFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<PeerMemoryFeatureFlagBits> for PeerMemoryFeatureFlags {
    fn extend<T: IntoIterator<Item = PeerMemoryFeatureFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<PeerMemoryFeatureFlagBits> for PeerMemoryFeatureFlags {
    fn from_iter<T: IntoIterator<Item = PeerMemoryFeatureFlagBits>>(iterator: T) -> PeerMemoryFeatureFlags {
        let mut out = Self::empty();
        <Self as Extend<PeerMemoryFeatureFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for PeerMemoryFeatureFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(PeerMemoryFeatureFlags);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == PeerMemoryFeatureFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(PeerMemoryFeatureFlags::COPY_SRC) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(COPY_SRC))?;
                    }
                    if self.0.contains(PeerMemoryFeatureFlags::COPY_DST) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(COPY_DST))?;
                    }
                    if self.0.contains(PeerMemoryFeatureFlags::GENERIC_SRC) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(GENERIC_SRC))?;
                    }
                    if self.0.contains(PeerMemoryFeatureFlags::GENERIC_DST) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(GENERIC_DST))?;
                    }
                    #[cfg(feature = "VK_KHR_device_group")]
                    if self.0.contains(PeerMemoryFeatureFlags::COPY_SRC_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(COPY_SRC_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_device_group")]
                    if self.0.contains(PeerMemoryFeatureFlags::COPY_DST_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(COPY_DST_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_device_group")]
                    if self.0.contains(PeerMemoryFeatureFlags::GENERIC_SRC_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(GENERIC_SRC_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_device_group")]
                    if self.0.contains(PeerMemoryFeatureFlags::GENERIC_DST_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(GENERIC_DST_KHR))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(PeerMemoryFeatureFlags))
            .field(&Flags(*self))
            .finish()
    }
}
///# [VkMemoryAllocateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryAllocateFlagBits.html)
# [doc = include_str ! ("../../../doc/VkMemoryAllocateFlagBits.md")]
#[doc(alias = "VkMemoryAllocateFlags")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MemoryAllocateFlags(u32);
impl MemoryAllocateFlags {
    #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT")]
    pub const DEVICE_MASK: Self = Self(1);
    #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT")]
    #[cfg(feature = "VULKAN_1_2")]
    pub const DEVICE_ADDRESS: Self = Self(2);
    #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT")]
    #[cfg(feature = "VULKAN_1_2")]
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY: Self = Self(4);
    #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT_KHR")]
    #[cfg(feature = "VK_KHR_device_group")]
    pub const DEVICE_MASK_KHR: Self = Self::DEVICE_MASK;
    #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT_KHR")]
    #[cfg(feature = "VK_KHR_buffer_device_address")]
    pub const DEVICE_ADDRESS_KHR: Self = Self::DEVICE_ADDRESS;
    #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR")]
    #[cfg(feature = "VK_KHR_buffer_device_address")]
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR: Self = Self::DEVICE_ADDRESS_CAPTURE_REPLAY;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        {
            all |= Self::DEVICE_MASK;
        }
        #[cfg(feature = "VULKAN_1_2")]
        {
            all |= Self::DEVICE_ADDRESS;
        }
        #[cfg(feature = "VULKAN_1_2")]
        {
            all |= Self::DEVICE_ADDRESS_CAPTURE_REPLAY;
        }
        #[cfg(feature = "VK_KHR_device_group")]
        {
            all |= Self::DEVICE_MASK_KHR;
        }
        #[cfg(feature = "VK_KHR_buffer_device_address")]
        {
            all |= Self::DEVICE_ADDRESS_KHR;
        }
        #[cfg(feature = "VK_KHR_buffer_device_address")]
        {
            all |= Self::DEVICE_ADDRESS_CAPTURE_REPLAY_KHR;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for MemoryAllocateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for MemoryAllocateFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for MemoryAllocateFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for MemoryAllocateFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for MemoryAllocateFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for MemoryAllocateFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for MemoryAllocateFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for MemoryAllocateFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for MemoryAllocateFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<MemoryAllocateFlags> for MemoryAllocateFlags {
    fn extend<T: IntoIterator<Item = MemoryAllocateFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<MemoryAllocateFlags> for MemoryAllocateFlags {
    fn from_iter<T: IntoIterator<Item = MemoryAllocateFlags>>(iterator: T) -> MemoryAllocateFlags {
        let mut out = Self::empty();
        <Self as Extend<MemoryAllocateFlags>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for MemoryAllocateFlags {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<MemoryAllocateFlagBits> for MemoryAllocateFlags {
    fn from(bit: MemoryAllocateFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<MemoryAllocateFlagBits> for MemoryAllocateFlags {
    fn extend<T: IntoIterator<Item = MemoryAllocateFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<MemoryAllocateFlagBits> for MemoryAllocateFlags {
    fn from_iter<T: IntoIterator<Item = MemoryAllocateFlagBits>>(iterator: T) -> MemoryAllocateFlags {
        let mut out = Self::empty();
        <Self as Extend<MemoryAllocateFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for MemoryAllocateFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(MemoryAllocateFlags);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == MemoryAllocateFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(MemoryAllocateFlags::DEVICE_MASK) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DEVICE_MASK))?;
                    }
                    #[cfg(feature = "VULKAN_1_2")]
                    if self.0.contains(MemoryAllocateFlags::DEVICE_ADDRESS) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DEVICE_ADDRESS))?;
                    }
                    #[cfg(feature = "VULKAN_1_2")]
                    if self.0.contains(MemoryAllocateFlags::DEVICE_ADDRESS_CAPTURE_REPLAY) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DEVICE_ADDRESS_CAPTURE_REPLAY))?;
                    }
                    #[cfg(feature = "VK_KHR_device_group")]
                    if self.0.contains(MemoryAllocateFlags::DEVICE_MASK_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DEVICE_MASK_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_buffer_device_address")]
                    if self.0.contains(MemoryAllocateFlags::DEVICE_ADDRESS_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DEVICE_ADDRESS_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_buffer_device_address")]
                    if self.0.contains(MemoryAllocateFlags::DEVICE_ADDRESS_CAPTURE_REPLAY_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DEVICE_ADDRESS_CAPTURE_REPLAY_KHR))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(MemoryAllocateFlags))
            .field(&Flags(*self))
            .finish()
    }
}
#[doc(alias = "VkCommandPoolTrimFlags")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CommandPoolTrimFlags(u32);
impl CommandPoolTrimFlags {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
///# [VkExternalMemoryHandleTypeFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryHandleTypeFlagBits.html)
# [doc = include_str ! ("../../../doc/VkExternalMemoryHandleTypeFlagBits.md")]
#[doc(alias = "VkExternalMemoryHandleTypeFlags")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExternalMemoryHandleTypeFlags(u32);
impl ExternalMemoryHandleTypeFlags {
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT")]
    pub const OPAQUE_FD: Self = Self(1);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT")]
    pub const OPAQUE_WIN32: Self = Self(2);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT")]
    pub const OPAQUE_WIN32_KMT: Self = Self(4);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT")]
    pub const D3D11_TEXTURE: Self = Self(8);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT")]
    pub const D3D11_TEXTURE_KMT: Self = Self(16);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT")]
    pub const D3D12_HEAP: Self = Self(32);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT")]
    pub const D3D12_RESOURCE: Self = Self(64);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF_BIT_EXT")]
    #[cfg(feature = "VK_EXT_external_memory_dma_buf")]
    pub const DMA_BUF_EXT: Self = Self(512);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID")]
    #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
    pub const ANDROID_HARDWARE_BUFFER_ANDROID: Self = Self(1024);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT")]
    #[cfg(feature = "VK_EXT_external_memory_host")]
    pub const HOST_ALLOCATION_EXT: Self = Self(128);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT")]
    #[cfg(feature = "VK_EXT_external_memory_host")]
    pub const HOST_MAPPED_FOREIGN_MEMORY_EXT: Self = Self(256);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_ZIRCON_VMO_BIT_FUCHSIA")]
    #[cfg(feature = "VK_FUCHSIA_external_memory")]
    pub const ZIRCON_VMO_FUCHSIA: Self = Self(2048);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_RDMA_ADDRESS_BIT_NV")]
    #[cfg(feature = "VK_NV_external_memory_rdma")]
    pub const RDMA_ADDRESS_NV: Self = Self(4096);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const D3D11_TEXTURE_KHR: Self = Self::D3D11_TEXTURE;
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const D3D11_TEXTURE_KMT_KHR: Self = Self::D3D11_TEXTURE_KMT;
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const D3D12_HEAP_KHR: Self = Self::D3D12_HEAP;
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const D3D12_RESOURCE_KHR: Self = Self::D3D12_RESOURCE;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        {
            all |= Self::OPAQUE_FD;
        }
        {
            all |= Self::OPAQUE_WIN32;
        }
        {
            all |= Self::OPAQUE_WIN32_KMT;
        }
        {
            all |= Self::D3D11_TEXTURE;
        }
        {
            all |= Self::D3D11_TEXTURE_KMT;
        }
        {
            all |= Self::D3D12_HEAP;
        }
        {
            all |= Self::D3D12_RESOURCE;
        }
        #[cfg(feature = "VK_EXT_external_memory_dma_buf")]
        {
            all |= Self::DMA_BUF_EXT;
        }
        #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
        {
            all |= Self::ANDROID_HARDWARE_BUFFER_ANDROID;
        }
        #[cfg(feature = "VK_EXT_external_memory_host")]
        {
            all |= Self::HOST_ALLOCATION_EXT;
        }
        #[cfg(feature = "VK_EXT_external_memory_host")]
        {
            all |= Self::HOST_MAPPED_FOREIGN_MEMORY_EXT;
        }
        #[cfg(feature = "VK_FUCHSIA_external_memory")]
        {
            all |= Self::ZIRCON_VMO_FUCHSIA;
        }
        #[cfg(feature = "VK_NV_external_memory_rdma")]
        {
            all |= Self::RDMA_ADDRESS_NV;
        }
        #[cfg(feature = "VK_KHR_external_memory_capabilities")]
        {
            all |= Self::OPAQUE_FD_KHR;
        }
        #[cfg(feature = "VK_KHR_external_memory_capabilities")]
        {
            all |= Self::OPAQUE_WIN32_KHR;
        }
        #[cfg(feature = "VK_KHR_external_memory_capabilities")]
        {
            all |= Self::OPAQUE_WIN32_KMT_KHR;
        }
        #[cfg(feature = "VK_KHR_external_memory_capabilities")]
        {
            all |= Self::D3D11_TEXTURE_KHR;
        }
        #[cfg(feature = "VK_KHR_external_memory_capabilities")]
        {
            all |= Self::D3D11_TEXTURE_KMT_KHR;
        }
        #[cfg(feature = "VK_KHR_external_memory_capabilities")]
        {
            all |= Self::D3D12_HEAP_KHR;
        }
        #[cfg(feature = "VK_KHR_external_memory_capabilities")]
        {
            all |= Self::D3D12_RESOURCE_KHR;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for ExternalMemoryHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for ExternalMemoryHandleTypeFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for ExternalMemoryHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for ExternalMemoryHandleTypeFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for ExternalMemoryHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for ExternalMemoryHandleTypeFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for ExternalMemoryHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for ExternalMemoryHandleTypeFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for ExternalMemoryHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<ExternalMemoryHandleTypeFlags> for ExternalMemoryHandleTypeFlags {
    fn extend<T: IntoIterator<Item = ExternalMemoryHandleTypeFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<ExternalMemoryHandleTypeFlags> for ExternalMemoryHandleTypeFlags {
    fn from_iter<T: IntoIterator<Item = ExternalMemoryHandleTypeFlags>>(iterator: T) -> ExternalMemoryHandleTypeFlags {
        let mut out = Self::empty();
        <Self as Extend<ExternalMemoryHandleTypeFlags>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for ExternalMemoryHandleTypeFlags {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<ExternalMemoryHandleTypeFlagBits> for ExternalMemoryHandleTypeFlags {
    fn from(bit: ExternalMemoryHandleTypeFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<ExternalMemoryHandleTypeFlagBits> for ExternalMemoryHandleTypeFlags {
    fn extend<T: IntoIterator<Item = ExternalMemoryHandleTypeFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<ExternalMemoryHandleTypeFlagBits> for ExternalMemoryHandleTypeFlags {
    fn from_iter<T: IntoIterator<Item = ExternalMemoryHandleTypeFlagBits>>(
        iterator: T,
    ) -> ExternalMemoryHandleTypeFlags {
        let mut out = Self::empty();
        <Self as Extend<ExternalMemoryHandleTypeFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for ExternalMemoryHandleTypeFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(ExternalMemoryHandleTypeFlags);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == ExternalMemoryHandleTypeFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(ExternalMemoryHandleTypeFlags::OPAQUE_FD) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_FD))?;
                    }
                    if self.0.contains(ExternalMemoryHandleTypeFlags::OPAQUE_WIN32) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_WIN32))?;
                    }
                    if self.0.contains(ExternalMemoryHandleTypeFlags::OPAQUE_WIN32_KMT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_WIN32_KMT))?;
                    }
                    if self.0.contains(ExternalMemoryHandleTypeFlags::D3D11_TEXTURE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(D3D11_TEXTURE))?;
                    }
                    if self.0.contains(ExternalMemoryHandleTypeFlags::D3D11_TEXTURE_KMT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(D3D11_TEXTURE_KMT))?;
                    }
                    if self.0.contains(ExternalMemoryHandleTypeFlags::D3D12_HEAP) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(D3D12_HEAP))?;
                    }
                    if self.0.contains(ExternalMemoryHandleTypeFlags::D3D12_RESOURCE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(D3D12_RESOURCE))?;
                    }
                    #[cfg(feature = "VK_EXT_external_memory_dma_buf")]
                    if self.0.contains(ExternalMemoryHandleTypeFlags::DMA_BUF_EXT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DMA_BUF_EXT))?;
                    }
                    #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
                    if self
                        .0
                        .contains(ExternalMemoryHandleTypeFlags::ANDROID_HARDWARE_BUFFER_ANDROID)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ANDROID_HARDWARE_BUFFER_ANDROID))?;
                    }
                    #[cfg(feature = "VK_EXT_external_memory_host")]
                    if self.0.contains(ExternalMemoryHandleTypeFlags::HOST_ALLOCATION_EXT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(HOST_ALLOCATION_EXT))?;
                    }
                    #[cfg(feature = "VK_EXT_external_memory_host")]
                    if self
                        .0
                        .contains(ExternalMemoryHandleTypeFlags::HOST_MAPPED_FOREIGN_MEMORY_EXT)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(HOST_MAPPED_FOREIGN_MEMORY_EXT))?;
                    }
                    #[cfg(feature = "VK_FUCHSIA_external_memory")]
                    if self.0.contains(ExternalMemoryHandleTypeFlags::ZIRCON_VMO_FUCHSIA) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ZIRCON_VMO_FUCHSIA))?;
                    }
                    #[cfg(feature = "VK_NV_external_memory_rdma")]
                    if self.0.contains(ExternalMemoryHandleTypeFlags::RDMA_ADDRESS_NV) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(RDMA_ADDRESS_NV))?;
                    }
                    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
                    if self.0.contains(ExternalMemoryHandleTypeFlags::OPAQUE_FD_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_FD_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
                    if self.0.contains(ExternalMemoryHandleTypeFlags::OPAQUE_WIN32_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_WIN32_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
                    if self.0.contains(ExternalMemoryHandleTypeFlags::OPAQUE_WIN32_KMT_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_WIN32_KMT_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
                    if self.0.contains(ExternalMemoryHandleTypeFlags::D3D11_TEXTURE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(D3D11_TEXTURE_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
                    if self.0.contains(ExternalMemoryHandleTypeFlags::D3D11_TEXTURE_KMT_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(D3D11_TEXTURE_KMT_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
                    if self.0.contains(ExternalMemoryHandleTypeFlags::D3D12_HEAP_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(D3D12_HEAP_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
                    if self.0.contains(ExternalMemoryHandleTypeFlags::D3D12_RESOURCE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(D3D12_RESOURCE_KHR))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(ExternalMemoryHandleTypeFlags))
            .field(&Flags(*self))
            .finish()
    }
}
///# [VkExternalMemoryFeatureFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryFeatureFlagBits.html)
# [doc = include_str ! ("../../../doc/VkExternalMemoryFeatureFlagBits.md")]
#[doc(alias = "VkExternalMemoryFeatureFlags")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExternalMemoryFeatureFlags(u32);
impl ExternalMemoryFeatureFlags {
    #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT")]
    pub const DEDICATED_ONLY: Self = Self(1);
    #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT")]
    pub const EXPORTABLE: Self = Self(2);
    #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT")]
    pub const IMPORTABLE: Self = Self(4);
    #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const DEDICATED_ONLY_KHR: Self = Self::DEDICATED_ONLY;
    #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
    #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        {
            all |= Self::DEDICATED_ONLY;
        }
        {
            all |= Self::EXPORTABLE;
        }
        {
            all |= Self::IMPORTABLE;
        }
        #[cfg(feature = "VK_KHR_external_memory_capabilities")]
        {
            all |= Self::DEDICATED_ONLY_KHR;
        }
        #[cfg(feature = "VK_KHR_external_memory_capabilities")]
        {
            all |= Self::EXPORTABLE_KHR;
        }
        #[cfg(feature = "VK_KHR_external_memory_capabilities")]
        {
            all |= Self::IMPORTABLE_KHR;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for ExternalMemoryFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for ExternalMemoryFeatureFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for ExternalMemoryFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for ExternalMemoryFeatureFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for ExternalMemoryFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for ExternalMemoryFeatureFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for ExternalMemoryFeatureFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for ExternalMemoryFeatureFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for ExternalMemoryFeatureFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<ExternalMemoryFeatureFlags> for ExternalMemoryFeatureFlags {
    fn extend<T: IntoIterator<Item = ExternalMemoryFeatureFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<ExternalMemoryFeatureFlags> for ExternalMemoryFeatureFlags {
    fn from_iter<T: IntoIterator<Item = ExternalMemoryFeatureFlags>>(iterator: T) -> ExternalMemoryFeatureFlags {
        let mut out = Self::empty();
        <Self as Extend<ExternalMemoryFeatureFlags>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for ExternalMemoryFeatureFlags {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<ExternalMemoryFeatureFlagBits> for ExternalMemoryFeatureFlags {
    fn from(bit: ExternalMemoryFeatureFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<ExternalMemoryFeatureFlagBits> for ExternalMemoryFeatureFlags {
    fn extend<T: IntoIterator<Item = ExternalMemoryFeatureFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<ExternalMemoryFeatureFlagBits> for ExternalMemoryFeatureFlags {
    fn from_iter<T: IntoIterator<Item = ExternalMemoryFeatureFlagBits>>(iterator: T) -> ExternalMemoryFeatureFlags {
        let mut out = Self::empty();
        <Self as Extend<ExternalMemoryFeatureFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for ExternalMemoryFeatureFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(ExternalMemoryFeatureFlags);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == ExternalMemoryFeatureFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(ExternalMemoryFeatureFlags::DEDICATED_ONLY) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DEDICATED_ONLY))?;
                    }
                    if self.0.contains(ExternalMemoryFeatureFlags::EXPORTABLE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(EXPORTABLE))?;
                    }
                    if self.0.contains(ExternalMemoryFeatureFlags::IMPORTABLE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(IMPORTABLE))?;
                    }
                    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
                    if self.0.contains(ExternalMemoryFeatureFlags::DEDICATED_ONLY_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DEDICATED_ONLY_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
                    if self.0.contains(ExternalMemoryFeatureFlags::EXPORTABLE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(EXPORTABLE_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
                    if self.0.contains(ExternalMemoryFeatureFlags::IMPORTABLE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(IMPORTABLE_KHR))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(ExternalMemoryFeatureFlags))
            .field(&Flags(*self))
            .finish()
    }
}
///# [VkExternalSemaphoreHandleTypeFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalSemaphoreHandleTypeFlagBits.html)
# [doc = include_str ! ("../../../doc/VkExternalSemaphoreHandleTypeFlagBits.md")]
#[doc(alias = "VkExternalSemaphoreHandleTypeFlags")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExternalSemaphoreHandleTypeFlags(u32);
impl ExternalSemaphoreHandleTypeFlags {
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT")]
    pub const OPAQUE_FD: Self = Self(1);
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT")]
    pub const OPAQUE_WIN32: Self = Self(2);
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT")]
    pub const OPAQUE_WIN32_KMT: Self = Self(4);
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT")]
    pub const D3D12_FENCE: Self = Self(8);
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT")]
    pub const SYNC_FD: Self = Self(16);
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_ZIRCON_EVENT_BIT_FUCHSIA")]
    #[cfg(feature = "VK_FUCHSIA_external_semaphore")]
    pub const ZIRCON_EVENT_FUCHSIA: Self = Self(128);
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_FENCE_BIT")]
    pub const D3D11_FENCE: Self = Self::D3D12_FENCE;
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    pub const D3D12_FENCE_KHR: Self = Self::D3D12_FENCE;
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    pub const SYNC_FD_KHR: Self = Self::SYNC_FD;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        {
            all |= Self::OPAQUE_FD;
        }
        {
            all |= Self::OPAQUE_WIN32;
        }
        {
            all |= Self::OPAQUE_WIN32_KMT;
        }
        {
            all |= Self::D3D12_FENCE;
        }
        {
            all |= Self::SYNC_FD;
        }
        #[cfg(feature = "VK_FUCHSIA_external_semaphore")]
        {
            all |= Self::ZIRCON_EVENT_FUCHSIA;
        }
        {
            all |= Self::D3D11_FENCE;
        }
        #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
        {
            all |= Self::OPAQUE_FD_KHR;
        }
        #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
        {
            all |= Self::OPAQUE_WIN32_KHR;
        }
        #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
        {
            all |= Self::OPAQUE_WIN32_KMT_KHR;
        }
        #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
        {
            all |= Self::D3D12_FENCE_KHR;
        }
        #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
        {
            all |= Self::SYNC_FD_KHR;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for ExternalSemaphoreHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for ExternalSemaphoreHandleTypeFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for ExternalSemaphoreHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for ExternalSemaphoreHandleTypeFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for ExternalSemaphoreHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for ExternalSemaphoreHandleTypeFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for ExternalSemaphoreHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for ExternalSemaphoreHandleTypeFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for ExternalSemaphoreHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<ExternalSemaphoreHandleTypeFlags> for ExternalSemaphoreHandleTypeFlags {
    fn extend<T: IntoIterator<Item = ExternalSemaphoreHandleTypeFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<ExternalSemaphoreHandleTypeFlags> for ExternalSemaphoreHandleTypeFlags {
    fn from_iter<T: IntoIterator<Item = ExternalSemaphoreHandleTypeFlags>>(
        iterator: T,
    ) -> ExternalSemaphoreHandleTypeFlags {
        let mut out = Self::empty();
        <Self as Extend<ExternalSemaphoreHandleTypeFlags>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for ExternalSemaphoreHandleTypeFlags {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<ExternalSemaphoreHandleTypeFlagBits> for ExternalSemaphoreHandleTypeFlags {
    fn from(bit: ExternalSemaphoreHandleTypeFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<ExternalSemaphoreHandleTypeFlagBits> for ExternalSemaphoreHandleTypeFlags {
    fn extend<T: IntoIterator<Item = ExternalSemaphoreHandleTypeFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<ExternalSemaphoreHandleTypeFlagBits> for ExternalSemaphoreHandleTypeFlags {
    fn from_iter<T: IntoIterator<Item = ExternalSemaphoreHandleTypeFlagBits>>(
        iterator: T,
    ) -> ExternalSemaphoreHandleTypeFlags {
        let mut out = Self::empty();
        <Self as Extend<ExternalSemaphoreHandleTypeFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for ExternalSemaphoreHandleTypeFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(ExternalSemaphoreHandleTypeFlags);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == ExternalSemaphoreHandleTypeFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(ExternalSemaphoreHandleTypeFlags::OPAQUE_FD) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_FD))?;
                    }
                    if self.0.contains(ExternalSemaphoreHandleTypeFlags::OPAQUE_WIN32) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_WIN32))?;
                    }
                    if self.0.contains(ExternalSemaphoreHandleTypeFlags::OPAQUE_WIN32_KMT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_WIN32_KMT))?;
                    }
                    if self.0.contains(ExternalSemaphoreHandleTypeFlags::D3D12_FENCE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(D3D12_FENCE))?;
                    }
                    if self.0.contains(ExternalSemaphoreHandleTypeFlags::SYNC_FD) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(SYNC_FD))?;
                    }
                    #[cfg(feature = "VK_FUCHSIA_external_semaphore")]
                    if self.0.contains(ExternalSemaphoreHandleTypeFlags::ZIRCON_EVENT_FUCHSIA) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ZIRCON_EVENT_FUCHSIA))?;
                    }
                    if self.0.contains(ExternalSemaphoreHandleTypeFlags::D3D11_FENCE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(D3D11_FENCE))?;
                    }
                    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
                    if self.0.contains(ExternalSemaphoreHandleTypeFlags::OPAQUE_FD_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_FD_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
                    if self.0.contains(ExternalSemaphoreHandleTypeFlags::OPAQUE_WIN32_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_WIN32_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
                    if self.0.contains(ExternalSemaphoreHandleTypeFlags::OPAQUE_WIN32_KMT_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_WIN32_KMT_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
                    if self.0.contains(ExternalSemaphoreHandleTypeFlags::D3D12_FENCE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(D3D12_FENCE_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
                    if self.0.contains(ExternalSemaphoreHandleTypeFlags::SYNC_FD_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(SYNC_FD_KHR))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(ExternalSemaphoreHandleTypeFlags))
            .field(&Flags(*self))
            .finish()
    }
}
///# [VkExternalSemaphoreFeatureFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalSemaphoreFeatureFlagBits.html)
# [doc = include_str ! ("../../../doc/VkExternalSemaphoreFeatureFlagBits.md")]
#[doc(alias = "VkExternalSemaphoreFeatureFlags")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExternalSemaphoreFeatureFlags(u32);
impl ExternalSemaphoreFeatureFlags {
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT")]
    pub const EXPORTABLE: Self = Self(1);
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT")]
    pub const IMPORTABLE: Self = Self(2);
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        {
            all |= Self::EXPORTABLE;
        }
        {
            all |= Self::IMPORTABLE;
        }
        #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
        {
            all |= Self::EXPORTABLE_KHR;
        }
        #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
        {
            all |= Self::IMPORTABLE_KHR;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for ExternalSemaphoreFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for ExternalSemaphoreFeatureFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for ExternalSemaphoreFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for ExternalSemaphoreFeatureFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for ExternalSemaphoreFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for ExternalSemaphoreFeatureFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for ExternalSemaphoreFeatureFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for ExternalSemaphoreFeatureFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for ExternalSemaphoreFeatureFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<ExternalSemaphoreFeatureFlags> for ExternalSemaphoreFeatureFlags {
    fn extend<T: IntoIterator<Item = ExternalSemaphoreFeatureFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<ExternalSemaphoreFeatureFlags> for ExternalSemaphoreFeatureFlags {
    fn from_iter<T: IntoIterator<Item = ExternalSemaphoreFeatureFlags>>(iterator: T) -> ExternalSemaphoreFeatureFlags {
        let mut out = Self::empty();
        <Self as Extend<ExternalSemaphoreFeatureFlags>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for ExternalSemaphoreFeatureFlags {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<ExternalSemaphoreFeatureFlagBits> for ExternalSemaphoreFeatureFlags {
    fn from(bit: ExternalSemaphoreFeatureFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<ExternalSemaphoreFeatureFlagBits> for ExternalSemaphoreFeatureFlags {
    fn extend<T: IntoIterator<Item = ExternalSemaphoreFeatureFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<ExternalSemaphoreFeatureFlagBits> for ExternalSemaphoreFeatureFlags {
    fn from_iter<T: IntoIterator<Item = ExternalSemaphoreFeatureFlagBits>>(
        iterator: T,
    ) -> ExternalSemaphoreFeatureFlags {
        let mut out = Self::empty();
        <Self as Extend<ExternalSemaphoreFeatureFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for ExternalSemaphoreFeatureFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(ExternalSemaphoreFeatureFlags);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == ExternalSemaphoreFeatureFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(ExternalSemaphoreFeatureFlags::EXPORTABLE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(EXPORTABLE))?;
                    }
                    if self.0.contains(ExternalSemaphoreFeatureFlags::IMPORTABLE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(IMPORTABLE))?;
                    }
                    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
                    if self.0.contains(ExternalSemaphoreFeatureFlags::EXPORTABLE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(EXPORTABLE_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
                    if self.0.contains(ExternalSemaphoreFeatureFlags::IMPORTABLE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(IMPORTABLE_KHR))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(ExternalSemaphoreFeatureFlags))
            .field(&Flags(*self))
            .finish()
    }
}
///# [VkSemaphoreImportFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreImportFlagBits.html)
# [doc = include_str ! ("../../../doc/VkSemaphoreImportFlagBits.md")]
#[doc(alias = "VkSemaphoreImportFlags")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SemaphoreImportFlags(u32);
impl SemaphoreImportFlags {
    #[doc(alias = "VK_SEMAPHORE_IMPORT_TEMPORARY_BIT")]
    pub const TEMPORARY: Self = Self(1);
    #[doc(alias = "VK_SEMAPHORE_IMPORT_TEMPORARY_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_semaphore")]
    pub const TEMPORARY_KHR: Self = Self::TEMPORARY;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        {
            all |= Self::TEMPORARY;
        }
        #[cfg(feature = "VK_KHR_external_semaphore")]
        {
            all |= Self::TEMPORARY_KHR;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for SemaphoreImportFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for SemaphoreImportFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for SemaphoreImportFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for SemaphoreImportFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for SemaphoreImportFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for SemaphoreImportFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for SemaphoreImportFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for SemaphoreImportFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for SemaphoreImportFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<SemaphoreImportFlags> for SemaphoreImportFlags {
    fn extend<T: IntoIterator<Item = SemaphoreImportFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<SemaphoreImportFlags> for SemaphoreImportFlags {
    fn from_iter<T: IntoIterator<Item = SemaphoreImportFlags>>(iterator: T) -> SemaphoreImportFlags {
        let mut out = Self::empty();
        <Self as Extend<SemaphoreImportFlags>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for SemaphoreImportFlags {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<SemaphoreImportFlagBits> for SemaphoreImportFlags {
    fn from(bit: SemaphoreImportFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<SemaphoreImportFlagBits> for SemaphoreImportFlags {
    fn extend<T: IntoIterator<Item = SemaphoreImportFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<SemaphoreImportFlagBits> for SemaphoreImportFlags {
    fn from_iter<T: IntoIterator<Item = SemaphoreImportFlagBits>>(iterator: T) -> SemaphoreImportFlags {
        let mut out = Self::empty();
        <Self as Extend<SemaphoreImportFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for SemaphoreImportFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(SemaphoreImportFlags);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == SemaphoreImportFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(SemaphoreImportFlags::TEMPORARY) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(TEMPORARY))?;
                    }
                    #[cfg(feature = "VK_KHR_external_semaphore")]
                    if self.0.contains(SemaphoreImportFlags::TEMPORARY_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(TEMPORARY_KHR))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(SemaphoreImportFlags))
            .field(&Flags(*self))
            .finish()
    }
}
///# [VkExternalFenceHandleTypeFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalFenceHandleTypeFlagBits.html)
# [doc = include_str ! ("../../../doc/VkExternalFenceHandleTypeFlagBits.md")]
#[doc(alias = "VkExternalFenceHandleTypeFlags")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExternalFenceHandleTypeFlags(u32);
impl ExternalFenceHandleTypeFlags {
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT")]
    pub const OPAQUE_FD: Self = Self(1);
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT")]
    pub const OPAQUE_WIN32: Self = Self(2);
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT")]
    pub const OPAQUE_WIN32_KMT: Self = Self(4);
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT")]
    pub const SYNC_FD: Self = Self(8);
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    pub const SYNC_FD_KHR: Self = Self::SYNC_FD;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        {
            all |= Self::OPAQUE_FD;
        }
        {
            all |= Self::OPAQUE_WIN32;
        }
        {
            all |= Self::OPAQUE_WIN32_KMT;
        }
        {
            all |= Self::SYNC_FD;
        }
        #[cfg(feature = "VK_KHR_external_fence_capabilities")]
        {
            all |= Self::OPAQUE_FD_KHR;
        }
        #[cfg(feature = "VK_KHR_external_fence_capabilities")]
        {
            all |= Self::OPAQUE_WIN32_KHR;
        }
        #[cfg(feature = "VK_KHR_external_fence_capabilities")]
        {
            all |= Self::OPAQUE_WIN32_KMT_KHR;
        }
        #[cfg(feature = "VK_KHR_external_fence_capabilities")]
        {
            all |= Self::SYNC_FD_KHR;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for ExternalFenceHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for ExternalFenceHandleTypeFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for ExternalFenceHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for ExternalFenceHandleTypeFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for ExternalFenceHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for ExternalFenceHandleTypeFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for ExternalFenceHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for ExternalFenceHandleTypeFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for ExternalFenceHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<ExternalFenceHandleTypeFlags> for ExternalFenceHandleTypeFlags {
    fn extend<T: IntoIterator<Item = ExternalFenceHandleTypeFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<ExternalFenceHandleTypeFlags> for ExternalFenceHandleTypeFlags {
    fn from_iter<T: IntoIterator<Item = ExternalFenceHandleTypeFlags>>(iterator: T) -> ExternalFenceHandleTypeFlags {
        let mut out = Self::empty();
        <Self as Extend<ExternalFenceHandleTypeFlags>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for ExternalFenceHandleTypeFlags {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<ExternalFenceHandleTypeFlagBits> for ExternalFenceHandleTypeFlags {
    fn from(bit: ExternalFenceHandleTypeFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<ExternalFenceHandleTypeFlagBits> for ExternalFenceHandleTypeFlags {
    fn extend<T: IntoIterator<Item = ExternalFenceHandleTypeFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<ExternalFenceHandleTypeFlagBits> for ExternalFenceHandleTypeFlags {
    fn from_iter<T: IntoIterator<Item = ExternalFenceHandleTypeFlagBits>>(iterator: T) -> ExternalFenceHandleTypeFlags {
        let mut out = Self::empty();
        <Self as Extend<ExternalFenceHandleTypeFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for ExternalFenceHandleTypeFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(ExternalFenceHandleTypeFlags);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == ExternalFenceHandleTypeFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(ExternalFenceHandleTypeFlags::OPAQUE_FD) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_FD))?;
                    }
                    if self.0.contains(ExternalFenceHandleTypeFlags::OPAQUE_WIN32) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_WIN32))?;
                    }
                    if self.0.contains(ExternalFenceHandleTypeFlags::OPAQUE_WIN32_KMT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_WIN32_KMT))?;
                    }
                    if self.0.contains(ExternalFenceHandleTypeFlags::SYNC_FD) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(SYNC_FD))?;
                    }
                    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
                    if self.0.contains(ExternalFenceHandleTypeFlags::OPAQUE_FD_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_FD_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
                    if self.0.contains(ExternalFenceHandleTypeFlags::OPAQUE_WIN32_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_WIN32_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
                    if self.0.contains(ExternalFenceHandleTypeFlags::OPAQUE_WIN32_KMT_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_WIN32_KMT_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
                    if self.0.contains(ExternalFenceHandleTypeFlags::SYNC_FD_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(SYNC_FD_KHR))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(ExternalFenceHandleTypeFlags))
            .field(&Flags(*self))
            .finish()
    }
}
///# [VkExternalFenceFeatureFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalFenceFeatureFlagBits.html)
# [doc = include_str ! ("../../../doc/VkExternalFenceFeatureFlagBits.md")]
#[doc(alias = "VkExternalFenceFeatureFlags")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExternalFenceFeatureFlags(u32);
impl ExternalFenceFeatureFlags {
    #[doc(alias = "VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT")]
    pub const EXPORTABLE: Self = Self(1);
    #[doc(alias = "VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT")]
    pub const IMPORTABLE: Self = Self(2);
    #[doc(alias = "VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
    #[doc(alias = "VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        {
            all |= Self::EXPORTABLE;
        }
        {
            all |= Self::IMPORTABLE;
        }
        #[cfg(feature = "VK_KHR_external_fence_capabilities")]
        {
            all |= Self::EXPORTABLE_KHR;
        }
        #[cfg(feature = "VK_KHR_external_fence_capabilities")]
        {
            all |= Self::IMPORTABLE_KHR;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for ExternalFenceFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for ExternalFenceFeatureFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for ExternalFenceFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for ExternalFenceFeatureFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for ExternalFenceFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for ExternalFenceFeatureFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for ExternalFenceFeatureFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for ExternalFenceFeatureFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for ExternalFenceFeatureFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<ExternalFenceFeatureFlags> for ExternalFenceFeatureFlags {
    fn extend<T: IntoIterator<Item = ExternalFenceFeatureFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<ExternalFenceFeatureFlags> for ExternalFenceFeatureFlags {
    fn from_iter<T: IntoIterator<Item = ExternalFenceFeatureFlags>>(iterator: T) -> ExternalFenceFeatureFlags {
        let mut out = Self::empty();
        <Self as Extend<ExternalFenceFeatureFlags>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for ExternalFenceFeatureFlags {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<ExternalFenceFeatureFlagBits> for ExternalFenceFeatureFlags {
    fn from(bit: ExternalFenceFeatureFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<ExternalFenceFeatureFlagBits> for ExternalFenceFeatureFlags {
    fn extend<T: IntoIterator<Item = ExternalFenceFeatureFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<ExternalFenceFeatureFlagBits> for ExternalFenceFeatureFlags {
    fn from_iter<T: IntoIterator<Item = ExternalFenceFeatureFlagBits>>(iterator: T) -> ExternalFenceFeatureFlags {
        let mut out = Self::empty();
        <Self as Extend<ExternalFenceFeatureFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for ExternalFenceFeatureFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(ExternalFenceFeatureFlags);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == ExternalFenceFeatureFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(ExternalFenceFeatureFlags::EXPORTABLE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(EXPORTABLE))?;
                    }
                    if self.0.contains(ExternalFenceFeatureFlags::IMPORTABLE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(IMPORTABLE))?;
                    }
                    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
                    if self.0.contains(ExternalFenceFeatureFlags::EXPORTABLE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(EXPORTABLE_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
                    if self.0.contains(ExternalFenceFeatureFlags::IMPORTABLE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(IMPORTABLE_KHR))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(ExternalFenceFeatureFlags))
            .field(&Flags(*self))
            .finish()
    }
}
///# [VkFenceImportFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFenceImportFlagBits.html)
# [doc = include_str ! ("../../../doc/VkFenceImportFlagBits.md")]
#[doc(alias = "VkFenceImportFlags")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FenceImportFlags(u32);
impl FenceImportFlags {
    #[doc(alias = "VK_FENCE_IMPORT_TEMPORARY_BIT")]
    pub const TEMPORARY: Self = Self(1);
    #[doc(alias = "VK_FENCE_IMPORT_TEMPORARY_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_fence")]
    pub const TEMPORARY_KHR: Self = Self::TEMPORARY;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        {
            all |= Self::TEMPORARY;
        }
        #[cfg(feature = "VK_KHR_external_fence")]
        {
            all |= Self::TEMPORARY_KHR;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for FenceImportFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for FenceImportFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for FenceImportFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for FenceImportFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for FenceImportFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for FenceImportFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for FenceImportFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for FenceImportFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for FenceImportFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<FenceImportFlags> for FenceImportFlags {
    fn extend<T: IntoIterator<Item = FenceImportFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<FenceImportFlags> for FenceImportFlags {
    fn from_iter<T: IntoIterator<Item = FenceImportFlags>>(iterator: T) -> FenceImportFlags {
        let mut out = Self::empty();
        <Self as Extend<FenceImportFlags>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for FenceImportFlags {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<FenceImportFlagBits> for FenceImportFlags {
    fn from(bit: FenceImportFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<FenceImportFlagBits> for FenceImportFlags {
    fn extend<T: IntoIterator<Item = FenceImportFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<FenceImportFlagBits> for FenceImportFlags {
    fn from_iter<T: IntoIterator<Item = FenceImportFlagBits>>(iterator: T) -> FenceImportFlags {
        let mut out = Self::empty();
        <Self as Extend<FenceImportFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for FenceImportFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(FenceImportFlags);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == FenceImportFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(FenceImportFlags::TEMPORARY) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(TEMPORARY))?;
                    }
                    #[cfg(feature = "VK_KHR_external_fence")]
                    if self.0.contains(FenceImportFlags::TEMPORARY_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(TEMPORARY_KHR))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(FenceImportFlags))
            .field(&Flags(*self))
            .finish()
    }
}
///# [VkDeviceQueueCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueCreateFlagBits.html)
# [doc = include_str ! ("../../../doc/VkDeviceQueueCreateFlagBits.md")]
#[doc(alias = "VkDeviceQueueCreateFlagBits")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct DeviceQueueCreateFlagBits(u32);
impl DeviceQueueCreateFlagBits {
    #[doc(alias = "VK_DEVICE_QUEUE_CREATE_PROTECTED_BIT")]
    pub const PROTECTED: Self = Self(1);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        match bits {
            x if x == Self::PROTECTED.bits() => Some(Self(x)),
            #[cfg(feature = "VK_QCOM_extension_440")]
            x if x == Self::RESERVED1_QCOM.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
///# [VkSubgroupFeatureFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubgroupFeatureFlagBits.html)
# [doc = include_str ! ("../../../doc/VkSubgroupFeatureFlagBits.md")]
#[doc(alias = "VkSubgroupFeatureFlagBits")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct SubgroupFeatureFlagBits(u32);
impl SubgroupFeatureFlagBits {
    #[doc(alias = "VK_SUBGROUP_FEATURE_BASIC_BIT")]
    pub const BASIC: Self = Self(1);
    #[doc(alias = "VK_SUBGROUP_FEATURE_VOTE_BIT")]
    pub const VOTE: Self = Self(2);
    #[doc(alias = "VK_SUBGROUP_FEATURE_ARITHMETIC_BIT")]
    pub const ARITHMETIC: Self = Self(4);
    #[doc(alias = "VK_SUBGROUP_FEATURE_BALLOT_BIT")]
    pub const BALLOT: Self = Self(8);
    #[doc(alias = "VK_SUBGROUP_FEATURE_SHUFFLE_BIT")]
    pub const SHUFFLE: Self = Self(16);
    #[doc(alias = "VK_SUBGROUP_FEATURE_SHUFFLE_RELATIVE_BIT")]
    pub const SHUFFLE_RELATIVE: Self = Self(32);
    #[doc(alias = "VK_SUBGROUP_FEATURE_CLUSTERED_BIT")]
    pub const CLUSTERED: Self = Self(64);
    #[doc(alias = "VK_SUBGROUP_FEATURE_QUAD_BIT")]
    pub const QUAD: Self = Self(128);
    #[doc(alias = "VK_SUBGROUP_FEATURE_PARTITIONED_BIT_NV")]
    #[cfg(feature = "VK_NV_shader_subgroup_partitioned")]
    pub const PARTITIONED_NV: Self = Self(256);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        match bits {
            x if x == Self::BASIC.bits() => Some(Self(x)),
            x if x == Self::VOTE.bits() => Some(Self(x)),
            x if x == Self::ARITHMETIC.bits() => Some(Self(x)),
            x if x == Self::BALLOT.bits() => Some(Self(x)),
            x if x == Self::SHUFFLE.bits() => Some(Self(x)),
            x if x == Self::SHUFFLE_RELATIVE.bits() => Some(Self(x)),
            x if x == Self::CLUSTERED.bits() => Some(Self(x)),
            x if x == Self::QUAD.bits() => Some(Self(x)),
            #[cfg(feature = "VK_NV_shader_subgroup_partitioned")]
            x if x == Self::PARTITIONED_NV.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
///# [VkExternalMemoryHandleTypeFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryHandleTypeFlagBits.html)
# [doc = include_str ! ("../../../doc/VkExternalMemoryHandleTypeFlagBits.md")]
#[doc(alias = "VkExternalMemoryHandleTypeFlagBits")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ExternalMemoryHandleTypeFlagBits(u32);
impl ExternalMemoryHandleTypeFlagBits {
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT")]
    pub const OPAQUE_FD: Self = Self(1);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT")]
    pub const OPAQUE_WIN32: Self = Self(2);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT")]
    pub const OPAQUE_WIN32_KMT: Self = Self(4);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT")]
    pub const D3D11_TEXTURE: Self = Self(8);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT")]
    pub const D3D11_TEXTURE_KMT: Self = Self(16);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT")]
    pub const D3D12_HEAP: Self = Self(32);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT")]
    pub const D3D12_RESOURCE: Self = Self(64);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF_BIT_EXT")]
    #[cfg(feature = "VK_EXT_external_memory_dma_buf")]
    pub const DMA_BUF_EXT: Self = Self(512);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID")]
    #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
    pub const ANDROID_HARDWARE_BUFFER_ANDROID: Self = Self(1024);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT")]
    #[cfg(feature = "VK_EXT_external_memory_host")]
    pub const HOST_ALLOCATION_EXT: Self = Self(128);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT")]
    #[cfg(feature = "VK_EXT_external_memory_host")]
    pub const HOST_MAPPED_FOREIGN_MEMORY_EXT: Self = Self(256);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_ZIRCON_VMO_BIT_FUCHSIA")]
    #[cfg(feature = "VK_FUCHSIA_external_memory")]
    pub const ZIRCON_VMO_FUCHSIA: Self = Self(2048);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_RDMA_ADDRESS_BIT_NV")]
    #[cfg(feature = "VK_NV_external_memory_rdma")]
    pub const RDMA_ADDRESS_NV: Self = Self(4096);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const D3D11_TEXTURE_KHR: Self = Self::D3D11_TEXTURE;
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const D3D11_TEXTURE_KMT_KHR: Self = Self::D3D11_TEXTURE_KMT;
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const D3D12_HEAP_KHR: Self = Self::D3D12_HEAP;
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const D3D12_RESOURCE_KHR: Self = Self::D3D12_RESOURCE;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        match bits {
            x if x == Self::OPAQUE_FD.bits() => Some(Self(x)),
            x if x == Self::OPAQUE_WIN32.bits() => Some(Self(x)),
            x if x == Self::OPAQUE_WIN32_KMT.bits() => Some(Self(x)),
            x if x == Self::D3D11_TEXTURE.bits() => Some(Self(x)),
            x if x == Self::D3D11_TEXTURE_KMT.bits() => Some(Self(x)),
            x if x == Self::D3D12_HEAP.bits() => Some(Self(x)),
            x if x == Self::D3D12_RESOURCE.bits() => Some(Self(x)),
            #[cfg(feature = "VK_EXT_external_memory_dma_buf")]
            x if x == Self::DMA_BUF_EXT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
            x if x == Self::ANDROID_HARDWARE_BUFFER_ANDROID.bits() => Some(Self(x)),
            #[cfg(feature = "VK_EXT_external_memory_host")]
            x if x == Self::HOST_ALLOCATION_EXT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_EXT_external_memory_host")]
            x if x == Self::HOST_MAPPED_FOREIGN_MEMORY_EXT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_FUCHSIA_external_memory")]
            x if x == Self::ZIRCON_VMO_FUCHSIA.bits() => Some(Self(x)),
            #[cfg(feature = "VK_NV_external_memory_rdma")]
            x if x == Self::RDMA_ADDRESS_NV.bits() => Some(Self(x)),
            #[cfg(feature = "VK_NV_extension_375")]
            x if x == Self::RESERVED13_NV.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
///# [VkExternalMemoryFeatureFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryFeatureFlagBits.html)
# [doc = include_str ! ("../../../doc/VkExternalMemoryFeatureFlagBits.md")]
#[doc(alias = "VkExternalMemoryFeatureFlagBits")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ExternalMemoryFeatureFlagBits(u32);
impl ExternalMemoryFeatureFlagBits {
    #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT")]
    pub const DEDICATED_ONLY: Self = Self(1);
    #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT")]
    pub const EXPORTABLE: Self = Self(2);
    #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT")]
    pub const IMPORTABLE: Self = Self(4);
    #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const DEDICATED_ONLY_KHR: Self = Self::DEDICATED_ONLY;
    #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
    #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        match bits {
            x if x == Self::DEDICATED_ONLY.bits() => Some(Self(x)),
            x if x == Self::EXPORTABLE.bits() => Some(Self(x)),
            x if x == Self::IMPORTABLE.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
///# [VkExternalSemaphoreHandleTypeFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalSemaphoreHandleTypeFlagBits.html)
# [doc = include_str ! ("../../../doc/VkExternalSemaphoreHandleTypeFlagBits.md")]
#[doc(alias = "VkExternalSemaphoreHandleTypeFlagBits")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ExternalSemaphoreHandleTypeFlagBits(u32);
impl ExternalSemaphoreHandleTypeFlagBits {
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT")]
    pub const OPAQUE_FD: Self = Self(1);
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT")]
    pub const OPAQUE_WIN32: Self = Self(2);
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT")]
    pub const OPAQUE_WIN32_KMT: Self = Self(4);
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT")]
    pub const D3D12_FENCE: Self = Self(8);
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT")]
    pub const SYNC_FD: Self = Self(16);
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_ZIRCON_EVENT_BIT_FUCHSIA")]
    #[cfg(feature = "VK_FUCHSIA_external_semaphore")]
    pub const ZIRCON_EVENT_FUCHSIA: Self = Self(128);
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_FENCE_BIT")]
    pub const D3D11_FENCE: Self = Self::D3D12_FENCE;
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    pub const D3D12_FENCE_KHR: Self = Self::D3D12_FENCE;
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    pub const SYNC_FD_KHR: Self = Self::SYNC_FD;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        match bits {
            x if x == Self::OPAQUE_FD.bits() => Some(Self(x)),
            x if x == Self::OPAQUE_WIN32.bits() => Some(Self(x)),
            x if x == Self::OPAQUE_WIN32_KMT.bits() => Some(Self(x)),
            x if x == Self::D3D12_FENCE.bits() => Some(Self(x)),
            x if x == Self::SYNC_FD.bits() => Some(Self(x)),
            #[cfg(feature = "VK_FUCHSIA_external_semaphore")]
            x if x == Self::ZIRCON_EVENT_FUCHSIA.bits() => Some(Self(x)),
            #[cfg(feature = "VK_NV_extension_374")]
            x if x == Self::RESERVED5_NV.bits() => Some(Self(x)),
            #[cfg(feature = "VK_NV_extension_374")]
            x if x == Self::RESERVED6_NV.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
///# [VkExternalSemaphoreFeatureFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalSemaphoreFeatureFlagBits.html)
# [doc = include_str ! ("../../../doc/VkExternalSemaphoreFeatureFlagBits.md")]
#[doc(alias = "VkExternalSemaphoreFeatureFlagBits")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ExternalSemaphoreFeatureFlagBits(u32);
impl ExternalSemaphoreFeatureFlagBits {
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT")]
    pub const EXPORTABLE: Self = Self(1);
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT")]
    pub const IMPORTABLE: Self = Self(2);
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        match bits {
            x if x == Self::EXPORTABLE.bits() => Some(Self(x)),
            x if x == Self::IMPORTABLE.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
///# [VkSemaphoreImportFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreImportFlagBits.html)
# [doc = include_str ! ("../../../doc/VkSemaphoreImportFlagBits.md")]
#[doc(alias = "VkSemaphoreImportFlagBits")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct SemaphoreImportFlagBits(u32);
impl SemaphoreImportFlagBits {
    #[doc(alias = "VK_SEMAPHORE_IMPORT_TEMPORARY_BIT")]
    pub const TEMPORARY: Self = Self(1);
    #[doc(alias = "VK_SEMAPHORE_IMPORT_TEMPORARY_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_semaphore")]
    pub const TEMPORARY_KHR: Self = Self::TEMPORARY;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        match bits {
            x if x == Self::TEMPORARY.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
///# [VkExternalFenceHandleTypeFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalFenceHandleTypeFlagBits.html)
# [doc = include_str ! ("../../../doc/VkExternalFenceHandleTypeFlagBits.md")]
#[doc(alias = "VkExternalFenceHandleTypeFlagBits")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ExternalFenceHandleTypeFlagBits(u32);
impl ExternalFenceHandleTypeFlagBits {
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT")]
    pub const OPAQUE_FD: Self = Self(1);
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT")]
    pub const OPAQUE_WIN32: Self = Self(2);
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT")]
    pub const OPAQUE_WIN32_KMT: Self = Self(4);
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT")]
    pub const SYNC_FD: Self = Self(8);
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    pub const SYNC_FD_KHR: Self = Self::SYNC_FD;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        match bits {
            x if x == Self::OPAQUE_FD.bits() => Some(Self(x)),
            x if x == Self::OPAQUE_WIN32.bits() => Some(Self(x)),
            x if x == Self::OPAQUE_WIN32_KMT.bits() => Some(Self(x)),
            x if x == Self::SYNC_FD.bits() => Some(Self(x)),
            #[cfg(feature = "VK_NV_extension_374")]
            x if x == Self::RESERVED4_NV.bits() => Some(Self(x)),
            #[cfg(feature = "VK_NV_extension_374")]
            x if x == Self::RESERVED5_NV.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
///# [VkExternalFenceFeatureFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalFenceFeatureFlagBits.html)
# [doc = include_str ! ("../../../doc/VkExternalFenceFeatureFlagBits.md")]
#[doc(alias = "VkExternalFenceFeatureFlagBits")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ExternalFenceFeatureFlagBits(u32);
impl ExternalFenceFeatureFlagBits {
    #[doc(alias = "VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT")]
    pub const EXPORTABLE: Self = Self(1);
    #[doc(alias = "VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT")]
    pub const IMPORTABLE: Self = Self(2);
    #[doc(alias = "VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
    #[doc(alias = "VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        match bits {
            x if x == Self::EXPORTABLE.bits() => Some(Self(x)),
            x if x == Self::IMPORTABLE.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
///# [VkFenceImportFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFenceImportFlagBits.html)
# [doc = include_str ! ("../../../doc/VkFenceImportFlagBits.md")]
#[doc(alias = "VkFenceImportFlagBits")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct FenceImportFlagBits(u32);
impl FenceImportFlagBits {
    #[doc(alias = "VK_FENCE_IMPORT_TEMPORARY_BIT")]
    pub const TEMPORARY: Self = Self(1);
    #[doc(alias = "VK_FENCE_IMPORT_TEMPORARY_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_fence")]
    pub const TEMPORARY_KHR: Self = Self::TEMPORARY;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        match bits {
            x if x == Self::TEMPORARY.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
///# [VkPeerMemoryFeatureFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPeerMemoryFeatureFlagBits.html)
# [doc = include_str ! ("../../../doc/VkPeerMemoryFeatureFlagBits.md")]
#[doc(alias = "VkPeerMemoryFeatureFlagBits")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct PeerMemoryFeatureFlagBits(u32);
impl PeerMemoryFeatureFlagBits {
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT")]
    pub const COPY_SRC: Self = Self(1);
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_COPY_DST_BIT")]
    pub const COPY_DST: Self = Self(2);
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT")]
    pub const GENERIC_SRC: Self = Self(4);
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT")]
    pub const GENERIC_DST: Self = Self(8);
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT_KHR")]
    #[cfg(feature = "VK_KHR_device_group")]
    pub const COPY_SRC_KHR: Self = Self::COPY_SRC;
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_COPY_DST_BIT_KHR")]
    #[cfg(feature = "VK_KHR_device_group")]
    pub const COPY_DST_KHR: Self = Self::COPY_DST;
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT_KHR")]
    #[cfg(feature = "VK_KHR_device_group")]
    pub const GENERIC_SRC_KHR: Self = Self::GENERIC_SRC;
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT_KHR")]
    #[cfg(feature = "VK_KHR_device_group")]
    pub const GENERIC_DST_KHR: Self = Self::GENERIC_DST;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        match bits {
            x if x == Self::COPY_SRC.bits() => Some(Self(x)),
            x if x == Self::COPY_DST.bits() => Some(Self(x)),
            x if x == Self::GENERIC_SRC.bits() => Some(Self(x)),
            x if x == Self::GENERIC_DST.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
///# [VkMemoryAllocateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryAllocateFlagBits.html)
# [doc = include_str ! ("../../../doc/VkMemoryAllocateFlagBits.md")]
#[doc(alias = "VkMemoryAllocateFlagBits")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct MemoryAllocateFlagBits(u32);
impl MemoryAllocateFlagBits {
    #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT")]
    pub const DEVICE_MASK: Self = Self(1);
    #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT")]
    #[cfg(feature = "VULKAN_1_2")]
    pub const DEVICE_ADDRESS: Self = Self(2);
    #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT")]
    #[cfg(feature = "VULKAN_1_2")]
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY: Self = Self(4);
    #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT_KHR")]
    #[cfg(feature = "VK_KHR_device_group")]
    pub const DEVICE_MASK_KHR: Self = Self::DEVICE_MASK;
    #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT_KHR")]
    #[cfg(feature = "VK_KHR_buffer_device_address")]
    pub const DEVICE_ADDRESS_KHR: Self = Self::DEVICE_ADDRESS;
    #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR")]
    #[cfg(feature = "VK_KHR_buffer_device_address")]
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR: Self = Self::DEVICE_ADDRESS_CAPTURE_REPLAY;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        match bits {
            x if x == Self::DEVICE_MASK.bits() => Some(Self(x)),
            #[cfg(feature = "VULKAN_1_2")]
            x if x == Self::DEVICE_ADDRESS.bits() => Some(Self(x)),
            #[cfg(feature = "VULKAN_1_2")]
            x if x == Self::DEVICE_ADDRESS_CAPTURE_REPLAY.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
///# [VkDescriptorUpdateTemplateType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplateType.html)
# [doc = include_str ! ("../../../doc/VkDescriptorUpdateTemplateType.md")]
#[doc(alias = "VkDescriptorUpdateTemplateType")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct DescriptorUpdateTemplateType(i32);
impl DescriptorUpdateTemplateType {
    #[doc(alias = "VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET")]
    pub const DESCRIPTOR_SET: Self = Self(0);
    #[doc(alias = "VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR")]
    #[cfg(feature = "VK_KHR_descriptor_update_template")]
    pub const PUSH_DESCRIPTORS_KHR: Self = Self(1);
    #[doc(alias = "VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET_KHR")]
    #[cfg(feature = "VK_KHR_descriptor_update_template")]
    pub const DESCRIPTOR_SET_KHR: Self = Self::DESCRIPTOR_SET;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::DESCRIPTOR_SET.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_descriptor_update_template")]
            x if x == Self::PUSH_DESCRIPTORS_KHR.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
///# [VkPointClippingBehavior](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPointClippingBehavior.html)
# [doc = include_str ! ("../../../doc/VkPointClippingBehavior.md")]
#[doc(alias = "VkPointClippingBehavior")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct PointClippingBehavior(i32);
impl PointClippingBehavior {
    #[doc(alias = "VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES")]
    pub const ALL_CLIP_PLANES: Self = Self(0);
    #[doc(alias = "VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY")]
    pub const USER_CLIP_PLANES_ONLY: Self = Self(1);
    #[doc(alias = "VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES_KHR")]
    #[cfg(feature = "VK_KHR_maintenance2")]
    pub const ALL_CLIP_PLANES_KHR: Self = Self::ALL_CLIP_PLANES;
    #[doc(alias = "VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY_KHR")]
    #[cfg(feature = "VK_KHR_maintenance2")]
    pub const USER_CLIP_PLANES_ONLY_KHR: Self = Self::USER_CLIP_PLANES_ONLY;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::ALL_CLIP_PLANES.bits() => Some(Self(x)),
            x if x == Self::USER_CLIP_PLANES_ONLY.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
///# [VkTessellationDomainOrigin](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkTessellationDomainOrigin.html)
# [doc = include_str ! ("../../../doc/VkTessellationDomainOrigin.md")]
#[doc(alias = "VkTessellationDomainOrigin")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct TessellationDomainOrigin(i32);
impl TessellationDomainOrigin {
    #[doc(alias = "VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT")]
    pub const UPPER_LEFT: Self = Self(0);
    #[doc(alias = "VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT")]
    pub const LOWER_LEFT: Self = Self(1);
    #[doc(alias = "VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT_KHR")]
    #[cfg(feature = "VK_KHR_maintenance2")]
    pub const UPPER_LEFT_KHR: Self = Self::UPPER_LEFT;
    #[doc(alias = "VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT_KHR")]
    #[cfg(feature = "VK_KHR_maintenance2")]
    pub const LOWER_LEFT_KHR: Self = Self::LOWER_LEFT;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::UPPER_LEFT.bits() => Some(Self(x)),
            x if x == Self::LOWER_LEFT.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
///# [VkSamplerYcbcrModelConversion](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrModelConversion.html)
# [doc = include_str ! ("../../../doc/VkSamplerYcbcrModelConversion.md")]
#[doc(alias = "VkSamplerYcbcrModelConversion")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct SamplerYcbcrModelConversion(i32);
impl SamplerYcbcrModelConversion {
    #[doc(alias = "VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY")]
    pub const RGB_IDENTITY: Self = Self(0);
    #[doc(alias = "VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY")]
    pub const YCBCR_IDENTITY: Self = Self(1);
    #[doc(alias = "VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709")]
    pub const YCBCR709: Self = Self(2);
    #[doc(alias = "VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601")]
    pub const YCBCR601: Self = Self(3);
    #[doc(alias = "VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020")]
    pub const YCBCR2020: Self = Self(4);
    #[doc(alias = "VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY_KHR")]
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    pub const RGB_IDENTITY_KHR: Self = Self::RGB_IDENTITY;
    #[doc(alias = "VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY_KHR")]
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    pub const YCBCR_IDENTITY_KHR: Self = Self::YCBCR_IDENTITY;
    #[doc(alias = "VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709_KHR")]
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    pub const YCBCR709_KHR: Self = Self::YCBCR709;
    #[doc(alias = "VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601_KHR")]
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    pub const YCBCR601_KHR: Self = Self::YCBCR601;
    #[doc(alias = "VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020_KHR")]
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    pub const YCBCR2020_KHR: Self = Self::YCBCR2020;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::RGB_IDENTITY.bits() => Some(Self(x)),
            x if x == Self::YCBCR_IDENTITY.bits() => Some(Self(x)),
            x if x == Self::YCBCR709.bits() => Some(Self(x)),
            x if x == Self::YCBCR601.bits() => Some(Self(x)),
            x if x == Self::YCBCR2020.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
///# [VkSamplerYcbcrRange](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrRange.html)
# [doc = include_str ! ("../../../doc/VkSamplerYcbcrRange.md")]
#[doc(alias = "VkSamplerYcbcrRange")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct SamplerYcbcrRange(i32);
impl SamplerYcbcrRange {
    #[doc(alias = "VK_SAMPLER_YCBCR_RANGE_ITU_FULL")]
    pub const ITU_FULL: Self = Self(0);
    #[doc(alias = "VK_SAMPLER_YCBCR_RANGE_ITU_NARROW")]
    pub const ITU_NARROW: Self = Self(1);
    #[doc(alias = "VK_SAMPLER_YCBCR_RANGE_ITU_FULL_KHR")]
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    pub const ITU_FULL_KHR: Self = Self::ITU_FULL;
    #[doc(alias = "VK_SAMPLER_YCBCR_RANGE_ITU_NARROW_KHR")]
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    pub const ITU_NARROW_KHR: Self = Self::ITU_NARROW;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::ITU_FULL.bits() => Some(Self(x)),
            x if x == Self::ITU_NARROW.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
///# [VkChromaLocation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkChromaLocation.html)
# [doc = include_str ! ("../../../doc/VkChromaLocation.md")]
#[doc(alias = "VkChromaLocation")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ChromaLocation(i32);
impl ChromaLocation {
    #[doc(alias = "VK_CHROMA_LOCATION_COSITED_EVEN")]
    pub const COSITED_EVEN: Self = Self(0);
    #[doc(alias = "VK_CHROMA_LOCATION_MIDPOINT")]
    pub const MIDPOINT: Self = Self(1);
    #[doc(alias = "VK_CHROMA_LOCATION_COSITED_EVEN_KHR")]
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    pub const COSITED_EVEN_KHR: Self = Self::COSITED_EVEN;
    #[doc(alias = "VK_CHROMA_LOCATION_MIDPOINT_KHR")]
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    pub const MIDPOINT_KHR: Self = Self::MIDPOINT;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::COSITED_EVEN.bits() => Some(Self(x)),
            x if x == Self::MIDPOINT.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
///# [vkEnumerateInstanceVersion](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceVersion.html)
# [doc = include_str ! ("../../../doc/vkEnumerateInstanceVersion.md")]
#[doc(alias = "vkEnumerateInstanceVersion")]
pub type FNEnumerateInstanceVersion = unsafe extern "system" fn(p_api_version: *mut u32) -> VulkanResultCodes;
///# [vkGetPhysicalDeviceFeatures2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFeatures2.html)
# [doc = include_str ! ("../../../doc/vkGetPhysicalDeviceFeatures2.md")]
#[doc(alias = "vkGetPhysicalDeviceFeatures2")]
pub type FNGetPhysicalDeviceFeatures2 =
    unsafe extern "system" fn(physical_device: PhysicalDevice, p_features: *mut PhysicalDeviceFeatures2);
///# [vkGetPhysicalDeviceProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceProperties2.html)
# [doc = include_str ! ("../../../doc/vkGetPhysicalDeviceProperties2.md")]
#[doc(alias = "vkGetPhysicalDeviceProperties2")]
pub type FNGetPhysicalDeviceProperties2 =
    unsafe extern "system" fn(physical_device: PhysicalDevice, p_properties: *mut PhysicalDeviceProperties2);
///# [vkGetPhysicalDeviceFormatProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFormatProperties2.html)
# [doc = include_str ! ("../../../doc/vkGetPhysicalDeviceFormatProperties2.md")]
#[doc(alias = "vkGetPhysicalDeviceFormatProperties2")]
pub type FNGetPhysicalDeviceFormatProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    format: Format,
    p_format_properties: *mut FormatProperties2,
);
///# [vkGetPhysicalDeviceImageFormatProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties2.html)
# [doc = include_str ! ("../../../doc/vkGetPhysicalDeviceImageFormatProperties2.md")]
#[doc(alias = "vkGetPhysicalDeviceImageFormatProperties2")]
pub type FNGetPhysicalDeviceImageFormatProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_image_format_info: *const PhysicalDeviceImageFormatInfo2,
    p_image_format_properties: *mut ImageFormatProperties2,
) -> VulkanResultCodes;
///# [vkGetPhysicalDeviceQueueFamilyProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties2.html)
# [doc = include_str ! ("../../../doc/vkGetPhysicalDeviceQueueFamilyProperties2.md")]
#[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties2")]
pub type FNGetPhysicalDeviceQueueFamilyProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_queue_family_property_count: *mut u32,
    p_queue_family_properties: *mut QueueFamilyProperties2,
);
///# [vkGetPhysicalDeviceMemoryProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMemoryProperties2.html)
# [doc = include_str ! ("../../../doc/vkGetPhysicalDeviceMemoryProperties2.md")]
#[doc(alias = "vkGetPhysicalDeviceMemoryProperties2")]
pub type FNGetPhysicalDeviceMemoryProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_memory_properties: *mut PhysicalDeviceMemoryProperties2,
);
///# [vkGetPhysicalDeviceSparseImageFormatProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties2.html)
# [doc = include_str ! ("../../../doc/vkGetPhysicalDeviceSparseImageFormatProperties2.md")]
#[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties2")]
pub type FNGetPhysicalDeviceSparseImageFormatProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_format_info: *const PhysicalDeviceSparseImageFormatInfo2,
    p_property_count: *mut u32,
    p_properties: *mut SparseImageFormatProperties2,
);
///# [vkTrimCommandPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkTrimCommandPool.html)
# [doc = include_str ! ("../../../doc/vkTrimCommandPool.md")]
#[doc(alias = "vkTrimCommandPool")]
pub type FNTrimCommandPool =
    unsafe extern "system" fn(device: Device, command_pool: CommandPool, flags: CommandPoolTrimFlags);
///# [vkGetPhysicalDeviceExternalBufferProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalBufferProperties.html)
# [doc = include_str ! ("../../../doc/vkGetPhysicalDeviceExternalBufferProperties.md")]
#[doc(alias = "vkGetPhysicalDeviceExternalBufferProperties")]
pub type FNGetPhysicalDeviceExternalBufferProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_external_buffer_info: *const PhysicalDeviceExternalBufferInfo,
    p_external_buffer_properties: *mut ExternalBufferProperties,
);
///# [vkGetPhysicalDeviceExternalSemaphoreProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalSemaphoreProperties.html)
# [doc = include_str ! ("../../../doc/vkGetPhysicalDeviceExternalSemaphoreProperties.md")]
#[doc(alias = "vkGetPhysicalDeviceExternalSemaphoreProperties")]
pub type FNGetPhysicalDeviceExternalSemaphoreProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_external_semaphore_info: *const PhysicalDeviceExternalSemaphoreInfo,
    p_external_semaphore_properties: *mut ExternalSemaphoreProperties,
);
///# [vkGetPhysicalDeviceExternalFenceProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalFenceProperties.html)
# [doc = include_str ! ("../../../doc/vkGetPhysicalDeviceExternalFenceProperties.md")]
#[doc(alias = "vkGetPhysicalDeviceExternalFenceProperties")]
pub type FNGetPhysicalDeviceExternalFenceProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_external_fence_info: *const PhysicalDeviceExternalFenceInfo,
    p_external_fence_properties: *mut ExternalFenceProperties,
);
///# [vkEnumeratePhysicalDeviceGroups](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDeviceGroups.html)
# [doc = include_str ! ("../../../doc/vkEnumeratePhysicalDeviceGroups.md")]
#[doc(alias = "vkEnumeratePhysicalDeviceGroups")]
pub type FNEnumeratePhysicalDeviceGroups = unsafe extern "system" fn(
    instance: Instance,
    p_physical_device_group_count: *mut u32,
    p_physical_device_group_properties: *mut PhysicalDeviceGroupProperties,
) -> VulkanResultCodes;
///# [vkGetDeviceGroupPeerMemoryFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupPeerMemoryFeatures.html)
# [doc = include_str ! ("../../../doc/vkGetDeviceGroupPeerMemoryFeatures.md")]
#[doc(alias = "vkGetDeviceGroupPeerMemoryFeatures")]
pub type FNGetDeviceGroupPeerMemoryFeatures = unsafe extern "system" fn(
    device: Device,
    heap_index: u32,
    local_device_index: u32,
    remote_device_index: u32,
    p_peer_memory_features: *mut PeerMemoryFeatureFlags,
);
///# [vkBindBufferMemory2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBindBufferMemory2.html)
# [doc = include_str ! ("../../../doc/vkBindBufferMemory2.md")]
#[doc(alias = "vkBindBufferMemory2")]
pub type FNBindBufferMemory2 = unsafe extern "system" fn(
    device: Device,
    bind_info_count: u32,
    p_bind_infos: *const BindBufferMemoryInfo,
) -> VulkanResultCodes;
///# [vkBindImageMemory2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBindImageMemory2.html)
# [doc = include_str ! ("../../../doc/vkBindImageMemory2.md")]
#[doc(alias = "vkBindImageMemory2")]
pub type FNBindImageMemory2 = unsafe extern "system" fn(
    device: Device,
    bind_info_count: u32,
    p_bind_infos: *const BindImageMemoryInfo,
) -> VulkanResultCodes;
///# [vkCreateDescriptorUpdateTemplate](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorUpdateTemplate.html)
# [doc = include_str ! ("../../../doc/vkCreateDescriptorUpdateTemplate.md")]
#[doc(alias = "vkCreateDescriptorUpdateTemplate")]
pub type FNCreateDescriptorUpdateTemplate = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const DescriptorUpdateTemplateCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_descriptor_update_template: *mut DescriptorUpdateTemplate,
) -> VulkanResultCodes;
///# [vkDestroyDescriptorUpdateTemplate](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorUpdateTemplate.html)
# [doc = include_str ! ("../../../doc/vkDestroyDescriptorUpdateTemplate.md")]
#[doc(alias = "vkDestroyDescriptorUpdateTemplate")]
pub type FNDestroyDescriptorUpdateTemplate = unsafe extern "system" fn(
    device: Device,
    descriptor_update_template: DescriptorUpdateTemplate,
    p_allocator: *const AllocationCallbacks,
);
///# [vkUpdateDescriptorSetWithTemplate](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkUpdateDescriptorSetWithTemplate.html)
# [doc = include_str ! ("../../../doc/vkUpdateDescriptorSetWithTemplate.md")]
#[doc(alias = "vkUpdateDescriptorSetWithTemplate")]
pub type FNUpdateDescriptorSetWithTemplate = unsafe extern "system" fn(
    device: Device,
    descriptor_set: DescriptorSet,
    descriptor_update_template: DescriptorUpdateTemplate,
    p_data: *const std::ffi::c_void,
);
///# [vkGetBufferMemoryRequirements2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetBufferMemoryRequirements2.html)
# [doc = include_str ! ("../../../doc/vkGetBufferMemoryRequirements2.md")]
#[doc(alias = "vkGetBufferMemoryRequirements2")]
pub type FNGetBufferMemoryRequirements2 = unsafe extern "system" fn(
    device: Device,
    p_info: *const BufferMemoryRequirementsInfo2,
    p_memory_requirements: *mut MemoryRequirements2,
);
///# [vkGetImageMemoryRequirements2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageMemoryRequirements2.html)
# [doc = include_str ! ("../../../doc/vkGetImageMemoryRequirements2.md")]
#[doc(alias = "vkGetImageMemoryRequirements2")]
pub type FNGetImageMemoryRequirements2 = unsafe extern "system" fn(
    device: Device,
    p_info: *const ImageMemoryRequirementsInfo2,
    p_memory_requirements: *mut MemoryRequirements2,
);
///# [vkGetImageSparseMemoryRequirements2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageSparseMemoryRequirements2.html)
# [doc = include_str ! ("../../../doc/vkGetImageSparseMemoryRequirements2.md")]
#[doc(alias = "vkGetImageSparseMemoryRequirements2")]
pub type FNGetImageSparseMemoryRequirements2 = unsafe extern "system" fn(
    device: Device,
    p_info: *const ImageSparseMemoryRequirementsInfo2,
    p_sparse_memory_requirement_count: *mut u32,
    p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
);
///# [vkCreateSamplerYcbcrConversion](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateSamplerYcbcrConversion.html)
# [doc = include_str ! ("../../../doc/vkCreateSamplerYcbcrConversion.md")]
#[doc(alias = "vkCreateSamplerYcbcrConversion")]
pub type FNCreateSamplerYcbcrConversion = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const SamplerYcbcrConversionCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_ycbcr_conversion: *mut SamplerYcbcrConversion,
) -> VulkanResultCodes;
///# [vkDestroySamplerYcbcrConversion](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroySamplerYcbcrConversion.html)
# [doc = include_str ! ("../../../doc/vkDestroySamplerYcbcrConversion.md")]
#[doc(alias = "vkDestroySamplerYcbcrConversion")]
pub type FNDestroySamplerYcbcrConversion = unsafe extern "system" fn(
    device: Device,
    ycbcr_conversion: SamplerYcbcrConversion,
    p_allocator: *const AllocationCallbacks,
);
///# [vkGetDeviceQueue2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceQueue2.html)
# [doc = include_str ! ("../../../doc/vkGetDeviceQueue2.md")]
#[doc(alias = "vkGetDeviceQueue2")]
pub type FNGetDeviceQueue2 =
    unsafe extern "system" fn(device: Device, p_queue_info: *const DeviceQueueInfo2, p_queue: *mut Queue);
///# [vkGetDescriptorSetLayoutSupport](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutSupport.html)
# [doc = include_str ! ("../../../doc/vkGetDescriptorSetLayoutSupport.md")]
#[doc(alias = "vkGetDescriptorSetLayoutSupport")]
pub type FNGetDescriptorSetLayoutSupport = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const DescriptorSetLayoutCreateInfo,
    p_support: *mut DescriptorSetLayoutSupport,
);
///# [vkCmdSetDeviceMask](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDeviceMask.html)
# [doc = include_str ! ("../../../doc/vkCmdSetDeviceMask.md")]
#[doc(alias = "vkCmdSetDeviceMask")]
pub type FNCmdSetDeviceMask = unsafe extern "system" fn(command_buffer: CommandBuffer, device_mask: u32);
///# [vkCmdDispatchBase](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchBase.html)
# [doc = include_str ! ("../../../doc/vkCmdDispatchBase.md")]
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
