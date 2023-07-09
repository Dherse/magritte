pub use crate::common::extensions::ext_ycbcr_image_arrays::{
    EXT_YCBCR_IMAGE_ARRAYS_EXTENSION_NAME, EXT_YCBCR_IMAGE_ARRAYS_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceYcbcrImageArraysFeaturesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceYcbcrImageArraysFeaturesEXT {
    #[doc(alias = "ycbcrImageArrays")]
    pub ycbcr_image_arrays: bool,
}
impl PhysicalDeviceYcbcrImageArraysFeaturesEXT {
    ///Get a reference to the `ycbcr_image_arrays` field.
    pub fn ycbcr_image_arrays(&self) -> &bool {
        &self.ycbcr_image_arrays
    }
    ///Get a mutable reference to the `ycbcr_image_arrays` field.
    pub fn ycbcr_image_arrays_mut(&mut self) -> &mut bool {
        &mut self.ycbcr_image_arrays
    }
    ///Sets the `ycbcr_image_arrays` field.
    pub fn set_ycbcr_image_arrays(&mut self, ycbcr_image_arrays: bool) -> &mut Self {
        self.ycbcr_image_arrays = ycbcr_image_arrays;
        self
    }
    ///Sets the `ycbcr_image_arrays` field in a builder way.
    pub fn with_ycbcr_image_arrays(mut self, ycbcr_image_arrays: bool) -> Self {
        self.ycbcr_image_arrays = ycbcr_image_arrays;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceYcbcrImageArraysFeaturesEXT {
    type LowLevel = crate::native::extensions::ext_ycbcr_image_arrays::PhysicalDeviceYcbcrImageArraysFeaturesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_ycbcr_image_arrays::PhysicalDeviceYcbcrImageArraysFeaturesEXT {
            s_type: StructureType::PhysicalDeviceYcbcrImageArraysFeaturesExt,
            p_next: std::ptr::null_mut(),
            ycbcr_image_arrays: self.ycbcr_image_arrays.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceYcbcrImageArraysFeaturesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            ycbcr_image_arrays: crate::conv::FromLowLevel::from_low_level(context, value.ycbcr_image_arrays),
        }
    }
}
