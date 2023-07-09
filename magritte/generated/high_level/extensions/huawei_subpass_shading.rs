pub use crate::common::extensions::huawei_subpass_shading::{
    HUAWEI_SUBPASS_SHADING_EXTENSION_NAME, HUAWEI_SUBPASS_SHADING_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{RenderPass, StructureType},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkSubpassShadingPipelineCreateInfoHUAWEI")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SubpassShadingPipelineCreateInfoHUAWEI {
    #[doc(alias = "renderPass")]
    pub render_pass: RenderPass,
    pub subpass: u32,
}
impl SubpassShadingPipelineCreateInfoHUAWEI {
    ///Get a reference to the `render_pass` field.
    pub fn render_pass(&self) -> &RenderPass {
        &self.render_pass
    }
    ///Get a reference to the `subpass` field.
    pub fn subpass(&self) -> u32 {
        self.subpass
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SubpassShadingPipelineCreateInfoHUAWEI {
    type LowLevel = crate::native::extensions::huawei_subpass_shading::SubpassShadingPipelineCreateInfoHUAWEI;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::huawei_subpass_shading::SubpassShadingPipelineCreateInfoHUAWEI {
            s_type: StructureType::SubpassShadingPipelineCreateInfoHuawei,
            p_next: std::ptr::null_mut(),
            render_pass: self.render_pass.into_low_level(context, bump),
            subpass: self.subpass.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SubpassShadingPipelineCreateInfoHUAWEI {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            render_pass: crate::conv::FromLowLevel::from_low_level(context, value.render_pass),
            subpass: crate::conv::FromLowLevel::from_low_level(context, value.subpass),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceSubpassShadingPropertiesHUAWEI")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceSubpassShadingPropertiesHUAWEI {
    #[doc(alias = "maxSubpassShadingWorkgroupSizeAspectRatio")]
    pub max_subpass_shading_workgroup_size_aspect_ratio: u32,
}
impl PhysicalDeviceSubpassShadingPropertiesHUAWEI {
    ///Get a reference to the `max_subpass_shading_workgroup_size_aspect_ratio` field.
    pub fn max_subpass_shading_workgroup_size_aspect_ratio(&self) -> u32 {
        self.max_subpass_shading_workgroup_size_aspect_ratio
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceSubpassShadingPropertiesHUAWEI {
    type LowLevel = crate::native::extensions::huawei_subpass_shading::PhysicalDeviceSubpassShadingPropertiesHUAWEI;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::huawei_subpass_shading::PhysicalDeviceSubpassShadingPropertiesHUAWEI {
            s_type: StructureType::PhysicalDeviceSubpassShadingPropertiesHuawei,
            p_next: std::ptr::null_mut(),
            max_subpass_shading_workgroup_size_aspect_ratio: self
                .max_subpass_shading_workgroup_size_aspect_ratio
                .into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceSubpassShadingPropertiesHUAWEI {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            max_subpass_shading_workgroup_size_aspect_ratio: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_subpass_shading_workgroup_size_aspect_ratio,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceSubpassShadingFeaturesHUAWEI")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceSubpassShadingFeaturesHUAWEI {
    #[doc(alias = "subpassShading")]
    pub subpass_shading: bool,
}
impl PhysicalDeviceSubpassShadingFeaturesHUAWEI {
    ///Get a reference to the `subpass_shading` field.
    pub fn subpass_shading(&self) -> &bool {
        &self.subpass_shading
    }
    ///Get a mutable reference to the `subpass_shading` field.
    pub fn subpass_shading_mut(&mut self) -> &mut bool {
        &mut self.subpass_shading
    }
    ///Sets the `subpass_shading` field.
    pub fn set_subpass_shading(&mut self, subpass_shading: bool) -> &mut Self {
        self.subpass_shading = subpass_shading;
        self
    }
    ///Sets the `subpass_shading` field in a builder way.
    pub fn with_subpass_shading(mut self, subpass_shading: bool) -> Self {
        self.subpass_shading = subpass_shading;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceSubpassShadingFeaturesHUAWEI {
    type LowLevel = crate::native::extensions::huawei_subpass_shading::PhysicalDeviceSubpassShadingFeaturesHUAWEI;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::huawei_subpass_shading::PhysicalDeviceSubpassShadingFeaturesHUAWEI {
            s_type: StructureType::PhysicalDeviceSubpassShadingFeaturesHuawei,
            p_next: std::ptr::null_mut(),
            subpass_shading: self.subpass_shading.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceSubpassShadingFeaturesHUAWEI {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            subpass_shading: crate::conv::FromLowLevel::from_low_level(context, value.subpass_shading),
        }
    }
}
