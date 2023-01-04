#[cfg(feature = "VK_EXT_debug_report")]
use crate::extensions::ext_debug_report::DebugReportCallbackEXT;
#[cfg(feature = "VK_EXT_debug_utils")]
use crate::extensions::ext_debug_utils::DebugUtilsMessengerEXT;
#[cfg(feature = "VK_EXT_validation_cache")]
use crate::extensions::ext_validation_cache::ValidationCacheEXT;
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
use crate::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA;
#[cfg(feature = "VK_INTEL_performance_query")]
use crate::extensions::intel_performance_query::PerformanceConfigurationINTEL;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::extensions::khr_acceleration_structure::AccelerationStructureKHR;
#[cfg(feature = "VK_KHR_deferred_host_operations")]
use crate::extensions::khr_deferred_host_operations::DeferredOperationKHR;
#[cfg(feature = "VK_KHR_display")]
use crate::extensions::khr_display::DisplayKHR;
#[cfg(feature = "VK_KHR_display")]
use crate::extensions::khr_display::DisplayModeKHR;
#[cfg(feature = "VK_KHR_surface")]
use crate::extensions::khr_surface::SurfaceKHR;
#[cfg(feature = "VK_KHR_swapchain")]
use crate::extensions::khr_swapchain::SwapchainImage;
#[cfg(feature = "VK_KHR_swapchain")]
use crate::extensions::khr_swapchain::SwapchainImageView;
#[cfg(feature = "VK_KHR_swapchain")]
use crate::extensions::khr_swapchain::SwapchainKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::extensions::khr_video_queue::VideoSessionKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::extensions::khr_video_queue::VideoSessionParametersKHR;
#[cfg(feature = "VK_NV_device_generated_commands")]
use crate::extensions::nv_device_generated_commands::IndirectCommandsLayoutNV;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::extensions::nv_ray_tracing::AccelerationStructureNV;
#[cfg(feature = "VK_NVX_binary_import")]
use crate::extensions::nvx_binary_import::CuFunctionNVX;
#[cfg(feature = "VK_NVX_binary_import")]
use crate::extensions::nvx_binary_import::CuModuleNVX;
#[cfg(feature = "VK_EXT_debug_marker")]
use crate::generated::extensions::ext_debug_marker::DebugReportObjectTypeEXT;
use crate::{
    vulkan1_0::{
        Buffer, BufferView, CommandBuffer, CommandPool, DescriptorPool, DescriptorSet, DescriptorSetLayout, Device,
        DeviceMemory, Event, Fence, Framebuffer, Image, ImageView, Instance, ObjectType, PhysicalDevice, Pipeline,
        PipelineCache, PipelineLayout, QueryPool, Queue, RenderPass, Sampler, Semaphore, ShaderModule,
    },
    vulkan1_1::{DescriptorUpdateTemplate, SamplerYcbcrConversion},
    vulkan1_3::PrivateDataSlot,
};
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Handles {
    Instance(Unique<Instance>),
    PhysicalDevice(Unique<PhysicalDevice>),
    Device(Unique<Device>),
    Queue(Unique<Queue>),
    CommandBuffer(Unique<CommandBuffer>),
    DeviceMemory(Unique<DeviceMemory>),
    CommandPool(Unique<CommandPool>),
    Buffer(Unique<Buffer>),
    BufferView(Unique<BufferView>),
    Image(Unique<Image>),
    ImageView(Unique<ImageView>),
    ShaderModule(Unique<ShaderModule>),
    Pipeline(Unique<Pipeline>),
    PipelineLayout(Unique<PipelineLayout>),
    Sampler(Unique<Sampler>),
    DescriptorSet(Unique<DescriptorSet>),
    DescriptorSetLayout(Unique<DescriptorSetLayout>),
    DescriptorPool(Unique<DescriptorPool>),
    Fence(Unique<Fence>),
    Semaphore(Unique<Semaphore>),
    Event(Unique<Event>),
    QueryPool(Unique<QueryPool>),
    Framebuffer(Unique<Framebuffer>),
    RenderPass(Unique<RenderPass>),
    PipelineCache(Unique<PipelineCache>),
    #[cfg(feature = "VK_NV_device_generated_commands")]
    IndirectCommandsLayoutNV(Unique<IndirectCommandsLayoutNV>),
    DescriptorUpdateTemplate(Unique<DescriptorUpdateTemplate>),
    SamplerYcbcrConversion(Unique<SamplerYcbcrConversion>),
    #[cfg(feature = "VK_EXT_validation_cache")]
    ValidationCacheEXT(Unique<ValidationCacheEXT>),
    #[cfg(feature = "VK_KHR_acceleration_structure")]
    AccelerationStructureKHR(Unique<AccelerationStructureKHR>),
    #[cfg(feature = "VK_NV_ray_tracing")]
    AccelerationStructureNV(Unique<AccelerationStructureNV>),
    #[cfg(feature = "VK_INTEL_performance_query")]
    PerformanceConfigurationINTEL(Unique<PerformanceConfigurationINTEL>),
    #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
    BufferCollectionFUCHSIA(Unique<BufferCollectionFUCHSIA>),
    #[cfg(feature = "VK_KHR_deferred_host_operations")]
    DeferredOperationKHR(Unique<DeferredOperationKHR>),
    PrivateDataSlot(Unique<PrivateDataSlot>),
    #[cfg(feature = "VK_NVX_binary_import")]
    CuModuleNVX(Unique<CuModuleNVX>),
    #[cfg(feature = "VK_NVX_binary_import")]
    CuFunctionNVX(Unique<CuFunctionNVX>),
    #[cfg(feature = "VK_KHR_display")]
    DisplayKHR(Unique<DisplayKHR>),
    #[cfg(feature = "VK_KHR_display")]
    DisplayModeKHR(Unique<DisplayModeKHR>),
    #[cfg(feature = "VK_KHR_surface")]
    SurfaceKHR(Unique<SurfaceKHR>),
    #[cfg(feature = "VK_KHR_swapchain")]
    SwapchainKHR(Unique<SwapchainKHR>),
    #[cfg(feature = "VK_EXT_debug_report")]
    DebugReportCallbackEXT(Unique<DebugReportCallbackEXT>),
    #[cfg(feature = "VK_EXT_debug_utils")]
    DebugUtilsMessengerEXT(Unique<DebugUtilsMessengerEXT>),
    #[cfg(feature = "VK_KHR_video_queue")]
    VideoSessionKHR(Unique<VideoSessionKHR>),
    #[cfg(feature = "VK_KHR_video_queue")]
    VideoSessionParametersKHR(Unique<VideoSessionParametersKHR>),
    #[cfg(feature = "VK_KHR_swapchain")]
    SwapchainImage(Unique<SwapchainImage>),
    #[cfg(feature = "VK_KHR_swapchain")]
    SwapchainImageView(Unique<SwapchainImageView>),
}
impl Handles {
    #[inline]
    pub fn strong_count(&self) -> usize {
        match self {
            Self::Instance(handle) => handle.strong_count(),
            Self::PhysicalDevice(handle) => handle.strong_count(),
            Self::Device(handle) => handle.strong_count(),
            Self::Queue(handle) => handle.strong_count(),
            Self::CommandBuffer(handle) => handle.strong_count(),
            Self::DeviceMemory(handle) => handle.strong_count(),
            Self::CommandPool(handle) => handle.strong_count(),
            Self::Buffer(handle) => handle.strong_count(),
            Self::BufferView(handle) => handle.strong_count(),
            Self::Image(handle) => handle.strong_count(),
            Self::ImageView(handle) => handle.strong_count(),
            Self::ShaderModule(handle) => handle.strong_count(),
            Self::Pipeline(handle) => handle.strong_count(),
            Self::PipelineLayout(handle) => handle.strong_count(),
            Self::Sampler(handle) => handle.strong_count(),
            Self::DescriptorSet(handle) => handle.strong_count(),
            Self::DescriptorSetLayout(handle) => handle.strong_count(),
            Self::DescriptorPool(handle) => handle.strong_count(),
            Self::Fence(handle) => handle.strong_count(),
            Self::Semaphore(handle) => handle.strong_count(),
            Self::Event(handle) => handle.strong_count(),
            Self::QueryPool(handle) => handle.strong_count(),
            Self::Framebuffer(handle) => handle.strong_count(),
            Self::RenderPass(handle) => handle.strong_count(),
            Self::PipelineCache(handle) => handle.strong_count(),
            #[cfg(feature = "VK_NV_device_generated_commands")]
            Self::IndirectCommandsLayoutNV(handle) => handle.strong_count(),
            Self::DescriptorUpdateTemplate(handle) => handle.strong_count(),
            Self::SamplerYcbcrConversion(handle) => handle.strong_count(),
            #[cfg(feature = "VK_EXT_validation_cache")]
            Self::ValidationCacheEXT(handle) => handle.strong_count(),
            #[cfg(feature = "VK_KHR_acceleration_structure")]
            Self::AccelerationStructureKHR(handle) => handle.strong_count(),
            #[cfg(feature = "VK_NV_ray_tracing")]
            Self::AccelerationStructureNV(handle) => handle.strong_count(),
            #[cfg(feature = "VK_INTEL_performance_query")]
            Self::PerformanceConfigurationINTEL(handle) => handle.strong_count(),
            #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
            Self::BufferCollectionFUCHSIA(handle) => handle.strong_count(),
            #[cfg(feature = "VK_KHR_deferred_host_operations")]
            Self::DeferredOperationKHR(handle) => handle.strong_count(),
            Self::PrivateDataSlot(handle) => handle.strong_count(),
            #[cfg(feature = "VK_NVX_binary_import")]
            Self::CuModuleNVX(handle) => handle.strong_count(),
            #[cfg(feature = "VK_NVX_binary_import")]
            Self::CuFunctionNVX(handle) => handle.strong_count(),
            #[cfg(feature = "VK_KHR_display")]
            Self::DisplayKHR(handle) => handle.strong_count(),
            #[cfg(feature = "VK_KHR_display")]
            Self::DisplayModeKHR(handle) => handle.strong_count(),
            #[cfg(feature = "VK_KHR_surface")]
            Self::SurfaceKHR(handle) => handle.strong_count(),
            #[cfg(feature = "VK_KHR_swapchain")]
            Self::SwapchainKHR(handle) => handle.strong_count(),
            #[cfg(feature = "VK_EXT_debug_report")]
            Self::DebugReportCallbackEXT(handle) => handle.strong_count(),
            #[cfg(feature = "VK_EXT_debug_utils")]
            Self::DebugUtilsMessengerEXT(handle) => handle.strong_count(),
            #[cfg(feature = "VK_KHR_video_queue")]
            Self::VideoSessionKHR(handle) => handle.strong_count(),
            #[cfg(feature = "VK_KHR_video_queue")]
            Self::VideoSessionParametersKHR(handle) => handle.strong_count(),
            #[cfg(feature = "VK_KHR_swapchain")]
            Self::SwapchainImage(handle) => handle.strong_count(),
            #[cfg(feature = "VK_KHR_swapchain")]
            Self::SwapchainImageView(handle) => handle.strong_count(),
        }
    }
}
impl Handles {
    #[inline]
    pub fn as_raw(&self) -> u64 {
        match self {
            Self::Instance(handle) => handle.as_stored() as u64,
            Self::PhysicalDevice(handle) => handle.as_stored() as u64,
            Self::Device(handle) => handle.as_stored() as u64,
            Self::Queue(handle) => handle.as_stored() as u64,
            Self::CommandBuffer(handle) => handle.as_stored() as u64,
            Self::DeviceMemory(handle) => handle.as_stored() as u64,
            Self::CommandPool(handle) => handle.as_stored() as u64,
            Self::Buffer(handle) => handle.as_stored() as u64,
            Self::BufferView(handle) => handle.as_stored() as u64,
            Self::Image(handle) => handle.as_stored() as u64,
            Self::ImageView(handle) => handle.as_stored() as u64,
            Self::ShaderModule(handle) => handle.as_stored() as u64,
            Self::Pipeline(handle) => handle.as_stored() as u64,
            Self::PipelineLayout(handle) => handle.as_stored() as u64,
            Self::Sampler(handle) => handle.as_stored() as u64,
            Self::DescriptorSet(handle) => handle.as_stored() as u64,
            Self::DescriptorSetLayout(handle) => handle.as_stored() as u64,
            Self::DescriptorPool(handle) => handle.as_stored() as u64,
            Self::Fence(handle) => handle.as_stored() as u64,
            Self::Semaphore(handle) => handle.as_stored() as u64,
            Self::Event(handle) => handle.as_stored() as u64,
            Self::QueryPool(handle) => handle.as_stored() as u64,
            Self::Framebuffer(handle) => handle.as_stored() as u64,
            Self::RenderPass(handle) => handle.as_stored() as u64,
            Self::PipelineCache(handle) => handle.as_stored() as u64,
            #[cfg(feature = "VK_NV_device_generated_commands")]
            Self::IndirectCommandsLayoutNV(handle) => handle.as_stored() as u64,
            Self::DescriptorUpdateTemplate(handle) => handle.as_stored() as u64,
            Self::SamplerYcbcrConversion(handle) => handle.as_stored() as u64,
            #[cfg(feature = "VK_EXT_validation_cache")]
            Self::ValidationCacheEXT(handle) => handle.as_stored() as u64,
            #[cfg(feature = "VK_KHR_acceleration_structure")]
            Self::AccelerationStructureKHR(handle) => handle.as_stored() as u64,
            #[cfg(feature = "VK_NV_ray_tracing")]
            Self::AccelerationStructureNV(handle) => handle.as_stored() as u64,
            #[cfg(feature = "VK_INTEL_performance_query")]
            Self::PerformanceConfigurationINTEL(handle) => handle.as_stored() as u64,
            #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
            Self::BufferCollectionFUCHSIA(handle) => handle.as_stored() as u64,
            #[cfg(feature = "VK_KHR_deferred_host_operations")]
            Self::DeferredOperationKHR(handle) => handle.as_stored() as u64,
            Self::PrivateDataSlot(handle) => handle.as_stored() as u64,
            #[cfg(feature = "VK_NVX_binary_import")]
            Self::CuModuleNVX(handle) => handle.as_stored() as u64,
            #[cfg(feature = "VK_NVX_binary_import")]
            Self::CuFunctionNVX(handle) => handle.as_stored() as u64,
            #[cfg(feature = "VK_KHR_display")]
            Self::DisplayKHR(handle) => handle.as_stored() as u64,
            #[cfg(feature = "VK_KHR_display")]
            Self::DisplayModeKHR(handle) => handle.as_stored() as u64,
            #[cfg(feature = "VK_KHR_surface")]
            Self::SurfaceKHR(handle) => handle.as_stored() as u64,
            #[cfg(feature = "VK_KHR_swapchain")]
            Self::SwapchainKHR(handle) => handle.as_stored() as u64,
            #[cfg(feature = "VK_EXT_debug_report")]
            Self::DebugReportCallbackEXT(handle) => handle.as_stored() as u64,
            #[cfg(feature = "VK_EXT_debug_utils")]
            Self::DebugUtilsMessengerEXT(handle) => handle.as_stored() as u64,
            #[cfg(feature = "VK_KHR_video_queue")]
            Self::VideoSessionKHR(handle) => handle.as_stored() as u64,
            #[cfg(feature = "VK_KHR_video_queue")]
            Self::VideoSessionParametersKHR(handle) => handle.as_stored() as u64,
            #[cfg(feature = "VK_KHR_swapchain")]
            Self::SwapchainImage(handle) => handle.as_stored() as u64,
            #[cfg(feature = "VK_KHR_swapchain")]
            Self::SwapchainImageView(handle) => handle.as_stored() as u64,
        }
    }
}
impl Handles {
    ///Gets the [`ObjectType`] from the handle
    pub fn as_object_type(&self) -> ObjectType {
        match self {
            Self::Instance(_) => ObjectType::INSTANCE,
            Self::PhysicalDevice(_) => ObjectType::PHYSICAL_DEVICE,
            Self::Device(_) => ObjectType::DEVICE,
            Self::Queue(_) => ObjectType::QUEUE,
            Self::CommandBuffer(_) => ObjectType::COMMAND_BUFFER,
            Self::DeviceMemory(_) => ObjectType::DEVICE_MEMORY,
            Self::CommandPool(_) => ObjectType::COMMAND_POOL,
            Self::Buffer(_) => ObjectType::BUFFER,
            Self::BufferView(_) => ObjectType::BUFFER_VIEW,
            Self::Image(_) => ObjectType::IMAGE,
            Self::ImageView(_) => ObjectType::IMAGE_VIEW,
            Self::ShaderModule(_) => ObjectType::SHADER_MODULE,
            Self::Pipeline(_) => ObjectType::PIPELINE,
            Self::PipelineLayout(_) => ObjectType::PIPELINE_LAYOUT,
            Self::Sampler(_) => ObjectType::SAMPLER,
            Self::DescriptorSet(_) => ObjectType::DESCRIPTOR_SET,
            Self::DescriptorSetLayout(_) => ObjectType::DESCRIPTOR_SET_LAYOUT,
            Self::DescriptorPool(_) => ObjectType::DESCRIPTOR_POOL,
            Self::Fence(_) => ObjectType::FENCE,
            Self::Semaphore(_) => ObjectType::SEMAPHORE,
            Self::Event(_) => ObjectType::EVENT,
            Self::QueryPool(_) => ObjectType::QUERY_POOL,
            Self::Framebuffer(_) => ObjectType::FRAMEBUFFER,
            Self::RenderPass(_) => ObjectType::RENDER_PASS,
            Self::PipelineCache(_) => ObjectType::PIPELINE_CACHE,
            #[cfg(feature = "VK_NV_device_generated_commands")]
            Self::IndirectCommandsLayoutNV(_) => ObjectType::INDIRECT_COMMANDS_LAYOUT_NV,
            Self::DescriptorUpdateTemplate(_) => ObjectType::DESCRIPTOR_UPDATE_TEMPLATE,
            Self::SamplerYcbcrConversion(_) => ObjectType::SAMPLER_YCBCR_CONVERSION,
            #[cfg(feature = "VK_EXT_validation_cache")]
            Self::ValidationCacheEXT(_) => ObjectType::VALIDATION_CACHE_EXT,
            #[cfg(feature = "VK_KHR_acceleration_structure")]
            Self::AccelerationStructureKHR(_) => ObjectType::ACCELERATION_STRUCTURE_KHR,
            #[cfg(feature = "VK_NV_ray_tracing")]
            Self::AccelerationStructureNV(_) => ObjectType::ACCELERATION_STRUCTURE_NV,
            #[cfg(feature = "VK_INTEL_performance_query")]
            Self::PerformanceConfigurationINTEL(_) => ObjectType::PERFORMANCE_CONFIGURATION_INTEL,
            #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
            Self::BufferCollectionFUCHSIA(_) => ObjectType::BUFFER_COLLECTION_FUCHSIA,
            #[cfg(feature = "VK_KHR_deferred_host_operations")]
            Self::DeferredOperationKHR(_) => ObjectType::DEFERRED_OPERATION_KHR,
            Self::PrivateDataSlot(_) => ObjectType::PRIVATE_DATA_SLOT,
            #[cfg(feature = "VK_NVX_binary_import")]
            Self::CuModuleNVX(_) => ObjectType::CU_MODULE_NVX,
            #[cfg(feature = "VK_NVX_binary_import")]
            Self::CuFunctionNVX(_) => ObjectType::CU_FUNCTION_NVX,
            #[cfg(feature = "VK_KHR_display")]
            Self::DisplayKHR(_) => ObjectType::DISPLAY_KHR,
            #[cfg(feature = "VK_KHR_display")]
            Self::DisplayModeKHR(_) => ObjectType::DISPLAY_MODE_KHR,
            #[cfg(feature = "VK_KHR_surface")]
            Self::SurfaceKHR(_) => ObjectType::SURFACE_KHR,
            #[cfg(feature = "VK_KHR_swapchain")]
            Self::SwapchainKHR(_) => ObjectType::SWAPCHAIN_KHR,
            #[cfg(feature = "VK_EXT_debug_report")]
            Self::DebugReportCallbackEXT(_) => ObjectType::DEBUG_REPORT_CALLBACK_EXT,
            #[cfg(feature = "VK_EXT_debug_utils")]
            Self::DebugUtilsMessengerEXT(_) => ObjectType::DEBUG_UTILS_MESSENGER_EXT,
            #[cfg(feature = "VK_KHR_video_queue")]
            Self::VideoSessionKHR(_) => ObjectType::VIDEO_SESSION_KHR,
            #[cfg(feature = "VK_KHR_video_queue")]
            Self::VideoSessionParametersKHR(_) => ObjectType::VIDEO_SESSION_PARAMETERS_KHR,
            #[cfg(feature = "VK_KHR_swapchain")]
            Self::SwapchainImage(_) => ObjectType::IMAGE,
            #[cfg(feature = "VK_KHR_swapchain")]
            Self::SwapchainImageView(_) => ObjectType::IMAGE_VIEW,
        }
    }
}
#[cfg(feature = "VK_EXT_debug_marker")]
impl Handles {
    ///Gets the [`DebugReportObjectTypeEXT`] from the handle
    pub fn as_debug_report_object_type_ext(&self) -> DebugReportObjectTypeEXT {
        match self {
            Self::Instance(_) => DebugReportObjectTypeEXT::INSTANCE,
            Self::PhysicalDevice(_) => DebugReportObjectTypeEXT::PHYSICAL_DEVICE,
            Self::Device(_) => DebugReportObjectTypeEXT::DEVICE,
            Self::Queue(_) => DebugReportObjectTypeEXT::QUEUE,
            Self::CommandBuffer(_) => DebugReportObjectTypeEXT::COMMAND_BUFFER,
            Self::DeviceMemory(_) => DebugReportObjectTypeEXT::DEVICE_MEMORY,
            Self::CommandPool(_) => DebugReportObjectTypeEXT::COMMAND_POOL,
            Self::Buffer(_) => DebugReportObjectTypeEXT::BUFFER,
            Self::BufferView(_) => DebugReportObjectTypeEXT::BUFFER_VIEW,
            Self::Image(_) => DebugReportObjectTypeEXT::IMAGE,
            Self::ImageView(_) => DebugReportObjectTypeEXT::IMAGE_VIEW,
            Self::ShaderModule(_) => DebugReportObjectTypeEXT::SHADER_MODULE,
            Self::Pipeline(_) => DebugReportObjectTypeEXT::PIPELINE,
            Self::PipelineLayout(_) => DebugReportObjectTypeEXT::PIPELINE_LAYOUT,
            Self::Sampler(_) => DebugReportObjectTypeEXT::SAMPLER,
            Self::DescriptorSet(_) => DebugReportObjectTypeEXT::DESCRIPTOR_SET,
            Self::DescriptorSetLayout(_) => DebugReportObjectTypeEXT::DESCRIPTOR_SET_LAYOUT,
            Self::DescriptorPool(_) => DebugReportObjectTypeEXT::DESCRIPTOR_POOL,
            Self::Fence(_) => DebugReportObjectTypeEXT::FENCE,
            Self::Semaphore(_) => DebugReportObjectTypeEXT::SEMAPHORE,
            Self::Event(_) => DebugReportObjectTypeEXT::EVENT,
            Self::QueryPool(_) => DebugReportObjectTypeEXT::QUERY_POOL,
            Self::Framebuffer(_) => DebugReportObjectTypeEXT::FRAMEBUFFER,
            Self::RenderPass(_) => DebugReportObjectTypeEXT::RENDER_PASS,
            Self::PipelineCache(_) => DebugReportObjectTypeEXT::PIPELINE_CACHE,
            #[cfg(feature = "VK_NV_device_generated_commands")]
            Self::IndirectCommandsLayoutNV(_) => DebugReportObjectTypeEXT::UNKNOWN,
            Self::DescriptorUpdateTemplate(_) => DebugReportObjectTypeEXT::DESCRIPTOR_UPDATE_TEMPLATE,
            Self::SamplerYcbcrConversion(_) => DebugReportObjectTypeEXT::SAMPLER_YCBCR_CONVERSION,
            #[cfg(feature = "VK_EXT_validation_cache")]
            Self::ValidationCacheEXT(_) => DebugReportObjectTypeEXT::VALIDATION_CACHE,
            #[cfg(feature = "VK_KHR_acceleration_structure")]
            Self::AccelerationStructureKHR(_) => DebugReportObjectTypeEXT::ACCELERATION_STRUCTURE_KHR,
            #[cfg(feature = "VK_NV_ray_tracing")]
            Self::AccelerationStructureNV(_) => DebugReportObjectTypeEXT::ACCELERATION_STRUCTURE_NV,
            #[cfg(feature = "VK_INTEL_performance_query")]
            Self::PerformanceConfigurationINTEL(_) => DebugReportObjectTypeEXT::UNKNOWN,
            #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
            Self::BufferCollectionFUCHSIA(_) => DebugReportObjectTypeEXT::BUFFER_COLLECTION_FUCHSIA,
            #[cfg(feature = "VK_KHR_deferred_host_operations")]
            Self::DeferredOperationKHR(_) => DebugReportObjectTypeEXT::UNKNOWN,
            Self::PrivateDataSlot(_) => DebugReportObjectTypeEXT::UNKNOWN,
            #[cfg(feature = "VK_NVX_binary_import")]
            Self::CuModuleNVX(_) => DebugReportObjectTypeEXT::CU_MODULE_NVX,
            #[cfg(feature = "VK_NVX_binary_import")]
            Self::CuFunctionNVX(_) => DebugReportObjectTypeEXT::CU_FUNCTION_NVX,
            #[cfg(feature = "VK_KHR_display")]
            Self::DisplayKHR(_) => DebugReportObjectTypeEXT::DISPLAY_KHR,
            #[cfg(feature = "VK_KHR_display")]
            Self::DisplayModeKHR(_) => DebugReportObjectTypeEXT::DISPLAY_MODE_KHR,
            #[cfg(feature = "VK_KHR_surface")]
            Self::SurfaceKHR(_) => DebugReportObjectTypeEXT::SURFACE_KHR,
            #[cfg(feature = "VK_KHR_swapchain")]
            Self::SwapchainKHR(_) => DebugReportObjectTypeEXT::SWAPCHAIN_KHR,
            #[cfg(feature = "VK_EXT_debug_report")]
            Self::DebugReportCallbackEXT(_) => DebugReportObjectTypeEXT::DEBUG_REPORT_CALLBACK,
            #[cfg(feature = "VK_EXT_debug_utils")]
            Self::DebugUtilsMessengerEXT(_) => DebugReportObjectTypeEXT::UNKNOWN,
            #[cfg(feature = "VK_KHR_video_queue")]
            Self::VideoSessionKHR(_) => DebugReportObjectTypeEXT::UNKNOWN,
            #[cfg(feature = "VK_KHR_video_queue")]
            Self::VideoSessionParametersKHR(_) => DebugReportObjectTypeEXT::UNKNOWN,
            #[cfg(feature = "VK_KHR_swapchain")]
            Self::SwapchainImage(_) => DebugReportObjectTypeEXT::IMAGE,
            #[cfg(feature = "VK_KHR_swapchain")]
            Self::SwapchainImageView(_) => DebugReportObjectTypeEXT::IMAGE_VIEW,
        }
    }
}
impl Handles {
    ///Checks if the handle is a `Instance`
    #[inline]
    pub fn is_instance(&self) -> bool {
        matches!(self, Self::Instance(_))
    }
    ///Gets the handle as a `Instance`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_instance(self) -> Unique<Instance> {
        self.try_as_instance().expect("Handle is not a `Instance`")
    }
    ///Gets the handle as a `Instance`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_instance(self) -> Option<Unique<Instance>> {
        match self {
            Self::Instance(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `Instance`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_instance_unchecked(self) -> Unique<Instance> {
        match self {
            Self::Instance(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
impl From<Unique<Instance>> for Handles {
    fn from(value: Unique<Instance>) -> Self {
        Self::Instance(value)
    }
}
impl Into<Unique<Instance>> for Handles {
    fn into(self) -> Unique<Instance> {
        self.as_instance()
    }
}
impl Handles {
    ///Checks if the handle is a `PhysicalDevice`
    #[inline]
    pub fn is_physical_device(&self) -> bool {
        matches!(self, Self::PhysicalDevice(_))
    }
    ///Gets the handle as a `PhysicalDevice`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_physical_device(self) -> Unique<PhysicalDevice> {
        self.try_as_physical_device().expect("Handle is not a `PhysicalDevice`")
    }
    ///Gets the handle as a `PhysicalDevice`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_physical_device(self) -> Option<Unique<PhysicalDevice>> {
        match self {
            Self::PhysicalDevice(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `PhysicalDevice`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_physical_device_unchecked(self) -> Unique<PhysicalDevice> {
        match self {
            Self::PhysicalDevice(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
impl From<Unique<PhysicalDevice>> for Handles {
    fn from(value: Unique<PhysicalDevice>) -> Self {
        Self::PhysicalDevice(value)
    }
}
impl Into<Unique<PhysicalDevice>> for Handles {
    fn into(self) -> Unique<PhysicalDevice> {
        self.as_physical_device()
    }
}
impl Handles {
    ///Checks if the handle is a `Device`
    #[inline]
    pub fn is_device(&self) -> bool {
        matches!(self, Self::Device(_))
    }
    ///Gets the handle as a `Device`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_device(self) -> Unique<Device> {
        self.try_as_device().expect("Handle is not a `Device`")
    }
    ///Gets the handle as a `Device`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_device(self) -> Option<Unique<Device>> {
        match self {
            Self::Device(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `Device`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_device_unchecked(self) -> Unique<Device> {
        match self {
            Self::Device(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
impl From<Unique<Device>> for Handles {
    fn from(value: Unique<Device>) -> Self {
        Self::Device(value)
    }
}
impl Into<Unique<Device>> for Handles {
    fn into(self) -> Unique<Device> {
        self.as_device()
    }
}
impl Handles {
    ///Checks if the handle is a `Queue`
    #[inline]
    pub fn is_queue(&self) -> bool {
        matches!(self, Self::Queue(_))
    }
    ///Gets the handle as a `Queue`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_queue(self) -> Unique<Queue> {
        self.try_as_queue().expect("Handle is not a `Queue`")
    }
    ///Gets the handle as a `Queue`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_queue(self) -> Option<Unique<Queue>> {
        match self {
            Self::Queue(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `Queue`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_queue_unchecked(self) -> Unique<Queue> {
        match self {
            Self::Queue(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
impl From<Unique<Queue>> for Handles {
    fn from(value: Unique<Queue>) -> Self {
        Self::Queue(value)
    }
}
impl Into<Unique<Queue>> for Handles {
    fn into(self) -> Unique<Queue> {
        self.as_queue()
    }
}
impl Handles {
    ///Checks if the handle is a `CommandBuffer`
    #[inline]
    pub fn is_command_buffer(&self) -> bool {
        matches!(self, Self::CommandBuffer(_))
    }
    ///Gets the handle as a `CommandBuffer`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_command_buffer(self) -> Unique<CommandBuffer> {
        self.try_as_command_buffer().expect("Handle is not a `CommandBuffer`")
    }
    ///Gets the handle as a `CommandBuffer`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_command_buffer(self) -> Option<Unique<CommandBuffer>> {
        match self {
            Self::CommandBuffer(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `CommandBuffer`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_command_buffer_unchecked(self) -> Unique<CommandBuffer> {
        match self {
            Self::CommandBuffer(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
impl From<Unique<CommandBuffer>> for Handles {
    fn from(value: Unique<CommandBuffer>) -> Self {
        Self::CommandBuffer(value)
    }
}
impl Into<Unique<CommandBuffer>> for Handles {
    fn into(self) -> Unique<CommandBuffer> {
        self.as_command_buffer()
    }
}
impl Handles {
    ///Checks if the handle is a `DeviceMemory`
    #[inline]
    pub fn is_device_memory(&self) -> bool {
        matches!(self, Self::DeviceMemory(_))
    }
    ///Gets the handle as a `DeviceMemory`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_device_memory(self) -> Unique<DeviceMemory> {
        self.try_as_device_memory().expect("Handle is not a `DeviceMemory`")
    }
    ///Gets the handle as a `DeviceMemory`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_device_memory(self) -> Option<Unique<DeviceMemory>> {
        match self {
            Self::DeviceMemory(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `DeviceMemory`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_device_memory_unchecked(self) -> Unique<DeviceMemory> {
        match self {
            Self::DeviceMemory(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
impl From<Unique<DeviceMemory>> for Handles {
    fn from(value: Unique<DeviceMemory>) -> Self {
        Self::DeviceMemory(value)
    }
}
impl Into<Unique<DeviceMemory>> for Handles {
    fn into(self) -> Unique<DeviceMemory> {
        self.as_device_memory()
    }
}
impl Handles {
    ///Checks if the handle is a `CommandPool`
    #[inline]
    pub fn is_command_pool(&self) -> bool {
        matches!(self, Self::CommandPool(_))
    }
    ///Gets the handle as a `CommandPool`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_command_pool(self) -> Unique<CommandPool> {
        self.try_as_command_pool().expect("Handle is not a `CommandPool`")
    }
    ///Gets the handle as a `CommandPool`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_command_pool(self) -> Option<Unique<CommandPool>> {
        match self {
            Self::CommandPool(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `CommandPool`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_command_pool_unchecked(self) -> Unique<CommandPool> {
        match self {
            Self::CommandPool(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
impl From<Unique<CommandPool>> for Handles {
    fn from(value: Unique<CommandPool>) -> Self {
        Self::CommandPool(value)
    }
}
impl Into<Unique<CommandPool>> for Handles {
    fn into(self) -> Unique<CommandPool> {
        self.as_command_pool()
    }
}
impl Handles {
    ///Checks if the handle is a `Buffer`
    #[inline]
    pub fn is_buffer(&self) -> bool {
        matches!(self, Self::Buffer(_))
    }
    ///Gets the handle as a `Buffer`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_buffer(self) -> Unique<Buffer> {
        self.try_as_buffer().expect("Handle is not a `Buffer`")
    }
    ///Gets the handle as a `Buffer`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_buffer(self) -> Option<Unique<Buffer>> {
        match self {
            Self::Buffer(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `Buffer`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_buffer_unchecked(self) -> Unique<Buffer> {
        match self {
            Self::Buffer(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
impl From<Unique<Buffer>> for Handles {
    fn from(value: Unique<Buffer>) -> Self {
        Self::Buffer(value)
    }
}
impl Into<Unique<Buffer>> for Handles {
    fn into(self) -> Unique<Buffer> {
        self.as_buffer()
    }
}
impl Handles {
    ///Checks if the handle is a `BufferView`
    #[inline]
    pub fn is_buffer_view(&self) -> bool {
        matches!(self, Self::BufferView(_))
    }
    ///Gets the handle as a `BufferView`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_buffer_view(self) -> Unique<BufferView> {
        self.try_as_buffer_view().expect("Handle is not a `BufferView`")
    }
    ///Gets the handle as a `BufferView`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_buffer_view(self) -> Option<Unique<BufferView>> {
        match self {
            Self::BufferView(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `BufferView`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_buffer_view_unchecked(self) -> Unique<BufferView> {
        match self {
            Self::BufferView(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
impl From<Unique<BufferView>> for Handles {
    fn from(value: Unique<BufferView>) -> Self {
        Self::BufferView(value)
    }
}
impl Into<Unique<BufferView>> for Handles {
    fn into(self) -> Unique<BufferView> {
        self.as_buffer_view()
    }
}
impl Handles {
    ///Checks if the handle is a `Image`
    #[inline]
    pub fn is_image(&self) -> bool {
        matches!(self, Self::Image(_))
    }
    ///Gets the handle as a `Image`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_image(self) -> Unique<Image> {
        self.try_as_image().expect("Handle is not a `Image`")
    }
    ///Gets the handle as a `Image`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_image(self) -> Option<Unique<Image>> {
        match self {
            Self::Image(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `Image`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_image_unchecked(self) -> Unique<Image> {
        match self {
            Self::Image(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
impl From<Unique<Image>> for Handles {
    fn from(value: Unique<Image>) -> Self {
        Self::Image(value)
    }
}
impl Into<Unique<Image>> for Handles {
    fn into(self) -> Unique<Image> {
        self.as_image()
    }
}
impl Handles {
    ///Checks if the handle is a `ImageView`
    #[inline]
    pub fn is_image_view(&self) -> bool {
        matches!(self, Self::ImageView(_))
    }
    ///Gets the handle as a `ImageView`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_image_view(self) -> Unique<ImageView> {
        self.try_as_image_view().expect("Handle is not a `ImageView`")
    }
    ///Gets the handle as a `ImageView`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_image_view(self) -> Option<Unique<ImageView>> {
        match self {
            Self::ImageView(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `ImageView`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_image_view_unchecked(self) -> Unique<ImageView> {
        match self {
            Self::ImageView(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
impl From<Unique<ImageView>> for Handles {
    fn from(value: Unique<ImageView>) -> Self {
        Self::ImageView(value)
    }
}
impl Into<Unique<ImageView>> for Handles {
    fn into(self) -> Unique<ImageView> {
        self.as_image_view()
    }
}
impl Handles {
    ///Checks if the handle is a `ShaderModule`
    #[inline]
    pub fn is_shader_module(&self) -> bool {
        matches!(self, Self::ShaderModule(_))
    }
    ///Gets the handle as a `ShaderModule`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_shader_module(self) -> Unique<ShaderModule> {
        self.try_as_shader_module().expect("Handle is not a `ShaderModule`")
    }
    ///Gets the handle as a `ShaderModule`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_shader_module(self) -> Option<Unique<ShaderModule>> {
        match self {
            Self::ShaderModule(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `ShaderModule`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_shader_module_unchecked(self) -> Unique<ShaderModule> {
        match self {
            Self::ShaderModule(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
impl From<Unique<ShaderModule>> for Handles {
    fn from(value: Unique<ShaderModule>) -> Self {
        Self::ShaderModule(value)
    }
}
impl Into<Unique<ShaderModule>> for Handles {
    fn into(self) -> Unique<ShaderModule> {
        self.as_shader_module()
    }
}
impl Handles {
    ///Checks if the handle is a `Pipeline`
    #[inline]
    pub fn is_pipeline(&self) -> bool {
        matches!(self, Self::Pipeline(_))
    }
    ///Gets the handle as a `Pipeline`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_pipeline(self) -> Unique<Pipeline> {
        self.try_as_pipeline().expect("Handle is not a `Pipeline`")
    }
    ///Gets the handle as a `Pipeline`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_pipeline(self) -> Option<Unique<Pipeline>> {
        match self {
            Self::Pipeline(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `Pipeline`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_pipeline_unchecked(self) -> Unique<Pipeline> {
        match self {
            Self::Pipeline(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
impl From<Unique<Pipeline>> for Handles {
    fn from(value: Unique<Pipeline>) -> Self {
        Self::Pipeline(value)
    }
}
impl Into<Unique<Pipeline>> for Handles {
    fn into(self) -> Unique<Pipeline> {
        self.as_pipeline()
    }
}
impl Handles {
    ///Checks if the handle is a `PipelineLayout`
    #[inline]
    pub fn is_pipeline_layout(&self) -> bool {
        matches!(self, Self::PipelineLayout(_))
    }
    ///Gets the handle as a `PipelineLayout`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_pipeline_layout(self) -> Unique<PipelineLayout> {
        self.try_as_pipeline_layout().expect("Handle is not a `PipelineLayout`")
    }
    ///Gets the handle as a `PipelineLayout`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_pipeline_layout(self) -> Option<Unique<PipelineLayout>> {
        match self {
            Self::PipelineLayout(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `PipelineLayout`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_pipeline_layout_unchecked(self) -> Unique<PipelineLayout> {
        match self {
            Self::PipelineLayout(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
impl From<Unique<PipelineLayout>> for Handles {
    fn from(value: Unique<PipelineLayout>) -> Self {
        Self::PipelineLayout(value)
    }
}
impl Into<Unique<PipelineLayout>> for Handles {
    fn into(self) -> Unique<PipelineLayout> {
        self.as_pipeline_layout()
    }
}
impl Handles {
    ///Checks if the handle is a `Sampler`
    #[inline]
    pub fn is_sampler(&self) -> bool {
        matches!(self, Self::Sampler(_))
    }
    ///Gets the handle as a `Sampler`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_sampler(self) -> Unique<Sampler> {
        self.try_as_sampler().expect("Handle is not a `Sampler`")
    }
    ///Gets the handle as a `Sampler`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_sampler(self) -> Option<Unique<Sampler>> {
        match self {
            Self::Sampler(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `Sampler`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_sampler_unchecked(self) -> Unique<Sampler> {
        match self {
            Self::Sampler(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
impl From<Unique<Sampler>> for Handles {
    fn from(value: Unique<Sampler>) -> Self {
        Self::Sampler(value)
    }
}
impl Into<Unique<Sampler>> for Handles {
    fn into(self) -> Unique<Sampler> {
        self.as_sampler()
    }
}
impl Handles {
    ///Checks if the handle is a `DescriptorSet`
    #[inline]
    pub fn is_descriptor_set(&self) -> bool {
        matches!(self, Self::DescriptorSet(_))
    }
    ///Gets the handle as a `DescriptorSet`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_descriptor_set(self) -> Unique<DescriptorSet> {
        self.try_as_descriptor_set().expect("Handle is not a `DescriptorSet`")
    }
    ///Gets the handle as a `DescriptorSet`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_descriptor_set(self) -> Option<Unique<DescriptorSet>> {
        match self {
            Self::DescriptorSet(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `DescriptorSet`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_descriptor_set_unchecked(self) -> Unique<DescriptorSet> {
        match self {
            Self::DescriptorSet(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
impl From<Unique<DescriptorSet>> for Handles {
    fn from(value: Unique<DescriptorSet>) -> Self {
        Self::DescriptorSet(value)
    }
}
impl Into<Unique<DescriptorSet>> for Handles {
    fn into(self) -> Unique<DescriptorSet> {
        self.as_descriptor_set()
    }
}
impl Handles {
    ///Checks if the handle is a `DescriptorSetLayout`
    #[inline]
    pub fn is_descriptor_set_layout(&self) -> bool {
        matches!(self, Self::DescriptorSetLayout(_))
    }
    ///Gets the handle as a `DescriptorSetLayout`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_descriptor_set_layout(self) -> Unique<DescriptorSetLayout> {
        self.try_as_descriptor_set_layout()
            .expect("Handle is not a `DescriptorSetLayout`")
    }
    ///Gets the handle as a `DescriptorSetLayout`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_descriptor_set_layout(self) -> Option<Unique<DescriptorSetLayout>> {
        match self {
            Self::DescriptorSetLayout(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `DescriptorSetLayout`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_descriptor_set_layout_unchecked(self) -> Unique<DescriptorSetLayout> {
        match self {
            Self::DescriptorSetLayout(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
impl From<Unique<DescriptorSetLayout>> for Handles {
    fn from(value: Unique<DescriptorSetLayout>) -> Self {
        Self::DescriptorSetLayout(value)
    }
}
impl Into<Unique<DescriptorSetLayout>> for Handles {
    fn into(self) -> Unique<DescriptorSetLayout> {
        self.as_descriptor_set_layout()
    }
}
impl Handles {
    ///Checks if the handle is a `DescriptorPool`
    #[inline]
    pub fn is_descriptor_pool(&self) -> bool {
        matches!(self, Self::DescriptorPool(_))
    }
    ///Gets the handle as a `DescriptorPool`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_descriptor_pool(self) -> Unique<DescriptorPool> {
        self.try_as_descriptor_pool().expect("Handle is not a `DescriptorPool`")
    }
    ///Gets the handle as a `DescriptorPool`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_descriptor_pool(self) -> Option<Unique<DescriptorPool>> {
        match self {
            Self::DescriptorPool(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `DescriptorPool`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_descriptor_pool_unchecked(self) -> Unique<DescriptorPool> {
        match self {
            Self::DescriptorPool(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
impl From<Unique<DescriptorPool>> for Handles {
    fn from(value: Unique<DescriptorPool>) -> Self {
        Self::DescriptorPool(value)
    }
}
impl Into<Unique<DescriptorPool>> for Handles {
    fn into(self) -> Unique<DescriptorPool> {
        self.as_descriptor_pool()
    }
}
impl Handles {
    ///Checks if the handle is a `Fence`
    #[inline]
    pub fn is_fence(&self) -> bool {
        matches!(self, Self::Fence(_))
    }
    ///Gets the handle as a `Fence`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_fence(self) -> Unique<Fence> {
        self.try_as_fence().expect("Handle is not a `Fence`")
    }
    ///Gets the handle as a `Fence`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_fence(self) -> Option<Unique<Fence>> {
        match self {
            Self::Fence(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `Fence`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_fence_unchecked(self) -> Unique<Fence> {
        match self {
            Self::Fence(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
impl From<Unique<Fence>> for Handles {
    fn from(value: Unique<Fence>) -> Self {
        Self::Fence(value)
    }
}
impl Into<Unique<Fence>> for Handles {
    fn into(self) -> Unique<Fence> {
        self.as_fence()
    }
}
impl Handles {
    ///Checks if the handle is a `Semaphore`
    #[inline]
    pub fn is_semaphore(&self) -> bool {
        matches!(self, Self::Semaphore(_))
    }
    ///Gets the handle as a `Semaphore`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_semaphore(self) -> Unique<Semaphore> {
        self.try_as_semaphore().expect("Handle is not a `Semaphore`")
    }
    ///Gets the handle as a `Semaphore`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_semaphore(self) -> Option<Unique<Semaphore>> {
        match self {
            Self::Semaphore(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `Semaphore`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_semaphore_unchecked(self) -> Unique<Semaphore> {
        match self {
            Self::Semaphore(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
impl From<Unique<Semaphore>> for Handles {
    fn from(value: Unique<Semaphore>) -> Self {
        Self::Semaphore(value)
    }
}
impl Into<Unique<Semaphore>> for Handles {
    fn into(self) -> Unique<Semaphore> {
        self.as_semaphore()
    }
}
impl Handles {
    ///Checks if the handle is a `Event`
    #[inline]
    pub fn is_event(&self) -> bool {
        matches!(self, Self::Event(_))
    }
    ///Gets the handle as a `Event`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_event(self) -> Unique<Event> {
        self.try_as_event().expect("Handle is not a `Event`")
    }
    ///Gets the handle as a `Event`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_event(self) -> Option<Unique<Event>> {
        match self {
            Self::Event(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `Event`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_event_unchecked(self) -> Unique<Event> {
        match self {
            Self::Event(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
impl From<Unique<Event>> for Handles {
    fn from(value: Unique<Event>) -> Self {
        Self::Event(value)
    }
}
impl Into<Unique<Event>> for Handles {
    fn into(self) -> Unique<Event> {
        self.as_event()
    }
}
impl Handles {
    ///Checks if the handle is a `QueryPool`
    #[inline]
    pub fn is_query_pool(&self) -> bool {
        matches!(self, Self::QueryPool(_))
    }
    ///Gets the handle as a `QueryPool`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_query_pool(self) -> Unique<QueryPool> {
        self.try_as_query_pool().expect("Handle is not a `QueryPool`")
    }
    ///Gets the handle as a `QueryPool`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_query_pool(self) -> Option<Unique<QueryPool>> {
        match self {
            Self::QueryPool(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `QueryPool`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_query_pool_unchecked(self) -> Unique<QueryPool> {
        match self {
            Self::QueryPool(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
impl From<Unique<QueryPool>> for Handles {
    fn from(value: Unique<QueryPool>) -> Self {
        Self::QueryPool(value)
    }
}
impl Into<Unique<QueryPool>> for Handles {
    fn into(self) -> Unique<QueryPool> {
        self.as_query_pool()
    }
}
impl Handles {
    ///Checks if the handle is a `Framebuffer`
    #[inline]
    pub fn is_framebuffer(&self) -> bool {
        matches!(self, Self::Framebuffer(_))
    }
    ///Gets the handle as a `Framebuffer`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_framebuffer(self) -> Unique<Framebuffer> {
        self.try_as_framebuffer().expect("Handle is not a `Framebuffer`")
    }
    ///Gets the handle as a `Framebuffer`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_framebuffer(self) -> Option<Unique<Framebuffer>> {
        match self {
            Self::Framebuffer(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `Framebuffer`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_framebuffer_unchecked(self) -> Unique<Framebuffer> {
        match self {
            Self::Framebuffer(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
impl From<Unique<Framebuffer>> for Handles {
    fn from(value: Unique<Framebuffer>) -> Self {
        Self::Framebuffer(value)
    }
}
impl Into<Unique<Framebuffer>> for Handles {
    fn into(self) -> Unique<Framebuffer> {
        self.as_framebuffer()
    }
}
impl Handles {
    ///Checks if the handle is a `RenderPass`
    #[inline]
    pub fn is_render_pass(&self) -> bool {
        matches!(self, Self::RenderPass(_))
    }
    ///Gets the handle as a `RenderPass`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_render_pass(self) -> Unique<RenderPass> {
        self.try_as_render_pass().expect("Handle is not a `RenderPass`")
    }
    ///Gets the handle as a `RenderPass`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_render_pass(self) -> Option<Unique<RenderPass>> {
        match self {
            Self::RenderPass(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `RenderPass`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_render_pass_unchecked(self) -> Unique<RenderPass> {
        match self {
            Self::RenderPass(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
impl From<Unique<RenderPass>> for Handles {
    fn from(value: Unique<RenderPass>) -> Self {
        Self::RenderPass(value)
    }
}
impl Into<Unique<RenderPass>> for Handles {
    fn into(self) -> Unique<RenderPass> {
        self.as_render_pass()
    }
}
impl Handles {
    ///Checks if the handle is a `PipelineCache`
    #[inline]
    pub fn is_pipeline_cache(&self) -> bool {
        matches!(self, Self::PipelineCache(_))
    }
    ///Gets the handle as a `PipelineCache`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_pipeline_cache(self) -> Unique<PipelineCache> {
        self.try_as_pipeline_cache().expect("Handle is not a `PipelineCache`")
    }
    ///Gets the handle as a `PipelineCache`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_pipeline_cache(self) -> Option<Unique<PipelineCache>> {
        match self {
            Self::PipelineCache(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `PipelineCache`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_pipeline_cache_unchecked(self) -> Unique<PipelineCache> {
        match self {
            Self::PipelineCache(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
impl From<Unique<PipelineCache>> for Handles {
    fn from(value: Unique<PipelineCache>) -> Self {
        Self::PipelineCache(value)
    }
}
impl Into<Unique<PipelineCache>> for Handles {
    fn into(self) -> Unique<PipelineCache> {
        self.as_pipeline_cache()
    }
}
#[cfg(feature = "VK_NV_device_generated_commands")]
impl Handles {
    ///Checks if the handle is a `IndirectCommandsLayoutNV`
    #[inline]
    pub fn is_indirect_commands_layout_nv(&self) -> bool {
        matches!(self, Self::IndirectCommandsLayoutNV(_))
    }
    ///Gets the handle as a `IndirectCommandsLayoutNV`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_indirect_commands_layout_nv(self) -> Unique<IndirectCommandsLayoutNV> {
        self.try_as_indirect_commands_layout_nv()
            .expect("Handle is not a `IndirectCommandsLayoutNV`")
    }
    ///Gets the handle as a `IndirectCommandsLayoutNV`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_indirect_commands_layout_nv(self) -> Option<Unique<IndirectCommandsLayoutNV>> {
        match self {
            Self::IndirectCommandsLayoutNV(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `IndirectCommandsLayoutNV`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_indirect_commands_layout_nv_unchecked(self) -> Unique<IndirectCommandsLayoutNV> {
        match self {
            Self::IndirectCommandsLayoutNV(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
#[cfg(feature = "VK_NV_device_generated_commands")]
impl From<Unique<IndirectCommandsLayoutNV>> for Handles {
    fn from(value: Unique<IndirectCommandsLayoutNV>) -> Self {
        Self::IndirectCommandsLayoutNV(value)
    }
}
#[cfg(feature = "VK_NV_device_generated_commands")]
impl Into<Unique<IndirectCommandsLayoutNV>> for Handles {
    fn into(self) -> Unique<IndirectCommandsLayoutNV> {
        self.as_indirect_commands_layout_nv()
    }
}
impl Handles {
    ///Checks if the handle is a `DescriptorUpdateTemplate`
    #[inline]
    pub fn is_descriptor_update_template(&self) -> bool {
        matches!(self, Self::DescriptorUpdateTemplate(_))
    }
    ///Gets the handle as a `DescriptorUpdateTemplate`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_descriptor_update_template(self) -> Unique<DescriptorUpdateTemplate> {
        self.try_as_descriptor_update_template()
            .expect("Handle is not a `DescriptorUpdateTemplate`")
    }
    ///Gets the handle as a `DescriptorUpdateTemplate`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_descriptor_update_template(self) -> Option<Unique<DescriptorUpdateTemplate>> {
        match self {
            Self::DescriptorUpdateTemplate(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `DescriptorUpdateTemplate`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_descriptor_update_template_unchecked(self) -> Unique<DescriptorUpdateTemplate> {
        match self {
            Self::DescriptorUpdateTemplate(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
impl From<Unique<DescriptorUpdateTemplate>> for Handles {
    fn from(value: Unique<DescriptorUpdateTemplate>) -> Self {
        Self::DescriptorUpdateTemplate(value)
    }
}
impl Into<Unique<DescriptorUpdateTemplate>> for Handles {
    fn into(self) -> Unique<DescriptorUpdateTemplate> {
        self.as_descriptor_update_template()
    }
}
impl Handles {
    ///Checks if the handle is a `SamplerYcbcrConversion`
    #[inline]
    pub fn is_sampler_ycbcr_conversion(&self) -> bool {
        matches!(self, Self::SamplerYcbcrConversion(_))
    }
    ///Gets the handle as a `SamplerYcbcrConversion`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_sampler_ycbcr_conversion(self) -> Unique<SamplerYcbcrConversion> {
        self.try_as_sampler_ycbcr_conversion()
            .expect("Handle is not a `SamplerYcbcrConversion`")
    }
    ///Gets the handle as a `SamplerYcbcrConversion`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_sampler_ycbcr_conversion(self) -> Option<Unique<SamplerYcbcrConversion>> {
        match self {
            Self::SamplerYcbcrConversion(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `SamplerYcbcrConversion`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_sampler_ycbcr_conversion_unchecked(self) -> Unique<SamplerYcbcrConversion> {
        match self {
            Self::SamplerYcbcrConversion(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
impl From<Unique<SamplerYcbcrConversion>> for Handles {
    fn from(value: Unique<SamplerYcbcrConversion>) -> Self {
        Self::SamplerYcbcrConversion(value)
    }
}
impl Into<Unique<SamplerYcbcrConversion>> for Handles {
    fn into(self) -> Unique<SamplerYcbcrConversion> {
        self.as_sampler_ycbcr_conversion()
    }
}
#[cfg(feature = "VK_EXT_validation_cache")]
impl Handles {
    ///Checks if the handle is a `ValidationCacheEXT`
    #[inline]
    pub fn is_validation_cache_ext(&self) -> bool {
        matches!(self, Self::ValidationCacheEXT(_))
    }
    ///Gets the handle as a `ValidationCacheEXT`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_validation_cache_ext(self) -> Unique<ValidationCacheEXT> {
        self.try_as_validation_cache_ext()
            .expect("Handle is not a `ValidationCacheEXT`")
    }
    ///Gets the handle as a `ValidationCacheEXT`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_validation_cache_ext(self) -> Option<Unique<ValidationCacheEXT>> {
        match self {
            Self::ValidationCacheEXT(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `ValidationCacheEXT`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_validation_cache_ext_unchecked(self) -> Unique<ValidationCacheEXT> {
        match self {
            Self::ValidationCacheEXT(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
#[cfg(feature = "VK_EXT_validation_cache")]
impl From<Unique<ValidationCacheEXT>> for Handles {
    fn from(value: Unique<ValidationCacheEXT>) -> Self {
        Self::ValidationCacheEXT(value)
    }
}
#[cfg(feature = "VK_EXT_validation_cache")]
impl Into<Unique<ValidationCacheEXT>> for Handles {
    fn into(self) -> Unique<ValidationCacheEXT> {
        self.as_validation_cache_ext()
    }
}
#[cfg(feature = "VK_KHR_acceleration_structure")]
impl Handles {
    ///Checks if the handle is a `AccelerationStructureKHR`
    #[inline]
    pub fn is_acceleration_structure_khr(&self) -> bool {
        matches!(self, Self::AccelerationStructureKHR(_))
    }
    ///Gets the handle as a `AccelerationStructureKHR`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_acceleration_structure_khr(self) -> Unique<AccelerationStructureKHR> {
        self.try_as_acceleration_structure_khr()
            .expect("Handle is not a `AccelerationStructureKHR`")
    }
    ///Gets the handle as a `AccelerationStructureKHR`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_acceleration_structure_khr(self) -> Option<Unique<AccelerationStructureKHR>> {
        match self {
            Self::AccelerationStructureKHR(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `AccelerationStructureKHR`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_acceleration_structure_khr_unchecked(self) -> Unique<AccelerationStructureKHR> {
        match self {
            Self::AccelerationStructureKHR(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
#[cfg(feature = "VK_KHR_acceleration_structure")]
impl From<Unique<AccelerationStructureKHR>> for Handles {
    fn from(value: Unique<AccelerationStructureKHR>) -> Self {
        Self::AccelerationStructureKHR(value)
    }
}
#[cfg(feature = "VK_KHR_acceleration_structure")]
impl Into<Unique<AccelerationStructureKHR>> for Handles {
    fn into(self) -> Unique<AccelerationStructureKHR> {
        self.as_acceleration_structure_khr()
    }
}
#[cfg(feature = "VK_NV_ray_tracing")]
impl Handles {
    ///Checks if the handle is a `AccelerationStructureNV`
    #[inline]
    pub fn is_acceleration_structure_nv(&self) -> bool {
        matches!(self, Self::AccelerationStructureNV(_))
    }
    ///Gets the handle as a `AccelerationStructureNV`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_acceleration_structure_nv(self) -> Unique<AccelerationStructureNV> {
        self.try_as_acceleration_structure_nv()
            .expect("Handle is not a `AccelerationStructureNV`")
    }
    ///Gets the handle as a `AccelerationStructureNV`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_acceleration_structure_nv(self) -> Option<Unique<AccelerationStructureNV>> {
        match self {
            Self::AccelerationStructureNV(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `AccelerationStructureNV`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_acceleration_structure_nv_unchecked(self) -> Unique<AccelerationStructureNV> {
        match self {
            Self::AccelerationStructureNV(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
#[cfg(feature = "VK_NV_ray_tracing")]
impl From<Unique<AccelerationStructureNV>> for Handles {
    fn from(value: Unique<AccelerationStructureNV>) -> Self {
        Self::AccelerationStructureNV(value)
    }
}
#[cfg(feature = "VK_NV_ray_tracing")]
impl Into<Unique<AccelerationStructureNV>> for Handles {
    fn into(self) -> Unique<AccelerationStructureNV> {
        self.as_acceleration_structure_nv()
    }
}
#[cfg(feature = "VK_INTEL_performance_query")]
impl Handles {
    ///Checks if the handle is a `PerformanceConfigurationINTEL`
    #[inline]
    pub fn is_performance_configuration_intel(&self) -> bool {
        matches!(self, Self::PerformanceConfigurationINTEL(_))
    }
    ///Gets the handle as a `PerformanceConfigurationINTEL`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_performance_configuration_intel(self) -> Unique<PerformanceConfigurationINTEL> {
        self.try_as_performance_configuration_intel()
            .expect("Handle is not a `PerformanceConfigurationINTEL`")
    }
    ///Gets the handle as a `PerformanceConfigurationINTEL`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_performance_configuration_intel(self) -> Option<Unique<PerformanceConfigurationINTEL>> {
        match self {
            Self::PerformanceConfigurationINTEL(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `PerformanceConfigurationINTEL`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_performance_configuration_intel_unchecked(self) -> Unique<PerformanceConfigurationINTEL> {
        match self {
            Self::PerformanceConfigurationINTEL(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
#[cfg(feature = "VK_INTEL_performance_query")]
impl From<Unique<PerformanceConfigurationINTEL>> for Handles {
    fn from(value: Unique<PerformanceConfigurationINTEL>) -> Self {
        Self::PerformanceConfigurationINTEL(value)
    }
}
#[cfg(feature = "VK_INTEL_performance_query")]
impl Into<Unique<PerformanceConfigurationINTEL>> for Handles {
    fn into(self) -> Unique<PerformanceConfigurationINTEL> {
        self.as_performance_configuration_intel()
    }
}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
impl Handles {
    ///Checks if the handle is a `BufferCollectionFUCHSIA`
    #[inline]
    pub fn is_buffer_collection_fuchsia(&self) -> bool {
        matches!(self, Self::BufferCollectionFUCHSIA(_))
    }
    ///Gets the handle as a `BufferCollectionFUCHSIA`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_buffer_collection_fuchsia(self) -> Unique<BufferCollectionFUCHSIA> {
        self.try_as_buffer_collection_fuchsia()
            .expect("Handle is not a `BufferCollectionFUCHSIA`")
    }
    ///Gets the handle as a `BufferCollectionFUCHSIA`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_buffer_collection_fuchsia(self) -> Option<Unique<BufferCollectionFUCHSIA>> {
        match self {
            Self::BufferCollectionFUCHSIA(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `BufferCollectionFUCHSIA`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_buffer_collection_fuchsia_unchecked(self) -> Unique<BufferCollectionFUCHSIA> {
        match self {
            Self::BufferCollectionFUCHSIA(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
impl From<Unique<BufferCollectionFUCHSIA>> for Handles {
    fn from(value: Unique<BufferCollectionFUCHSIA>) -> Self {
        Self::BufferCollectionFUCHSIA(value)
    }
}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
impl Into<Unique<BufferCollectionFUCHSIA>> for Handles {
    fn into(self) -> Unique<BufferCollectionFUCHSIA> {
        self.as_buffer_collection_fuchsia()
    }
}
#[cfg(feature = "VK_KHR_deferred_host_operations")]
impl Handles {
    ///Checks if the handle is a `DeferredOperationKHR`
    #[inline]
    pub fn is_deferred_operation_khr(&self) -> bool {
        matches!(self, Self::DeferredOperationKHR(_))
    }
    ///Gets the handle as a `DeferredOperationKHR`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_deferred_operation_khr(self) -> Unique<DeferredOperationKHR> {
        self.try_as_deferred_operation_khr()
            .expect("Handle is not a `DeferredOperationKHR`")
    }
    ///Gets the handle as a `DeferredOperationKHR`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_deferred_operation_khr(self) -> Option<Unique<DeferredOperationKHR>> {
        match self {
            Self::DeferredOperationKHR(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `DeferredOperationKHR`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_deferred_operation_khr_unchecked(self) -> Unique<DeferredOperationKHR> {
        match self {
            Self::DeferredOperationKHR(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
#[cfg(feature = "VK_KHR_deferred_host_operations")]
impl From<Unique<DeferredOperationKHR>> for Handles {
    fn from(value: Unique<DeferredOperationKHR>) -> Self {
        Self::DeferredOperationKHR(value)
    }
}
#[cfg(feature = "VK_KHR_deferred_host_operations")]
impl Into<Unique<DeferredOperationKHR>> for Handles {
    fn into(self) -> Unique<DeferredOperationKHR> {
        self.as_deferred_operation_khr()
    }
}
impl Handles {
    ///Checks if the handle is a `PrivateDataSlot`
    #[inline]
    pub fn is_private_data_slot(&self) -> bool {
        matches!(self, Self::PrivateDataSlot(_))
    }
    ///Gets the handle as a `PrivateDataSlot`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_private_data_slot(self) -> Unique<PrivateDataSlot> {
        self.try_as_private_data_slot()
            .expect("Handle is not a `PrivateDataSlot`")
    }
    ///Gets the handle as a `PrivateDataSlot`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_private_data_slot(self) -> Option<Unique<PrivateDataSlot>> {
        match self {
            Self::PrivateDataSlot(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `PrivateDataSlot`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_private_data_slot_unchecked(self) -> Unique<PrivateDataSlot> {
        match self {
            Self::PrivateDataSlot(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
impl From<Unique<PrivateDataSlot>> for Handles {
    fn from(value: Unique<PrivateDataSlot>) -> Self {
        Self::PrivateDataSlot(value)
    }
}
impl Into<Unique<PrivateDataSlot>> for Handles {
    fn into(self) -> Unique<PrivateDataSlot> {
        self.as_private_data_slot()
    }
}
#[cfg(feature = "VK_NVX_binary_import")]
impl Handles {
    ///Checks if the handle is a `CuModuleNVX`
    #[inline]
    pub fn is_cu_module_nvx(&self) -> bool {
        matches!(self, Self::CuModuleNVX(_))
    }
    ///Gets the handle as a `CuModuleNVX`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_cu_module_nvx(self) -> Unique<CuModuleNVX> {
        self.try_as_cu_module_nvx().expect("Handle is not a `CuModuleNVX`")
    }
    ///Gets the handle as a `CuModuleNVX`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_cu_module_nvx(self) -> Option<Unique<CuModuleNVX>> {
        match self {
            Self::CuModuleNVX(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `CuModuleNVX`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_cu_module_nvx_unchecked(self) -> Unique<CuModuleNVX> {
        match self {
            Self::CuModuleNVX(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
#[cfg(feature = "VK_NVX_binary_import")]
impl From<Unique<CuModuleNVX>> for Handles {
    fn from(value: Unique<CuModuleNVX>) -> Self {
        Self::CuModuleNVX(value)
    }
}
#[cfg(feature = "VK_NVX_binary_import")]
impl Into<Unique<CuModuleNVX>> for Handles {
    fn into(self) -> Unique<CuModuleNVX> {
        self.as_cu_module_nvx()
    }
}
#[cfg(feature = "VK_NVX_binary_import")]
impl Handles {
    ///Checks if the handle is a `CuFunctionNVX`
    #[inline]
    pub fn is_cu_function_nvx(&self) -> bool {
        matches!(self, Self::CuFunctionNVX(_))
    }
    ///Gets the handle as a `CuFunctionNVX`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_cu_function_nvx(self) -> Unique<CuFunctionNVX> {
        self.try_as_cu_function_nvx().expect("Handle is not a `CuFunctionNVX`")
    }
    ///Gets the handle as a `CuFunctionNVX`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_cu_function_nvx(self) -> Option<Unique<CuFunctionNVX>> {
        match self {
            Self::CuFunctionNVX(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `CuFunctionNVX`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_cu_function_nvx_unchecked(self) -> Unique<CuFunctionNVX> {
        match self {
            Self::CuFunctionNVX(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
#[cfg(feature = "VK_NVX_binary_import")]
impl From<Unique<CuFunctionNVX>> for Handles {
    fn from(value: Unique<CuFunctionNVX>) -> Self {
        Self::CuFunctionNVX(value)
    }
}
#[cfg(feature = "VK_NVX_binary_import")]
impl Into<Unique<CuFunctionNVX>> for Handles {
    fn into(self) -> Unique<CuFunctionNVX> {
        self.as_cu_function_nvx()
    }
}
#[cfg(feature = "VK_KHR_display")]
impl Handles {
    ///Checks if the handle is a `DisplayKHR`
    #[inline]
    pub fn is_display_khr(&self) -> bool {
        matches!(self, Self::DisplayKHR(_))
    }
    ///Gets the handle as a `DisplayKHR`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_display_khr(self) -> Unique<DisplayKHR> {
        self.try_as_display_khr().expect("Handle is not a `DisplayKHR`")
    }
    ///Gets the handle as a `DisplayKHR`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_display_khr(self) -> Option<Unique<DisplayKHR>> {
        match self {
            Self::DisplayKHR(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `DisplayKHR`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_display_khr_unchecked(self) -> Unique<DisplayKHR> {
        match self {
            Self::DisplayKHR(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
#[cfg(feature = "VK_KHR_display")]
impl From<Unique<DisplayKHR>> for Handles {
    fn from(value: Unique<DisplayKHR>) -> Self {
        Self::DisplayKHR(value)
    }
}
#[cfg(feature = "VK_KHR_display")]
impl Into<Unique<DisplayKHR>> for Handles {
    fn into(self) -> Unique<DisplayKHR> {
        self.as_display_khr()
    }
}
#[cfg(feature = "VK_KHR_display")]
impl Handles {
    ///Checks if the handle is a `DisplayModeKHR`
    #[inline]
    pub fn is_display_mode_khr(&self) -> bool {
        matches!(self, Self::DisplayModeKHR(_))
    }
    ///Gets the handle as a `DisplayModeKHR`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_display_mode_khr(self) -> Unique<DisplayModeKHR> {
        self.try_as_display_mode_khr()
            .expect("Handle is not a `DisplayModeKHR`")
    }
    ///Gets the handle as a `DisplayModeKHR`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_display_mode_khr(self) -> Option<Unique<DisplayModeKHR>> {
        match self {
            Self::DisplayModeKHR(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `DisplayModeKHR`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_display_mode_khr_unchecked(self) -> Unique<DisplayModeKHR> {
        match self {
            Self::DisplayModeKHR(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
#[cfg(feature = "VK_KHR_display")]
impl From<Unique<DisplayModeKHR>> for Handles {
    fn from(value: Unique<DisplayModeKHR>) -> Self {
        Self::DisplayModeKHR(value)
    }
}
#[cfg(feature = "VK_KHR_display")]
impl Into<Unique<DisplayModeKHR>> for Handles {
    fn into(self) -> Unique<DisplayModeKHR> {
        self.as_display_mode_khr()
    }
}
#[cfg(feature = "VK_KHR_surface")]
impl Handles {
    ///Checks if the handle is a `SurfaceKHR`
    #[inline]
    pub fn is_surface_khr(&self) -> bool {
        matches!(self, Self::SurfaceKHR(_))
    }
    ///Gets the handle as a `SurfaceKHR`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_surface_khr(self) -> Unique<SurfaceKHR> {
        self.try_as_surface_khr().expect("Handle is not a `SurfaceKHR`")
    }
    ///Gets the handle as a `SurfaceKHR`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_surface_khr(self) -> Option<Unique<SurfaceKHR>> {
        match self {
            Self::SurfaceKHR(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `SurfaceKHR`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_surface_khr_unchecked(self) -> Unique<SurfaceKHR> {
        match self {
            Self::SurfaceKHR(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
#[cfg(feature = "VK_KHR_surface")]
impl From<Unique<SurfaceKHR>> for Handles {
    fn from(value: Unique<SurfaceKHR>) -> Self {
        Self::SurfaceKHR(value)
    }
}
#[cfg(feature = "VK_KHR_surface")]
impl Into<Unique<SurfaceKHR>> for Handles {
    fn into(self) -> Unique<SurfaceKHR> {
        self.as_surface_khr()
    }
}
#[cfg(feature = "VK_KHR_swapchain")]
impl Handles {
    ///Checks if the handle is a `SwapchainKHR`
    #[inline]
    pub fn is_swapchain_khr(&self) -> bool {
        matches!(self, Self::SwapchainKHR(_))
    }
    ///Gets the handle as a `SwapchainKHR`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_swapchain_khr(self) -> Unique<SwapchainKHR> {
        self.try_as_swapchain_khr().expect("Handle is not a `SwapchainKHR`")
    }
    ///Gets the handle as a `SwapchainKHR`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_swapchain_khr(self) -> Option<Unique<SwapchainKHR>> {
        match self {
            Self::SwapchainKHR(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `SwapchainKHR`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_swapchain_khr_unchecked(self) -> Unique<SwapchainKHR> {
        match self {
            Self::SwapchainKHR(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
#[cfg(feature = "VK_KHR_swapchain")]
impl From<Unique<SwapchainKHR>> for Handles {
    fn from(value: Unique<SwapchainKHR>) -> Self {
        Self::SwapchainKHR(value)
    }
}
#[cfg(feature = "VK_KHR_swapchain")]
impl Into<Unique<SwapchainKHR>> for Handles {
    fn into(self) -> Unique<SwapchainKHR> {
        self.as_swapchain_khr()
    }
}
#[cfg(feature = "VK_EXT_debug_report")]
impl Handles {
    ///Checks if the handle is a `DebugReportCallbackEXT`
    #[inline]
    pub fn is_debug_report_callback_ext(&self) -> bool {
        matches!(self, Self::DebugReportCallbackEXT(_))
    }
    ///Gets the handle as a `DebugReportCallbackEXT`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_debug_report_callback_ext(self) -> Unique<DebugReportCallbackEXT> {
        self.try_as_debug_report_callback_ext()
            .expect("Handle is not a `DebugReportCallbackEXT`")
    }
    ///Gets the handle as a `DebugReportCallbackEXT`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_debug_report_callback_ext(self) -> Option<Unique<DebugReportCallbackEXT>> {
        match self {
            Self::DebugReportCallbackEXT(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `DebugReportCallbackEXT`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_debug_report_callback_ext_unchecked(self) -> Unique<DebugReportCallbackEXT> {
        match self {
            Self::DebugReportCallbackEXT(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
#[cfg(feature = "VK_EXT_debug_report")]
impl From<Unique<DebugReportCallbackEXT>> for Handles {
    fn from(value: Unique<DebugReportCallbackEXT>) -> Self {
        Self::DebugReportCallbackEXT(value)
    }
}
#[cfg(feature = "VK_EXT_debug_report")]
impl Into<Unique<DebugReportCallbackEXT>> for Handles {
    fn into(self) -> Unique<DebugReportCallbackEXT> {
        self.as_debug_report_callback_ext()
    }
}
#[cfg(feature = "VK_EXT_debug_utils")]
impl Handles {
    ///Checks if the handle is a `DebugUtilsMessengerEXT`
    #[inline]
    pub fn is_debug_utils_messenger_ext(&self) -> bool {
        matches!(self, Self::DebugUtilsMessengerEXT(_))
    }
    ///Gets the handle as a `DebugUtilsMessengerEXT`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_debug_utils_messenger_ext(self) -> Unique<DebugUtilsMessengerEXT> {
        self.try_as_debug_utils_messenger_ext()
            .expect("Handle is not a `DebugUtilsMessengerEXT`")
    }
    ///Gets the handle as a `DebugUtilsMessengerEXT`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_debug_utils_messenger_ext(self) -> Option<Unique<DebugUtilsMessengerEXT>> {
        match self {
            Self::DebugUtilsMessengerEXT(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `DebugUtilsMessengerEXT`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_debug_utils_messenger_ext_unchecked(self) -> Unique<DebugUtilsMessengerEXT> {
        match self {
            Self::DebugUtilsMessengerEXT(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
#[cfg(feature = "VK_EXT_debug_utils")]
impl From<Unique<DebugUtilsMessengerEXT>> for Handles {
    fn from(value: Unique<DebugUtilsMessengerEXT>) -> Self {
        Self::DebugUtilsMessengerEXT(value)
    }
}
#[cfg(feature = "VK_EXT_debug_utils")]
impl Into<Unique<DebugUtilsMessengerEXT>> for Handles {
    fn into(self) -> Unique<DebugUtilsMessengerEXT> {
        self.as_debug_utils_messenger_ext()
    }
}
#[cfg(feature = "VK_KHR_video_queue")]
impl Handles {
    ///Checks if the handle is a `VideoSessionKHR`
    #[inline]
    pub fn is_video_session_khr(&self) -> bool {
        matches!(self, Self::VideoSessionKHR(_))
    }
    ///Gets the handle as a `VideoSessionKHR`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_video_session_khr(self) -> Unique<VideoSessionKHR> {
        self.try_as_video_session_khr()
            .expect("Handle is not a `VideoSessionKHR`")
    }
    ///Gets the handle as a `VideoSessionKHR`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_video_session_khr(self) -> Option<Unique<VideoSessionKHR>> {
        match self {
            Self::VideoSessionKHR(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `VideoSessionKHR`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_video_session_khr_unchecked(self) -> Unique<VideoSessionKHR> {
        match self {
            Self::VideoSessionKHR(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
#[cfg(feature = "VK_KHR_video_queue")]
impl From<Unique<VideoSessionKHR>> for Handles {
    fn from(value: Unique<VideoSessionKHR>) -> Self {
        Self::VideoSessionKHR(value)
    }
}
#[cfg(feature = "VK_KHR_video_queue")]
impl Into<Unique<VideoSessionKHR>> for Handles {
    fn into(self) -> Unique<VideoSessionKHR> {
        self.as_video_session_khr()
    }
}
#[cfg(feature = "VK_KHR_video_queue")]
impl Handles {
    ///Checks if the handle is a `VideoSessionParametersKHR`
    #[inline]
    pub fn is_video_session_parameters_khr(&self) -> bool {
        matches!(self, Self::VideoSessionParametersKHR(_))
    }
    ///Gets the handle as a `VideoSessionParametersKHR`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_video_session_parameters_khr(self) -> Unique<VideoSessionParametersKHR> {
        self.try_as_video_session_parameters_khr()
            .expect("Handle is not a `VideoSessionParametersKHR`")
    }
    ///Gets the handle as a `VideoSessionParametersKHR`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_video_session_parameters_khr(self) -> Option<Unique<VideoSessionParametersKHR>> {
        match self {
            Self::VideoSessionParametersKHR(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `VideoSessionParametersKHR`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_video_session_parameters_khr_unchecked(self) -> Unique<VideoSessionParametersKHR> {
        match self {
            Self::VideoSessionParametersKHR(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
#[cfg(feature = "VK_KHR_video_queue")]
impl From<Unique<VideoSessionParametersKHR>> for Handles {
    fn from(value: Unique<VideoSessionParametersKHR>) -> Self {
        Self::VideoSessionParametersKHR(value)
    }
}
#[cfg(feature = "VK_KHR_video_queue")]
impl Into<Unique<VideoSessionParametersKHR>> for Handles {
    fn into(self) -> Unique<VideoSessionParametersKHR> {
        self.as_video_session_parameters_khr()
    }
}
#[cfg(feature = "VK_KHR_swapchain")]
impl Handles {
    ///Checks if the handle is a `SwapchainImage`
    #[inline]
    pub fn is_swapchain_image(&self) -> bool {
        matches!(self, Self::SwapchainImage(_))
    }
    ///Gets the handle as a `SwapchainImage`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_swapchain_image(self) -> Unique<SwapchainImage> {
        self.try_as_swapchain_image().expect("Handle is not a `SwapchainImage`")
    }
    ///Gets the handle as a `SwapchainImage`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_swapchain_image(self) -> Option<Unique<SwapchainImage>> {
        match self {
            Self::SwapchainImage(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `SwapchainImage`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_swapchain_image_unchecked(self) -> Unique<SwapchainImage> {
        match self {
            Self::SwapchainImage(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
#[cfg(feature = "VK_KHR_swapchain")]
impl From<Unique<SwapchainImage>> for Handles {
    fn from(value: Unique<SwapchainImage>) -> Self {
        Self::SwapchainImage(value)
    }
}
#[cfg(feature = "VK_KHR_swapchain")]
impl Into<Unique<SwapchainImage>> for Handles {
    fn into(self) -> Unique<SwapchainImage> {
        self.as_swapchain_image()
    }
}
#[cfg(feature = "VK_KHR_swapchain")]
impl Handles {
    ///Checks if the handle is a `SwapchainImageView`
    #[inline]
    pub fn is_swapchain_image_view(&self) -> bool {
        matches!(self, Self::SwapchainImageView(_))
    }
    ///Gets the handle as a `SwapchainImageView`, if it is not then panic
    #[track_caller]
    #[inline]
    pub fn as_swapchain_image_view(self) -> Unique<SwapchainImageView> {
        self.try_as_swapchain_image_view()
            .expect("Handle is not a `SwapchainImageView`")
    }
    ///Gets the handle as a `SwapchainImageView`, if it is not then returns [`None`]
    #[inline]
    pub fn try_as_swapchain_image_view(self) -> Option<Unique<SwapchainImageView>> {
        match self {
            Self::SwapchainImageView(value) => Some(value),
            _ => None,
        }
    }
    ///Gets the handle as a `SwapchainImageView`, if it is not then can cause UB
    #[inline]
    pub unsafe fn as_swapchain_image_view_unchecked(self) -> Unique<SwapchainImageView> {
        match self {
            Self::SwapchainImageView(value) => value,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
#[cfg(feature = "VK_KHR_swapchain")]
impl From<Unique<SwapchainImageView>> for Handles {
    fn from(value: Unique<SwapchainImageView>) -> Self {
        Self::SwapchainImageView(value)
    }
}
#[cfg(feature = "VK_KHR_swapchain")]
impl Into<Unique<SwapchainImageView>> for Handles {
    fn into(self) -> Unique<SwapchainImageView> {
        self.as_swapchain_image_view()
    }
}
