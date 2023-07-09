pub use crate::common::extensions::khr_workgroup_memory_explicit_layout::{
    KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_EXTENSION_NAME, KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
    #[doc(alias = "workgroupMemoryExplicitLayout")]
    pub workgroup_memory_explicit_layout: bool,
    #[doc(alias = "workgroupMemoryExplicitLayoutScalarBlockLayout")]
    pub workgroup_memory_explicit_layout_scalar_block_layout: bool,
    #[doc(alias = "workgroupMemoryExplicitLayout8BitAccess")]
    pub workgroup_memory_explicit_layout8_bit_access: bool,
    #[doc(alias = "workgroupMemoryExplicitLayout16BitAccess")]
    pub workgroup_memory_explicit_layout16_bit_access: bool,
}
impl PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
    ///Get a reference to the `workgroup_memory_explicit_layout` field.
    pub fn workgroup_memory_explicit_layout(&self) -> &bool {
        &self.workgroup_memory_explicit_layout
    }
    ///Get a reference to the `workgroup_memory_explicit_layout_scalar_block_layout` field.
    pub fn workgroup_memory_explicit_layout_scalar_block_layout(&self) -> &bool {
        &self.workgroup_memory_explicit_layout_scalar_block_layout
    }
    ///Get a reference to the `workgroup_memory_explicit_layout8_bit_access` field.
    pub fn workgroup_memory_explicit_layout8_bit_access(&self) -> &bool {
        &self.workgroup_memory_explicit_layout8_bit_access
    }
    ///Get a reference to the `workgroup_memory_explicit_layout16_bit_access` field.
    pub fn workgroup_memory_explicit_layout16_bit_access(&self) -> &bool {
        &self.workgroup_memory_explicit_layout16_bit_access
    }
    ///Get a mutable reference to the `workgroup_memory_explicit_layout` field.
    pub fn workgroup_memory_explicit_layout_mut(&mut self) -> &mut bool {
        &mut self.workgroup_memory_explicit_layout
    }
    ///Get a mutable reference to the `workgroup_memory_explicit_layout_scalar_block_layout` field.
    pub fn workgroup_memory_explicit_layout_scalar_block_layout_mut(&mut self) -> &mut bool {
        &mut self.workgroup_memory_explicit_layout_scalar_block_layout
    }
    ///Get a mutable reference to the `workgroup_memory_explicit_layout8_bit_access` field.
    pub fn workgroup_memory_explicit_layout8_bit_access_mut(&mut self) -> &mut bool {
        &mut self.workgroup_memory_explicit_layout8_bit_access
    }
    ///Get a mutable reference to the `workgroup_memory_explicit_layout16_bit_access` field.
    pub fn workgroup_memory_explicit_layout16_bit_access_mut(&mut self) -> &mut bool {
        &mut self.workgroup_memory_explicit_layout16_bit_access
    }
    ///Sets the `workgroup_memory_explicit_layout` field.
    pub fn set_workgroup_memory_explicit_layout(&mut self, workgroup_memory_explicit_layout: bool) -> &mut Self {
        self.workgroup_memory_explicit_layout = workgroup_memory_explicit_layout;
        self
    }
    ///Sets the `workgroup_memory_explicit_layout_scalar_block_layout` field.
    pub fn set_workgroup_memory_explicit_layout_scalar_block_layout(
        &mut self,
        workgroup_memory_explicit_layout_scalar_block_layout: bool,
    ) -> &mut Self {
        self.workgroup_memory_explicit_layout_scalar_block_layout =
            workgroup_memory_explicit_layout_scalar_block_layout;
        self
    }
    ///Sets the `workgroup_memory_explicit_layout8_bit_access` field.
    pub fn set_workgroup_memory_explicit_layout8_bit_access(
        &mut self,
        workgroup_memory_explicit_layout8_bit_access: bool,
    ) -> &mut Self {
        self.workgroup_memory_explicit_layout8_bit_access = workgroup_memory_explicit_layout8_bit_access;
        self
    }
    ///Sets the `workgroup_memory_explicit_layout16_bit_access` field.
    pub fn set_workgroup_memory_explicit_layout16_bit_access(
        &mut self,
        workgroup_memory_explicit_layout16_bit_access: bool,
    ) -> &mut Self {
        self.workgroup_memory_explicit_layout16_bit_access = workgroup_memory_explicit_layout16_bit_access;
        self
    }
    ///Sets the `workgroup_memory_explicit_layout` field in a builder way.
    pub fn with_workgroup_memory_explicit_layout(mut self, workgroup_memory_explicit_layout: bool) -> Self {
        self.workgroup_memory_explicit_layout = workgroup_memory_explicit_layout;
        self
    }
    ///Sets the `workgroup_memory_explicit_layout_scalar_block_layout` field in a builder way.
    pub fn with_workgroup_memory_explicit_layout_scalar_block_layout(
        mut self,
        workgroup_memory_explicit_layout_scalar_block_layout: bool,
    ) -> Self {
        self.workgroup_memory_explicit_layout_scalar_block_layout =
            workgroup_memory_explicit_layout_scalar_block_layout;
        self
    }
    ///Sets the `workgroup_memory_explicit_layout8_bit_access` field in a builder way.
    pub fn with_workgroup_memory_explicit_layout8_bit_access(
        mut self,
        workgroup_memory_explicit_layout8_bit_access: bool,
    ) -> Self {
        self.workgroup_memory_explicit_layout8_bit_access = workgroup_memory_explicit_layout8_bit_access;
        self
    }
    ///Sets the `workgroup_memory_explicit_layout16_bit_access` field in a builder way.
    pub fn with_workgroup_memory_explicit_layout16_bit_access(
        mut self,
        workgroup_memory_explicit_layout16_bit_access: bool,
    ) -> Self {
        self.workgroup_memory_explicit_layout16_bit_access = workgroup_memory_explicit_layout16_bit_access;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
    type LowLevel = crate :: native :: extensions :: khr_workgroup_memory_explicit_layout :: PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR ;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate :: native :: extensions :: khr_workgroup_memory_explicit_layout :: PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR { s_type : StructureType :: PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKhr , p_next : std :: ptr :: null_mut () , workgroup_memory_explicit_layout : self . workgroup_memory_explicit_layout . into_low_level (context , bump) , workgroup_memory_explicit_layout_scalar_block_layout : self . workgroup_memory_explicit_layout_scalar_block_layout . into_low_level (context , bump) , workgroup_memory_explicit_layout8_bit_access : self . workgroup_memory_explicit_layout8_bit_access . into_low_level (context , bump) , workgroup_memory_explicit_layout16_bit_access : self . workgroup_memory_explicit_layout16_bit_access . into_low_level (context , bump) }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            workgroup_memory_explicit_layout: crate::conv::FromLowLevel::from_low_level(
                context,
                value.workgroup_memory_explicit_layout,
            ),
            workgroup_memory_explicit_layout_scalar_block_layout: crate::conv::FromLowLevel::from_low_level(
                context,
                value.workgroup_memory_explicit_layout_scalar_block_layout,
            ),
            workgroup_memory_explicit_layout8_bit_access: crate::conv::FromLowLevel::from_low_level(
                context,
                value.workgroup_memory_explicit_layout8_bit_access,
            ),
            workgroup_memory_explicit_layout16_bit_access: crate::conv::FromLowLevel::from_low_level(
                context,
                value.workgroup_memory_explicit_layout16_bit_access,
            ),
        }
    }
}
