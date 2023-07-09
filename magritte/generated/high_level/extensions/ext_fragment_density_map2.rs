pub use crate::common::extensions::ext_fragment_density_map2::{
    EXT_FRAGMENT_DENSITY_MAP_2_EXTENSION_NAME, EXT_FRAGMENT_DENSITY_MAP_2_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceFragmentDensityMap2FeaturesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceFragmentDensityMap2FeaturesEXT {
    #[doc(alias = "fragmentDensityMapDeferred")]
    pub fragment_density_map_deferred: bool,
}
impl PhysicalDeviceFragmentDensityMap2FeaturesEXT {
    ///Get a reference to the `fragment_density_map_deferred` field.
    pub fn fragment_density_map_deferred(&self) -> &bool {
        &self.fragment_density_map_deferred
    }
    ///Get a mutable reference to the `fragment_density_map_deferred` field.
    pub fn fragment_density_map_deferred_mut(&mut self) -> &mut bool {
        &mut self.fragment_density_map_deferred
    }
    ///Sets the `fragment_density_map_deferred` field.
    pub fn set_fragment_density_map_deferred(&mut self, fragment_density_map_deferred: bool) -> &mut Self {
        self.fragment_density_map_deferred = fragment_density_map_deferred;
        self
    }
    ///Sets the `fragment_density_map_deferred` field in a builder way.
    pub fn with_fragment_density_map_deferred(mut self, fragment_density_map_deferred: bool) -> Self {
        self.fragment_density_map_deferred = fragment_density_map_deferred;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceFragmentDensityMap2FeaturesEXT {
    type LowLevel = crate::native::extensions::ext_fragment_density_map2::PhysicalDeviceFragmentDensityMap2FeaturesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_fragment_density_map2::PhysicalDeviceFragmentDensityMap2FeaturesEXT {
            s_type: StructureType::PhysicalDeviceFragmentDensityMap2FeaturesExt,
            p_next: std::ptr::null_mut(),
            fragment_density_map_deferred: self.fragment_density_map_deferred.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceFragmentDensityMap2FeaturesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            fragment_density_map_deferred: crate::conv::FromLowLevel::from_low_level(
                context,
                value.fragment_density_map_deferred,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceFragmentDensityMap2PropertiesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceFragmentDensityMap2PropertiesEXT {
    #[doc(alias = "subsampledLoads")]
    pub subsampled_loads: bool,
    #[doc(alias = "subsampledCoarseReconstructionEarlyAccess")]
    pub subsampled_coarse_reconstruction_early_access: bool,
    #[doc(alias = "maxSubsampledArrayLayers")]
    pub max_subsampled_array_layers: u32,
    #[doc(alias = "maxDescriptorSetSubsampledSamplers")]
    pub max_descriptor_set_subsampled_samplers: u32,
}
impl PhysicalDeviceFragmentDensityMap2PropertiesEXT {
    ///Get a reference to the `subsampled_loads` field.
    pub fn subsampled_loads(&self) -> &bool {
        &self.subsampled_loads
    }
    ///Get a reference to the `subsampled_coarse_reconstruction_early_access` field.
    pub fn subsampled_coarse_reconstruction_early_access(&self) -> &bool {
        &self.subsampled_coarse_reconstruction_early_access
    }
    ///Get a reference to the `max_subsampled_array_layers` field.
    pub fn max_subsampled_array_layers(&self) -> u32 {
        self.max_subsampled_array_layers
    }
    ///Get a reference to the `max_descriptor_set_subsampled_samplers` field.
    pub fn max_descriptor_set_subsampled_samplers(&self) -> u32 {
        self.max_descriptor_set_subsampled_samplers
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceFragmentDensityMap2PropertiesEXT {
    type LowLevel =
        crate::native::extensions::ext_fragment_density_map2::PhysicalDeviceFragmentDensityMap2PropertiesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_fragment_density_map2::PhysicalDeviceFragmentDensityMap2PropertiesEXT {
            s_type: StructureType::PhysicalDeviceFragmentDensityMap2PropertiesExt,
            p_next: std::ptr::null_mut(),
            subsampled_loads: self.subsampled_loads.into_low_level(context, bump),
            subsampled_coarse_reconstruction_early_access: self
                .subsampled_coarse_reconstruction_early_access
                .into_low_level(context, bump),
            max_subsampled_array_layers: self.max_subsampled_array_layers.into_low_level(context, bump),
            max_descriptor_set_subsampled_samplers: self
                .max_descriptor_set_subsampled_samplers
                .into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceFragmentDensityMap2PropertiesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            subsampled_loads: crate::conv::FromLowLevel::from_low_level(context, value.subsampled_loads),
            subsampled_coarse_reconstruction_early_access: crate::conv::FromLowLevel::from_low_level(
                context,
                value.subsampled_coarse_reconstruction_early_access,
            ),
            max_subsampled_array_layers: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_subsampled_array_layers,
            ),
            max_descriptor_set_subsampled_samplers: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_descriptor_set_subsampled_samplers,
            ),
        }
    }
}
