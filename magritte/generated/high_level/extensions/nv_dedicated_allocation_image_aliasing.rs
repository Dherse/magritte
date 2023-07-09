pub use crate::common::extensions::nv_dedicated_allocation_image_aliasing::{
    NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_EXTENSION_NAME, NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
    #[doc(alias = "dedicatedAllocationImageAliasing")]
    pub dedicated_allocation_image_aliasing: bool,
}
impl PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
    ///Get a reference to the `dedicated_allocation_image_aliasing` field.
    pub fn dedicated_allocation_image_aliasing(&self) -> &bool {
        &self.dedicated_allocation_image_aliasing
    }
    ///Get a mutable reference to the `dedicated_allocation_image_aliasing` field.
    pub fn dedicated_allocation_image_aliasing_mut(&mut self) -> &mut bool {
        &mut self.dedicated_allocation_image_aliasing
    }
    ///Sets the `dedicated_allocation_image_aliasing` field.
    pub fn set_dedicated_allocation_image_aliasing(&mut self, dedicated_allocation_image_aliasing: bool) -> &mut Self {
        self.dedicated_allocation_image_aliasing = dedicated_allocation_image_aliasing;
        self
    }
    ///Sets the `dedicated_allocation_image_aliasing` field in a builder way.
    pub fn with_dedicated_allocation_image_aliasing(mut self, dedicated_allocation_image_aliasing: bool) -> Self {
        self.dedicated_allocation_image_aliasing = dedicated_allocation_image_aliasing;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
    type LowLevel = crate :: native :: extensions :: nv_dedicated_allocation_image_aliasing :: PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV ;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate :: native :: extensions :: nv_dedicated_allocation_image_aliasing :: PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV { s_type : StructureType :: PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNv , p_next : std :: ptr :: null_mut () , dedicated_allocation_image_aliasing : self . dedicated_allocation_image_aliasing . into_low_level (context , bump) }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            dedicated_allocation_image_aliasing: crate::conv::FromLowLevel::from_low_level(
                context,
                value.dedicated_allocation_image_aliasing,
            ),
        }
    }
}
