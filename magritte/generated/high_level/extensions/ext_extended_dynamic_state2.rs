pub use crate::common::extensions::ext_extended_dynamic_state2::{
    EXT_EXTENDED_DYNAMIC_STATE_2_EXTENSION_NAME, EXT_EXTENDED_DYNAMIC_STATE_2_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceExtendedDynamicState2FeaturesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceExtendedDynamicState2FeaturesEXT {
    #[doc(alias = "extendedDynamicState2")]
    pub extended_dynamic_state2: bool,
    #[doc(alias = "extendedDynamicState2LogicOp")]
    pub extended_dynamic_state2_logic_op: bool,
    #[doc(alias = "extendedDynamicState2PatchControlPoints")]
    pub extended_dynamic_state2_patch_control_points: bool,
}
impl PhysicalDeviceExtendedDynamicState2FeaturesEXT {
    ///Get a reference to the `extended_dynamic_state2` field.
    pub fn extended_dynamic_state2(&self) -> &bool {
        &self.extended_dynamic_state2
    }
    ///Get a reference to the `extended_dynamic_state2_logic_op` field.
    pub fn extended_dynamic_state2_logic_op(&self) -> &bool {
        &self.extended_dynamic_state2_logic_op
    }
    ///Get a reference to the `extended_dynamic_state2_patch_control_points` field.
    pub fn extended_dynamic_state2_patch_control_points(&self) -> &bool {
        &self.extended_dynamic_state2_patch_control_points
    }
    ///Get a mutable reference to the `extended_dynamic_state2` field.
    pub fn extended_dynamic_state2_mut(&mut self) -> &mut bool {
        &mut self.extended_dynamic_state2
    }
    ///Get a mutable reference to the `extended_dynamic_state2_logic_op` field.
    pub fn extended_dynamic_state2_logic_op_mut(&mut self) -> &mut bool {
        &mut self.extended_dynamic_state2_logic_op
    }
    ///Get a mutable reference to the `extended_dynamic_state2_patch_control_points` field.
    pub fn extended_dynamic_state2_patch_control_points_mut(&mut self) -> &mut bool {
        &mut self.extended_dynamic_state2_patch_control_points
    }
    ///Sets the `extended_dynamic_state2` field.
    pub fn set_extended_dynamic_state2(&mut self, extended_dynamic_state2: bool) -> &mut Self {
        self.extended_dynamic_state2 = extended_dynamic_state2;
        self
    }
    ///Sets the `extended_dynamic_state2_logic_op` field.
    pub fn set_extended_dynamic_state2_logic_op(&mut self, extended_dynamic_state2_logic_op: bool) -> &mut Self {
        self.extended_dynamic_state2_logic_op = extended_dynamic_state2_logic_op;
        self
    }
    ///Sets the `extended_dynamic_state2_patch_control_points` field.
    pub fn set_extended_dynamic_state2_patch_control_points(
        &mut self,
        extended_dynamic_state2_patch_control_points: bool,
    ) -> &mut Self {
        self.extended_dynamic_state2_patch_control_points = extended_dynamic_state2_patch_control_points;
        self
    }
    ///Sets the `extended_dynamic_state2` field in a builder way.
    pub fn with_extended_dynamic_state2(mut self, extended_dynamic_state2: bool) -> Self {
        self.extended_dynamic_state2 = extended_dynamic_state2;
        self
    }
    ///Sets the `extended_dynamic_state2_logic_op` field in a builder way.
    pub fn with_extended_dynamic_state2_logic_op(mut self, extended_dynamic_state2_logic_op: bool) -> Self {
        self.extended_dynamic_state2_logic_op = extended_dynamic_state2_logic_op;
        self
    }
    ///Sets the `extended_dynamic_state2_patch_control_points` field in a builder way.
    pub fn with_extended_dynamic_state2_patch_control_points(
        mut self,
        extended_dynamic_state2_patch_control_points: bool,
    ) -> Self {
        self.extended_dynamic_state2_patch_control_points = extended_dynamic_state2_patch_control_points;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceExtendedDynamicState2FeaturesEXT {
    type LowLevel =
        crate::native::extensions::ext_extended_dynamic_state2::PhysicalDeviceExtendedDynamicState2FeaturesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_extended_dynamic_state2::PhysicalDeviceExtendedDynamicState2FeaturesEXT {
            s_type: StructureType::PhysicalDeviceExtendedDynamicState2FeaturesExt,
            p_next: std::ptr::null_mut(),
            extended_dynamic_state2: self.extended_dynamic_state2.into_low_level(context, bump),
            extended_dynamic_state2_logic_op: self.extended_dynamic_state2_logic_op.into_low_level(context, bump),
            extended_dynamic_state2_patch_control_points: self
                .extended_dynamic_state2_patch_control_points
                .into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceExtendedDynamicState2FeaturesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            extended_dynamic_state2: crate::conv::FromLowLevel::from_low_level(context, value.extended_dynamic_state2),
            extended_dynamic_state2_logic_op: crate::conv::FromLowLevel::from_low_level(
                context,
                value.extended_dynamic_state2_logic_op,
            ),
            extended_dynamic_state2_patch_control_points: crate::conv::FromLowLevel::from_low_level(
                context,
                value.extended_dynamic_state2_patch_control_points,
            ),
        }
    }
}
