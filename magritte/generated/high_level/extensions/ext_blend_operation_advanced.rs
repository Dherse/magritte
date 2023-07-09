pub use crate::common::extensions::ext_blend_operation_advanced::{
    BlendOverlapEXT, EXT_BLEND_OPERATION_ADVANCED_EXTENSION_NAME, EXT_BLEND_OPERATION_ADVANCED_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    #[doc(alias = "advancedBlendCoherentOperations")]
    pub advanced_blend_coherent_operations: bool,
}
impl PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    ///Get a reference to the `advanced_blend_coherent_operations` field.
    pub fn advanced_blend_coherent_operations(&self) -> &bool {
        &self.advanced_blend_coherent_operations
    }
    ///Get a mutable reference to the `advanced_blend_coherent_operations` field.
    pub fn advanced_blend_coherent_operations_mut(&mut self) -> &mut bool {
        &mut self.advanced_blend_coherent_operations
    }
    ///Sets the `advanced_blend_coherent_operations` field.
    pub fn set_advanced_blend_coherent_operations(&mut self, advanced_blend_coherent_operations: bool) -> &mut Self {
        self.advanced_blend_coherent_operations = advanced_blend_coherent_operations;
        self
    }
    ///Sets the `advanced_blend_coherent_operations` field in a builder way.
    pub fn with_advanced_blend_coherent_operations(mut self, advanced_blend_coherent_operations: bool) -> Self {
        self.advanced_blend_coherent_operations = advanced_blend_coherent_operations;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    type LowLevel =
        crate::native::extensions::ext_blend_operation_advanced::PhysicalDeviceBlendOperationAdvancedFeaturesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_blend_operation_advanced::PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
            s_type: StructureType::PhysicalDeviceBlendOperationAdvancedFeaturesExt,
            p_next: std::ptr::null_mut(),
            advanced_blend_coherent_operations: self.advanced_blend_coherent_operations.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            advanced_blend_coherent_operations: crate::conv::FromLowLevel::from_low_level(
                context,
                value.advanced_blend_coherent_operations,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    #[doc(alias = "advancedBlendMaxColorAttachments")]
    pub advanced_blend_max_color_attachments: u32,
    #[doc(alias = "advancedBlendIndependentBlend")]
    pub advanced_blend_independent_blend: bool,
    #[doc(alias = "advancedBlendNonPremultipliedSrcColor")]
    pub advanced_blend_non_premultiplied_src_color: bool,
    #[doc(alias = "advancedBlendNonPremultipliedDstColor")]
    pub advanced_blend_non_premultiplied_dst_color: bool,
    #[doc(alias = "advancedBlendCorrelatedOverlap")]
    pub advanced_blend_correlated_overlap: bool,
    #[doc(alias = "advancedBlendAllOperations")]
    pub advanced_blend_all_operations: bool,
}
impl PhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    ///Get a reference to the `advanced_blend_max_color_attachments` field.
    pub fn advanced_blend_max_color_attachments(&self) -> u32 {
        self.advanced_blend_max_color_attachments
    }
    ///Get a reference to the `advanced_blend_independent_blend` field.
    pub fn advanced_blend_independent_blend(&self) -> &bool {
        &self.advanced_blend_independent_blend
    }
    ///Get a reference to the `advanced_blend_non_premultiplied_src_color` field.
    pub fn advanced_blend_non_premultiplied_src_color(&self) -> &bool {
        &self.advanced_blend_non_premultiplied_src_color
    }
    ///Get a reference to the `advanced_blend_non_premultiplied_dst_color` field.
    pub fn advanced_blend_non_premultiplied_dst_color(&self) -> &bool {
        &self.advanced_blend_non_premultiplied_dst_color
    }
    ///Get a reference to the `advanced_blend_correlated_overlap` field.
    pub fn advanced_blend_correlated_overlap(&self) -> &bool {
        &self.advanced_blend_correlated_overlap
    }
    ///Get a reference to the `advanced_blend_all_operations` field.
    pub fn advanced_blend_all_operations(&self) -> &bool {
        &self.advanced_blend_all_operations
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    type LowLevel =
        crate::native::extensions::ext_blend_operation_advanced::PhysicalDeviceBlendOperationAdvancedPropertiesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_blend_operation_advanced::PhysicalDeviceBlendOperationAdvancedPropertiesEXT {
            s_type: StructureType::PhysicalDeviceBlendOperationAdvancedPropertiesExt,
            p_next: std::ptr::null_mut(),
            advanced_blend_max_color_attachments: self
                .advanced_blend_max_color_attachments
                .into_low_level(context, bump),
            advanced_blend_independent_blend: self.advanced_blend_independent_blend.into_low_level(context, bump),
            advanced_blend_non_premultiplied_src_color: self
                .advanced_blend_non_premultiplied_src_color
                .into_low_level(context, bump),
            advanced_blend_non_premultiplied_dst_color: self
                .advanced_blend_non_premultiplied_dst_color
                .into_low_level(context, bump),
            advanced_blend_correlated_overlap: self.advanced_blend_correlated_overlap.into_low_level(context, bump),
            advanced_blend_all_operations: self.advanced_blend_all_operations.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            advanced_blend_max_color_attachments: crate::conv::FromLowLevel::from_low_level(
                context,
                value.advanced_blend_max_color_attachments,
            ),
            advanced_blend_independent_blend: crate::conv::FromLowLevel::from_low_level(
                context,
                value.advanced_blend_independent_blend,
            ),
            advanced_blend_non_premultiplied_src_color: crate::conv::FromLowLevel::from_low_level(
                context,
                value.advanced_blend_non_premultiplied_src_color,
            ),
            advanced_blend_non_premultiplied_dst_color: crate::conv::FromLowLevel::from_low_level(
                context,
                value.advanced_blend_non_premultiplied_dst_color,
            ),
            advanced_blend_correlated_overlap: crate::conv::FromLowLevel::from_low_level(
                context,
                value.advanced_blend_correlated_overlap,
            ),
            advanced_blend_all_operations: crate::conv::FromLowLevel::from_low_level(
                context,
                value.advanced_blend_all_operations,
            ),
        }
    }
}
#[doc(alias = "VkPipelineColorBlendAdvancedStateCreateInfoEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineColorBlendAdvancedStateCreateInfoEXT {
    #[doc(alias = "srcPremultiplied")]
    pub src_premultiplied: bool,
    #[doc(alias = "dstPremultiplied")]
    pub dst_premultiplied: bool,
    #[doc(alias = "blendOverlap")]
    pub blend_overlap: BlendOverlapEXT,
}
impl PipelineColorBlendAdvancedStateCreateInfoEXT {
    ///Get a reference to the `src_premultiplied` field.
    pub fn src_premultiplied(&self) -> &bool {
        &self.src_premultiplied
    }
    ///Get a reference to the `dst_premultiplied` field.
    pub fn dst_premultiplied(&self) -> &bool {
        &self.dst_premultiplied
    }
    ///Get a reference to the `blend_overlap` field.
    pub fn blend_overlap(&self) -> BlendOverlapEXT {
        self.blend_overlap
    }
    ///Get a mutable reference to the `src_premultiplied` field.
    pub fn src_premultiplied_mut(&mut self) -> &mut bool {
        &mut self.src_premultiplied
    }
    ///Get a mutable reference to the `dst_premultiplied` field.
    pub fn dst_premultiplied_mut(&mut self) -> &mut bool {
        &mut self.dst_premultiplied
    }
    ///Get a mutable reference to the `blend_overlap` field.
    pub fn blend_overlap_mut(&mut self) -> &mut BlendOverlapEXT {
        &mut self.blend_overlap
    }
    ///Sets the `src_premultiplied` field.
    pub fn set_src_premultiplied(&mut self, src_premultiplied: bool) -> &mut Self {
        self.src_premultiplied = src_premultiplied;
        self
    }
    ///Sets the `dst_premultiplied` field.
    pub fn set_dst_premultiplied(&mut self, dst_premultiplied: bool) -> &mut Self {
        self.dst_premultiplied = dst_premultiplied;
        self
    }
    ///Sets the `blend_overlap` field.
    pub fn set_blend_overlap(&mut self, blend_overlap: BlendOverlapEXT) -> &mut Self {
        self.blend_overlap = blend_overlap;
        self
    }
    ///Sets the `src_premultiplied` field in a builder way.
    pub fn with_src_premultiplied(mut self, src_premultiplied: bool) -> Self {
        self.src_premultiplied = src_premultiplied;
        self
    }
    ///Sets the `dst_premultiplied` field in a builder way.
    pub fn with_dst_premultiplied(mut self, dst_premultiplied: bool) -> Self {
        self.dst_premultiplied = dst_premultiplied;
        self
    }
    ///Sets the `blend_overlap` field in a builder way.
    pub fn with_blend_overlap(mut self, blend_overlap: BlendOverlapEXT) -> Self {
        self.blend_overlap = blend_overlap;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineColorBlendAdvancedStateCreateInfoEXT {
    type LowLevel =
        crate::native::extensions::ext_blend_operation_advanced::PipelineColorBlendAdvancedStateCreateInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_blend_operation_advanced::PipelineColorBlendAdvancedStateCreateInfoEXT {
            s_type: StructureType::PipelineColorBlendAdvancedStateCreateInfoExt,
            p_next: std::ptr::null(),
            src_premultiplied: self.src_premultiplied.into_low_level(context, bump),
            dst_premultiplied: self.dst_premultiplied.into_low_level(context, bump),
            blend_overlap: self.blend_overlap.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineColorBlendAdvancedStateCreateInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            src_premultiplied: crate::conv::FromLowLevel::from_low_level(context, value.src_premultiplied),
            dst_premultiplied: crate::conv::FromLowLevel::from_low_level(context, value.dst_premultiplied),
            blend_overlap: crate::conv::FromLowLevel::from_low_level(context, value.blend_overlap),
        }
    }
}
