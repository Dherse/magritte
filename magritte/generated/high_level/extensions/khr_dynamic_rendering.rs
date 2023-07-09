pub use crate::common::extensions::khr_dynamic_rendering::{
    KHR_DYNAMIC_RENDERING_EXTENSION_NAME, KHR_DYNAMIC_RENDERING_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{Extent2D, ImageLayout, ImageView, SampleCountFlagBits, StructureType},
    vulkan1_3::{
        CommandBufferInheritanceRenderingInfo, PhysicalDeviceDynamicRenderingFeatures, PipelineRenderingCreateInfo,
        RenderingAttachmentInfo, RenderingFlagBits, RenderingFlags, RenderingInfo,
    },
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
#[doc(alias = "VkRenderingFlagsKHR")]
pub type RenderingFlagsKHR = RenderingFlags;
#[doc(alias = "VkRenderingFlagBitsKHR")]
pub type RenderingFlagBitsKHR = RenderingFlagBits;
#[doc(alias = "VkPipelineRenderingCreateInfoKHR")]
pub type PipelineRenderingCreateInfoKHR = PipelineRenderingCreateInfo;
#[doc(alias = "VkRenderingInfoKHR")]
pub type RenderingInfoKHR = RenderingInfo;
#[doc(alias = "VkRenderingAttachmentInfoKHR")]
pub type RenderingAttachmentInfoKHR = RenderingAttachmentInfo;
#[doc(alias = "VkPhysicalDeviceDynamicRenderingFeaturesKHR")]
pub type PhysicalDeviceDynamicRenderingFeaturesKHR = PhysicalDeviceDynamicRenderingFeatures;
#[doc(alias = "VkCommandBufferInheritanceRenderingInfoKHR")]
pub type CommandBufferInheritanceRenderingInfoKHR = CommandBufferInheritanceRenderingInfo;
#[doc(alias = "VkAttachmentSampleCountInfoNV")]
pub type AttachmentSampleCountInfoNV = AttachmentSampleCountInfoAMD;
#[doc(alias = "VkRenderingFragmentShadingRateAttachmentInfoKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RenderingFragmentShadingRateAttachmentInfoKHR {
    #[doc(alias = "imageView")]
    pub image_view: Option<ImageView>,
    #[doc(alias = "imageLayout")]
    pub image_layout: ImageLayout,
    #[doc(alias = "shadingRateAttachmentTexelSize")]
    pub shading_rate_attachment_texel_size: Extent2D,
}
impl RenderingFragmentShadingRateAttachmentInfoKHR {
    ///Get a reference to the `image_view` field.
    pub fn image_view(&self) -> &Option<ImageView> {
        &self.image_view
    }
    ///Get a reference to the `image_layout` field.
    pub fn image_layout(&self) -> ImageLayout {
        self.image_layout
    }
    ///Get a reference to the `shading_rate_attachment_texel_size` field.
    pub fn shading_rate_attachment_texel_size(&self) -> Extent2D {
        self.shading_rate_attachment_texel_size
    }
    ///Get a mutable reference to the `image_view` field.
    pub fn image_view_mut(&mut self) -> &mut Option<ImageView> {
        &mut self.image_view
    }
    ///Get a mutable reference to the `image_layout` field.
    pub fn image_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.image_layout
    }
    ///Get a mutable reference to the `shading_rate_attachment_texel_size` field.
    pub fn shading_rate_attachment_texel_size_mut(&mut self) -> &mut Extent2D {
        &mut self.shading_rate_attachment_texel_size
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
    ///Sets the `shading_rate_attachment_texel_size` field.
    pub fn set_shading_rate_attachment_texel_size(
        &mut self,
        shading_rate_attachment_texel_size: Extent2D,
    ) -> &mut Self {
        self.shading_rate_attachment_texel_size = shading_rate_attachment_texel_size;
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
    ///Sets the `shading_rate_attachment_texel_size` field in a builder way.
    pub fn with_shading_rate_attachment_texel_size(mut self, shading_rate_attachment_texel_size: Extent2D) -> Self {
        self.shading_rate_attachment_texel_size = shading_rate_attachment_texel_size;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for RenderingFragmentShadingRateAttachmentInfoKHR {
    type LowLevel = crate::native::extensions::khr_dynamic_rendering::RenderingFragmentShadingRateAttachmentInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_dynamic_rendering::RenderingFragmentShadingRateAttachmentInfoKHR {
            s_type: StructureType::RenderingFragmentShadingRateAttachmentInfoKhr,
            p_next: std::ptr::null(),
            image_view: self
                .image_view
                .as_ref()
                .map(|v| v.into_low_level(context, bump))
                .unwrap_or_else(Default::default),
            image_layout: self.image_layout.into_low_level(context, bump),
            shading_rate_attachment_texel_size: self.shading_rate_attachment_texel_size.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for RenderingFragmentShadingRateAttachmentInfoKHR {
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
            shading_rate_attachment_texel_size: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shading_rate_attachment_texel_size,
            ),
        }
    }
}
#[doc(alias = "VkRenderingFragmentDensityMapAttachmentInfoEXT")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RenderingFragmentDensityMapAttachmentInfoEXT {
    #[doc(alias = "imageView")]
    pub image_view: ImageView,
    #[doc(alias = "imageLayout")]
    pub image_layout: ImageLayout,
}
impl RenderingFragmentDensityMapAttachmentInfoEXT {
    ///Get a reference to the `image_view` field.
    pub fn image_view(&self) -> &ImageView {
        &self.image_view
    }
    ///Get a reference to the `image_layout` field.
    pub fn image_layout(&self) -> ImageLayout {
        self.image_layout
    }
    ///Get a mutable reference to the `image_view` field.
    pub fn image_view_mut(&mut self) -> &mut ImageView {
        &mut self.image_view
    }
    ///Get a mutable reference to the `image_layout` field.
    pub fn image_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.image_layout
    }
    ///Sets the `image_view` field.
    pub fn set_image_view(&mut self, image_view: ImageView) -> &mut Self {
        self.image_view = image_view;
        self
    }
    ///Sets the `image_layout` field.
    pub fn set_image_layout(&mut self, image_layout: ImageLayout) -> &mut Self {
        self.image_layout = image_layout;
        self
    }
    ///Sets the `image_view` field in a builder way.
    pub fn with_image_view(mut self, image_view: ImageView) -> Self {
        self.image_view = image_view;
        self
    }
    ///Sets the `image_layout` field in a builder way.
    pub fn with_image_layout(mut self, image_layout: ImageLayout) -> Self {
        self.image_layout = image_layout;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for RenderingFragmentDensityMapAttachmentInfoEXT {
    type LowLevel = crate::native::extensions::khr_dynamic_rendering::RenderingFragmentDensityMapAttachmentInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_dynamic_rendering::RenderingFragmentDensityMapAttachmentInfoEXT {
            s_type: StructureType::RenderingFragmentDensityMapAttachmentInfoExt,
            p_next: std::ptr::null(),
            image_view: self.image_view.into_low_level(context, bump),
            image_layout: self.image_layout.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for RenderingFragmentDensityMapAttachmentInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            image_view: crate::conv::FromLowLevel::from_low_level(context, value.image_view),
            image_layout: crate::conv::FromLowLevel::from_low_level(context, value.image_layout),
        }
    }
}
#[doc(alias = "VkAttachmentSampleCountInfoAMD")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AttachmentSampleCountInfoAMD {
    #[doc(alias = "pColorAttachmentSamples")]
    pub color_attachment_samples: SmallVec<[SampleCountFlagBits; 8]>,
    #[doc(alias = "depthStencilAttachmentSamples")]
    pub depth_stencil_attachment_samples: SampleCountFlagBits,
}
impl AttachmentSampleCountInfoAMD {
    ///Get a reference to the `color_attachment_samples` field.
    pub fn color_attachment_samples(&self) -> &SmallVec<[SampleCountFlagBits; 8]> {
        &self.color_attachment_samples
    }
    ///Get a reference to the `depth_stencil_attachment_samples` field.
    pub fn depth_stencil_attachment_samples(&self) -> SampleCountFlagBits {
        self.depth_stencil_attachment_samples
    }
    ///Get a mutable reference to the `color_attachment_samples` field.
    pub fn color_attachment_samples_mut(&mut self) -> &mut SmallVec<[SampleCountFlagBits; 8]> {
        &mut self.color_attachment_samples
    }
    ///Get a mutable reference to the `depth_stencil_attachment_samples` field.
    pub fn depth_stencil_attachment_samples_mut(&mut self) -> &mut SampleCountFlagBits {
        &mut self.depth_stencil_attachment_samples
    }
    ///Sets the `color_attachment_samples` field.
    pub fn set_color_attachment_samples(
        &mut self,
        color_attachment_samples: SmallVec<[SampleCountFlagBits; 8]>,
    ) -> &mut Self {
        self.color_attachment_samples = color_attachment_samples;
        self
    }
    ///Sets the `depth_stencil_attachment_samples` field.
    pub fn set_depth_stencil_attachment_samples(
        &mut self,
        depth_stencil_attachment_samples: SampleCountFlagBits,
    ) -> &mut Self {
        self.depth_stencil_attachment_samples = depth_stencil_attachment_samples;
        self
    }
    ///Sets the `color_attachment_samples` field in a builder way.
    pub fn with_color_attachment_samples(
        mut self,
        color_attachment_samples: SmallVec<[SampleCountFlagBits; 8]>,
    ) -> Self {
        self.color_attachment_samples = color_attachment_samples;
        self
    }
    ///Sets the `depth_stencil_attachment_samples` field in a builder way.
    pub fn with_depth_stencil_attachment_samples(
        mut self,
        depth_stencil_attachment_samples: SampleCountFlagBits,
    ) -> Self {
        self.depth_stencil_attachment_samples = depth_stencil_attachment_samples;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AttachmentSampleCountInfoAMD {
    type LowLevel = crate::native::extensions::khr_dynamic_rendering::AttachmentSampleCountInfoAMD;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_color_attachment_samples = self.color_attachment_samples.len() as u32;
        let color_attachment_samples = bump
            .alloc_slice_fill_iter(
                self.color_attachment_samples
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_ptr()
            .cast();
        crate::native::extensions::khr_dynamic_rendering::AttachmentSampleCountInfoAMD {
            s_type: StructureType::AttachmentSampleCountInfoAmd,
            p_next: std::ptr::null(),
            color_attachment_count: len_color_attachment_samples,
            color_attachment_samples: color_attachment_samples,
            depth_stencil_attachment_samples: self.depth_stencil_attachment_samples.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AttachmentSampleCountInfoAMD {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let color_attachment_samples_len = value.color_attachment_count;
        let mut color_attachment_samples = SmallVec::with_capacity(color_attachment_samples_len as usize);
        for i in 0..color_attachment_samples_len {
            color_attachment_samples.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.color_attachment_samples.add(i as usize).read(),
            ));
        }
        Self {
            color_attachment_samples: color_attachment_samples,
            depth_stencil_attachment_samples: crate::conv::FromLowLevel::from_low_level(
                context,
                value.depth_stencil_attachment_samples,
            ),
        }
    }
}
#[doc(alias = "VkMultiviewPerViewAttributesInfoNVX")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MultiviewPerViewAttributesInfoNVX {
    #[doc(alias = "perViewAttributes")]
    pub per_view_attributes: bool,
    #[doc(alias = "perViewAttributesPositionXOnly")]
    pub per_view_attributes_position_x_only: bool,
}
impl MultiviewPerViewAttributesInfoNVX {
    ///Get a reference to the `per_view_attributes` field.
    pub fn per_view_attributes(&self) -> &bool {
        &self.per_view_attributes
    }
    ///Get a reference to the `per_view_attributes_position_x_only` field.
    pub fn per_view_attributes_position_x_only(&self) -> &bool {
        &self.per_view_attributes_position_x_only
    }
    ///Get a mutable reference to the `per_view_attributes` field.
    pub fn per_view_attributes_mut(&mut self) -> &mut bool {
        &mut self.per_view_attributes
    }
    ///Get a mutable reference to the `per_view_attributes_position_x_only` field.
    pub fn per_view_attributes_position_x_only_mut(&mut self) -> &mut bool {
        &mut self.per_view_attributes_position_x_only
    }
    ///Sets the `per_view_attributes` field.
    pub fn set_per_view_attributes(&mut self, per_view_attributes: bool) -> &mut Self {
        self.per_view_attributes = per_view_attributes;
        self
    }
    ///Sets the `per_view_attributes_position_x_only` field.
    pub fn set_per_view_attributes_position_x_only(&mut self, per_view_attributes_position_x_only: bool) -> &mut Self {
        self.per_view_attributes_position_x_only = per_view_attributes_position_x_only;
        self
    }
    ///Sets the `per_view_attributes` field in a builder way.
    pub fn with_per_view_attributes(mut self, per_view_attributes: bool) -> Self {
        self.per_view_attributes = per_view_attributes;
        self
    }
    ///Sets the `per_view_attributes_position_x_only` field in a builder way.
    pub fn with_per_view_attributes_position_x_only(mut self, per_view_attributes_position_x_only: bool) -> Self {
        self.per_view_attributes_position_x_only = per_view_attributes_position_x_only;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for MultiviewPerViewAttributesInfoNVX {
    type LowLevel = crate::native::extensions::khr_dynamic_rendering::MultiviewPerViewAttributesInfoNVX;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_dynamic_rendering::MultiviewPerViewAttributesInfoNVX {
            s_type: StructureType::MultiviewPerViewAttributesInfoNvx,
            p_next: std::ptr::null(),
            per_view_attributes: self.per_view_attributes.into_low_level(context, bump),
            per_view_attributes_position_x_only: self.per_view_attributes_position_x_only.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for MultiviewPerViewAttributesInfoNVX {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            per_view_attributes: crate::conv::FromLowLevel::from_low_level(context, value.per_view_attributes),
            per_view_attributes_position_x_only: crate::conv::FromLowLevel::from_low_level(
                context,
                value.per_view_attributes_position_x_only,
            ),
        }
    }
}
