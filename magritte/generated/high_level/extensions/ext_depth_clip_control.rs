pub use crate::common::extensions::ext_depth_clip_control::{
    EXT_DEPTH_CLIP_CONTROL_EXTENSION_NAME, EXT_DEPTH_CLIP_CONTROL_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceDepthClipControlFeaturesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceDepthClipControlFeaturesEXT {
    #[doc(alias = "depthClipControl")]
    pub depth_clip_control: bool,
}
impl PhysicalDeviceDepthClipControlFeaturesEXT {
    ///Get a reference to the `depth_clip_control` field.
    pub fn depth_clip_control(&self) -> &bool {
        &self.depth_clip_control
    }
    ///Get a mutable reference to the `depth_clip_control` field.
    pub fn depth_clip_control_mut(&mut self) -> &mut bool {
        &mut self.depth_clip_control
    }
    ///Sets the `depth_clip_control` field.
    pub fn set_depth_clip_control(&mut self, depth_clip_control: bool) -> &mut Self {
        self.depth_clip_control = depth_clip_control;
        self
    }
    ///Sets the `depth_clip_control` field in a builder way.
    pub fn with_depth_clip_control(mut self, depth_clip_control: bool) -> Self {
        self.depth_clip_control = depth_clip_control;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceDepthClipControlFeaturesEXT {
    type LowLevel = crate::native::extensions::ext_depth_clip_control::PhysicalDeviceDepthClipControlFeaturesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_depth_clip_control::PhysicalDeviceDepthClipControlFeaturesEXT {
            s_type: StructureType::PhysicalDeviceDepthClipControlFeaturesExt,
            p_next: std::ptr::null_mut(),
            depth_clip_control: self.depth_clip_control.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceDepthClipControlFeaturesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            depth_clip_control: crate::conv::FromLowLevel::from_low_level(context, value.depth_clip_control),
        }
    }
}
#[doc(alias = "VkPipelineViewportDepthClipControlCreateInfoEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineViewportDepthClipControlCreateInfoEXT {
    #[doc(alias = "negativeOneToOne")]
    pub negative_one_to_one: bool,
}
impl PipelineViewportDepthClipControlCreateInfoEXT {
    ///Get a reference to the `negative_one_to_one` field.
    pub fn negative_one_to_one(&self) -> &bool {
        &self.negative_one_to_one
    }
    ///Get a mutable reference to the `negative_one_to_one` field.
    pub fn negative_one_to_one_mut(&mut self) -> &mut bool {
        &mut self.negative_one_to_one
    }
    ///Sets the `negative_one_to_one` field.
    pub fn set_negative_one_to_one(&mut self, negative_one_to_one: bool) -> &mut Self {
        self.negative_one_to_one = negative_one_to_one;
        self
    }
    ///Sets the `negative_one_to_one` field in a builder way.
    pub fn with_negative_one_to_one(mut self, negative_one_to_one: bool) -> Self {
        self.negative_one_to_one = negative_one_to_one;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineViewportDepthClipControlCreateInfoEXT {
    type LowLevel = crate::native::extensions::ext_depth_clip_control::PipelineViewportDepthClipControlCreateInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_depth_clip_control::PipelineViewportDepthClipControlCreateInfoEXT {
            s_type: StructureType::PipelineViewportDepthClipControlCreateInfoExt,
            p_next: std::ptr::null(),
            negative_one_to_one: self.negative_one_to_one.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineViewportDepthClipControlCreateInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            negative_one_to_one: crate::conv::FromLowLevel::from_low_level(context, value.negative_one_to_one),
        }
    }
}
