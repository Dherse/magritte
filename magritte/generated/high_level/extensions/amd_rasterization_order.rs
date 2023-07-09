pub use crate::common::extensions::amd_rasterization_order::{
    RasterizationOrderAMD, AMD_RASTERIZATION_ORDER_EXTENSION_NAME, AMD_RASTERIZATION_ORDER_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPipelineRasterizationStateRasterizationOrderAMD")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineRasterizationStateRasterizationOrderAMD {
    #[doc(alias = "rasterizationOrder")]
    pub rasterization_order: RasterizationOrderAMD,
}
impl PipelineRasterizationStateRasterizationOrderAMD {
    ///Get a reference to the `rasterization_order` field.
    pub fn rasterization_order(&self) -> RasterizationOrderAMD {
        self.rasterization_order
    }
    ///Get a mutable reference to the `rasterization_order` field.
    pub fn rasterization_order_mut(&mut self) -> &mut RasterizationOrderAMD {
        &mut self.rasterization_order
    }
    ///Sets the `rasterization_order` field.
    pub fn set_rasterization_order(&mut self, rasterization_order: RasterizationOrderAMD) -> &mut Self {
        self.rasterization_order = rasterization_order;
        self
    }
    ///Sets the `rasterization_order` field in a builder way.
    pub fn with_rasterization_order(mut self, rasterization_order: RasterizationOrderAMD) -> Self {
        self.rasterization_order = rasterization_order;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineRasterizationStateRasterizationOrderAMD {
    type LowLevel = crate::native::extensions::amd_rasterization_order::PipelineRasterizationStateRasterizationOrderAMD;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::amd_rasterization_order::PipelineRasterizationStateRasterizationOrderAMD {
            s_type: StructureType::PipelineRasterizationStateRasterizationOrderAmd,
            p_next: std::ptr::null(),
            rasterization_order: self.rasterization_order.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineRasterizationStateRasterizationOrderAMD {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            rasterization_order: crate::conv::FromLowLevel::from_low_level(context, value.rasterization_order),
        }
    }
}
