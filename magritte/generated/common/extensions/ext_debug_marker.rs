use crate::cstr;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
#[doc(alias = "VK_EXT_DEBUG_MARKER_SPEC_VERSION")]
pub const EXT_DEBUG_MARKER_SPEC_VERSION: u32 = 4;
#[doc(alias = "VK_EXT_DEBUG_MARKER_EXTENSION_NAME")]
pub const EXT_DEBUG_MARKER_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_debug_marker");
#[doc(alias = "VkDebugReportObjectTypeEXT")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum DebugReportObjectTypeEXT {
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT")]
    Unknown = 0,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_INSTANCE_EXT")]
    Instance = 1,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE_EXT")]
    PhysicalDevice = 2,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_EXT")]
    Device = 3,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_QUEUE_EXT")]
    Queue = 4,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE_EXT")]
    Semaphore = 5,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER_EXT")]
    CommandBuffer = 6,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_FENCE_EXT")]
    Fence = 7,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY_EXT")]
    DeviceMemory = 8,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_EXT")]
    Buffer = 9,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_EXT")]
    Image = 10,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_EVENT_EXT")]
    Event = 11,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL_EXT")]
    QueryPool = 12,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW_EXT")]
    BufferView = 13,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW_EXT")]
    ImageView = 14,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE_EXT")]
    ShaderModule = 15,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE_EXT")]
    PipelineCache = 16,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT_EXT")]
    PipelineLayout = 17,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS_EXT")]
    RenderPass = 18,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_EXT")]
    Pipeline = 19,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT_EXT")]
    DescriptorSetLayout = 20,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_EXT")]
    Sampler = 21,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL_EXT")]
    DescriptorPool = 22,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_EXT")]
    DescriptorSet = 23,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER_EXT")]
    Framebuffer = 24,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL_EXT")]
    CommandPool = 25,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR_EXT")]
    SurfaceKhr = 26,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR_EXT")]
    SwapchainKhr = 27,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT_EXT")]
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_EXT")]
    DebugReportCallback = 28,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_KHR_EXT")]
    DisplayKhr = 29,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_MODE_KHR_EXT")]
    DisplayModeKhr = 30,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT_EXT")]
    ValidationCache = 33,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_EXT")]
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_KHR_EXT")]
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    SamplerYcbcrConversion = 1000156000,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_EXT")]
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR_EXT")]
    DescriptorUpdateTemplate = 1000085000,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_CU_MODULE_NVX_EXT")]
    #[cfg(feature = "VK_NVX_binary_import")]
    CuModuleNvx = 1000029000,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_CU_FUNCTION_NVX_EXT")]
    #[cfg(feature = "VK_NVX_binary_import")]
    CuFunctionNvx = 1000029001,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_ACCELERATION_STRUCTURE_KHR_EXT")]
    #[cfg(feature = "VK_KHR_acceleration_structure")]
    AccelerationStructureKhr = 1000150000,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_ACCELERATION_STRUCTURE_NV_EXT")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    AccelerationStructureNv = 1000165000,
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_COLLECTION_FUCHSIA_EXT")]
    #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
    BufferCollectionFuchsia = 1000366000,
}
impl Default for DebugReportObjectTypeEXT {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl DebugReportObjectTypeEXT {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        unsafe { Self::from_bits_unchecked(0) }
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        *self as i32
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::Unknown.bits() => Some(Self::Unknown),
            x if x == Self::Instance.bits() => Some(Self::Instance),
            x if x == Self::PhysicalDevice.bits() => Some(Self::PhysicalDevice),
            x if x == Self::Device.bits() => Some(Self::Device),
            x if x == Self::Queue.bits() => Some(Self::Queue),
            x if x == Self::Semaphore.bits() => Some(Self::Semaphore),
            x if x == Self::CommandBuffer.bits() => Some(Self::CommandBuffer),
            x if x == Self::Fence.bits() => Some(Self::Fence),
            x if x == Self::DeviceMemory.bits() => Some(Self::DeviceMemory),
            x if x == Self::Buffer.bits() => Some(Self::Buffer),
            x if x == Self::Image.bits() => Some(Self::Image),
            x if x == Self::Event.bits() => Some(Self::Event),
            x if x == Self::QueryPool.bits() => Some(Self::QueryPool),
            x if x == Self::BufferView.bits() => Some(Self::BufferView),
            x if x == Self::ImageView.bits() => Some(Self::ImageView),
            x if x == Self::ShaderModule.bits() => Some(Self::ShaderModule),
            x if x == Self::PipelineCache.bits() => Some(Self::PipelineCache),
            x if x == Self::PipelineLayout.bits() => Some(Self::PipelineLayout),
            x if x == Self::RenderPass.bits() => Some(Self::RenderPass),
            x if x == Self::Pipeline.bits() => Some(Self::Pipeline),
            x if x == Self::DescriptorSetLayout.bits() => Some(Self::DescriptorSetLayout),
            x if x == Self::Sampler.bits() => Some(Self::Sampler),
            x if x == Self::DescriptorPool.bits() => Some(Self::DescriptorPool),
            x if x == Self::DescriptorSet.bits() => Some(Self::DescriptorSet),
            x if x == Self::Framebuffer.bits() => Some(Self::Framebuffer),
            x if x == Self::CommandPool.bits() => Some(Self::CommandPool),
            x if x == Self::SurfaceKhr.bits() => Some(Self::SurfaceKhr),
            x if x == Self::SwapchainKhr.bits() => Some(Self::SwapchainKhr),
            x if x == Self::DebugReportCallback.bits() => Some(Self::DebugReportCallback),
            x if x == Self::DisplayKhr.bits() => Some(Self::DisplayKhr),
            x if x == Self::DisplayModeKhr.bits() => Some(Self::DisplayModeKhr),
            x if x == Self::ValidationCache.bits() => Some(Self::ValidationCache),
            #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
            x if x == Self::SamplerYcbcrConversion.bits() => Some(Self::SamplerYcbcrConversion),
            x if x == Self::DescriptorUpdateTemplate.bits() => Some(Self::DescriptorUpdateTemplate),
            #[cfg(feature = "VK_NVX_binary_import")]
            x if x == Self::CuModuleNvx.bits() => Some(Self::CuModuleNvx),
            #[cfg(feature = "VK_NVX_binary_import")]
            x if x == Self::CuFunctionNvx.bits() => Some(Self::CuFunctionNvx),
            #[cfg(feature = "VK_KHR_acceleration_structure")]
            x if x == Self::AccelerationStructureKhr.bits() => Some(Self::AccelerationStructureKhr),
            #[cfg(feature = "VK_NV_ray_tracing")]
            x if x == Self::AccelerationStructureNv.bits() => Some(Self::AccelerationStructureNv),
            #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
            x if x == Self::BufferCollectionFuchsia.bits() => Some(Self::BufferCollectionFuchsia),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        std::mem::transmute(bits)
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DebugReportObjectTypeEXT {
    type LowLevel = Self;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DebugReportObjectTypeEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
