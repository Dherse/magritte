pub use crate::common::extensions::ext_primitive_topology_list_restart::{
    EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_EXTENSION_NAME, EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {
    #[doc(alias = "primitiveTopologyListRestart")]
    pub primitive_topology_list_restart: bool,
    #[doc(alias = "primitiveTopologyPatchListRestart")]
    pub primitive_topology_patch_list_restart: bool,
}
impl PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {
    ///Get a reference to the `primitive_topology_list_restart` field.
    pub fn primitive_topology_list_restart(&self) -> &bool {
        &self.primitive_topology_list_restart
    }
    ///Get a reference to the `primitive_topology_patch_list_restart` field.
    pub fn primitive_topology_patch_list_restart(&self) -> &bool {
        &self.primitive_topology_patch_list_restart
    }
    ///Get a mutable reference to the `primitive_topology_list_restart` field.
    pub fn primitive_topology_list_restart_mut(&mut self) -> &mut bool {
        &mut self.primitive_topology_list_restart
    }
    ///Get a mutable reference to the `primitive_topology_patch_list_restart` field.
    pub fn primitive_topology_patch_list_restart_mut(&mut self) -> &mut bool {
        &mut self.primitive_topology_patch_list_restart
    }
    ///Sets the `primitive_topology_list_restart` field.
    pub fn set_primitive_topology_list_restart(&mut self, primitive_topology_list_restart: bool) -> &mut Self {
        self.primitive_topology_list_restart = primitive_topology_list_restart;
        self
    }
    ///Sets the `primitive_topology_patch_list_restart` field.
    pub fn set_primitive_topology_patch_list_restart(
        &mut self,
        primitive_topology_patch_list_restart: bool,
    ) -> &mut Self {
        self.primitive_topology_patch_list_restart = primitive_topology_patch_list_restart;
        self
    }
    ///Sets the `primitive_topology_list_restart` field in a builder way.
    pub fn with_primitive_topology_list_restart(mut self, primitive_topology_list_restart: bool) -> Self {
        self.primitive_topology_list_restart = primitive_topology_list_restart;
        self
    }
    ///Sets the `primitive_topology_patch_list_restart` field in a builder way.
    pub fn with_primitive_topology_patch_list_restart(mut self, primitive_topology_patch_list_restart: bool) -> Self {
        self.primitive_topology_patch_list_restart = primitive_topology_patch_list_restart;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {
    type LowLevel = crate :: native :: extensions :: ext_primitive_topology_list_restart :: PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT ;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate :: native :: extensions :: ext_primitive_topology_list_restart :: PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT { s_type : StructureType :: PhysicalDevicePrimitiveTopologyListRestartFeaturesExt , p_next : std :: ptr :: null_mut () , primitive_topology_list_restart : self . primitive_topology_list_restart . into_low_level (context , bump) , primitive_topology_patch_list_restart : self . primitive_topology_patch_list_restart . into_low_level (context , bump) }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            primitive_topology_list_restart: crate::conv::FromLowLevel::from_low_level(
                context,
                value.primitive_topology_list_restart,
            ),
            primitive_topology_patch_list_restart: crate::conv::FromLowLevel::from_low_level(
                context,
                value.primitive_topology_patch_list_restart,
            ),
        }
    }
}
