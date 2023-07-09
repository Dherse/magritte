pub use crate::common::extensions::khr_ray_query::{KHR_RAY_QUERY_EXTENSION_NAME, KHR_RAY_QUERY_SPEC_VERSION};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceRayQueryFeaturesKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceRayQueryFeaturesKHR {
    #[doc(alias = "rayQuery")]
    pub ray_query: bool,
}
impl PhysicalDeviceRayQueryFeaturesKHR {
    ///Get a reference to the `ray_query` field.
    pub fn ray_query(&self) -> &bool {
        &self.ray_query
    }
    ///Get a mutable reference to the `ray_query` field.
    pub fn ray_query_mut(&mut self) -> &mut bool {
        &mut self.ray_query
    }
    ///Sets the `ray_query` field.
    pub fn set_ray_query(&mut self, ray_query: bool) -> &mut Self {
        self.ray_query = ray_query;
        self
    }
    ///Sets the `ray_query` field in a builder way.
    pub fn with_ray_query(mut self, ray_query: bool) -> Self {
        self.ray_query = ray_query;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceRayQueryFeaturesKHR {
    type LowLevel = crate::native::extensions::khr_ray_query::PhysicalDeviceRayQueryFeaturesKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_ray_query::PhysicalDeviceRayQueryFeaturesKHR {
            s_type: StructureType::PhysicalDeviceRayQueryFeaturesKhr,
            p_next: std::ptr::null_mut(),
            ray_query: self.ray_query.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceRayQueryFeaturesKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            ray_query: crate::conv::FromLowLevel::from_low_level(context, value.ray_query),
        }
    }
}
