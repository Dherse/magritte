pub use crate::common::extensions::nv_inherited_viewport_scissor::{
    NV_INHERITED_VIEWPORT_SCISSOR_EXTENSION_NAME, NV_INHERITED_VIEWPORT_SCISSOR_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{StructureType, Viewport},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceInheritedViewportScissorFeaturesNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceInheritedViewportScissorFeaturesNV {
    #[doc(alias = "inheritedViewportScissor2D")]
    pub inherited_viewport_scissor2_d: bool,
}
impl PhysicalDeviceInheritedViewportScissorFeaturesNV {
    ///Get a reference to the `inherited_viewport_scissor2_d` field.
    pub fn inherited_viewport_scissor2_d(&self) -> &bool {
        &self.inherited_viewport_scissor2_d
    }
    ///Get a mutable reference to the `inherited_viewport_scissor2_d` field.
    pub fn inherited_viewport_scissor2_d_mut(&mut self) -> &mut bool {
        &mut self.inherited_viewport_scissor2_d
    }
    ///Sets the `inherited_viewport_scissor2_d` field.
    pub fn set_inherited_viewport_scissor2_d(&mut self, inherited_viewport_scissor2_d: bool) -> &mut Self {
        self.inherited_viewport_scissor2_d = inherited_viewport_scissor2_d;
        self
    }
    ///Sets the `inherited_viewport_scissor2_d` field in a builder way.
    pub fn with_inherited_viewport_scissor2_d(mut self, inherited_viewport_scissor2_d: bool) -> Self {
        self.inherited_viewport_scissor2_d = inherited_viewport_scissor2_d;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceInheritedViewportScissorFeaturesNV {
    type LowLevel =
        crate::native::extensions::nv_inherited_viewport_scissor::PhysicalDeviceInheritedViewportScissorFeaturesNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_inherited_viewport_scissor::PhysicalDeviceInheritedViewportScissorFeaturesNV {
            s_type: StructureType::PhysicalDeviceInheritedViewportScissorFeaturesNv,
            p_next: std::ptr::null_mut(),
            inherited_viewport_scissor2_d: self.inherited_viewport_scissor2_d.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceInheritedViewportScissorFeaturesNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            inherited_viewport_scissor2_d: crate::conv::FromLowLevel::from_low_level(
                context,
                value.inherited_viewport_scissor2_d,
            ),
        }
    }
}
#[doc(alias = "VkCommandBufferInheritanceViewportScissorInfoNV")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CommandBufferInheritanceViewportScissorInfoNV {
    #[doc(alias = "viewportScissor2D")]
    pub viewport_scissor2_d: bool,
    #[doc(alias = "viewportDepthCount")]
    pub viewport_depth_count: u32,
    #[doc(alias = "pViewportDepths")]
    pub viewport_depths: Viewport,
}
impl CommandBufferInheritanceViewportScissorInfoNV {
    ///Get a reference to the `viewport_scissor2_d` field.
    pub fn viewport_scissor2_d(&self) -> &bool {
        &self.viewport_scissor2_d
    }
    ///Get a reference to the `viewport_depth_count` field.
    pub fn viewport_depth_count(&self) -> u32 {
        self.viewport_depth_count
    }
    ///Get a reference to the `viewport_depths` field.
    pub fn viewport_depths(&self) -> Viewport {
        self.viewport_depths
    }
    ///Get a mutable reference to the `viewport_scissor2_d` field.
    pub fn viewport_scissor2_d_mut(&mut self) -> &mut bool {
        &mut self.viewport_scissor2_d
    }
    ///Get a mutable reference to the `viewport_depth_count` field.
    pub fn viewport_depth_count_mut(&mut self) -> &mut u32 {
        &mut self.viewport_depth_count
    }
    ///Get a mutable reference to the `viewport_depths` field.
    pub fn viewport_depths_mut(&mut self) -> &mut Viewport {
        &mut self.viewport_depths
    }
    ///Sets the `viewport_scissor2_d` field.
    pub fn set_viewport_scissor2_d(&mut self, viewport_scissor2_d: bool) -> &mut Self {
        self.viewport_scissor2_d = viewport_scissor2_d;
        self
    }
    ///Sets the `viewport_depth_count` field.
    pub fn set_viewport_depth_count(&mut self, viewport_depth_count: u32) -> &mut Self {
        self.viewport_depth_count = viewport_depth_count;
        self
    }
    ///Sets the `viewport_depths` field.
    pub fn set_viewport_depths(&mut self, viewport_depths: Viewport) -> &mut Self {
        self.viewport_depths = viewport_depths;
        self
    }
    ///Sets the `viewport_scissor2_d` field in a builder way.
    pub fn with_viewport_scissor2_d(mut self, viewport_scissor2_d: bool) -> Self {
        self.viewport_scissor2_d = viewport_scissor2_d;
        self
    }
    ///Sets the `viewport_depth_count` field in a builder way.
    pub fn with_viewport_depth_count(mut self, viewport_depth_count: u32) -> Self {
        self.viewport_depth_count = viewport_depth_count;
        self
    }
    ///Sets the `viewport_depths` field in a builder way.
    pub fn with_viewport_depths(mut self, viewport_depths: Viewport) -> Self {
        self.viewport_depths = viewport_depths;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for CommandBufferInheritanceViewportScissorInfoNV {
    type LowLevel =
        crate::native::extensions::nv_inherited_viewport_scissor::CommandBufferInheritanceViewportScissorInfoNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_inherited_viewport_scissor::CommandBufferInheritanceViewportScissorInfoNV {
            s_type: StructureType::CommandBufferInheritanceViewportScissorInfoNv,
            p_next: std::ptr::null(),
            viewport_scissor2_d: self.viewport_scissor2_d.into_low_level(context, bump),
            viewport_depth_count: self.viewport_depth_count.into_low_level(context, bump),
            viewport_depths: bump.alloc(self.viewport_depths.into_low_level(context, bump)) as *const _,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for CommandBufferInheritanceViewportScissorInfoNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            viewport_scissor2_d: crate::conv::FromLowLevel::from_low_level(context, value.viewport_scissor2_d),
            viewport_depth_count: crate::conv::FromLowLevel::from_low_level(context, value.viewport_depth_count),
            viewport_depths: crate::conv::FromLowLevel::from_low_level(context, *value.viewport_depths),
        }
    }
}
