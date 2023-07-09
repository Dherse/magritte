pub use crate::common::extensions::arm_rasterization_order_attachment_access::{
    PipelineColorBlendStateCreateFlagBits, PipelineDepthStencilStateCreateFlagBits,
    ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXTENSION_NAME, ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM {
    #[doc(alias = "rasterizationOrderColorAttachmentAccess")]
    pub rasterization_order_color_attachment_access: bool,
    #[doc(alias = "rasterizationOrderDepthAttachmentAccess")]
    pub rasterization_order_depth_attachment_access: bool,
    #[doc(alias = "rasterizationOrderStencilAttachmentAccess")]
    pub rasterization_order_stencil_attachment_access: bool,
}
impl PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM {
    ///Get a reference to the `rasterization_order_color_attachment_access` field.
    pub fn rasterization_order_color_attachment_access(&self) -> &bool {
        &self.rasterization_order_color_attachment_access
    }
    ///Get a reference to the `rasterization_order_depth_attachment_access` field.
    pub fn rasterization_order_depth_attachment_access(&self) -> &bool {
        &self.rasterization_order_depth_attachment_access
    }
    ///Get a reference to the `rasterization_order_stencil_attachment_access` field.
    pub fn rasterization_order_stencil_attachment_access(&self) -> &bool {
        &self.rasterization_order_stencil_attachment_access
    }
    ///Get a mutable reference to the `rasterization_order_color_attachment_access` field.
    pub fn rasterization_order_color_attachment_access_mut(&mut self) -> &mut bool {
        &mut self.rasterization_order_color_attachment_access
    }
    ///Get a mutable reference to the `rasterization_order_depth_attachment_access` field.
    pub fn rasterization_order_depth_attachment_access_mut(&mut self) -> &mut bool {
        &mut self.rasterization_order_depth_attachment_access
    }
    ///Get a mutable reference to the `rasterization_order_stencil_attachment_access` field.
    pub fn rasterization_order_stencil_attachment_access_mut(&mut self) -> &mut bool {
        &mut self.rasterization_order_stencil_attachment_access
    }
    ///Sets the `rasterization_order_color_attachment_access` field.
    pub fn set_rasterization_order_color_attachment_access(
        &mut self,
        rasterization_order_color_attachment_access: bool,
    ) -> &mut Self {
        self.rasterization_order_color_attachment_access = rasterization_order_color_attachment_access;
        self
    }
    ///Sets the `rasterization_order_depth_attachment_access` field.
    pub fn set_rasterization_order_depth_attachment_access(
        &mut self,
        rasterization_order_depth_attachment_access: bool,
    ) -> &mut Self {
        self.rasterization_order_depth_attachment_access = rasterization_order_depth_attachment_access;
        self
    }
    ///Sets the `rasterization_order_stencil_attachment_access` field.
    pub fn set_rasterization_order_stencil_attachment_access(
        &mut self,
        rasterization_order_stencil_attachment_access: bool,
    ) -> &mut Self {
        self.rasterization_order_stencil_attachment_access = rasterization_order_stencil_attachment_access;
        self
    }
    ///Sets the `rasterization_order_color_attachment_access` field in a builder way.
    pub fn with_rasterization_order_color_attachment_access(
        mut self,
        rasterization_order_color_attachment_access: bool,
    ) -> Self {
        self.rasterization_order_color_attachment_access = rasterization_order_color_attachment_access;
        self
    }
    ///Sets the `rasterization_order_depth_attachment_access` field in a builder way.
    pub fn with_rasterization_order_depth_attachment_access(
        mut self,
        rasterization_order_depth_attachment_access: bool,
    ) -> Self {
        self.rasterization_order_depth_attachment_access = rasterization_order_depth_attachment_access;
        self
    }
    ///Sets the `rasterization_order_stencil_attachment_access` field in a builder way.
    pub fn with_rasterization_order_stencil_attachment_access(
        mut self,
        rasterization_order_stencil_attachment_access: bool,
    ) -> Self {
        self.rasterization_order_stencil_attachment_access = rasterization_order_stencil_attachment_access;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM {
    type LowLevel = crate :: native :: extensions :: arm_rasterization_order_attachment_access :: PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM ;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate :: native :: extensions :: arm_rasterization_order_attachment_access :: PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM { s_type : StructureType :: PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesArm , p_next : std :: ptr :: null_mut () , rasterization_order_color_attachment_access : self . rasterization_order_color_attachment_access . into_low_level (context , bump) , rasterization_order_depth_attachment_access : self . rasterization_order_depth_attachment_access . into_low_level (context , bump) , rasterization_order_stencil_attachment_access : self . rasterization_order_stencil_attachment_access . into_low_level (context , bump) }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            rasterization_order_color_attachment_access: crate::conv::FromLowLevel::from_low_level(
                context,
                value.rasterization_order_color_attachment_access,
            ),
            rasterization_order_depth_attachment_access: crate::conv::FromLowLevel::from_low_level(
                context,
                value.rasterization_order_depth_attachment_access,
            ),
            rasterization_order_stencil_attachment_access: crate::conv::FromLowLevel::from_low_level(
                context,
                value.rasterization_order_stencil_attachment_access,
            ),
        }
    }
}
