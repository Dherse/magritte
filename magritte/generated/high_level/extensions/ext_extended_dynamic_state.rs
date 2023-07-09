pub use crate::common::extensions::ext_extended_dynamic_state::{
    EXT_EXTENDED_DYNAMIC_STATE_EXTENSION_NAME, EXT_EXTENDED_DYNAMIC_STATE_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceExtendedDynamicStateFeaturesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceExtendedDynamicStateFeaturesEXT {
    #[doc(alias = "extendedDynamicState")]
    pub extended_dynamic_state: bool,
}
impl PhysicalDeviceExtendedDynamicStateFeaturesEXT {
    ///Get a reference to the `extended_dynamic_state` field.
    pub fn extended_dynamic_state(&self) -> &bool {
        &self.extended_dynamic_state
    }
    ///Get a mutable reference to the `extended_dynamic_state` field.
    pub fn extended_dynamic_state_mut(&mut self) -> &mut bool {
        &mut self.extended_dynamic_state
    }
    ///Sets the `extended_dynamic_state` field.
    pub fn set_extended_dynamic_state(&mut self, extended_dynamic_state: bool) -> &mut Self {
        self.extended_dynamic_state = extended_dynamic_state;
        self
    }
    ///Sets the `extended_dynamic_state` field in a builder way.
    pub fn with_extended_dynamic_state(mut self, extended_dynamic_state: bool) -> Self {
        self.extended_dynamic_state = extended_dynamic_state;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceExtendedDynamicStateFeaturesEXT {
    type LowLevel =
        crate::native::extensions::ext_extended_dynamic_state::PhysicalDeviceExtendedDynamicStateFeaturesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_extended_dynamic_state::PhysicalDeviceExtendedDynamicStateFeaturesEXT {
            s_type: StructureType::PhysicalDeviceExtendedDynamicStateFeaturesExt,
            p_next: std::ptr::null_mut(),
            extended_dynamic_state: self.extended_dynamic_state.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceExtendedDynamicStateFeaturesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            extended_dynamic_state: crate::conv::FromLowLevel::from_low_level(context, value.extended_dynamic_state),
        }
    }
}
