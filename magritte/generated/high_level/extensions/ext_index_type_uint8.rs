pub use crate::common::extensions::ext_index_type_uint8::{
    EXT_INDEX_TYPE_UINT8_EXTENSION_NAME, EXT_INDEX_TYPE_UINT8_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceIndexTypeUint8FeaturesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceIndexTypeUint8FeaturesEXT {
    #[doc(alias = "indexTypeUint8")]
    pub index_type_uint8: bool,
}
impl PhysicalDeviceIndexTypeUint8FeaturesEXT {
    ///Get a reference to the `index_type_uint8` field.
    pub fn index_type_uint8(&self) -> &bool {
        &self.index_type_uint8
    }
    ///Get a mutable reference to the `index_type_uint8` field.
    pub fn index_type_uint8_mut(&mut self) -> &mut bool {
        &mut self.index_type_uint8
    }
    ///Sets the `index_type_uint8` field.
    pub fn set_index_type_uint8(&mut self, index_type_uint8: bool) -> &mut Self {
        self.index_type_uint8 = index_type_uint8;
        self
    }
    ///Sets the `index_type_uint8` field in a builder way.
    pub fn with_index_type_uint8(mut self, index_type_uint8: bool) -> Self {
        self.index_type_uint8 = index_type_uint8;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceIndexTypeUint8FeaturesEXT {
    type LowLevel = crate::native::extensions::ext_index_type_uint8::PhysicalDeviceIndexTypeUint8FeaturesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_index_type_uint8::PhysicalDeviceIndexTypeUint8FeaturesEXT {
            s_type: StructureType::PhysicalDeviceIndexTypeUint8FeaturesExt,
            p_next: std::ptr::null_mut(),
            index_type_uint8: self.index_type_uint8.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceIndexTypeUint8FeaturesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            index_type_uint8: crate::conv::FromLowLevel::from_low_level(context, value.index_type_uint8),
        }
    }
}
