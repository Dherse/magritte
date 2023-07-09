pub use crate::common::vulkan1_3::{
    AccessFlagBits2, AccessFlags2, FormatFeatureFlagBits2, FormatFeatureFlags2, PipelineCreationFeedback,
    PipelineCreationFeedbackFlagBits, PipelineCreationFeedbackFlags, PipelineStageFlagBits2, PipelineStageFlags2,
    PrivateDataSlotCreateFlags, RenderingFlagBits, RenderingFlags, SubmitFlagBits, SubmitFlags, ToolPurposeFlagBits,
    ToolPurposeFlags,
};
#[cfg(feature = "VK_EXT_sample_locations")]
use crate::extensions::ext_sample_locations::SampleLocationsInfoEXT;
#[cfg(feature = "VK_KHR_dynamic_rendering")]
use crate::extensions::khr_dynamic_rendering::MultiviewPerViewAttributesInfoNVX;
#[cfg(feature = "VK_KHR_dynamic_rendering")]
use crate::extensions::khr_dynamic_rendering::RenderingFragmentDensityMapAttachmentInfoEXT;
#[cfg(feature = "VK_KHR_dynamic_rendering")]
use crate::extensions::khr_dynamic_rendering::RenderingFragmentShadingRateAttachmentInfoKHR;
#[cfg(feature = "VK_KHR_performance_query")]
use crate::extensions::khr_performance_query::PerformanceQuerySubmitInfoKHR;
#[cfg(feature = "VK_KHR_win32_keyed_mutex")]
use crate::extensions::khr_win32_keyed_mutex::Win32KeyedMutexAcquireReleaseInfoKHR;
#[cfg(feature = "VK_NV_win32_keyed_mutex")]
use crate::extensions::nv_win32_keyed_mutex::Win32KeyedMutexAcquireReleaseInfoNV;
#[cfg(feature = "VULKAN_1_2")]
use crate::vulkan1_2::ResolveModeFlagBits;
use crate::{
    context::{Container, Context, Error, ObjectId},
    vulkan1_0::{
        AttachmentLoadOp, AttachmentStoreOp, Buffer, BufferCreateInfo, ClearValue, CommandBuffer, DependencyFlags,
        DeviceSize, Extent3D, Filter, Format, Image, ImageAspectFlagBits, ImageCreateInfo, ImageLayout,
        ImageSubresourceLayers, ImageSubresourceRange, ImageView, Offset3D, Rect2D, SampleCountFlagBits, Semaphore,
        ShaderStageFlags, StructureType, MAX_DESCRIPTION_SIZE, MAX_EXTENSION_NAME_SIZE,
    },
    vulkan1_1::DeviceGroupRenderPassBeginInfo,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
#[doc(alias = "VkDevicePrivateDataCreateInfo")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DevicePrivateDataCreateInfo {
    #[doc(alias = "privateDataSlotRequestCount")]
    pub private_data_slot_request_count: u32,
}
impl DevicePrivateDataCreateInfo {
    ///Get a reference to the `private_data_slot_request_count` field.
    pub fn private_data_slot_request_count(&self) -> u32 {
        self.private_data_slot_request_count
    }
    ///Get a mutable reference to the `private_data_slot_request_count` field.
    pub fn private_data_slot_request_count_mut(&mut self) -> &mut u32 {
        &mut self.private_data_slot_request_count
    }
    ///Sets the `private_data_slot_request_count` field.
    pub fn set_private_data_slot_request_count(&mut self, private_data_slot_request_count: u32) -> &mut Self {
        self.private_data_slot_request_count = private_data_slot_request_count;
        self
    }
    ///Sets the `private_data_slot_request_count` field in a builder way.
    pub fn with_private_data_slot_request_count(mut self, private_data_slot_request_count: u32) -> Self {
        self.private_data_slot_request_count = private_data_slot_request_count;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DevicePrivateDataCreateInfo {
    type LowLevel = crate::native::vulkan1_3::DevicePrivateDataCreateInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::DevicePrivateDataCreateInfo {
            s_type: StructureType::DevicePrivateDataCreateInfo,
            p_next: std::ptr::null(),
            private_data_slot_request_count: self.private_data_slot_request_count.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DevicePrivateDataCreateInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            private_data_slot_request_count: crate::conv::FromLowLevel::from_low_level(
                context,
                value.private_data_slot_request_count,
            ),
        }
    }
}
#[doc(alias = "VkPrivateDataSlotCreateInfo")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PrivateDataSlotCreateInfo {
    pub flags: PrivateDataSlotCreateFlags,
}
impl PrivateDataSlotCreateInfo {
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> PrivateDataSlotCreateFlags {
        self.flags
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut PrivateDataSlotCreateFlags {
        &mut self.flags
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: PrivateDataSlotCreateFlags) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: PrivateDataSlotCreateFlags) -> Self {
        self.flags = flags;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PrivateDataSlotCreateInfo {
    type LowLevel = crate::native::vulkan1_3::PrivateDataSlotCreateInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::PrivateDataSlotCreateInfo {
            s_type: StructureType::PrivateDataSlotCreateInfo,
            p_next: std::ptr::null(),
            flags: self.flags.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PrivateDataSlotCreateInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
        }
    }
}
#[doc(alias = "VkPhysicalDevicePrivateDataFeatures")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDevicePrivateDataFeatures {
    #[doc(alias = "privateData")]
    pub private_data: bool,
}
impl PhysicalDevicePrivateDataFeatures {
    ///Get a reference to the `private_data` field.
    pub fn private_data(&self) -> &bool {
        &self.private_data
    }
    ///Get a mutable reference to the `private_data` field.
    pub fn private_data_mut(&mut self) -> &mut bool {
        &mut self.private_data
    }
    ///Sets the `private_data` field.
    pub fn set_private_data(&mut self, private_data: bool) -> &mut Self {
        self.private_data = private_data;
        self
    }
    ///Sets the `private_data` field in a builder way.
    pub fn with_private_data(mut self, private_data: bool) -> Self {
        self.private_data = private_data;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDevicePrivateDataFeatures {
    type LowLevel = crate::native::vulkan1_3::PhysicalDevicePrivateDataFeatures;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::PhysicalDevicePrivateDataFeatures {
            s_type: StructureType::PhysicalDevicePrivateDataFeatures,
            p_next: std::ptr::null_mut(),
            private_data: self.private_data.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDevicePrivateDataFeatures {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            private_data: crate::conv::FromLowLevel::from_low_level(context, value.private_data),
        }
    }
}
#[doc(alias = "VkDeviceBufferMemoryRequirements")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DeviceBufferMemoryRequirements {
    #[doc(alias = "pCreateInfo")]
    pub create_info: BufferCreateInfo,
}
impl DeviceBufferMemoryRequirements {
    ///Get a reference to the `create_info` field.
    pub fn create_info(&self) -> &BufferCreateInfo {
        &self.create_info
    }
    ///Get a mutable reference to the `create_info` field.
    pub fn create_info_mut(&mut self) -> &mut BufferCreateInfo {
        &mut self.create_info
    }
    ///Sets the `create_info` field.
    pub fn set_create_info(&mut self, create_info: BufferCreateInfo) -> &mut Self {
        self.create_info = create_info;
        self
    }
    ///Sets the `create_info` field in a builder way.
    pub fn with_create_info(mut self, create_info: BufferCreateInfo) -> Self {
        self.create_info = create_info;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DeviceBufferMemoryRequirements {
    type LowLevel = crate::native::vulkan1_3::DeviceBufferMemoryRequirements;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::DeviceBufferMemoryRequirements {
            s_type: StructureType::DeviceBufferMemoryRequirements,
            p_next: std::ptr::null(),
            create_info: bump.alloc(self.create_info.into_low_level(context, bump)) as *const _,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DeviceBufferMemoryRequirements {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            create_info: crate::conv::FromLowLevel::from_low_level(context, *value.create_info),
        }
    }
}
#[doc(alias = "VkDeviceImageMemoryRequirements")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DeviceImageMemoryRequirements {
    #[doc(alias = "pCreateInfo")]
    pub create_info: ImageCreateInfo,
    #[doc(alias = "planeAspect")]
    pub plane_aspect: ImageAspectFlagBits,
}
impl DeviceImageMemoryRequirements {
    ///Get a reference to the `create_info` field.
    pub fn create_info(&self) -> &ImageCreateInfo {
        &self.create_info
    }
    ///Get a reference to the `plane_aspect` field.
    pub fn plane_aspect(&self) -> ImageAspectFlagBits {
        self.plane_aspect
    }
    ///Get a mutable reference to the `create_info` field.
    pub fn create_info_mut(&mut self) -> &mut ImageCreateInfo {
        &mut self.create_info
    }
    ///Get a mutable reference to the `plane_aspect` field.
    pub fn plane_aspect_mut(&mut self) -> &mut ImageAspectFlagBits {
        &mut self.plane_aspect
    }
    ///Sets the `create_info` field.
    pub fn set_create_info(&mut self, create_info: ImageCreateInfo) -> &mut Self {
        self.create_info = create_info;
        self
    }
    ///Sets the `plane_aspect` field.
    pub fn set_plane_aspect(&mut self, plane_aspect: ImageAspectFlagBits) -> &mut Self {
        self.plane_aspect = plane_aspect;
        self
    }
    ///Sets the `create_info` field in a builder way.
    pub fn with_create_info(mut self, create_info: ImageCreateInfo) -> Self {
        self.create_info = create_info;
        self
    }
    ///Sets the `plane_aspect` field in a builder way.
    pub fn with_plane_aspect(mut self, plane_aspect: ImageAspectFlagBits) -> Self {
        self.plane_aspect = plane_aspect;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DeviceImageMemoryRequirements {
    type LowLevel = crate::native::vulkan1_3::DeviceImageMemoryRequirements;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::DeviceImageMemoryRequirements {
            s_type: StructureType::DeviceImageMemoryRequirements,
            p_next: std::ptr::null(),
            create_info: bump.alloc(self.create_info.into_low_level(context, bump)) as *const _,
            plane_aspect: self.plane_aspect.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DeviceImageMemoryRequirements {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            create_info: crate::conv::FromLowLevel::from_low_level(context, *value.create_info),
            plane_aspect: crate::conv::FromLowLevel::from_low_level(context, value.plane_aspect),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceInlineUniformBlockFeatures")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceInlineUniformBlockFeatures {
    #[doc(alias = "inlineUniformBlock")]
    pub inline_uniform_block: bool,
    #[doc(alias = "descriptorBindingInlineUniformBlockUpdateAfterBind")]
    pub descriptor_binding_inline_uniform_block_update_after_bind: bool,
}
impl PhysicalDeviceInlineUniformBlockFeatures {
    ///Get a reference to the `inline_uniform_block` field.
    pub fn inline_uniform_block(&self) -> &bool {
        &self.inline_uniform_block
    }
    ///Get a reference to the `descriptor_binding_inline_uniform_block_update_after_bind` field.
    pub fn descriptor_binding_inline_uniform_block_update_after_bind(&self) -> &bool {
        &self.descriptor_binding_inline_uniform_block_update_after_bind
    }
    ///Get a mutable reference to the `inline_uniform_block` field.
    pub fn inline_uniform_block_mut(&mut self) -> &mut bool {
        &mut self.inline_uniform_block
    }
    ///Get a mutable reference to the `descriptor_binding_inline_uniform_block_update_after_bind`
    /// field.
    pub fn descriptor_binding_inline_uniform_block_update_after_bind_mut(&mut self) -> &mut bool {
        &mut self.descriptor_binding_inline_uniform_block_update_after_bind
    }
    ///Sets the `inline_uniform_block` field.
    pub fn set_inline_uniform_block(&mut self, inline_uniform_block: bool) -> &mut Self {
        self.inline_uniform_block = inline_uniform_block;
        self
    }
    ///Sets the `descriptor_binding_inline_uniform_block_update_after_bind` field.
    pub fn set_descriptor_binding_inline_uniform_block_update_after_bind(
        &mut self,
        descriptor_binding_inline_uniform_block_update_after_bind: bool,
    ) -> &mut Self {
        self.descriptor_binding_inline_uniform_block_update_after_bind =
            descriptor_binding_inline_uniform_block_update_after_bind;
        self
    }
    ///Sets the `inline_uniform_block` field in a builder way.
    pub fn with_inline_uniform_block(mut self, inline_uniform_block: bool) -> Self {
        self.inline_uniform_block = inline_uniform_block;
        self
    }
    ///Sets the `descriptor_binding_inline_uniform_block_update_after_bind` field in a builder way.
    pub fn with_descriptor_binding_inline_uniform_block_update_after_bind(
        mut self,
        descriptor_binding_inline_uniform_block_update_after_bind: bool,
    ) -> Self {
        self.descriptor_binding_inline_uniform_block_update_after_bind =
            descriptor_binding_inline_uniform_block_update_after_bind;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceInlineUniformBlockFeatures {
    type LowLevel = crate::native::vulkan1_3::PhysicalDeviceInlineUniformBlockFeatures;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::PhysicalDeviceInlineUniformBlockFeatures {
            s_type: StructureType::PhysicalDeviceInlineUniformBlockFeatures,
            p_next: std::ptr::null_mut(),
            inline_uniform_block: self.inline_uniform_block.into_low_level(context, bump),
            descriptor_binding_inline_uniform_block_update_after_bind: self
                .descriptor_binding_inline_uniform_block_update_after_bind
                .into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceInlineUniformBlockFeatures {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            inline_uniform_block: crate::conv::FromLowLevel::from_low_level(context, value.inline_uniform_block),
            descriptor_binding_inline_uniform_block_update_after_bind: crate::conv::FromLowLevel::from_low_level(
                context,
                value.descriptor_binding_inline_uniform_block_update_after_bind,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceInlineUniformBlockProperties")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceInlineUniformBlockProperties {
    #[doc(alias = "maxInlineUniformBlockSize")]
    pub max_inline_uniform_block_size: u32,
    #[doc(alias = "maxPerStageDescriptorInlineUniformBlocks")]
    pub max_per_stage_descriptor_inline_uniform_blocks: u32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindInlineUniformBlocks")]
    pub max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: u32,
    #[doc(alias = "maxDescriptorSetInlineUniformBlocks")]
    pub max_descriptor_set_inline_uniform_blocks: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindInlineUniformBlocks")]
    pub max_descriptor_set_update_after_bind_inline_uniform_blocks: u32,
}
impl PhysicalDeviceInlineUniformBlockProperties {
    ///Get a reference to the `max_inline_uniform_block_size` field.
    pub fn max_inline_uniform_block_size(&self) -> u32 {
        self.max_inline_uniform_block_size
    }
    ///Get a reference to the `max_per_stage_descriptor_inline_uniform_blocks` field.
    pub fn max_per_stage_descriptor_inline_uniform_blocks(&self) -> u32 {
        self.max_per_stage_descriptor_inline_uniform_blocks
    }
    ///Get a reference to the `max_per_stage_descriptor_update_after_bind_inline_uniform_blocks`
    /// field.
    pub fn max_per_stage_descriptor_update_after_bind_inline_uniform_blocks(&self) -> u32 {
        self.max_per_stage_descriptor_update_after_bind_inline_uniform_blocks
    }
    ///Get a reference to the `max_descriptor_set_inline_uniform_blocks` field.
    pub fn max_descriptor_set_inline_uniform_blocks(&self) -> u32 {
        self.max_descriptor_set_inline_uniform_blocks
    }
    ///Get a reference to the `max_descriptor_set_update_after_bind_inline_uniform_blocks` field.
    pub fn max_descriptor_set_update_after_bind_inline_uniform_blocks(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_inline_uniform_blocks
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceInlineUniformBlockProperties {
    type LowLevel = crate::native::vulkan1_3::PhysicalDeviceInlineUniformBlockProperties;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::PhysicalDeviceInlineUniformBlockProperties {
            s_type: StructureType::PhysicalDeviceInlineUniformBlockProperties,
            p_next: std::ptr::null_mut(),
            max_inline_uniform_block_size: self.max_inline_uniform_block_size.into_low_level(context, bump),
            max_per_stage_descriptor_inline_uniform_blocks: self
                .max_per_stage_descriptor_inline_uniform_blocks
                .into_low_level(context, bump),
            max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: self
                .max_per_stage_descriptor_update_after_bind_inline_uniform_blocks
                .into_low_level(context, bump),
            max_descriptor_set_inline_uniform_blocks: self
                .max_descriptor_set_inline_uniform_blocks
                .into_low_level(context, bump),
            max_descriptor_set_update_after_bind_inline_uniform_blocks: self
                .max_descriptor_set_update_after_bind_inline_uniform_blocks
                .into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceInlineUniformBlockProperties {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            max_inline_uniform_block_size: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_inline_uniform_block_size,
            ),
            max_per_stage_descriptor_inline_uniform_blocks: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_per_stage_descriptor_inline_uniform_blocks,
            ),
            max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_per_stage_descriptor_update_after_bind_inline_uniform_blocks,
            ),
            max_descriptor_set_inline_uniform_blocks: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_descriptor_set_inline_uniform_blocks,
            ),
            max_descriptor_set_update_after_bind_inline_uniform_blocks: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_descriptor_set_update_after_bind_inline_uniform_blocks,
            ),
        }
    }
}
#[doc(alias = "VkWriteDescriptorSetInlineUniformBlock")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WriteDescriptorSetInlineUniformBlock {
    #[doc(alias = "pData")]
    pub data: Vec<u8>,
}
impl WriteDescriptorSetInlineUniformBlock {
    ///Get a reference to the `data` field.
    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
    ///Get a mutable reference to the `data` field.
    pub fn data_mut(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }
    ///Sets the `data` field.
    pub fn set_data(&mut self, data: Vec<u8>) -> &mut Self {
        self.data = data;
        self
    }
    ///Sets the `data` field in a builder way.
    pub fn with_data(mut self, data: Vec<u8>) -> Self {
        self.data = data;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for WriteDescriptorSetInlineUniformBlock {
    type LowLevel = crate::native::vulkan1_3::WriteDescriptorSetInlineUniformBlock;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_data = self.data.len() as u32;
        crate::native::vulkan1_3::WriteDescriptorSetInlineUniformBlock {
            s_type: StructureType::WriteDescriptorSetInlineUniformBlock,
            p_next: std::ptr::null(),
            data_size: len_data,
            data: self.data.as_ptr().cast(),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for WriteDescriptorSetInlineUniformBlock {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let data_len = value.data_size;
        let mut data = Vec::with_capacity(data_len as usize);
        let data_ptr = value.data as *const u8;
        for i in 0..data_len {
            data.push(crate::conv::FromLowLevel::from_low_level(
                context,
                data_ptr.add(i as usize).read(),
            ));
        }
        Self { data: data }
    }
}
#[doc(alias = "VkDescriptorPoolInlineUniformBlockCreateInfo")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DescriptorPoolInlineUniformBlockCreateInfo {
    #[doc(alias = "maxInlineUniformBlockBindings")]
    pub max_inline_uniform_block_bindings: u32,
}
impl DescriptorPoolInlineUniformBlockCreateInfo {
    ///Get a reference to the `max_inline_uniform_block_bindings` field.
    pub fn max_inline_uniform_block_bindings(&self) -> u32 {
        self.max_inline_uniform_block_bindings
    }
    ///Get a mutable reference to the `max_inline_uniform_block_bindings` field.
    pub fn max_inline_uniform_block_bindings_mut(&mut self) -> &mut u32 {
        &mut self.max_inline_uniform_block_bindings
    }
    ///Sets the `max_inline_uniform_block_bindings` field.
    pub fn set_max_inline_uniform_block_bindings(&mut self, max_inline_uniform_block_bindings: u32) -> &mut Self {
        self.max_inline_uniform_block_bindings = max_inline_uniform_block_bindings;
        self
    }
    ///Sets the `max_inline_uniform_block_bindings` field in a builder way.
    pub fn with_max_inline_uniform_block_bindings(mut self, max_inline_uniform_block_bindings: u32) -> Self {
        self.max_inline_uniform_block_bindings = max_inline_uniform_block_bindings;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DescriptorPoolInlineUniformBlockCreateInfo {
    type LowLevel = crate::native::vulkan1_3::DescriptorPoolInlineUniformBlockCreateInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::DescriptorPoolInlineUniformBlockCreateInfo {
            s_type: StructureType::DescriptorPoolInlineUniformBlockCreateInfo,
            p_next: std::ptr::null(),
            max_inline_uniform_block_bindings: self.max_inline_uniform_block_bindings.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DescriptorPoolInlineUniformBlockCreateInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            max_inline_uniform_block_bindings: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_inline_uniform_block_bindings,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceMaintenance4Features")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceMaintenance4Features {
    pub maintenance4: bool,
}
impl PhysicalDeviceMaintenance4Features {
    ///Get a reference to the `maintenance4` field.
    pub fn maintenance4(&self) -> &bool {
        &self.maintenance4
    }
    ///Get a mutable reference to the `maintenance4` field.
    pub fn maintenance4_mut(&mut self) -> &mut bool {
        &mut self.maintenance4
    }
    ///Sets the `maintenance4` field.
    pub fn set_maintenance4(&mut self, maintenance4: bool) -> &mut Self {
        self.maintenance4 = maintenance4;
        self
    }
    ///Sets the `maintenance4` field in a builder way.
    pub fn with_maintenance4(mut self, maintenance4: bool) -> Self {
        self.maintenance4 = maintenance4;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceMaintenance4Features {
    type LowLevel = crate::native::vulkan1_3::PhysicalDeviceMaintenance4Features;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::PhysicalDeviceMaintenance4Features {
            s_type: StructureType::PhysicalDeviceMaintenance4Features,
            p_next: std::ptr::null_mut(),
            maintenance4: self.maintenance4.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceMaintenance4Features {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            maintenance4: crate::conv::FromLowLevel::from_low_level(context, value.maintenance4),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceMaintenance4Properties")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceMaintenance4Properties {
    #[doc(alias = "maxBufferSize")]
    pub max_buffer_size: DeviceSize,
}
impl PhysicalDeviceMaintenance4Properties {
    ///Get a reference to the `max_buffer_size` field.
    pub fn max_buffer_size(&self) -> &DeviceSize {
        &self.max_buffer_size
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceMaintenance4Properties {
    type LowLevel = crate::native::vulkan1_3::PhysicalDeviceMaintenance4Properties;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::PhysicalDeviceMaintenance4Properties {
            s_type: StructureType::PhysicalDeviceMaintenance4Properties,
            p_next: std::ptr::null_mut(),
            max_buffer_size: self.max_buffer_size.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceMaintenance4Properties {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            max_buffer_size: crate::conv::FromLowLevel::from_low_level(context, value.max_buffer_size),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceTextureCompressionASTCHDRFeatures")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceTextureCompressionAstchdrFeatures {
    #[doc(alias = "textureCompressionASTC_HDR")]
    pub texture_compression_astc_hdr: bool,
}
impl PhysicalDeviceTextureCompressionAstchdrFeatures {
    ///Get a reference to the `texture_compression_astc_hdr` field.
    pub fn texture_compression_astc_hdr(&self) -> &bool {
        &self.texture_compression_astc_hdr
    }
    ///Get a mutable reference to the `texture_compression_astc_hdr` field.
    pub fn texture_compression_astc_hdr_mut(&mut self) -> &mut bool {
        &mut self.texture_compression_astc_hdr
    }
    ///Sets the `texture_compression_astc_hdr` field.
    pub fn set_texture_compression_astc_hdr(&mut self, texture_compression_astc_hdr: bool) -> &mut Self {
        self.texture_compression_astc_hdr = texture_compression_astc_hdr;
        self
    }
    ///Sets the `texture_compression_astc_hdr` field in a builder way.
    pub fn with_texture_compression_astc_hdr(mut self, texture_compression_astc_hdr: bool) -> Self {
        self.texture_compression_astc_hdr = texture_compression_astc_hdr;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceTextureCompressionAstchdrFeatures {
    type LowLevel = crate::native::vulkan1_3::PhysicalDeviceTextureCompressionAstchdrFeatures;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::PhysicalDeviceTextureCompressionAstchdrFeatures {
            s_type: StructureType::PhysicalDeviceTextureCompressionAstcHdrFeatures,
            p_next: std::ptr::null_mut(),
            texture_compression_astc_hdr: self.texture_compression_astc_hdr.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceTextureCompressionAstchdrFeatures {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            texture_compression_astc_hdr: crate::conv::FromLowLevel::from_low_level(
                context,
                value.texture_compression_astc_hdr,
            ),
        }
    }
}
impl PipelineCreationFeedback {
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> PipelineCreationFeedbackFlags {
        self.flags
    }
    ///Get a reference to the `duration` field.
    pub fn duration(&self) -> u64 {
        self.duration
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineCreationFeedback {
    type LowLevel = crate::native::vulkan1_3::PipelineCreationFeedback;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::PipelineCreationFeedback {
            flags: self.flags.into_low_level(context, bump),
            duration: self.duration.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineCreationFeedback {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            duration: crate::conv::FromLowLevel::from_low_level(context, value.duration),
        }
    }
}
#[doc(alias = "VkPipelineCreationFeedbackCreateInfo")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineCreationFeedbackCreateInfo {
    #[doc(alias = "pPipelineCreationFeedback")]
    pub pipeline_creation_feedback: PipelineCreationFeedback,
    #[doc(alias = "pPipelineStageCreationFeedbacks")]
    pub pipeline_stage_creation_feedbacks: SmallVec<[PipelineCreationFeedback; 8]>,
}
impl PipelineCreationFeedbackCreateInfo {
    ///Get a reference to the `pipeline_creation_feedback` field.
    pub fn pipeline_creation_feedback(&self) -> PipelineCreationFeedback {
        self.pipeline_creation_feedback
    }
    ///Get a reference to the `pipeline_stage_creation_feedbacks` field.
    pub fn pipeline_stage_creation_feedbacks(&self) -> &SmallVec<[PipelineCreationFeedback; 8]> {
        &self.pipeline_stage_creation_feedbacks
    }
    ///Get a mutable reference to the `pipeline_creation_feedback` field.
    pub fn pipeline_creation_feedback_mut(&mut self) -> &mut PipelineCreationFeedback {
        &mut self.pipeline_creation_feedback
    }
    ///Get a mutable reference to the `pipeline_stage_creation_feedbacks` field.
    pub fn pipeline_stage_creation_feedbacks_mut(&mut self) -> &mut SmallVec<[PipelineCreationFeedback; 8]> {
        &mut self.pipeline_stage_creation_feedbacks
    }
    ///Sets the `pipeline_creation_feedback` field.
    pub fn set_pipeline_creation_feedback(
        &mut self,
        pipeline_creation_feedback: PipelineCreationFeedback,
    ) -> &mut Self {
        self.pipeline_creation_feedback = pipeline_creation_feedback;
        self
    }
    ///Sets the `pipeline_stage_creation_feedbacks` field.
    pub fn set_pipeline_stage_creation_feedbacks(
        &mut self,
        pipeline_stage_creation_feedbacks: SmallVec<[PipelineCreationFeedback; 8]>,
    ) -> &mut Self {
        self.pipeline_stage_creation_feedbacks = pipeline_stage_creation_feedbacks;
        self
    }
    ///Sets the `pipeline_creation_feedback` field in a builder way.
    pub fn with_pipeline_creation_feedback(mut self, pipeline_creation_feedback: PipelineCreationFeedback) -> Self {
        self.pipeline_creation_feedback = pipeline_creation_feedback;
        self
    }
    ///Sets the `pipeline_stage_creation_feedbacks` field in a builder way.
    pub fn with_pipeline_stage_creation_feedbacks(
        mut self,
        pipeline_stage_creation_feedbacks: SmallVec<[PipelineCreationFeedback; 8]>,
    ) -> Self {
        self.pipeline_stage_creation_feedbacks = pipeline_stage_creation_feedbacks;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineCreationFeedbackCreateInfo {
    type LowLevel = crate::native::vulkan1_3::PipelineCreationFeedbackCreateInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_pipeline_stage_creation_feedbacks = self.pipeline_stage_creation_feedbacks.len() as u32;
        let pipeline_stage_creation_feedbacks = bump
            .alloc_slice_fill_iter(
                self.pipeline_stage_creation_feedbacks
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_mut_ptr()
            .cast();
        crate::native::vulkan1_3::PipelineCreationFeedbackCreateInfo {
            s_type: StructureType::PipelineCreationFeedbackCreateInfo,
            p_next: std::ptr::null(),
            pipeline_creation_feedback: bump.alloc(self.pipeline_creation_feedback.into_low_level(context, bump))
                as *mut _,
            pipeline_stage_creation_feedback_count: len_pipeline_stage_creation_feedbacks,
            pipeline_stage_creation_feedbacks: pipeline_stage_creation_feedbacks,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineCreationFeedbackCreateInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let pipeline_stage_creation_feedbacks_len = value.pipeline_stage_creation_feedback_count;
        let mut pipeline_stage_creation_feedbacks =
            SmallVec::with_capacity(pipeline_stage_creation_feedbacks_len as usize);
        for i in 0..pipeline_stage_creation_feedbacks_len {
            pipeline_stage_creation_feedbacks.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.pipeline_stage_creation_feedbacks.add(i as usize).read(),
            ));
        }
        Self {
            pipeline_creation_feedback: crate::conv::FromLowLevel::from_low_level(
                context,
                *value.pipeline_creation_feedback,
            ),
            pipeline_stage_creation_feedbacks: pipeline_stage_creation_feedbacks,
        }
    }
}
#[doc(alias = "VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceShaderDemoteToHelperInvocationFeatures {
    #[doc(alias = "shaderDemoteToHelperInvocation")]
    pub shader_demote_to_helper_invocation: bool,
}
impl PhysicalDeviceShaderDemoteToHelperInvocationFeatures {
    ///Get a reference to the `shader_demote_to_helper_invocation` field.
    pub fn shader_demote_to_helper_invocation(&self) -> &bool {
        &self.shader_demote_to_helper_invocation
    }
    ///Get a mutable reference to the `shader_demote_to_helper_invocation` field.
    pub fn shader_demote_to_helper_invocation_mut(&mut self) -> &mut bool {
        &mut self.shader_demote_to_helper_invocation
    }
    ///Sets the `shader_demote_to_helper_invocation` field.
    pub fn set_shader_demote_to_helper_invocation(&mut self, shader_demote_to_helper_invocation: bool) -> &mut Self {
        self.shader_demote_to_helper_invocation = shader_demote_to_helper_invocation;
        self
    }
    ///Sets the `shader_demote_to_helper_invocation` field in a builder way.
    pub fn with_shader_demote_to_helper_invocation(mut self, shader_demote_to_helper_invocation: bool) -> Self {
        self.shader_demote_to_helper_invocation = shader_demote_to_helper_invocation;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceShaderDemoteToHelperInvocationFeatures {
    type LowLevel = crate::native::vulkan1_3::PhysicalDeviceShaderDemoteToHelperInvocationFeatures;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::PhysicalDeviceShaderDemoteToHelperInvocationFeatures {
            s_type: StructureType::PhysicalDeviceShaderDemoteToHelperInvocationFeatures,
            p_next: std::ptr::null_mut(),
            shader_demote_to_helper_invocation: self.shader_demote_to_helper_invocation.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceShaderDemoteToHelperInvocationFeatures {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            shader_demote_to_helper_invocation: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_demote_to_helper_invocation,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceTexelBufferAlignmentProperties")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceTexelBufferAlignmentProperties {
    #[doc(alias = "storageTexelBufferOffsetAlignmentBytes")]
    pub storage_texel_buffer_offset_alignment_bytes: DeviceSize,
    #[doc(alias = "storageTexelBufferOffsetSingleTexelAlignment")]
    pub storage_texel_buffer_offset_single_texel_alignment: bool,
    #[doc(alias = "uniformTexelBufferOffsetAlignmentBytes")]
    pub uniform_texel_buffer_offset_alignment_bytes: DeviceSize,
    #[doc(alias = "uniformTexelBufferOffsetSingleTexelAlignment")]
    pub uniform_texel_buffer_offset_single_texel_alignment: bool,
}
impl PhysicalDeviceTexelBufferAlignmentProperties {
    ///Get a reference to the `storage_texel_buffer_offset_alignment_bytes` field.
    pub fn storage_texel_buffer_offset_alignment_bytes(&self) -> &DeviceSize {
        &self.storage_texel_buffer_offset_alignment_bytes
    }
    ///Get a reference to the `storage_texel_buffer_offset_single_texel_alignment` field.
    pub fn storage_texel_buffer_offset_single_texel_alignment(&self) -> &bool {
        &self.storage_texel_buffer_offset_single_texel_alignment
    }
    ///Get a reference to the `uniform_texel_buffer_offset_alignment_bytes` field.
    pub fn uniform_texel_buffer_offset_alignment_bytes(&self) -> &DeviceSize {
        &self.uniform_texel_buffer_offset_alignment_bytes
    }
    ///Get a reference to the `uniform_texel_buffer_offset_single_texel_alignment` field.
    pub fn uniform_texel_buffer_offset_single_texel_alignment(&self) -> &bool {
        &self.uniform_texel_buffer_offset_single_texel_alignment
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceTexelBufferAlignmentProperties {
    type LowLevel = crate::native::vulkan1_3::PhysicalDeviceTexelBufferAlignmentProperties;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::PhysicalDeviceTexelBufferAlignmentProperties {
            s_type: StructureType::PhysicalDeviceTexelBufferAlignmentProperties,
            p_next: std::ptr::null_mut(),
            storage_texel_buffer_offset_alignment_bytes: self
                .storage_texel_buffer_offset_alignment_bytes
                .into_low_level(context, bump),
            storage_texel_buffer_offset_single_texel_alignment: self
                .storage_texel_buffer_offset_single_texel_alignment
                .into_low_level(context, bump),
            uniform_texel_buffer_offset_alignment_bytes: self
                .uniform_texel_buffer_offset_alignment_bytes
                .into_low_level(context, bump),
            uniform_texel_buffer_offset_single_texel_alignment: self
                .uniform_texel_buffer_offset_single_texel_alignment
                .into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceTexelBufferAlignmentProperties {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            storage_texel_buffer_offset_alignment_bytes: crate::conv::FromLowLevel::from_low_level(
                context,
                value.storage_texel_buffer_offset_alignment_bytes,
            ),
            storage_texel_buffer_offset_single_texel_alignment: crate::conv::FromLowLevel::from_low_level(
                context,
                value.storage_texel_buffer_offset_single_texel_alignment,
            ),
            uniform_texel_buffer_offset_alignment_bytes: crate::conv::FromLowLevel::from_low_level(
                context,
                value.uniform_texel_buffer_offset_alignment_bytes,
            ),
            uniform_texel_buffer_offset_single_texel_alignment: crate::conv::FromLowLevel::from_low_level(
                context,
                value.uniform_texel_buffer_offset_single_texel_alignment,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceSubgroupSizeControlFeatures")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceSubgroupSizeControlFeatures {
    #[doc(alias = "subgroupSizeControl")]
    pub subgroup_size_control: bool,
    #[doc(alias = "computeFullSubgroups")]
    pub compute_full_subgroups: bool,
}
impl PhysicalDeviceSubgroupSizeControlFeatures {
    ///Get a reference to the `subgroup_size_control` field.
    pub fn subgroup_size_control(&self) -> &bool {
        &self.subgroup_size_control
    }
    ///Get a reference to the `compute_full_subgroups` field.
    pub fn compute_full_subgroups(&self) -> &bool {
        &self.compute_full_subgroups
    }
    ///Get a mutable reference to the `subgroup_size_control` field.
    pub fn subgroup_size_control_mut(&mut self) -> &mut bool {
        &mut self.subgroup_size_control
    }
    ///Get a mutable reference to the `compute_full_subgroups` field.
    pub fn compute_full_subgroups_mut(&mut self) -> &mut bool {
        &mut self.compute_full_subgroups
    }
    ///Sets the `subgroup_size_control` field.
    pub fn set_subgroup_size_control(&mut self, subgroup_size_control: bool) -> &mut Self {
        self.subgroup_size_control = subgroup_size_control;
        self
    }
    ///Sets the `compute_full_subgroups` field.
    pub fn set_compute_full_subgroups(&mut self, compute_full_subgroups: bool) -> &mut Self {
        self.compute_full_subgroups = compute_full_subgroups;
        self
    }
    ///Sets the `subgroup_size_control` field in a builder way.
    pub fn with_subgroup_size_control(mut self, subgroup_size_control: bool) -> Self {
        self.subgroup_size_control = subgroup_size_control;
        self
    }
    ///Sets the `compute_full_subgroups` field in a builder way.
    pub fn with_compute_full_subgroups(mut self, compute_full_subgroups: bool) -> Self {
        self.compute_full_subgroups = compute_full_subgroups;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceSubgroupSizeControlFeatures {
    type LowLevel = crate::native::vulkan1_3::PhysicalDeviceSubgroupSizeControlFeatures;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::PhysicalDeviceSubgroupSizeControlFeatures {
            s_type: StructureType::PhysicalDeviceSubgroupSizeControlFeatures,
            p_next: std::ptr::null_mut(),
            subgroup_size_control: self.subgroup_size_control.into_low_level(context, bump),
            compute_full_subgroups: self.compute_full_subgroups.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceSubgroupSizeControlFeatures {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            subgroup_size_control: crate::conv::FromLowLevel::from_low_level(context, value.subgroup_size_control),
            compute_full_subgroups: crate::conv::FromLowLevel::from_low_level(context, value.compute_full_subgroups),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceSubgroupSizeControlProperties")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceSubgroupSizeControlProperties {
    #[doc(alias = "minSubgroupSize")]
    pub min_subgroup_size: u32,
    #[doc(alias = "maxSubgroupSize")]
    pub max_subgroup_size: u32,
    #[doc(alias = "maxComputeWorkgroupSubgroups")]
    pub max_compute_workgroup_subgroups: u32,
    #[doc(alias = "requiredSubgroupSizeStages")]
    pub required_subgroup_size_stages: ShaderStageFlags,
}
impl PhysicalDeviceSubgroupSizeControlProperties {
    ///Get a reference to the `min_subgroup_size` field.
    pub fn min_subgroup_size(&self) -> u32 {
        self.min_subgroup_size
    }
    ///Get a reference to the `max_subgroup_size` field.
    pub fn max_subgroup_size(&self) -> u32 {
        self.max_subgroup_size
    }
    ///Get a reference to the `max_compute_workgroup_subgroups` field.
    pub fn max_compute_workgroup_subgroups(&self) -> u32 {
        self.max_compute_workgroup_subgroups
    }
    ///Get a reference to the `required_subgroup_size_stages` field.
    pub fn required_subgroup_size_stages(&self) -> ShaderStageFlags {
        self.required_subgroup_size_stages
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceSubgroupSizeControlProperties {
    type LowLevel = crate::native::vulkan1_3::PhysicalDeviceSubgroupSizeControlProperties;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::PhysicalDeviceSubgroupSizeControlProperties {
            s_type: StructureType::PhysicalDeviceSubgroupSizeControlProperties,
            p_next: std::ptr::null_mut(),
            min_subgroup_size: self.min_subgroup_size.into_low_level(context, bump),
            max_subgroup_size: self.max_subgroup_size.into_low_level(context, bump),
            max_compute_workgroup_subgroups: self.max_compute_workgroup_subgroups.into_low_level(context, bump),
            required_subgroup_size_stages: self.required_subgroup_size_stages.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceSubgroupSizeControlProperties {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            min_subgroup_size: crate::conv::FromLowLevel::from_low_level(context, value.min_subgroup_size),
            max_subgroup_size: crate::conv::FromLowLevel::from_low_level(context, value.max_subgroup_size),
            max_compute_workgroup_subgroups: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_compute_workgroup_subgroups,
            ),
            required_subgroup_size_stages: crate::conv::FromLowLevel::from_low_level(
                context,
                value.required_subgroup_size_stages,
            ),
        }
    }
}
#[doc(alias = "VkPipelineShaderStageRequiredSubgroupSizeCreateInfo")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineShaderStageRequiredSubgroupSizeCreateInfo {
    #[doc(alias = "requiredSubgroupSize")]
    pub required_subgroup_size: u32,
}
impl PipelineShaderStageRequiredSubgroupSizeCreateInfo {
    ///Get a reference to the `required_subgroup_size` field.
    pub fn required_subgroup_size(&self) -> u32 {
        self.required_subgroup_size
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineShaderStageRequiredSubgroupSizeCreateInfo {
    type LowLevel = crate::native::vulkan1_3::PipelineShaderStageRequiredSubgroupSizeCreateInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::PipelineShaderStageRequiredSubgroupSizeCreateInfo {
            s_type: StructureType::PipelineShaderStageRequiredSubgroupSizeCreateInfo,
            p_next: std::ptr::null_mut(),
            required_subgroup_size: self.required_subgroup_size.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineShaderStageRequiredSubgroupSizeCreateInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            required_subgroup_size: crate::conv::FromLowLevel::from_low_level(context, value.required_subgroup_size),
        }
    }
}
#[doc(alias = "VkPhysicalDevicePipelineCreationCacheControlFeatures")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDevicePipelineCreationCacheControlFeatures {
    #[doc(alias = "pipelineCreationCacheControl")]
    pub pipeline_creation_cache_control: bool,
}
impl PhysicalDevicePipelineCreationCacheControlFeatures {
    ///Get a reference to the `pipeline_creation_cache_control` field.
    pub fn pipeline_creation_cache_control(&self) -> &bool {
        &self.pipeline_creation_cache_control
    }
    ///Get a mutable reference to the `pipeline_creation_cache_control` field.
    pub fn pipeline_creation_cache_control_mut(&mut self) -> &mut bool {
        &mut self.pipeline_creation_cache_control
    }
    ///Sets the `pipeline_creation_cache_control` field.
    pub fn set_pipeline_creation_cache_control(&mut self, pipeline_creation_cache_control: bool) -> &mut Self {
        self.pipeline_creation_cache_control = pipeline_creation_cache_control;
        self
    }
    ///Sets the `pipeline_creation_cache_control` field in a builder way.
    pub fn with_pipeline_creation_cache_control(mut self, pipeline_creation_cache_control: bool) -> Self {
        self.pipeline_creation_cache_control = pipeline_creation_cache_control;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDevicePipelineCreationCacheControlFeatures {
    type LowLevel = crate::native::vulkan1_3::PhysicalDevicePipelineCreationCacheControlFeatures;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::PhysicalDevicePipelineCreationCacheControlFeatures {
            s_type: StructureType::PhysicalDevicePipelineCreationCacheControlFeatures,
            p_next: std::ptr::null_mut(),
            pipeline_creation_cache_control: self.pipeline_creation_cache_control.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDevicePipelineCreationCacheControlFeatures {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            pipeline_creation_cache_control: crate::conv::FromLowLevel::from_low_level(
                context,
                value.pipeline_creation_cache_control,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceVulkan13Features")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceVulkan13Features {
    #[doc(alias = "robustImageAccess")]
    pub robust_image_access: bool,
    #[doc(alias = "inlineUniformBlock")]
    pub inline_uniform_block: bool,
    #[doc(alias = "descriptorBindingInlineUniformBlockUpdateAfterBind")]
    pub descriptor_binding_inline_uniform_block_update_after_bind: bool,
    #[doc(alias = "pipelineCreationCacheControl")]
    pub pipeline_creation_cache_control: bool,
    #[doc(alias = "privateData")]
    pub private_data: bool,
    #[doc(alias = "shaderDemoteToHelperInvocation")]
    pub shader_demote_to_helper_invocation: bool,
    #[doc(alias = "shaderTerminateInvocation")]
    pub shader_terminate_invocation: bool,
    #[doc(alias = "subgroupSizeControl")]
    pub subgroup_size_control: bool,
    #[doc(alias = "computeFullSubgroups")]
    pub compute_full_subgroups: bool,
    pub synchronization2: bool,
    #[doc(alias = "textureCompressionASTC_HDR")]
    pub texture_compression_astc_hdr: bool,
    #[doc(alias = "shaderZeroInitializeWorkgroupMemory")]
    pub shader_zero_initialize_workgroup_memory: bool,
    #[doc(alias = "dynamicRendering")]
    pub dynamic_rendering: bool,
    #[doc(alias = "shaderIntegerDotProduct")]
    pub shader_integer_dot_product: bool,
    pub maintenance4: bool,
}
impl PhysicalDeviceVulkan13Features {
    ///Get a reference to the `robust_image_access` field.
    pub fn robust_image_access(&self) -> &bool {
        &self.robust_image_access
    }
    ///Get a reference to the `inline_uniform_block` field.
    pub fn inline_uniform_block(&self) -> &bool {
        &self.inline_uniform_block
    }
    ///Get a reference to the `descriptor_binding_inline_uniform_block_update_after_bind` field.
    pub fn descriptor_binding_inline_uniform_block_update_after_bind(&self) -> &bool {
        &self.descriptor_binding_inline_uniform_block_update_after_bind
    }
    ///Get a reference to the `pipeline_creation_cache_control` field.
    pub fn pipeline_creation_cache_control(&self) -> &bool {
        &self.pipeline_creation_cache_control
    }
    ///Get a reference to the `private_data` field.
    pub fn private_data(&self) -> &bool {
        &self.private_data
    }
    ///Get a reference to the `shader_demote_to_helper_invocation` field.
    pub fn shader_demote_to_helper_invocation(&self) -> &bool {
        &self.shader_demote_to_helper_invocation
    }
    ///Get a reference to the `shader_terminate_invocation` field.
    pub fn shader_terminate_invocation(&self) -> &bool {
        &self.shader_terminate_invocation
    }
    ///Get a reference to the `subgroup_size_control` field.
    pub fn subgroup_size_control(&self) -> &bool {
        &self.subgroup_size_control
    }
    ///Get a reference to the `compute_full_subgroups` field.
    pub fn compute_full_subgroups(&self) -> &bool {
        &self.compute_full_subgroups
    }
    ///Get a reference to the `synchronization2` field.
    pub fn synchronization2(&self) -> &bool {
        &self.synchronization2
    }
    ///Get a reference to the `texture_compression_astc_hdr` field.
    pub fn texture_compression_astc_hdr(&self) -> &bool {
        &self.texture_compression_astc_hdr
    }
    ///Get a reference to the `shader_zero_initialize_workgroup_memory` field.
    pub fn shader_zero_initialize_workgroup_memory(&self) -> &bool {
        &self.shader_zero_initialize_workgroup_memory
    }
    ///Get a reference to the `dynamic_rendering` field.
    pub fn dynamic_rendering(&self) -> &bool {
        &self.dynamic_rendering
    }
    ///Get a reference to the `shader_integer_dot_product` field.
    pub fn shader_integer_dot_product(&self) -> &bool {
        &self.shader_integer_dot_product
    }
    ///Get a reference to the `maintenance4` field.
    pub fn maintenance4(&self) -> &bool {
        &self.maintenance4
    }
    ///Get a mutable reference to the `robust_image_access` field.
    pub fn robust_image_access_mut(&mut self) -> &mut bool {
        &mut self.robust_image_access
    }
    ///Get a mutable reference to the `inline_uniform_block` field.
    pub fn inline_uniform_block_mut(&mut self) -> &mut bool {
        &mut self.inline_uniform_block
    }
    ///Get a mutable reference to the `descriptor_binding_inline_uniform_block_update_after_bind`
    /// field.
    pub fn descriptor_binding_inline_uniform_block_update_after_bind_mut(&mut self) -> &mut bool {
        &mut self.descriptor_binding_inline_uniform_block_update_after_bind
    }
    ///Get a mutable reference to the `pipeline_creation_cache_control` field.
    pub fn pipeline_creation_cache_control_mut(&mut self) -> &mut bool {
        &mut self.pipeline_creation_cache_control
    }
    ///Get a mutable reference to the `private_data` field.
    pub fn private_data_mut(&mut self) -> &mut bool {
        &mut self.private_data
    }
    ///Get a mutable reference to the `shader_demote_to_helper_invocation` field.
    pub fn shader_demote_to_helper_invocation_mut(&mut self) -> &mut bool {
        &mut self.shader_demote_to_helper_invocation
    }
    ///Get a mutable reference to the `shader_terminate_invocation` field.
    pub fn shader_terminate_invocation_mut(&mut self) -> &mut bool {
        &mut self.shader_terminate_invocation
    }
    ///Get a mutable reference to the `subgroup_size_control` field.
    pub fn subgroup_size_control_mut(&mut self) -> &mut bool {
        &mut self.subgroup_size_control
    }
    ///Get a mutable reference to the `compute_full_subgroups` field.
    pub fn compute_full_subgroups_mut(&mut self) -> &mut bool {
        &mut self.compute_full_subgroups
    }
    ///Get a mutable reference to the `synchronization2` field.
    pub fn synchronization2_mut(&mut self) -> &mut bool {
        &mut self.synchronization2
    }
    ///Get a mutable reference to the `texture_compression_astc_hdr` field.
    pub fn texture_compression_astc_hdr_mut(&mut self) -> &mut bool {
        &mut self.texture_compression_astc_hdr
    }
    ///Get a mutable reference to the `shader_zero_initialize_workgroup_memory` field.
    pub fn shader_zero_initialize_workgroup_memory_mut(&mut self) -> &mut bool {
        &mut self.shader_zero_initialize_workgroup_memory
    }
    ///Get a mutable reference to the `dynamic_rendering` field.
    pub fn dynamic_rendering_mut(&mut self) -> &mut bool {
        &mut self.dynamic_rendering
    }
    ///Get a mutable reference to the `shader_integer_dot_product` field.
    pub fn shader_integer_dot_product_mut(&mut self) -> &mut bool {
        &mut self.shader_integer_dot_product
    }
    ///Get a mutable reference to the `maintenance4` field.
    pub fn maintenance4_mut(&mut self) -> &mut bool {
        &mut self.maintenance4
    }
    ///Sets the `robust_image_access` field.
    pub fn set_robust_image_access(&mut self, robust_image_access: bool) -> &mut Self {
        self.robust_image_access = robust_image_access;
        self
    }
    ///Sets the `inline_uniform_block` field.
    pub fn set_inline_uniform_block(&mut self, inline_uniform_block: bool) -> &mut Self {
        self.inline_uniform_block = inline_uniform_block;
        self
    }
    ///Sets the `descriptor_binding_inline_uniform_block_update_after_bind` field.
    pub fn set_descriptor_binding_inline_uniform_block_update_after_bind(
        &mut self,
        descriptor_binding_inline_uniform_block_update_after_bind: bool,
    ) -> &mut Self {
        self.descriptor_binding_inline_uniform_block_update_after_bind =
            descriptor_binding_inline_uniform_block_update_after_bind;
        self
    }
    ///Sets the `pipeline_creation_cache_control` field.
    pub fn set_pipeline_creation_cache_control(&mut self, pipeline_creation_cache_control: bool) -> &mut Self {
        self.pipeline_creation_cache_control = pipeline_creation_cache_control;
        self
    }
    ///Sets the `private_data` field.
    pub fn set_private_data(&mut self, private_data: bool) -> &mut Self {
        self.private_data = private_data;
        self
    }
    ///Sets the `shader_demote_to_helper_invocation` field.
    pub fn set_shader_demote_to_helper_invocation(&mut self, shader_demote_to_helper_invocation: bool) -> &mut Self {
        self.shader_demote_to_helper_invocation = shader_demote_to_helper_invocation;
        self
    }
    ///Sets the `shader_terminate_invocation` field.
    pub fn set_shader_terminate_invocation(&mut self, shader_terminate_invocation: bool) -> &mut Self {
        self.shader_terminate_invocation = shader_terminate_invocation;
        self
    }
    ///Sets the `subgroup_size_control` field.
    pub fn set_subgroup_size_control(&mut self, subgroup_size_control: bool) -> &mut Self {
        self.subgroup_size_control = subgroup_size_control;
        self
    }
    ///Sets the `compute_full_subgroups` field.
    pub fn set_compute_full_subgroups(&mut self, compute_full_subgroups: bool) -> &mut Self {
        self.compute_full_subgroups = compute_full_subgroups;
        self
    }
    ///Sets the `synchronization2` field.
    pub fn set_synchronization2(&mut self, synchronization2: bool) -> &mut Self {
        self.synchronization2 = synchronization2;
        self
    }
    ///Sets the `texture_compression_astc_hdr` field.
    pub fn set_texture_compression_astc_hdr(&mut self, texture_compression_astc_hdr: bool) -> &mut Self {
        self.texture_compression_astc_hdr = texture_compression_astc_hdr;
        self
    }
    ///Sets the `shader_zero_initialize_workgroup_memory` field.
    pub fn set_shader_zero_initialize_workgroup_memory(
        &mut self,
        shader_zero_initialize_workgroup_memory: bool,
    ) -> &mut Self {
        self.shader_zero_initialize_workgroup_memory = shader_zero_initialize_workgroup_memory;
        self
    }
    ///Sets the `dynamic_rendering` field.
    pub fn set_dynamic_rendering(&mut self, dynamic_rendering: bool) -> &mut Self {
        self.dynamic_rendering = dynamic_rendering;
        self
    }
    ///Sets the `shader_integer_dot_product` field.
    pub fn set_shader_integer_dot_product(&mut self, shader_integer_dot_product: bool) -> &mut Self {
        self.shader_integer_dot_product = shader_integer_dot_product;
        self
    }
    ///Sets the `maintenance4` field.
    pub fn set_maintenance4(&mut self, maintenance4: bool) -> &mut Self {
        self.maintenance4 = maintenance4;
        self
    }
    ///Sets the `robust_image_access` field in a builder way.
    pub fn with_robust_image_access(mut self, robust_image_access: bool) -> Self {
        self.robust_image_access = robust_image_access;
        self
    }
    ///Sets the `inline_uniform_block` field in a builder way.
    pub fn with_inline_uniform_block(mut self, inline_uniform_block: bool) -> Self {
        self.inline_uniform_block = inline_uniform_block;
        self
    }
    ///Sets the `descriptor_binding_inline_uniform_block_update_after_bind` field in a builder way.
    pub fn with_descriptor_binding_inline_uniform_block_update_after_bind(
        mut self,
        descriptor_binding_inline_uniform_block_update_after_bind: bool,
    ) -> Self {
        self.descriptor_binding_inline_uniform_block_update_after_bind =
            descriptor_binding_inline_uniform_block_update_after_bind;
        self
    }
    ///Sets the `pipeline_creation_cache_control` field in a builder way.
    pub fn with_pipeline_creation_cache_control(mut self, pipeline_creation_cache_control: bool) -> Self {
        self.pipeline_creation_cache_control = pipeline_creation_cache_control;
        self
    }
    ///Sets the `private_data` field in a builder way.
    pub fn with_private_data(mut self, private_data: bool) -> Self {
        self.private_data = private_data;
        self
    }
    ///Sets the `shader_demote_to_helper_invocation` field in a builder way.
    pub fn with_shader_demote_to_helper_invocation(mut self, shader_demote_to_helper_invocation: bool) -> Self {
        self.shader_demote_to_helper_invocation = shader_demote_to_helper_invocation;
        self
    }
    ///Sets the `shader_terminate_invocation` field in a builder way.
    pub fn with_shader_terminate_invocation(mut self, shader_terminate_invocation: bool) -> Self {
        self.shader_terminate_invocation = shader_terminate_invocation;
        self
    }
    ///Sets the `subgroup_size_control` field in a builder way.
    pub fn with_subgroup_size_control(mut self, subgroup_size_control: bool) -> Self {
        self.subgroup_size_control = subgroup_size_control;
        self
    }
    ///Sets the `compute_full_subgroups` field in a builder way.
    pub fn with_compute_full_subgroups(mut self, compute_full_subgroups: bool) -> Self {
        self.compute_full_subgroups = compute_full_subgroups;
        self
    }
    ///Sets the `synchronization2` field in a builder way.
    pub fn with_synchronization2(mut self, synchronization2: bool) -> Self {
        self.synchronization2 = synchronization2;
        self
    }
    ///Sets the `texture_compression_astc_hdr` field in a builder way.
    pub fn with_texture_compression_astc_hdr(mut self, texture_compression_astc_hdr: bool) -> Self {
        self.texture_compression_astc_hdr = texture_compression_astc_hdr;
        self
    }
    ///Sets the `shader_zero_initialize_workgroup_memory` field in a builder way.
    pub fn with_shader_zero_initialize_workgroup_memory(
        mut self,
        shader_zero_initialize_workgroup_memory: bool,
    ) -> Self {
        self.shader_zero_initialize_workgroup_memory = shader_zero_initialize_workgroup_memory;
        self
    }
    ///Sets the `dynamic_rendering` field in a builder way.
    pub fn with_dynamic_rendering(mut self, dynamic_rendering: bool) -> Self {
        self.dynamic_rendering = dynamic_rendering;
        self
    }
    ///Sets the `shader_integer_dot_product` field in a builder way.
    pub fn with_shader_integer_dot_product(mut self, shader_integer_dot_product: bool) -> Self {
        self.shader_integer_dot_product = shader_integer_dot_product;
        self
    }
    ///Sets the `maintenance4` field in a builder way.
    pub fn with_maintenance4(mut self, maintenance4: bool) -> Self {
        self.maintenance4 = maintenance4;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceVulkan13Features {
    type LowLevel = crate::native::vulkan1_3::PhysicalDeviceVulkan13Features;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::PhysicalDeviceVulkan13Features {
            s_type: StructureType::PhysicalDeviceVulkan13Features,
            p_next: std::ptr::null_mut(),
            robust_image_access: self.robust_image_access.into_low_level(context, bump),
            inline_uniform_block: self.inline_uniform_block.into_low_level(context, bump),
            descriptor_binding_inline_uniform_block_update_after_bind: self
                .descriptor_binding_inline_uniform_block_update_after_bind
                .into_low_level(context, bump),
            pipeline_creation_cache_control: self.pipeline_creation_cache_control.into_low_level(context, bump),
            private_data: self.private_data.into_low_level(context, bump),
            shader_demote_to_helper_invocation: self.shader_demote_to_helper_invocation.into_low_level(context, bump),
            shader_terminate_invocation: self.shader_terminate_invocation.into_low_level(context, bump),
            subgroup_size_control: self.subgroup_size_control.into_low_level(context, bump),
            compute_full_subgroups: self.compute_full_subgroups.into_low_level(context, bump),
            synchronization2: self.synchronization2.into_low_level(context, bump),
            texture_compression_astc_hdr: self.texture_compression_astc_hdr.into_low_level(context, bump),
            shader_zero_initialize_workgroup_memory: self
                .shader_zero_initialize_workgroup_memory
                .into_low_level(context, bump),
            dynamic_rendering: self.dynamic_rendering.into_low_level(context, bump),
            shader_integer_dot_product: self.shader_integer_dot_product.into_low_level(context, bump),
            maintenance4: self.maintenance4.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceVulkan13Features {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            robust_image_access: crate::conv::FromLowLevel::from_low_level(context, value.robust_image_access),
            inline_uniform_block: crate::conv::FromLowLevel::from_low_level(context, value.inline_uniform_block),
            descriptor_binding_inline_uniform_block_update_after_bind: crate::conv::FromLowLevel::from_low_level(
                context,
                value.descriptor_binding_inline_uniform_block_update_after_bind,
            ),
            pipeline_creation_cache_control: crate::conv::FromLowLevel::from_low_level(
                context,
                value.pipeline_creation_cache_control,
            ),
            private_data: crate::conv::FromLowLevel::from_low_level(context, value.private_data),
            shader_demote_to_helper_invocation: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_demote_to_helper_invocation,
            ),
            shader_terminate_invocation: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_terminate_invocation,
            ),
            subgroup_size_control: crate::conv::FromLowLevel::from_low_level(context, value.subgroup_size_control),
            compute_full_subgroups: crate::conv::FromLowLevel::from_low_level(context, value.compute_full_subgroups),
            synchronization2: crate::conv::FromLowLevel::from_low_level(context, value.synchronization2),
            texture_compression_astc_hdr: crate::conv::FromLowLevel::from_low_level(
                context,
                value.texture_compression_astc_hdr,
            ),
            shader_zero_initialize_workgroup_memory: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_zero_initialize_workgroup_memory,
            ),
            dynamic_rendering: crate::conv::FromLowLevel::from_low_level(context, value.dynamic_rendering),
            shader_integer_dot_product: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_integer_dot_product,
            ),
            maintenance4: crate::conv::FromLowLevel::from_low_level(context, value.maintenance4),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceVulkan13Properties")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceVulkan13Properties {
    #[doc(alias = "minSubgroupSize")]
    pub min_subgroup_size: u32,
    #[doc(alias = "maxSubgroupSize")]
    pub max_subgroup_size: u32,
    #[doc(alias = "maxComputeWorkgroupSubgroups")]
    pub max_compute_workgroup_subgroups: u32,
    #[doc(alias = "requiredSubgroupSizeStages")]
    pub required_subgroup_size_stages: ShaderStageFlags,
    #[doc(alias = "maxInlineUniformBlockSize")]
    pub max_inline_uniform_block_size: u32,
    #[doc(alias = "maxPerStageDescriptorInlineUniformBlocks")]
    pub max_per_stage_descriptor_inline_uniform_blocks: u32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindInlineUniformBlocks")]
    pub max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: u32,
    #[doc(alias = "maxDescriptorSetInlineUniformBlocks")]
    pub max_descriptor_set_inline_uniform_blocks: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindInlineUniformBlocks")]
    pub max_descriptor_set_update_after_bind_inline_uniform_blocks: u32,
    #[doc(alias = "maxInlineUniformTotalSize")]
    pub max_inline_uniform_total_size: u32,
    #[doc(alias = "integerDotProduct8BitUnsignedAccelerated")]
    pub integer_dot_product8_bit_unsigned_accelerated: bool,
    #[doc(alias = "integerDotProduct8BitSignedAccelerated")]
    pub integer_dot_product8_bit_signed_accelerated: bool,
    #[doc(alias = "integerDotProduct8BitMixedSignednessAccelerated")]
    pub integer_dot_product8_bit_mixed_signedness_accelerated: bool,
    #[doc(alias = "integerDotProduct4x8BitPackedUnsignedAccelerated")]
    pub integer_dot_product4x8_bit_packed_unsigned_accelerated: bool,
    #[doc(alias = "integerDotProduct4x8BitPackedSignedAccelerated")]
    pub integer_dot_product4x8_bit_packed_signed_accelerated: bool,
    #[doc(alias = "integerDotProduct4x8BitPackedMixedSignednessAccelerated")]
    pub integer_dot_product4x8_bit_packed_mixed_signedness_accelerated: bool,
    #[doc(alias = "integerDotProduct16BitUnsignedAccelerated")]
    pub integer_dot_product16_bit_unsigned_accelerated: bool,
    #[doc(alias = "integerDotProduct16BitSignedAccelerated")]
    pub integer_dot_product16_bit_signed_accelerated: bool,
    #[doc(alias = "integerDotProduct16BitMixedSignednessAccelerated")]
    pub integer_dot_product16_bit_mixed_signedness_accelerated: bool,
    #[doc(alias = "integerDotProduct32BitUnsignedAccelerated")]
    pub integer_dot_product32_bit_unsigned_accelerated: bool,
    #[doc(alias = "integerDotProduct32BitSignedAccelerated")]
    pub integer_dot_product32_bit_signed_accelerated: bool,
    #[doc(alias = "integerDotProduct32BitMixedSignednessAccelerated")]
    pub integer_dot_product32_bit_mixed_signedness_accelerated: bool,
    #[doc(alias = "integerDotProduct64BitUnsignedAccelerated")]
    pub integer_dot_product64_bit_unsigned_accelerated: bool,
    #[doc(alias = "integerDotProduct64BitSignedAccelerated")]
    pub integer_dot_product64_bit_signed_accelerated: bool,
    #[doc(alias = "integerDotProduct64BitMixedSignednessAccelerated")]
    pub integer_dot_product64_bit_mixed_signedness_accelerated: bool,
    #[doc(alias = "integerDotProductAccumulatingSaturating8BitUnsignedAccelerated")]
    pub integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated: bool,
    #[doc(alias = "integerDotProductAccumulatingSaturating8BitSignedAccelerated")]
    pub integer_dot_product_accumulating_saturating8_bit_signed_accelerated: bool,
    #[doc(alias = "integerDotProductAccumulatingSaturating8BitMixedSignednessAccelerated")]
    pub integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated: bool,
    #[doc(alias = "integerDotProductAccumulatingSaturating4x8BitPackedUnsignedAccelerated")]
    pub integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated: bool,
    #[doc(alias = "integerDotProductAccumulatingSaturating4x8BitPackedSignedAccelerated")]
    pub integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated: bool,
    #[doc(alias = "integerDotProductAccumulatingSaturating4x8BitPackedMixedSignednessAccelerated")]
    pub integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated: bool,
    #[doc(alias = "integerDotProductAccumulatingSaturating16BitUnsignedAccelerated")]
    pub integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated: bool,
    #[doc(alias = "integerDotProductAccumulatingSaturating16BitSignedAccelerated")]
    pub integer_dot_product_accumulating_saturating16_bit_signed_accelerated: bool,
    #[doc(alias = "integerDotProductAccumulatingSaturating16BitMixedSignednessAccelerated")]
    pub integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated: bool,
    #[doc(alias = "integerDotProductAccumulatingSaturating32BitUnsignedAccelerated")]
    pub integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated: bool,
    #[doc(alias = "integerDotProductAccumulatingSaturating32BitSignedAccelerated")]
    pub integer_dot_product_accumulating_saturating32_bit_signed_accelerated: bool,
    #[doc(alias = "integerDotProductAccumulatingSaturating32BitMixedSignednessAccelerated")]
    pub integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated: bool,
    #[doc(alias = "integerDotProductAccumulatingSaturating64BitUnsignedAccelerated")]
    pub integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated: bool,
    #[doc(alias = "integerDotProductAccumulatingSaturating64BitSignedAccelerated")]
    pub integer_dot_product_accumulating_saturating64_bit_signed_accelerated: bool,
    #[doc(alias = "integerDotProductAccumulatingSaturating64BitMixedSignednessAccelerated")]
    pub integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated: bool,
    #[doc(alias = "storageTexelBufferOffsetAlignmentBytes")]
    pub storage_texel_buffer_offset_alignment_bytes: DeviceSize,
    #[doc(alias = "storageTexelBufferOffsetSingleTexelAlignment")]
    pub storage_texel_buffer_offset_single_texel_alignment: bool,
    #[doc(alias = "uniformTexelBufferOffsetAlignmentBytes")]
    pub uniform_texel_buffer_offset_alignment_bytes: DeviceSize,
    #[doc(alias = "uniformTexelBufferOffsetSingleTexelAlignment")]
    pub uniform_texel_buffer_offset_single_texel_alignment: bool,
    #[doc(alias = "maxBufferSize")]
    pub max_buffer_size: DeviceSize,
}
impl PhysicalDeviceVulkan13Properties {
    ///Get a reference to the `min_subgroup_size` field.
    pub fn min_subgroup_size(&self) -> u32 {
        self.min_subgroup_size
    }
    ///Get a reference to the `max_subgroup_size` field.
    pub fn max_subgroup_size(&self) -> u32 {
        self.max_subgroup_size
    }
    ///Get a reference to the `max_compute_workgroup_subgroups` field.
    pub fn max_compute_workgroup_subgroups(&self) -> u32 {
        self.max_compute_workgroup_subgroups
    }
    ///Get a reference to the `required_subgroup_size_stages` field.
    pub fn required_subgroup_size_stages(&self) -> ShaderStageFlags {
        self.required_subgroup_size_stages
    }
    ///Get a reference to the `max_inline_uniform_block_size` field.
    pub fn max_inline_uniform_block_size(&self) -> u32 {
        self.max_inline_uniform_block_size
    }
    ///Get a reference to the `max_per_stage_descriptor_inline_uniform_blocks` field.
    pub fn max_per_stage_descriptor_inline_uniform_blocks(&self) -> u32 {
        self.max_per_stage_descriptor_inline_uniform_blocks
    }
    ///Get a reference to the `max_per_stage_descriptor_update_after_bind_inline_uniform_blocks`
    /// field.
    pub fn max_per_stage_descriptor_update_after_bind_inline_uniform_blocks(&self) -> u32 {
        self.max_per_stage_descriptor_update_after_bind_inline_uniform_blocks
    }
    ///Get a reference to the `max_descriptor_set_inline_uniform_blocks` field.
    pub fn max_descriptor_set_inline_uniform_blocks(&self) -> u32 {
        self.max_descriptor_set_inline_uniform_blocks
    }
    ///Get a reference to the `max_descriptor_set_update_after_bind_inline_uniform_blocks` field.
    pub fn max_descriptor_set_update_after_bind_inline_uniform_blocks(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_inline_uniform_blocks
    }
    ///Get a reference to the `max_inline_uniform_total_size` field.
    pub fn max_inline_uniform_total_size(&self) -> u32 {
        self.max_inline_uniform_total_size
    }
    ///Get a reference to the `integer_dot_product8_bit_unsigned_accelerated` field.
    pub fn integer_dot_product8_bit_unsigned_accelerated(&self) -> &bool {
        &self.integer_dot_product8_bit_unsigned_accelerated
    }
    ///Get a reference to the `integer_dot_product8_bit_signed_accelerated` field.
    pub fn integer_dot_product8_bit_signed_accelerated(&self) -> &bool {
        &self.integer_dot_product8_bit_signed_accelerated
    }
    ///Get a reference to the `integer_dot_product8_bit_mixed_signedness_accelerated` field.
    pub fn integer_dot_product8_bit_mixed_signedness_accelerated(&self) -> &bool {
        &self.integer_dot_product8_bit_mixed_signedness_accelerated
    }
    ///Get a reference to the `integer_dot_product4x8_bit_packed_unsigned_accelerated` field.
    pub fn integer_dot_product4x8_bit_packed_unsigned_accelerated(&self) -> &bool {
        &self.integer_dot_product4x8_bit_packed_unsigned_accelerated
    }
    ///Get a reference to the `integer_dot_product4x8_bit_packed_signed_accelerated` field.
    pub fn integer_dot_product4x8_bit_packed_signed_accelerated(&self) -> &bool {
        &self.integer_dot_product4x8_bit_packed_signed_accelerated
    }
    ///Get a reference to the `integer_dot_product4x8_bit_packed_mixed_signedness_accelerated`
    /// field.
    pub fn integer_dot_product4x8_bit_packed_mixed_signedness_accelerated(&self) -> &bool {
        &self.integer_dot_product4x8_bit_packed_mixed_signedness_accelerated
    }
    ///Get a reference to the `integer_dot_product16_bit_unsigned_accelerated` field.
    pub fn integer_dot_product16_bit_unsigned_accelerated(&self) -> &bool {
        &self.integer_dot_product16_bit_unsigned_accelerated
    }
    ///Get a reference to the `integer_dot_product16_bit_signed_accelerated` field.
    pub fn integer_dot_product16_bit_signed_accelerated(&self) -> &bool {
        &self.integer_dot_product16_bit_signed_accelerated
    }
    ///Get a reference to the `integer_dot_product16_bit_mixed_signedness_accelerated` field.
    pub fn integer_dot_product16_bit_mixed_signedness_accelerated(&self) -> &bool {
        &self.integer_dot_product16_bit_mixed_signedness_accelerated
    }
    ///Get a reference to the `integer_dot_product32_bit_unsigned_accelerated` field.
    pub fn integer_dot_product32_bit_unsigned_accelerated(&self) -> &bool {
        &self.integer_dot_product32_bit_unsigned_accelerated
    }
    ///Get a reference to the `integer_dot_product32_bit_signed_accelerated` field.
    pub fn integer_dot_product32_bit_signed_accelerated(&self) -> &bool {
        &self.integer_dot_product32_bit_signed_accelerated
    }
    ///Get a reference to the `integer_dot_product32_bit_mixed_signedness_accelerated` field.
    pub fn integer_dot_product32_bit_mixed_signedness_accelerated(&self) -> &bool {
        &self.integer_dot_product32_bit_mixed_signedness_accelerated
    }
    ///Get a reference to the `integer_dot_product64_bit_unsigned_accelerated` field.
    pub fn integer_dot_product64_bit_unsigned_accelerated(&self) -> &bool {
        &self.integer_dot_product64_bit_unsigned_accelerated
    }
    ///Get a reference to the `integer_dot_product64_bit_signed_accelerated` field.
    pub fn integer_dot_product64_bit_signed_accelerated(&self) -> &bool {
        &self.integer_dot_product64_bit_signed_accelerated
    }
    ///Get a reference to the `integer_dot_product64_bit_mixed_signedness_accelerated` field.
    pub fn integer_dot_product64_bit_mixed_signedness_accelerated(&self) -> &bool {
        &self.integer_dot_product64_bit_mixed_signedness_accelerated
    }
    ///Get a reference to the
    /// `integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated` field.
    pub fn integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated(&self) -> &bool {
        &self.integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated
    }
    ///Get a reference to the `integer_dot_product_accumulating_saturating8_bit_signed_accelerated`
    /// field.
    pub fn integer_dot_product_accumulating_saturating8_bit_signed_accelerated(&self) -> &bool {
        &self.integer_dot_product_accumulating_saturating8_bit_signed_accelerated
    }
    ///Get a reference to the
    /// `integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated` field.
    pub fn integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated(&self) -> &bool {
        &self.integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated
    }
    ///Get a reference to the
    /// `integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated` field.
    pub fn integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated(&self) -> &bool {
        &self.integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated
    }
    ///Get a reference to the
    /// `integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated` field.
    pub fn integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated(&self) -> &bool {
        &self.integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated
    }
    ///Get a reference to the
    /// `integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated`
    /// field.
    pub fn integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated(&self) -> &bool {
        &self.integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated
    }
    ///Get a reference to the
    /// `integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated` field.
    pub fn integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated(&self) -> &bool {
        &self.integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated
    }
    ///Get a reference to the
    /// `integer_dot_product_accumulating_saturating16_bit_signed_accelerated` field.
    pub fn integer_dot_product_accumulating_saturating16_bit_signed_accelerated(&self) -> &bool {
        &self.integer_dot_product_accumulating_saturating16_bit_signed_accelerated
    }
    ///Get a reference to the
    /// `integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated` field.
    pub fn integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated(&self) -> &bool {
        &self.integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated
    }
    ///Get a reference to the
    /// `integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated` field.
    pub fn integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated(&self) -> &bool {
        &self.integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated
    }
    ///Get a reference to the
    /// `integer_dot_product_accumulating_saturating32_bit_signed_accelerated` field.
    pub fn integer_dot_product_accumulating_saturating32_bit_signed_accelerated(&self) -> &bool {
        &self.integer_dot_product_accumulating_saturating32_bit_signed_accelerated
    }
    ///Get a reference to the
    /// `integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated` field.
    pub fn integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated(&self) -> &bool {
        &self.integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated
    }
    ///Get a reference to the
    /// `integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated` field.
    pub fn integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated(&self) -> &bool {
        &self.integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated
    }
    ///Get a reference to the
    /// `integer_dot_product_accumulating_saturating64_bit_signed_accelerated` field.
    pub fn integer_dot_product_accumulating_saturating64_bit_signed_accelerated(&self) -> &bool {
        &self.integer_dot_product_accumulating_saturating64_bit_signed_accelerated
    }
    ///Get a reference to the
    /// `integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated` field.
    pub fn integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated(&self) -> &bool {
        &self.integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated
    }
    ///Get a reference to the `storage_texel_buffer_offset_alignment_bytes` field.
    pub fn storage_texel_buffer_offset_alignment_bytes(&self) -> &DeviceSize {
        &self.storage_texel_buffer_offset_alignment_bytes
    }
    ///Get a reference to the `storage_texel_buffer_offset_single_texel_alignment` field.
    pub fn storage_texel_buffer_offset_single_texel_alignment(&self) -> &bool {
        &self.storage_texel_buffer_offset_single_texel_alignment
    }
    ///Get a reference to the `uniform_texel_buffer_offset_alignment_bytes` field.
    pub fn uniform_texel_buffer_offset_alignment_bytes(&self) -> &DeviceSize {
        &self.uniform_texel_buffer_offset_alignment_bytes
    }
    ///Get a reference to the `uniform_texel_buffer_offset_single_texel_alignment` field.
    pub fn uniform_texel_buffer_offset_single_texel_alignment(&self) -> &bool {
        &self.uniform_texel_buffer_offset_single_texel_alignment
    }
    ///Get a reference to the `max_buffer_size` field.
    pub fn max_buffer_size(&self) -> &DeviceSize {
        &self.max_buffer_size
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceVulkan13Properties {
    type LowLevel = crate::native::vulkan1_3::PhysicalDeviceVulkan13Properties;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::PhysicalDeviceVulkan13Properties {
            s_type: StructureType::PhysicalDeviceVulkan13Properties,
            p_next: std::ptr::null_mut(),
            min_subgroup_size: self.min_subgroup_size.into_low_level(context, bump),
            max_subgroup_size: self.max_subgroup_size.into_low_level(context, bump),
            max_compute_workgroup_subgroups: self.max_compute_workgroup_subgroups.into_low_level(context, bump),
            required_subgroup_size_stages: self.required_subgroup_size_stages.into_low_level(context, bump),
            max_inline_uniform_block_size: self.max_inline_uniform_block_size.into_low_level(context, bump),
            max_per_stage_descriptor_inline_uniform_blocks: self
                .max_per_stage_descriptor_inline_uniform_blocks
                .into_low_level(context, bump),
            max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: self
                .max_per_stage_descriptor_update_after_bind_inline_uniform_blocks
                .into_low_level(context, bump),
            max_descriptor_set_inline_uniform_blocks: self
                .max_descriptor_set_inline_uniform_blocks
                .into_low_level(context, bump),
            max_descriptor_set_update_after_bind_inline_uniform_blocks: self
                .max_descriptor_set_update_after_bind_inline_uniform_blocks
                .into_low_level(context, bump),
            max_inline_uniform_total_size: self.max_inline_uniform_total_size.into_low_level(context, bump),
            integer_dot_product8_bit_unsigned_accelerated: self
                .integer_dot_product8_bit_unsigned_accelerated
                .into_low_level(context, bump),
            integer_dot_product8_bit_signed_accelerated: self
                .integer_dot_product8_bit_signed_accelerated
                .into_low_level(context, bump),
            integer_dot_product8_bit_mixed_signedness_accelerated: self
                .integer_dot_product8_bit_mixed_signedness_accelerated
                .into_low_level(context, bump),
            integer_dot_product4x8_bit_packed_unsigned_accelerated: self
                .integer_dot_product4x8_bit_packed_unsigned_accelerated
                .into_low_level(context, bump),
            integer_dot_product4x8_bit_packed_signed_accelerated: self
                .integer_dot_product4x8_bit_packed_signed_accelerated
                .into_low_level(context, bump),
            integer_dot_product4x8_bit_packed_mixed_signedness_accelerated: self
                .integer_dot_product4x8_bit_packed_mixed_signedness_accelerated
                .into_low_level(context, bump),
            integer_dot_product16_bit_unsigned_accelerated: self
                .integer_dot_product16_bit_unsigned_accelerated
                .into_low_level(context, bump),
            integer_dot_product16_bit_signed_accelerated: self
                .integer_dot_product16_bit_signed_accelerated
                .into_low_level(context, bump),
            integer_dot_product16_bit_mixed_signedness_accelerated: self
                .integer_dot_product16_bit_mixed_signedness_accelerated
                .into_low_level(context, bump),
            integer_dot_product32_bit_unsigned_accelerated: self
                .integer_dot_product32_bit_unsigned_accelerated
                .into_low_level(context, bump),
            integer_dot_product32_bit_signed_accelerated: self
                .integer_dot_product32_bit_signed_accelerated
                .into_low_level(context, bump),
            integer_dot_product32_bit_mixed_signedness_accelerated: self
                .integer_dot_product32_bit_mixed_signedness_accelerated
                .into_low_level(context, bump),
            integer_dot_product64_bit_unsigned_accelerated: self
                .integer_dot_product64_bit_unsigned_accelerated
                .into_low_level(context, bump),
            integer_dot_product64_bit_signed_accelerated: self
                .integer_dot_product64_bit_signed_accelerated
                .into_low_level(context, bump),
            integer_dot_product64_bit_mixed_signedness_accelerated: self
                .integer_dot_product64_bit_mixed_signedness_accelerated
                .into_low_level(context, bump),
            integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated: self
                .integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated
                .into_low_level(context, bump),
            integer_dot_product_accumulating_saturating8_bit_signed_accelerated: self
                .integer_dot_product_accumulating_saturating8_bit_signed_accelerated
                .into_low_level(context, bump),
            integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated: self
                .integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated
                .into_low_level(context, bump),
            integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated: self
                .integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated
                .into_low_level(context, bump),
            integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated: self
                .integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated
                .into_low_level(context, bump),
            integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated: self
                .integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated
                .into_low_level(context, bump),
            integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated: self
                .integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated
                .into_low_level(context, bump),
            integer_dot_product_accumulating_saturating16_bit_signed_accelerated: self
                .integer_dot_product_accumulating_saturating16_bit_signed_accelerated
                .into_low_level(context, bump),
            integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated: self
                .integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated
                .into_low_level(context, bump),
            integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated: self
                .integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated
                .into_low_level(context, bump),
            integer_dot_product_accumulating_saturating32_bit_signed_accelerated: self
                .integer_dot_product_accumulating_saturating32_bit_signed_accelerated
                .into_low_level(context, bump),
            integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated: self
                .integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated
                .into_low_level(context, bump),
            integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated: self
                .integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated
                .into_low_level(context, bump),
            integer_dot_product_accumulating_saturating64_bit_signed_accelerated: self
                .integer_dot_product_accumulating_saturating64_bit_signed_accelerated
                .into_low_level(context, bump),
            integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated: self
                .integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated
                .into_low_level(context, bump),
            storage_texel_buffer_offset_alignment_bytes: self
                .storage_texel_buffer_offset_alignment_bytes
                .into_low_level(context, bump),
            storage_texel_buffer_offset_single_texel_alignment: self
                .storage_texel_buffer_offset_single_texel_alignment
                .into_low_level(context, bump),
            uniform_texel_buffer_offset_alignment_bytes: self
                .uniform_texel_buffer_offset_alignment_bytes
                .into_low_level(context, bump),
            uniform_texel_buffer_offset_single_texel_alignment: self
                .uniform_texel_buffer_offset_single_texel_alignment
                .into_low_level(context, bump),
            max_buffer_size: self.max_buffer_size.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceVulkan13Properties {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            min_subgroup_size: crate::conv::FromLowLevel::from_low_level(context, value.min_subgroup_size),
            max_subgroup_size: crate::conv::FromLowLevel::from_low_level(context, value.max_subgroup_size),
            max_compute_workgroup_subgroups: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_compute_workgroup_subgroups,
            ),
            required_subgroup_size_stages: crate::conv::FromLowLevel::from_low_level(
                context,
                value.required_subgroup_size_stages,
            ),
            max_inline_uniform_block_size: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_inline_uniform_block_size,
            ),
            max_per_stage_descriptor_inline_uniform_blocks: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_per_stage_descriptor_inline_uniform_blocks,
            ),
            max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_per_stage_descriptor_update_after_bind_inline_uniform_blocks,
            ),
            max_descriptor_set_inline_uniform_blocks: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_descriptor_set_inline_uniform_blocks,
            ),
            max_descriptor_set_update_after_bind_inline_uniform_blocks: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_descriptor_set_update_after_bind_inline_uniform_blocks,
            ),
            max_inline_uniform_total_size: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_inline_uniform_total_size,
            ),
            integer_dot_product8_bit_unsigned_accelerated: crate::conv::FromLowLevel::from_low_level(
                context,
                value.integer_dot_product8_bit_unsigned_accelerated,
            ),
            integer_dot_product8_bit_signed_accelerated: crate::conv::FromLowLevel::from_low_level(
                context,
                value.integer_dot_product8_bit_signed_accelerated,
            ),
            integer_dot_product8_bit_mixed_signedness_accelerated: crate::conv::FromLowLevel::from_low_level(
                context,
                value.integer_dot_product8_bit_mixed_signedness_accelerated,
            ),
            integer_dot_product4x8_bit_packed_unsigned_accelerated: crate::conv::FromLowLevel::from_low_level(
                context,
                value.integer_dot_product4x8_bit_packed_unsigned_accelerated,
            ),
            integer_dot_product4x8_bit_packed_signed_accelerated: crate::conv::FromLowLevel::from_low_level(
                context,
                value.integer_dot_product4x8_bit_packed_signed_accelerated,
            ),
            integer_dot_product4x8_bit_packed_mixed_signedness_accelerated: crate::conv::FromLowLevel::from_low_level(
                context,
                value.integer_dot_product4x8_bit_packed_mixed_signedness_accelerated,
            ),
            integer_dot_product16_bit_unsigned_accelerated: crate::conv::FromLowLevel::from_low_level(
                context,
                value.integer_dot_product16_bit_unsigned_accelerated,
            ),
            integer_dot_product16_bit_signed_accelerated: crate::conv::FromLowLevel::from_low_level(
                context,
                value.integer_dot_product16_bit_signed_accelerated,
            ),
            integer_dot_product16_bit_mixed_signedness_accelerated: crate::conv::FromLowLevel::from_low_level(
                context,
                value.integer_dot_product16_bit_mixed_signedness_accelerated,
            ),
            integer_dot_product32_bit_unsigned_accelerated: crate::conv::FromLowLevel::from_low_level(
                context,
                value.integer_dot_product32_bit_unsigned_accelerated,
            ),
            integer_dot_product32_bit_signed_accelerated: crate::conv::FromLowLevel::from_low_level(
                context,
                value.integer_dot_product32_bit_signed_accelerated,
            ),
            integer_dot_product32_bit_mixed_signedness_accelerated: crate::conv::FromLowLevel::from_low_level(
                context,
                value.integer_dot_product32_bit_mixed_signedness_accelerated,
            ),
            integer_dot_product64_bit_unsigned_accelerated: crate::conv::FromLowLevel::from_low_level(
                context,
                value.integer_dot_product64_bit_unsigned_accelerated,
            ),
            integer_dot_product64_bit_signed_accelerated: crate::conv::FromLowLevel::from_low_level(
                context,
                value.integer_dot_product64_bit_signed_accelerated,
            ),
            integer_dot_product64_bit_mixed_signedness_accelerated: crate::conv::FromLowLevel::from_low_level(
                context,
                value.integer_dot_product64_bit_mixed_signedness_accelerated,
            ),
            integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated:
                crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated,
                ),
            integer_dot_product_accumulating_saturating8_bit_signed_accelerated:
                crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.integer_dot_product_accumulating_saturating8_bit_signed_accelerated,
                ),
            integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated:
                crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated,
                ),
            integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated:
                crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated,
                ),
            integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated:
                crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated,
                ),
            integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated:
                crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated,
                ),
            integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated:
                crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated,
                ),
            integer_dot_product_accumulating_saturating16_bit_signed_accelerated:
                crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.integer_dot_product_accumulating_saturating16_bit_signed_accelerated,
                ),
            integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated:
                crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated,
                ),
            integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated:
                crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated,
                ),
            integer_dot_product_accumulating_saturating32_bit_signed_accelerated:
                crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.integer_dot_product_accumulating_saturating32_bit_signed_accelerated,
                ),
            integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated:
                crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated,
                ),
            integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated:
                crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated,
                ),
            integer_dot_product_accumulating_saturating64_bit_signed_accelerated:
                crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.integer_dot_product_accumulating_saturating64_bit_signed_accelerated,
                ),
            integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated:
                crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated,
                ),
            storage_texel_buffer_offset_alignment_bytes: crate::conv::FromLowLevel::from_low_level(
                context,
                value.storage_texel_buffer_offset_alignment_bytes,
            ),
            storage_texel_buffer_offset_single_texel_alignment: crate::conv::FromLowLevel::from_low_level(
                context,
                value.storage_texel_buffer_offset_single_texel_alignment,
            ),
            uniform_texel_buffer_offset_alignment_bytes: crate::conv::FromLowLevel::from_low_level(
                context,
                value.uniform_texel_buffer_offset_alignment_bytes,
            ),
            uniform_texel_buffer_offset_single_texel_alignment: crate::conv::FromLowLevel::from_low_level(
                context,
                value.uniform_texel_buffer_offset_single_texel_alignment,
            ),
            max_buffer_size: crate::conv::FromLowLevel::from_low_level(context, value.max_buffer_size),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceToolProperties")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceToolProperties {
    pub name: String,
    pub version: String,
    pub purposes: ToolPurposeFlags,
    pub description: String,
    pub layer: String,
}
impl PhysicalDeviceToolProperties {
    ///Get a reference to the `name` field.
    pub fn name(&self) -> &String {
        &self.name
    }
    ///Get a reference to the `version` field.
    pub fn version(&self) -> &String {
        &self.version
    }
    ///Get a reference to the `purposes` field.
    pub fn purposes(&self) -> ToolPurposeFlags {
        self.purposes
    }
    ///Get a reference to the `description` field.
    pub fn description(&self) -> &String {
        &self.description
    }
    ///Get a reference to the `layer` field.
    pub fn layer(&self) -> &String {
        &self.layer
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceToolProperties {
    type LowLevel = crate::native::vulkan1_3::PhysicalDeviceToolProperties;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let name_bytes = self.name.as_bytes();
        debug_assert!(
            memchr::memchr(0, name_bytes).is_none(),
            "string array contains null characters"
        );
        debug_assert!(
            name_bytes.len() <= MAX_EXTENSION_NAME_SIZE as usize,
            "string is too long for the backing array"
        );
        let mut name: [std::ffi::c_char; MAX_EXTENSION_NAME_SIZE as usize] = [0; MAX_EXTENSION_NAME_SIZE as usize];
        name[0..name_bytes.len()].copy_from_slice(std::slice::from_raw_parts(
            name_bytes.as_ptr() as *const std::ffi::c_char,
            name_bytes.len(),
        ));
        let version_bytes = self.version.as_bytes();
        debug_assert!(
            memchr::memchr(0, version_bytes).is_none(),
            "string array contains null characters"
        );
        debug_assert!(
            version_bytes.len() <= MAX_EXTENSION_NAME_SIZE as usize,
            "string is too long for the backing array"
        );
        let mut version: [std::ffi::c_char; MAX_EXTENSION_NAME_SIZE as usize] = [0; MAX_EXTENSION_NAME_SIZE as usize];
        version[0..version_bytes.len()].copy_from_slice(std::slice::from_raw_parts(
            version_bytes.as_ptr() as *const std::ffi::c_char,
            version_bytes.len(),
        ));
        let description_bytes = self.description.as_bytes();
        debug_assert!(
            memchr::memchr(0, description_bytes).is_none(),
            "string array contains null characters"
        );
        debug_assert!(
            description_bytes.len() <= MAX_DESCRIPTION_SIZE as usize,
            "string is too long for the backing array"
        );
        let mut description: [std::ffi::c_char; MAX_DESCRIPTION_SIZE as usize] = [0; MAX_DESCRIPTION_SIZE as usize];
        description[0..description_bytes.len()].copy_from_slice(std::slice::from_raw_parts(
            description_bytes.as_ptr() as *const std::ffi::c_char,
            description_bytes.len(),
        ));
        let layer_bytes = self.layer.as_bytes();
        debug_assert!(
            memchr::memchr(0, layer_bytes).is_none(),
            "string array contains null characters"
        );
        debug_assert!(
            layer_bytes.len() <= MAX_EXTENSION_NAME_SIZE as usize,
            "string is too long for the backing array"
        );
        let mut layer: [std::ffi::c_char; MAX_EXTENSION_NAME_SIZE as usize] = [0; MAX_EXTENSION_NAME_SIZE as usize];
        layer[0..layer_bytes.len()].copy_from_slice(std::slice::from_raw_parts(
            layer_bytes.as_ptr() as *const std::ffi::c_char,
            layer_bytes.len(),
        ));
        crate::native::vulkan1_3::PhysicalDeviceToolProperties {
            s_type: StructureType::PhysicalDeviceToolProperties,
            p_next: std::ptr::null_mut(),
            name: name,
            version: version,
            purposes: self.purposes.into_low_level(context, bump),
            description: description,
            layer: layer,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceToolProperties {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let name_cstr = std::ffi::CStr::from_ptr(value.name.as_ptr());
        let name = name_cstr.to_string_lossy().into_owned();
        let version_cstr = std::ffi::CStr::from_ptr(value.version.as_ptr());
        let version = version_cstr.to_string_lossy().into_owned();
        let description_cstr = std::ffi::CStr::from_ptr(value.description.as_ptr());
        let description = description_cstr.to_string_lossy().into_owned();
        let layer_cstr = std::ffi::CStr::from_ptr(value.layer.as_ptr());
        let layer = layer_cstr.to_string_lossy().into_owned();
        Self {
            name: name,
            version: version,
            purposes: crate::conv::FromLowLevel::from_low_level(context, value.purposes),
            description: description,
            layer: layer,
        }
    }
}
#[doc(alias = "VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {
    #[doc(alias = "shaderZeroInitializeWorkgroupMemory")]
    pub shader_zero_initialize_workgroup_memory: bool,
}
impl PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {
    ///Get a reference to the `shader_zero_initialize_workgroup_memory` field.
    pub fn shader_zero_initialize_workgroup_memory(&self) -> &bool {
        &self.shader_zero_initialize_workgroup_memory
    }
    ///Get a mutable reference to the `shader_zero_initialize_workgroup_memory` field.
    pub fn shader_zero_initialize_workgroup_memory_mut(&mut self) -> &mut bool {
        &mut self.shader_zero_initialize_workgroup_memory
    }
    ///Sets the `shader_zero_initialize_workgroup_memory` field.
    pub fn set_shader_zero_initialize_workgroup_memory(
        &mut self,
        shader_zero_initialize_workgroup_memory: bool,
    ) -> &mut Self {
        self.shader_zero_initialize_workgroup_memory = shader_zero_initialize_workgroup_memory;
        self
    }
    ///Sets the `shader_zero_initialize_workgroup_memory` field in a builder way.
    pub fn with_shader_zero_initialize_workgroup_memory(
        mut self,
        shader_zero_initialize_workgroup_memory: bool,
    ) -> Self {
        self.shader_zero_initialize_workgroup_memory = shader_zero_initialize_workgroup_memory;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {
    type LowLevel = crate::native::vulkan1_3::PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {
            s_type: StructureType::PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures,
            p_next: std::ptr::null_mut(),
            shader_zero_initialize_workgroup_memory: self
                .shader_zero_initialize_workgroup_memory
                .into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            shader_zero_initialize_workgroup_memory: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_zero_initialize_workgroup_memory,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceImageRobustnessFeatures")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceImageRobustnessFeatures {
    #[doc(alias = "robustImageAccess")]
    pub robust_image_access: bool,
}
impl PhysicalDeviceImageRobustnessFeatures {
    ///Get a reference to the `robust_image_access` field.
    pub fn robust_image_access(&self) -> &bool {
        &self.robust_image_access
    }
    ///Get a mutable reference to the `robust_image_access` field.
    pub fn robust_image_access_mut(&mut self) -> &mut bool {
        &mut self.robust_image_access
    }
    ///Sets the `robust_image_access` field.
    pub fn set_robust_image_access(&mut self, robust_image_access: bool) -> &mut Self {
        self.robust_image_access = robust_image_access;
        self
    }
    ///Sets the `robust_image_access` field in a builder way.
    pub fn with_robust_image_access(mut self, robust_image_access: bool) -> Self {
        self.robust_image_access = robust_image_access;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceImageRobustnessFeatures {
    type LowLevel = crate::native::vulkan1_3::PhysicalDeviceImageRobustnessFeatures;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::PhysicalDeviceImageRobustnessFeatures {
            s_type: StructureType::PhysicalDeviceImageRobustnessFeatures,
            p_next: std::ptr::null_mut(),
            robust_image_access: self.robust_image_access.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceImageRobustnessFeatures {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            robust_image_access: crate::conv::FromLowLevel::from_low_level(context, value.robust_image_access),
        }
    }
}
#[doc(alias = "VkBufferCopy2")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BufferCopy2 {
    #[doc(alias = "srcOffset")]
    pub src_offset: DeviceSize,
    #[doc(alias = "dstOffset")]
    pub dst_offset: DeviceSize,
    pub size: DeviceSize,
}
impl BufferCopy2 {
    ///Get a reference to the `src_offset` field.
    pub fn src_offset(&self) -> &DeviceSize {
        &self.src_offset
    }
    ///Get a reference to the `dst_offset` field.
    pub fn dst_offset(&self) -> &DeviceSize {
        &self.dst_offset
    }
    ///Get a reference to the `size` field.
    pub fn size(&self) -> &DeviceSize {
        &self.size
    }
    ///Get a mutable reference to the `src_offset` field.
    pub fn src_offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.src_offset
    }
    ///Get a mutable reference to the `dst_offset` field.
    pub fn dst_offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.dst_offset
    }
    ///Get a mutable reference to the `size` field.
    pub fn size_mut(&mut self) -> &mut DeviceSize {
        &mut self.size
    }
    ///Sets the `src_offset` field.
    pub fn set_src_offset(&mut self, src_offset: DeviceSize) -> &mut Self {
        self.src_offset = src_offset;
        self
    }
    ///Sets the `dst_offset` field.
    pub fn set_dst_offset(&mut self, dst_offset: DeviceSize) -> &mut Self {
        self.dst_offset = dst_offset;
        self
    }
    ///Sets the `size` field.
    pub fn set_size(&mut self, size: DeviceSize) -> &mut Self {
        self.size = size;
        self
    }
    ///Sets the `src_offset` field in a builder way.
    pub fn with_src_offset(mut self, src_offset: DeviceSize) -> Self {
        self.src_offset = src_offset;
        self
    }
    ///Sets the `dst_offset` field in a builder way.
    pub fn with_dst_offset(mut self, dst_offset: DeviceSize) -> Self {
        self.dst_offset = dst_offset;
        self
    }
    ///Sets the `size` field in a builder way.
    pub fn with_size(mut self, size: DeviceSize) -> Self {
        self.size = size;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for BufferCopy2 {
    type LowLevel = crate::native::vulkan1_3::BufferCopy2;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::BufferCopy2 {
            s_type: StructureType::BufferCopy2,
            p_next: std::ptr::null(),
            src_offset: self.src_offset.into_low_level(context, bump),
            dst_offset: self.dst_offset.into_low_level(context, bump),
            size: self.size.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for BufferCopy2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            src_offset: crate::conv::FromLowLevel::from_low_level(context, value.src_offset),
            dst_offset: crate::conv::FromLowLevel::from_low_level(context, value.dst_offset),
            size: crate::conv::FromLowLevel::from_low_level(context, value.size),
        }
    }
}
#[doc(alias = "VkImageCopy2")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ImageCopy2 {
    #[doc(alias = "srcSubresource")]
    pub src_subresource: ImageSubresourceLayers,
    #[doc(alias = "srcOffset")]
    pub src_offset: Offset3D,
    #[doc(alias = "dstSubresource")]
    pub dst_subresource: ImageSubresourceLayers,
    #[doc(alias = "dstOffset")]
    pub dst_offset: Offset3D,
    pub extent: Extent3D,
}
impl ImageCopy2 {
    ///Get a reference to the `src_subresource` field.
    pub fn src_subresource(&self) -> ImageSubresourceLayers {
        self.src_subresource
    }
    ///Get a reference to the `src_offset` field.
    pub fn src_offset(&self) -> Offset3D {
        self.src_offset
    }
    ///Get a reference to the `dst_subresource` field.
    pub fn dst_subresource(&self) -> ImageSubresourceLayers {
        self.dst_subresource
    }
    ///Get a reference to the `dst_offset` field.
    pub fn dst_offset(&self) -> Offset3D {
        self.dst_offset
    }
    ///Get a reference to the `extent` field.
    pub fn extent(&self) -> Extent3D {
        self.extent
    }
    ///Get a mutable reference to the `src_subresource` field.
    pub fn src_subresource_mut(&mut self) -> &mut ImageSubresourceLayers {
        &mut self.src_subresource
    }
    ///Get a mutable reference to the `src_offset` field.
    pub fn src_offset_mut(&mut self) -> &mut Offset3D {
        &mut self.src_offset
    }
    ///Get a mutable reference to the `dst_subresource` field.
    pub fn dst_subresource_mut(&mut self) -> &mut ImageSubresourceLayers {
        &mut self.dst_subresource
    }
    ///Get a mutable reference to the `dst_offset` field.
    pub fn dst_offset_mut(&mut self) -> &mut Offset3D {
        &mut self.dst_offset
    }
    ///Get a mutable reference to the `extent` field.
    pub fn extent_mut(&mut self) -> &mut Extent3D {
        &mut self.extent
    }
    ///Sets the `src_subresource` field.
    pub fn set_src_subresource(&mut self, src_subresource: ImageSubresourceLayers) -> &mut Self {
        self.src_subresource = src_subresource;
        self
    }
    ///Sets the `src_offset` field.
    pub fn set_src_offset(&mut self, src_offset: Offset3D) -> &mut Self {
        self.src_offset = src_offset;
        self
    }
    ///Sets the `dst_subresource` field.
    pub fn set_dst_subresource(&mut self, dst_subresource: ImageSubresourceLayers) -> &mut Self {
        self.dst_subresource = dst_subresource;
        self
    }
    ///Sets the `dst_offset` field.
    pub fn set_dst_offset(&mut self, dst_offset: Offset3D) -> &mut Self {
        self.dst_offset = dst_offset;
        self
    }
    ///Sets the `extent` field.
    pub fn set_extent(&mut self, extent: Extent3D) -> &mut Self {
        self.extent = extent;
        self
    }
    ///Sets the `src_subresource` field in a builder way.
    pub fn with_src_subresource(mut self, src_subresource: ImageSubresourceLayers) -> Self {
        self.src_subresource = src_subresource;
        self
    }
    ///Sets the `src_offset` field in a builder way.
    pub fn with_src_offset(mut self, src_offset: Offset3D) -> Self {
        self.src_offset = src_offset;
        self
    }
    ///Sets the `dst_subresource` field in a builder way.
    pub fn with_dst_subresource(mut self, dst_subresource: ImageSubresourceLayers) -> Self {
        self.dst_subresource = dst_subresource;
        self
    }
    ///Sets the `dst_offset` field in a builder way.
    pub fn with_dst_offset(mut self, dst_offset: Offset3D) -> Self {
        self.dst_offset = dst_offset;
        self
    }
    ///Sets the `extent` field in a builder way.
    pub fn with_extent(mut self, extent: Extent3D) -> Self {
        self.extent = extent;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ImageCopy2 {
    type LowLevel = crate::native::vulkan1_3::ImageCopy2;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::ImageCopy2 {
            s_type: StructureType::ImageCopy2,
            p_next: std::ptr::null(),
            src_subresource: self.src_subresource.into_low_level(context, bump),
            src_offset: self.src_offset.into_low_level(context, bump),
            dst_subresource: self.dst_subresource.into_low_level(context, bump),
            dst_offset: self.dst_offset.into_low_level(context, bump),
            extent: self.extent.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ImageCopy2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            src_subresource: crate::conv::FromLowLevel::from_low_level(context, value.src_subresource),
            src_offset: crate::conv::FromLowLevel::from_low_level(context, value.src_offset),
            dst_subresource: crate::conv::FromLowLevel::from_low_level(context, value.dst_subresource),
            dst_offset: crate::conv::FromLowLevel::from_low_level(context, value.dst_offset),
            extent: crate::conv::FromLowLevel::from_low_level(context, value.extent),
        }
    }
}
#[doc(alias = "VkImageBlit2")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ImageBlit2 {
    #[doc(alias = "srcSubresource")]
    pub src_subresource: ImageSubresourceLayers,
    #[doc(alias = "srcOffsets")]
    pub src_offsets: [Offset3D; 2 as usize],
    #[doc(alias = "dstSubresource")]
    pub dst_subresource: ImageSubresourceLayers,
    #[doc(alias = "dstOffsets")]
    pub dst_offsets: [Offset3D; 2 as usize],
}
impl ImageBlit2 {
    ///Get a reference to the `src_subresource` field.
    pub fn src_subresource(&self) -> ImageSubresourceLayers {
        self.src_subresource
    }
    ///Get a reference to the `src_offsets` field.
    pub fn src_offsets(&self) -> [Offset3D; 2 as usize] {
        self.src_offsets
    }
    ///Get a reference to the `dst_subresource` field.
    pub fn dst_subresource(&self) -> ImageSubresourceLayers {
        self.dst_subresource
    }
    ///Get a reference to the `dst_offsets` field.
    pub fn dst_offsets(&self) -> [Offset3D; 2 as usize] {
        self.dst_offsets
    }
    ///Get a mutable reference to the `src_subresource` field.
    pub fn src_subresource_mut(&mut self) -> &mut ImageSubresourceLayers {
        &mut self.src_subresource
    }
    ///Get a mutable reference to the `src_offsets` field.
    pub fn src_offsets_mut(&mut self) -> &mut [Offset3D; 2 as usize] {
        &mut self.src_offsets
    }
    ///Get a mutable reference to the `dst_subresource` field.
    pub fn dst_subresource_mut(&mut self) -> &mut ImageSubresourceLayers {
        &mut self.dst_subresource
    }
    ///Get a mutable reference to the `dst_offsets` field.
    pub fn dst_offsets_mut(&mut self) -> &mut [Offset3D; 2 as usize] {
        &mut self.dst_offsets
    }
    ///Sets the `src_subresource` field.
    pub fn set_src_subresource(&mut self, src_subresource: ImageSubresourceLayers) -> &mut Self {
        self.src_subresource = src_subresource;
        self
    }
    ///Sets the `src_offsets` field.
    pub fn set_src_offsets(&mut self, src_offsets: [Offset3D; 2 as usize]) -> &mut Self {
        self.src_offsets = src_offsets;
        self
    }
    ///Sets the `dst_subresource` field.
    pub fn set_dst_subresource(&mut self, dst_subresource: ImageSubresourceLayers) -> &mut Self {
        self.dst_subresource = dst_subresource;
        self
    }
    ///Sets the `dst_offsets` field.
    pub fn set_dst_offsets(&mut self, dst_offsets: [Offset3D; 2 as usize]) -> &mut Self {
        self.dst_offsets = dst_offsets;
        self
    }
    ///Sets the `src_subresource` field in a builder way.
    pub fn with_src_subresource(mut self, src_subresource: ImageSubresourceLayers) -> Self {
        self.src_subresource = src_subresource;
        self
    }
    ///Sets the `src_offsets` field in a builder way.
    pub fn with_src_offsets(mut self, src_offsets: [Offset3D; 2 as usize]) -> Self {
        self.src_offsets = src_offsets;
        self
    }
    ///Sets the `dst_subresource` field in a builder way.
    pub fn with_dst_subresource(mut self, dst_subresource: ImageSubresourceLayers) -> Self {
        self.dst_subresource = dst_subresource;
        self
    }
    ///Sets the `dst_offsets` field in a builder way.
    pub fn with_dst_offsets(mut self, dst_offsets: [Offset3D; 2 as usize]) -> Self {
        self.dst_offsets = dst_offsets;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ImageBlit2 {
    type LowLevel = crate::native::vulkan1_3::ImageBlit2;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::ImageBlit2 {
            s_type: StructureType::ImageBlit2,
            p_next: std::ptr::null(),
            src_subresource: self.src_subresource.into_low_level(context, bump),
            src_offsets: self.src_offsets.into_low_level(context, bump),
            dst_subresource: self.dst_subresource.into_low_level(context, bump),
            dst_offsets: self.dst_offsets.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ImageBlit2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            src_subresource: crate::conv::FromLowLevel::from_low_level(context, value.src_subresource),
            src_offsets: crate::conv::FromLowLevel::from_low_level(context, value.src_offsets),
            dst_subresource: crate::conv::FromLowLevel::from_low_level(context, value.dst_subresource),
            dst_offsets: crate::conv::FromLowLevel::from_low_level(context, value.dst_offsets),
        }
    }
}
#[doc(alias = "VkBufferImageCopy2")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BufferImageCopy2 {
    #[doc(alias = "bufferOffset")]
    pub buffer_offset: DeviceSize,
    #[doc(alias = "bufferRowLength")]
    pub buffer_row_length: u32,
    #[doc(alias = "bufferImageHeight")]
    pub buffer_image_height: u32,
    #[doc(alias = "imageSubresource")]
    pub image_subresource: ImageSubresourceLayers,
    #[doc(alias = "imageOffset")]
    pub image_offset: Offset3D,
    #[doc(alias = "imageExtent")]
    pub image_extent: Extent3D,
}
impl BufferImageCopy2 {
    ///Get a reference to the `buffer_offset` field.
    pub fn buffer_offset(&self) -> &DeviceSize {
        &self.buffer_offset
    }
    ///Get a reference to the `buffer_row_length` field.
    pub fn buffer_row_length(&self) -> u32 {
        self.buffer_row_length
    }
    ///Get a reference to the `buffer_image_height` field.
    pub fn buffer_image_height(&self) -> u32 {
        self.buffer_image_height
    }
    ///Get a reference to the `image_subresource` field.
    pub fn image_subresource(&self) -> ImageSubresourceLayers {
        self.image_subresource
    }
    ///Get a reference to the `image_offset` field.
    pub fn image_offset(&self) -> Offset3D {
        self.image_offset
    }
    ///Get a reference to the `image_extent` field.
    pub fn image_extent(&self) -> Extent3D {
        self.image_extent
    }
    ///Get a mutable reference to the `buffer_offset` field.
    pub fn buffer_offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.buffer_offset
    }
    ///Get a mutable reference to the `buffer_row_length` field.
    pub fn buffer_row_length_mut(&mut self) -> &mut u32 {
        &mut self.buffer_row_length
    }
    ///Get a mutable reference to the `buffer_image_height` field.
    pub fn buffer_image_height_mut(&mut self) -> &mut u32 {
        &mut self.buffer_image_height
    }
    ///Get a mutable reference to the `image_subresource` field.
    pub fn image_subresource_mut(&mut self) -> &mut ImageSubresourceLayers {
        &mut self.image_subresource
    }
    ///Get a mutable reference to the `image_offset` field.
    pub fn image_offset_mut(&mut self) -> &mut Offset3D {
        &mut self.image_offset
    }
    ///Get a mutable reference to the `image_extent` field.
    pub fn image_extent_mut(&mut self) -> &mut Extent3D {
        &mut self.image_extent
    }
    ///Sets the `buffer_offset` field.
    pub fn set_buffer_offset(&mut self, buffer_offset: DeviceSize) -> &mut Self {
        self.buffer_offset = buffer_offset;
        self
    }
    ///Sets the `buffer_row_length` field.
    pub fn set_buffer_row_length(&mut self, buffer_row_length: u32) -> &mut Self {
        self.buffer_row_length = buffer_row_length;
        self
    }
    ///Sets the `buffer_image_height` field.
    pub fn set_buffer_image_height(&mut self, buffer_image_height: u32) -> &mut Self {
        self.buffer_image_height = buffer_image_height;
        self
    }
    ///Sets the `image_subresource` field.
    pub fn set_image_subresource(&mut self, image_subresource: ImageSubresourceLayers) -> &mut Self {
        self.image_subresource = image_subresource;
        self
    }
    ///Sets the `image_offset` field.
    pub fn set_image_offset(&mut self, image_offset: Offset3D) -> &mut Self {
        self.image_offset = image_offset;
        self
    }
    ///Sets the `image_extent` field.
    pub fn set_image_extent(&mut self, image_extent: Extent3D) -> &mut Self {
        self.image_extent = image_extent;
        self
    }
    ///Sets the `buffer_offset` field in a builder way.
    pub fn with_buffer_offset(mut self, buffer_offset: DeviceSize) -> Self {
        self.buffer_offset = buffer_offset;
        self
    }
    ///Sets the `buffer_row_length` field in a builder way.
    pub fn with_buffer_row_length(mut self, buffer_row_length: u32) -> Self {
        self.buffer_row_length = buffer_row_length;
        self
    }
    ///Sets the `buffer_image_height` field in a builder way.
    pub fn with_buffer_image_height(mut self, buffer_image_height: u32) -> Self {
        self.buffer_image_height = buffer_image_height;
        self
    }
    ///Sets the `image_subresource` field in a builder way.
    pub fn with_image_subresource(mut self, image_subresource: ImageSubresourceLayers) -> Self {
        self.image_subresource = image_subresource;
        self
    }
    ///Sets the `image_offset` field in a builder way.
    pub fn with_image_offset(mut self, image_offset: Offset3D) -> Self {
        self.image_offset = image_offset;
        self
    }
    ///Sets the `image_extent` field in a builder way.
    pub fn with_image_extent(mut self, image_extent: Extent3D) -> Self {
        self.image_extent = image_extent;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for BufferImageCopy2 {
    type LowLevel = crate::native::vulkan1_3::BufferImageCopy2;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::BufferImageCopy2 {
            s_type: StructureType::BufferImageCopy2,
            p_next: std::ptr::null(),
            buffer_offset: self.buffer_offset.into_low_level(context, bump),
            buffer_row_length: self.buffer_row_length.into_low_level(context, bump),
            buffer_image_height: self.buffer_image_height.into_low_level(context, bump),
            image_subresource: self.image_subresource.into_low_level(context, bump),
            image_offset: self.image_offset.into_low_level(context, bump),
            image_extent: self.image_extent.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for BufferImageCopy2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            buffer_offset: crate::conv::FromLowLevel::from_low_level(context, value.buffer_offset),
            buffer_row_length: crate::conv::FromLowLevel::from_low_level(context, value.buffer_row_length),
            buffer_image_height: crate::conv::FromLowLevel::from_low_level(context, value.buffer_image_height),
            image_subresource: crate::conv::FromLowLevel::from_low_level(context, value.image_subresource),
            image_offset: crate::conv::FromLowLevel::from_low_level(context, value.image_offset),
            image_extent: crate::conv::FromLowLevel::from_low_level(context, value.image_extent),
        }
    }
}
#[doc(alias = "VkImageResolve2")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ImageResolve2 {
    #[doc(alias = "srcSubresource")]
    pub src_subresource: ImageSubresourceLayers,
    #[doc(alias = "srcOffset")]
    pub src_offset: Offset3D,
    #[doc(alias = "dstSubresource")]
    pub dst_subresource: ImageSubresourceLayers,
    #[doc(alias = "dstOffset")]
    pub dst_offset: Offset3D,
    pub extent: Extent3D,
}
impl ImageResolve2 {
    ///Get a reference to the `src_subresource` field.
    pub fn src_subresource(&self) -> ImageSubresourceLayers {
        self.src_subresource
    }
    ///Get a reference to the `src_offset` field.
    pub fn src_offset(&self) -> Offset3D {
        self.src_offset
    }
    ///Get a reference to the `dst_subresource` field.
    pub fn dst_subresource(&self) -> ImageSubresourceLayers {
        self.dst_subresource
    }
    ///Get a reference to the `dst_offset` field.
    pub fn dst_offset(&self) -> Offset3D {
        self.dst_offset
    }
    ///Get a reference to the `extent` field.
    pub fn extent(&self) -> Extent3D {
        self.extent
    }
    ///Get a mutable reference to the `src_subresource` field.
    pub fn src_subresource_mut(&mut self) -> &mut ImageSubresourceLayers {
        &mut self.src_subresource
    }
    ///Get a mutable reference to the `src_offset` field.
    pub fn src_offset_mut(&mut self) -> &mut Offset3D {
        &mut self.src_offset
    }
    ///Get a mutable reference to the `dst_subresource` field.
    pub fn dst_subresource_mut(&mut self) -> &mut ImageSubresourceLayers {
        &mut self.dst_subresource
    }
    ///Get a mutable reference to the `dst_offset` field.
    pub fn dst_offset_mut(&mut self) -> &mut Offset3D {
        &mut self.dst_offset
    }
    ///Get a mutable reference to the `extent` field.
    pub fn extent_mut(&mut self) -> &mut Extent3D {
        &mut self.extent
    }
    ///Sets the `src_subresource` field.
    pub fn set_src_subresource(&mut self, src_subresource: ImageSubresourceLayers) -> &mut Self {
        self.src_subresource = src_subresource;
        self
    }
    ///Sets the `src_offset` field.
    pub fn set_src_offset(&mut self, src_offset: Offset3D) -> &mut Self {
        self.src_offset = src_offset;
        self
    }
    ///Sets the `dst_subresource` field.
    pub fn set_dst_subresource(&mut self, dst_subresource: ImageSubresourceLayers) -> &mut Self {
        self.dst_subresource = dst_subresource;
        self
    }
    ///Sets the `dst_offset` field.
    pub fn set_dst_offset(&mut self, dst_offset: Offset3D) -> &mut Self {
        self.dst_offset = dst_offset;
        self
    }
    ///Sets the `extent` field.
    pub fn set_extent(&mut self, extent: Extent3D) -> &mut Self {
        self.extent = extent;
        self
    }
    ///Sets the `src_subresource` field in a builder way.
    pub fn with_src_subresource(mut self, src_subresource: ImageSubresourceLayers) -> Self {
        self.src_subresource = src_subresource;
        self
    }
    ///Sets the `src_offset` field in a builder way.
    pub fn with_src_offset(mut self, src_offset: Offset3D) -> Self {
        self.src_offset = src_offset;
        self
    }
    ///Sets the `dst_subresource` field in a builder way.
    pub fn with_dst_subresource(mut self, dst_subresource: ImageSubresourceLayers) -> Self {
        self.dst_subresource = dst_subresource;
        self
    }
    ///Sets the `dst_offset` field in a builder way.
    pub fn with_dst_offset(mut self, dst_offset: Offset3D) -> Self {
        self.dst_offset = dst_offset;
        self
    }
    ///Sets the `extent` field in a builder way.
    pub fn with_extent(mut self, extent: Extent3D) -> Self {
        self.extent = extent;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ImageResolve2 {
    type LowLevel = crate::native::vulkan1_3::ImageResolve2;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::ImageResolve2 {
            s_type: StructureType::ImageResolve2,
            p_next: std::ptr::null(),
            src_subresource: self.src_subresource.into_low_level(context, bump),
            src_offset: self.src_offset.into_low_level(context, bump),
            dst_subresource: self.dst_subresource.into_low_level(context, bump),
            dst_offset: self.dst_offset.into_low_level(context, bump),
            extent: self.extent.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ImageResolve2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            src_subresource: crate::conv::FromLowLevel::from_low_level(context, value.src_subresource),
            src_offset: crate::conv::FromLowLevel::from_low_level(context, value.src_offset),
            dst_subresource: crate::conv::FromLowLevel::from_low_level(context, value.dst_subresource),
            dst_offset: crate::conv::FromLowLevel::from_low_level(context, value.dst_offset),
            extent: crate::conv::FromLowLevel::from_low_level(context, value.extent),
        }
    }
}
#[doc(alias = "VkCopyBufferInfo2")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CopyBufferInfo2 {
    #[doc(alias = "srcBuffer")]
    pub src_buffer: Buffer,
    #[doc(alias = "dstBuffer")]
    pub dst_buffer: Buffer,
    #[doc(alias = "pRegions")]
    pub regions: SmallVec<[BufferCopy2; 8]>,
}
impl CopyBufferInfo2 {
    ///Get a reference to the `src_buffer` field.
    pub fn src_buffer(&self) -> &Buffer {
        &self.src_buffer
    }
    ///Get a reference to the `dst_buffer` field.
    pub fn dst_buffer(&self) -> &Buffer {
        &self.dst_buffer
    }
    ///Get a reference to the `regions` field.
    pub fn regions(&self) -> &SmallVec<[BufferCopy2; 8]> {
        &self.regions
    }
    ///Get a mutable reference to the `src_buffer` field.
    pub fn src_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.src_buffer
    }
    ///Get a mutable reference to the `dst_buffer` field.
    pub fn dst_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.dst_buffer
    }
    ///Get a mutable reference to the `regions` field.
    pub fn regions_mut(&mut self) -> &mut SmallVec<[BufferCopy2; 8]> {
        &mut self.regions
    }
    ///Sets the `src_buffer` field.
    pub fn set_src_buffer(&mut self, src_buffer: Buffer) -> &mut Self {
        self.src_buffer = src_buffer;
        self
    }
    ///Sets the `dst_buffer` field.
    pub fn set_dst_buffer(&mut self, dst_buffer: Buffer) -> &mut Self {
        self.dst_buffer = dst_buffer;
        self
    }
    ///Sets the `regions` field.
    pub fn set_regions(&mut self, regions: SmallVec<[BufferCopy2; 8]>) -> &mut Self {
        self.regions = regions;
        self
    }
    ///Sets the `src_buffer` field in a builder way.
    pub fn with_src_buffer(mut self, src_buffer: Buffer) -> Self {
        self.src_buffer = src_buffer;
        self
    }
    ///Sets the `dst_buffer` field in a builder way.
    pub fn with_dst_buffer(mut self, dst_buffer: Buffer) -> Self {
        self.dst_buffer = dst_buffer;
        self
    }
    ///Sets the `regions` field in a builder way.
    pub fn with_regions(mut self, regions: SmallVec<[BufferCopy2; 8]>) -> Self {
        self.regions = regions;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for CopyBufferInfo2 {
    type LowLevel = crate::native::vulkan1_3::CopyBufferInfo2;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_regions = self.regions.len() as u32;
        let regions = bump
            .alloc_slice_fill_iter(self.regions.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::vulkan1_3::CopyBufferInfo2 {
            s_type: StructureType::CopyBufferInfo2,
            p_next: std::ptr::null(),
            src_buffer: self.src_buffer.into_low_level(context, bump),
            dst_buffer: self.dst_buffer.into_low_level(context, bump),
            region_count: len_regions,
            regions: regions,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for CopyBufferInfo2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let regions_len = value.region_count;
        let mut regions = SmallVec::with_capacity(regions_len as usize);
        for i in 0..regions_len {
            regions.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.regions.add(i as usize).read(),
            ));
        }
        Self {
            src_buffer: crate::conv::FromLowLevel::from_low_level(context, value.src_buffer),
            dst_buffer: crate::conv::FromLowLevel::from_low_level(context, value.dst_buffer),
            regions: regions,
        }
    }
}
#[doc(alias = "VkCopyImageInfo2")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CopyImageInfo2 {
    #[doc(alias = "srcImage")]
    pub src_image: Image,
    #[doc(alias = "srcImageLayout")]
    pub src_image_layout: ImageLayout,
    #[doc(alias = "dstImage")]
    pub dst_image: Image,
    #[doc(alias = "dstImageLayout")]
    pub dst_image_layout: ImageLayout,
    #[doc(alias = "pRegions")]
    pub regions: SmallVec<[ImageCopy2; 8]>,
}
impl CopyImageInfo2 {
    ///Get a reference to the `src_image` field.
    pub fn src_image(&self) -> &Image {
        &self.src_image
    }
    ///Get a reference to the `src_image_layout` field.
    pub fn src_image_layout(&self) -> ImageLayout {
        self.src_image_layout
    }
    ///Get a reference to the `dst_image` field.
    pub fn dst_image(&self) -> &Image {
        &self.dst_image
    }
    ///Get a reference to the `dst_image_layout` field.
    pub fn dst_image_layout(&self) -> ImageLayout {
        self.dst_image_layout
    }
    ///Get a reference to the `regions` field.
    pub fn regions(&self) -> &SmallVec<[ImageCopy2; 8]> {
        &self.regions
    }
    ///Get a mutable reference to the `src_image` field.
    pub fn src_image_mut(&mut self) -> &mut Image {
        &mut self.src_image
    }
    ///Get a mutable reference to the `src_image_layout` field.
    pub fn src_image_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.src_image_layout
    }
    ///Get a mutable reference to the `dst_image` field.
    pub fn dst_image_mut(&mut self) -> &mut Image {
        &mut self.dst_image
    }
    ///Get a mutable reference to the `dst_image_layout` field.
    pub fn dst_image_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.dst_image_layout
    }
    ///Get a mutable reference to the `regions` field.
    pub fn regions_mut(&mut self) -> &mut SmallVec<[ImageCopy2; 8]> {
        &mut self.regions
    }
    ///Sets the `src_image` field.
    pub fn set_src_image(&mut self, src_image: Image) -> &mut Self {
        self.src_image = src_image;
        self
    }
    ///Sets the `src_image_layout` field.
    pub fn set_src_image_layout(&mut self, src_image_layout: ImageLayout) -> &mut Self {
        self.src_image_layout = src_image_layout;
        self
    }
    ///Sets the `dst_image` field.
    pub fn set_dst_image(&mut self, dst_image: Image) -> &mut Self {
        self.dst_image = dst_image;
        self
    }
    ///Sets the `dst_image_layout` field.
    pub fn set_dst_image_layout(&mut self, dst_image_layout: ImageLayout) -> &mut Self {
        self.dst_image_layout = dst_image_layout;
        self
    }
    ///Sets the `regions` field.
    pub fn set_regions(&mut self, regions: SmallVec<[ImageCopy2; 8]>) -> &mut Self {
        self.regions = regions;
        self
    }
    ///Sets the `src_image` field in a builder way.
    pub fn with_src_image(mut self, src_image: Image) -> Self {
        self.src_image = src_image;
        self
    }
    ///Sets the `src_image_layout` field in a builder way.
    pub fn with_src_image_layout(mut self, src_image_layout: ImageLayout) -> Self {
        self.src_image_layout = src_image_layout;
        self
    }
    ///Sets the `dst_image` field in a builder way.
    pub fn with_dst_image(mut self, dst_image: Image) -> Self {
        self.dst_image = dst_image;
        self
    }
    ///Sets the `dst_image_layout` field in a builder way.
    pub fn with_dst_image_layout(mut self, dst_image_layout: ImageLayout) -> Self {
        self.dst_image_layout = dst_image_layout;
        self
    }
    ///Sets the `regions` field in a builder way.
    pub fn with_regions(mut self, regions: SmallVec<[ImageCopy2; 8]>) -> Self {
        self.regions = regions;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for CopyImageInfo2 {
    type LowLevel = crate::native::vulkan1_3::CopyImageInfo2;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_regions = self.regions.len() as u32;
        let regions = bump
            .alloc_slice_fill_iter(self.regions.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::vulkan1_3::CopyImageInfo2 {
            s_type: StructureType::CopyImageInfo2,
            p_next: std::ptr::null(),
            src_image: self.src_image.into_low_level(context, bump),
            src_image_layout: self.src_image_layout.into_low_level(context, bump),
            dst_image: self.dst_image.into_low_level(context, bump),
            dst_image_layout: self.dst_image_layout.into_low_level(context, bump),
            region_count: len_regions,
            regions: regions,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for CopyImageInfo2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let regions_len = value.region_count;
        let mut regions = SmallVec::with_capacity(regions_len as usize);
        for i in 0..regions_len {
            regions.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.regions.add(i as usize).read(),
            ));
        }
        Self {
            src_image: crate::conv::FromLowLevel::from_low_level(context, value.src_image),
            src_image_layout: crate::conv::FromLowLevel::from_low_level(context, value.src_image_layout),
            dst_image: crate::conv::FromLowLevel::from_low_level(context, value.dst_image),
            dst_image_layout: crate::conv::FromLowLevel::from_low_level(context, value.dst_image_layout),
            regions: regions,
        }
    }
}
#[doc(alias = "VkBlitImageInfo2")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BlitImageInfo2 {
    #[doc(alias = "srcImage")]
    pub src_image: Image,
    #[doc(alias = "srcImageLayout")]
    pub src_image_layout: ImageLayout,
    #[doc(alias = "dstImage")]
    pub dst_image: Image,
    #[doc(alias = "dstImageLayout")]
    pub dst_image_layout: ImageLayout,
    #[doc(alias = "pRegions")]
    pub regions: SmallVec<[ImageBlit2; 8]>,
    pub filter: Filter,
}
impl BlitImageInfo2 {
    ///Get a reference to the `src_image` field.
    pub fn src_image(&self) -> &Image {
        &self.src_image
    }
    ///Get a reference to the `src_image_layout` field.
    pub fn src_image_layout(&self) -> ImageLayout {
        self.src_image_layout
    }
    ///Get a reference to the `dst_image` field.
    pub fn dst_image(&self) -> &Image {
        &self.dst_image
    }
    ///Get a reference to the `dst_image_layout` field.
    pub fn dst_image_layout(&self) -> ImageLayout {
        self.dst_image_layout
    }
    ///Get a reference to the `regions` field.
    pub fn regions(&self) -> &SmallVec<[ImageBlit2; 8]> {
        &self.regions
    }
    ///Get a reference to the `filter` field.
    pub fn filter(&self) -> Filter {
        self.filter
    }
    ///Get a mutable reference to the `src_image` field.
    pub fn src_image_mut(&mut self) -> &mut Image {
        &mut self.src_image
    }
    ///Get a mutable reference to the `src_image_layout` field.
    pub fn src_image_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.src_image_layout
    }
    ///Get a mutable reference to the `dst_image` field.
    pub fn dst_image_mut(&mut self) -> &mut Image {
        &mut self.dst_image
    }
    ///Get a mutable reference to the `dst_image_layout` field.
    pub fn dst_image_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.dst_image_layout
    }
    ///Get a mutable reference to the `regions` field.
    pub fn regions_mut(&mut self) -> &mut SmallVec<[ImageBlit2; 8]> {
        &mut self.regions
    }
    ///Get a mutable reference to the `filter` field.
    pub fn filter_mut(&mut self) -> &mut Filter {
        &mut self.filter
    }
    ///Sets the `src_image` field.
    pub fn set_src_image(&mut self, src_image: Image) -> &mut Self {
        self.src_image = src_image;
        self
    }
    ///Sets the `src_image_layout` field.
    pub fn set_src_image_layout(&mut self, src_image_layout: ImageLayout) -> &mut Self {
        self.src_image_layout = src_image_layout;
        self
    }
    ///Sets the `dst_image` field.
    pub fn set_dst_image(&mut self, dst_image: Image) -> &mut Self {
        self.dst_image = dst_image;
        self
    }
    ///Sets the `dst_image_layout` field.
    pub fn set_dst_image_layout(&mut self, dst_image_layout: ImageLayout) -> &mut Self {
        self.dst_image_layout = dst_image_layout;
        self
    }
    ///Sets the `regions` field.
    pub fn set_regions(&mut self, regions: SmallVec<[ImageBlit2; 8]>) -> &mut Self {
        self.regions = regions;
        self
    }
    ///Sets the `filter` field.
    pub fn set_filter(&mut self, filter: Filter) -> &mut Self {
        self.filter = filter;
        self
    }
    ///Sets the `src_image` field in a builder way.
    pub fn with_src_image(mut self, src_image: Image) -> Self {
        self.src_image = src_image;
        self
    }
    ///Sets the `src_image_layout` field in a builder way.
    pub fn with_src_image_layout(mut self, src_image_layout: ImageLayout) -> Self {
        self.src_image_layout = src_image_layout;
        self
    }
    ///Sets the `dst_image` field in a builder way.
    pub fn with_dst_image(mut self, dst_image: Image) -> Self {
        self.dst_image = dst_image;
        self
    }
    ///Sets the `dst_image_layout` field in a builder way.
    pub fn with_dst_image_layout(mut self, dst_image_layout: ImageLayout) -> Self {
        self.dst_image_layout = dst_image_layout;
        self
    }
    ///Sets the `regions` field in a builder way.
    pub fn with_regions(mut self, regions: SmallVec<[ImageBlit2; 8]>) -> Self {
        self.regions = regions;
        self
    }
    ///Sets the `filter` field in a builder way.
    pub fn with_filter(mut self, filter: Filter) -> Self {
        self.filter = filter;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for BlitImageInfo2 {
    type LowLevel = crate::native::vulkan1_3::BlitImageInfo2;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_regions = self.regions.len() as u32;
        let regions = bump
            .alloc_slice_fill_iter(self.regions.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::vulkan1_3::BlitImageInfo2 {
            s_type: StructureType::BlitImageInfo2,
            p_next: std::ptr::null(),
            src_image: self.src_image.into_low_level(context, bump),
            src_image_layout: self.src_image_layout.into_low_level(context, bump),
            dst_image: self.dst_image.into_low_level(context, bump),
            dst_image_layout: self.dst_image_layout.into_low_level(context, bump),
            region_count: len_regions,
            regions: regions,
            filter: self.filter.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for BlitImageInfo2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let regions_len = value.region_count;
        let mut regions = SmallVec::with_capacity(regions_len as usize);
        for i in 0..regions_len {
            regions.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.regions.add(i as usize).read(),
            ));
        }
        Self {
            src_image: crate::conv::FromLowLevel::from_low_level(context, value.src_image),
            src_image_layout: crate::conv::FromLowLevel::from_low_level(context, value.src_image_layout),
            dst_image: crate::conv::FromLowLevel::from_low_level(context, value.dst_image),
            dst_image_layout: crate::conv::FromLowLevel::from_low_level(context, value.dst_image_layout),
            regions: regions,
            filter: crate::conv::FromLowLevel::from_low_level(context, value.filter),
        }
    }
}
#[doc(alias = "VkCopyBufferToImageInfo2")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CopyBufferToImageInfo2 {
    #[doc(alias = "srcBuffer")]
    pub src_buffer: Buffer,
    #[doc(alias = "dstImage")]
    pub dst_image: Image,
    #[doc(alias = "dstImageLayout")]
    pub dst_image_layout: ImageLayout,
    #[doc(alias = "pRegions")]
    pub regions: SmallVec<[BufferImageCopy2; 8]>,
}
impl CopyBufferToImageInfo2 {
    ///Get a reference to the `src_buffer` field.
    pub fn src_buffer(&self) -> &Buffer {
        &self.src_buffer
    }
    ///Get a reference to the `dst_image` field.
    pub fn dst_image(&self) -> &Image {
        &self.dst_image
    }
    ///Get a reference to the `dst_image_layout` field.
    pub fn dst_image_layout(&self) -> ImageLayout {
        self.dst_image_layout
    }
    ///Get a reference to the `regions` field.
    pub fn regions(&self) -> &SmallVec<[BufferImageCopy2; 8]> {
        &self.regions
    }
    ///Get a mutable reference to the `src_buffer` field.
    pub fn src_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.src_buffer
    }
    ///Get a mutable reference to the `dst_image` field.
    pub fn dst_image_mut(&mut self) -> &mut Image {
        &mut self.dst_image
    }
    ///Get a mutable reference to the `dst_image_layout` field.
    pub fn dst_image_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.dst_image_layout
    }
    ///Get a mutable reference to the `regions` field.
    pub fn regions_mut(&mut self) -> &mut SmallVec<[BufferImageCopy2; 8]> {
        &mut self.regions
    }
    ///Sets the `src_buffer` field.
    pub fn set_src_buffer(&mut self, src_buffer: Buffer) -> &mut Self {
        self.src_buffer = src_buffer;
        self
    }
    ///Sets the `dst_image` field.
    pub fn set_dst_image(&mut self, dst_image: Image) -> &mut Self {
        self.dst_image = dst_image;
        self
    }
    ///Sets the `dst_image_layout` field.
    pub fn set_dst_image_layout(&mut self, dst_image_layout: ImageLayout) -> &mut Self {
        self.dst_image_layout = dst_image_layout;
        self
    }
    ///Sets the `regions` field.
    pub fn set_regions(&mut self, regions: SmallVec<[BufferImageCopy2; 8]>) -> &mut Self {
        self.regions = regions;
        self
    }
    ///Sets the `src_buffer` field in a builder way.
    pub fn with_src_buffer(mut self, src_buffer: Buffer) -> Self {
        self.src_buffer = src_buffer;
        self
    }
    ///Sets the `dst_image` field in a builder way.
    pub fn with_dst_image(mut self, dst_image: Image) -> Self {
        self.dst_image = dst_image;
        self
    }
    ///Sets the `dst_image_layout` field in a builder way.
    pub fn with_dst_image_layout(mut self, dst_image_layout: ImageLayout) -> Self {
        self.dst_image_layout = dst_image_layout;
        self
    }
    ///Sets the `regions` field in a builder way.
    pub fn with_regions(mut self, regions: SmallVec<[BufferImageCopy2; 8]>) -> Self {
        self.regions = regions;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for CopyBufferToImageInfo2 {
    type LowLevel = crate::native::vulkan1_3::CopyBufferToImageInfo2;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_regions = self.regions.len() as u32;
        let regions = bump
            .alloc_slice_fill_iter(self.regions.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::vulkan1_3::CopyBufferToImageInfo2 {
            s_type: StructureType::CopyBufferToImageInfo2,
            p_next: std::ptr::null(),
            src_buffer: self.src_buffer.into_low_level(context, bump),
            dst_image: self.dst_image.into_low_level(context, bump),
            dst_image_layout: self.dst_image_layout.into_low_level(context, bump),
            region_count: len_regions,
            regions: regions,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for CopyBufferToImageInfo2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let regions_len = value.region_count;
        let mut regions = SmallVec::with_capacity(regions_len as usize);
        for i in 0..regions_len {
            regions.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.regions.add(i as usize).read(),
            ));
        }
        Self {
            src_buffer: crate::conv::FromLowLevel::from_low_level(context, value.src_buffer),
            dst_image: crate::conv::FromLowLevel::from_low_level(context, value.dst_image),
            dst_image_layout: crate::conv::FromLowLevel::from_low_level(context, value.dst_image_layout),
            regions: regions,
        }
    }
}
#[doc(alias = "VkCopyImageToBufferInfo2")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CopyImageToBufferInfo2 {
    #[doc(alias = "srcImage")]
    pub src_image: Image,
    #[doc(alias = "srcImageLayout")]
    pub src_image_layout: ImageLayout,
    #[doc(alias = "dstBuffer")]
    pub dst_buffer: Buffer,
    #[doc(alias = "pRegions")]
    pub regions: SmallVec<[BufferImageCopy2; 8]>,
}
impl CopyImageToBufferInfo2 {
    ///Get a reference to the `src_image` field.
    pub fn src_image(&self) -> &Image {
        &self.src_image
    }
    ///Get a reference to the `src_image_layout` field.
    pub fn src_image_layout(&self) -> ImageLayout {
        self.src_image_layout
    }
    ///Get a reference to the `dst_buffer` field.
    pub fn dst_buffer(&self) -> &Buffer {
        &self.dst_buffer
    }
    ///Get a reference to the `regions` field.
    pub fn regions(&self) -> &SmallVec<[BufferImageCopy2; 8]> {
        &self.regions
    }
    ///Get a mutable reference to the `src_image` field.
    pub fn src_image_mut(&mut self) -> &mut Image {
        &mut self.src_image
    }
    ///Get a mutable reference to the `src_image_layout` field.
    pub fn src_image_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.src_image_layout
    }
    ///Get a mutable reference to the `dst_buffer` field.
    pub fn dst_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.dst_buffer
    }
    ///Get a mutable reference to the `regions` field.
    pub fn regions_mut(&mut self) -> &mut SmallVec<[BufferImageCopy2; 8]> {
        &mut self.regions
    }
    ///Sets the `src_image` field.
    pub fn set_src_image(&mut self, src_image: Image) -> &mut Self {
        self.src_image = src_image;
        self
    }
    ///Sets the `src_image_layout` field.
    pub fn set_src_image_layout(&mut self, src_image_layout: ImageLayout) -> &mut Self {
        self.src_image_layout = src_image_layout;
        self
    }
    ///Sets the `dst_buffer` field.
    pub fn set_dst_buffer(&mut self, dst_buffer: Buffer) -> &mut Self {
        self.dst_buffer = dst_buffer;
        self
    }
    ///Sets the `regions` field.
    pub fn set_regions(&mut self, regions: SmallVec<[BufferImageCopy2; 8]>) -> &mut Self {
        self.regions = regions;
        self
    }
    ///Sets the `src_image` field in a builder way.
    pub fn with_src_image(mut self, src_image: Image) -> Self {
        self.src_image = src_image;
        self
    }
    ///Sets the `src_image_layout` field in a builder way.
    pub fn with_src_image_layout(mut self, src_image_layout: ImageLayout) -> Self {
        self.src_image_layout = src_image_layout;
        self
    }
    ///Sets the `dst_buffer` field in a builder way.
    pub fn with_dst_buffer(mut self, dst_buffer: Buffer) -> Self {
        self.dst_buffer = dst_buffer;
        self
    }
    ///Sets the `regions` field in a builder way.
    pub fn with_regions(mut self, regions: SmallVec<[BufferImageCopy2; 8]>) -> Self {
        self.regions = regions;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for CopyImageToBufferInfo2 {
    type LowLevel = crate::native::vulkan1_3::CopyImageToBufferInfo2;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_regions = self.regions.len() as u32;
        let regions = bump
            .alloc_slice_fill_iter(self.regions.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::vulkan1_3::CopyImageToBufferInfo2 {
            s_type: StructureType::CopyImageToBufferInfo2,
            p_next: std::ptr::null(),
            src_image: self.src_image.into_low_level(context, bump),
            src_image_layout: self.src_image_layout.into_low_level(context, bump),
            dst_buffer: self.dst_buffer.into_low_level(context, bump),
            region_count: len_regions,
            regions: regions,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for CopyImageToBufferInfo2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let regions_len = value.region_count;
        let mut regions = SmallVec::with_capacity(regions_len as usize);
        for i in 0..regions_len {
            regions.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.regions.add(i as usize).read(),
            ));
        }
        Self {
            src_image: crate::conv::FromLowLevel::from_low_level(context, value.src_image),
            src_image_layout: crate::conv::FromLowLevel::from_low_level(context, value.src_image_layout),
            dst_buffer: crate::conv::FromLowLevel::from_low_level(context, value.dst_buffer),
            regions: regions,
        }
    }
}
#[doc(alias = "VkResolveImageInfo2")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ResolveImageInfo2 {
    #[doc(alias = "srcImage")]
    pub src_image: Image,
    #[doc(alias = "srcImageLayout")]
    pub src_image_layout: ImageLayout,
    #[doc(alias = "dstImage")]
    pub dst_image: Image,
    #[doc(alias = "dstImageLayout")]
    pub dst_image_layout: ImageLayout,
    #[doc(alias = "pRegions")]
    pub regions: SmallVec<[ImageResolve2; 8]>,
}
impl ResolveImageInfo2 {
    ///Get a reference to the `src_image` field.
    pub fn src_image(&self) -> &Image {
        &self.src_image
    }
    ///Get a reference to the `src_image_layout` field.
    pub fn src_image_layout(&self) -> ImageLayout {
        self.src_image_layout
    }
    ///Get a reference to the `dst_image` field.
    pub fn dst_image(&self) -> &Image {
        &self.dst_image
    }
    ///Get a reference to the `dst_image_layout` field.
    pub fn dst_image_layout(&self) -> ImageLayout {
        self.dst_image_layout
    }
    ///Get a reference to the `regions` field.
    pub fn regions(&self) -> &SmallVec<[ImageResolve2; 8]> {
        &self.regions
    }
    ///Get a mutable reference to the `src_image` field.
    pub fn src_image_mut(&mut self) -> &mut Image {
        &mut self.src_image
    }
    ///Get a mutable reference to the `src_image_layout` field.
    pub fn src_image_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.src_image_layout
    }
    ///Get a mutable reference to the `dst_image` field.
    pub fn dst_image_mut(&mut self) -> &mut Image {
        &mut self.dst_image
    }
    ///Get a mutable reference to the `dst_image_layout` field.
    pub fn dst_image_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.dst_image_layout
    }
    ///Get a mutable reference to the `regions` field.
    pub fn regions_mut(&mut self) -> &mut SmallVec<[ImageResolve2; 8]> {
        &mut self.regions
    }
    ///Sets the `src_image` field.
    pub fn set_src_image(&mut self, src_image: Image) -> &mut Self {
        self.src_image = src_image;
        self
    }
    ///Sets the `src_image_layout` field.
    pub fn set_src_image_layout(&mut self, src_image_layout: ImageLayout) -> &mut Self {
        self.src_image_layout = src_image_layout;
        self
    }
    ///Sets the `dst_image` field.
    pub fn set_dst_image(&mut self, dst_image: Image) -> &mut Self {
        self.dst_image = dst_image;
        self
    }
    ///Sets the `dst_image_layout` field.
    pub fn set_dst_image_layout(&mut self, dst_image_layout: ImageLayout) -> &mut Self {
        self.dst_image_layout = dst_image_layout;
        self
    }
    ///Sets the `regions` field.
    pub fn set_regions(&mut self, regions: SmallVec<[ImageResolve2; 8]>) -> &mut Self {
        self.regions = regions;
        self
    }
    ///Sets the `src_image` field in a builder way.
    pub fn with_src_image(mut self, src_image: Image) -> Self {
        self.src_image = src_image;
        self
    }
    ///Sets the `src_image_layout` field in a builder way.
    pub fn with_src_image_layout(mut self, src_image_layout: ImageLayout) -> Self {
        self.src_image_layout = src_image_layout;
        self
    }
    ///Sets the `dst_image` field in a builder way.
    pub fn with_dst_image(mut self, dst_image: Image) -> Self {
        self.dst_image = dst_image;
        self
    }
    ///Sets the `dst_image_layout` field in a builder way.
    pub fn with_dst_image_layout(mut self, dst_image_layout: ImageLayout) -> Self {
        self.dst_image_layout = dst_image_layout;
        self
    }
    ///Sets the `regions` field in a builder way.
    pub fn with_regions(mut self, regions: SmallVec<[ImageResolve2; 8]>) -> Self {
        self.regions = regions;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ResolveImageInfo2 {
    type LowLevel = crate::native::vulkan1_3::ResolveImageInfo2;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_regions = self.regions.len() as u32;
        let regions = bump
            .alloc_slice_fill_iter(self.regions.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::vulkan1_3::ResolveImageInfo2 {
            s_type: StructureType::ResolveImageInfo2,
            p_next: std::ptr::null(),
            src_image: self.src_image.into_low_level(context, bump),
            src_image_layout: self.src_image_layout.into_low_level(context, bump),
            dst_image: self.dst_image.into_low_level(context, bump),
            dst_image_layout: self.dst_image_layout.into_low_level(context, bump),
            region_count: len_regions,
            regions: regions,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ResolveImageInfo2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let regions_len = value.region_count;
        let mut regions = SmallVec::with_capacity(regions_len as usize);
        for i in 0..regions_len {
            regions.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.regions.add(i as usize).read(),
            ));
        }
        Self {
            src_image: crate::conv::FromLowLevel::from_low_level(context, value.src_image),
            src_image_layout: crate::conv::FromLowLevel::from_low_level(context, value.src_image_layout),
            dst_image: crate::conv::FromLowLevel::from_low_level(context, value.dst_image),
            dst_image_layout: crate::conv::FromLowLevel::from_low_level(context, value.dst_image_layout),
            regions: regions,
        }
    }
}
#[doc(alias = "VkPhysicalDeviceShaderTerminateInvocationFeatures")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceShaderTerminateInvocationFeatures {
    #[doc(alias = "shaderTerminateInvocation")]
    pub shader_terminate_invocation: bool,
}
impl PhysicalDeviceShaderTerminateInvocationFeatures {
    ///Get a reference to the `shader_terminate_invocation` field.
    pub fn shader_terminate_invocation(&self) -> &bool {
        &self.shader_terminate_invocation
    }
    ///Get a mutable reference to the `shader_terminate_invocation` field.
    pub fn shader_terminate_invocation_mut(&mut self) -> &mut bool {
        &mut self.shader_terminate_invocation
    }
    ///Sets the `shader_terminate_invocation` field.
    pub fn set_shader_terminate_invocation(&mut self, shader_terminate_invocation: bool) -> &mut Self {
        self.shader_terminate_invocation = shader_terminate_invocation;
        self
    }
    ///Sets the `shader_terminate_invocation` field in a builder way.
    pub fn with_shader_terminate_invocation(mut self, shader_terminate_invocation: bool) -> Self {
        self.shader_terminate_invocation = shader_terminate_invocation;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceShaderTerminateInvocationFeatures {
    type LowLevel = crate::native::vulkan1_3::PhysicalDeviceShaderTerminateInvocationFeatures;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::PhysicalDeviceShaderTerminateInvocationFeatures {
            s_type: StructureType::PhysicalDeviceShaderTerminateInvocationFeatures,
            p_next: std::ptr::null_mut(),
            shader_terminate_invocation: self.shader_terminate_invocation.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceShaderTerminateInvocationFeatures {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            shader_terminate_invocation: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_terminate_invocation,
            ),
        }
    }
}
#[doc(alias = "VkMemoryBarrier2")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MemoryBarrier2 {
    #[doc(alias = "srcStageMask")]
    pub src_stage_mask: PipelineStageFlags2,
    #[doc(alias = "srcAccessMask")]
    pub src_access_mask: AccessFlags2,
    #[doc(alias = "dstStageMask")]
    pub dst_stage_mask: PipelineStageFlags2,
    #[doc(alias = "dstAccessMask")]
    pub dst_access_mask: AccessFlags2,
}
impl MemoryBarrier2 {
    ///Get a reference to the `src_stage_mask` field.
    pub fn src_stage_mask(&self) -> PipelineStageFlags2 {
        self.src_stage_mask
    }
    ///Get a reference to the `src_access_mask` field.
    pub fn src_access_mask(&self) -> AccessFlags2 {
        self.src_access_mask
    }
    ///Get a reference to the `dst_stage_mask` field.
    pub fn dst_stage_mask(&self) -> PipelineStageFlags2 {
        self.dst_stage_mask
    }
    ///Get a reference to the `dst_access_mask` field.
    pub fn dst_access_mask(&self) -> AccessFlags2 {
        self.dst_access_mask
    }
    ///Get a mutable reference to the `src_stage_mask` field.
    pub fn src_stage_mask_mut(&mut self) -> &mut PipelineStageFlags2 {
        &mut self.src_stage_mask
    }
    ///Get a mutable reference to the `src_access_mask` field.
    pub fn src_access_mask_mut(&mut self) -> &mut AccessFlags2 {
        &mut self.src_access_mask
    }
    ///Get a mutable reference to the `dst_stage_mask` field.
    pub fn dst_stage_mask_mut(&mut self) -> &mut PipelineStageFlags2 {
        &mut self.dst_stage_mask
    }
    ///Get a mutable reference to the `dst_access_mask` field.
    pub fn dst_access_mask_mut(&mut self) -> &mut AccessFlags2 {
        &mut self.dst_access_mask
    }
    ///Sets the `src_stage_mask` field.
    pub fn set_src_stage_mask(&mut self, src_stage_mask: PipelineStageFlags2) -> &mut Self {
        self.src_stage_mask = src_stage_mask;
        self
    }
    ///Sets the `src_access_mask` field.
    pub fn set_src_access_mask(&mut self, src_access_mask: AccessFlags2) -> &mut Self {
        self.src_access_mask = src_access_mask;
        self
    }
    ///Sets the `dst_stage_mask` field.
    pub fn set_dst_stage_mask(&mut self, dst_stage_mask: PipelineStageFlags2) -> &mut Self {
        self.dst_stage_mask = dst_stage_mask;
        self
    }
    ///Sets the `dst_access_mask` field.
    pub fn set_dst_access_mask(&mut self, dst_access_mask: AccessFlags2) -> &mut Self {
        self.dst_access_mask = dst_access_mask;
        self
    }
    ///Sets the `src_stage_mask` field in a builder way.
    pub fn with_src_stage_mask(mut self, src_stage_mask: PipelineStageFlags2) -> Self {
        self.src_stage_mask = src_stage_mask;
        self
    }
    ///Sets the `src_access_mask` field in a builder way.
    pub fn with_src_access_mask(mut self, src_access_mask: AccessFlags2) -> Self {
        self.src_access_mask = src_access_mask;
        self
    }
    ///Sets the `dst_stage_mask` field in a builder way.
    pub fn with_dst_stage_mask(mut self, dst_stage_mask: PipelineStageFlags2) -> Self {
        self.dst_stage_mask = dst_stage_mask;
        self
    }
    ///Sets the `dst_access_mask` field in a builder way.
    pub fn with_dst_access_mask(mut self, dst_access_mask: AccessFlags2) -> Self {
        self.dst_access_mask = dst_access_mask;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for MemoryBarrier2 {
    type LowLevel = crate::native::vulkan1_3::MemoryBarrier2;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::MemoryBarrier2 {
            s_type: StructureType::MemoryBarrier2,
            p_next: std::ptr::null(),
            src_stage_mask: self.src_stage_mask.into_low_level(context, bump),
            src_access_mask: self.src_access_mask.into_low_level(context, bump),
            dst_stage_mask: self.dst_stage_mask.into_low_level(context, bump),
            dst_access_mask: self.dst_access_mask.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for MemoryBarrier2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            src_stage_mask: crate::conv::FromLowLevel::from_low_level(context, value.src_stage_mask),
            src_access_mask: crate::conv::FromLowLevel::from_low_level(context, value.src_access_mask),
            dst_stage_mask: crate::conv::FromLowLevel::from_low_level(context, value.dst_stage_mask),
            dst_access_mask: crate::conv::FromLowLevel::from_low_level(context, value.dst_access_mask),
        }
    }
}
#[doc(alias = "VkImageMemoryBarrier2")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ImageMemoryBarrier2 {
    #[doc(alias = "pNext")]
    pub extensions: SmallVec<[ImageMemoryBarrier2Extension; 1]>,
    #[doc(alias = "srcStageMask")]
    pub src_stage_mask: PipelineStageFlags2,
    #[doc(alias = "srcAccessMask")]
    pub src_access_mask: AccessFlags2,
    #[doc(alias = "dstStageMask")]
    pub dst_stage_mask: PipelineStageFlags2,
    #[doc(alias = "dstAccessMask")]
    pub dst_access_mask: AccessFlags2,
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
impl ImageMemoryBarrier2 {
    ///Adds an extension to the struct
    pub fn with_extension(mut self, ext: impl Into<ImageMemoryBarrier2Extension>) -> Self {
        self.extensions.push(ext.into());
        self
    }
    ///Get a reference to the `extensions` field.
    pub fn extensions(&self) -> &SmallVec<[ImageMemoryBarrier2Extension; 1]> {
        &self.extensions
    }
    ///Get a reference to the `src_stage_mask` field.
    pub fn src_stage_mask(&self) -> PipelineStageFlags2 {
        self.src_stage_mask
    }
    ///Get a reference to the `src_access_mask` field.
    pub fn src_access_mask(&self) -> AccessFlags2 {
        self.src_access_mask
    }
    ///Get a reference to the `dst_stage_mask` field.
    pub fn dst_stage_mask(&self) -> PipelineStageFlags2 {
        self.dst_stage_mask
    }
    ///Get a reference to the `dst_access_mask` field.
    pub fn dst_access_mask(&self) -> AccessFlags2 {
        self.dst_access_mask
    }
    ///Get a reference to the `old_layout` field.
    pub fn old_layout(&self) -> ImageLayout {
        self.old_layout
    }
    ///Get a reference to the `new_layout` field.
    pub fn new_layout(&self) -> ImageLayout {
        self.new_layout
    }
    ///Get a reference to the `src_queue_family_index` field.
    pub fn src_queue_family_index(&self) -> u32 {
        self.src_queue_family_index
    }
    ///Get a reference to the `dst_queue_family_index` field.
    pub fn dst_queue_family_index(&self) -> u32 {
        self.dst_queue_family_index
    }
    ///Get a reference to the `image` field.
    pub fn image(&self) -> &Image {
        &self.image
    }
    ///Get a reference to the `subresource_range` field.
    pub fn subresource_range(&self) -> ImageSubresourceRange {
        self.subresource_range
    }
    ///Get a mutable reference to the `extensions` field.
    pub fn extensions_mut(&mut self) -> &mut SmallVec<[ImageMemoryBarrier2Extension; 1]> {
        &mut self.extensions
    }
    ///Get a mutable reference to the `src_stage_mask` field.
    pub fn src_stage_mask_mut(&mut self) -> &mut PipelineStageFlags2 {
        &mut self.src_stage_mask
    }
    ///Get a mutable reference to the `src_access_mask` field.
    pub fn src_access_mask_mut(&mut self) -> &mut AccessFlags2 {
        &mut self.src_access_mask
    }
    ///Get a mutable reference to the `dst_stage_mask` field.
    pub fn dst_stage_mask_mut(&mut self) -> &mut PipelineStageFlags2 {
        &mut self.dst_stage_mask
    }
    ///Get a mutable reference to the `dst_access_mask` field.
    pub fn dst_access_mask_mut(&mut self) -> &mut AccessFlags2 {
        &mut self.dst_access_mask
    }
    ///Get a mutable reference to the `old_layout` field.
    pub fn old_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.old_layout
    }
    ///Get a mutable reference to the `new_layout` field.
    pub fn new_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.new_layout
    }
    ///Get a mutable reference to the `src_queue_family_index` field.
    pub fn src_queue_family_index_mut(&mut self) -> &mut u32 {
        &mut self.src_queue_family_index
    }
    ///Get a mutable reference to the `dst_queue_family_index` field.
    pub fn dst_queue_family_index_mut(&mut self) -> &mut u32 {
        &mut self.dst_queue_family_index
    }
    ///Get a mutable reference to the `image` field.
    pub fn image_mut(&mut self) -> &mut Image {
        &mut self.image
    }
    ///Get a mutable reference to the `subresource_range` field.
    pub fn subresource_range_mut(&mut self) -> &mut ImageSubresourceRange {
        &mut self.subresource_range
    }
    ///Sets the `extensions` field.
    pub fn set_extensions(&mut self, extensions: SmallVec<[ImageMemoryBarrier2Extension; 1]>) -> &mut Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `src_stage_mask` field.
    pub fn set_src_stage_mask(&mut self, src_stage_mask: PipelineStageFlags2) -> &mut Self {
        self.src_stage_mask = src_stage_mask;
        self
    }
    ///Sets the `src_access_mask` field.
    pub fn set_src_access_mask(&mut self, src_access_mask: AccessFlags2) -> &mut Self {
        self.src_access_mask = src_access_mask;
        self
    }
    ///Sets the `dst_stage_mask` field.
    pub fn set_dst_stage_mask(&mut self, dst_stage_mask: PipelineStageFlags2) -> &mut Self {
        self.dst_stage_mask = dst_stage_mask;
        self
    }
    ///Sets the `dst_access_mask` field.
    pub fn set_dst_access_mask(&mut self, dst_access_mask: AccessFlags2) -> &mut Self {
        self.dst_access_mask = dst_access_mask;
        self
    }
    ///Sets the `old_layout` field.
    pub fn set_old_layout(&mut self, old_layout: ImageLayout) -> &mut Self {
        self.old_layout = old_layout;
        self
    }
    ///Sets the `new_layout` field.
    pub fn set_new_layout(&mut self, new_layout: ImageLayout) -> &mut Self {
        self.new_layout = new_layout;
        self
    }
    ///Sets the `src_queue_family_index` field.
    pub fn set_src_queue_family_index(&mut self, src_queue_family_index: u32) -> &mut Self {
        self.src_queue_family_index = src_queue_family_index;
        self
    }
    ///Sets the `dst_queue_family_index` field.
    pub fn set_dst_queue_family_index(&mut self, dst_queue_family_index: u32) -> &mut Self {
        self.dst_queue_family_index = dst_queue_family_index;
        self
    }
    ///Sets the `image` field.
    pub fn set_image(&mut self, image: Image) -> &mut Self {
        self.image = image;
        self
    }
    ///Sets the `subresource_range` field.
    pub fn set_subresource_range(&mut self, subresource_range: ImageSubresourceRange) -> &mut Self {
        self.subresource_range = subresource_range;
        self
    }
    ///Sets the `extensions` field in a builder way.
    pub fn with_extensions(mut self, extensions: SmallVec<[ImageMemoryBarrier2Extension; 1]>) -> Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `src_stage_mask` field in a builder way.
    pub fn with_src_stage_mask(mut self, src_stage_mask: PipelineStageFlags2) -> Self {
        self.src_stage_mask = src_stage_mask;
        self
    }
    ///Sets the `src_access_mask` field in a builder way.
    pub fn with_src_access_mask(mut self, src_access_mask: AccessFlags2) -> Self {
        self.src_access_mask = src_access_mask;
        self
    }
    ///Sets the `dst_stage_mask` field in a builder way.
    pub fn with_dst_stage_mask(mut self, dst_stage_mask: PipelineStageFlags2) -> Self {
        self.dst_stage_mask = dst_stage_mask;
        self
    }
    ///Sets the `dst_access_mask` field in a builder way.
    pub fn with_dst_access_mask(mut self, dst_access_mask: AccessFlags2) -> Self {
        self.dst_access_mask = dst_access_mask;
        self
    }
    ///Sets the `old_layout` field in a builder way.
    pub fn with_old_layout(mut self, old_layout: ImageLayout) -> Self {
        self.old_layout = old_layout;
        self
    }
    ///Sets the `new_layout` field in a builder way.
    pub fn with_new_layout(mut self, new_layout: ImageLayout) -> Self {
        self.new_layout = new_layout;
        self
    }
    ///Sets the `src_queue_family_index` field in a builder way.
    pub fn with_src_queue_family_index(mut self, src_queue_family_index: u32) -> Self {
        self.src_queue_family_index = src_queue_family_index;
        self
    }
    ///Sets the `dst_queue_family_index` field in a builder way.
    pub fn with_dst_queue_family_index(mut self, dst_queue_family_index: u32) -> Self {
        self.dst_queue_family_index = dst_queue_family_index;
        self
    }
    ///Sets the `image` field in a builder way.
    pub fn with_image(mut self, image: Image) -> Self {
        self.image = image;
        self
    }
    ///Sets the `subresource_range` field in a builder way.
    pub fn with_subresource_range(mut self, subresource_range: ImageSubresourceRange) -> Self {
        self.subresource_range = subresource_range;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ImageMemoryBarrier2 {
    type LowLevel = crate::native::vulkan1_3::ImageMemoryBarrier2;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let mut next = std::ptr::null();
        let mut extensions = self.extensions.iter();
        while let Some(ext) = extensions.next() {
            let ext = ext.into_low_level(context, bump);
            (*ext).next = next;
            next = ext;
        }
        crate::native::vulkan1_3::ImageMemoryBarrier2 {
            s_type: StructureType::ImageMemoryBarrier2,
            p_next: next,
            src_stage_mask: self.src_stage_mask.into_low_level(context, bump),
            src_access_mask: self.src_access_mask.into_low_level(context, bump),
            dst_stage_mask: self.dst_stage_mask.into_low_level(context, bump),
            dst_access_mask: self.dst_access_mask.into_low_level(context, bump),
            old_layout: self.old_layout.into_low_level(context, bump),
            new_layout: self.new_layout.into_low_level(context, bump),
            src_queue_family_index: self.src_queue_family_index.into_low_level(context, bump),
            dst_queue_family_index: self.dst_queue_family_index.into_low_level(context, bump),
            image: self.image.into_low_level(context, bump),
            subresource_range: self.subresource_range.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ImageMemoryBarrier2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let mut next = value.p_next;
        let mut extensions = SmallVec::new();
        while !next.is_null() {
            extensions.push(crate::conv::FromLowLevel::from_low_level(context, next));
            next = std::ptr::read(next).next;
        }
        Self {
            extensions: extensions,
            src_stage_mask: crate::conv::FromLowLevel::from_low_level(context, value.src_stage_mask),
            src_access_mask: crate::conv::FromLowLevel::from_low_level(context, value.src_access_mask),
            dst_stage_mask: crate::conv::FromLowLevel::from_low_level(context, value.dst_stage_mask),
            dst_access_mask: crate::conv::FromLowLevel::from_low_level(context, value.dst_access_mask),
            old_layout: crate::conv::FromLowLevel::from_low_level(context, value.old_layout),
            new_layout: crate::conv::FromLowLevel::from_low_level(context, value.new_layout),
            src_queue_family_index: crate::conv::FromLowLevel::from_low_level(context, value.src_queue_family_index),
            dst_queue_family_index: crate::conv::FromLowLevel::from_low_level(context, value.dst_queue_family_index),
            image: crate::conv::FromLowLevel::from_low_level(context, value.image),
            subresource_range: crate::conv::FromLowLevel::from_low_level(context, value.subresource_range),
        }
    }
}
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
///Extensions for [`ImageMemoryBarrier2`]
pub enum ImageMemoryBarrier2Extension {
    #[cfg(feature = "VK_EXT_sample_locations")]
    ///Contains a type [`SampleLocationsInfoEXT`] for extending [`ImageMemoryBarrier2`]
    SampleLocationsInfoEXT(SampleLocationsInfoEXT),
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ImageMemoryBarrier2Extension {
    type LowLevel = *const crate::native::vulkan1_0::BaseInStructure;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self {
            #[cfg(feature = "VK_EXT_sample_locations")]
            Self::SampleLocationsInfoEXT(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::extensions::ext_sample_locations::SampleLocationsInfoEXT)
                .cast(),
            other => unreachable!("unexpected variant {:?}", other),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ImageMemoryBarrier2Extension {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        assert!(!value.is_null());
        match (*value).s_type {
            #[cfg(feature = "VK_EXT_sample_locations")]
            crate::native::vulkan1_0::StructureType::SampleLocationsInfoExt => {
                Self::SampleLocationsInfoEXT(SampleLocationsInfoEXT::from_low_level(
                    context,
                    std::ptr::read(
                        value.cast::<crate::native::extensions::ext_sample_locations::SampleLocationsInfoEXT>(),
                    ),
                ))
            },
            other => panic!(
                "Structure type {:?} is not a member of {}",
                other,
                stringify!(ImageMemoryBarrier2)
            ),
        }
    }
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl From<SampleLocationsInfoEXT> for ImageMemoryBarrier2Extension {
    fn from(ext: SampleLocationsInfoEXT) -> Self {
        Self::SampleLocationsInfoEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl TryInto<SampleLocationsInfoEXT> for ImageMemoryBarrier2Extension {
    type Error = ImageMemoryBarrier2Extension;
    fn try_into(self) -> Result<SampleLocationsInfoEXT, Self::Error> {
        match self {
            Self::SampleLocationsInfoEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[doc(alias = "VkBufferMemoryBarrier2")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BufferMemoryBarrier2 {
    #[doc(alias = "srcStageMask")]
    pub src_stage_mask: PipelineStageFlags2,
    #[doc(alias = "srcAccessMask")]
    pub src_access_mask: AccessFlags2,
    #[doc(alias = "dstStageMask")]
    pub dst_stage_mask: PipelineStageFlags2,
    #[doc(alias = "dstAccessMask")]
    pub dst_access_mask: AccessFlags2,
    #[doc(alias = "srcQueueFamilyIndex")]
    pub src_queue_family_index: u32,
    #[doc(alias = "dstQueueFamilyIndex")]
    pub dst_queue_family_index: u32,
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub size: DeviceSize,
}
impl BufferMemoryBarrier2 {
    ///Get a reference to the `src_stage_mask` field.
    pub fn src_stage_mask(&self) -> PipelineStageFlags2 {
        self.src_stage_mask
    }
    ///Get a reference to the `src_access_mask` field.
    pub fn src_access_mask(&self) -> AccessFlags2 {
        self.src_access_mask
    }
    ///Get a reference to the `dst_stage_mask` field.
    pub fn dst_stage_mask(&self) -> PipelineStageFlags2 {
        self.dst_stage_mask
    }
    ///Get a reference to the `dst_access_mask` field.
    pub fn dst_access_mask(&self) -> AccessFlags2 {
        self.dst_access_mask
    }
    ///Get a reference to the `src_queue_family_index` field.
    pub fn src_queue_family_index(&self) -> u32 {
        self.src_queue_family_index
    }
    ///Get a reference to the `dst_queue_family_index` field.
    pub fn dst_queue_family_index(&self) -> u32 {
        self.dst_queue_family_index
    }
    ///Get a reference to the `buffer` field.
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }
    ///Get a reference to the `offset` field.
    pub fn offset(&self) -> &DeviceSize {
        &self.offset
    }
    ///Get a reference to the `size` field.
    pub fn size(&self) -> &DeviceSize {
        &self.size
    }
    ///Get a mutable reference to the `src_stage_mask` field.
    pub fn src_stage_mask_mut(&mut self) -> &mut PipelineStageFlags2 {
        &mut self.src_stage_mask
    }
    ///Get a mutable reference to the `src_access_mask` field.
    pub fn src_access_mask_mut(&mut self) -> &mut AccessFlags2 {
        &mut self.src_access_mask
    }
    ///Get a mutable reference to the `dst_stage_mask` field.
    pub fn dst_stage_mask_mut(&mut self) -> &mut PipelineStageFlags2 {
        &mut self.dst_stage_mask
    }
    ///Get a mutable reference to the `dst_access_mask` field.
    pub fn dst_access_mask_mut(&mut self) -> &mut AccessFlags2 {
        &mut self.dst_access_mask
    }
    ///Get a mutable reference to the `src_queue_family_index` field.
    pub fn src_queue_family_index_mut(&mut self) -> &mut u32 {
        &mut self.src_queue_family_index
    }
    ///Get a mutable reference to the `dst_queue_family_index` field.
    pub fn dst_queue_family_index_mut(&mut self) -> &mut u32 {
        &mut self.dst_queue_family_index
    }
    ///Get a mutable reference to the `buffer` field.
    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }
    ///Get a mutable reference to the `offset` field.
    pub fn offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.offset
    }
    ///Get a mutable reference to the `size` field.
    pub fn size_mut(&mut self) -> &mut DeviceSize {
        &mut self.size
    }
    ///Sets the `src_stage_mask` field.
    pub fn set_src_stage_mask(&mut self, src_stage_mask: PipelineStageFlags2) -> &mut Self {
        self.src_stage_mask = src_stage_mask;
        self
    }
    ///Sets the `src_access_mask` field.
    pub fn set_src_access_mask(&mut self, src_access_mask: AccessFlags2) -> &mut Self {
        self.src_access_mask = src_access_mask;
        self
    }
    ///Sets the `dst_stage_mask` field.
    pub fn set_dst_stage_mask(&mut self, dst_stage_mask: PipelineStageFlags2) -> &mut Self {
        self.dst_stage_mask = dst_stage_mask;
        self
    }
    ///Sets the `dst_access_mask` field.
    pub fn set_dst_access_mask(&mut self, dst_access_mask: AccessFlags2) -> &mut Self {
        self.dst_access_mask = dst_access_mask;
        self
    }
    ///Sets the `src_queue_family_index` field.
    pub fn set_src_queue_family_index(&mut self, src_queue_family_index: u32) -> &mut Self {
        self.src_queue_family_index = src_queue_family_index;
        self
    }
    ///Sets the `dst_queue_family_index` field.
    pub fn set_dst_queue_family_index(&mut self, dst_queue_family_index: u32) -> &mut Self {
        self.dst_queue_family_index = dst_queue_family_index;
        self
    }
    ///Sets the `buffer` field.
    pub fn set_buffer(&mut self, buffer: Buffer) -> &mut Self {
        self.buffer = buffer;
        self
    }
    ///Sets the `offset` field.
    pub fn set_offset(&mut self, offset: DeviceSize) -> &mut Self {
        self.offset = offset;
        self
    }
    ///Sets the `size` field.
    pub fn set_size(&mut self, size: DeviceSize) -> &mut Self {
        self.size = size;
        self
    }
    ///Sets the `src_stage_mask` field in a builder way.
    pub fn with_src_stage_mask(mut self, src_stage_mask: PipelineStageFlags2) -> Self {
        self.src_stage_mask = src_stage_mask;
        self
    }
    ///Sets the `src_access_mask` field in a builder way.
    pub fn with_src_access_mask(mut self, src_access_mask: AccessFlags2) -> Self {
        self.src_access_mask = src_access_mask;
        self
    }
    ///Sets the `dst_stage_mask` field in a builder way.
    pub fn with_dst_stage_mask(mut self, dst_stage_mask: PipelineStageFlags2) -> Self {
        self.dst_stage_mask = dst_stage_mask;
        self
    }
    ///Sets the `dst_access_mask` field in a builder way.
    pub fn with_dst_access_mask(mut self, dst_access_mask: AccessFlags2) -> Self {
        self.dst_access_mask = dst_access_mask;
        self
    }
    ///Sets the `src_queue_family_index` field in a builder way.
    pub fn with_src_queue_family_index(mut self, src_queue_family_index: u32) -> Self {
        self.src_queue_family_index = src_queue_family_index;
        self
    }
    ///Sets the `dst_queue_family_index` field in a builder way.
    pub fn with_dst_queue_family_index(mut self, dst_queue_family_index: u32) -> Self {
        self.dst_queue_family_index = dst_queue_family_index;
        self
    }
    ///Sets the `buffer` field in a builder way.
    pub fn with_buffer(mut self, buffer: Buffer) -> Self {
        self.buffer = buffer;
        self
    }
    ///Sets the `offset` field in a builder way.
    pub fn with_offset(mut self, offset: DeviceSize) -> Self {
        self.offset = offset;
        self
    }
    ///Sets the `size` field in a builder way.
    pub fn with_size(mut self, size: DeviceSize) -> Self {
        self.size = size;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for BufferMemoryBarrier2 {
    type LowLevel = crate::native::vulkan1_3::BufferMemoryBarrier2;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::BufferMemoryBarrier2 {
            s_type: StructureType::BufferMemoryBarrier2,
            p_next: std::ptr::null(),
            src_stage_mask: self.src_stage_mask.into_low_level(context, bump),
            src_access_mask: self.src_access_mask.into_low_level(context, bump),
            dst_stage_mask: self.dst_stage_mask.into_low_level(context, bump),
            dst_access_mask: self.dst_access_mask.into_low_level(context, bump),
            src_queue_family_index: self.src_queue_family_index.into_low_level(context, bump),
            dst_queue_family_index: self.dst_queue_family_index.into_low_level(context, bump),
            buffer: self.buffer.into_low_level(context, bump),
            offset: self.offset.into_low_level(context, bump),
            size: self.size.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for BufferMemoryBarrier2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            src_stage_mask: crate::conv::FromLowLevel::from_low_level(context, value.src_stage_mask),
            src_access_mask: crate::conv::FromLowLevel::from_low_level(context, value.src_access_mask),
            dst_stage_mask: crate::conv::FromLowLevel::from_low_level(context, value.dst_stage_mask),
            dst_access_mask: crate::conv::FromLowLevel::from_low_level(context, value.dst_access_mask),
            src_queue_family_index: crate::conv::FromLowLevel::from_low_level(context, value.src_queue_family_index),
            dst_queue_family_index: crate::conv::FromLowLevel::from_low_level(context, value.dst_queue_family_index),
            buffer: crate::conv::FromLowLevel::from_low_level(context, value.buffer),
            offset: crate::conv::FromLowLevel::from_low_level(context, value.offset),
            size: crate::conv::FromLowLevel::from_low_level(context, value.size),
        }
    }
}
#[doc(alias = "VkDependencyInfo")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DependencyInfo {
    #[doc(alias = "dependencyFlags")]
    pub dependency_flags: DependencyFlags,
    #[doc(alias = "pMemoryBarriers")]
    pub memory_barriers: SmallVec<[MemoryBarrier2; 8]>,
    #[doc(alias = "pBufferMemoryBarriers")]
    pub buffer_memory_barriers: SmallVec<[BufferMemoryBarrier2; 8]>,
    #[doc(alias = "pImageMemoryBarriers")]
    pub image_memory_barriers: SmallVec<[ImageMemoryBarrier2; 8]>,
}
impl DependencyInfo {
    ///Get a reference to the `dependency_flags` field.
    pub fn dependency_flags(&self) -> DependencyFlags {
        self.dependency_flags
    }
    ///Get a reference to the `memory_barriers` field.
    pub fn memory_barriers(&self) -> &SmallVec<[MemoryBarrier2; 8]> {
        &self.memory_barriers
    }
    ///Get a reference to the `buffer_memory_barriers` field.
    pub fn buffer_memory_barriers(&self) -> &SmallVec<[BufferMemoryBarrier2; 8]> {
        &self.buffer_memory_barriers
    }
    ///Get a reference to the `image_memory_barriers` field.
    pub fn image_memory_barriers(&self) -> &SmallVec<[ImageMemoryBarrier2; 8]> {
        &self.image_memory_barriers
    }
    ///Get a mutable reference to the `dependency_flags` field.
    pub fn dependency_flags_mut(&mut self) -> &mut DependencyFlags {
        &mut self.dependency_flags
    }
    ///Get a mutable reference to the `memory_barriers` field.
    pub fn memory_barriers_mut(&mut self) -> &mut SmallVec<[MemoryBarrier2; 8]> {
        &mut self.memory_barriers
    }
    ///Get a mutable reference to the `buffer_memory_barriers` field.
    pub fn buffer_memory_barriers_mut(&mut self) -> &mut SmallVec<[BufferMemoryBarrier2; 8]> {
        &mut self.buffer_memory_barriers
    }
    ///Get a mutable reference to the `image_memory_barriers` field.
    pub fn image_memory_barriers_mut(&mut self) -> &mut SmallVec<[ImageMemoryBarrier2; 8]> {
        &mut self.image_memory_barriers
    }
    ///Sets the `dependency_flags` field.
    pub fn set_dependency_flags(&mut self, dependency_flags: DependencyFlags) -> &mut Self {
        self.dependency_flags = dependency_flags;
        self
    }
    ///Sets the `memory_barriers` field.
    pub fn set_memory_barriers(&mut self, memory_barriers: SmallVec<[MemoryBarrier2; 8]>) -> &mut Self {
        self.memory_barriers = memory_barriers;
        self
    }
    ///Sets the `buffer_memory_barriers` field.
    pub fn set_buffer_memory_barriers(
        &mut self,
        buffer_memory_barriers: SmallVec<[BufferMemoryBarrier2; 8]>,
    ) -> &mut Self {
        self.buffer_memory_barriers = buffer_memory_barriers;
        self
    }
    ///Sets the `image_memory_barriers` field.
    pub fn set_image_memory_barriers(
        &mut self,
        image_memory_barriers: SmallVec<[ImageMemoryBarrier2; 8]>,
    ) -> &mut Self {
        self.image_memory_barriers = image_memory_barriers;
        self
    }
    ///Sets the `dependency_flags` field in a builder way.
    pub fn with_dependency_flags(mut self, dependency_flags: DependencyFlags) -> Self {
        self.dependency_flags = dependency_flags;
        self
    }
    ///Sets the `memory_barriers` field in a builder way.
    pub fn with_memory_barriers(mut self, memory_barriers: SmallVec<[MemoryBarrier2; 8]>) -> Self {
        self.memory_barriers = memory_barriers;
        self
    }
    ///Sets the `buffer_memory_barriers` field in a builder way.
    pub fn with_buffer_memory_barriers(mut self, buffer_memory_barriers: SmallVec<[BufferMemoryBarrier2; 8]>) -> Self {
        self.buffer_memory_barriers = buffer_memory_barriers;
        self
    }
    ///Sets the `image_memory_barriers` field in a builder way.
    pub fn with_image_memory_barriers(mut self, image_memory_barriers: SmallVec<[ImageMemoryBarrier2; 8]>) -> Self {
        self.image_memory_barriers = image_memory_barriers;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DependencyInfo {
    type LowLevel = crate::native::vulkan1_3::DependencyInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_memory_barriers = self.memory_barriers.len() as u32;
        let memory_barriers = bump
            .alloc_slice_fill_iter(self.memory_barriers.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        let len_buffer_memory_barriers = self.buffer_memory_barriers.len() as u32;
        let buffer_memory_barriers = bump
            .alloc_slice_fill_iter(
                self.buffer_memory_barriers
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_ptr()
            .cast();
        let len_image_memory_barriers = self.image_memory_barriers.len() as u32;
        let image_memory_barriers = bump
            .alloc_slice_fill_iter(
                self.image_memory_barriers
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_ptr()
            .cast();
        crate::native::vulkan1_3::DependencyInfo {
            s_type: StructureType::DependencyInfo,
            p_next: std::ptr::null(),
            dependency_flags: self.dependency_flags.into_low_level(context, bump),
            memory_barrier_count: len_memory_barriers,
            memory_barriers: memory_barriers,
            buffer_memory_barrier_count: len_buffer_memory_barriers,
            buffer_memory_barriers: buffer_memory_barriers,
            image_memory_barrier_count: len_image_memory_barriers,
            image_memory_barriers: image_memory_barriers,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DependencyInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let memory_barriers_len = value.memory_barrier_count;
        let mut memory_barriers = SmallVec::with_capacity(memory_barriers_len as usize);
        for i in 0..memory_barriers_len {
            memory_barriers.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.memory_barriers.add(i as usize).read(),
            ));
        }
        let buffer_memory_barriers_len = value.buffer_memory_barrier_count;
        let mut buffer_memory_barriers = SmallVec::with_capacity(buffer_memory_barriers_len as usize);
        for i in 0..buffer_memory_barriers_len {
            buffer_memory_barriers.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.buffer_memory_barriers.add(i as usize).read(),
            ));
        }
        let image_memory_barriers_len = value.image_memory_barrier_count;
        let mut image_memory_barriers = SmallVec::with_capacity(image_memory_barriers_len as usize);
        for i in 0..image_memory_barriers_len {
            image_memory_barriers.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.image_memory_barriers.add(i as usize).read(),
            ));
        }
        Self {
            dependency_flags: crate::conv::FromLowLevel::from_low_level(context, value.dependency_flags),
            memory_barriers: memory_barriers,
            buffer_memory_barriers: buffer_memory_barriers,
            image_memory_barriers: image_memory_barriers,
        }
    }
}
#[doc(alias = "VkSemaphoreSubmitInfo")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SemaphoreSubmitInfo {
    pub semaphore: Semaphore,
    pub value: u64,
    #[doc(alias = "stageMask")]
    pub stage_mask: PipelineStageFlags2,
    #[doc(alias = "deviceIndex")]
    pub device_index: u32,
}
impl SemaphoreSubmitInfo {
    ///Get a reference to the `semaphore` field.
    pub fn semaphore(&self) -> &Semaphore {
        &self.semaphore
    }
    ///Get a reference to the `value` field.
    pub fn value(&self) -> u64 {
        self.value
    }
    ///Get a reference to the `stage_mask` field.
    pub fn stage_mask(&self) -> PipelineStageFlags2 {
        self.stage_mask
    }
    ///Get a reference to the `device_index` field.
    pub fn device_index(&self) -> u32 {
        self.device_index
    }
    ///Get a mutable reference to the `semaphore` field.
    pub fn semaphore_mut(&mut self) -> &mut Semaphore {
        &mut self.semaphore
    }
    ///Get a mutable reference to the `value` field.
    pub fn value_mut(&mut self) -> &mut u64 {
        &mut self.value
    }
    ///Get a mutable reference to the `stage_mask` field.
    pub fn stage_mask_mut(&mut self) -> &mut PipelineStageFlags2 {
        &mut self.stage_mask
    }
    ///Get a mutable reference to the `device_index` field.
    pub fn device_index_mut(&mut self) -> &mut u32 {
        &mut self.device_index
    }
    ///Sets the `semaphore` field.
    pub fn set_semaphore(&mut self, semaphore: Semaphore) -> &mut Self {
        self.semaphore = semaphore;
        self
    }
    ///Sets the `value` field.
    pub fn set_value(&mut self, value: u64) -> &mut Self {
        self.value = value;
        self
    }
    ///Sets the `stage_mask` field.
    pub fn set_stage_mask(&mut self, stage_mask: PipelineStageFlags2) -> &mut Self {
        self.stage_mask = stage_mask;
        self
    }
    ///Sets the `device_index` field.
    pub fn set_device_index(&mut self, device_index: u32) -> &mut Self {
        self.device_index = device_index;
        self
    }
    ///Sets the `semaphore` field in a builder way.
    pub fn with_semaphore(mut self, semaphore: Semaphore) -> Self {
        self.semaphore = semaphore;
        self
    }
    ///Sets the `value` field in a builder way.
    pub fn with_value(mut self, value: u64) -> Self {
        self.value = value;
        self
    }
    ///Sets the `stage_mask` field in a builder way.
    pub fn with_stage_mask(mut self, stage_mask: PipelineStageFlags2) -> Self {
        self.stage_mask = stage_mask;
        self
    }
    ///Sets the `device_index` field in a builder way.
    pub fn with_device_index(mut self, device_index: u32) -> Self {
        self.device_index = device_index;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SemaphoreSubmitInfo {
    type LowLevel = crate::native::vulkan1_3::SemaphoreSubmitInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::SemaphoreSubmitInfo {
            s_type: StructureType::SemaphoreSubmitInfo,
            p_next: std::ptr::null(),
            semaphore: self.semaphore.into_low_level(context, bump),
            value: self.value.into_low_level(context, bump),
            stage_mask: self.stage_mask.into_low_level(context, bump),
            device_index: self.device_index.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SemaphoreSubmitInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            semaphore: crate::conv::FromLowLevel::from_low_level(context, value.semaphore),
            value: crate::conv::FromLowLevel::from_low_level(context, value.value),
            stage_mask: crate::conv::FromLowLevel::from_low_level(context, value.stage_mask),
            device_index: crate::conv::FromLowLevel::from_low_level(context, value.device_index),
        }
    }
}
#[doc(alias = "VkCommandBufferSubmitInfo")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CommandBufferSubmitInfo {
    #[doc(alias = "commandBuffer")]
    pub command_buffer: CommandBuffer,
    #[doc(alias = "deviceMask")]
    pub device_mask: u32,
}
impl CommandBufferSubmitInfo {
    ///Get a reference to the `command_buffer` field.
    pub fn command_buffer(&self) -> &CommandBuffer {
        &self.command_buffer
    }
    ///Get a reference to the `device_mask` field.
    pub fn device_mask(&self) -> u32 {
        self.device_mask
    }
    ///Get a mutable reference to the `command_buffer` field.
    pub fn command_buffer_mut(&mut self) -> &mut CommandBuffer {
        &mut self.command_buffer
    }
    ///Get a mutable reference to the `device_mask` field.
    pub fn device_mask_mut(&mut self) -> &mut u32 {
        &mut self.device_mask
    }
    ///Sets the `command_buffer` field.
    pub fn set_command_buffer(&mut self, command_buffer: CommandBuffer) -> &mut Self {
        self.command_buffer = command_buffer;
        self
    }
    ///Sets the `device_mask` field.
    pub fn set_device_mask(&mut self, device_mask: u32) -> &mut Self {
        self.device_mask = device_mask;
        self
    }
    ///Sets the `command_buffer` field in a builder way.
    pub fn with_command_buffer(mut self, command_buffer: CommandBuffer) -> Self {
        self.command_buffer = command_buffer;
        self
    }
    ///Sets the `device_mask` field in a builder way.
    pub fn with_device_mask(mut self, device_mask: u32) -> Self {
        self.device_mask = device_mask;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for CommandBufferSubmitInfo {
    type LowLevel = crate::native::vulkan1_3::CommandBufferSubmitInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::CommandBufferSubmitInfo {
            s_type: StructureType::CommandBufferSubmitInfo,
            p_next: std::ptr::null(),
            command_buffer: self.command_buffer.into_low_level(context, bump),
            device_mask: self.device_mask.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for CommandBufferSubmitInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            command_buffer: crate::conv::FromLowLevel::from_low_level(context, value.command_buffer),
            device_mask: crate::conv::FromLowLevel::from_low_level(context, value.device_mask),
        }
    }
}
#[doc(alias = "VkSubmitInfo2")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SubmitInfo2 {
    #[doc(alias = "pNext")]
    pub extensions: SmallVec<[SubmitInfo2Extension; 1]>,
    pub flags: SubmitFlags,
    #[doc(alias = "pWaitSemaphoreInfos")]
    pub wait_semaphore_infos: SmallVec<[SemaphoreSubmitInfo; 8]>,
    #[doc(alias = "pCommandBufferInfos")]
    pub command_buffer_infos: SmallVec<[CommandBufferSubmitInfo; 8]>,
    #[doc(alias = "pSignalSemaphoreInfos")]
    pub signal_semaphore_infos: SmallVec<[SemaphoreSubmitInfo; 8]>,
}
impl SubmitInfo2 {
    ///Adds an extension to the struct
    pub fn with_extension(mut self, ext: impl Into<SubmitInfo2Extension>) -> Self {
        self.extensions.push(ext.into());
        self
    }
    ///Get a reference to the `extensions` field.
    pub fn extensions(&self) -> &SmallVec<[SubmitInfo2Extension; 1]> {
        &self.extensions
    }
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> SubmitFlags {
        self.flags
    }
    ///Get a reference to the `wait_semaphore_infos` field.
    pub fn wait_semaphore_infos(&self) -> &SmallVec<[SemaphoreSubmitInfo; 8]> {
        &self.wait_semaphore_infos
    }
    ///Get a reference to the `command_buffer_infos` field.
    pub fn command_buffer_infos(&self) -> &SmallVec<[CommandBufferSubmitInfo; 8]> {
        &self.command_buffer_infos
    }
    ///Get a reference to the `signal_semaphore_infos` field.
    pub fn signal_semaphore_infos(&self) -> &SmallVec<[SemaphoreSubmitInfo; 8]> {
        &self.signal_semaphore_infos
    }
    ///Get a mutable reference to the `extensions` field.
    pub fn extensions_mut(&mut self) -> &mut SmallVec<[SubmitInfo2Extension; 1]> {
        &mut self.extensions
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut SubmitFlags {
        &mut self.flags
    }
    ///Get a mutable reference to the `wait_semaphore_infos` field.
    pub fn wait_semaphore_infos_mut(&mut self) -> &mut SmallVec<[SemaphoreSubmitInfo; 8]> {
        &mut self.wait_semaphore_infos
    }
    ///Get a mutable reference to the `command_buffer_infos` field.
    pub fn command_buffer_infos_mut(&mut self) -> &mut SmallVec<[CommandBufferSubmitInfo; 8]> {
        &mut self.command_buffer_infos
    }
    ///Get a mutable reference to the `signal_semaphore_infos` field.
    pub fn signal_semaphore_infos_mut(&mut self) -> &mut SmallVec<[SemaphoreSubmitInfo; 8]> {
        &mut self.signal_semaphore_infos
    }
    ///Sets the `extensions` field.
    pub fn set_extensions(&mut self, extensions: SmallVec<[SubmitInfo2Extension; 1]>) -> &mut Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: SubmitFlags) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `wait_semaphore_infos` field.
    pub fn set_wait_semaphore_infos(&mut self, wait_semaphore_infos: SmallVec<[SemaphoreSubmitInfo; 8]>) -> &mut Self {
        self.wait_semaphore_infos = wait_semaphore_infos;
        self
    }
    ///Sets the `command_buffer_infos` field.
    pub fn set_command_buffer_infos(
        &mut self,
        command_buffer_infos: SmallVec<[CommandBufferSubmitInfo; 8]>,
    ) -> &mut Self {
        self.command_buffer_infos = command_buffer_infos;
        self
    }
    ///Sets the `signal_semaphore_infos` field.
    pub fn set_signal_semaphore_infos(
        &mut self,
        signal_semaphore_infos: SmallVec<[SemaphoreSubmitInfo; 8]>,
    ) -> &mut Self {
        self.signal_semaphore_infos = signal_semaphore_infos;
        self
    }
    ///Sets the `extensions` field in a builder way.
    pub fn with_extensions(mut self, extensions: SmallVec<[SubmitInfo2Extension; 1]>) -> Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: SubmitFlags) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `wait_semaphore_infos` field in a builder way.
    pub fn with_wait_semaphore_infos(mut self, wait_semaphore_infos: SmallVec<[SemaphoreSubmitInfo; 8]>) -> Self {
        self.wait_semaphore_infos = wait_semaphore_infos;
        self
    }
    ///Sets the `command_buffer_infos` field in a builder way.
    pub fn with_command_buffer_infos(mut self, command_buffer_infos: SmallVec<[CommandBufferSubmitInfo; 8]>) -> Self {
        self.command_buffer_infos = command_buffer_infos;
        self
    }
    ///Sets the `signal_semaphore_infos` field in a builder way.
    pub fn with_signal_semaphore_infos(mut self, signal_semaphore_infos: SmallVec<[SemaphoreSubmitInfo; 8]>) -> Self {
        self.signal_semaphore_infos = signal_semaphore_infos;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SubmitInfo2 {
    type LowLevel = crate::native::vulkan1_3::SubmitInfo2;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let mut next = std::ptr::null();
        let mut extensions = self.extensions.iter();
        while let Some(ext) = extensions.next() {
            let ext = ext.into_low_level(context, bump);
            (*ext).next = next;
            next = ext;
        }
        let len_wait_semaphore_infos = self.wait_semaphore_infos.len() as u32;
        let wait_semaphore_infos = bump
            .alloc_slice_fill_iter(
                self.wait_semaphore_infos
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_ptr()
            .cast();
        let len_command_buffer_infos = self.command_buffer_infos.len() as u32;
        let command_buffer_infos = bump
            .alloc_slice_fill_iter(
                self.command_buffer_infos
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_ptr()
            .cast();
        let len_signal_semaphore_infos = self.signal_semaphore_infos.len() as u32;
        let signal_semaphore_infos = bump
            .alloc_slice_fill_iter(
                self.signal_semaphore_infos
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_ptr()
            .cast();
        crate::native::vulkan1_3::SubmitInfo2 {
            s_type: StructureType::SubmitInfo2,
            p_next: next,
            flags: self.flags.into_low_level(context, bump),
            wait_semaphore_info_count: len_wait_semaphore_infos,
            wait_semaphore_infos: wait_semaphore_infos,
            command_buffer_info_count: len_command_buffer_infos,
            command_buffer_infos: command_buffer_infos,
            signal_semaphore_info_count: len_signal_semaphore_infos,
            signal_semaphore_infos: signal_semaphore_infos,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SubmitInfo2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let mut next = value.p_next;
        let mut extensions = SmallVec::new();
        while !next.is_null() {
            extensions.push(crate::conv::FromLowLevel::from_low_level(context, next));
            next = std::ptr::read(next).next;
        }
        let wait_semaphore_infos_len = value.wait_semaphore_info_count;
        let mut wait_semaphore_infos = SmallVec::with_capacity(wait_semaphore_infos_len as usize);
        for i in 0..wait_semaphore_infos_len {
            wait_semaphore_infos.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.wait_semaphore_infos.add(i as usize).read(),
            ));
        }
        let command_buffer_infos_len = value.command_buffer_info_count;
        let mut command_buffer_infos = SmallVec::with_capacity(command_buffer_infos_len as usize);
        for i in 0..command_buffer_infos_len {
            command_buffer_infos.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.command_buffer_infos.add(i as usize).read(),
            ));
        }
        let signal_semaphore_infos_len = value.signal_semaphore_info_count;
        let mut signal_semaphore_infos = SmallVec::with_capacity(signal_semaphore_infos_len as usize);
        for i in 0..signal_semaphore_infos_len {
            signal_semaphore_infos.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.signal_semaphore_infos.add(i as usize).read(),
            ));
        }
        Self {
            extensions: extensions,
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            wait_semaphore_infos: wait_semaphore_infos,
            command_buffer_infos: command_buffer_infos,
            signal_semaphore_infos: signal_semaphore_infos,
        }
    }
}
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
///Extensions for [`SubmitInfo2`]
pub enum SubmitInfo2Extension {
    #[cfg(feature = "VK_NV_win32_keyed_mutex")]
    ///Contains a type [`Win32KeyedMutexAcquireReleaseInfoNV`] for extending [`SubmitInfo2`]
    Win32KeyedMutexAcquireReleaseInfoNV(Win32KeyedMutexAcquireReleaseInfoNV),
    #[cfg(feature = "VK_KHR_win32_keyed_mutex")]
    ///Contains a type [`Win32KeyedMutexAcquireReleaseInfoKHR`] for extending [`SubmitInfo2`]
    Win32KeyedMutexAcquireReleaseInfoKHR(Win32KeyedMutexAcquireReleaseInfoKHR),
    #[cfg(feature = "VK_KHR_performance_query")]
    ///Contains a type [`PerformanceQuerySubmitInfoKHR`] for extending [`SubmitInfo2`]
    PerformanceQuerySubmitInfoKHR(PerformanceQuerySubmitInfoKHR),
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SubmitInfo2Extension {
    type LowLevel = *const crate::native::vulkan1_0::BaseInStructure;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self {
            #[cfg(feature = "VK_NV_win32_keyed_mutex")]
            Self::Win32KeyedMutexAcquireReleaseInfoNV(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::extensions::nv_win32_keyed_mutex::Win32KeyedMutexAcquireReleaseInfoNV)
                .cast(),
            #[cfg(feature = "VK_KHR_win32_keyed_mutex")]
            Self::Win32KeyedMutexAcquireReleaseInfoKHR(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::extensions::khr_win32_keyed_mutex::Win32KeyedMutexAcquireReleaseInfoKHR)
                .cast(),
            #[cfg(feature = "VK_KHR_performance_query")]
            Self::PerformanceQuerySubmitInfoKHR(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::extensions::khr_performance_query::PerformanceQuerySubmitInfoKHR)
                .cast(),
            other => unreachable!("unexpected variant {:?}", other),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SubmitInfo2Extension {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        assert!(!value.is_null());
        match (* value) . s_type { # [cfg (feature = "VK_NV_win32_keyed_mutex")] crate :: native :: vulkan1_0 :: StructureType :: Win32KeyedMutexAcquireReleaseInfoNv => Self :: Win32KeyedMutexAcquireReleaseInfoNV (Win32KeyedMutexAcquireReleaseInfoNV :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: nv_win32_keyed_mutex :: Win32KeyedMutexAcquireReleaseInfoNV > ()))) , # [cfg (feature = "VK_KHR_win32_keyed_mutex")] crate :: native :: vulkan1_0 :: StructureType :: Win32KeyedMutexAcquireReleaseInfoKhr => Self :: Win32KeyedMutexAcquireReleaseInfoKHR (Win32KeyedMutexAcquireReleaseInfoKHR :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: khr_win32_keyed_mutex :: Win32KeyedMutexAcquireReleaseInfoKHR > ()))) , # [cfg (feature = "VK_KHR_performance_query")] crate :: native :: vulkan1_0 :: StructureType :: PerformanceQuerySubmitInfoKhr => Self :: PerformanceQuerySubmitInfoKHR (PerformanceQuerySubmitInfoKHR :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: khr_performance_query :: PerformanceQuerySubmitInfoKHR > ()))) , other => panic ! ("Structure type {:?} is not a member of {}" , other , stringify ! (SubmitInfo2)) }
    }
}
#[cfg(feature = "VK_NV_win32_keyed_mutex")]
impl From<Win32KeyedMutexAcquireReleaseInfoNV> for SubmitInfo2Extension {
    fn from(ext: Win32KeyedMutexAcquireReleaseInfoNV) -> Self {
        Self::Win32KeyedMutexAcquireReleaseInfoNV(ext)
    }
}
#[cfg(feature = "VK_NV_win32_keyed_mutex")]
impl TryInto<Win32KeyedMutexAcquireReleaseInfoNV> for SubmitInfo2Extension {
    type Error = SubmitInfo2Extension;
    fn try_into(self) -> Result<Win32KeyedMutexAcquireReleaseInfoNV, Self::Error> {
        match self {
            Self::Win32KeyedMutexAcquireReleaseInfoNV(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_win32_keyed_mutex")]
impl From<Win32KeyedMutexAcquireReleaseInfoKHR> for SubmitInfo2Extension {
    fn from(ext: Win32KeyedMutexAcquireReleaseInfoKHR) -> Self {
        Self::Win32KeyedMutexAcquireReleaseInfoKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_win32_keyed_mutex")]
impl TryInto<Win32KeyedMutexAcquireReleaseInfoKHR> for SubmitInfo2Extension {
    type Error = SubmitInfo2Extension;
    fn try_into(self) -> Result<Win32KeyedMutexAcquireReleaseInfoKHR, Self::Error> {
        match self {
            Self::Win32KeyedMutexAcquireReleaseInfoKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_performance_query")]
impl From<PerformanceQuerySubmitInfoKHR> for SubmitInfo2Extension {
    fn from(ext: PerformanceQuerySubmitInfoKHR) -> Self {
        Self::PerformanceQuerySubmitInfoKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_performance_query")]
impl TryInto<PerformanceQuerySubmitInfoKHR> for SubmitInfo2Extension {
    type Error = SubmitInfo2Extension;
    fn try_into(self) -> Result<PerformanceQuerySubmitInfoKHR, Self::Error> {
        match self {
            Self::PerformanceQuerySubmitInfoKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceSynchronization2Features")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceSynchronization2Features {
    pub synchronization2: bool,
}
impl PhysicalDeviceSynchronization2Features {
    ///Get a reference to the `synchronization2` field.
    pub fn synchronization2(&self) -> &bool {
        &self.synchronization2
    }
    ///Get a mutable reference to the `synchronization2` field.
    pub fn synchronization2_mut(&mut self) -> &mut bool {
        &mut self.synchronization2
    }
    ///Sets the `synchronization2` field.
    pub fn set_synchronization2(&mut self, synchronization2: bool) -> &mut Self {
        self.synchronization2 = synchronization2;
        self
    }
    ///Sets the `synchronization2` field in a builder way.
    pub fn with_synchronization2(mut self, synchronization2: bool) -> Self {
        self.synchronization2 = synchronization2;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceSynchronization2Features {
    type LowLevel = crate::native::vulkan1_3::PhysicalDeviceSynchronization2Features;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::PhysicalDeviceSynchronization2Features {
            s_type: StructureType::PhysicalDeviceSynchronization2Features,
            p_next: std::ptr::null_mut(),
            synchronization2: self.synchronization2.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceSynchronization2Features {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            synchronization2: crate::conv::FromLowLevel::from_low_level(context, value.synchronization2),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceShaderIntegerDotProductFeatures")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceShaderIntegerDotProductFeatures {
    #[doc(alias = "shaderIntegerDotProduct")]
    pub shader_integer_dot_product: bool,
}
impl PhysicalDeviceShaderIntegerDotProductFeatures {
    ///Get a reference to the `shader_integer_dot_product` field.
    pub fn shader_integer_dot_product(&self) -> &bool {
        &self.shader_integer_dot_product
    }
    ///Get a mutable reference to the `shader_integer_dot_product` field.
    pub fn shader_integer_dot_product_mut(&mut self) -> &mut bool {
        &mut self.shader_integer_dot_product
    }
    ///Sets the `shader_integer_dot_product` field.
    pub fn set_shader_integer_dot_product(&mut self, shader_integer_dot_product: bool) -> &mut Self {
        self.shader_integer_dot_product = shader_integer_dot_product;
        self
    }
    ///Sets the `shader_integer_dot_product` field in a builder way.
    pub fn with_shader_integer_dot_product(mut self, shader_integer_dot_product: bool) -> Self {
        self.shader_integer_dot_product = shader_integer_dot_product;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceShaderIntegerDotProductFeatures {
    type LowLevel = crate::native::vulkan1_3::PhysicalDeviceShaderIntegerDotProductFeatures;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::PhysicalDeviceShaderIntegerDotProductFeatures {
            s_type: StructureType::PhysicalDeviceShaderIntegerDotProductFeatures,
            p_next: std::ptr::null_mut(),
            shader_integer_dot_product: self.shader_integer_dot_product.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceShaderIntegerDotProductFeatures {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            shader_integer_dot_product: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_integer_dot_product,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceShaderIntegerDotProductProperties")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceShaderIntegerDotProductProperties {
    #[doc(alias = "integerDotProduct8BitUnsignedAccelerated")]
    pub integer_dot_product8_bit_unsigned_accelerated: bool,
    #[doc(alias = "integerDotProduct8BitSignedAccelerated")]
    pub integer_dot_product8_bit_signed_accelerated: bool,
    #[doc(alias = "integerDotProduct8BitMixedSignednessAccelerated")]
    pub integer_dot_product8_bit_mixed_signedness_accelerated: bool,
    #[doc(alias = "integerDotProduct4x8BitPackedUnsignedAccelerated")]
    pub integer_dot_product4x8_bit_packed_unsigned_accelerated: bool,
    #[doc(alias = "integerDotProduct4x8BitPackedSignedAccelerated")]
    pub integer_dot_product4x8_bit_packed_signed_accelerated: bool,
    #[doc(alias = "integerDotProduct4x8BitPackedMixedSignednessAccelerated")]
    pub integer_dot_product4x8_bit_packed_mixed_signedness_accelerated: bool,
    #[doc(alias = "integerDotProduct16BitUnsignedAccelerated")]
    pub integer_dot_product16_bit_unsigned_accelerated: bool,
    #[doc(alias = "integerDotProduct16BitSignedAccelerated")]
    pub integer_dot_product16_bit_signed_accelerated: bool,
    #[doc(alias = "integerDotProduct16BitMixedSignednessAccelerated")]
    pub integer_dot_product16_bit_mixed_signedness_accelerated: bool,
    #[doc(alias = "integerDotProduct32BitUnsignedAccelerated")]
    pub integer_dot_product32_bit_unsigned_accelerated: bool,
    #[doc(alias = "integerDotProduct32BitSignedAccelerated")]
    pub integer_dot_product32_bit_signed_accelerated: bool,
    #[doc(alias = "integerDotProduct32BitMixedSignednessAccelerated")]
    pub integer_dot_product32_bit_mixed_signedness_accelerated: bool,
    #[doc(alias = "integerDotProduct64BitUnsignedAccelerated")]
    pub integer_dot_product64_bit_unsigned_accelerated: bool,
    #[doc(alias = "integerDotProduct64BitSignedAccelerated")]
    pub integer_dot_product64_bit_signed_accelerated: bool,
    #[doc(alias = "integerDotProduct64BitMixedSignednessAccelerated")]
    pub integer_dot_product64_bit_mixed_signedness_accelerated: bool,
    #[doc(alias = "integerDotProductAccumulatingSaturating8BitUnsignedAccelerated")]
    pub integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated: bool,
    #[doc(alias = "integerDotProductAccumulatingSaturating8BitSignedAccelerated")]
    pub integer_dot_product_accumulating_saturating8_bit_signed_accelerated: bool,
    #[doc(alias = "integerDotProductAccumulatingSaturating8BitMixedSignednessAccelerated")]
    pub integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated: bool,
    #[doc(alias = "integerDotProductAccumulatingSaturating4x8BitPackedUnsignedAccelerated")]
    pub integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated: bool,
    #[doc(alias = "integerDotProductAccumulatingSaturating4x8BitPackedSignedAccelerated")]
    pub integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated: bool,
    #[doc(alias = "integerDotProductAccumulatingSaturating4x8BitPackedMixedSignednessAccelerated")]
    pub integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated: bool,
    #[doc(alias = "integerDotProductAccumulatingSaturating16BitUnsignedAccelerated")]
    pub integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated: bool,
    #[doc(alias = "integerDotProductAccumulatingSaturating16BitSignedAccelerated")]
    pub integer_dot_product_accumulating_saturating16_bit_signed_accelerated: bool,
    #[doc(alias = "integerDotProductAccumulatingSaturating16BitMixedSignednessAccelerated")]
    pub integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated: bool,
    #[doc(alias = "integerDotProductAccumulatingSaturating32BitUnsignedAccelerated")]
    pub integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated: bool,
    #[doc(alias = "integerDotProductAccumulatingSaturating32BitSignedAccelerated")]
    pub integer_dot_product_accumulating_saturating32_bit_signed_accelerated: bool,
    #[doc(alias = "integerDotProductAccumulatingSaturating32BitMixedSignednessAccelerated")]
    pub integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated: bool,
    #[doc(alias = "integerDotProductAccumulatingSaturating64BitUnsignedAccelerated")]
    pub integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated: bool,
    #[doc(alias = "integerDotProductAccumulatingSaturating64BitSignedAccelerated")]
    pub integer_dot_product_accumulating_saturating64_bit_signed_accelerated: bool,
    #[doc(alias = "integerDotProductAccumulatingSaturating64BitMixedSignednessAccelerated")]
    pub integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated: bool,
}
impl PhysicalDeviceShaderIntegerDotProductProperties {
    ///Get a reference to the `integer_dot_product8_bit_unsigned_accelerated` field.
    pub fn integer_dot_product8_bit_unsigned_accelerated(&self) -> &bool {
        &self.integer_dot_product8_bit_unsigned_accelerated
    }
    ///Get a reference to the `integer_dot_product8_bit_signed_accelerated` field.
    pub fn integer_dot_product8_bit_signed_accelerated(&self) -> &bool {
        &self.integer_dot_product8_bit_signed_accelerated
    }
    ///Get a reference to the `integer_dot_product8_bit_mixed_signedness_accelerated` field.
    pub fn integer_dot_product8_bit_mixed_signedness_accelerated(&self) -> &bool {
        &self.integer_dot_product8_bit_mixed_signedness_accelerated
    }
    ///Get a reference to the `integer_dot_product4x8_bit_packed_unsigned_accelerated` field.
    pub fn integer_dot_product4x8_bit_packed_unsigned_accelerated(&self) -> &bool {
        &self.integer_dot_product4x8_bit_packed_unsigned_accelerated
    }
    ///Get a reference to the `integer_dot_product4x8_bit_packed_signed_accelerated` field.
    pub fn integer_dot_product4x8_bit_packed_signed_accelerated(&self) -> &bool {
        &self.integer_dot_product4x8_bit_packed_signed_accelerated
    }
    ///Get a reference to the `integer_dot_product4x8_bit_packed_mixed_signedness_accelerated`
    /// field.
    pub fn integer_dot_product4x8_bit_packed_mixed_signedness_accelerated(&self) -> &bool {
        &self.integer_dot_product4x8_bit_packed_mixed_signedness_accelerated
    }
    ///Get a reference to the `integer_dot_product16_bit_unsigned_accelerated` field.
    pub fn integer_dot_product16_bit_unsigned_accelerated(&self) -> &bool {
        &self.integer_dot_product16_bit_unsigned_accelerated
    }
    ///Get a reference to the `integer_dot_product16_bit_signed_accelerated` field.
    pub fn integer_dot_product16_bit_signed_accelerated(&self) -> &bool {
        &self.integer_dot_product16_bit_signed_accelerated
    }
    ///Get a reference to the `integer_dot_product16_bit_mixed_signedness_accelerated` field.
    pub fn integer_dot_product16_bit_mixed_signedness_accelerated(&self) -> &bool {
        &self.integer_dot_product16_bit_mixed_signedness_accelerated
    }
    ///Get a reference to the `integer_dot_product32_bit_unsigned_accelerated` field.
    pub fn integer_dot_product32_bit_unsigned_accelerated(&self) -> &bool {
        &self.integer_dot_product32_bit_unsigned_accelerated
    }
    ///Get a reference to the `integer_dot_product32_bit_signed_accelerated` field.
    pub fn integer_dot_product32_bit_signed_accelerated(&self) -> &bool {
        &self.integer_dot_product32_bit_signed_accelerated
    }
    ///Get a reference to the `integer_dot_product32_bit_mixed_signedness_accelerated` field.
    pub fn integer_dot_product32_bit_mixed_signedness_accelerated(&self) -> &bool {
        &self.integer_dot_product32_bit_mixed_signedness_accelerated
    }
    ///Get a reference to the `integer_dot_product64_bit_unsigned_accelerated` field.
    pub fn integer_dot_product64_bit_unsigned_accelerated(&self) -> &bool {
        &self.integer_dot_product64_bit_unsigned_accelerated
    }
    ///Get a reference to the `integer_dot_product64_bit_signed_accelerated` field.
    pub fn integer_dot_product64_bit_signed_accelerated(&self) -> &bool {
        &self.integer_dot_product64_bit_signed_accelerated
    }
    ///Get a reference to the `integer_dot_product64_bit_mixed_signedness_accelerated` field.
    pub fn integer_dot_product64_bit_mixed_signedness_accelerated(&self) -> &bool {
        &self.integer_dot_product64_bit_mixed_signedness_accelerated
    }
    ///Get a reference to the
    /// `integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated` field.
    pub fn integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated(&self) -> &bool {
        &self.integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated
    }
    ///Get a reference to the `integer_dot_product_accumulating_saturating8_bit_signed_accelerated`
    /// field.
    pub fn integer_dot_product_accumulating_saturating8_bit_signed_accelerated(&self) -> &bool {
        &self.integer_dot_product_accumulating_saturating8_bit_signed_accelerated
    }
    ///Get a reference to the
    /// `integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated` field.
    pub fn integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated(&self) -> &bool {
        &self.integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated
    }
    ///Get a reference to the
    /// `integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated` field.
    pub fn integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated(&self) -> &bool {
        &self.integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated
    }
    ///Get a reference to the
    /// `integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated` field.
    pub fn integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated(&self) -> &bool {
        &self.integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated
    }
    ///Get a reference to the
    /// `integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated`
    /// field.
    pub fn integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated(&self) -> &bool {
        &self.integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated
    }
    ///Get a reference to the
    /// `integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated` field.
    pub fn integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated(&self) -> &bool {
        &self.integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated
    }
    ///Get a reference to the
    /// `integer_dot_product_accumulating_saturating16_bit_signed_accelerated` field.
    pub fn integer_dot_product_accumulating_saturating16_bit_signed_accelerated(&self) -> &bool {
        &self.integer_dot_product_accumulating_saturating16_bit_signed_accelerated
    }
    ///Get a reference to the
    /// `integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated` field.
    pub fn integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated(&self) -> &bool {
        &self.integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated
    }
    ///Get a reference to the
    /// `integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated` field.
    pub fn integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated(&self) -> &bool {
        &self.integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated
    }
    ///Get a reference to the
    /// `integer_dot_product_accumulating_saturating32_bit_signed_accelerated` field.
    pub fn integer_dot_product_accumulating_saturating32_bit_signed_accelerated(&self) -> &bool {
        &self.integer_dot_product_accumulating_saturating32_bit_signed_accelerated
    }
    ///Get a reference to the
    /// `integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated` field.
    pub fn integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated(&self) -> &bool {
        &self.integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated
    }
    ///Get a reference to the
    /// `integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated` field.
    pub fn integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated(&self) -> &bool {
        &self.integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated
    }
    ///Get a reference to the
    /// `integer_dot_product_accumulating_saturating64_bit_signed_accelerated` field.
    pub fn integer_dot_product_accumulating_saturating64_bit_signed_accelerated(&self) -> &bool {
        &self.integer_dot_product_accumulating_saturating64_bit_signed_accelerated
    }
    ///Get a reference to the
    /// `integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated` field.
    pub fn integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated(&self) -> &bool {
        &self.integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceShaderIntegerDotProductProperties {
    type LowLevel = crate::native::vulkan1_3::PhysicalDeviceShaderIntegerDotProductProperties;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::PhysicalDeviceShaderIntegerDotProductProperties {
            s_type: StructureType::PhysicalDeviceShaderIntegerDotProductProperties,
            p_next: std::ptr::null_mut(),
            integer_dot_product8_bit_unsigned_accelerated: self
                .integer_dot_product8_bit_unsigned_accelerated
                .into_low_level(context, bump),
            integer_dot_product8_bit_signed_accelerated: self
                .integer_dot_product8_bit_signed_accelerated
                .into_low_level(context, bump),
            integer_dot_product8_bit_mixed_signedness_accelerated: self
                .integer_dot_product8_bit_mixed_signedness_accelerated
                .into_low_level(context, bump),
            integer_dot_product4x8_bit_packed_unsigned_accelerated: self
                .integer_dot_product4x8_bit_packed_unsigned_accelerated
                .into_low_level(context, bump),
            integer_dot_product4x8_bit_packed_signed_accelerated: self
                .integer_dot_product4x8_bit_packed_signed_accelerated
                .into_low_level(context, bump),
            integer_dot_product4x8_bit_packed_mixed_signedness_accelerated: self
                .integer_dot_product4x8_bit_packed_mixed_signedness_accelerated
                .into_low_level(context, bump),
            integer_dot_product16_bit_unsigned_accelerated: self
                .integer_dot_product16_bit_unsigned_accelerated
                .into_low_level(context, bump),
            integer_dot_product16_bit_signed_accelerated: self
                .integer_dot_product16_bit_signed_accelerated
                .into_low_level(context, bump),
            integer_dot_product16_bit_mixed_signedness_accelerated: self
                .integer_dot_product16_bit_mixed_signedness_accelerated
                .into_low_level(context, bump),
            integer_dot_product32_bit_unsigned_accelerated: self
                .integer_dot_product32_bit_unsigned_accelerated
                .into_low_level(context, bump),
            integer_dot_product32_bit_signed_accelerated: self
                .integer_dot_product32_bit_signed_accelerated
                .into_low_level(context, bump),
            integer_dot_product32_bit_mixed_signedness_accelerated: self
                .integer_dot_product32_bit_mixed_signedness_accelerated
                .into_low_level(context, bump),
            integer_dot_product64_bit_unsigned_accelerated: self
                .integer_dot_product64_bit_unsigned_accelerated
                .into_low_level(context, bump),
            integer_dot_product64_bit_signed_accelerated: self
                .integer_dot_product64_bit_signed_accelerated
                .into_low_level(context, bump),
            integer_dot_product64_bit_mixed_signedness_accelerated: self
                .integer_dot_product64_bit_mixed_signedness_accelerated
                .into_low_level(context, bump),
            integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated: self
                .integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated
                .into_low_level(context, bump),
            integer_dot_product_accumulating_saturating8_bit_signed_accelerated: self
                .integer_dot_product_accumulating_saturating8_bit_signed_accelerated
                .into_low_level(context, bump),
            integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated: self
                .integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated
                .into_low_level(context, bump),
            integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated: self
                .integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated
                .into_low_level(context, bump),
            integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated: self
                .integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated
                .into_low_level(context, bump),
            integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated: self
                .integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated
                .into_low_level(context, bump),
            integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated: self
                .integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated
                .into_low_level(context, bump),
            integer_dot_product_accumulating_saturating16_bit_signed_accelerated: self
                .integer_dot_product_accumulating_saturating16_bit_signed_accelerated
                .into_low_level(context, bump),
            integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated: self
                .integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated
                .into_low_level(context, bump),
            integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated: self
                .integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated
                .into_low_level(context, bump),
            integer_dot_product_accumulating_saturating32_bit_signed_accelerated: self
                .integer_dot_product_accumulating_saturating32_bit_signed_accelerated
                .into_low_level(context, bump),
            integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated: self
                .integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated
                .into_low_level(context, bump),
            integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated: self
                .integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated
                .into_low_level(context, bump),
            integer_dot_product_accumulating_saturating64_bit_signed_accelerated: self
                .integer_dot_product_accumulating_saturating64_bit_signed_accelerated
                .into_low_level(context, bump),
            integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated: self
                .integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated
                .into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceShaderIntegerDotProductProperties {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            integer_dot_product8_bit_unsigned_accelerated: crate::conv::FromLowLevel::from_low_level(
                context,
                value.integer_dot_product8_bit_unsigned_accelerated,
            ),
            integer_dot_product8_bit_signed_accelerated: crate::conv::FromLowLevel::from_low_level(
                context,
                value.integer_dot_product8_bit_signed_accelerated,
            ),
            integer_dot_product8_bit_mixed_signedness_accelerated: crate::conv::FromLowLevel::from_low_level(
                context,
                value.integer_dot_product8_bit_mixed_signedness_accelerated,
            ),
            integer_dot_product4x8_bit_packed_unsigned_accelerated: crate::conv::FromLowLevel::from_low_level(
                context,
                value.integer_dot_product4x8_bit_packed_unsigned_accelerated,
            ),
            integer_dot_product4x8_bit_packed_signed_accelerated: crate::conv::FromLowLevel::from_low_level(
                context,
                value.integer_dot_product4x8_bit_packed_signed_accelerated,
            ),
            integer_dot_product4x8_bit_packed_mixed_signedness_accelerated: crate::conv::FromLowLevel::from_low_level(
                context,
                value.integer_dot_product4x8_bit_packed_mixed_signedness_accelerated,
            ),
            integer_dot_product16_bit_unsigned_accelerated: crate::conv::FromLowLevel::from_low_level(
                context,
                value.integer_dot_product16_bit_unsigned_accelerated,
            ),
            integer_dot_product16_bit_signed_accelerated: crate::conv::FromLowLevel::from_low_level(
                context,
                value.integer_dot_product16_bit_signed_accelerated,
            ),
            integer_dot_product16_bit_mixed_signedness_accelerated: crate::conv::FromLowLevel::from_low_level(
                context,
                value.integer_dot_product16_bit_mixed_signedness_accelerated,
            ),
            integer_dot_product32_bit_unsigned_accelerated: crate::conv::FromLowLevel::from_low_level(
                context,
                value.integer_dot_product32_bit_unsigned_accelerated,
            ),
            integer_dot_product32_bit_signed_accelerated: crate::conv::FromLowLevel::from_low_level(
                context,
                value.integer_dot_product32_bit_signed_accelerated,
            ),
            integer_dot_product32_bit_mixed_signedness_accelerated: crate::conv::FromLowLevel::from_low_level(
                context,
                value.integer_dot_product32_bit_mixed_signedness_accelerated,
            ),
            integer_dot_product64_bit_unsigned_accelerated: crate::conv::FromLowLevel::from_low_level(
                context,
                value.integer_dot_product64_bit_unsigned_accelerated,
            ),
            integer_dot_product64_bit_signed_accelerated: crate::conv::FromLowLevel::from_low_level(
                context,
                value.integer_dot_product64_bit_signed_accelerated,
            ),
            integer_dot_product64_bit_mixed_signedness_accelerated: crate::conv::FromLowLevel::from_low_level(
                context,
                value.integer_dot_product64_bit_mixed_signedness_accelerated,
            ),
            integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated:
                crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated,
                ),
            integer_dot_product_accumulating_saturating8_bit_signed_accelerated:
                crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.integer_dot_product_accumulating_saturating8_bit_signed_accelerated,
                ),
            integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated:
                crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated,
                ),
            integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated:
                crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated,
                ),
            integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated:
                crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated,
                ),
            integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated:
                crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated,
                ),
            integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated:
                crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated,
                ),
            integer_dot_product_accumulating_saturating16_bit_signed_accelerated:
                crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.integer_dot_product_accumulating_saturating16_bit_signed_accelerated,
                ),
            integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated:
                crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated,
                ),
            integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated:
                crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated,
                ),
            integer_dot_product_accumulating_saturating32_bit_signed_accelerated:
                crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.integer_dot_product_accumulating_saturating32_bit_signed_accelerated,
                ),
            integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated:
                crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated,
                ),
            integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated:
                crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated,
                ),
            integer_dot_product_accumulating_saturating64_bit_signed_accelerated:
                crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.integer_dot_product_accumulating_saturating64_bit_signed_accelerated,
                ),
            integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated:
                crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated,
                ),
        }
    }
}
#[doc(alias = "VkFormatProperties3")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FormatProperties3 {
    #[doc(alias = "linearTilingFeatures")]
    pub linear_tiling_features: FormatFeatureFlags2,
    #[doc(alias = "optimalTilingFeatures")]
    pub optimal_tiling_features: FormatFeatureFlags2,
    #[doc(alias = "bufferFeatures")]
    pub buffer_features: FormatFeatureFlags2,
}
impl FormatProperties3 {
    ///Get a reference to the `linear_tiling_features` field.
    pub fn linear_tiling_features(&self) -> FormatFeatureFlags2 {
        self.linear_tiling_features
    }
    ///Get a reference to the `optimal_tiling_features` field.
    pub fn optimal_tiling_features(&self) -> FormatFeatureFlags2 {
        self.optimal_tiling_features
    }
    ///Get a reference to the `buffer_features` field.
    pub fn buffer_features(&self) -> FormatFeatureFlags2 {
        self.buffer_features
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for FormatProperties3 {
    type LowLevel = crate::native::vulkan1_3::FormatProperties3;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::FormatProperties3 {
            s_type: StructureType::FormatProperties3,
            p_next: std::ptr::null_mut(),
            linear_tiling_features: self.linear_tiling_features.into_low_level(context, bump),
            optimal_tiling_features: self.optimal_tiling_features.into_low_level(context, bump),
            buffer_features: self.buffer_features.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for FormatProperties3 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            linear_tiling_features: crate::conv::FromLowLevel::from_low_level(context, value.linear_tiling_features),
            optimal_tiling_features: crate::conv::FromLowLevel::from_low_level(context, value.optimal_tiling_features),
            buffer_features: crate::conv::FromLowLevel::from_low_level(context, value.buffer_features),
        }
    }
}
#[doc(alias = "VkPipelineRenderingCreateInfo")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineRenderingCreateInfo {
    #[doc(alias = "viewMask")]
    pub view_mask: u32,
    #[doc(alias = "pColorAttachmentFormats")]
    pub color_attachment_formats: SmallVec<[Format; 8]>,
    #[doc(alias = "depthAttachmentFormat")]
    pub depth_attachment_format: Format,
    #[doc(alias = "stencilAttachmentFormat")]
    pub stencil_attachment_format: Format,
}
impl PipelineRenderingCreateInfo {
    ///Get a reference to the `view_mask` field.
    pub fn view_mask(&self) -> u32 {
        self.view_mask
    }
    ///Get a reference to the `color_attachment_formats` field.
    pub fn color_attachment_formats(&self) -> &SmallVec<[Format; 8]> {
        &self.color_attachment_formats
    }
    ///Get a reference to the `depth_attachment_format` field.
    pub fn depth_attachment_format(&self) -> Format {
        self.depth_attachment_format
    }
    ///Get a reference to the `stencil_attachment_format` field.
    pub fn stencil_attachment_format(&self) -> Format {
        self.stencil_attachment_format
    }
    ///Get a mutable reference to the `view_mask` field.
    pub fn view_mask_mut(&mut self) -> &mut u32 {
        &mut self.view_mask
    }
    ///Get a mutable reference to the `color_attachment_formats` field.
    pub fn color_attachment_formats_mut(&mut self) -> &mut SmallVec<[Format; 8]> {
        &mut self.color_attachment_formats
    }
    ///Get a mutable reference to the `depth_attachment_format` field.
    pub fn depth_attachment_format_mut(&mut self) -> &mut Format {
        &mut self.depth_attachment_format
    }
    ///Get a mutable reference to the `stencil_attachment_format` field.
    pub fn stencil_attachment_format_mut(&mut self) -> &mut Format {
        &mut self.stencil_attachment_format
    }
    ///Sets the `view_mask` field.
    pub fn set_view_mask(&mut self, view_mask: u32) -> &mut Self {
        self.view_mask = view_mask;
        self
    }
    ///Sets the `color_attachment_formats` field.
    pub fn set_color_attachment_formats(&mut self, color_attachment_formats: SmallVec<[Format; 8]>) -> &mut Self {
        self.color_attachment_formats = color_attachment_formats;
        self
    }
    ///Sets the `depth_attachment_format` field.
    pub fn set_depth_attachment_format(&mut self, depth_attachment_format: Format) -> &mut Self {
        self.depth_attachment_format = depth_attachment_format;
        self
    }
    ///Sets the `stencil_attachment_format` field.
    pub fn set_stencil_attachment_format(&mut self, stencil_attachment_format: Format) -> &mut Self {
        self.stencil_attachment_format = stencil_attachment_format;
        self
    }
    ///Sets the `view_mask` field in a builder way.
    pub fn with_view_mask(mut self, view_mask: u32) -> Self {
        self.view_mask = view_mask;
        self
    }
    ///Sets the `color_attachment_formats` field in a builder way.
    pub fn with_color_attachment_formats(mut self, color_attachment_formats: SmallVec<[Format; 8]>) -> Self {
        self.color_attachment_formats = color_attachment_formats;
        self
    }
    ///Sets the `depth_attachment_format` field in a builder way.
    pub fn with_depth_attachment_format(mut self, depth_attachment_format: Format) -> Self {
        self.depth_attachment_format = depth_attachment_format;
        self
    }
    ///Sets the `stencil_attachment_format` field in a builder way.
    pub fn with_stencil_attachment_format(mut self, stencil_attachment_format: Format) -> Self {
        self.stencil_attachment_format = stencil_attachment_format;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineRenderingCreateInfo {
    type LowLevel = crate::native::vulkan1_3::PipelineRenderingCreateInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_color_attachment_formats = self.color_attachment_formats.len() as u32;
        let color_attachment_formats = bump
            .alloc_slice_fill_iter(
                self.color_attachment_formats
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_ptr()
            .cast();
        crate::native::vulkan1_3::PipelineRenderingCreateInfo {
            s_type: StructureType::PipelineRenderingCreateInfo,
            p_next: std::ptr::null(),
            view_mask: self.view_mask.into_low_level(context, bump),
            color_attachment_count: len_color_attachment_formats,
            color_attachment_formats: color_attachment_formats,
            depth_attachment_format: self.depth_attachment_format.into_low_level(context, bump),
            stencil_attachment_format: self.stencil_attachment_format.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineRenderingCreateInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let color_attachment_formats_len = value.color_attachment_count;
        let mut color_attachment_formats = SmallVec::with_capacity(color_attachment_formats_len as usize);
        for i in 0..color_attachment_formats_len {
            color_attachment_formats.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.color_attachment_formats.add(i as usize).read(),
            ));
        }
        Self {
            view_mask: crate::conv::FromLowLevel::from_low_level(context, value.view_mask),
            color_attachment_formats: color_attachment_formats,
            depth_attachment_format: crate::conv::FromLowLevel::from_low_level(context, value.depth_attachment_format),
            stencil_attachment_format: crate::conv::FromLowLevel::from_low_level(
                context,
                value.stencil_attachment_format,
            ),
        }
    }
}
#[doc(alias = "VkRenderingInfo")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RenderingInfo {
    #[doc(alias = "pNext")]
    pub extensions: SmallVec<[RenderingInfoExtension; 1]>,
    pub flags: RenderingFlags,
    #[doc(alias = "renderArea")]
    pub render_area: Rect2D,
    #[doc(alias = "layerCount")]
    pub layer_count: u32,
    #[doc(alias = "viewMask")]
    pub view_mask: u32,
    #[doc(alias = "pColorAttachments")]
    pub color_attachments: SmallVec<[RenderingAttachmentInfo; 8]>,
    #[doc(alias = "pDepthAttachment")]
    pub depth_attachment: Option<RenderingAttachmentInfo>,
    #[doc(alias = "pStencilAttachment")]
    pub stencil_attachment: Option<RenderingAttachmentInfo>,
}
impl RenderingInfo {
    ///Adds an extension to the struct
    pub fn with_extension(mut self, ext: impl Into<RenderingInfoExtension>) -> Self {
        self.extensions.push(ext.into());
        self
    }
    ///Get a reference to the `extensions` field.
    pub fn extensions(&self) -> &SmallVec<[RenderingInfoExtension; 1]> {
        &self.extensions
    }
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> RenderingFlags {
        self.flags
    }
    ///Get a reference to the `render_area` field.
    pub fn render_area(&self) -> Rect2D {
        self.render_area
    }
    ///Get a reference to the `layer_count` field.
    pub fn layer_count(&self) -> u32 {
        self.layer_count
    }
    ///Get a reference to the `view_mask` field.
    pub fn view_mask(&self) -> u32 {
        self.view_mask
    }
    ///Get a reference to the `color_attachments` field.
    pub fn color_attachments(&self) -> &SmallVec<[RenderingAttachmentInfo; 8]> {
        &self.color_attachments
    }
    ///Get a reference to the `depth_attachment` field.
    pub fn depth_attachment(&self) -> &Option<RenderingAttachmentInfo> {
        &self.depth_attachment
    }
    ///Get a reference to the `stencil_attachment` field.
    pub fn stencil_attachment(&self) -> &Option<RenderingAttachmentInfo> {
        &self.stencil_attachment
    }
    ///Get a mutable reference to the `extensions` field.
    pub fn extensions_mut(&mut self) -> &mut SmallVec<[RenderingInfoExtension; 1]> {
        &mut self.extensions
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut RenderingFlags {
        &mut self.flags
    }
    ///Get a mutable reference to the `render_area` field.
    pub fn render_area_mut(&mut self) -> &mut Rect2D {
        &mut self.render_area
    }
    ///Get a mutable reference to the `layer_count` field.
    pub fn layer_count_mut(&mut self) -> &mut u32 {
        &mut self.layer_count
    }
    ///Get a mutable reference to the `view_mask` field.
    pub fn view_mask_mut(&mut self) -> &mut u32 {
        &mut self.view_mask
    }
    ///Get a mutable reference to the `color_attachments` field.
    pub fn color_attachments_mut(&mut self) -> &mut SmallVec<[RenderingAttachmentInfo; 8]> {
        &mut self.color_attachments
    }
    ///Get a mutable reference to the `depth_attachment` field.
    pub fn depth_attachment_mut(&mut self) -> &mut Option<RenderingAttachmentInfo> {
        &mut self.depth_attachment
    }
    ///Get a mutable reference to the `stencil_attachment` field.
    pub fn stencil_attachment_mut(&mut self) -> &mut Option<RenderingAttachmentInfo> {
        &mut self.stencil_attachment
    }
    ///Sets the `extensions` field.
    pub fn set_extensions(&mut self, extensions: SmallVec<[RenderingInfoExtension; 1]>) -> &mut Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: RenderingFlags) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `render_area` field.
    pub fn set_render_area(&mut self, render_area: Rect2D) -> &mut Self {
        self.render_area = render_area;
        self
    }
    ///Sets the `layer_count` field.
    pub fn set_layer_count(&mut self, layer_count: u32) -> &mut Self {
        self.layer_count = layer_count;
        self
    }
    ///Sets the `view_mask` field.
    pub fn set_view_mask(&mut self, view_mask: u32) -> &mut Self {
        self.view_mask = view_mask;
        self
    }
    ///Sets the `color_attachments` field.
    pub fn set_color_attachments(&mut self, color_attachments: SmallVec<[RenderingAttachmentInfo; 8]>) -> &mut Self {
        self.color_attachments = color_attachments;
        self
    }
    ///Sets the `depth_attachment` field.
    pub fn set_depth_attachment(&mut self, depth_attachment: Option<RenderingAttachmentInfo>) -> &mut Self {
        self.depth_attachment = depth_attachment;
        self
    }
    ///Sets the `stencil_attachment` field.
    pub fn set_stencil_attachment(&mut self, stencil_attachment: Option<RenderingAttachmentInfo>) -> &mut Self {
        self.stencil_attachment = stencil_attachment;
        self
    }
    ///Sets the `extensions` field in a builder way.
    pub fn with_extensions(mut self, extensions: SmallVec<[RenderingInfoExtension; 1]>) -> Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: RenderingFlags) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `render_area` field in a builder way.
    pub fn with_render_area(mut self, render_area: Rect2D) -> Self {
        self.render_area = render_area;
        self
    }
    ///Sets the `layer_count` field in a builder way.
    pub fn with_layer_count(mut self, layer_count: u32) -> Self {
        self.layer_count = layer_count;
        self
    }
    ///Sets the `view_mask` field in a builder way.
    pub fn with_view_mask(mut self, view_mask: u32) -> Self {
        self.view_mask = view_mask;
        self
    }
    ///Sets the `color_attachments` field in a builder way.
    pub fn with_color_attachments(mut self, color_attachments: SmallVec<[RenderingAttachmentInfo; 8]>) -> Self {
        self.color_attachments = color_attachments;
        self
    }
    ///Sets the `depth_attachment` field in a builder way.
    pub fn with_depth_attachment(mut self, depth_attachment: Option<RenderingAttachmentInfo>) -> Self {
        self.depth_attachment = depth_attachment;
        self
    }
    ///Sets the `stencil_attachment` field in a builder way.
    pub fn with_stencil_attachment(mut self, stencil_attachment: Option<RenderingAttachmentInfo>) -> Self {
        self.stencil_attachment = stencil_attachment;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for RenderingInfo {
    type LowLevel = crate::native::vulkan1_3::RenderingInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let mut next = std::ptr::null();
        let mut extensions = self.extensions.iter();
        while let Some(ext) = extensions.next() {
            let ext = ext.into_low_level(context, bump);
            (*ext).next = next;
            next = ext;
        }
        let len_color_attachments = self.color_attachments.len() as u32;
        let color_attachments = bump
            .alloc_slice_fill_iter(self.color_attachments.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::vulkan1_3::RenderingInfo {
            s_type: StructureType::RenderingInfo,
            p_next: next,
            flags: self.flags.into_low_level(context, bump),
            render_area: self.render_area.into_low_level(context, bump),
            layer_count: self.layer_count.into_low_level(context, bump),
            view_mask: self.view_mask.into_low_level(context, bump),
            color_attachment_count: len_color_attachments,
            color_attachments: color_attachments,
            depth_attachment: self
                .depth_attachment
                .as_ref()
                .map(|v| bump.alloc(v.into_low_level(context, bump)) as *const _)
                .unwrap_or_else(std::ptr::null),
            stencil_attachment: self
                .stencil_attachment
                .as_ref()
                .map(|v| bump.alloc(v.into_low_level(context, bump)) as *const _)
                .unwrap_or_else(std::ptr::null),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for RenderingInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let mut next = value.p_next;
        let mut extensions = SmallVec::new();
        while !next.is_null() {
            extensions.push(crate::conv::FromLowLevel::from_low_level(context, next));
            next = std::ptr::read(next).next;
        }
        let color_attachments_len = value.color_attachment_count;
        let mut color_attachments = SmallVec::with_capacity(color_attachments_len as usize);
        for i in 0..color_attachments_len {
            color_attachments.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.color_attachments.add(i as usize).read(),
            ));
        }
        Self {
            extensions: extensions,
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            render_area: crate::conv::FromLowLevel::from_low_level(context, value.render_area),
            layer_count: crate::conv::FromLowLevel::from_low_level(context, value.layer_count),
            view_mask: crate::conv::FromLowLevel::from_low_level(context, value.view_mask),
            color_attachments: color_attachments,
            depth_attachment: crate::conv::FromLowLevel::from_low_level(context, *value.depth_attachment),
            stencil_attachment: crate::conv::FromLowLevel::from_low_level(context, *value.stencil_attachment),
        }
    }
}
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
///Extensions for [`RenderingInfo`]
pub enum RenderingInfoExtension {
    ///Contains a type [`DeviceGroupRenderPassBeginInfo`] for extending [`RenderingInfo`]
    DeviceGroupRenderPassBeginInfo(DeviceGroupRenderPassBeginInfo),
    #[cfg(feature = "VK_KHR_dynamic_rendering")]
    ///Contains a type [`RenderingFragmentShadingRateAttachmentInfoKHR`] for extending
    /// [`RenderingInfo`]
    RenderingFragmentShadingRateAttachmentInfoKHR(RenderingFragmentShadingRateAttachmentInfoKHR),
    #[cfg(feature = "VK_KHR_dynamic_rendering")]
    ///Contains a type [`RenderingFragmentDensityMapAttachmentInfoEXT`] for extending
    /// [`RenderingInfo`]
    RenderingFragmentDensityMapAttachmentInfoEXT(RenderingFragmentDensityMapAttachmentInfoEXT),
    #[cfg(feature = "VK_KHR_dynamic_rendering")]
    ///Contains a type [`MultiviewPerViewAttributesInfoNVX`] for extending [`RenderingInfo`]
    MultiviewPerViewAttributesInfoNVX(MultiviewPerViewAttributesInfoNVX),
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for RenderingInfoExtension {
    type LowLevel = *const crate::native::vulkan1_0::BaseInStructure;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self { Self :: DeviceGroupRenderPassBeginInfo (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_1 :: DeviceGroupRenderPassBeginInfo) . cast () , # [cfg (feature = "VK_KHR_dynamic_rendering")] Self :: RenderingFragmentShadingRateAttachmentInfoKHR (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: khr_dynamic_rendering :: RenderingFragmentShadingRateAttachmentInfoKHR) . cast () , # [cfg (feature = "VK_KHR_dynamic_rendering")] Self :: RenderingFragmentDensityMapAttachmentInfoEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: khr_dynamic_rendering :: RenderingFragmentDensityMapAttachmentInfoEXT) . cast () , # [cfg (feature = "VK_KHR_dynamic_rendering")] Self :: MultiviewPerViewAttributesInfoNVX (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: khr_dynamic_rendering :: MultiviewPerViewAttributesInfoNVX) . cast () , other => unreachable ! ("unexpected variant {:?}" , other) }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for RenderingInfoExtension {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        assert!(!value.is_null());
        match (* value) . s_type { crate :: native :: vulkan1_0 :: StructureType :: DeviceGroupRenderPassBeginInfo => Self :: DeviceGroupRenderPassBeginInfo (DeviceGroupRenderPassBeginInfo :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_1 :: DeviceGroupRenderPassBeginInfo > ()))) , # [cfg (feature = "VK_KHR_dynamic_rendering")] crate :: native :: vulkan1_0 :: StructureType :: RenderingFragmentShadingRateAttachmentInfoKhr => Self :: RenderingFragmentShadingRateAttachmentInfoKHR (RenderingFragmentShadingRateAttachmentInfoKHR :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: khr_dynamic_rendering :: RenderingFragmentShadingRateAttachmentInfoKHR > ()))) , # [cfg (feature = "VK_KHR_dynamic_rendering")] crate :: native :: vulkan1_0 :: StructureType :: RenderingFragmentDensityMapAttachmentInfoExt => Self :: RenderingFragmentDensityMapAttachmentInfoEXT (RenderingFragmentDensityMapAttachmentInfoEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: khr_dynamic_rendering :: RenderingFragmentDensityMapAttachmentInfoEXT > ()))) , # [cfg (feature = "VK_KHR_dynamic_rendering")] crate :: native :: vulkan1_0 :: StructureType :: MultiviewPerViewAttributesInfoNvx => Self :: MultiviewPerViewAttributesInfoNVX (MultiviewPerViewAttributesInfoNVX :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: khr_dynamic_rendering :: MultiviewPerViewAttributesInfoNVX > ()))) , other => panic ! ("Structure type {:?} is not a member of {}" , other , stringify ! (RenderingInfo)) }
    }
}
impl From<DeviceGroupRenderPassBeginInfo> for RenderingInfoExtension {
    fn from(ext: DeviceGroupRenderPassBeginInfo) -> Self {
        Self::DeviceGroupRenderPassBeginInfo(ext)
    }
}
impl TryInto<DeviceGroupRenderPassBeginInfo> for RenderingInfoExtension {
    type Error = RenderingInfoExtension;
    fn try_into(self) -> Result<DeviceGroupRenderPassBeginInfo, Self::Error> {
        match self {
            Self::DeviceGroupRenderPassBeginInfo(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_dynamic_rendering")]
impl From<RenderingFragmentShadingRateAttachmentInfoKHR> for RenderingInfoExtension {
    fn from(ext: RenderingFragmentShadingRateAttachmentInfoKHR) -> Self {
        Self::RenderingFragmentShadingRateAttachmentInfoKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_dynamic_rendering")]
impl TryInto<RenderingFragmentShadingRateAttachmentInfoKHR> for RenderingInfoExtension {
    type Error = RenderingInfoExtension;
    fn try_into(self) -> Result<RenderingFragmentShadingRateAttachmentInfoKHR, Self::Error> {
        match self {
            Self::RenderingFragmentShadingRateAttachmentInfoKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_dynamic_rendering")]
impl From<RenderingFragmentDensityMapAttachmentInfoEXT> for RenderingInfoExtension {
    fn from(ext: RenderingFragmentDensityMapAttachmentInfoEXT) -> Self {
        Self::RenderingFragmentDensityMapAttachmentInfoEXT(ext)
    }
}
#[cfg(feature = "VK_KHR_dynamic_rendering")]
impl TryInto<RenderingFragmentDensityMapAttachmentInfoEXT> for RenderingInfoExtension {
    type Error = RenderingInfoExtension;
    fn try_into(self) -> Result<RenderingFragmentDensityMapAttachmentInfoEXT, Self::Error> {
        match self {
            Self::RenderingFragmentDensityMapAttachmentInfoEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_dynamic_rendering")]
impl From<MultiviewPerViewAttributesInfoNVX> for RenderingInfoExtension {
    fn from(ext: MultiviewPerViewAttributesInfoNVX) -> Self {
        Self::MultiviewPerViewAttributesInfoNVX(ext)
    }
}
#[cfg(feature = "VK_KHR_dynamic_rendering")]
impl TryInto<MultiviewPerViewAttributesInfoNVX> for RenderingInfoExtension {
    type Error = RenderingInfoExtension;
    fn try_into(self) -> Result<MultiviewPerViewAttributesInfoNVX, Self::Error> {
        match self {
            Self::MultiviewPerViewAttributesInfoNVX(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[doc(alias = "VkRenderingAttachmentInfo")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RenderingAttachmentInfo {
    #[doc(alias = "imageView")]
    pub image_view: Option<ImageView>,
    #[doc(alias = "imageLayout")]
    pub image_layout: ImageLayout,
    #[doc(alias = "resolveMode")]
    pub resolve_mode: ResolveModeFlagBits,
    #[doc(alias = "resolveImageView")]
    pub resolve_image_view: Option<ImageView>,
    #[doc(alias = "resolveImageLayout")]
    pub resolve_image_layout: ImageLayout,
    #[doc(alias = "loadOp")]
    pub load_op: AttachmentLoadOp,
    #[doc(alias = "storeOp")]
    pub store_op: AttachmentStoreOp,
    #[doc(alias = "clearValue")]
    pub clear_value: ClearValue,
}
impl RenderingAttachmentInfo {
    ///Get a reference to the `image_view` field.
    pub fn image_view(&self) -> &Option<ImageView> {
        &self.image_view
    }
    ///Get a reference to the `image_layout` field.
    pub fn image_layout(&self) -> ImageLayout {
        self.image_layout
    }
    ///Get a reference to the `resolve_mode` field.
    pub fn resolve_mode(&self) -> ResolveModeFlagBits {
        self.resolve_mode
    }
    ///Get a reference to the `resolve_image_view` field.
    pub fn resolve_image_view(&self) -> &Option<ImageView> {
        &self.resolve_image_view
    }
    ///Get a reference to the `resolve_image_layout` field.
    pub fn resolve_image_layout(&self) -> ImageLayout {
        self.resolve_image_layout
    }
    ///Get a reference to the `load_op` field.
    pub fn load_op(&self) -> AttachmentLoadOp {
        self.load_op
    }
    ///Get a reference to the `store_op` field.
    pub fn store_op(&self) -> AttachmentStoreOp {
        self.store_op
    }
    ///Get a reference to the `clear_value` field.
    pub fn clear_value(&self) -> ClearValue {
        self.clear_value
    }
    ///Get a mutable reference to the `image_view` field.
    pub fn image_view_mut(&mut self) -> &mut Option<ImageView> {
        &mut self.image_view
    }
    ///Get a mutable reference to the `image_layout` field.
    pub fn image_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.image_layout
    }
    ///Get a mutable reference to the `resolve_mode` field.
    pub fn resolve_mode_mut(&mut self) -> &mut ResolveModeFlagBits {
        &mut self.resolve_mode
    }
    ///Get a mutable reference to the `resolve_image_view` field.
    pub fn resolve_image_view_mut(&mut self) -> &mut Option<ImageView> {
        &mut self.resolve_image_view
    }
    ///Get a mutable reference to the `resolve_image_layout` field.
    pub fn resolve_image_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.resolve_image_layout
    }
    ///Get a mutable reference to the `load_op` field.
    pub fn load_op_mut(&mut self) -> &mut AttachmentLoadOp {
        &mut self.load_op
    }
    ///Get a mutable reference to the `store_op` field.
    pub fn store_op_mut(&mut self) -> &mut AttachmentStoreOp {
        &mut self.store_op
    }
    ///Get a mutable reference to the `clear_value` field.
    pub fn clear_value_mut(&mut self) -> &mut ClearValue {
        &mut self.clear_value
    }
    ///Sets the `image_view` field.
    pub fn set_image_view(&mut self, image_view: Option<ImageView>) -> &mut Self {
        self.image_view = image_view;
        self
    }
    ///Sets the `image_layout` field.
    pub fn set_image_layout(&mut self, image_layout: ImageLayout) -> &mut Self {
        self.image_layout = image_layout;
        self
    }
    ///Sets the `resolve_mode` field.
    pub fn set_resolve_mode(&mut self, resolve_mode: ResolveModeFlagBits) -> &mut Self {
        self.resolve_mode = resolve_mode;
        self
    }
    ///Sets the `resolve_image_view` field.
    pub fn set_resolve_image_view(&mut self, resolve_image_view: Option<ImageView>) -> &mut Self {
        self.resolve_image_view = resolve_image_view;
        self
    }
    ///Sets the `resolve_image_layout` field.
    pub fn set_resolve_image_layout(&mut self, resolve_image_layout: ImageLayout) -> &mut Self {
        self.resolve_image_layout = resolve_image_layout;
        self
    }
    ///Sets the `load_op` field.
    pub fn set_load_op(&mut self, load_op: AttachmentLoadOp) -> &mut Self {
        self.load_op = load_op;
        self
    }
    ///Sets the `store_op` field.
    pub fn set_store_op(&mut self, store_op: AttachmentStoreOp) -> &mut Self {
        self.store_op = store_op;
        self
    }
    ///Sets the `clear_value` field.
    pub fn set_clear_value(&mut self, clear_value: ClearValue) -> &mut Self {
        self.clear_value = clear_value;
        self
    }
    ///Sets the `image_view` field in a builder way.
    pub fn with_image_view(mut self, image_view: Option<ImageView>) -> Self {
        self.image_view = image_view;
        self
    }
    ///Sets the `image_layout` field in a builder way.
    pub fn with_image_layout(mut self, image_layout: ImageLayout) -> Self {
        self.image_layout = image_layout;
        self
    }
    ///Sets the `resolve_mode` field in a builder way.
    pub fn with_resolve_mode(mut self, resolve_mode: ResolveModeFlagBits) -> Self {
        self.resolve_mode = resolve_mode;
        self
    }
    ///Sets the `resolve_image_view` field in a builder way.
    pub fn with_resolve_image_view(mut self, resolve_image_view: Option<ImageView>) -> Self {
        self.resolve_image_view = resolve_image_view;
        self
    }
    ///Sets the `resolve_image_layout` field in a builder way.
    pub fn with_resolve_image_layout(mut self, resolve_image_layout: ImageLayout) -> Self {
        self.resolve_image_layout = resolve_image_layout;
        self
    }
    ///Sets the `load_op` field in a builder way.
    pub fn with_load_op(mut self, load_op: AttachmentLoadOp) -> Self {
        self.load_op = load_op;
        self
    }
    ///Sets the `store_op` field in a builder way.
    pub fn with_store_op(mut self, store_op: AttachmentStoreOp) -> Self {
        self.store_op = store_op;
        self
    }
    ///Sets the `clear_value` field in a builder way.
    pub fn with_clear_value(mut self, clear_value: ClearValue) -> Self {
        self.clear_value = clear_value;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for RenderingAttachmentInfo {
    type LowLevel = crate::native::vulkan1_3::RenderingAttachmentInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::RenderingAttachmentInfo {
            s_type: StructureType::RenderingAttachmentInfo,
            p_next: std::ptr::null(),
            image_view: self
                .image_view
                .as_ref()
                .map(|v| v.into_low_level(context, bump))
                .unwrap_or_else(Default::default),
            image_layout: self.image_layout.into_low_level(context, bump),
            resolve_mode: self.resolve_mode.into_low_level(context, bump),
            resolve_image_view: self
                .resolve_image_view
                .as_ref()
                .map(|v| v.into_low_level(context, bump))
                .unwrap_or_else(Default::default),
            resolve_image_layout: self.resolve_image_layout.into_low_level(context, bump),
            load_op: self.load_op.into_low_level(context, bump),
            store_op: self.store_op.into_low_level(context, bump),
            clear_value: self.clear_value.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for RenderingAttachmentInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            image_view: if value.image_view == crate::native::vulkan1_0::ImageView::null() {
                None
            } else {
                Some(crate::conv::FromLowLevel::from_low_level(context, value.image_view))
            },
            image_layout: crate::conv::FromLowLevel::from_low_level(context, value.image_layout),
            resolve_mode: crate::conv::FromLowLevel::from_low_level(context, value.resolve_mode),
            resolve_image_view: if value.resolve_image_view == crate::native::vulkan1_0::ImageView::null() {
                None
            } else {
                Some(crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.resolve_image_view,
                ))
            },
            resolve_image_layout: crate::conv::FromLowLevel::from_low_level(context, value.resolve_image_layout),
            load_op: crate::conv::FromLowLevel::from_low_level(context, value.load_op),
            store_op: crate::conv::FromLowLevel::from_low_level(context, value.store_op),
            clear_value: crate::conv::FromLowLevel::from_low_level(context, value.clear_value),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceDynamicRenderingFeatures")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceDynamicRenderingFeatures {
    #[doc(alias = "dynamicRendering")]
    pub dynamic_rendering: bool,
}
impl PhysicalDeviceDynamicRenderingFeatures {
    ///Get a reference to the `dynamic_rendering` field.
    pub fn dynamic_rendering(&self) -> &bool {
        &self.dynamic_rendering
    }
    ///Get a mutable reference to the `dynamic_rendering` field.
    pub fn dynamic_rendering_mut(&mut self) -> &mut bool {
        &mut self.dynamic_rendering
    }
    ///Sets the `dynamic_rendering` field.
    pub fn set_dynamic_rendering(&mut self, dynamic_rendering: bool) -> &mut Self {
        self.dynamic_rendering = dynamic_rendering;
        self
    }
    ///Sets the `dynamic_rendering` field in a builder way.
    pub fn with_dynamic_rendering(mut self, dynamic_rendering: bool) -> Self {
        self.dynamic_rendering = dynamic_rendering;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceDynamicRenderingFeatures {
    type LowLevel = crate::native::vulkan1_3::PhysicalDeviceDynamicRenderingFeatures;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_3::PhysicalDeviceDynamicRenderingFeatures {
            s_type: StructureType::PhysicalDeviceDynamicRenderingFeatures,
            p_next: std::ptr::null_mut(),
            dynamic_rendering: self.dynamic_rendering.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceDynamicRenderingFeatures {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            dynamic_rendering: crate::conv::FromLowLevel::from_low_level(context, value.dynamic_rendering),
        }
    }
}
#[doc(alias = "VkCommandBufferInheritanceRenderingInfo")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CommandBufferInheritanceRenderingInfo {
    pub flags: RenderingFlags,
    #[doc(alias = "viewMask")]
    pub view_mask: u32,
    #[doc(alias = "pColorAttachmentFormats")]
    pub color_attachment_formats: SmallVec<[Format; 8]>,
    #[doc(alias = "depthAttachmentFormat")]
    pub depth_attachment_format: Format,
    #[doc(alias = "stencilAttachmentFormat")]
    pub stencil_attachment_format: Format,
    #[doc(alias = "rasterizationSamples")]
    pub rasterization_samples: SampleCountFlagBits,
}
impl CommandBufferInheritanceRenderingInfo {
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> RenderingFlags {
        self.flags
    }
    ///Get a reference to the `view_mask` field.
    pub fn view_mask(&self) -> u32 {
        self.view_mask
    }
    ///Get a reference to the `color_attachment_formats` field.
    pub fn color_attachment_formats(&self) -> &SmallVec<[Format; 8]> {
        &self.color_attachment_formats
    }
    ///Get a reference to the `depth_attachment_format` field.
    pub fn depth_attachment_format(&self) -> Format {
        self.depth_attachment_format
    }
    ///Get a reference to the `stencil_attachment_format` field.
    pub fn stencil_attachment_format(&self) -> Format {
        self.stencil_attachment_format
    }
    ///Get a reference to the `rasterization_samples` field.
    pub fn rasterization_samples(&self) -> SampleCountFlagBits {
        self.rasterization_samples
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut RenderingFlags {
        &mut self.flags
    }
    ///Get a mutable reference to the `view_mask` field.
    pub fn view_mask_mut(&mut self) -> &mut u32 {
        &mut self.view_mask
    }
    ///Get a mutable reference to the `color_attachment_formats` field.
    pub fn color_attachment_formats_mut(&mut self) -> &mut SmallVec<[Format; 8]> {
        &mut self.color_attachment_formats
    }
    ///Get a mutable reference to the `depth_attachment_format` field.
    pub fn depth_attachment_format_mut(&mut self) -> &mut Format {
        &mut self.depth_attachment_format
    }
    ///Get a mutable reference to the `stencil_attachment_format` field.
    pub fn stencil_attachment_format_mut(&mut self) -> &mut Format {
        &mut self.stencil_attachment_format
    }
    ///Get a mutable reference to the `rasterization_samples` field.
    pub fn rasterization_samples_mut(&mut self) -> &mut SampleCountFlagBits {
        &mut self.rasterization_samples
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: RenderingFlags) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `view_mask` field.
    pub fn set_view_mask(&mut self, view_mask: u32) -> &mut Self {
        self.view_mask = view_mask;
        self
    }
    ///Sets the `color_attachment_formats` field.
    pub fn set_color_attachment_formats(&mut self, color_attachment_formats: SmallVec<[Format; 8]>) -> &mut Self {
        self.color_attachment_formats = color_attachment_formats;
        self
    }
    ///Sets the `depth_attachment_format` field.
    pub fn set_depth_attachment_format(&mut self, depth_attachment_format: Format) -> &mut Self {
        self.depth_attachment_format = depth_attachment_format;
        self
    }
    ///Sets the `stencil_attachment_format` field.
    pub fn set_stencil_attachment_format(&mut self, stencil_attachment_format: Format) -> &mut Self {
        self.stencil_attachment_format = stencil_attachment_format;
        self
    }
    ///Sets the `rasterization_samples` field.
    pub fn set_rasterization_samples(&mut self, rasterization_samples: SampleCountFlagBits) -> &mut Self {
        self.rasterization_samples = rasterization_samples;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: RenderingFlags) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `view_mask` field in a builder way.
    pub fn with_view_mask(mut self, view_mask: u32) -> Self {
        self.view_mask = view_mask;
        self
    }
    ///Sets the `color_attachment_formats` field in a builder way.
    pub fn with_color_attachment_formats(mut self, color_attachment_formats: SmallVec<[Format; 8]>) -> Self {
        self.color_attachment_formats = color_attachment_formats;
        self
    }
    ///Sets the `depth_attachment_format` field in a builder way.
    pub fn with_depth_attachment_format(mut self, depth_attachment_format: Format) -> Self {
        self.depth_attachment_format = depth_attachment_format;
        self
    }
    ///Sets the `stencil_attachment_format` field in a builder way.
    pub fn with_stencil_attachment_format(mut self, stencil_attachment_format: Format) -> Self {
        self.stencil_attachment_format = stencil_attachment_format;
        self
    }
    ///Sets the `rasterization_samples` field in a builder way.
    pub fn with_rasterization_samples(mut self, rasterization_samples: SampleCountFlagBits) -> Self {
        self.rasterization_samples = rasterization_samples;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for CommandBufferInheritanceRenderingInfo {
    type LowLevel = crate::native::vulkan1_3::CommandBufferInheritanceRenderingInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_color_attachment_formats = self.color_attachment_formats.len() as u32;
        let color_attachment_formats = bump
            .alloc_slice_fill_iter(
                self.color_attachment_formats
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_ptr()
            .cast();
        crate::native::vulkan1_3::CommandBufferInheritanceRenderingInfo {
            s_type: StructureType::CommandBufferInheritanceRenderingInfo,
            p_next: std::ptr::null(),
            flags: self.flags.into_low_level(context, bump),
            view_mask: self.view_mask.into_low_level(context, bump),
            color_attachment_count: len_color_attachment_formats,
            color_attachment_formats: color_attachment_formats,
            depth_attachment_format: self.depth_attachment_format.into_low_level(context, bump),
            stencil_attachment_format: self.stencil_attachment_format.into_low_level(context, bump),
            rasterization_samples: self.rasterization_samples.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for CommandBufferInheritanceRenderingInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let color_attachment_formats_len = value.color_attachment_count;
        let mut color_attachment_formats = SmallVec::with_capacity(color_attachment_formats_len as usize);
        for i in 0..color_attachment_formats_len {
            color_attachment_formats.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.color_attachment_formats.add(i as usize).read(),
            ));
        }
        Self {
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            view_mask: crate::conv::FromLowLevel::from_low_level(context, value.view_mask),
            color_attachment_formats: color_attachment_formats,
            depth_attachment_format: crate::conv::FromLowLevel::from_low_level(context, value.depth_attachment_format),
            stencil_attachment_format: crate::conv::FromLowLevel::from_low_level(
                context,
                value.stencil_attachment_format,
            ),
            rasterization_samples: crate::conv::FromLowLevel::from_low_level(context, value.rasterization_samples),
        }
    }
}
#[doc(alias = "VkPrivateDataSlot")]
#[derive(Debug)]
pub struct PrivateDataSlot {
    context: Arc<Context>,
    id: ObjectId,
}
impl Clone for PrivateDataSlot {
    fn clone(&self) -> Self {
        self.context.clone_private_data_slot(self.id);
        Self {
            context: Arc::clone(&self.context),
            id: self.id,
        }
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for PrivateDataSlot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.id.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for PrivateDataSlot {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id = ObjectId::deserialize(deserializer)?;
        crate::context::CONTEXT.with(|context| {
            let borrow = context.borrow();
            let context = borrow.as_ref().expect("Context not set.");
            Ok(Self {
                context: Arc::clone(context),
                id,
            })
        })
    }
}
impl Drop for PrivateDataSlot {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.context.drop_private_data_slot(&self.id);
        }
    }
}
impl PartialEq for PrivateDataSlot {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl PrivateDataSlot {
    ///Creates a new instance of this handle from its core components.
    pub(crate) const unsafe fn new(context: Arc<Context>, id: ObjectId) -> Self {
        Self { context, id }
    }
    ///Gets the object id
    pub fn id(&self) -> &ObjectId {
        &self.id
    }
    ///Gets a reference to the context
    pub fn context(&self) -> &Context {
        &self.context
    }
    ///Gets a reference to the context wrapped in an [`Arc`]
    pub fn arc_context(&self) -> &Arc<Context> {
        &self.context
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PrivateDataSlot {
    type LowLevel = crate::native::vulkan1_3::PrivateDataSlot;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *context
            .private_data_slot()
            .get(&self.id)
            .expect("unknwon handle")
            .handle()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PrivateDataSlot {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let object_id = ObjectId::random();
        context.private_data_slot().insert(object_id, Container::new(value));
        Self {
            context: context.clone(),
            id: object_id,
        }
    }
}
