#[cfg(feature = "VK_EXT_calibrated_timestamps")]
use crate::native::extensions::ext_calibrated_timestamps::CalibratedTimestampInfoEXT;
#[cfg(feature = "VK_EXT_calibrated_timestamps")]
use crate::native::extensions::ext_calibrated_timestamps::TimeDomainEXT;
#[cfg(feature = "VK_EXT_conditional_rendering")]
use crate::native::extensions::ext_conditional_rendering::ConditionalRenderingBeginInfoEXT;
#[cfg(feature = "VK_EXT_debug_marker")]
use crate::native::extensions::ext_debug_marker::DebugMarkerMarkerInfoEXT;
#[cfg(feature = "VK_EXT_debug_marker")]
use crate::native::extensions::ext_debug_marker::DebugMarkerObjectNameInfoEXT;
#[cfg(feature = "VK_EXT_debug_marker")]
use crate::native::extensions::ext_debug_marker::DebugMarkerObjectTagInfoEXT;
#[cfg(feature = "VK_EXT_debug_marker")]
use crate::native::extensions::ext_debug_marker::DebugReportObjectTypeEXT;
#[cfg(feature = "VK_EXT_debug_report")]
use crate::native::extensions::ext_debug_report::DebugReportCallbackEXT;
#[cfg(feature = "VK_EXT_debug_report")]
use crate::native::extensions::ext_debug_report::DebugReportFlagsEXT;
#[cfg(feature = "VK_EXT_debug_utils")]
use crate::native::extensions::ext_debug_utils::DebugUtilsLabelEXT;
#[cfg(feature = "VK_EXT_debug_utils")]
use crate::native::extensions::ext_debug_utils::DebugUtilsMessageSeverityFlagBitsEXT;
#[cfg(feature = "VK_EXT_debug_utils")]
use crate::native::extensions::ext_debug_utils::DebugUtilsMessageTypeFlagsEXT;
#[cfg(feature = "VK_EXT_debug_utils")]
use crate::native::extensions::ext_debug_utils::DebugUtilsMessengerCallbackDataEXT;
#[cfg(feature = "VK_EXT_debug_utils")]
use crate::native::extensions::ext_debug_utils::DebugUtilsMessengerEXT;
#[cfg(feature = "VK_EXT_debug_utils")]
use crate::native::extensions::ext_debug_utils::DebugUtilsObjectNameInfoEXT;
#[cfg(feature = "VK_EXT_debug_utils")]
use crate::native::extensions::ext_debug_utils::DebugUtilsObjectTagInfoEXT;
#[cfg(feature = "VK_EXT_display_control")]
use crate::native::extensions::ext_display_control::DisplayPowerInfoEXT;
#[cfg(feature = "VK_EXT_display_surface_counter")]
use crate::native::extensions::ext_display_surface_counter::SurfaceCapabilities2EXT;
#[cfg(feature = "VK_EXT_display_surface_counter")]
use crate::native::extensions::ext_display_surface_counter::SurfaceCounterFlagBitsEXT;
#[cfg(feature = "VK_EXT_hdr_metadata")]
use crate::native::extensions::ext_hdr_metadata::HdrMetadataEXT;
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
use crate::native::extensions::ext_image_drm_format_modifier::ImageDrmFormatModifierPropertiesEXT;
#[cfg(feature = "VK_EXT_multi_draw")]
use crate::native::extensions::ext_multi_draw::MultiDrawIndexedInfoEXT;
#[cfg(feature = "VK_EXT_multi_draw")]
use crate::native::extensions::ext_multi_draw::MultiDrawInfoEXT;
#[cfg(feature = "VK_EXT_sample_locations")]
use crate::native::extensions::ext_sample_locations::MultisamplePropertiesEXT;
#[cfg(feature = "VK_EXT_sample_locations")]
use crate::native::extensions::ext_sample_locations::SampleLocationsInfoEXT;
#[cfg(feature = "VK_EXT_validation_cache")]
use crate::native::extensions::ext_validation_cache::ValidationCacheEXT;
#[cfg(feature = "VK_EXT_vertex_input_dynamic_state")]
use crate::native::extensions::ext_vertex_input_dynamic_state::VertexInputAttributeDescription2EXT;
#[cfg(feature = "VK_EXT_vertex_input_dynamic_state")]
use crate::native::extensions::ext_vertex_input_dynamic_state::VertexInputBindingDescription2EXT;
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
use crate::native::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA;
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
use crate::native::extensions::fuchsia_buffer_collection::BufferCollectionPropertiesFUCHSIA;
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
use crate::native::extensions::fuchsia_buffer_collection::BufferConstraintsInfoFUCHSIA;
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
use crate::native::extensions::fuchsia_buffer_collection::ImageConstraintsInfoFUCHSIA;
#[cfg(feature = "VK_GOOGLE_display_timing")]
use crate::native::extensions::google_display_timing::PastPresentationTimingGOOGLE;
#[cfg(feature = "VK_GOOGLE_display_timing")]
use crate::native::extensions::google_display_timing::RefreshCycleDurationGOOGLE;
#[cfg(feature = "VK_INTEL_performance_query")]
use crate::native::extensions::intel_performance_query::InitializePerformanceApiInfoINTEL;
#[cfg(feature = "VK_INTEL_performance_query")]
use crate::native::extensions::intel_performance_query::PerformanceConfigurationAcquireInfoINTEL;
#[cfg(feature = "VK_INTEL_performance_query")]
use crate::native::extensions::intel_performance_query::PerformanceConfigurationINTEL;
#[cfg(feature = "VK_INTEL_performance_query")]
use crate::native::extensions::intel_performance_query::PerformanceMarkerInfoINTEL;
#[cfg(feature = "VK_INTEL_performance_query")]
use crate::native::extensions::intel_performance_query::PerformanceOverrideInfoINTEL;
#[cfg(feature = "VK_INTEL_performance_query")]
use crate::native::extensions::intel_performance_query::PerformanceParameterTypeINTEL;
#[cfg(feature = "VK_INTEL_performance_query")]
use crate::native::extensions::intel_performance_query::PerformanceStreamMarkerInfoINTEL;
#[cfg(feature = "VK_INTEL_performance_query")]
use crate::native::extensions::intel_performance_query::PerformanceValueINTEL;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::native::extensions::khr_acceleration_structure::AccelerationStructureBuildGeometryInfoKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::native::extensions::khr_acceleration_structure::AccelerationStructureBuildRangeInfoKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::native::extensions::khr_acceleration_structure::AccelerationStructureBuildSizesInfoKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::native::extensions::khr_acceleration_structure::AccelerationStructureBuildTypeKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::native::extensions::khr_acceleration_structure::AccelerationStructureCompatibilityKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::native::extensions::khr_acceleration_structure::AccelerationStructureDeviceAddressInfoKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::native::extensions::khr_acceleration_structure::AccelerationStructureKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::native::extensions::khr_acceleration_structure::AccelerationStructureVersionInfoKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::native::extensions::khr_acceleration_structure::CopyAccelerationStructureInfoKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::native::extensions::khr_acceleration_structure::CopyAccelerationStructureModeKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::native::extensions::khr_acceleration_structure::CopyAccelerationStructureToMemoryInfoKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::native::extensions::khr_acceleration_structure::CopyMemoryToAccelerationStructureInfoKHR;
#[cfg(feature = "VK_KHR_deferred_host_operations")]
use crate::native::extensions::khr_deferred_host_operations::DeferredOperationKHR;
#[cfg(feature = "VK_KHR_device_group")]
use crate::native::extensions::khr_device_group::AcquireNextImageInfoKHR;
#[cfg(feature = "VK_KHR_device_group")]
use crate::native::extensions::khr_device_group::DeviceGroupPresentCapabilitiesKHR;
#[cfg(feature = "VK_KHR_device_group")]
use crate::native::extensions::khr_device_group::DeviceGroupPresentModeFlagsKHR;
#[cfg(feature = "VK_KHR_display")]
use crate::native::extensions::khr_display::DisplayKHR;
#[cfg(feature = "VK_KHR_display")]
use crate::native::extensions::khr_display::DisplayModeKHR;
#[cfg(feature = "VK_KHR_display")]
use crate::native::extensions::khr_display::DisplayModePropertiesKHR;
#[cfg(feature = "VK_KHR_display")]
use crate::native::extensions::khr_display::DisplayPlaneCapabilitiesKHR;
#[cfg(feature = "VK_KHR_display")]
use crate::native::extensions::khr_display::DisplayPlanePropertiesKHR;
#[cfg(feature = "VK_KHR_display")]
use crate::native::extensions::khr_display::DisplayPropertiesKHR;
#[cfg(feature = "VK_KHR_external_fence_fd")]
use crate::native::extensions::khr_external_fence_fd::FenceGetFdInfoKHR;
#[cfg(feature = "VK_KHR_external_fence_fd")]
use crate::native::extensions::khr_external_fence_fd::ImportFenceFdInfoKHR;
#[cfg(feature = "VK_KHR_external_memory_fd")]
use crate::native::extensions::khr_external_memory_fd::MemoryFdPropertiesKHR;
#[cfg(feature = "VK_KHR_external_memory_fd")]
use crate::native::extensions::khr_external_memory_fd::MemoryGetFdInfoKHR;
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
use crate::native::extensions::khr_external_semaphore_fd::ImportSemaphoreFdInfoKHR;
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
use crate::native::extensions::khr_external_semaphore_fd::SemaphoreGetFdInfoKHR;
#[cfg(feature = "VK_KHR_fragment_shading_rate")]
use crate::native::extensions::khr_fragment_shading_rate::FragmentShadingRateCombinerOpKHR;
#[cfg(feature = "VK_KHR_fragment_shading_rate")]
use crate::native::extensions::khr_fragment_shading_rate::PhysicalDeviceFragmentShadingRateKHR;
#[cfg(feature = "VK_KHR_get_display_properties2")]
use crate::native::extensions::khr_get_display_properties2::DisplayModeProperties2KHR;
#[cfg(feature = "VK_KHR_get_display_properties2")]
use crate::native::extensions::khr_get_display_properties2::DisplayPlaneCapabilities2KHR;
#[cfg(feature = "VK_KHR_get_display_properties2")]
use crate::native::extensions::khr_get_display_properties2::DisplayPlaneInfo2KHR;
#[cfg(feature = "VK_KHR_get_display_properties2")]
use crate::native::extensions::khr_get_display_properties2::DisplayPlaneProperties2KHR;
#[cfg(feature = "VK_KHR_get_display_properties2")]
use crate::native::extensions::khr_get_display_properties2::DisplayProperties2KHR;
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
use crate::native::extensions::khr_get_surface_capabilities2::PhysicalDeviceSurfaceInfo2KHR;
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
use crate::native::extensions::khr_get_surface_capabilities2::SurfaceCapabilities2KHR;
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
use crate::native::extensions::khr_get_surface_capabilities2::SurfaceFormat2KHR;
#[cfg(feature = "VK_KHR_performance_query")]
use crate::native::extensions::khr_performance_query::AcquireProfilingLockInfoKHR;
#[cfg(feature = "VK_KHR_performance_query")]
use crate::native::extensions::khr_performance_query::PerformanceCounterDescriptionKHR;
#[cfg(feature = "VK_KHR_performance_query")]
use crate::native::extensions::khr_performance_query::PerformanceCounterKHR;
#[cfg(feature = "VK_KHR_performance_query")]
use crate::native::extensions::khr_performance_query::QueryPoolPerformanceCreateInfoKHR;
#[cfg(feature = "VK_KHR_pipeline_executable_properties")]
use crate::native::extensions::khr_pipeline_executable_properties::PipelineExecutableInfoKHR;
#[cfg(feature = "VK_KHR_pipeline_executable_properties")]
use crate::native::extensions::khr_pipeline_executable_properties::PipelineExecutableInternalRepresentationKHR;
#[cfg(feature = "VK_KHR_pipeline_executable_properties")]
use crate::native::extensions::khr_pipeline_executable_properties::PipelineExecutablePropertiesKHR;
#[cfg(feature = "VK_KHR_pipeline_executable_properties")]
use crate::native::extensions::khr_pipeline_executable_properties::PipelineExecutableStatisticKHR;
#[cfg(feature = "VK_KHR_pipeline_executable_properties")]
use crate::native::extensions::khr_pipeline_executable_properties::PipelineInfoKHR;
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
use crate::native::extensions::khr_ray_tracing_pipeline::ShaderGroupShaderKHR;
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
use crate::native::extensions::khr_ray_tracing_pipeline::StridedDeviceAddressRegionKHR;
#[cfg(feature = "VK_KHR_surface")]
use crate::native::extensions::khr_surface::PresentModeKHR;
#[cfg(feature = "VK_KHR_surface")]
use crate::native::extensions::khr_surface::SurfaceCapabilitiesKHR;
#[cfg(feature = "VK_KHR_surface")]
use crate::native::extensions::khr_surface::SurfaceFormatKHR;
#[cfg(feature = "VK_KHR_surface")]
use crate::native::extensions::khr_surface::SurfaceKHR;
#[cfg(feature = "VK_KHR_swapchain")]
use crate::native::extensions::khr_swapchain::PresentInfoKHR;
#[cfg(feature = "VK_KHR_swapchain")]
use crate::native::extensions::khr_swapchain::SwapchainKHR;
#[cfg(feature = "VK_KHR_video_decode_queue")]
use crate::native::extensions::khr_video_decode_queue::VideoDecodeInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::native::extensions::khr_video_encode_queue::VideoEncodeInfoKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::native::extensions::khr_video_queue::PhysicalDeviceVideoFormatInfoKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::native::extensions::khr_video_queue::VideoBeginCodingInfoKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::native::extensions::khr_video_queue::VideoBindMemoryKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::native::extensions::khr_video_queue::VideoCapabilitiesKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::native::extensions::khr_video_queue::VideoCodingControlInfoKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::native::extensions::khr_video_queue::VideoEndCodingInfoKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::native::extensions::khr_video_queue::VideoFormatPropertiesKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::native::extensions::khr_video_queue::VideoGetMemoryPropertiesKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::native::extensions::khr_video_queue::VideoProfileKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::native::extensions::khr_video_queue::VideoSessionKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::native::extensions::khr_video_queue::VideoSessionParametersKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::native::extensions::khr_video_queue::VideoSessionParametersUpdateInfoKHR;
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
use crate::native::extensions::nv_clip_space_w_scaling::ViewportWScalingNV;
#[cfg(feature = "VK_NV_cooperative_matrix")]
use crate::native::extensions::nv_cooperative_matrix::CooperativeMatrixPropertiesNV;
#[cfg(feature = "VK_NV_coverage_reduction_mode")]
use crate::native::extensions::nv_coverage_reduction_mode::FramebufferMixedSamplesCombinationNV;
#[cfg(feature = "VK_NV_device_generated_commands")]
use crate::native::extensions::nv_device_generated_commands::GeneratedCommandsInfoNV;
#[cfg(feature = "VK_NV_device_generated_commands")]
use crate::native::extensions::nv_device_generated_commands::GeneratedCommandsMemoryRequirementsInfoNV;
#[cfg(feature = "VK_NV_device_generated_commands")]
use crate::native::extensions::nv_device_generated_commands::IndirectCommandsLayoutNV;
#[cfg(feature = "VK_NV_external_memory_capabilities")]
use crate::native::extensions::nv_external_memory_capabilities::ExternalImageFormatPropertiesNV;
#[cfg(feature = "VK_NV_external_memory_capabilities")]
use crate::native::extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV;
#[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
use crate::native::extensions::nv_fragment_shading_rate_enums::FragmentShadingRateNV;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::native::extensions::nv_ray_tracing::AccelerationStructureInfoNV;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::native::extensions::nv_ray_tracing::AccelerationStructureMemoryRequirementsInfoNV;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::native::extensions::nv_ray_tracing::AccelerationStructureNV;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::native::extensions::nv_ray_tracing::BindAccelerationStructureMemoryInfoNV;
#[cfg(feature = "VK_NV_shading_rate_image")]
use crate::native::extensions::nv_shading_rate_image::CoarseSampleOrderCustomNV;
#[cfg(feature = "VK_NV_shading_rate_image")]
use crate::native::extensions::nv_shading_rate_image::CoarseSampleOrderTypeNV;
#[cfg(feature = "VK_NV_shading_rate_image")]
use crate::native::extensions::nv_shading_rate_image::ShadingRatePaletteNV;
#[cfg(feature = "VK_NVX_binary_import")]
use crate::native::extensions::nvx_binary_import::CuFunctionNVX;
#[cfg(feature = "VK_NVX_binary_import")]
use crate::native::extensions::nvx_binary_import::CuModuleNVX;
#[cfg(feature = "VK_NVX_image_view_handle")]
use crate::native::extensions::nvx_image_view_handle::ImageViewAddressPropertiesNVX;
#[cfg(feature = "VK_NVX_image_view_handle")]
use crate::native::extensions::nvx_image_view_handle::ImageViewHandleInfoNVX;
#[cfg(feature = "VULKAN_1_2")]
use crate::native::vulkan1_2::BufferDeviceAddressInfo;
#[cfg(feature = "VULKAN_1_2")]
use crate::native::vulkan1_2::DeviceMemoryOpaqueCaptureAddressInfo;
#[cfg(feature = "VULKAN_1_2")]
use crate::native::vulkan1_2::SemaphoreSignalInfo;
#[cfg(feature = "VULKAN_1_2")]
use crate::native::vulkan1_2::SemaphoreWaitInfo;
#[cfg(feature = "VULKAN_1_2")]
use crate::native::vulkan1_2::SubpassBeginInfo;
#[cfg(feature = "VULKAN_1_2")]
use crate::native::vulkan1_2::SubpassEndInfo;
#[cfg(feature = "VULKAN_1_3")]
use crate::native::vulkan1_3::BlitImageInfo2;
#[cfg(feature = "VULKAN_1_3")]
use crate::native::vulkan1_3::CopyBufferInfo2;
#[cfg(feature = "VULKAN_1_3")]
use crate::native::vulkan1_3::CopyBufferToImageInfo2;
#[cfg(feature = "VULKAN_1_3")]
use crate::native::vulkan1_3::CopyImageInfo2;
#[cfg(feature = "VULKAN_1_3")]
use crate::native::vulkan1_3::CopyImageToBufferInfo2;
#[cfg(feature = "VULKAN_1_3")]
use crate::native::vulkan1_3::DependencyInfo;
#[cfg(feature = "VULKAN_1_3")]
use crate::native::vulkan1_3::DeviceBufferMemoryRequirements;
#[cfg(feature = "VULKAN_1_3")]
use crate::native::vulkan1_3::DeviceImageMemoryRequirements;
#[cfg(feature = "VULKAN_1_3")]
use crate::native::vulkan1_3::PhysicalDeviceToolProperties;
#[cfg(feature = "VULKAN_1_3")]
use crate::native::vulkan1_3::PipelineStageFlags2;
#[cfg(feature = "VULKAN_1_3")]
use crate::native::vulkan1_3::PrivateDataSlot;
#[cfg(feature = "VULKAN_1_3")]
use crate::native::vulkan1_3::RenderingInfo;
#[cfg(feature = "VULKAN_1_3")]
use crate::native::vulkan1_3::ResolveImageInfo2;
#[cfg(feature = "VULKAN_1_3")]
use crate::native::vulkan1_3::SubmitInfo2;
use crate::{
    context::{Container, ObjectId},
    native::{
        vulkan1_0::{
            BindSparseInfo, Bool32, Buffer, BufferCopy, BufferImageCopy, BufferMemoryBarrier, BufferView,
            ClearAttachment, ClearColorValue, ClearDepthStencilValue, ClearRect, CommandBuffer,
            CommandBufferAllocateInfo, CommandBufferBeginInfo, CommandBufferResetFlags, CommandPool,
            CommandPoolResetFlags, CompareOp, CopyDescriptorSet, CullModeFlags, DependencyFlags, DescriptorPool,
            DescriptorPoolResetFlags, DescriptorSet, DescriptorSetAllocateInfo, DescriptorSetLayout,
            DescriptorSetLayoutCreateInfo, Device, DeviceAddress, DeviceMemory, DeviceSize, Event, ExtensionProperties,
            Extent2D, Fence, Filter, Format, FormatProperties, Framebuffer, FrontFace, Image, ImageBlit, ImageCopy,
            ImageCreateFlags, ImageFormatProperties, ImageLayout, ImageMemoryBarrier, ImageResolve, ImageSubresource,
            ImageSubresourceRange, ImageTiling, ImageType, ImageUsageFlags, ImageView, IndexType, Instance,
            LayerProperties, LogicOp, MappedMemoryRange, MemoryBarrier, MemoryRequirements, ObjectType, PhysicalDevice,
            PhysicalDeviceFeatures, PhysicalDeviceMemoryProperties, PhysicalDeviceProperties, Pipeline,
            PipelineBindPoint, PipelineCache, PipelineLayout, PipelineStageFlagBits, PipelineStageFlags,
            PrimitiveTopology, QueryControlFlags, QueryPool, QueryResultFlags, QueryType, Queue, QueueFamilyProperties,
            Rect2D, RenderPass, RenderPassBeginInfo, SampleCountFlagBits, Sampler, Semaphore, ShaderModule,
            SparseImageFormatProperties, SparseImageMemoryRequirements, StencilFaceFlags, StencilOp, SubmitInfo,
            SubpassContents, SubresourceLayout, Viewport, WriteDescriptorSet,
        },
        vulkan1_1::{
            BindBufferMemoryInfo, BindImageMemoryInfo, BufferMemoryRequirementsInfo2, CommandPoolTrimFlags,
            DescriptorSetLayoutSupport, DescriptorUpdateTemplate, DeviceQueueInfo2, ExternalBufferProperties,
            ExternalFenceProperties, ExternalMemoryHandleTypeFlagBits, ExternalSemaphoreProperties, FormatProperties2,
            ImageFormatProperties2, ImageMemoryRequirementsInfo2, ImageSparseMemoryRequirementsInfo2,
            MemoryRequirements2, PeerMemoryFeatureFlags, PhysicalDeviceExternalBufferInfo,
            PhysicalDeviceExternalFenceInfo, PhysicalDeviceExternalSemaphoreInfo, PhysicalDeviceFeatures2,
            PhysicalDeviceGroupProperties, PhysicalDeviceImageFormatInfo2, PhysicalDeviceMemoryProperties2,
            PhysicalDeviceProperties2, PhysicalDeviceSparseImageFormatInfo2, QueueFamilyProperties2,
            SamplerYcbcrConversion, SparseImageFormatProperties2, SparseImageMemoryRequirements2,
        },
    },
    Entry,
};
use dashmap::DashMap;
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Object {
    Instance,
    PhysicalDevice,
    Device,
    Queue,
    CommandBuffer,
    DeviceMemory,
    CommandPool,
    Buffer,
    BufferView,
    Image,
    ImageView,
    ShaderModule,
    Pipeline,
    PipelineLayout,
    Sampler,
    DescriptorSet,
    DescriptorSetLayout,
    DescriptorPool,
    Fence,
    Semaphore,
    Event,
    QueryPool,
    Framebuffer,
    RenderPass,
    PipelineCache,
    DescriptorUpdateTemplate,
    SamplerYcbcrConversion,
    #[cfg(feature = "VULKAN_1_3")]
    PrivateDataSlot,
    #[cfg(feature = "VK_KHR_surface")]
    SurfaceKHR,
    #[cfg(feature = "VK_KHR_swapchain")]
    SwapchainKHR,
    #[cfg(feature = "VK_KHR_display")]
    DisplayKHR,
    #[cfg(feature = "VK_KHR_display")]
    DisplayModeKHR,
    #[cfg(feature = "VK_EXT_debug_report")]
    DebugReportCallbackEXT,
    #[cfg(feature = "VK_KHR_video_queue")]
    VideoSessionKHR,
    #[cfg(feature = "VK_KHR_video_queue")]
    VideoSessionParametersKHR,
    #[cfg(feature = "VK_NVX_binary_import")]
    CuModuleNVX,
    #[cfg(feature = "VK_NVX_binary_import")]
    CuFunctionNVX,
    #[cfg(feature = "VK_EXT_debug_utils")]
    DebugUtilsMessengerEXT,
    #[cfg(feature = "VK_KHR_acceleration_structure")]
    AccelerationStructureKHR,
    #[cfg(feature = "VK_EXT_validation_cache")]
    ValidationCacheEXT,
    #[cfg(feature = "VK_NV_ray_tracing")]
    AccelerationStructureNV,
    #[cfg(feature = "VK_INTEL_performance_query")]
    PerformanceConfigurationINTEL,
    #[cfg(feature = "VK_KHR_deferred_host_operations")]
    DeferredOperationKHR,
    #[cfg(feature = "VK_NV_device_generated_commands")]
    IndirectCommandsLayoutNV,
    #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
    BufferCollectionFUCHSIA,
}
pub struct Api {
    ///The loading Vulkan entry, required to create Vulkan instances.
    entry: Entry,
    objects: DashMap<ObjectId, Object>,
    instance: DashMap<ObjectId, Container<Instance>>,
    physical_device: DashMap<ObjectId, Container<PhysicalDevice>>,
    device: DashMap<ObjectId, Container<Device>>,
    queue: DashMap<ObjectId, Container<Queue>>,
    command_buffer: DashMap<ObjectId, Container<CommandBuffer>>,
    device_memory: DashMap<ObjectId, Container<DeviceMemory>>,
    command_pool: DashMap<ObjectId, Container<CommandPool>>,
    buffer: DashMap<ObjectId, Container<Buffer>>,
    buffer_view: DashMap<ObjectId, Container<BufferView>>,
    image: DashMap<ObjectId, Container<Image>>,
    image_view: DashMap<ObjectId, Container<ImageView>>,
    shader_module: DashMap<ObjectId, Container<ShaderModule>>,
    pipeline: DashMap<ObjectId, Container<Pipeline>>,
    pipeline_layout: DashMap<ObjectId, Container<PipelineLayout>>,
    sampler: DashMap<ObjectId, Container<Sampler>>,
    descriptor_set: DashMap<ObjectId, Container<DescriptorSet>>,
    descriptor_set_layout: DashMap<ObjectId, Container<DescriptorSetLayout>>,
    descriptor_pool: DashMap<ObjectId, Container<DescriptorPool>>,
    fence: DashMap<ObjectId, Container<Fence>>,
    semaphore: DashMap<ObjectId, Container<Semaphore>>,
    event: DashMap<ObjectId, Container<Event>>,
    query_pool: DashMap<ObjectId, Container<QueryPool>>,
    framebuffer: DashMap<ObjectId, Container<Framebuffer>>,
    render_pass: DashMap<ObjectId, Container<RenderPass>>,
    pipeline_cache: DashMap<ObjectId, Container<PipelineCache>>,
    descriptor_update_template: DashMap<ObjectId, Container<DescriptorUpdateTemplate>>,
    sampler_ycbcr_conversion: DashMap<ObjectId, Container<SamplerYcbcrConversion>>,
    #[cfg(feature = "VULKAN_1_3")]
    private_data_slot: DashMap<ObjectId, Container<PrivateDataSlot>>,
    #[cfg(feature = "VK_KHR_surface")]
    surface_khr: DashMap<ObjectId, Container<SurfaceKHR>>,
    #[cfg(feature = "VK_KHR_swapchain")]
    swapchain_khr: DashMap<ObjectId, Container<SwapchainKHR>>,
    #[cfg(feature = "VK_KHR_display")]
    display_khr: DashMap<ObjectId, Container<DisplayKHR>>,
    #[cfg(feature = "VK_KHR_display")]
    display_mode_khr: DashMap<ObjectId, Container<DisplayModeKHR>>,
    #[cfg(feature = "VK_EXT_debug_report")]
    debug_report_callback_ext: DashMap<ObjectId, Container<DebugReportCallbackEXT>>,
    #[cfg(feature = "VK_KHR_video_queue")]
    video_session_khr: DashMap<ObjectId, Container<VideoSessionKHR>>,
    #[cfg(feature = "VK_KHR_video_queue")]
    video_session_parameters_khr: DashMap<ObjectId, Container<VideoSessionParametersKHR>>,
    #[cfg(feature = "VK_NVX_binary_import")]
    cu_module_nvx: DashMap<ObjectId, Container<CuModuleNVX>>,
    #[cfg(feature = "VK_NVX_binary_import")]
    cu_function_nvx: DashMap<ObjectId, Container<CuFunctionNVX>>,
    #[cfg(feature = "VK_EXT_debug_utils")]
    debug_utils_messenger_ext: DashMap<ObjectId, Container<DebugUtilsMessengerEXT>>,
    #[cfg(feature = "VK_KHR_acceleration_structure")]
    acceleration_structure_khr: DashMap<ObjectId, Container<AccelerationStructureKHR>>,
    #[cfg(feature = "VK_EXT_validation_cache")]
    validation_cache_ext: DashMap<ObjectId, Container<ValidationCacheEXT>>,
    #[cfg(feature = "VK_NV_ray_tracing")]
    acceleration_structure_nv: DashMap<ObjectId, Container<AccelerationStructureNV>>,
    #[cfg(feature = "VK_INTEL_performance_query")]
    performance_configuration_intel: DashMap<ObjectId, Container<PerformanceConfigurationINTEL>>,
    #[cfg(feature = "VK_KHR_deferred_host_operations")]
    deferred_operation_khr: DashMap<ObjectId, Container<DeferredOperationKHR>>,
    #[cfg(feature = "VK_NV_device_generated_commands")]
    indirect_commands_layout_nv: DashMap<ObjectId, Container<IndirectCommandsLayoutNV>>,
    #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
    buffer_collection_fuchsia: DashMap<ObjectId, Container<BufferCollectionFUCHSIA>>,
}
impl std::fmt::Debug for Api {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut s = f.debug_struct("Api");
        {
            s.field(stringify!(instance), &self.instance.len());
        }
        {
            s.field(stringify!(physical_device), &self.physical_device.len());
        }
        {
            s.field(stringify!(device), &self.device.len());
        }
        {
            s.field(stringify!(queue), &self.queue.len());
        }
        {
            s.field(stringify!(command_buffer), &self.command_buffer.len());
        }
        {
            s.field(stringify!(device_memory), &self.device_memory.len());
        }
        {
            s.field(stringify!(command_pool), &self.command_pool.len());
        }
        {
            s.field(stringify!(buffer), &self.buffer.len());
        }
        {
            s.field(stringify!(buffer_view), &self.buffer_view.len());
        }
        {
            s.field(stringify!(image), &self.image.len());
        }
        {
            s.field(stringify!(image_view), &self.image_view.len());
        }
        {
            s.field(stringify!(shader_module), &self.shader_module.len());
        }
        {
            s.field(stringify!(pipeline), &self.pipeline.len());
        }
        {
            s.field(stringify!(pipeline_layout), &self.pipeline_layout.len());
        }
        {
            s.field(stringify!(sampler), &self.sampler.len());
        }
        {
            s.field(stringify!(descriptor_set), &self.descriptor_set.len());
        }
        {
            s.field(stringify!(descriptor_set_layout), &self.descriptor_set_layout.len());
        }
        {
            s.field(stringify!(descriptor_pool), &self.descriptor_pool.len());
        }
        {
            s.field(stringify!(fence), &self.fence.len());
        }
        {
            s.field(stringify!(semaphore), &self.semaphore.len());
        }
        {
            s.field(stringify!(event), &self.event.len());
        }
        {
            s.field(stringify!(query_pool), &self.query_pool.len());
        }
        {
            s.field(stringify!(framebuffer), &self.framebuffer.len());
        }
        {
            s.field(stringify!(render_pass), &self.render_pass.len());
        }
        {
            s.field(stringify!(pipeline_cache), &self.pipeline_cache.len());
        }
        {
            s.field(
                stringify!(descriptor_update_template),
                &self.descriptor_update_template.len(),
            );
        }
        {
            s.field(
                stringify!(sampler_ycbcr_conversion),
                &self.sampler_ycbcr_conversion.len(),
            );
        }
        #[cfg(feature = "VULKAN_1_3")]
        {
            s.field(stringify!(private_data_slot), &self.private_data_slot.len());
        }
        #[cfg(feature = "VK_KHR_surface")]
        {
            s.field(stringify!(surface_khr), &self.surface_khr.len());
        }
        #[cfg(feature = "VK_KHR_swapchain")]
        {
            s.field(stringify!(swapchain_khr), &self.swapchain_khr.len());
        }
        #[cfg(feature = "VK_KHR_display")]
        {
            s.field(stringify!(display_khr), &self.display_khr.len());
        }
        #[cfg(feature = "VK_KHR_display")]
        {
            s.field(stringify!(display_mode_khr), &self.display_mode_khr.len());
        }
        #[cfg(feature = "VK_EXT_debug_report")]
        {
            s.field(
                stringify!(debug_report_callback_ext),
                &self.debug_report_callback_ext.len(),
            );
        }
        #[cfg(feature = "VK_KHR_video_queue")]
        {
            s.field(stringify!(video_session_khr), &self.video_session_khr.len());
        }
        #[cfg(feature = "VK_KHR_video_queue")]
        {
            s.field(
                stringify!(video_session_parameters_khr),
                &self.video_session_parameters_khr.len(),
            );
        }
        #[cfg(feature = "VK_NVX_binary_import")]
        {
            s.field(stringify!(cu_module_nvx), &self.cu_module_nvx.len());
        }
        #[cfg(feature = "VK_NVX_binary_import")]
        {
            s.field(stringify!(cu_function_nvx), &self.cu_function_nvx.len());
        }
        #[cfg(feature = "VK_EXT_debug_utils")]
        {
            s.field(
                stringify!(debug_utils_messenger_ext),
                &self.debug_utils_messenger_ext.len(),
            );
        }
        #[cfg(feature = "VK_KHR_acceleration_structure")]
        {
            s.field(
                stringify!(acceleration_structure_khr),
                &self.acceleration_structure_khr.len(),
            );
        }
        #[cfg(feature = "VK_EXT_validation_cache")]
        {
            s.field(stringify!(validation_cache_ext), &self.validation_cache_ext.len());
        }
        #[cfg(feature = "VK_NV_ray_tracing")]
        {
            s.field(
                stringify!(acceleration_structure_nv),
                &self.acceleration_structure_nv.len(),
            );
        }
        #[cfg(feature = "VK_INTEL_performance_query")]
        {
            s.field(
                stringify!(performance_configuration_intel),
                &self.performance_configuration_intel.len(),
            );
        }
        #[cfg(feature = "VK_KHR_deferred_host_operations")]
        {
            s.field(stringify!(deferred_operation_khr), &self.deferred_operation_khr.len());
        }
        #[cfg(feature = "VK_NV_device_generated_commands")]
        {
            s.field(
                stringify!(indirect_commands_layout_nv),
                &self.indirect_commands_layout_nv.len(),
            );
        }
        #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
        {
            s.field(
                stringify!(buffer_collection_fuchsia),
                &self.buffer_collection_fuchsia.len(),
            );
        }
        s.finish_non_exhaustive()
    }
}
impl Api {
    pub fn new(entry: Entry) -> Self {
        Self {
            entry,
            objects: DashMap::new(),
            instance: DashMap::new(),
            physical_device: DashMap::new(),
            device: DashMap::new(),
            queue: DashMap::new(),
            command_buffer: DashMap::new(),
            device_memory: DashMap::new(),
            command_pool: DashMap::new(),
            buffer: DashMap::new(),
            buffer_view: DashMap::new(),
            image: DashMap::new(),
            image_view: DashMap::new(),
            shader_module: DashMap::new(),
            pipeline: DashMap::new(),
            pipeline_layout: DashMap::new(),
            sampler: DashMap::new(),
            descriptor_set: DashMap::new(),
            descriptor_set_layout: DashMap::new(),
            descriptor_pool: DashMap::new(),
            fence: DashMap::new(),
            semaphore: DashMap::new(),
            event: DashMap::new(),
            query_pool: DashMap::new(),
            framebuffer: DashMap::new(),
            render_pass: DashMap::new(),
            pipeline_cache: DashMap::new(),
            descriptor_update_template: DashMap::new(),
            sampler_ycbcr_conversion: DashMap::new(),
            #[cfg(feature = "VULKAN_1_3")]
            private_data_slot: DashMap::new(),
            #[cfg(feature = "VK_KHR_surface")]
            surface_khr: DashMap::new(),
            #[cfg(feature = "VK_KHR_swapchain")]
            swapchain_khr: DashMap::new(),
            #[cfg(feature = "VK_KHR_display")]
            display_khr: DashMap::new(),
            #[cfg(feature = "VK_KHR_display")]
            display_mode_khr: DashMap::new(),
            #[cfg(feature = "VK_EXT_debug_report")]
            debug_report_callback_ext: DashMap::new(),
            #[cfg(feature = "VK_KHR_video_queue")]
            video_session_khr: DashMap::new(),
            #[cfg(feature = "VK_KHR_video_queue")]
            video_session_parameters_khr: DashMap::new(),
            #[cfg(feature = "VK_NVX_binary_import")]
            cu_module_nvx: DashMap::new(),
            #[cfg(feature = "VK_NVX_binary_import")]
            cu_function_nvx: DashMap::new(),
            #[cfg(feature = "VK_EXT_debug_utils")]
            debug_utils_messenger_ext: DashMap::new(),
            #[cfg(feature = "VK_KHR_acceleration_structure")]
            acceleration_structure_khr: DashMap::new(),
            #[cfg(feature = "VK_EXT_validation_cache")]
            validation_cache_ext: DashMap::new(),
            #[cfg(feature = "VK_NV_ray_tracing")]
            acceleration_structure_nv: DashMap::new(),
            #[cfg(feature = "VK_INTEL_performance_query")]
            performance_configuration_intel: DashMap::new(),
            #[cfg(feature = "VK_KHR_deferred_host_operations")]
            deferred_operation_khr: DashMap::new(),
            #[cfg(feature = "VK_NV_device_generated_commands")]
            indirect_commands_layout_nv: DashMap::new(),
            #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
            buffer_collection_fuchsia: DashMap::new(),
        }
    }
    pub fn objects(&self) -> &DashMap<ObjectId, Object> {
        &self.objects
    }
    pub fn reference_counter_of(&self, object_id: ObjectId) -> Option<usize> {
        self.objects.get(&object_id).map(|object| match object.value() {
            Object::Instance => unsafe { self.instance().get(&object_id).unwrap_unchecked().ref_count() },
            Object::PhysicalDevice => unsafe { self.physical_device().get(&object_id).unwrap_unchecked().ref_count() },
            Object::Device => unsafe { self.device().get(&object_id).unwrap_unchecked().ref_count() },
            Object::Queue => unsafe { self.queue().get(&object_id).unwrap_unchecked().ref_count() },
            Object::CommandBuffer => unsafe { self.command_buffer().get(&object_id).unwrap_unchecked().ref_count() },
            Object::DeviceMemory => unsafe { self.device_memory().get(&object_id).unwrap_unchecked().ref_count() },
            Object::CommandPool => unsafe { self.command_pool().get(&object_id).unwrap_unchecked().ref_count() },
            Object::Buffer => unsafe { self.buffer().get(&object_id).unwrap_unchecked().ref_count() },
            Object::BufferView => unsafe { self.buffer_view().get(&object_id).unwrap_unchecked().ref_count() },
            Object::Image => unsafe { self.image().get(&object_id).unwrap_unchecked().ref_count() },
            Object::ImageView => unsafe { self.image_view().get(&object_id).unwrap_unchecked().ref_count() },
            Object::ShaderModule => unsafe { self.shader_module().get(&object_id).unwrap_unchecked().ref_count() },
            Object::Pipeline => unsafe { self.pipeline().get(&object_id).unwrap_unchecked().ref_count() },
            Object::PipelineLayout => unsafe { self.pipeline_layout().get(&object_id).unwrap_unchecked().ref_count() },
            Object::Sampler => unsafe { self.sampler().get(&object_id).unwrap_unchecked().ref_count() },
            Object::DescriptorSet => unsafe { self.descriptor_set().get(&object_id).unwrap_unchecked().ref_count() },
            Object::DescriptorSetLayout => unsafe {
                self.descriptor_set_layout()
                    .get(&object_id)
                    .unwrap_unchecked()
                    .ref_count()
            },
            Object::DescriptorPool => unsafe { self.descriptor_pool().get(&object_id).unwrap_unchecked().ref_count() },
            Object::Fence => unsafe { self.fence().get(&object_id).unwrap_unchecked().ref_count() },
            Object::Semaphore => unsafe { self.semaphore().get(&object_id).unwrap_unchecked().ref_count() },
            Object::Event => unsafe { self.event().get(&object_id).unwrap_unchecked().ref_count() },
            Object::QueryPool => unsafe { self.query_pool().get(&object_id).unwrap_unchecked().ref_count() },
            Object::Framebuffer => unsafe { self.framebuffer().get(&object_id).unwrap_unchecked().ref_count() },
            Object::RenderPass => unsafe { self.render_pass().get(&object_id).unwrap_unchecked().ref_count() },
            Object::PipelineCache => unsafe { self.pipeline_cache().get(&object_id).unwrap_unchecked().ref_count() },
            Object::DescriptorUpdateTemplate => unsafe {
                self.descriptor_update_template()
                    .get(&object_id)
                    .unwrap_unchecked()
                    .ref_count()
            },
            Object::SamplerYcbcrConversion => unsafe {
                self.sampler_ycbcr_conversion()
                    .get(&object_id)
                    .unwrap_unchecked()
                    .ref_count()
            },
            #[cfg(feature = "VULKAN_1_3")]
            Object::PrivateDataSlot => unsafe {
                self.private_data_slot().get(&object_id).unwrap_unchecked().ref_count()
            },
            #[cfg(feature = "VK_KHR_surface")]
            Object::SurfaceKHR => unsafe { self.surface_khr().get(&object_id).unwrap_unchecked().ref_count() },
            #[cfg(feature = "VK_KHR_swapchain")]
            Object::SwapchainKHR => unsafe { self.swapchain_khr().get(&object_id).unwrap_unchecked().ref_count() },
            #[cfg(feature = "VK_KHR_display")]
            Object::DisplayKHR => unsafe { self.display_khr().get(&object_id).unwrap_unchecked().ref_count() },
            #[cfg(feature = "VK_KHR_display")]
            Object::DisplayModeKHR => unsafe { self.display_mode_khr().get(&object_id).unwrap_unchecked().ref_count() },
            #[cfg(feature = "VK_EXT_debug_report")]
            Object::DebugReportCallbackEXT => unsafe {
                self.debug_report_callback_ext()
                    .get(&object_id)
                    .unwrap_unchecked()
                    .ref_count()
            },
            #[cfg(feature = "VK_KHR_video_queue")]
            Object::VideoSessionKHR => unsafe {
                self.video_session_khr().get(&object_id).unwrap_unchecked().ref_count()
            },
            #[cfg(feature = "VK_KHR_video_queue")]
            Object::VideoSessionParametersKHR => unsafe {
                self.video_session_parameters_khr()
                    .get(&object_id)
                    .unwrap_unchecked()
                    .ref_count()
            },
            #[cfg(feature = "VK_NVX_binary_import")]
            Object::CuModuleNVX => unsafe { self.cu_module_nvx().get(&object_id).unwrap_unchecked().ref_count() },
            #[cfg(feature = "VK_NVX_binary_import")]
            Object::CuFunctionNVX => unsafe { self.cu_function_nvx().get(&object_id).unwrap_unchecked().ref_count() },
            #[cfg(feature = "VK_EXT_debug_utils")]
            Object::DebugUtilsMessengerEXT => unsafe {
                self.debug_utils_messenger_ext()
                    .get(&object_id)
                    .unwrap_unchecked()
                    .ref_count()
            },
            #[cfg(feature = "VK_KHR_acceleration_structure")]
            Object::AccelerationStructureKHR => unsafe {
                self.acceleration_structure_khr()
                    .get(&object_id)
                    .unwrap_unchecked()
                    .ref_count()
            },
            #[cfg(feature = "VK_EXT_validation_cache")]
            Object::ValidationCacheEXT => unsafe {
                self.validation_cache_ext()
                    .get(&object_id)
                    .unwrap_unchecked()
                    .ref_count()
            },
            #[cfg(feature = "VK_NV_ray_tracing")]
            Object::AccelerationStructureNV => unsafe {
                self.acceleration_structure_nv()
                    .get(&object_id)
                    .unwrap_unchecked()
                    .ref_count()
            },
            #[cfg(feature = "VK_INTEL_performance_query")]
            Object::PerformanceConfigurationINTEL => unsafe {
                self.performance_configuration_intel()
                    .get(&object_id)
                    .unwrap_unchecked()
                    .ref_count()
            },
            #[cfg(feature = "VK_KHR_deferred_host_operations")]
            Object::DeferredOperationKHR => unsafe {
                self.deferred_operation_khr()
                    .get(&object_id)
                    .unwrap_unchecked()
                    .ref_count()
            },
            #[cfg(feature = "VK_NV_device_generated_commands")]
            Object::IndirectCommandsLayoutNV => unsafe {
                self.indirect_commands_layout_nv()
                    .get(&object_id)
                    .unwrap_unchecked()
                    .ref_count()
            },
            #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
            Object::BufferCollectionFUCHSIA => unsafe {
                self.buffer_collection_fuchsia()
                    .get(&object_id)
                    .unwrap_unchecked()
                    .ref_count()
            },
        })
    }
    pub fn parent_of(&self, object_id: ObjectId) -> Option<ObjectId> {
        self.objects.get(&object_id).and_then(|object| match object.value() {
            Object::Instance => unsafe { self.instance().get(&object_id).unwrap_unchecked().parent() },
            Object::PhysicalDevice => unsafe { self.physical_device().get(&object_id).unwrap_unchecked().parent() },
            Object::Device => unsafe { self.device().get(&object_id).unwrap_unchecked().parent() },
            Object::Queue => unsafe { self.queue().get(&object_id).unwrap_unchecked().parent() },
            Object::CommandBuffer => unsafe { self.command_buffer().get(&object_id).unwrap_unchecked().parent() },
            Object::DeviceMemory => unsafe { self.device_memory().get(&object_id).unwrap_unchecked().parent() },
            Object::CommandPool => unsafe { self.command_pool().get(&object_id).unwrap_unchecked().parent() },
            Object::Buffer => unsafe { self.buffer().get(&object_id).unwrap_unchecked().parent() },
            Object::BufferView => unsafe { self.buffer_view().get(&object_id).unwrap_unchecked().parent() },
            Object::Image => unsafe { self.image().get(&object_id).unwrap_unchecked().parent() },
            Object::ImageView => unsafe { self.image_view().get(&object_id).unwrap_unchecked().parent() },
            Object::ShaderModule => unsafe { self.shader_module().get(&object_id).unwrap_unchecked().parent() },
            Object::Pipeline => unsafe { self.pipeline().get(&object_id).unwrap_unchecked().parent() },
            Object::PipelineLayout => unsafe { self.pipeline_layout().get(&object_id).unwrap_unchecked().parent() },
            Object::Sampler => unsafe { self.sampler().get(&object_id).unwrap_unchecked().parent() },
            Object::DescriptorSet => unsafe { self.descriptor_set().get(&object_id).unwrap_unchecked().parent() },
            Object::DescriptorSetLayout => unsafe {
                self.descriptor_set_layout().get(&object_id).unwrap_unchecked().parent()
            },
            Object::DescriptorPool => unsafe { self.descriptor_pool().get(&object_id).unwrap_unchecked().parent() },
            Object::Fence => unsafe { self.fence().get(&object_id).unwrap_unchecked().parent() },
            Object::Semaphore => unsafe { self.semaphore().get(&object_id).unwrap_unchecked().parent() },
            Object::Event => unsafe { self.event().get(&object_id).unwrap_unchecked().parent() },
            Object::QueryPool => unsafe { self.query_pool().get(&object_id).unwrap_unchecked().parent() },
            Object::Framebuffer => unsafe { self.framebuffer().get(&object_id).unwrap_unchecked().parent() },
            Object::RenderPass => unsafe { self.render_pass().get(&object_id).unwrap_unchecked().parent() },
            Object::PipelineCache => unsafe { self.pipeline_cache().get(&object_id).unwrap_unchecked().parent() },
            Object::DescriptorUpdateTemplate => unsafe {
                self.descriptor_update_template()
                    .get(&object_id)
                    .unwrap_unchecked()
                    .parent()
            },
            Object::SamplerYcbcrConversion => unsafe {
                self.sampler_ycbcr_conversion()
                    .get(&object_id)
                    .unwrap_unchecked()
                    .parent()
            },
            #[cfg(feature = "VULKAN_1_3")]
            Object::PrivateDataSlot => unsafe { self.private_data_slot().get(&object_id).unwrap_unchecked().parent() },
            #[cfg(feature = "VK_KHR_surface")]
            Object::SurfaceKHR => unsafe { self.surface_khr().get(&object_id).unwrap_unchecked().parent() },
            #[cfg(feature = "VK_KHR_swapchain")]
            Object::SwapchainKHR => unsafe { self.swapchain_khr().get(&object_id).unwrap_unchecked().parent() },
            #[cfg(feature = "VK_KHR_display")]
            Object::DisplayKHR => unsafe { self.display_khr().get(&object_id).unwrap_unchecked().parent() },
            #[cfg(feature = "VK_KHR_display")]
            Object::DisplayModeKHR => unsafe { self.display_mode_khr().get(&object_id).unwrap_unchecked().parent() },
            #[cfg(feature = "VK_EXT_debug_report")]
            Object::DebugReportCallbackEXT => unsafe {
                self.debug_report_callback_ext()
                    .get(&object_id)
                    .unwrap_unchecked()
                    .parent()
            },
            #[cfg(feature = "VK_KHR_video_queue")]
            Object::VideoSessionKHR => unsafe { self.video_session_khr().get(&object_id).unwrap_unchecked().parent() },
            #[cfg(feature = "VK_KHR_video_queue")]
            Object::VideoSessionParametersKHR => unsafe {
                self.video_session_parameters_khr()
                    .get(&object_id)
                    .unwrap_unchecked()
                    .parent()
            },
            #[cfg(feature = "VK_NVX_binary_import")]
            Object::CuModuleNVX => unsafe { self.cu_module_nvx().get(&object_id).unwrap_unchecked().parent() },
            #[cfg(feature = "VK_NVX_binary_import")]
            Object::CuFunctionNVX => unsafe { self.cu_function_nvx().get(&object_id).unwrap_unchecked().parent() },
            #[cfg(feature = "VK_EXT_debug_utils")]
            Object::DebugUtilsMessengerEXT => unsafe {
                self.debug_utils_messenger_ext()
                    .get(&object_id)
                    .unwrap_unchecked()
                    .parent()
            },
            #[cfg(feature = "VK_KHR_acceleration_structure")]
            Object::AccelerationStructureKHR => unsafe {
                self.acceleration_structure_khr()
                    .get(&object_id)
                    .unwrap_unchecked()
                    .parent()
            },
            #[cfg(feature = "VK_EXT_validation_cache")]
            Object::ValidationCacheEXT => unsafe {
                self.validation_cache_ext().get(&object_id).unwrap_unchecked().parent()
            },
            #[cfg(feature = "VK_NV_ray_tracing")]
            Object::AccelerationStructureNV => unsafe {
                self.acceleration_structure_nv()
                    .get(&object_id)
                    .unwrap_unchecked()
                    .parent()
            },
            #[cfg(feature = "VK_INTEL_performance_query")]
            Object::PerformanceConfigurationINTEL => unsafe {
                self.performance_configuration_intel()
                    .get(&object_id)
                    .unwrap_unchecked()
                    .parent()
            },
            #[cfg(feature = "VK_KHR_deferred_host_operations")]
            Object::DeferredOperationKHR => unsafe {
                self.deferred_operation_khr()
                    .get(&object_id)
                    .unwrap_unchecked()
                    .parent()
            },
            #[cfg(feature = "VK_NV_device_generated_commands")]
            Object::IndirectCommandsLayoutNV => unsafe {
                self.indirect_commands_layout_nv()
                    .get(&object_id)
                    .unwrap_unchecked()
                    .parent()
            },
            #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
            Object::BufferCollectionFUCHSIA => unsafe {
                self.buffer_collection_fuchsia()
                    .get(&object_id)
                    .unwrap_unchecked()
                    .parent()
            },
        })
    }
    pub fn instance(&self) -> &DashMap<ObjectId, Container<Instance>> {
        &self.instance
    }
    pub fn physical_device(&self) -> &DashMap<ObjectId, Container<PhysicalDevice>> {
        &self.physical_device
    }
    pub fn device(&self) -> &DashMap<ObjectId, Container<Device>> {
        &self.device
    }
    pub fn queue(&self) -> &DashMap<ObjectId, Container<Queue>> {
        &self.queue
    }
    pub fn command_buffer(&self) -> &DashMap<ObjectId, Container<CommandBuffer>> {
        &self.command_buffer
    }
    pub fn device_memory(&self) -> &DashMap<ObjectId, Container<DeviceMemory>> {
        &self.device_memory
    }
    pub fn command_pool(&self) -> &DashMap<ObjectId, Container<CommandPool>> {
        &self.command_pool
    }
    pub fn buffer(&self) -> &DashMap<ObjectId, Container<Buffer>> {
        &self.buffer
    }
    pub fn buffer_view(&self) -> &DashMap<ObjectId, Container<BufferView>> {
        &self.buffer_view
    }
    pub fn image(&self) -> &DashMap<ObjectId, Container<Image>> {
        &self.image
    }
    pub fn image_view(&self) -> &DashMap<ObjectId, Container<ImageView>> {
        &self.image_view
    }
    pub fn shader_module(&self) -> &DashMap<ObjectId, Container<ShaderModule>> {
        &self.shader_module
    }
    pub fn pipeline(&self) -> &DashMap<ObjectId, Container<Pipeline>> {
        &self.pipeline
    }
    pub fn pipeline_layout(&self) -> &DashMap<ObjectId, Container<PipelineLayout>> {
        &self.pipeline_layout
    }
    pub fn sampler(&self) -> &DashMap<ObjectId, Container<Sampler>> {
        &self.sampler
    }
    pub fn descriptor_set(&self) -> &DashMap<ObjectId, Container<DescriptorSet>> {
        &self.descriptor_set
    }
    pub fn descriptor_set_layout(&self) -> &DashMap<ObjectId, Container<DescriptorSetLayout>> {
        &self.descriptor_set_layout
    }
    pub fn descriptor_pool(&self) -> &DashMap<ObjectId, Container<DescriptorPool>> {
        &self.descriptor_pool
    }
    pub fn fence(&self) -> &DashMap<ObjectId, Container<Fence>> {
        &self.fence
    }
    pub fn semaphore(&self) -> &DashMap<ObjectId, Container<Semaphore>> {
        &self.semaphore
    }
    pub fn event(&self) -> &DashMap<ObjectId, Container<Event>> {
        &self.event
    }
    pub fn query_pool(&self) -> &DashMap<ObjectId, Container<QueryPool>> {
        &self.query_pool
    }
    pub fn framebuffer(&self) -> &DashMap<ObjectId, Container<Framebuffer>> {
        &self.framebuffer
    }
    pub fn render_pass(&self) -> &DashMap<ObjectId, Container<RenderPass>> {
        &self.render_pass
    }
    pub fn pipeline_cache(&self) -> &DashMap<ObjectId, Container<PipelineCache>> {
        &self.pipeline_cache
    }
    pub fn descriptor_update_template(&self) -> &DashMap<ObjectId, Container<DescriptorUpdateTemplate>> {
        &self.descriptor_update_template
    }
    pub fn sampler_ycbcr_conversion(&self) -> &DashMap<ObjectId, Container<SamplerYcbcrConversion>> {
        &self.sampler_ycbcr_conversion
    }
    #[cfg(feature = "VULKAN_1_3")]
    pub fn private_data_slot(&self) -> &DashMap<ObjectId, Container<PrivateDataSlot>> {
        &self.private_data_slot
    }
    #[cfg(feature = "VK_KHR_surface")]
    pub fn surface_khr(&self) -> &DashMap<ObjectId, Container<SurfaceKHR>> {
        &self.surface_khr
    }
    #[cfg(feature = "VK_KHR_swapchain")]
    pub fn swapchain_khr(&self) -> &DashMap<ObjectId, Container<SwapchainKHR>> {
        &self.swapchain_khr
    }
    #[cfg(feature = "VK_KHR_display")]
    pub fn display_khr(&self) -> &DashMap<ObjectId, Container<DisplayKHR>> {
        &self.display_khr
    }
    #[cfg(feature = "VK_KHR_display")]
    pub fn display_mode_khr(&self) -> &DashMap<ObjectId, Container<DisplayModeKHR>> {
        &self.display_mode_khr
    }
    #[cfg(feature = "VK_EXT_debug_report")]
    pub fn debug_report_callback_ext(&self) -> &DashMap<ObjectId, Container<DebugReportCallbackEXT>> {
        &self.debug_report_callback_ext
    }
    #[cfg(feature = "VK_KHR_video_queue")]
    pub fn video_session_khr(&self) -> &DashMap<ObjectId, Container<VideoSessionKHR>> {
        &self.video_session_khr
    }
    #[cfg(feature = "VK_KHR_video_queue")]
    pub fn video_session_parameters_khr(&self) -> &DashMap<ObjectId, Container<VideoSessionParametersKHR>> {
        &self.video_session_parameters_khr
    }
    #[cfg(feature = "VK_NVX_binary_import")]
    pub fn cu_module_nvx(&self) -> &DashMap<ObjectId, Container<CuModuleNVX>> {
        &self.cu_module_nvx
    }
    #[cfg(feature = "VK_NVX_binary_import")]
    pub fn cu_function_nvx(&self) -> &DashMap<ObjectId, Container<CuFunctionNVX>> {
        &self.cu_function_nvx
    }
    #[cfg(feature = "VK_EXT_debug_utils")]
    pub fn debug_utils_messenger_ext(&self) -> &DashMap<ObjectId, Container<DebugUtilsMessengerEXT>> {
        &self.debug_utils_messenger_ext
    }
    #[cfg(feature = "VK_KHR_acceleration_structure")]
    pub fn acceleration_structure_khr(&self) -> &DashMap<ObjectId, Container<AccelerationStructureKHR>> {
        &self.acceleration_structure_khr
    }
    #[cfg(feature = "VK_EXT_validation_cache")]
    pub fn validation_cache_ext(&self) -> &DashMap<ObjectId, Container<ValidationCacheEXT>> {
        &self.validation_cache_ext
    }
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub fn acceleration_structure_nv(&self) -> &DashMap<ObjectId, Container<AccelerationStructureNV>> {
        &self.acceleration_structure_nv
    }
    #[cfg(feature = "VK_INTEL_performance_query")]
    pub fn performance_configuration_intel(&self) -> &DashMap<ObjectId, Container<PerformanceConfigurationINTEL>> {
        &self.performance_configuration_intel
    }
    #[cfg(feature = "VK_KHR_deferred_host_operations")]
    pub fn deferred_operation_khr(&self) -> &DashMap<ObjectId, Container<DeferredOperationKHR>> {
        &self.deferred_operation_khr
    }
    #[cfg(feature = "VK_NV_device_generated_commands")]
    pub fn indirect_commands_layout_nv(&self) -> &DashMap<ObjectId, Container<IndirectCommandsLayoutNV>> {
        &self.indirect_commands_layout_nv
    }
    #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
    pub fn buffer_collection_fuchsia(&self) -> &DashMap<ObjectId, Container<BufferCollectionFUCHSIA>> {
        &self.buffer_collection_fuchsia
    }
}
