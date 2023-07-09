pub use crate::common::extensions::nv_clip_space_w_scaling::{
    ViewportWScalingNV, NV_CLIP_SPACE_W_SCALING_EXTENSION_NAME, NV_CLIP_SPACE_W_SCALING_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
impl ViewportWScalingNV {
    ///Get a reference to the `xcoeff` field.
    pub fn xcoeff(&self) -> f32 {
        self.xcoeff
    }
    ///Get a reference to the `ycoeff` field.
    pub fn ycoeff(&self) -> f32 {
        self.ycoeff
    }
    ///Get a mutable reference to the `xcoeff` field.
    pub fn xcoeff_mut(&mut self) -> &mut f32 {
        &mut self.xcoeff
    }
    ///Get a mutable reference to the `ycoeff` field.
    pub fn ycoeff_mut(&mut self) -> &mut f32 {
        &mut self.ycoeff
    }
    ///Sets the `xcoeff` field.
    pub fn set_xcoeff(&mut self, xcoeff: f32) -> &mut Self {
        self.xcoeff = xcoeff;
        self
    }
    ///Sets the `ycoeff` field.
    pub fn set_ycoeff(&mut self, ycoeff: f32) -> &mut Self {
        self.ycoeff = ycoeff;
        self
    }
    ///Sets the `xcoeff` field in a builder way.
    pub fn with_xcoeff(mut self, xcoeff: f32) -> Self {
        self.xcoeff = xcoeff;
        self
    }
    ///Sets the `ycoeff` field in a builder way.
    pub fn with_ycoeff(mut self, ycoeff: f32) -> Self {
        self.ycoeff = ycoeff;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ViewportWScalingNV {
    type LowLevel = crate::native::extensions::nv_clip_space_w_scaling::ViewportWScalingNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_clip_space_w_scaling::ViewportWScalingNV {
            xcoeff: self.xcoeff.into_low_level(context, bump),
            ycoeff: self.ycoeff.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ViewportWScalingNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            xcoeff: crate::conv::FromLowLevel::from_low_level(context, value.xcoeff),
            ycoeff: crate::conv::FromLowLevel::from_low_level(context, value.ycoeff),
        }
    }
}
#[doc(alias = "VkPipelineViewportWScalingStateCreateInfoNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineViewportWScalingStateCreateInfoNV {
    #[doc(alias = "viewportWScalingEnable")]
    pub viewport_w_scaling_enable: bool,
    #[doc(alias = "pViewportWScalings")]
    pub viewport_w_scalings: SmallVec<[ViewportWScalingNV; 8]>,
}
impl PipelineViewportWScalingStateCreateInfoNV {
    ///Get a reference to the `viewport_w_scaling_enable` field.
    pub fn viewport_w_scaling_enable(&self) -> &bool {
        &self.viewport_w_scaling_enable
    }
    ///Get a reference to the `viewport_w_scalings` field.
    pub fn viewport_w_scalings(&self) -> &SmallVec<[ViewportWScalingNV; 8]> {
        &self.viewport_w_scalings
    }
    ///Get a mutable reference to the `viewport_w_scaling_enable` field.
    pub fn viewport_w_scaling_enable_mut(&mut self) -> &mut bool {
        &mut self.viewport_w_scaling_enable
    }
    ///Get a mutable reference to the `viewport_w_scalings` field.
    pub fn viewport_w_scalings_mut(&mut self) -> &mut SmallVec<[ViewportWScalingNV; 8]> {
        &mut self.viewport_w_scalings
    }
    ///Sets the `viewport_w_scaling_enable` field.
    pub fn set_viewport_w_scaling_enable(&mut self, viewport_w_scaling_enable: bool) -> &mut Self {
        self.viewport_w_scaling_enable = viewport_w_scaling_enable;
        self
    }
    ///Sets the `viewport_w_scalings` field.
    pub fn set_viewport_w_scalings(&mut self, viewport_w_scalings: SmallVec<[ViewportWScalingNV; 8]>) -> &mut Self {
        self.viewport_w_scalings = viewport_w_scalings;
        self
    }
    ///Sets the `viewport_w_scaling_enable` field in a builder way.
    pub fn with_viewport_w_scaling_enable(mut self, viewport_w_scaling_enable: bool) -> Self {
        self.viewport_w_scaling_enable = viewport_w_scaling_enable;
        self
    }
    ///Sets the `viewport_w_scalings` field in a builder way.
    pub fn with_viewport_w_scalings(mut self, viewport_w_scalings: SmallVec<[ViewportWScalingNV; 8]>) -> Self {
        self.viewport_w_scalings = viewport_w_scalings;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineViewportWScalingStateCreateInfoNV {
    type LowLevel = crate::native::extensions::nv_clip_space_w_scaling::PipelineViewportWScalingStateCreateInfoNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_viewport_w_scalings = self.viewport_w_scalings.len() as u32;
        let viewport_w_scalings = bump
            .alloc_slice_fill_iter(self.viewport_w_scalings.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::extensions::nv_clip_space_w_scaling::PipelineViewportWScalingStateCreateInfoNV {
            s_type: StructureType::PipelineViewportWScalingStateCreateInfoNv,
            p_next: std::ptr::null(),
            viewport_w_scaling_enable: self.viewport_w_scaling_enable.into_low_level(context, bump),
            viewport_count: len_viewport_w_scalings,
            viewport_w_scalings: viewport_w_scalings,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineViewportWScalingStateCreateInfoNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let viewport_w_scalings_len = value.viewport_count;
        let mut viewport_w_scalings = SmallVec::with_capacity(viewport_w_scalings_len as usize);
        for i in 0..viewport_w_scalings_len {
            viewport_w_scalings.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.viewport_w_scalings.add(i as usize).read(),
            ));
        }
        Self {
            viewport_w_scaling_enable: crate::conv::FromLowLevel::from_low_level(
                context,
                value.viewport_w_scaling_enable,
            ),
            viewport_w_scalings: viewport_w_scalings,
        }
    }
}
