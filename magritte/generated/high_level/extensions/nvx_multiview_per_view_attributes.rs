pub use crate::common::extensions::nvx_multiview_per_view_attributes::{
    NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_EXTENSION_NAME, NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
    #[doc(alias = "perViewPositionAllComponents")]
    pub per_view_position_all_components: bool,
}
impl PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
    ///Get a reference to the `per_view_position_all_components` field.
    pub fn per_view_position_all_components(&self) -> &bool {
        &self.per_view_position_all_components
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
    type LowLevel = crate :: native :: extensions :: nvx_multiview_per_view_attributes :: PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX ;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate :: native :: extensions :: nvx_multiview_per_view_attributes :: PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX { s_type : StructureType :: PhysicalDeviceMultiviewPerViewAttributesPropertiesNvx , p_next : std :: ptr :: null_mut () , per_view_position_all_components : self . per_view_position_all_components . into_low_level (context , bump) }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            per_view_position_all_components: crate::conv::FromLowLevel::from_low_level(
                context,
                value.per_view_position_all_components,
            ),
        }
    }
}