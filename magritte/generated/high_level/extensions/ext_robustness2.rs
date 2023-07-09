pub use crate::common::extensions::ext_robustness2::{EXT_ROBUSTNESS_2_EXTENSION_NAME, EXT_ROBUSTNESS_2_SPEC_VERSION};
use crate::{
    context::Context,
    vulkan1_0::{DeviceSize, StructureType},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceRobustness2FeaturesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceRobustness2FeaturesEXT {
    #[doc(alias = "robustBufferAccess2")]
    pub robust_buffer_access2: bool,
    #[doc(alias = "robustImageAccess2")]
    pub robust_image_access2: bool,
    #[doc(alias = "nullDescriptor")]
    pub null_descriptor: bool,
}
impl PhysicalDeviceRobustness2FeaturesEXT {
    ///Get a reference to the `robust_buffer_access2` field.
    pub fn robust_buffer_access2(&self) -> &bool {
        &self.robust_buffer_access2
    }
    ///Get a reference to the `robust_image_access2` field.
    pub fn robust_image_access2(&self) -> &bool {
        &self.robust_image_access2
    }
    ///Get a reference to the `null_descriptor` field.
    pub fn null_descriptor(&self) -> &bool {
        &self.null_descriptor
    }
    ///Get a mutable reference to the `robust_buffer_access2` field.
    pub fn robust_buffer_access2_mut(&mut self) -> &mut bool {
        &mut self.robust_buffer_access2
    }
    ///Get a mutable reference to the `robust_image_access2` field.
    pub fn robust_image_access2_mut(&mut self) -> &mut bool {
        &mut self.robust_image_access2
    }
    ///Get a mutable reference to the `null_descriptor` field.
    pub fn null_descriptor_mut(&mut self) -> &mut bool {
        &mut self.null_descriptor
    }
    ///Sets the `robust_buffer_access2` field.
    pub fn set_robust_buffer_access2(&mut self, robust_buffer_access2: bool) -> &mut Self {
        self.robust_buffer_access2 = robust_buffer_access2;
        self
    }
    ///Sets the `robust_image_access2` field.
    pub fn set_robust_image_access2(&mut self, robust_image_access2: bool) -> &mut Self {
        self.robust_image_access2 = robust_image_access2;
        self
    }
    ///Sets the `null_descriptor` field.
    pub fn set_null_descriptor(&mut self, null_descriptor: bool) -> &mut Self {
        self.null_descriptor = null_descriptor;
        self
    }
    ///Sets the `robust_buffer_access2` field in a builder way.
    pub fn with_robust_buffer_access2(mut self, robust_buffer_access2: bool) -> Self {
        self.robust_buffer_access2 = robust_buffer_access2;
        self
    }
    ///Sets the `robust_image_access2` field in a builder way.
    pub fn with_robust_image_access2(mut self, robust_image_access2: bool) -> Self {
        self.robust_image_access2 = robust_image_access2;
        self
    }
    ///Sets the `null_descriptor` field in a builder way.
    pub fn with_null_descriptor(mut self, null_descriptor: bool) -> Self {
        self.null_descriptor = null_descriptor;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceRobustness2FeaturesEXT {
    type LowLevel = crate::native::extensions::ext_robustness2::PhysicalDeviceRobustness2FeaturesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_robustness2::PhysicalDeviceRobustness2FeaturesEXT {
            s_type: StructureType::PhysicalDeviceRobustness2FeaturesExt,
            p_next: std::ptr::null_mut(),
            robust_buffer_access2: self.robust_buffer_access2.into_low_level(context, bump),
            robust_image_access2: self.robust_image_access2.into_low_level(context, bump),
            null_descriptor: self.null_descriptor.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceRobustness2FeaturesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            robust_buffer_access2: crate::conv::FromLowLevel::from_low_level(context, value.robust_buffer_access2),
            robust_image_access2: crate::conv::FromLowLevel::from_low_level(context, value.robust_image_access2),
            null_descriptor: crate::conv::FromLowLevel::from_low_level(context, value.null_descriptor),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceRobustness2PropertiesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceRobustness2PropertiesEXT {
    #[doc(alias = "robustStorageBufferAccessSizeAlignment")]
    pub robust_storage_buffer_access_size_alignment: DeviceSize,
    #[doc(alias = "robustUniformBufferAccessSizeAlignment")]
    pub robust_uniform_buffer_access_size_alignment: DeviceSize,
}
impl PhysicalDeviceRobustness2PropertiesEXT {
    ///Get a reference to the `robust_storage_buffer_access_size_alignment` field.
    pub fn robust_storage_buffer_access_size_alignment(&self) -> &DeviceSize {
        &self.robust_storage_buffer_access_size_alignment
    }
    ///Get a reference to the `robust_uniform_buffer_access_size_alignment` field.
    pub fn robust_uniform_buffer_access_size_alignment(&self) -> &DeviceSize {
        &self.robust_uniform_buffer_access_size_alignment
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceRobustness2PropertiesEXT {
    type LowLevel = crate::native::extensions::ext_robustness2::PhysicalDeviceRobustness2PropertiesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_robustness2::PhysicalDeviceRobustness2PropertiesEXT {
            s_type: StructureType::PhysicalDeviceRobustness2PropertiesExt,
            p_next: std::ptr::null_mut(),
            robust_storage_buffer_access_size_alignment: self
                .robust_storage_buffer_access_size_alignment
                .into_low_level(context, bump),
            robust_uniform_buffer_access_size_alignment: self
                .robust_uniform_buffer_access_size_alignment
                .into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceRobustness2PropertiesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            robust_storage_buffer_access_size_alignment: crate::conv::FromLowLevel::from_low_level(
                context,
                value.robust_storage_buffer_access_size_alignment,
            ),
            robust_uniform_buffer_access_size_alignment: crate::conv::FromLowLevel::from_low_level(
                context,
                value.robust_uniform_buffer_access_size_alignment,
            ),
        }
    }
}
