pub use crate::common::extensions::ext_depth_clip_enable::{
    PipelineRasterizationDepthClipStateCreateFlagsEXT, EXT_DEPTH_CLIP_ENABLE_EXTENSION_NAME,
    EXT_DEPTH_CLIP_ENABLE_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceDepthClipEnableFeaturesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceDepthClipEnableFeaturesEXT {
    #[doc(alias = "depthClipEnable")]
    pub depth_clip_enable: bool,
}
impl PhysicalDeviceDepthClipEnableFeaturesEXT {
    ///Get a reference to the `depth_clip_enable` field.
    pub fn depth_clip_enable(&self) -> &bool {
        &self.depth_clip_enable
    }
    ///Get a mutable reference to the `depth_clip_enable` field.
    pub fn depth_clip_enable_mut(&mut self) -> &mut bool {
        &mut self.depth_clip_enable
    }
    ///Sets the `depth_clip_enable` field.
    pub fn set_depth_clip_enable(&mut self, depth_clip_enable: bool) -> &mut Self {
        self.depth_clip_enable = depth_clip_enable;
        self
    }
    ///Sets the `depth_clip_enable` field in a builder way.
    pub fn with_depth_clip_enable(mut self, depth_clip_enable: bool) -> Self {
        self.depth_clip_enable = depth_clip_enable;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceDepthClipEnableFeaturesEXT {
    type LowLevel = crate::native::extensions::ext_depth_clip_enable::PhysicalDeviceDepthClipEnableFeaturesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_depth_clip_enable::PhysicalDeviceDepthClipEnableFeaturesEXT {
            s_type: StructureType::PhysicalDeviceDepthClipEnableFeaturesExt,
            p_next: std::ptr::null_mut(),
            depth_clip_enable: self.depth_clip_enable.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceDepthClipEnableFeaturesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            depth_clip_enable: crate::conv::FromLowLevel::from_low_level(context, value.depth_clip_enable),
        }
    }
}
#[doc(alias = "VkPipelineRasterizationDepthClipStateCreateInfoEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineRasterizationDepthClipStateCreateInfoEXT {
    pub flags: PipelineRasterizationDepthClipStateCreateFlagsEXT,
    #[doc(alias = "depthClipEnable")]
    pub depth_clip_enable: bool,
}
impl PipelineRasterizationDepthClipStateCreateInfoEXT {
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> PipelineRasterizationDepthClipStateCreateFlagsEXT {
        self.flags
    }
    ///Get a reference to the `depth_clip_enable` field.
    pub fn depth_clip_enable(&self) -> &bool {
        &self.depth_clip_enable
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut PipelineRasterizationDepthClipStateCreateFlagsEXT {
        &mut self.flags
    }
    ///Get a mutable reference to the `depth_clip_enable` field.
    pub fn depth_clip_enable_mut(&mut self) -> &mut bool {
        &mut self.depth_clip_enable
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: PipelineRasterizationDepthClipStateCreateFlagsEXT) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `depth_clip_enable` field.
    pub fn set_depth_clip_enable(&mut self, depth_clip_enable: bool) -> &mut Self {
        self.depth_clip_enable = depth_clip_enable;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: PipelineRasterizationDepthClipStateCreateFlagsEXT) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `depth_clip_enable` field in a builder way.
    pub fn with_depth_clip_enable(mut self, depth_clip_enable: bool) -> Self {
        self.depth_clip_enable = depth_clip_enable;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineRasterizationDepthClipStateCreateInfoEXT {
    type LowLevel = crate::native::extensions::ext_depth_clip_enable::PipelineRasterizationDepthClipStateCreateInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_depth_clip_enable::PipelineRasterizationDepthClipStateCreateInfoEXT {
            s_type: StructureType::PipelineRasterizationDepthClipStateCreateInfoExt,
            p_next: std::ptr::null(),
            flags: self.flags.into_low_level(context, bump),
            depth_clip_enable: self.depth_clip_enable.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineRasterizationDepthClipStateCreateInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            depth_clip_enable: crate::conv::FromLowLevel::from_low_level(context, value.depth_clip_enable),
        }
    }
}
