pub use crate::common::vulkan1_0::{
    AttachmentDescription, AttachmentReference, BufferCopy, BufferImageCopy, ClearDepthStencilValue, ClearRect,
    ComponentMapping, DescriptorPoolSize, DispatchIndirectCommand, DrawIndexedIndirectCommand, DrawIndirectCommand,
    Extent2D, Extent3D, FormatProperties, ImageBlit, ImageCopy, ImageFormatProperties, ImageResolve, ImageSubresource,
    ImageSubresourceLayers, ImageSubresourceRange, MemoryHeap, MemoryRequirements, MemoryType, Offset2D, Offset3D,
    PhysicalDeviceMemoryProperties, PipelineCacheHeaderVersionOne, PushConstantRange, QueueFamilyProperties, Rect2D,
    SparseImageFormatProperties, SparseImageMemoryRequirements, SpecializationMapEntry, StencilOpState,
    SubpassDependency, SubresourceLayout, VertexInputAttributeDescription, VertexInputBindingDescription, Viewport,
};
use std::ffi::c_char;
use uuid::Uuid;
#[doc(alias = "VkBaseOutStructure")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BaseOutStructure {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub next: *mut BaseOutStructure,
}
impl Default for BaseOutStructure {
    fn default() -> Self {
        Self {
            s_type: unsafe { std::mem::zeroed() },
            next: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkBaseInStructure")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BaseInStructure {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub next: *const BaseInStructure,
}
impl Default for BaseInStructure {
    fn default() -> Self {
        Self {
            s_type: unsafe { std::mem::zeroed() },
            next: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceProperties {
    #[doc(alias = "apiVersion")]
    pub api_version: u32,
    #[doc(alias = "driverVersion")]
    pub driver_version: u32,
    #[doc(alias = "vendorID")]
    pub vendor_id: u32,
    #[doc(alias = "deviceID")]
    pub device_id: u32,
    #[doc(alias = "deviceType")]
    pub device_type: PhysicalDeviceType,
    #[doc(alias = "deviceName")]
    pub device_name: [std::ffi::c_char; MAX_PHYSICAL_DEVICE_NAME_SIZE as usize],
    #[doc(alias = "pipelineCacheUUID")]
    pub pipeline_cache_uuid: Uuid,
    pub limits: PhysicalDeviceLimits,
    #[doc(alias = "sparseProperties")]
    pub sparse_properties: PhysicalDeviceSparseProperties,
}
impl Default for PhysicalDeviceProperties {
    fn default() -> Self {
        Self {
            api_version: unsafe { std::mem::zeroed() },
            driver_version: unsafe { std::mem::zeroed() },
            vendor_id: unsafe { std::mem::zeroed() },
            device_id: unsafe { std::mem::zeroed() },
            device_type: unsafe { std::mem::zeroed() },
            device_name: unsafe { std::mem::zeroed() },
            pipeline_cache_uuid: unsafe { std::mem::zeroed() },
            limits: unsafe { std::mem::zeroed() },
            sparse_properties: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkExtensionProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExtensionProperties {
    #[doc(alias = "extensionName")]
    pub extension_name: [std::ffi::c_char; MAX_EXTENSION_NAME_SIZE as usize],
    #[doc(alias = "specVersion")]
    pub spec_version: u32,
}
impl Default for ExtensionProperties {
    fn default() -> Self {
        Self {
            extension_name: unsafe { std::mem::zeroed() },
            spec_version: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkLayerProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct LayerProperties {
    #[doc(alias = "layerName")]
    pub layer_name: [std::ffi::c_char; MAX_EXTENSION_NAME_SIZE as usize],
    #[doc(alias = "specVersion")]
    pub spec_version: u32,
    #[doc(alias = "implementationVersion")]
    pub implementation_version: u32,
    pub description: [std::ffi::c_char; MAX_DESCRIPTION_SIZE as usize],
}
impl Default for LayerProperties {
    fn default() -> Self {
        Self {
            layer_name: unsafe { std::mem::zeroed() },
            spec_version: unsafe { std::mem::zeroed() },
            implementation_version: unsafe { std::mem::zeroed() },
            description: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkApplicationInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ApplicationInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "pApplicationName")]
    pub application_name: *const c_char,
    #[doc(alias = "applicationVersion")]
    pub application_version: u32,
    #[doc(alias = "pEngineName")]
    pub engine_name: *const c_char,
    #[doc(alias = "engineVersion")]
    pub engine_version: u32,
    #[doc(alias = "apiVersion")]
    pub api_version: u32,
}
impl Default for ApplicationInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::ApplicationInfo,
            p_next: unsafe { std::mem::zeroed() },
            application_name: unsafe { std::mem::zeroed() },
            application_version: unsafe { std::mem::zeroed() },
            engine_name: unsafe { std::mem::zeroed() },
            engine_version: unsafe { std::mem::zeroed() },
            api_version: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkAllocationCallbacks")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AllocationCallbacks {
    #[doc(alias = "pUserData")]
    pub user_data: *mut std::ffi::c_void,
    #[doc(alias = "pfnAllocation")]
    pub pfn_allocation: PFNAllocationFunction,
    #[doc(alias = "pfnReallocation")]
    pub pfn_reallocation: PFNReallocationFunction,
    #[doc(alias = "pfnFree")]
    pub pfn_free: PFNFreeFunction,
    #[doc(alias = "pfnInternalAllocation")]
    pub pfn_internal_allocation: PFNInternalAllocationNotification,
    #[doc(alias = "pfnInternalFree")]
    pub pfn_internal_free: PFNInternalFreeNotification,
}
impl Default for AllocationCallbacks {
    fn default() -> Self {
        Self {
            user_data: unsafe { std::mem::zeroed() },
            pfn_allocation: unsafe { std::mem::zeroed() },
            pfn_reallocation: unsafe { std::mem::zeroed() },
            pfn_free: unsafe { std::mem::zeroed() },
            pfn_internal_allocation: unsafe { std::mem::zeroed() },
            pfn_internal_free: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDeviceQueueCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceQueueCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: DeviceQueueCreateFlags,
    #[doc(alias = "queueFamilyIndex")]
    pub queue_family_index: u32,
    #[doc(alias = "queueCount")]
    pub queue_count: u32,
    #[doc(alias = "pQueuePriorities")]
    pub queue_priorities: *const f32,
}
impl Default for DeviceQueueCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::DeviceQueueCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            queue_family_index: unsafe { std::mem::zeroed() },
            queue_count: unsafe { std::mem::zeroed() },
            queue_priorities: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDeviceCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: DeviceCreateFlags,
    #[doc(alias = "queueCreateInfoCount")]
    pub queue_create_info_count: u32,
    #[doc(alias = "pQueueCreateInfos")]
    pub queue_create_infos: *const DeviceQueueCreateInfo,
    #[doc(alias = "enabledLayerCount")]
    pub enabled_layer_count: u32,
    #[doc(alias = "ppEnabledLayerNames")]
    pub pp_enabled_layer_names: *const *const c_char,
    #[doc(alias = "enabledExtensionCount")]
    pub enabled_extension_count: u32,
    #[doc(alias = "ppEnabledExtensionNames")]
    pub pp_enabled_extension_names: *const *const c_char,
    #[doc(alias = "pEnabledFeatures")]
    pub enabled_features: *const PhysicalDeviceFeatures,
}
impl Default for DeviceCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::DeviceCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            queue_create_info_count: unsafe { std::mem::zeroed() },
            queue_create_infos: unsafe { std::mem::zeroed() },
            enabled_layer_count: unsafe { std::mem::zeroed() },
            pp_enabled_layer_names: unsafe { std::mem::zeroed() },
            enabled_extension_count: unsafe { std::mem::zeroed() },
            pp_enabled_extension_names: unsafe { std::mem::zeroed() },
            enabled_features: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkInstanceCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct InstanceCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: InstanceCreateFlags,
    #[doc(alias = "pApplicationInfo")]
    pub application_info: *const ApplicationInfo,
    #[doc(alias = "enabledLayerCount")]
    pub enabled_layer_count: u32,
    #[doc(alias = "ppEnabledLayerNames")]
    pub pp_enabled_layer_names: *const *const c_char,
    #[doc(alias = "enabledExtensionCount")]
    pub enabled_extension_count: u32,
    #[doc(alias = "ppEnabledExtensionNames")]
    pub pp_enabled_extension_names: *const *const c_char,
}
impl Default for InstanceCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::InstanceCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            application_info: unsafe { std::mem::zeroed() },
            enabled_layer_count: unsafe { std::mem::zeroed() },
            pp_enabled_layer_names: unsafe { std::mem::zeroed() },
            enabled_extension_count: unsafe { std::mem::zeroed() },
            pp_enabled_extension_names: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkMemoryAllocateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryAllocateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "allocationSize")]
    pub allocation_size: DeviceSize,
    #[doc(alias = "memoryTypeIndex")]
    pub memory_type_index: u32,
}
impl Default for MemoryAllocateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::MemoryAllocateInfo,
            p_next: unsafe { std::mem::zeroed() },
            allocation_size: unsafe { std::mem::zeroed() },
            memory_type_index: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkMappedMemoryRange")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MappedMemoryRange {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub memory: DeviceMemory,
    pub offset: DeviceSize,
    pub size: DeviceSize,
}
impl Default for MappedMemoryRange {
    fn default() -> Self {
        Self {
            s_type: StructureType::MappedMemoryRange,
            p_next: unsafe { std::mem::zeroed() },
            memory: unsafe { std::mem::zeroed() },
            offset: unsafe { std::mem::zeroed() },
            size: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDescriptorBufferInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DescriptorBufferInfo {
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub range: DeviceSize,
}
impl Default for DescriptorBufferInfo {
    fn default() -> Self {
        Self {
            buffer: unsafe { std::mem::zeroed() },
            offset: unsafe { std::mem::zeroed() },
            range: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDescriptorImageInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DescriptorImageInfo {
    pub sampler: Sampler,
    #[doc(alias = "imageView")]
    pub image_view: ImageView,
    #[doc(alias = "imageLayout")]
    pub image_layout: ImageLayout,
}
impl Default for DescriptorImageInfo {
    fn default() -> Self {
        Self {
            sampler: unsafe { std::mem::zeroed() },
            image_view: unsafe { std::mem::zeroed() },
            image_layout: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkWriteDescriptorSet")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct WriteDescriptorSet {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "dstSet")]
    pub dst_set: DescriptorSet,
    #[doc(alias = "dstBinding")]
    pub dst_binding: u32,
    #[doc(alias = "dstArrayElement")]
    pub dst_array_element: u32,
    #[doc(alias = "descriptorCount")]
    pub descriptor_count: u32,
    #[doc(alias = "descriptorType")]
    pub descriptor_type: DescriptorType,
    #[doc(alias = "pImageInfo")]
    pub image_info: *const DescriptorImageInfo,
    #[doc(alias = "pBufferInfo")]
    pub buffer_info: *const DescriptorBufferInfo,
    #[doc(alias = "pTexelBufferView")]
    pub texel_buffer_view: *const BufferView,
}
impl Default for WriteDescriptorSet {
    fn default() -> Self {
        Self {
            s_type: StructureType::WriteDescriptorSet,
            p_next: unsafe { std::mem::zeroed() },
            dst_set: unsafe { std::mem::zeroed() },
            dst_binding: unsafe { std::mem::zeroed() },
            dst_array_element: unsafe { std::mem::zeroed() },
            descriptor_count: unsafe { std::mem::zeroed() },
            descriptor_type: unsafe { std::mem::zeroed() },
            image_info: unsafe { std::mem::zeroed() },
            buffer_info: unsafe { std::mem::zeroed() },
            texel_buffer_view: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkCopyDescriptorSet")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CopyDescriptorSet {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "srcSet")]
    pub src_set: DescriptorSet,
    #[doc(alias = "srcBinding")]
    pub src_binding: u32,
    #[doc(alias = "srcArrayElement")]
    pub src_array_element: u32,
    #[doc(alias = "dstSet")]
    pub dst_set: DescriptorSet,
    #[doc(alias = "dstBinding")]
    pub dst_binding: u32,
    #[doc(alias = "dstArrayElement")]
    pub dst_array_element: u32,
    #[doc(alias = "descriptorCount")]
    pub descriptor_count: u32,
}
impl Default for CopyDescriptorSet {
    fn default() -> Self {
        Self {
            s_type: StructureType::CopyDescriptorSet,
            p_next: unsafe { std::mem::zeroed() },
            src_set: unsafe { std::mem::zeroed() },
            src_binding: unsafe { std::mem::zeroed() },
            src_array_element: unsafe { std::mem::zeroed() },
            dst_set: unsafe { std::mem::zeroed() },
            dst_binding: unsafe { std::mem::zeroed() },
            dst_array_element: unsafe { std::mem::zeroed() },
            descriptor_count: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkBufferCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BufferCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: BufferCreateFlags,
    pub size: DeviceSize,
    pub usage: BufferUsageFlags,
    #[doc(alias = "sharingMode")]
    pub sharing_mode: SharingMode,
    #[doc(alias = "queueFamilyIndexCount")]
    pub queue_family_index_count: u32,
    #[doc(alias = "pQueueFamilyIndices")]
    pub queue_family_indices: *const u32,
}
impl Default for BufferCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::BufferCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            size: unsafe { std::mem::zeroed() },
            usage: unsafe { std::mem::zeroed() },
            sharing_mode: unsafe { std::mem::zeroed() },
            queue_family_index_count: unsafe { std::mem::zeroed() },
            queue_family_indices: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkBufferViewCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BufferViewCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: BufferViewCreateFlags,
    pub buffer: Buffer,
    pub format: Format,
    pub offset: DeviceSize,
    pub range: DeviceSize,
}
impl Default for BufferViewCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::BufferViewCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            buffer: unsafe { std::mem::zeroed() },
            format: unsafe { std::mem::zeroed() },
            offset: unsafe { std::mem::zeroed() },
            range: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkMemoryBarrier")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryBarrier {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "srcAccessMask")]
    pub src_access_mask: AccessFlags,
    #[doc(alias = "dstAccessMask")]
    pub dst_access_mask: AccessFlags,
}
impl Default for MemoryBarrier {
    fn default() -> Self {
        Self {
            s_type: StructureType::MemoryBarrier,
            p_next: unsafe { std::mem::zeroed() },
            src_access_mask: unsafe { std::mem::zeroed() },
            dst_access_mask: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkBufferMemoryBarrier")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BufferMemoryBarrier {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "srcAccessMask")]
    pub src_access_mask: AccessFlags,
    #[doc(alias = "dstAccessMask")]
    pub dst_access_mask: AccessFlags,
    #[doc(alias = "srcQueueFamilyIndex")]
    pub src_queue_family_index: u32,
    #[doc(alias = "dstQueueFamilyIndex")]
    pub dst_queue_family_index: u32,
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub size: DeviceSize,
}
impl Default for BufferMemoryBarrier {
    fn default() -> Self {
        Self {
            s_type: StructureType::BufferMemoryBarrier,
            p_next: unsafe { std::mem::zeroed() },
            src_access_mask: unsafe { std::mem::zeroed() },
            dst_access_mask: unsafe { std::mem::zeroed() },
            src_queue_family_index: unsafe { std::mem::zeroed() },
            dst_queue_family_index: unsafe { std::mem::zeroed() },
            buffer: unsafe { std::mem::zeroed() },
            offset: unsafe { std::mem::zeroed() },
            size: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkImageMemoryBarrier")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageMemoryBarrier {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "srcAccessMask")]
    pub src_access_mask: AccessFlags,
    #[doc(alias = "dstAccessMask")]
    pub dst_access_mask: AccessFlags,
    #[doc(alias = "oldLayout")]
    pub old_layout: ImageLayout,
    #[doc(alias = "newLayout")]
    pub new_layout: ImageLayout,
    #[doc(alias = "srcQueueFamilyIndex")]
    pub src_queue_family_index: u32,
    #[doc(alias = "dstQueueFamilyIndex")]
    pub dst_queue_family_index: u32,
    pub image: Image,
    #[doc(alias = "subresourceRange")]
    pub subresource_range: ImageSubresourceRange,
}
impl Default for ImageMemoryBarrier {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImageMemoryBarrier,
            p_next: unsafe { std::mem::zeroed() },
            src_access_mask: unsafe { std::mem::zeroed() },
            dst_access_mask: unsafe { std::mem::zeroed() },
            old_layout: unsafe { std::mem::zeroed() },
            new_layout: unsafe { std::mem::zeroed() },
            src_queue_family_index: unsafe { std::mem::zeroed() },
            dst_queue_family_index: unsafe { std::mem::zeroed() },
            image: unsafe { std::mem::zeroed() },
            subresource_range: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkImageCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: ImageCreateFlags,
    #[doc(alias = "imageType")]
    pub image_type: ImageType,
    pub format: Format,
    pub extent: Extent3D,
    #[doc(alias = "mipLevels")]
    pub mip_levels: u32,
    #[doc(alias = "arrayLayers")]
    pub array_layers: u32,
    pub samples: SampleCountFlagBits,
    pub tiling: ImageTiling,
    pub usage: ImageUsageFlags,
    #[doc(alias = "sharingMode")]
    pub sharing_mode: SharingMode,
    #[doc(alias = "queueFamilyIndexCount")]
    pub queue_family_index_count: u32,
    #[doc(alias = "pQueueFamilyIndices")]
    pub queue_family_indices: *const u32,
    #[doc(alias = "initialLayout")]
    pub initial_layout: ImageLayout,
}
impl Default for ImageCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImageCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            image_type: unsafe { std::mem::zeroed() },
            format: unsafe { std::mem::zeroed() },
            extent: unsafe { std::mem::zeroed() },
            mip_levels: unsafe { std::mem::zeroed() },
            array_layers: unsafe { std::mem::zeroed() },
            samples: unsafe { std::mem::zeroed() },
            tiling: unsafe { std::mem::zeroed() },
            usage: unsafe { std::mem::zeroed() },
            sharing_mode: unsafe { std::mem::zeroed() },
            queue_family_index_count: unsafe { std::mem::zeroed() },
            queue_family_indices: unsafe { std::mem::zeroed() },
            initial_layout: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkImageViewCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageViewCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: ImageViewCreateFlags,
    pub image: Image,
    #[doc(alias = "viewType")]
    pub view_type: ImageViewType,
    pub format: Format,
    pub components: ComponentMapping,
    #[doc(alias = "subresourceRange")]
    pub subresource_range: ImageSubresourceRange,
}
impl Default for ImageViewCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImageViewCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            image: unsafe { std::mem::zeroed() },
            view_type: unsafe { std::mem::zeroed() },
            format: unsafe { std::mem::zeroed() },
            components: unsafe { std::mem::zeroed() },
            subresource_range: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSparseMemoryBind")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SparseMemoryBind {
    #[doc(alias = "resourceOffset")]
    pub resource_offset: DeviceSize,
    pub size: DeviceSize,
    pub memory: DeviceMemory,
    #[doc(alias = "memoryOffset")]
    pub memory_offset: DeviceSize,
    pub flags: SparseMemoryBindFlags,
}
impl Default for SparseMemoryBind {
    fn default() -> Self {
        Self {
            resource_offset: unsafe { std::mem::zeroed() },
            size: unsafe { std::mem::zeroed() },
            memory: unsafe { std::mem::zeroed() },
            memory_offset: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSparseImageMemoryBind")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SparseImageMemoryBind {
    pub subresource: ImageSubresource,
    pub offset: Offset3D,
    pub extent: Extent3D,
    pub memory: DeviceMemory,
    #[doc(alias = "memoryOffset")]
    pub memory_offset: DeviceSize,
    pub flags: SparseMemoryBindFlags,
}
impl Default for SparseImageMemoryBind {
    fn default() -> Self {
        Self {
            subresource: unsafe { std::mem::zeroed() },
            offset: unsafe { std::mem::zeroed() },
            extent: unsafe { std::mem::zeroed() },
            memory: unsafe { std::mem::zeroed() },
            memory_offset: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSparseBufferMemoryBindInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SparseBufferMemoryBindInfo {
    pub buffer: Buffer,
    #[doc(alias = "bindCount")]
    pub bind_count: u32,
    #[doc(alias = "pBinds")]
    pub binds: *const SparseMemoryBind,
}
impl Default for SparseBufferMemoryBindInfo {
    fn default() -> Self {
        Self {
            buffer: unsafe { std::mem::zeroed() },
            bind_count: unsafe { std::mem::zeroed() },
            binds: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSparseImageOpaqueMemoryBindInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SparseImageOpaqueMemoryBindInfo {
    pub image: Image,
    #[doc(alias = "bindCount")]
    pub bind_count: u32,
    #[doc(alias = "pBinds")]
    pub binds: *const SparseMemoryBind,
}
impl Default for SparseImageOpaqueMemoryBindInfo {
    fn default() -> Self {
        Self {
            image: unsafe { std::mem::zeroed() },
            bind_count: unsafe { std::mem::zeroed() },
            binds: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSparseImageMemoryBindInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SparseImageMemoryBindInfo {
    pub image: Image,
    #[doc(alias = "bindCount")]
    pub bind_count: u32,
    #[doc(alias = "pBinds")]
    pub binds: *const SparseImageMemoryBind,
}
impl Default for SparseImageMemoryBindInfo {
    fn default() -> Self {
        Self {
            image: unsafe { std::mem::zeroed() },
            bind_count: unsafe { std::mem::zeroed() },
            binds: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkBindSparseInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BindSparseInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "waitSemaphoreCount")]
    pub wait_semaphore_count: u32,
    #[doc(alias = "pWaitSemaphores")]
    pub wait_semaphores: *const Semaphore,
    #[doc(alias = "bufferBindCount")]
    pub buffer_bind_count: u32,
    #[doc(alias = "pBufferBinds")]
    pub buffer_binds: *const SparseBufferMemoryBindInfo,
    #[doc(alias = "imageOpaqueBindCount")]
    pub image_opaque_bind_count: u32,
    #[doc(alias = "pImageOpaqueBinds")]
    pub image_opaque_binds: *const SparseImageOpaqueMemoryBindInfo,
    #[doc(alias = "imageBindCount")]
    pub image_bind_count: u32,
    #[doc(alias = "pImageBinds")]
    pub image_binds: *const SparseImageMemoryBindInfo,
    #[doc(alias = "signalSemaphoreCount")]
    pub signal_semaphore_count: u32,
    #[doc(alias = "pSignalSemaphores")]
    pub signal_semaphores: *const Semaphore,
}
impl Default for BindSparseInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::BindSparseInfo,
            p_next: unsafe { std::mem::zeroed() },
            wait_semaphore_count: unsafe { std::mem::zeroed() },
            wait_semaphores: unsafe { std::mem::zeroed() },
            buffer_bind_count: unsafe { std::mem::zeroed() },
            buffer_binds: unsafe { std::mem::zeroed() },
            image_opaque_bind_count: unsafe { std::mem::zeroed() },
            image_opaque_binds: unsafe { std::mem::zeroed() },
            image_bind_count: unsafe { std::mem::zeroed() },
            image_binds: unsafe { std::mem::zeroed() },
            signal_semaphore_count: unsafe { std::mem::zeroed() },
            signal_semaphores: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkShaderModuleCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ShaderModuleCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: ShaderModuleCreateFlags,
    #[doc(alias = "codeSize")]
    pub code_size: usize,
    #[doc(alias = "pCode")]
    pub code: *const u32,
}
impl Default for ShaderModuleCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::ShaderModuleCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            code_size: unsafe { std::mem::zeroed() },
            code: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDescriptorSetLayoutBinding")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DescriptorSetLayoutBinding {
    pub binding: u32,
    #[doc(alias = "descriptorType")]
    pub descriptor_type: DescriptorType,
    #[doc(alias = "descriptorCount")]
    pub descriptor_count: u32,
    #[doc(alias = "stageFlags")]
    pub stage_flags: ShaderStageFlags,
    #[doc(alias = "pImmutableSamplers")]
    pub immutable_samplers: *const Sampler,
}
impl Default for DescriptorSetLayoutBinding {
    fn default() -> Self {
        Self {
            binding: unsafe { std::mem::zeroed() },
            descriptor_type: unsafe { std::mem::zeroed() },
            descriptor_count: unsafe { std::mem::zeroed() },
            stage_flags: unsafe { std::mem::zeroed() },
            immutable_samplers: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDescriptorSetLayoutCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DescriptorSetLayoutCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: DescriptorSetLayoutCreateFlags,
    #[doc(alias = "bindingCount")]
    pub binding_count: u32,
    #[doc(alias = "pBindings")]
    pub bindings: *const DescriptorSetLayoutBinding,
}
impl Default for DescriptorSetLayoutCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::DescriptorSetLayoutCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            binding_count: unsafe { std::mem::zeroed() },
            bindings: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDescriptorPoolCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DescriptorPoolCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: DescriptorPoolCreateFlags,
    #[doc(alias = "maxSets")]
    pub max_sets: u32,
    #[doc(alias = "poolSizeCount")]
    pub pool_size_count: u32,
    #[doc(alias = "pPoolSizes")]
    pub pool_sizes: *const DescriptorPoolSize,
}
impl Default for DescriptorPoolCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::DescriptorPoolCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            max_sets: unsafe { std::mem::zeroed() },
            pool_size_count: unsafe { std::mem::zeroed() },
            pool_sizes: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDescriptorSetAllocateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DescriptorSetAllocateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "descriptorPool")]
    pub descriptor_pool: DescriptorPool,
    #[doc(alias = "descriptorSetCount")]
    pub descriptor_set_count: u32,
    #[doc(alias = "pSetLayouts")]
    pub set_layouts: *const DescriptorSetLayout,
}
impl Default for DescriptorSetAllocateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::DescriptorSetAllocateInfo,
            p_next: unsafe { std::mem::zeroed() },
            descriptor_pool: unsafe { std::mem::zeroed() },
            descriptor_set_count: unsafe { std::mem::zeroed() },
            set_layouts: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSpecializationInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SpecializationInfo {
    #[doc(alias = "mapEntryCount")]
    pub map_entry_count: u32,
    #[doc(alias = "pMapEntries")]
    pub map_entries: *const SpecializationMapEntry,
    #[doc(alias = "dataSize")]
    pub data_size: usize,
    #[doc(alias = "pData")]
    pub data: *const std::ffi::c_void,
}
impl Default for SpecializationInfo {
    fn default() -> Self {
        Self {
            map_entry_count: unsafe { std::mem::zeroed() },
            map_entries: unsafe { std::mem::zeroed() },
            data_size: unsafe { std::mem::zeroed() },
            data: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineShaderStageCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineShaderStageCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: PipelineShaderStageCreateFlags,
    pub stage: ShaderStageFlagBits,
    pub module: ShaderModule,
    #[doc(alias = "pName")]
    pub name: *const c_char,
    #[doc(alias = "pSpecializationInfo")]
    pub specialization_info: *const SpecializationInfo,
}
impl Default for PipelineShaderStageCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineShaderStageCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            stage: unsafe { std::mem::zeroed() },
            module: unsafe { std::mem::zeroed() },
            name: unsafe { std::mem::zeroed() },
            specialization_info: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkComputePipelineCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ComputePipelineCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: PipelineCreateFlags,
    pub stage: PipelineShaderStageCreateInfo,
    pub layout: PipelineLayout,
    #[doc(alias = "basePipelineHandle")]
    pub base_pipeline_handle: Pipeline,
    #[doc(alias = "basePipelineIndex")]
    pub base_pipeline_index: i32,
}
impl Default for ComputePipelineCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::ComputePipelineCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            stage: unsafe { std::mem::zeroed() },
            layout: unsafe { std::mem::zeroed() },
            base_pipeline_handle: unsafe { std::mem::zeroed() },
            base_pipeline_index: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineVertexInputStateCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineVertexInputStateCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: PipelineVertexInputStateCreateFlags,
    #[doc(alias = "vertexBindingDescriptionCount")]
    pub vertex_binding_description_count: u32,
    #[doc(alias = "pVertexBindingDescriptions")]
    pub vertex_binding_descriptions: *const VertexInputBindingDescription,
    #[doc(alias = "vertexAttributeDescriptionCount")]
    pub vertex_attribute_description_count: u32,
    #[doc(alias = "pVertexAttributeDescriptions")]
    pub vertex_attribute_descriptions: *const VertexInputAttributeDescription,
}
impl Default for PipelineVertexInputStateCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineVertexInputStateCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            vertex_binding_description_count: unsafe { std::mem::zeroed() },
            vertex_binding_descriptions: unsafe { std::mem::zeroed() },
            vertex_attribute_description_count: unsafe { std::mem::zeroed() },
            vertex_attribute_descriptions: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineInputAssemblyStateCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineInputAssemblyStateCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: PipelineInputAssemblyStateCreateFlags,
    pub topology: PrimitiveTopology,
    #[doc(alias = "primitiveRestartEnable")]
    pub primitive_restart_enable: Bool32,
}
impl Default for PipelineInputAssemblyStateCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineInputAssemblyStateCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            topology: unsafe { std::mem::zeroed() },
            primitive_restart_enable: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineTessellationStateCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineTessellationStateCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: PipelineTessellationStateCreateFlags,
    #[doc(alias = "patchControlPoints")]
    pub patch_control_points: u32,
}
impl Default for PipelineTessellationStateCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineTessellationStateCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            patch_control_points: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineViewportStateCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineViewportStateCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: PipelineViewportStateCreateFlags,
    #[doc(alias = "viewportCount")]
    pub viewport_count: u32,
    #[doc(alias = "pViewports")]
    pub viewports: *const Viewport,
    #[doc(alias = "scissorCount")]
    pub scissor_count: u32,
    #[doc(alias = "pScissors")]
    pub scissors: *const Rect2D,
}
impl Default for PipelineViewportStateCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineViewportStateCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            viewport_count: unsafe { std::mem::zeroed() },
            viewports: unsafe { std::mem::zeroed() },
            scissor_count: unsafe { std::mem::zeroed() },
            scissors: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineRasterizationStateCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineRasterizationStateCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: PipelineRasterizationStateCreateFlags,
    #[doc(alias = "depthClampEnable")]
    pub depth_clamp_enable: Bool32,
    #[doc(alias = "rasterizerDiscardEnable")]
    pub rasterizer_discard_enable: Bool32,
    #[doc(alias = "polygonMode")]
    pub polygon_mode: PolygonMode,
    #[doc(alias = "cullMode")]
    pub cull_mode: CullModeFlags,
    #[doc(alias = "frontFace")]
    pub front_face: FrontFace,
    #[doc(alias = "depthBiasEnable")]
    pub depth_bias_enable: Bool32,
    #[doc(alias = "depthBiasConstantFactor")]
    pub depth_bias_constant_factor: f32,
    #[doc(alias = "depthBiasClamp")]
    pub depth_bias_clamp: f32,
    #[doc(alias = "depthBiasSlopeFactor")]
    pub depth_bias_slope_factor: f32,
    #[doc(alias = "lineWidth")]
    pub line_width: f32,
}
impl Default for PipelineRasterizationStateCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineRasterizationStateCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            depth_clamp_enable: unsafe { std::mem::zeroed() },
            rasterizer_discard_enable: unsafe { std::mem::zeroed() },
            polygon_mode: unsafe { std::mem::zeroed() },
            cull_mode: unsafe { std::mem::zeroed() },
            front_face: unsafe { std::mem::zeroed() },
            depth_bias_enable: unsafe { std::mem::zeroed() },
            depth_bias_constant_factor: unsafe { std::mem::zeroed() },
            depth_bias_clamp: unsafe { std::mem::zeroed() },
            depth_bias_slope_factor: unsafe { std::mem::zeroed() },
            line_width: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineMultisampleStateCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineMultisampleStateCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: PipelineMultisampleStateCreateFlags,
    #[doc(alias = "rasterizationSamples")]
    pub rasterization_samples: SampleCountFlagBits,
    #[doc(alias = "sampleShadingEnable")]
    pub sample_shading_enable: Bool32,
    #[doc(alias = "minSampleShading")]
    pub min_sample_shading: f32,
    #[doc(alias = "pSampleMask")]
    pub sample_mask: *const SampleMask,
    #[doc(alias = "alphaToCoverageEnable")]
    pub alpha_to_coverage_enable: Bool32,
    #[doc(alias = "alphaToOneEnable")]
    pub alpha_to_one_enable: Bool32,
}
impl Default for PipelineMultisampleStateCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineMultisampleStateCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            rasterization_samples: unsafe { std::mem::zeroed() },
            sample_shading_enable: unsafe { std::mem::zeroed() },
            min_sample_shading: unsafe { std::mem::zeroed() },
            sample_mask: unsafe { std::mem::zeroed() },
            alpha_to_coverage_enable: unsafe { std::mem::zeroed() },
            alpha_to_one_enable: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineColorBlendAttachmentState")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineColorBlendAttachmentState {
    #[doc(alias = "blendEnable")]
    pub blend_enable: Bool32,
    #[doc(alias = "srcColorBlendFactor")]
    pub src_color_blend_factor: BlendFactor,
    #[doc(alias = "dstColorBlendFactor")]
    pub dst_color_blend_factor: BlendFactor,
    #[doc(alias = "colorBlendOp")]
    pub color_blend_op: BlendOp,
    #[doc(alias = "srcAlphaBlendFactor")]
    pub src_alpha_blend_factor: BlendFactor,
    #[doc(alias = "dstAlphaBlendFactor")]
    pub dst_alpha_blend_factor: BlendFactor,
    #[doc(alias = "alphaBlendOp")]
    pub alpha_blend_op: BlendOp,
    #[doc(alias = "colorWriteMask")]
    pub color_write_mask: ColorComponentFlags,
}
impl Default for PipelineColorBlendAttachmentState {
    fn default() -> Self {
        Self {
            blend_enable: unsafe { std::mem::zeroed() },
            src_color_blend_factor: unsafe { std::mem::zeroed() },
            dst_color_blend_factor: unsafe { std::mem::zeroed() },
            color_blend_op: unsafe { std::mem::zeroed() },
            src_alpha_blend_factor: unsafe { std::mem::zeroed() },
            dst_alpha_blend_factor: unsafe { std::mem::zeroed() },
            alpha_blend_op: unsafe { std::mem::zeroed() },
            color_write_mask: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineColorBlendStateCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineColorBlendStateCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: PipelineColorBlendStateCreateFlags,
    #[doc(alias = "logicOpEnable")]
    pub logic_op_enable: Bool32,
    #[doc(alias = "logicOp")]
    pub logic_op: LogicOp,
    #[doc(alias = "attachmentCount")]
    pub attachment_count: u32,
    #[doc(alias = "pAttachments")]
    pub attachments: *const PipelineColorBlendAttachmentState,
    #[doc(alias = "blendConstants")]
    pub blend_constants: [f32; 4 as usize],
}
impl Default for PipelineColorBlendStateCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineColorBlendStateCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            logic_op_enable: unsafe { std::mem::zeroed() },
            logic_op: unsafe { std::mem::zeroed() },
            attachment_count: unsafe { std::mem::zeroed() },
            attachments: unsafe { std::mem::zeroed() },
            blend_constants: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineDynamicStateCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineDynamicStateCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: PipelineDynamicStateCreateFlags,
    #[doc(alias = "dynamicStateCount")]
    pub dynamic_state_count: u32,
    #[doc(alias = "pDynamicStates")]
    pub dynamic_states: *const DynamicState,
}
impl Default for PipelineDynamicStateCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineDynamicStateCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            dynamic_state_count: unsafe { std::mem::zeroed() },
            dynamic_states: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineDepthStencilStateCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineDepthStencilStateCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: PipelineDepthStencilStateCreateFlags,
    #[doc(alias = "depthTestEnable")]
    pub depth_test_enable: Bool32,
    #[doc(alias = "depthWriteEnable")]
    pub depth_write_enable: Bool32,
    #[doc(alias = "depthCompareOp")]
    pub depth_compare_op: CompareOp,
    #[doc(alias = "depthBoundsTestEnable")]
    pub depth_bounds_test_enable: Bool32,
    #[doc(alias = "stencilTestEnable")]
    pub stencil_test_enable: Bool32,
    pub front: StencilOpState,
    pub back: StencilOpState,
    #[doc(alias = "minDepthBounds")]
    pub min_depth_bounds: f32,
    #[doc(alias = "maxDepthBounds")]
    pub max_depth_bounds: f32,
}
impl Default for PipelineDepthStencilStateCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineDepthStencilStateCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            depth_test_enable: unsafe { std::mem::zeroed() },
            depth_write_enable: unsafe { std::mem::zeroed() },
            depth_compare_op: unsafe { std::mem::zeroed() },
            depth_bounds_test_enable: unsafe { std::mem::zeroed() },
            stencil_test_enable: unsafe { std::mem::zeroed() },
            front: unsafe { std::mem::zeroed() },
            back: unsafe { std::mem::zeroed() },
            min_depth_bounds: unsafe { std::mem::zeroed() },
            max_depth_bounds: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkGraphicsPipelineCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GraphicsPipelineCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: PipelineCreateFlags,
    #[doc(alias = "stageCount")]
    pub stage_count: u32,
    #[doc(alias = "pStages")]
    pub stages: *const PipelineShaderStageCreateInfo,
    #[doc(alias = "pVertexInputState")]
    pub vertex_input_state: *const PipelineVertexInputStateCreateInfo,
    #[doc(alias = "pInputAssemblyState")]
    pub input_assembly_state: *const PipelineInputAssemblyStateCreateInfo,
    #[doc(alias = "pTessellationState")]
    pub tessellation_state: *const PipelineTessellationStateCreateInfo,
    #[doc(alias = "pViewportState")]
    pub viewport_state: *const PipelineViewportStateCreateInfo,
    #[doc(alias = "pRasterizationState")]
    pub rasterization_state: *const PipelineRasterizationStateCreateInfo,
    #[doc(alias = "pMultisampleState")]
    pub multisample_state: *const PipelineMultisampleStateCreateInfo,
    #[doc(alias = "pDepthStencilState")]
    pub depth_stencil_state: *const PipelineDepthStencilStateCreateInfo,
    #[doc(alias = "pColorBlendState")]
    pub color_blend_state: *const PipelineColorBlendStateCreateInfo,
    #[doc(alias = "pDynamicState")]
    pub dynamic_state: *const PipelineDynamicStateCreateInfo,
    pub layout: PipelineLayout,
    #[doc(alias = "renderPass")]
    pub render_pass: RenderPass,
    pub subpass: u32,
    #[doc(alias = "basePipelineHandle")]
    pub base_pipeline_handle: Pipeline,
    #[doc(alias = "basePipelineIndex")]
    pub base_pipeline_index: i32,
}
impl Default for GraphicsPipelineCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::GraphicsPipelineCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            stage_count: unsafe { std::mem::zeroed() },
            stages: unsafe { std::mem::zeroed() },
            vertex_input_state: unsafe { std::mem::zeroed() },
            input_assembly_state: unsafe { std::mem::zeroed() },
            tessellation_state: unsafe { std::mem::zeroed() },
            viewport_state: unsafe { std::mem::zeroed() },
            rasterization_state: unsafe { std::mem::zeroed() },
            multisample_state: unsafe { std::mem::zeroed() },
            depth_stencil_state: unsafe { std::mem::zeroed() },
            color_blend_state: unsafe { std::mem::zeroed() },
            dynamic_state: unsafe { std::mem::zeroed() },
            layout: unsafe { std::mem::zeroed() },
            render_pass: unsafe { std::mem::zeroed() },
            subpass: unsafe { std::mem::zeroed() },
            base_pipeline_handle: unsafe { std::mem::zeroed() },
            base_pipeline_index: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineCacheCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineCacheCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: PipelineCacheCreateFlags,
    #[doc(alias = "initialDataSize")]
    pub initial_data_size: usize,
    #[doc(alias = "pInitialData")]
    pub initial_data: *const std::ffi::c_void,
}
impl Default for PipelineCacheCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineCacheCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            initial_data_size: unsafe { std::mem::zeroed() },
            initial_data: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineLayoutCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineLayoutCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: PipelineLayoutCreateFlags,
    #[doc(alias = "setLayoutCount")]
    pub set_layout_count: u32,
    #[doc(alias = "pSetLayouts")]
    pub set_layouts: *const DescriptorSetLayout,
    #[doc(alias = "pushConstantRangeCount")]
    pub push_constant_range_count: u32,
    #[doc(alias = "pPushConstantRanges")]
    pub push_constant_ranges: *const PushConstantRange,
}
impl Default for PipelineLayoutCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineLayoutCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            set_layout_count: unsafe { std::mem::zeroed() },
            set_layouts: unsafe { std::mem::zeroed() },
            push_constant_range_count: unsafe { std::mem::zeroed() },
            push_constant_ranges: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSamplerCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SamplerCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: SamplerCreateFlags,
    #[doc(alias = "magFilter")]
    pub mag_filter: Filter,
    #[doc(alias = "minFilter")]
    pub min_filter: Filter,
    #[doc(alias = "mipmapMode")]
    pub mipmap_mode: SamplerMipmapMode,
    #[doc(alias = "addressModeU")]
    pub address_mode_u: SamplerAddressMode,
    #[doc(alias = "addressModeV")]
    pub address_mode_v: SamplerAddressMode,
    #[doc(alias = "addressModeW")]
    pub address_mode_w: SamplerAddressMode,
    #[doc(alias = "mipLodBias")]
    pub mip_lod_bias: f32,
    #[doc(alias = "anisotropyEnable")]
    pub anisotropy_enable: Bool32,
    #[doc(alias = "maxAnisotropy")]
    pub max_anisotropy: f32,
    #[doc(alias = "compareEnable")]
    pub compare_enable: Bool32,
    #[doc(alias = "compareOp")]
    pub compare_op: CompareOp,
    #[doc(alias = "minLod")]
    pub min_lod: f32,
    #[doc(alias = "maxLod")]
    pub max_lod: f32,
    #[doc(alias = "borderColor")]
    pub border_color: BorderColor,
    #[doc(alias = "unnormalizedCoordinates")]
    pub unnormalized_coordinates: Bool32,
}
impl Default for SamplerCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::SamplerCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            mag_filter: unsafe { std::mem::zeroed() },
            min_filter: unsafe { std::mem::zeroed() },
            mipmap_mode: unsafe { std::mem::zeroed() },
            address_mode_u: unsafe { std::mem::zeroed() },
            address_mode_v: unsafe { std::mem::zeroed() },
            address_mode_w: unsafe { std::mem::zeroed() },
            mip_lod_bias: unsafe { std::mem::zeroed() },
            anisotropy_enable: unsafe { std::mem::zeroed() },
            max_anisotropy: unsafe { std::mem::zeroed() },
            compare_enable: unsafe { std::mem::zeroed() },
            compare_op: unsafe { std::mem::zeroed() },
            min_lod: unsafe { std::mem::zeroed() },
            max_lod: unsafe { std::mem::zeroed() },
            border_color: unsafe { std::mem::zeroed() },
            unnormalized_coordinates: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkCommandPoolCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CommandPoolCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: CommandPoolCreateFlags,
    #[doc(alias = "queueFamilyIndex")]
    pub queue_family_index: u32,
}
impl Default for CommandPoolCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::CommandPoolCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            queue_family_index: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkCommandBufferAllocateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CommandBufferAllocateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "commandPool")]
    pub command_pool: CommandPool,
    pub level: CommandBufferLevel,
    #[doc(alias = "commandBufferCount")]
    pub command_buffer_count: u32,
}
impl Default for CommandBufferAllocateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::CommandBufferAllocateInfo,
            p_next: unsafe { std::mem::zeroed() },
            command_pool: unsafe { std::mem::zeroed() },
            level: unsafe { std::mem::zeroed() },
            command_buffer_count: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkCommandBufferInheritanceInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CommandBufferInheritanceInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "renderPass")]
    pub render_pass: RenderPass,
    pub subpass: u32,
    pub framebuffer: Framebuffer,
    #[doc(alias = "occlusionQueryEnable")]
    pub occlusion_query_enable: Bool32,
    #[doc(alias = "queryFlags")]
    pub query_flags: QueryControlFlags,
    #[doc(alias = "pipelineStatistics")]
    pub pipeline_statistics: QueryPipelineStatisticFlags,
}
impl Default for CommandBufferInheritanceInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::CommandBufferInheritanceInfo,
            p_next: unsafe { std::mem::zeroed() },
            render_pass: unsafe { std::mem::zeroed() },
            subpass: unsafe { std::mem::zeroed() },
            framebuffer: unsafe { std::mem::zeroed() },
            occlusion_query_enable: unsafe { std::mem::zeroed() },
            query_flags: unsafe { std::mem::zeroed() },
            pipeline_statistics: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkCommandBufferBeginInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CommandBufferBeginInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: CommandBufferUsageFlags,
    #[doc(alias = "pInheritanceInfo")]
    pub inheritance_info: *const CommandBufferInheritanceInfo,
}
impl Default for CommandBufferBeginInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::CommandBufferBeginInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            inheritance_info: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkRenderPassBeginInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RenderPassBeginInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "renderPass")]
    pub render_pass: RenderPass,
    pub framebuffer: Framebuffer,
    #[doc(alias = "renderArea")]
    pub render_area: Rect2D,
    #[doc(alias = "clearValueCount")]
    pub clear_value_count: u32,
    #[doc(alias = "pClearValues")]
    pub clear_values: *const ClearValue,
}
impl Default for RenderPassBeginInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::RenderPassBeginInfo,
            p_next: unsafe { std::mem::zeroed() },
            render_pass: unsafe { std::mem::zeroed() },
            framebuffer: unsafe { std::mem::zeroed() },
            render_area: unsafe { std::mem::zeroed() },
            clear_value_count: unsafe { std::mem::zeroed() },
            clear_values: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkClearAttachment")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ClearAttachment {
    #[doc(alias = "aspectMask")]
    pub aspect_mask: ImageAspectFlags,
    #[doc(alias = "colorAttachment")]
    pub color_attachment: u32,
    #[doc(alias = "clearValue")]
    pub clear_value: ClearValue,
}
impl Default for ClearAttachment {
    fn default() -> Self {
        Self {
            aspect_mask: unsafe { std::mem::zeroed() },
            color_attachment: unsafe { std::mem::zeroed() },
            clear_value: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSubpassDescription")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SubpassDescription {
    pub flags: SubpassDescriptionFlags,
    #[doc(alias = "pipelineBindPoint")]
    pub pipeline_bind_point: PipelineBindPoint,
    #[doc(alias = "inputAttachmentCount")]
    pub input_attachment_count: u32,
    #[doc(alias = "pInputAttachments")]
    pub input_attachments: *const AttachmentReference,
    #[doc(alias = "colorAttachmentCount")]
    pub color_attachment_count: u32,
    #[doc(alias = "pColorAttachments")]
    pub color_attachments: *const AttachmentReference,
    #[doc(alias = "pResolveAttachments")]
    pub resolve_attachments: *const AttachmentReference,
    #[doc(alias = "pDepthStencilAttachment")]
    pub depth_stencil_attachment: *const AttachmentReference,
    #[doc(alias = "preserveAttachmentCount")]
    pub preserve_attachment_count: u32,
    #[doc(alias = "pPreserveAttachments")]
    pub preserve_attachments: *const u32,
}
impl Default for SubpassDescription {
    fn default() -> Self {
        Self {
            flags: unsafe { std::mem::zeroed() },
            pipeline_bind_point: unsafe { std::mem::zeroed() },
            input_attachment_count: unsafe { std::mem::zeroed() },
            input_attachments: unsafe { std::mem::zeroed() },
            color_attachment_count: unsafe { std::mem::zeroed() },
            color_attachments: unsafe { std::mem::zeroed() },
            resolve_attachments: unsafe { std::mem::zeroed() },
            depth_stencil_attachment: unsafe { std::mem::zeroed() },
            preserve_attachment_count: unsafe { std::mem::zeroed() },
            preserve_attachments: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkRenderPassCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RenderPassCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: RenderPassCreateFlags,
    #[doc(alias = "attachmentCount")]
    pub attachment_count: u32,
    #[doc(alias = "pAttachments")]
    pub attachments: *const AttachmentDescription,
    #[doc(alias = "subpassCount")]
    pub subpass_count: u32,
    #[doc(alias = "pSubpasses")]
    pub subpasses: *const SubpassDescription,
    #[doc(alias = "dependencyCount")]
    pub dependency_count: u32,
    #[doc(alias = "pDependencies")]
    pub dependencies: *const SubpassDependency,
}
impl Default for RenderPassCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::RenderPassCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            attachment_count: unsafe { std::mem::zeroed() },
            attachments: unsafe { std::mem::zeroed() },
            subpass_count: unsafe { std::mem::zeroed() },
            subpasses: unsafe { std::mem::zeroed() },
            dependency_count: unsafe { std::mem::zeroed() },
            dependencies: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkEventCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct EventCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: EventCreateFlags,
}
impl Default for EventCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::EventCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkFenceCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct FenceCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: FenceCreateFlags,
}
impl Default for FenceCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::FenceCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFeatures {
    #[doc(alias = "robustBufferAccess")]
    pub robust_buffer_access: Bool32,
    #[doc(alias = "fullDrawIndexUint32")]
    pub full_draw_index_uint32: Bool32,
    #[doc(alias = "imageCubeArray")]
    pub image_cube_array: Bool32,
    #[doc(alias = "independentBlend")]
    pub independent_blend: Bool32,
    #[doc(alias = "geometryShader")]
    pub geometry_shader: Bool32,
    #[doc(alias = "tessellationShader")]
    pub tessellation_shader: Bool32,
    #[doc(alias = "sampleRateShading")]
    pub sample_rate_shading: Bool32,
    #[doc(alias = "dualSrcBlend")]
    pub dual_src_blend: Bool32,
    #[doc(alias = "logicOp")]
    pub logic_op: Bool32,
    #[doc(alias = "multiDrawIndirect")]
    pub multi_draw_indirect: Bool32,
    #[doc(alias = "drawIndirectFirstInstance")]
    pub draw_indirect_first_instance: Bool32,
    #[doc(alias = "depthClamp")]
    pub depth_clamp: Bool32,
    #[doc(alias = "depthBiasClamp")]
    pub depth_bias_clamp: Bool32,
    #[doc(alias = "fillModeNonSolid")]
    pub fill_mode_non_solid: Bool32,
    #[doc(alias = "depthBounds")]
    pub depth_bounds: Bool32,
    #[doc(alias = "wideLines")]
    pub wide_lines: Bool32,
    #[doc(alias = "largePoints")]
    pub large_points: Bool32,
    #[doc(alias = "alphaToOne")]
    pub alpha_to_one: Bool32,
    #[doc(alias = "multiViewport")]
    pub multi_viewport: Bool32,
    #[doc(alias = "samplerAnisotropy")]
    pub sampler_anisotropy: Bool32,
    #[doc(alias = "textureCompressionETC2")]
    pub texture_compression_etc2: Bool32,
    #[doc(alias = "textureCompressionASTC_LDR")]
    pub texture_compression_astc_ldr: Bool32,
    #[doc(alias = "textureCompressionBC")]
    pub texture_compression_bc: Bool32,
    #[doc(alias = "occlusionQueryPrecise")]
    pub occlusion_query_precise: Bool32,
    #[doc(alias = "pipelineStatisticsQuery")]
    pub pipeline_statistics_query: Bool32,
    #[doc(alias = "vertexPipelineStoresAndAtomics")]
    pub vertex_pipeline_stores_and_atomics: Bool32,
    #[doc(alias = "fragmentStoresAndAtomics")]
    pub fragment_stores_and_atomics: Bool32,
    #[doc(alias = "shaderTessellationAndGeometryPointSize")]
    pub shader_tessellation_and_geometry_point_size: Bool32,
    #[doc(alias = "shaderImageGatherExtended")]
    pub shader_image_gather_extended: Bool32,
    #[doc(alias = "shaderStorageImageExtendedFormats")]
    pub shader_storage_image_extended_formats: Bool32,
    #[doc(alias = "shaderStorageImageMultisample")]
    pub shader_storage_image_multisample: Bool32,
    #[doc(alias = "shaderStorageImageReadWithoutFormat")]
    pub shader_storage_image_read_without_format: Bool32,
    #[doc(alias = "shaderStorageImageWriteWithoutFormat")]
    pub shader_storage_image_write_without_format: Bool32,
    #[doc(alias = "shaderUniformBufferArrayDynamicIndexing")]
    pub shader_uniform_buffer_array_dynamic_indexing: Bool32,
    #[doc(alias = "shaderSampledImageArrayDynamicIndexing")]
    pub shader_sampled_image_array_dynamic_indexing: Bool32,
    #[doc(alias = "shaderStorageBufferArrayDynamicIndexing")]
    pub shader_storage_buffer_array_dynamic_indexing: Bool32,
    #[doc(alias = "shaderStorageImageArrayDynamicIndexing")]
    pub shader_storage_image_array_dynamic_indexing: Bool32,
    #[doc(alias = "shaderClipDistance")]
    pub shader_clip_distance: Bool32,
    #[doc(alias = "shaderCullDistance")]
    pub shader_cull_distance: Bool32,
    #[doc(alias = "shaderFloat64")]
    pub shader_float64: Bool32,
    #[doc(alias = "shaderInt64")]
    pub shader_int64: Bool32,
    #[doc(alias = "shaderInt16")]
    pub shader_int16: Bool32,
    #[doc(alias = "shaderResourceResidency")]
    pub shader_resource_residency: Bool32,
    #[doc(alias = "shaderResourceMinLod")]
    pub shader_resource_min_lod: Bool32,
    #[doc(alias = "sparseBinding")]
    pub sparse_binding: Bool32,
    #[doc(alias = "sparseResidencyBuffer")]
    pub sparse_residency_buffer: Bool32,
    #[doc(alias = "sparseResidencyImage2D")]
    pub sparse_residency_image2_d: Bool32,
    #[doc(alias = "sparseResidencyImage3D")]
    pub sparse_residency_image3_d: Bool32,
    #[doc(alias = "sparseResidency2Samples")]
    pub sparse_residency2_samples: Bool32,
    #[doc(alias = "sparseResidency4Samples")]
    pub sparse_residency4_samples: Bool32,
    #[doc(alias = "sparseResidency8Samples")]
    pub sparse_residency8_samples: Bool32,
    #[doc(alias = "sparseResidency16Samples")]
    pub sparse_residency16_samples: Bool32,
    #[doc(alias = "sparseResidencyAliased")]
    pub sparse_residency_aliased: Bool32,
    #[doc(alias = "variableMultisampleRate")]
    pub variable_multisample_rate: Bool32,
    #[doc(alias = "inheritedQueries")]
    pub inherited_queries: Bool32,
}
impl Default for PhysicalDeviceFeatures {
    fn default() -> Self {
        Self {
            robust_buffer_access: unsafe { std::mem::zeroed() },
            full_draw_index_uint32: unsafe { std::mem::zeroed() },
            image_cube_array: unsafe { std::mem::zeroed() },
            independent_blend: unsafe { std::mem::zeroed() },
            geometry_shader: unsafe { std::mem::zeroed() },
            tessellation_shader: unsafe { std::mem::zeroed() },
            sample_rate_shading: unsafe { std::mem::zeroed() },
            dual_src_blend: unsafe { std::mem::zeroed() },
            logic_op: unsafe { std::mem::zeroed() },
            multi_draw_indirect: unsafe { std::mem::zeroed() },
            draw_indirect_first_instance: unsafe { std::mem::zeroed() },
            depth_clamp: unsafe { std::mem::zeroed() },
            depth_bias_clamp: unsafe { std::mem::zeroed() },
            fill_mode_non_solid: unsafe { std::mem::zeroed() },
            depth_bounds: unsafe { std::mem::zeroed() },
            wide_lines: unsafe { std::mem::zeroed() },
            large_points: unsafe { std::mem::zeroed() },
            alpha_to_one: unsafe { std::mem::zeroed() },
            multi_viewport: unsafe { std::mem::zeroed() },
            sampler_anisotropy: unsafe { std::mem::zeroed() },
            texture_compression_etc2: unsafe { std::mem::zeroed() },
            texture_compression_astc_ldr: unsafe { std::mem::zeroed() },
            texture_compression_bc: unsafe { std::mem::zeroed() },
            occlusion_query_precise: unsafe { std::mem::zeroed() },
            pipeline_statistics_query: unsafe { std::mem::zeroed() },
            vertex_pipeline_stores_and_atomics: unsafe { std::mem::zeroed() },
            fragment_stores_and_atomics: unsafe { std::mem::zeroed() },
            shader_tessellation_and_geometry_point_size: unsafe { std::mem::zeroed() },
            shader_image_gather_extended: unsafe { std::mem::zeroed() },
            shader_storage_image_extended_formats: unsafe { std::mem::zeroed() },
            shader_storage_image_multisample: unsafe { std::mem::zeroed() },
            shader_storage_image_read_without_format: unsafe { std::mem::zeroed() },
            shader_storage_image_write_without_format: unsafe { std::mem::zeroed() },
            shader_uniform_buffer_array_dynamic_indexing: unsafe { std::mem::zeroed() },
            shader_sampled_image_array_dynamic_indexing: unsafe { std::mem::zeroed() },
            shader_storage_buffer_array_dynamic_indexing: unsafe { std::mem::zeroed() },
            shader_storage_image_array_dynamic_indexing: unsafe { std::mem::zeroed() },
            shader_clip_distance: unsafe { std::mem::zeroed() },
            shader_cull_distance: unsafe { std::mem::zeroed() },
            shader_float64: unsafe { std::mem::zeroed() },
            shader_int64: unsafe { std::mem::zeroed() },
            shader_int16: unsafe { std::mem::zeroed() },
            shader_resource_residency: unsafe { std::mem::zeroed() },
            shader_resource_min_lod: unsafe { std::mem::zeroed() },
            sparse_binding: unsafe { std::mem::zeroed() },
            sparse_residency_buffer: unsafe { std::mem::zeroed() },
            sparse_residency_image2_d: unsafe { std::mem::zeroed() },
            sparse_residency_image3_d: unsafe { std::mem::zeroed() },
            sparse_residency2_samples: unsafe { std::mem::zeroed() },
            sparse_residency4_samples: unsafe { std::mem::zeroed() },
            sparse_residency8_samples: unsafe { std::mem::zeroed() },
            sparse_residency16_samples: unsafe { std::mem::zeroed() },
            sparse_residency_aliased: unsafe { std::mem::zeroed() },
            variable_multisample_rate: unsafe { std::mem::zeroed() },
            inherited_queries: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceSparseProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSparseProperties {
    #[doc(alias = "residencyStandard2DBlockShape")]
    pub residency_standard2_d_block_shape: Bool32,
    #[doc(alias = "residencyStandard2DMultisampleBlockShape")]
    pub residency_standard2_d_multisample_block_shape: Bool32,
    #[doc(alias = "residencyStandard3DBlockShape")]
    pub residency_standard3_d_block_shape: Bool32,
    #[doc(alias = "residencyAlignedMipSize")]
    pub residency_aligned_mip_size: Bool32,
    #[doc(alias = "residencyNonResidentStrict")]
    pub residency_non_resident_strict: Bool32,
}
impl Default for PhysicalDeviceSparseProperties {
    fn default() -> Self {
        Self {
            residency_standard2_d_block_shape: unsafe { std::mem::zeroed() },
            residency_standard2_d_multisample_block_shape: unsafe { std::mem::zeroed() },
            residency_standard3_d_block_shape: unsafe { std::mem::zeroed() },
            residency_aligned_mip_size: unsafe { std::mem::zeroed() },
            residency_non_resident_strict: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceLimits")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceLimits {
    #[doc(alias = "maxImageDimension1D")]
    pub max_image_dimension1_d: u32,
    #[doc(alias = "maxImageDimension2D")]
    pub max_image_dimension2_d: u32,
    #[doc(alias = "maxImageDimension3D")]
    pub max_image_dimension3_d: u32,
    #[doc(alias = "maxImageDimensionCube")]
    pub max_image_dimension_cube: u32,
    #[doc(alias = "maxImageArrayLayers")]
    pub max_image_array_layers: u32,
    #[doc(alias = "maxTexelBufferElements")]
    pub max_texel_buffer_elements: u32,
    #[doc(alias = "maxUniformBufferRange")]
    pub max_uniform_buffer_range: u32,
    #[doc(alias = "maxStorageBufferRange")]
    pub max_storage_buffer_range: u32,
    #[doc(alias = "maxPushConstantsSize")]
    pub max_push_constants_size: u32,
    #[doc(alias = "maxMemoryAllocationCount")]
    pub max_memory_allocation_count: u32,
    #[doc(alias = "maxSamplerAllocationCount")]
    pub max_sampler_allocation_count: u32,
    #[doc(alias = "bufferImageGranularity")]
    pub buffer_image_granularity: DeviceSize,
    #[doc(alias = "sparseAddressSpaceSize")]
    pub sparse_address_space_size: DeviceSize,
    #[doc(alias = "maxBoundDescriptorSets")]
    pub max_bound_descriptor_sets: u32,
    #[doc(alias = "maxPerStageDescriptorSamplers")]
    pub max_per_stage_descriptor_samplers: u32,
    #[doc(alias = "maxPerStageDescriptorUniformBuffers")]
    pub max_per_stage_descriptor_uniform_buffers: u32,
    #[doc(alias = "maxPerStageDescriptorStorageBuffers")]
    pub max_per_stage_descriptor_storage_buffers: u32,
    #[doc(alias = "maxPerStageDescriptorSampledImages")]
    pub max_per_stage_descriptor_sampled_images: u32,
    #[doc(alias = "maxPerStageDescriptorStorageImages")]
    pub max_per_stage_descriptor_storage_images: u32,
    #[doc(alias = "maxPerStageDescriptorInputAttachments")]
    pub max_per_stage_descriptor_input_attachments: u32,
    #[doc(alias = "maxPerStageResources")]
    pub max_per_stage_resources: u32,
    #[doc(alias = "maxDescriptorSetSamplers")]
    pub max_descriptor_set_samplers: u32,
    #[doc(alias = "maxDescriptorSetUniformBuffers")]
    pub max_descriptor_set_uniform_buffers: u32,
    #[doc(alias = "maxDescriptorSetUniformBuffersDynamic")]
    pub max_descriptor_set_uniform_buffers_dynamic: u32,
    #[doc(alias = "maxDescriptorSetStorageBuffers")]
    pub max_descriptor_set_storage_buffers: u32,
    #[doc(alias = "maxDescriptorSetStorageBuffersDynamic")]
    pub max_descriptor_set_storage_buffers_dynamic: u32,
    #[doc(alias = "maxDescriptorSetSampledImages")]
    pub max_descriptor_set_sampled_images: u32,
    #[doc(alias = "maxDescriptorSetStorageImages")]
    pub max_descriptor_set_storage_images: u32,
    #[doc(alias = "maxDescriptorSetInputAttachments")]
    pub max_descriptor_set_input_attachments: u32,
    #[doc(alias = "maxVertexInputAttributes")]
    pub max_vertex_input_attributes: u32,
    #[doc(alias = "maxVertexInputBindings")]
    pub max_vertex_input_bindings: u32,
    #[doc(alias = "maxVertexInputAttributeOffset")]
    pub max_vertex_input_attribute_offset: u32,
    #[doc(alias = "maxVertexInputBindingStride")]
    pub max_vertex_input_binding_stride: u32,
    #[doc(alias = "maxVertexOutputComponents")]
    pub max_vertex_output_components: u32,
    #[doc(alias = "maxTessellationGenerationLevel")]
    pub max_tessellation_generation_level: u32,
    #[doc(alias = "maxTessellationPatchSize")]
    pub max_tessellation_patch_size: u32,
    #[doc(alias = "maxTessellationControlPerVertexInputComponents")]
    pub max_tessellation_control_per_vertex_input_components: u32,
    #[doc(alias = "maxTessellationControlPerVertexOutputComponents")]
    pub max_tessellation_control_per_vertex_output_components: u32,
    #[doc(alias = "maxTessellationControlPerPatchOutputComponents")]
    pub max_tessellation_control_per_patch_output_components: u32,
    #[doc(alias = "maxTessellationControlTotalOutputComponents")]
    pub max_tessellation_control_total_output_components: u32,
    #[doc(alias = "maxTessellationEvaluationInputComponents")]
    pub max_tessellation_evaluation_input_components: u32,
    #[doc(alias = "maxTessellationEvaluationOutputComponents")]
    pub max_tessellation_evaluation_output_components: u32,
    #[doc(alias = "maxGeometryShaderInvocations")]
    pub max_geometry_shader_invocations: u32,
    #[doc(alias = "maxGeometryInputComponents")]
    pub max_geometry_input_components: u32,
    #[doc(alias = "maxGeometryOutputComponents")]
    pub max_geometry_output_components: u32,
    #[doc(alias = "maxGeometryOutputVertices")]
    pub max_geometry_output_vertices: u32,
    #[doc(alias = "maxGeometryTotalOutputComponents")]
    pub max_geometry_total_output_components: u32,
    #[doc(alias = "maxFragmentInputComponents")]
    pub max_fragment_input_components: u32,
    #[doc(alias = "maxFragmentOutputAttachments")]
    pub max_fragment_output_attachments: u32,
    #[doc(alias = "maxFragmentDualSrcAttachments")]
    pub max_fragment_dual_src_attachments: u32,
    #[doc(alias = "maxFragmentCombinedOutputResources")]
    pub max_fragment_combined_output_resources: u32,
    #[doc(alias = "maxComputeSharedMemorySize")]
    pub max_compute_shared_memory_size: u32,
    #[doc(alias = "maxComputeWorkGroupCount")]
    pub max_compute_work_group_count: [u32; 3 as usize],
    #[doc(alias = "maxComputeWorkGroupInvocations")]
    pub max_compute_work_group_invocations: u32,
    #[doc(alias = "maxComputeWorkGroupSize")]
    pub max_compute_work_group_size: [u32; 3 as usize],
    #[doc(alias = "subPixelPrecisionBits")]
    pub sub_pixel_precision_bits: u32,
    #[doc(alias = "subTexelPrecisionBits")]
    pub sub_texel_precision_bits: u32,
    #[doc(alias = "mipmapPrecisionBits")]
    pub mipmap_precision_bits: u32,
    #[doc(alias = "maxDrawIndexedIndexValue")]
    pub max_draw_indexed_index_value: u32,
    #[doc(alias = "maxDrawIndirectCount")]
    pub max_draw_indirect_count: u32,
    #[doc(alias = "maxSamplerLodBias")]
    pub max_sampler_lod_bias: f32,
    #[doc(alias = "maxSamplerAnisotropy")]
    pub max_sampler_anisotropy: f32,
    #[doc(alias = "maxViewports")]
    pub max_viewports: u32,
    #[doc(alias = "maxViewportDimensions")]
    pub max_viewport_dimensions: [u32; 2 as usize],
    #[doc(alias = "viewportBoundsRange")]
    pub viewport_bounds_range: [f32; 2 as usize],
    #[doc(alias = "viewportSubPixelBits")]
    pub viewport_sub_pixel_bits: u32,
    #[doc(alias = "minMemoryMapAlignment")]
    pub min_memory_map_alignment: usize,
    #[doc(alias = "minTexelBufferOffsetAlignment")]
    pub min_texel_buffer_offset_alignment: DeviceSize,
    #[doc(alias = "minUniformBufferOffsetAlignment")]
    pub min_uniform_buffer_offset_alignment: DeviceSize,
    #[doc(alias = "minStorageBufferOffsetAlignment")]
    pub min_storage_buffer_offset_alignment: DeviceSize,
    #[doc(alias = "minTexelOffset")]
    pub min_texel_offset: i32,
    #[doc(alias = "maxTexelOffset")]
    pub max_texel_offset: u32,
    #[doc(alias = "minTexelGatherOffset")]
    pub min_texel_gather_offset: i32,
    #[doc(alias = "maxTexelGatherOffset")]
    pub max_texel_gather_offset: u32,
    #[doc(alias = "minInterpolationOffset")]
    pub min_interpolation_offset: f32,
    #[doc(alias = "maxInterpolationOffset")]
    pub max_interpolation_offset: f32,
    #[doc(alias = "subPixelInterpolationOffsetBits")]
    pub sub_pixel_interpolation_offset_bits: u32,
    #[doc(alias = "maxFramebufferWidth")]
    pub max_framebuffer_width: u32,
    #[doc(alias = "maxFramebufferHeight")]
    pub max_framebuffer_height: u32,
    #[doc(alias = "maxFramebufferLayers")]
    pub max_framebuffer_layers: u32,
    #[doc(alias = "framebufferColorSampleCounts")]
    pub framebuffer_color_sample_counts: SampleCountFlags,
    #[doc(alias = "framebufferDepthSampleCounts")]
    pub framebuffer_depth_sample_counts: SampleCountFlags,
    #[doc(alias = "framebufferStencilSampleCounts")]
    pub framebuffer_stencil_sample_counts: SampleCountFlags,
    #[doc(alias = "framebufferNoAttachmentsSampleCounts")]
    pub framebuffer_no_attachments_sample_counts: SampleCountFlags,
    #[doc(alias = "maxColorAttachments")]
    pub max_color_attachments: u32,
    #[doc(alias = "sampledImageColorSampleCounts")]
    pub sampled_image_color_sample_counts: SampleCountFlags,
    #[doc(alias = "sampledImageIntegerSampleCounts")]
    pub sampled_image_integer_sample_counts: SampleCountFlags,
    #[doc(alias = "sampledImageDepthSampleCounts")]
    pub sampled_image_depth_sample_counts: SampleCountFlags,
    #[doc(alias = "sampledImageStencilSampleCounts")]
    pub sampled_image_stencil_sample_counts: SampleCountFlags,
    #[doc(alias = "storageImageSampleCounts")]
    pub storage_image_sample_counts: SampleCountFlags,
    #[doc(alias = "maxSampleMaskWords")]
    pub max_sample_mask_words: u32,
    #[doc(alias = "timestampComputeAndGraphics")]
    pub timestamp_compute_and_graphics: Bool32,
    #[doc(alias = "timestampPeriod")]
    pub timestamp_period: f32,
    #[doc(alias = "maxClipDistances")]
    pub max_clip_distances: u32,
    #[doc(alias = "maxCullDistances")]
    pub max_cull_distances: u32,
    #[doc(alias = "maxCombinedClipAndCullDistances")]
    pub max_combined_clip_and_cull_distances: u32,
    #[doc(alias = "discreteQueuePriorities")]
    pub discrete_queue_priorities: u32,
    #[doc(alias = "pointSizeRange")]
    pub point_size_range: [f32; 2 as usize],
    #[doc(alias = "lineWidthRange")]
    pub line_width_range: [f32; 2 as usize],
    #[doc(alias = "pointSizeGranularity")]
    pub point_size_granularity: f32,
    #[doc(alias = "lineWidthGranularity")]
    pub line_width_granularity: f32,
    #[doc(alias = "strictLines")]
    pub strict_lines: Bool32,
    #[doc(alias = "standardSampleLocations")]
    pub standard_sample_locations: Bool32,
    #[doc(alias = "optimalBufferCopyOffsetAlignment")]
    pub optimal_buffer_copy_offset_alignment: DeviceSize,
    #[doc(alias = "optimalBufferCopyRowPitchAlignment")]
    pub optimal_buffer_copy_row_pitch_alignment: DeviceSize,
    #[doc(alias = "nonCoherentAtomSize")]
    pub non_coherent_atom_size: DeviceSize,
}
impl Default for PhysicalDeviceLimits {
    fn default() -> Self {
        Self {
            max_image_dimension1_d: unsafe { std::mem::zeroed() },
            max_image_dimension2_d: unsafe { std::mem::zeroed() },
            max_image_dimension3_d: unsafe { std::mem::zeroed() },
            max_image_dimension_cube: unsafe { std::mem::zeroed() },
            max_image_array_layers: unsafe { std::mem::zeroed() },
            max_texel_buffer_elements: unsafe { std::mem::zeroed() },
            max_uniform_buffer_range: unsafe { std::mem::zeroed() },
            max_storage_buffer_range: unsafe { std::mem::zeroed() },
            max_push_constants_size: unsafe { std::mem::zeroed() },
            max_memory_allocation_count: unsafe { std::mem::zeroed() },
            max_sampler_allocation_count: unsafe { std::mem::zeroed() },
            buffer_image_granularity: unsafe { std::mem::zeroed() },
            sparse_address_space_size: unsafe { std::mem::zeroed() },
            max_bound_descriptor_sets: unsafe { std::mem::zeroed() },
            max_per_stage_descriptor_samplers: unsafe { std::mem::zeroed() },
            max_per_stage_descriptor_uniform_buffers: unsafe { std::mem::zeroed() },
            max_per_stage_descriptor_storage_buffers: unsafe { std::mem::zeroed() },
            max_per_stage_descriptor_sampled_images: unsafe { std::mem::zeroed() },
            max_per_stage_descriptor_storage_images: unsafe { std::mem::zeroed() },
            max_per_stage_descriptor_input_attachments: unsafe { std::mem::zeroed() },
            max_per_stage_resources: unsafe { std::mem::zeroed() },
            max_descriptor_set_samplers: unsafe { std::mem::zeroed() },
            max_descriptor_set_uniform_buffers: unsafe { std::mem::zeroed() },
            max_descriptor_set_uniform_buffers_dynamic: unsafe { std::mem::zeroed() },
            max_descriptor_set_storage_buffers: unsafe { std::mem::zeroed() },
            max_descriptor_set_storage_buffers_dynamic: unsafe { std::mem::zeroed() },
            max_descriptor_set_sampled_images: unsafe { std::mem::zeroed() },
            max_descriptor_set_storage_images: unsafe { std::mem::zeroed() },
            max_descriptor_set_input_attachments: unsafe { std::mem::zeroed() },
            max_vertex_input_attributes: unsafe { std::mem::zeroed() },
            max_vertex_input_bindings: unsafe { std::mem::zeroed() },
            max_vertex_input_attribute_offset: unsafe { std::mem::zeroed() },
            max_vertex_input_binding_stride: unsafe { std::mem::zeroed() },
            max_vertex_output_components: unsafe { std::mem::zeroed() },
            max_tessellation_generation_level: unsafe { std::mem::zeroed() },
            max_tessellation_patch_size: unsafe { std::mem::zeroed() },
            max_tessellation_control_per_vertex_input_components: unsafe { std::mem::zeroed() },
            max_tessellation_control_per_vertex_output_components: unsafe { std::mem::zeroed() },
            max_tessellation_control_per_patch_output_components: unsafe { std::mem::zeroed() },
            max_tessellation_control_total_output_components: unsafe { std::mem::zeroed() },
            max_tessellation_evaluation_input_components: unsafe { std::mem::zeroed() },
            max_tessellation_evaluation_output_components: unsafe { std::mem::zeroed() },
            max_geometry_shader_invocations: unsafe { std::mem::zeroed() },
            max_geometry_input_components: unsafe { std::mem::zeroed() },
            max_geometry_output_components: unsafe { std::mem::zeroed() },
            max_geometry_output_vertices: unsafe { std::mem::zeroed() },
            max_geometry_total_output_components: unsafe { std::mem::zeroed() },
            max_fragment_input_components: unsafe { std::mem::zeroed() },
            max_fragment_output_attachments: unsafe { std::mem::zeroed() },
            max_fragment_dual_src_attachments: unsafe { std::mem::zeroed() },
            max_fragment_combined_output_resources: unsafe { std::mem::zeroed() },
            max_compute_shared_memory_size: unsafe { std::mem::zeroed() },
            max_compute_work_group_count: unsafe { std::mem::zeroed() },
            max_compute_work_group_invocations: unsafe { std::mem::zeroed() },
            max_compute_work_group_size: unsafe { std::mem::zeroed() },
            sub_pixel_precision_bits: unsafe { std::mem::zeroed() },
            sub_texel_precision_bits: unsafe { std::mem::zeroed() },
            mipmap_precision_bits: unsafe { std::mem::zeroed() },
            max_draw_indexed_index_value: unsafe { std::mem::zeroed() },
            max_draw_indirect_count: unsafe { std::mem::zeroed() },
            max_sampler_lod_bias: unsafe { std::mem::zeroed() },
            max_sampler_anisotropy: unsafe { std::mem::zeroed() },
            max_viewports: unsafe { std::mem::zeroed() },
            max_viewport_dimensions: unsafe { std::mem::zeroed() },
            viewport_bounds_range: unsafe { std::mem::zeroed() },
            viewport_sub_pixel_bits: unsafe { std::mem::zeroed() },
            min_memory_map_alignment: unsafe { std::mem::zeroed() },
            min_texel_buffer_offset_alignment: unsafe { std::mem::zeroed() },
            min_uniform_buffer_offset_alignment: unsafe { std::mem::zeroed() },
            min_storage_buffer_offset_alignment: unsafe { std::mem::zeroed() },
            min_texel_offset: unsafe { std::mem::zeroed() },
            max_texel_offset: unsafe { std::mem::zeroed() },
            min_texel_gather_offset: unsafe { std::mem::zeroed() },
            max_texel_gather_offset: unsafe { std::mem::zeroed() },
            min_interpolation_offset: unsafe { std::mem::zeroed() },
            max_interpolation_offset: unsafe { std::mem::zeroed() },
            sub_pixel_interpolation_offset_bits: unsafe { std::mem::zeroed() },
            max_framebuffer_width: unsafe { std::mem::zeroed() },
            max_framebuffer_height: unsafe { std::mem::zeroed() },
            max_framebuffer_layers: unsafe { std::mem::zeroed() },
            framebuffer_color_sample_counts: unsafe { std::mem::zeroed() },
            framebuffer_depth_sample_counts: unsafe { std::mem::zeroed() },
            framebuffer_stencil_sample_counts: unsafe { std::mem::zeroed() },
            framebuffer_no_attachments_sample_counts: unsafe { std::mem::zeroed() },
            max_color_attachments: unsafe { std::mem::zeroed() },
            sampled_image_color_sample_counts: unsafe { std::mem::zeroed() },
            sampled_image_integer_sample_counts: unsafe { std::mem::zeroed() },
            sampled_image_depth_sample_counts: unsafe { std::mem::zeroed() },
            sampled_image_stencil_sample_counts: unsafe { std::mem::zeroed() },
            storage_image_sample_counts: unsafe { std::mem::zeroed() },
            max_sample_mask_words: unsafe { std::mem::zeroed() },
            timestamp_compute_and_graphics: unsafe { std::mem::zeroed() },
            timestamp_period: unsafe { std::mem::zeroed() },
            max_clip_distances: unsafe { std::mem::zeroed() },
            max_cull_distances: unsafe { std::mem::zeroed() },
            max_combined_clip_and_cull_distances: unsafe { std::mem::zeroed() },
            discrete_queue_priorities: unsafe { std::mem::zeroed() },
            point_size_range: unsafe { std::mem::zeroed() },
            line_width_range: unsafe { std::mem::zeroed() },
            point_size_granularity: unsafe { std::mem::zeroed() },
            line_width_granularity: unsafe { std::mem::zeroed() },
            strict_lines: unsafe { std::mem::zeroed() },
            standard_sample_locations: unsafe { std::mem::zeroed() },
            optimal_buffer_copy_offset_alignment: unsafe { std::mem::zeroed() },
            optimal_buffer_copy_row_pitch_alignment: unsafe { std::mem::zeroed() },
            non_coherent_atom_size: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSemaphoreCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SemaphoreCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: SemaphoreCreateFlags,
}
impl Default for SemaphoreCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::SemaphoreCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkQueryPoolCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct QueryPoolCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: QueryPoolCreateFlags,
    #[doc(alias = "queryType")]
    pub query_type: QueryType,
    #[doc(alias = "queryCount")]
    pub query_count: u32,
    #[doc(alias = "pipelineStatistics")]
    pub pipeline_statistics: QueryPipelineStatisticFlags,
}
impl Default for QueryPoolCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::QueryPoolCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            query_type: unsafe { std::mem::zeroed() },
            query_count: unsafe { std::mem::zeroed() },
            pipeline_statistics: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkFramebufferCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct FramebufferCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: FramebufferCreateFlags,
    #[doc(alias = "renderPass")]
    pub render_pass: RenderPass,
    #[doc(alias = "attachmentCount")]
    pub attachment_count: u32,
    #[doc(alias = "pAttachments")]
    pub attachments: *const ImageView,
    pub width: u32,
    pub height: u32,
    pub layers: u32,
}
impl Default for FramebufferCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::FramebufferCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            render_pass: unsafe { std::mem::zeroed() },
            attachment_count: unsafe { std::mem::zeroed() },
            attachments: unsafe { std::mem::zeroed() },
            width: unsafe { std::mem::zeroed() },
            height: unsafe { std::mem::zeroed() },
            layers: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSubmitInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SubmitInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "waitSemaphoreCount")]
    pub wait_semaphore_count: u32,
    #[doc(alias = "pWaitSemaphores")]
    pub wait_semaphores: *const Semaphore,
    #[doc(alias = "pWaitDstStageMask")]
    pub wait_dst_stage_mask: *const PipelineStageFlags,
    #[doc(alias = "commandBufferCount")]
    pub command_buffer_count: u32,
    #[doc(alias = "pCommandBuffers")]
    pub command_buffers: *const CommandBuffer,
    #[doc(alias = "signalSemaphoreCount")]
    pub signal_semaphore_count: u32,
    #[doc(alias = "pSignalSemaphores")]
    pub signal_semaphores: *const Semaphore,
}
impl Default for SubmitInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::SubmitInfo,
            p_next: unsafe { std::mem::zeroed() },
            wait_semaphore_count: unsafe { std::mem::zeroed() },
            wait_semaphores: unsafe { std::mem::zeroed() },
            wait_dst_stage_mask: unsafe { std::mem::zeroed() },
            command_buffer_count: unsafe { std::mem::zeroed() },
            command_buffers: unsafe { std::mem::zeroed() },
            signal_semaphore_count: unsafe { std::mem::zeroed() },
            signal_semaphores: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkClearColorValue")]
#[repr(C)]
#[derive(Clone, Copy)]
pub union ClearColorValue {
    pub float32: [f32; 4 as usize],
    pub int32: [i32; 4 as usize],
    pub uint32: [u32; 4 as usize],
}
#[doc(alias = "VkClearValue")]
#[repr(C)]
#[derive(Clone, Copy)]
pub union ClearValue {
    pub color: ClearColorValue,
    #[doc(alias = "depthStencil")]
    pub depth_stencil: ClearDepthStencilValue,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkInstance")]
#[repr(transparent)]
pub struct Instance(*mut std::ffi::c_void);
impl Instance {
    pub const fn null() -> Self {
        Self(::std::ptr::null_mut() as _)
    }
    pub const fn raw(&self) -> *mut std::ffi::c_void {
        self.0
    }
}
impl Default for Instance {
    fn default() -> Self {
        Self::null()
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkPhysicalDevice")]
#[repr(transparent)]
pub struct PhysicalDevice(*mut std::ffi::c_void);
impl PhysicalDevice {
    pub const fn null() -> Self {
        Self(::std::ptr::null_mut() as _)
    }
    pub const fn raw(&self) -> *mut std::ffi::c_void {
        self.0
    }
}
impl Default for PhysicalDevice {
    fn default() -> Self {
        Self::null()
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkDevice")]
#[repr(transparent)]
pub struct Device(*mut std::ffi::c_void);
impl Device {
    pub const fn null() -> Self {
        Self(::std::ptr::null_mut() as _)
    }
    pub const fn raw(&self) -> *mut std::ffi::c_void {
        self.0
    }
}
impl Default for Device {
    fn default() -> Self {
        Self::null()
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkQueue")]
#[repr(transparent)]
pub struct Queue(*mut std::ffi::c_void);
impl Queue {
    pub const fn null() -> Self {
        Self(::std::ptr::null_mut() as _)
    }
    pub const fn raw(&self) -> *mut std::ffi::c_void {
        self.0
    }
}
impl Default for Queue {
    fn default() -> Self {
        Self::null()
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkCommandBuffer")]
#[repr(transparent)]
pub struct CommandBuffer(*mut std::ffi::c_void);
impl CommandBuffer {
    pub const fn null() -> Self {
        Self(::std::ptr::null_mut() as _)
    }
    pub const fn raw(&self) -> *mut std::ffi::c_void {
        self.0
    }
}
impl Default for CommandBuffer {
    fn default() -> Self {
        Self::null()
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkDeviceMemory")]
#[repr(transparent)]
pub struct DeviceMemory(u64);
impl DeviceMemory {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for DeviceMemory {
    fn default() -> Self {
        Self::null()
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkCommandPool")]
#[repr(transparent)]
pub struct CommandPool(u64);
impl CommandPool {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for CommandPool {
    fn default() -> Self {
        Self::null()
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkBuffer")]
#[repr(transparent)]
pub struct Buffer(u64);
impl Buffer {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for Buffer {
    fn default() -> Self {
        Self::null()
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkBufferView")]
#[repr(transparent)]
pub struct BufferView(u64);
impl BufferView {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for BufferView {
    fn default() -> Self {
        Self::null()
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkImage")]
#[repr(transparent)]
pub struct Image(u64);
impl Image {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for Image {
    fn default() -> Self {
        Self::null()
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkImageView")]
#[repr(transparent)]
pub struct ImageView(u64);
impl ImageView {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for ImageView {
    fn default() -> Self {
        Self::null()
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkShaderModule")]
#[repr(transparent)]
pub struct ShaderModule(u64);
impl ShaderModule {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for ShaderModule {
    fn default() -> Self {
        Self::null()
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkPipeline")]
#[repr(transparent)]
pub struct Pipeline(u64);
impl Pipeline {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for Pipeline {
    fn default() -> Self {
        Self::null()
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkPipelineLayout")]
#[repr(transparent)]
pub struct PipelineLayout(u64);
impl PipelineLayout {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for PipelineLayout {
    fn default() -> Self {
        Self::null()
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkSampler")]
#[repr(transparent)]
pub struct Sampler(u64);
impl Sampler {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for Sampler {
    fn default() -> Self {
        Self::null()
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkDescriptorSet")]
#[repr(transparent)]
pub struct DescriptorSet(u64);
impl DescriptorSet {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for DescriptorSet {
    fn default() -> Self {
        Self::null()
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkDescriptorSetLayout")]
#[repr(transparent)]
pub struct DescriptorSetLayout(u64);
impl DescriptorSetLayout {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for DescriptorSetLayout {
    fn default() -> Self {
        Self::null()
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkDescriptorPool")]
#[repr(transparent)]
pub struct DescriptorPool(u64);
impl DescriptorPool {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for DescriptorPool {
    fn default() -> Self {
        Self::null()
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkFence")]
#[repr(transparent)]
pub struct Fence(u64);
impl Fence {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for Fence {
    fn default() -> Self {
        Self::null()
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkSemaphore")]
#[repr(transparent)]
pub struct Semaphore(u64);
impl Semaphore {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for Semaphore {
    fn default() -> Self {
        Self::null()
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkEvent")]
#[repr(transparent)]
pub struct Event(u64);
impl Event {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for Event {
    fn default() -> Self {
        Self::null()
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkQueryPool")]
#[repr(transparent)]
pub struct QueryPool(u64);
impl QueryPool {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for QueryPool {
    fn default() -> Self {
        Self::null()
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkFramebuffer")]
#[repr(transparent)]
pub struct Framebuffer(u64);
impl Framebuffer {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for Framebuffer {
    fn default() -> Self {
        Self::null()
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkRenderPass")]
#[repr(transparent)]
pub struct RenderPass(u64);
impl RenderPass {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for RenderPass {
    fn default() -> Self {
        Self::null()
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkPipelineCache")]
#[repr(transparent)]
pub struct PipelineCache(u64);
impl PipelineCache {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for PipelineCache {
    fn default() -> Self {
        Self::null()
    }
}
#[doc(alias = "PFN_vkInternalAllocationNotification")]
pub type PFNInternalAllocationNotification = unsafe extern "system" fn(
    p_user_data: *mut std::ffi::c_void,
    size: usize,
    allocation_type: InternalAllocationType,
    allocation_scope: SystemAllocationScope,
);
#[doc(alias = "PFN_vkInternalFreeNotification")]
pub type PFNInternalFreeNotification = unsafe extern "system" fn(
    p_user_data: *mut std::ffi::c_void,
    size: usize,
    allocation_type: InternalAllocationType,
    allocation_scope: SystemAllocationScope,
);
#[doc(alias = "PFN_vkReallocationFunction")]
pub type PFNReallocationFunction = unsafe extern "system" fn(
    p_user_data: *mut std::ffi::c_void,
    p_original: *mut std::ffi::c_void,
    size: usize,
    alignment: usize,
    allocation_scope: SystemAllocationScope,
) -> *mut std::ffi::c_void;
#[doc(alias = "PFN_vkAllocationFunction")]
pub type PFNAllocationFunction = unsafe extern "system" fn(
    p_user_data: *mut std::ffi::c_void,
    size: usize,
    alignment: usize,
    allocation_scope: SystemAllocationScope,
) -> *mut std::ffi::c_void;
#[doc(alias = "PFN_vkFreeFunction")]
pub type PFNFreeFunction =
    unsafe extern "system" fn(p_user_data: *mut std::ffi::c_void, p_memory: *mut std::ffi::c_void);
#[doc(alias = "PFN_vkVoidFunction")]
pub type PFNVoidFunction = unsafe extern "system" fn();
pub use crate::common::vulkan1_0::{
    AccessFlagBits, AccessFlags, AttachmentDescriptionFlagBits, AttachmentDescriptionFlags, AttachmentLoadOp,
    AttachmentStoreOp, BlendFactor, BlendOp, Bool32, BorderColor, BufferCreateFlagBits, BufferCreateFlags,
    BufferUsageFlagBits, BufferUsageFlags, BufferViewCreateFlags, ColorComponentFlagBits, ColorComponentFlags,
    CommandBufferLevel, CommandBufferResetFlagBits, CommandBufferResetFlags, CommandBufferUsageFlagBits,
    CommandBufferUsageFlags, CommandPoolCreateFlagBits, CommandPoolCreateFlags, CommandPoolResetFlagBits,
    CommandPoolResetFlags, CompareOp, ComponentSwizzle, CullModeFlagBits, CullModeFlags, DependencyFlagBits,
    DependencyFlags, DescriptorPoolCreateFlagBits, DescriptorPoolCreateFlags, DescriptorPoolResetFlags,
    DescriptorSetLayoutCreateFlagBits, DescriptorSetLayoutCreateFlags, DescriptorType, DeviceAddress,
    DeviceCreateFlags, DeviceQueueCreateFlags, DeviceSize, DynamicState, EventCreateFlagBits, EventCreateFlags,
    FenceCreateFlagBits, FenceCreateFlags, Filter, Flags, Format, FormatFeatureFlagBits, FormatFeatureFlags,
    FramebufferCreateFlagBits, FramebufferCreateFlags, FrontFace, ImageAspectFlagBits, ImageAspectFlags,
    ImageCreateFlagBits, ImageCreateFlags, ImageLayout, ImageTiling, ImageType, ImageUsageFlagBits, ImageUsageFlags,
    ImageViewCreateFlagBits, ImageViewCreateFlags, ImageViewType, IndexType, InstanceCreateFlagBits,
    InstanceCreateFlags, InternalAllocationType, LogicOp, MemoryHeapFlagBits, MemoryHeapFlags, MemoryMapFlags,
    MemoryPropertyFlagBits, MemoryPropertyFlags, ObjectType, PhysicalDeviceType, PipelineBindPoint,
    PipelineCacheCreateFlags, PipelineCacheHeaderVersion, PipelineColorBlendStateCreateFlags, PipelineCreateFlagBits,
    PipelineCreateFlags, PipelineDepthStencilStateCreateFlags, PipelineDynamicStateCreateFlags,
    PipelineInputAssemblyStateCreateFlags, PipelineLayoutCreateFlags, PipelineMultisampleStateCreateFlags,
    PipelineRasterizationStateCreateFlags, PipelineShaderStageCreateFlagBits, PipelineShaderStageCreateFlags,
    PipelineStageFlagBits, PipelineStageFlags, PipelineTessellationStateCreateFlags,
    PipelineVertexInputStateCreateFlags, PipelineViewportStateCreateFlags, PolygonMode, PrimitiveTopology,
    QueryControlFlagBits, QueryControlFlags, QueryPipelineStatisticFlagBits, QueryPipelineStatisticFlags,
    QueryPoolCreateFlags, QueryResultFlagBits, QueryResultFlags, QueryType, QueueFlagBits, QueueFlags,
    RenderPassCreateFlagBits, RenderPassCreateFlags, SampleCountFlagBits, SampleCountFlags, SampleMask,
    SamplerAddressMode, SamplerCreateFlagBits, SamplerCreateFlags, SamplerMipmapMode, SemaphoreCreateFlags,
    ShaderModuleCreateFlags, ShaderStageFlagBits, ShaderStageFlags, SharingMode, SparseImageFormatFlagBits,
    SparseImageFormatFlags, SparseMemoryBindFlagBits, SparseMemoryBindFlags, StencilFaceFlagBits, StencilFaceFlags,
    StencilOp, StructureType, SubpassContents, SubpassDescriptionFlagBits, SubpassDescriptionFlags,
    SystemAllocationScope, VendorId, VertexInputRate, VulkanResultCodes, ATTACHMENT_UNUSED, FALSE, LOD_CLAMP_NONE,
    LUID_SIZE, MAX_DESCRIPTION_SIZE, MAX_DEVICE_GROUP_SIZE, MAX_DRIVER_INFO_SIZE, MAX_DRIVER_NAME_SIZE,
    MAX_EXTENSION_NAME_SIZE, MAX_GLOBAL_PRIORITY_SIZE_KHR, MAX_MEMORY_HEAPS, MAX_MEMORY_TYPES,
    MAX_PHYSICAL_DEVICE_NAME_SIZE, QUEUE_FAMILY_EXTERNAL, QUEUE_FAMILY_FOREIGN_EXT, QUEUE_FAMILY_IGNORED,
    REMAINING_ARRAY_LAYERS, REMAINING_MIP_LEVELS, SHADER_UNUSED_KHR, SUBPASS_EXTERNAL, TRUE, UUID_SIZE, WHOLE_SIZE,
};
#[doc(alias = "vkCreateInstance")]
pub type FNCreateInstance = unsafe extern "system" fn(
    p_create_info: *const InstanceCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_instance: *mut Instance,
) -> VulkanResultCodes;
#[doc(alias = "vkDestroyInstance")]
pub type FNDestroyInstance = unsafe extern "system" fn(instance: Instance, p_allocator: *const AllocationCallbacks);
#[doc(alias = "vkEnumeratePhysicalDevices")]
pub type FNEnumeratePhysicalDevices = unsafe extern "system" fn(
    instance: Instance,
    p_physical_device_count: *mut u32,
    p_physical_devices: *mut PhysicalDevice,
) -> VulkanResultCodes;
#[doc(alias = "vkGetDeviceProcAddr")]
pub type FNGetDeviceProcAddr = unsafe extern "system" fn(device: Device, p_name: *const c_char) -> PFNVoidFunction;
#[doc(alias = "vkGetInstanceProcAddr")]
pub type FNGetInstanceProcAddr =
    unsafe extern "system" fn(instance: Instance, p_name: *const c_char) -> PFNVoidFunction;
#[doc(alias = "vkGetPhysicalDeviceProperties")]
pub type FNGetPhysicalDeviceProperties =
    unsafe extern "system" fn(physical_device: PhysicalDevice, p_properties: *mut PhysicalDeviceProperties);
#[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties")]
pub type FNGetPhysicalDeviceQueueFamilyProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_queue_family_property_count: *mut u32,
    p_queue_family_properties: *mut QueueFamilyProperties,
);
#[doc(alias = "vkGetPhysicalDeviceMemoryProperties")]
pub type FNGetPhysicalDeviceMemoryProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_memory_properties: *mut PhysicalDeviceMemoryProperties,
);
#[doc(alias = "vkGetPhysicalDeviceFeatures")]
pub type FNGetPhysicalDeviceFeatures =
    unsafe extern "system" fn(physical_device: PhysicalDevice, p_features: *mut PhysicalDeviceFeatures);
#[doc(alias = "vkGetPhysicalDeviceFormatProperties")]
pub type FNGetPhysicalDeviceFormatProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    format: Format,
    p_format_properties: *mut FormatProperties,
);
#[doc(alias = "vkGetPhysicalDeviceImageFormatProperties")]
pub type FNGetPhysicalDeviceImageFormatProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    format: Format,
    type_: ImageType,
    tiling: ImageTiling,
    usage: ImageUsageFlags,
    flags: ImageCreateFlags,
    p_image_format_properties: *mut ImageFormatProperties,
) -> VulkanResultCodes;
#[doc(alias = "vkCreateDevice")]
pub type FNCreateDevice = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_create_info: *const DeviceCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_device: *mut Device,
) -> VulkanResultCodes;
#[doc(alias = "vkDestroyDevice")]
pub type FNDestroyDevice = unsafe extern "system" fn(device: Device, p_allocator: *const AllocationCallbacks);
#[doc(alias = "vkEnumerateInstanceLayerProperties")]
pub type FNEnumerateInstanceLayerProperties =
    unsafe extern "system" fn(p_property_count: *mut u32, p_properties: *mut LayerProperties) -> VulkanResultCodes;
#[doc(alias = "vkEnumerateInstanceExtensionProperties")]
pub type FNEnumerateInstanceExtensionProperties = unsafe extern "system" fn(
    p_layer_name: *const c_char,
    p_property_count: *mut u32,
    p_properties: *mut ExtensionProperties,
) -> VulkanResultCodes;
#[doc(alias = "vkEnumerateDeviceLayerProperties")]
pub type FNEnumerateDeviceLayerProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut LayerProperties,
) -> VulkanResultCodes;
#[doc(alias = "vkEnumerateDeviceExtensionProperties")]
pub type FNEnumerateDeviceExtensionProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_layer_name: *const c_char,
    p_property_count: *mut u32,
    p_properties: *mut ExtensionProperties,
) -> VulkanResultCodes;
#[doc(alias = "vkGetDeviceQueue")]
pub type FNGetDeviceQueue =
    unsafe extern "system" fn(device: Device, queue_family_index: u32, queue_index: u32, p_queue: *mut Queue);
#[doc(alias = "vkQueueSubmit")]
pub type FNQueueSubmit = unsafe extern "system" fn(
    queue: Queue,
    submit_count: u32,
    p_submits: *const SubmitInfo,
    fence: Fence,
) -> VulkanResultCodes;
#[doc(alias = "vkQueueWaitIdle")]
pub type FNQueueWaitIdle = unsafe extern "system" fn(queue: Queue) -> VulkanResultCodes;
#[doc(alias = "vkDeviceWaitIdle")]
pub type FNDeviceWaitIdle = unsafe extern "system" fn(device: Device) -> VulkanResultCodes;
#[doc(alias = "vkAllocateMemory")]
pub type FNAllocateMemory = unsafe extern "system" fn(
    device: Device,
    p_allocate_info: *const MemoryAllocateInfo,
    p_allocator: *const AllocationCallbacks,
    p_memory: *mut DeviceMemory,
) -> VulkanResultCodes;
#[doc(alias = "vkFreeMemory")]
pub type FNFreeMemory =
    unsafe extern "system" fn(device: Device, memory: DeviceMemory, p_allocator: *const AllocationCallbacks);
#[doc(alias = "vkMapMemory")]
pub type FNMapMemory = unsafe extern "system" fn(
    device: Device,
    memory: DeviceMemory,
    offset: DeviceSize,
    size: DeviceSize,
    flags: MemoryMapFlags,
    pp_data: *mut *mut std::ffi::c_void,
) -> VulkanResultCodes;
#[doc(alias = "vkUnmapMemory")]
pub type FNUnmapMemory = unsafe extern "system" fn(device: Device, memory: DeviceMemory);
#[doc(alias = "vkFlushMappedMemoryRanges")]
pub type FNFlushMappedMemoryRanges = unsafe extern "system" fn(
    device: Device,
    memory_range_count: u32,
    p_memory_ranges: *const MappedMemoryRange,
) -> VulkanResultCodes;
#[doc(alias = "vkInvalidateMappedMemoryRanges")]
pub type FNInvalidateMappedMemoryRanges = unsafe extern "system" fn(
    device: Device,
    memory_range_count: u32,
    p_memory_ranges: *const MappedMemoryRange,
) -> VulkanResultCodes;
#[doc(alias = "vkGetDeviceMemoryCommitment")]
pub type FNGetDeviceMemoryCommitment =
    unsafe extern "system" fn(device: Device, memory: DeviceMemory, p_committed_memory_in_bytes: *mut DeviceSize);
#[doc(alias = "vkGetBufferMemoryRequirements")]
pub type FNGetBufferMemoryRequirements =
    unsafe extern "system" fn(device: Device, buffer: Buffer, p_memory_requirements: *mut MemoryRequirements);
#[doc(alias = "vkBindBufferMemory")]
pub type FNBindBufferMemory = unsafe extern "system" fn(
    device: Device,
    buffer: Buffer,
    memory: DeviceMemory,
    memory_offset: DeviceSize,
) -> VulkanResultCodes;
#[doc(alias = "vkGetImageMemoryRequirements")]
pub type FNGetImageMemoryRequirements =
    unsafe extern "system" fn(device: Device, image: Image, p_memory_requirements: *mut MemoryRequirements);
#[doc(alias = "vkBindImageMemory")]
pub type FNBindImageMemory = unsafe extern "system" fn(
    device: Device,
    image: Image,
    memory: DeviceMemory,
    memory_offset: DeviceSize,
) -> VulkanResultCodes;
#[doc(alias = "vkGetImageSparseMemoryRequirements")]
pub type FNGetImageSparseMemoryRequirements = unsafe extern "system" fn(
    device: Device,
    image: Image,
    p_sparse_memory_requirement_count: *mut u32,
    p_sparse_memory_requirements: *mut SparseImageMemoryRequirements,
);
#[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties")]
pub type FNGetPhysicalDeviceSparseImageFormatProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    format: Format,
    type_: ImageType,
    samples: SampleCountFlagBits,
    usage: ImageUsageFlags,
    tiling: ImageTiling,
    p_property_count: *mut u32,
    p_properties: *mut SparseImageFormatProperties,
);
#[doc(alias = "vkQueueBindSparse")]
pub type FNQueueBindSparse = unsafe extern "system" fn(
    queue: Queue,
    bind_info_count: u32,
    p_bind_info: *const BindSparseInfo,
    fence: Fence,
) -> VulkanResultCodes;
#[doc(alias = "vkCreateFence")]
pub type FNCreateFence = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const FenceCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_fence: *mut Fence,
) -> VulkanResultCodes;
#[doc(alias = "vkDestroyFence")]
pub type FNDestroyFence =
    unsafe extern "system" fn(device: Device, fence: Fence, p_allocator: *const AllocationCallbacks);
#[doc(alias = "vkResetFences")]
pub type FNResetFences =
    unsafe extern "system" fn(device: Device, fence_count: u32, p_fences: *const Fence) -> VulkanResultCodes;
#[doc(alias = "vkGetFenceStatus")]
pub type FNGetFenceStatus = unsafe extern "system" fn(device: Device, fence: Fence) -> VulkanResultCodes;
#[doc(alias = "vkWaitForFences")]
pub type FNWaitForFences = unsafe extern "system" fn(
    device: Device,
    fence_count: u32,
    p_fences: *const Fence,
    wait_all: Bool32,
    timeout: u64,
) -> VulkanResultCodes;
#[doc(alias = "vkCreateSemaphore")]
pub type FNCreateSemaphore = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const SemaphoreCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_semaphore: *mut Semaphore,
) -> VulkanResultCodes;
#[doc(alias = "vkDestroySemaphore")]
pub type FNDestroySemaphore =
    unsafe extern "system" fn(device: Device, semaphore: Semaphore, p_allocator: *const AllocationCallbacks);
#[doc(alias = "vkCreateEvent")]
pub type FNCreateEvent = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const EventCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_event: *mut Event,
) -> VulkanResultCodes;
#[doc(alias = "vkDestroyEvent")]
pub type FNDestroyEvent =
    unsafe extern "system" fn(device: Device, event: Event, p_allocator: *const AllocationCallbacks);
#[doc(alias = "vkGetEventStatus")]
pub type FNGetEventStatus = unsafe extern "system" fn(device: Device, event: Event) -> VulkanResultCodes;
#[doc(alias = "vkSetEvent")]
pub type FNSetEvent = unsafe extern "system" fn(device: Device, event: Event) -> VulkanResultCodes;
#[doc(alias = "vkResetEvent")]
pub type FNResetEvent = unsafe extern "system" fn(device: Device, event: Event) -> VulkanResultCodes;
#[doc(alias = "vkCreateQueryPool")]
pub type FNCreateQueryPool = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const QueryPoolCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_query_pool: *mut QueryPool,
) -> VulkanResultCodes;
#[doc(alias = "vkDestroyQueryPool")]
pub type FNDestroyQueryPool =
    unsafe extern "system" fn(device: Device, query_pool: QueryPool, p_allocator: *const AllocationCallbacks);
#[doc(alias = "vkGetQueryPoolResults")]
pub type FNGetQueryPoolResults = unsafe extern "system" fn(
    device: Device,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
    data_size: usize,
    p_data: *mut std::ffi::c_void,
    stride: DeviceSize,
    flags: QueryResultFlags,
) -> VulkanResultCodes;
#[doc(alias = "vkCreateBuffer")]
pub type FNCreateBuffer = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const BufferCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_buffer: *mut Buffer,
) -> VulkanResultCodes;
#[doc(alias = "vkDestroyBuffer")]
pub type FNDestroyBuffer =
    unsafe extern "system" fn(device: Device, buffer: Buffer, p_allocator: *const AllocationCallbacks);
#[doc(alias = "vkCreateBufferView")]
pub type FNCreateBufferView = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const BufferViewCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_view: *mut BufferView,
) -> VulkanResultCodes;
#[doc(alias = "vkDestroyBufferView")]
pub type FNDestroyBufferView =
    unsafe extern "system" fn(device: Device, buffer_view: BufferView, p_allocator: *const AllocationCallbacks);
#[doc(alias = "vkCreateImage")]
pub type FNCreateImage = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const ImageCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_image: *mut Image,
) -> VulkanResultCodes;
#[doc(alias = "vkDestroyImage")]
pub type FNDestroyImage =
    unsafe extern "system" fn(device: Device, image: Image, p_allocator: *const AllocationCallbacks);
#[doc(alias = "vkGetImageSubresourceLayout")]
pub type FNGetImageSubresourceLayout = unsafe extern "system" fn(
    device: Device,
    image: Image,
    p_subresource: *const ImageSubresource,
    p_layout: *mut SubresourceLayout,
);
#[doc(alias = "vkCreateImageView")]
pub type FNCreateImageView = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const ImageViewCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_view: *mut ImageView,
) -> VulkanResultCodes;
#[doc(alias = "vkDestroyImageView")]
pub type FNDestroyImageView =
    unsafe extern "system" fn(device: Device, image_view: ImageView, p_allocator: *const AllocationCallbacks);
#[doc(alias = "vkCreateShaderModule")]
pub type FNCreateShaderModule = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const ShaderModuleCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_shader_module: *mut ShaderModule,
) -> VulkanResultCodes;
#[doc(alias = "vkDestroyShaderModule")]
pub type FNDestroyShaderModule =
    unsafe extern "system" fn(device: Device, shader_module: ShaderModule, p_allocator: *const AllocationCallbacks);
#[doc(alias = "vkCreatePipelineCache")]
pub type FNCreatePipelineCache = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const PipelineCacheCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_pipeline_cache: *mut PipelineCache,
) -> VulkanResultCodes;
#[doc(alias = "vkDestroyPipelineCache")]
pub type FNDestroyPipelineCache =
    unsafe extern "system" fn(device: Device, pipeline_cache: PipelineCache, p_allocator: *const AllocationCallbacks);
#[doc(alias = "vkGetPipelineCacheData")]
pub type FNGetPipelineCacheData = unsafe extern "system" fn(
    device: Device,
    pipeline_cache: PipelineCache,
    p_data_size: *mut usize,
    p_data: *mut std::ffi::c_void,
) -> VulkanResultCodes;
#[doc(alias = "vkMergePipelineCaches")]
pub type FNMergePipelineCaches = unsafe extern "system" fn(
    device: Device,
    dst_cache: PipelineCache,
    src_cache_count: u32,
    p_src_caches: *const PipelineCache,
) -> VulkanResultCodes;
#[doc(alias = "vkCreateGraphicsPipelines")]
pub type FNCreateGraphicsPipelines = unsafe extern "system" fn(
    device: Device,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    p_create_infos: *const GraphicsPipelineCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_pipelines: *mut Pipeline,
) -> VulkanResultCodes;
#[doc(alias = "vkCreateComputePipelines")]
pub type FNCreateComputePipelines = unsafe extern "system" fn(
    device: Device,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    p_create_infos: *const ComputePipelineCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_pipelines: *mut Pipeline,
) -> VulkanResultCodes;
#[doc(alias = "vkDestroyPipeline")]
pub type FNDestroyPipeline =
    unsafe extern "system" fn(device: Device, pipeline: Pipeline, p_allocator: *const AllocationCallbacks);
#[doc(alias = "vkCreatePipelineLayout")]
pub type FNCreatePipelineLayout = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const PipelineLayoutCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_pipeline_layout: *mut PipelineLayout,
) -> VulkanResultCodes;
#[doc(alias = "vkDestroyPipelineLayout")]
pub type FNDestroyPipelineLayout =
    unsafe extern "system" fn(device: Device, pipeline_layout: PipelineLayout, p_allocator: *const AllocationCallbacks);
#[doc(alias = "vkCreateSampler")]
pub type FNCreateSampler = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const SamplerCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_sampler: *mut Sampler,
) -> VulkanResultCodes;
#[doc(alias = "vkDestroySampler")]
pub type FNDestroySampler =
    unsafe extern "system" fn(device: Device, sampler: Sampler, p_allocator: *const AllocationCallbacks);
#[doc(alias = "vkCreateDescriptorSetLayout")]
pub type FNCreateDescriptorSetLayout = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const DescriptorSetLayoutCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_set_layout: *mut DescriptorSetLayout,
) -> VulkanResultCodes;
#[doc(alias = "vkDestroyDescriptorSetLayout")]
pub type FNDestroyDescriptorSetLayout = unsafe extern "system" fn(
    device: Device,
    descriptor_set_layout: DescriptorSetLayout,
    p_allocator: *const AllocationCallbacks,
);
#[doc(alias = "vkCreateDescriptorPool")]
pub type FNCreateDescriptorPool = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const DescriptorPoolCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_descriptor_pool: *mut DescriptorPool,
) -> VulkanResultCodes;
#[doc(alias = "vkDestroyDescriptorPool")]
pub type FNDestroyDescriptorPool =
    unsafe extern "system" fn(device: Device, descriptor_pool: DescriptorPool, p_allocator: *const AllocationCallbacks);
#[doc(alias = "vkResetDescriptorPool")]
pub type FNResetDescriptorPool = unsafe extern "system" fn(
    device: Device,
    descriptor_pool: DescriptorPool,
    flags: DescriptorPoolResetFlags,
) -> VulkanResultCodes;
#[doc(alias = "vkAllocateDescriptorSets")]
pub type FNAllocateDescriptorSets = unsafe extern "system" fn(
    device: Device,
    p_allocate_info: *const DescriptorSetAllocateInfo,
    p_descriptor_sets: *mut DescriptorSet,
) -> VulkanResultCodes;
#[doc(alias = "vkFreeDescriptorSets")]
pub type FNFreeDescriptorSets = unsafe extern "system" fn(
    device: Device,
    descriptor_pool: DescriptorPool,
    descriptor_set_count: u32,
    p_descriptor_sets: *const DescriptorSet,
) -> VulkanResultCodes;
#[doc(alias = "vkUpdateDescriptorSets")]
pub type FNUpdateDescriptorSets = unsafe extern "system" fn(
    device: Device,
    descriptor_write_count: u32,
    p_descriptor_writes: *const WriteDescriptorSet,
    descriptor_copy_count: u32,
    p_descriptor_copies: *const CopyDescriptorSet,
);
#[doc(alias = "vkCreateFramebuffer")]
pub type FNCreateFramebuffer = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const FramebufferCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_framebuffer: *mut Framebuffer,
) -> VulkanResultCodes;
#[doc(alias = "vkDestroyFramebuffer")]
pub type FNDestroyFramebuffer =
    unsafe extern "system" fn(device: Device, framebuffer: Framebuffer, p_allocator: *const AllocationCallbacks);
#[doc(alias = "vkCreateRenderPass")]
pub type FNCreateRenderPass = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const RenderPassCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_render_pass: *mut RenderPass,
) -> VulkanResultCodes;
#[doc(alias = "vkDestroyRenderPass")]
pub type FNDestroyRenderPass =
    unsafe extern "system" fn(device: Device, render_pass: RenderPass, p_allocator: *const AllocationCallbacks);
#[doc(alias = "vkGetRenderAreaGranularity")]
pub type FNGetRenderAreaGranularity =
    unsafe extern "system" fn(device: Device, render_pass: RenderPass, p_granularity: *mut Extent2D);
#[doc(alias = "vkCreateCommandPool")]
pub type FNCreateCommandPool = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const CommandPoolCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_command_pool: *mut CommandPool,
) -> VulkanResultCodes;
#[doc(alias = "vkDestroyCommandPool")]
pub type FNDestroyCommandPool =
    unsafe extern "system" fn(device: Device, command_pool: CommandPool, p_allocator: *const AllocationCallbacks);
#[doc(alias = "vkResetCommandPool")]
pub type FNResetCommandPool = unsafe extern "system" fn(
    device: Device,
    command_pool: CommandPool,
    flags: CommandPoolResetFlags,
) -> VulkanResultCodes;
#[doc(alias = "vkAllocateCommandBuffers")]
pub type FNAllocateCommandBuffers = unsafe extern "system" fn(
    device: Device,
    p_allocate_info: *const CommandBufferAllocateInfo,
    p_command_buffers: *mut CommandBuffer,
) -> VulkanResultCodes;
#[doc(alias = "vkFreeCommandBuffers")]
pub type FNFreeCommandBuffers = unsafe extern "system" fn(
    device: Device,
    command_pool: CommandPool,
    command_buffer_count: u32,
    p_command_buffers: *const CommandBuffer,
);
#[doc(alias = "vkBeginCommandBuffer")]
pub type FNBeginCommandBuffer = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_begin_info: *const CommandBufferBeginInfo,
) -> VulkanResultCodes;
#[doc(alias = "vkEndCommandBuffer")]
pub type FNEndCommandBuffer = unsafe extern "system" fn(command_buffer: CommandBuffer) -> VulkanResultCodes;
#[doc(alias = "vkResetCommandBuffer")]
pub type FNResetCommandBuffer =
    unsafe extern "system" fn(command_buffer: CommandBuffer, flags: CommandBufferResetFlags) -> VulkanResultCodes;
#[doc(alias = "vkCmdBindPipeline")]
pub type FNCmdBindPipeline = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    pipeline: Pipeline,
);
#[doc(alias = "vkCmdSetViewport")]
pub type FNCmdSetViewport = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_viewport: u32,
    viewport_count: u32,
    p_viewports: *const Viewport,
);
#[doc(alias = "vkCmdSetScissor")]
pub type FNCmdSetScissor = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_scissor: u32,
    scissor_count: u32,
    p_scissors: *const Rect2D,
);
#[doc(alias = "vkCmdSetLineWidth")]
pub type FNCmdSetLineWidth = unsafe extern "system" fn(command_buffer: CommandBuffer, line_width: f32);
#[doc(alias = "vkCmdSetDepthBias")]
pub type FNCmdSetDepthBias = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    depth_bias_constant_factor: f32,
    depth_bias_clamp: f32,
    depth_bias_slope_factor: f32,
);
#[doc(alias = "vkCmdSetBlendConstants")]
pub type FNCmdSetBlendConstants =
    unsafe extern "system" fn(command_buffer: CommandBuffer, blend_constants: [f32; 4 as usize]);
#[doc(alias = "vkCmdSetDepthBounds")]
pub type FNCmdSetDepthBounds =
    unsafe extern "system" fn(command_buffer: CommandBuffer, min_depth_bounds: f32, max_depth_bounds: f32);
#[doc(alias = "vkCmdSetStencilCompareMask")]
pub type FNCmdSetStencilCompareMask =
    unsafe extern "system" fn(command_buffer: CommandBuffer, face_mask: StencilFaceFlags, compare_mask: u32);
#[doc(alias = "vkCmdSetStencilWriteMask")]
pub type FNCmdSetStencilWriteMask =
    unsafe extern "system" fn(command_buffer: CommandBuffer, face_mask: StencilFaceFlags, write_mask: u32);
#[doc(alias = "vkCmdSetStencilReference")]
pub type FNCmdSetStencilReference =
    unsafe extern "system" fn(command_buffer: CommandBuffer, face_mask: StencilFaceFlags, reference: u32);
#[doc(alias = "vkCmdBindDescriptorSets")]
pub type FNCmdBindDescriptorSets = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    layout: PipelineLayout,
    first_set: u32,
    descriptor_set_count: u32,
    p_descriptor_sets: *const DescriptorSet,
    dynamic_offset_count: u32,
    p_dynamic_offsets: *const u32,
);
#[doc(alias = "vkCmdBindIndexBuffer")]
pub type FNCmdBindIndexBuffer =
    unsafe extern "system" fn(command_buffer: CommandBuffer, buffer: Buffer, offset: DeviceSize, index_type: IndexType);
#[doc(alias = "vkCmdBindVertexBuffers")]
pub type FNCmdBindVertexBuffers = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_binding: u32,
    binding_count: u32,
    p_buffers: *const Buffer,
    p_offsets: *const DeviceSize,
);
#[doc(alias = "vkCmdDraw")]
pub type FNCmdDraw = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
);
#[doc(alias = "vkCmdDrawIndexed")]
pub type FNCmdDrawIndexed = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    index_count: u32,
    instance_count: u32,
    first_index: u32,
    vertex_offset: i32,
    first_instance: u32,
);
#[doc(alias = "vkCmdDrawIndirect")]
pub type FNCmdDrawIndirect = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
);
#[doc(alias = "vkCmdDrawIndexedIndirect")]
pub type FNCmdDrawIndexedIndirect = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
);
#[doc(alias = "vkCmdDispatch")]
pub type FNCmdDispatch = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
);
#[doc(alias = "vkCmdDispatchIndirect")]
pub type FNCmdDispatchIndirect =
    unsafe extern "system" fn(command_buffer: CommandBuffer, buffer: Buffer, offset: DeviceSize);
#[doc(alias = "vkCmdCopyBuffer")]
pub type FNCmdCopyBuffer = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_buffer: Buffer,
    dst_buffer: Buffer,
    region_count: u32,
    p_regions: *const BufferCopy,
);
#[doc(alias = "vkCmdCopyImage")]
pub type FNCmdCopyImage = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    p_regions: *const ImageCopy,
);
#[doc(alias = "vkCmdBlitImage")]
pub type FNCmdBlitImage = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    p_regions: *const ImageBlit,
    filter: Filter,
);
#[doc(alias = "vkCmdCopyBufferToImage")]
pub type FNCmdCopyBufferToImage = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_buffer: Buffer,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    p_regions: *const BufferImageCopy,
);
#[doc(alias = "vkCmdCopyImageToBuffer")]
pub type FNCmdCopyImageToBuffer = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_buffer: Buffer,
    region_count: u32,
    p_regions: *const BufferImageCopy,
);
#[doc(alias = "vkCmdUpdateBuffer")]
pub type FNCmdUpdateBuffer = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    data_size: DeviceSize,
    p_data: *const std::ffi::c_void,
);
#[doc(alias = "vkCmdFillBuffer")]
pub type FNCmdFillBuffer = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    size: DeviceSize,
    data: u32,
);
#[doc(alias = "vkCmdClearColorImage")]
pub type FNCmdClearColorImage = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    image: Image,
    image_layout: ImageLayout,
    p_color: *const ClearColorValue,
    range_count: u32,
    p_ranges: *const ImageSubresourceRange,
);
#[doc(alias = "vkCmdClearDepthStencilImage")]
pub type FNCmdClearDepthStencilImage = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    image: Image,
    image_layout: ImageLayout,
    p_depth_stencil: *const ClearDepthStencilValue,
    range_count: u32,
    p_ranges: *const ImageSubresourceRange,
);
#[doc(alias = "vkCmdClearAttachments")]
pub type FNCmdClearAttachments = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    attachment_count: u32,
    p_attachments: *const ClearAttachment,
    rect_count: u32,
    p_rects: *const ClearRect,
);
#[doc(alias = "vkCmdResolveImage")]
pub type FNCmdResolveImage = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    p_regions: *const ImageResolve,
);
#[doc(alias = "vkCmdSetEvent")]
pub type FNCmdSetEvent =
    unsafe extern "system" fn(command_buffer: CommandBuffer, event: Event, stage_mask: PipelineStageFlags);
#[doc(alias = "vkCmdResetEvent")]
pub type FNCmdResetEvent =
    unsafe extern "system" fn(command_buffer: CommandBuffer, event: Event, stage_mask: PipelineStageFlags);
#[doc(alias = "vkCmdWaitEvents")]
pub type FNCmdWaitEvents = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    event_count: u32,
    p_events: *const Event,
    src_stage_mask: PipelineStageFlags,
    dst_stage_mask: PipelineStageFlags,
    memory_barrier_count: u32,
    p_memory_barriers: *const MemoryBarrier,
    buffer_memory_barrier_count: u32,
    p_buffer_memory_barriers: *const BufferMemoryBarrier,
    image_memory_barrier_count: u32,
    p_image_memory_barriers: *const ImageMemoryBarrier,
);
#[doc(alias = "vkCmdPipelineBarrier")]
pub type FNCmdPipelineBarrier = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_stage_mask: PipelineStageFlags,
    dst_stage_mask: PipelineStageFlags,
    dependency_flags: DependencyFlags,
    memory_barrier_count: u32,
    p_memory_barriers: *const MemoryBarrier,
    buffer_memory_barrier_count: u32,
    p_buffer_memory_barriers: *const BufferMemoryBarrier,
    image_memory_barrier_count: u32,
    p_image_memory_barriers: *const ImageMemoryBarrier,
);
#[doc(alias = "vkCmdBeginQuery")]
pub type FNCmdBeginQuery = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    query_pool: QueryPool,
    query: u32,
    flags: QueryControlFlags,
);
#[doc(alias = "vkCmdEndQuery")]
pub type FNCmdEndQuery = unsafe extern "system" fn(command_buffer: CommandBuffer, query_pool: QueryPool, query: u32);
#[doc(alias = "vkCmdResetQueryPool")]
pub type FNCmdResetQueryPool =
    unsafe extern "system" fn(command_buffer: CommandBuffer, query_pool: QueryPool, first_query: u32, query_count: u32);
#[doc(alias = "vkCmdWriteTimestamp")]
pub type FNCmdWriteTimestamp = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_stage: PipelineStageFlagBits,
    query_pool: QueryPool,
    query: u32,
);
#[doc(alias = "vkCmdCopyQueryPoolResults")]
pub type FNCmdCopyQueryPoolResults = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    stride: DeviceSize,
    flags: QueryResultFlags,
);
#[doc(alias = "vkCmdPushConstants")]
pub type FNCmdPushConstants = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    layout: PipelineLayout,
    stage_flags: ShaderStageFlags,
    offset: u32,
    size: u32,
    p_values: *const std::ffi::c_void,
);
#[doc(alias = "vkCmdBeginRenderPass")]
pub type FNCmdBeginRenderPass = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_render_pass_begin: *const RenderPassBeginInfo,
    contents: SubpassContents,
);
#[doc(alias = "vkCmdNextSubpass")]
pub type FNCmdNextSubpass = unsafe extern "system" fn(command_buffer: CommandBuffer, contents: SubpassContents);
#[doc(alias = "vkCmdEndRenderPass")]
pub type FNCmdEndRenderPass = unsafe extern "system" fn(command_buffer: CommandBuffer);
#[doc(alias = "vkCmdExecuteCommands")]
pub type FNCmdExecuteCommands = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    command_buffer_count: u32,
    p_command_buffers: *const CommandBuffer,
);
