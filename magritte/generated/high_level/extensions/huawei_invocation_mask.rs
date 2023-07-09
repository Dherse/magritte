pub use crate::common::extensions::huawei_invocation_mask::{
    HUAWEI_INVOCATION_MASK_EXTENSION_NAME, HUAWEI_INVOCATION_MASK_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceInvocationMaskFeaturesHUAWEI")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceInvocationMaskFeaturesHUAWEI {
    #[doc(alias = "invocationMask")]
    pub invocation_mask: bool,
}
impl PhysicalDeviceInvocationMaskFeaturesHUAWEI {
    ///Get a reference to the `invocation_mask` field.
    pub fn invocation_mask(&self) -> &bool {
        &self.invocation_mask
    }
    ///Get a mutable reference to the `invocation_mask` field.
    pub fn invocation_mask_mut(&mut self) -> &mut bool {
        &mut self.invocation_mask
    }
    ///Sets the `invocation_mask` field.
    pub fn set_invocation_mask(&mut self, invocation_mask: bool) -> &mut Self {
        self.invocation_mask = invocation_mask;
        self
    }
    ///Sets the `invocation_mask` field in a builder way.
    pub fn with_invocation_mask(mut self, invocation_mask: bool) -> Self {
        self.invocation_mask = invocation_mask;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceInvocationMaskFeaturesHUAWEI {
    type LowLevel = crate::native::extensions::huawei_invocation_mask::PhysicalDeviceInvocationMaskFeaturesHUAWEI;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::huawei_invocation_mask::PhysicalDeviceInvocationMaskFeaturesHUAWEI {
            s_type: StructureType::PhysicalDeviceInvocationMaskFeaturesHuawei,
            p_next: std::ptr::null_mut(),
            invocation_mask: self.invocation_mask.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceInvocationMaskFeaturesHUAWEI {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            invocation_mask: crate::conv::FromLowLevel::from_low_level(context, value.invocation_mask),
        }
    }
}
