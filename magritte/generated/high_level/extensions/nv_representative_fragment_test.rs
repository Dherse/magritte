pub use crate::common::extensions::nv_representative_fragment_test::{
    NV_REPRESENTATIVE_FRAGMENT_TEST_EXTENSION_NAME, NV_REPRESENTATIVE_FRAGMENT_TEST_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceRepresentativeFragmentTestFeaturesNV {
    #[doc(alias = "representativeFragmentTest")]
    pub representative_fragment_test: bool,
}
impl PhysicalDeviceRepresentativeFragmentTestFeaturesNV {
    ///Get a reference to the `representative_fragment_test` field.
    pub fn representative_fragment_test(&self) -> &bool {
        &self.representative_fragment_test
    }
    ///Get a mutable reference to the `representative_fragment_test` field.
    pub fn representative_fragment_test_mut(&mut self) -> &mut bool {
        &mut self.representative_fragment_test
    }
    ///Sets the `representative_fragment_test` field.
    pub fn set_representative_fragment_test(&mut self, representative_fragment_test: bool) -> &mut Self {
        self.representative_fragment_test = representative_fragment_test;
        self
    }
    ///Sets the `representative_fragment_test` field in a builder way.
    pub fn with_representative_fragment_test(mut self, representative_fragment_test: bool) -> Self {
        self.representative_fragment_test = representative_fragment_test;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceRepresentativeFragmentTestFeaturesNV {
    type LowLevel =
        crate::native::extensions::nv_representative_fragment_test::PhysicalDeviceRepresentativeFragmentTestFeaturesNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_representative_fragment_test::PhysicalDeviceRepresentativeFragmentTestFeaturesNV {
            s_type: StructureType::PhysicalDeviceRepresentativeFragmentTestFeaturesNv,
            p_next: std::ptr::null_mut(),
            representative_fragment_test: self.representative_fragment_test.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceRepresentativeFragmentTestFeaturesNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            representative_fragment_test: crate::conv::FromLowLevel::from_low_level(
                context,
                value.representative_fragment_test,
            ),
        }
    }
}
#[doc(alias = "VkPipelineRepresentativeFragmentTestStateCreateInfoNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineRepresentativeFragmentTestStateCreateInfoNV {
    #[doc(alias = "representativeFragmentTestEnable")]
    pub representative_fragment_test_enable: bool,
}
impl PipelineRepresentativeFragmentTestStateCreateInfoNV {
    ///Get a reference to the `representative_fragment_test_enable` field.
    pub fn representative_fragment_test_enable(&self) -> &bool {
        &self.representative_fragment_test_enable
    }
    ///Get a mutable reference to the `representative_fragment_test_enable` field.
    pub fn representative_fragment_test_enable_mut(&mut self) -> &mut bool {
        &mut self.representative_fragment_test_enable
    }
    ///Sets the `representative_fragment_test_enable` field.
    pub fn set_representative_fragment_test_enable(&mut self, representative_fragment_test_enable: bool) -> &mut Self {
        self.representative_fragment_test_enable = representative_fragment_test_enable;
        self
    }
    ///Sets the `representative_fragment_test_enable` field in a builder way.
    pub fn with_representative_fragment_test_enable(mut self, representative_fragment_test_enable: bool) -> Self {
        self.representative_fragment_test_enable = representative_fragment_test_enable;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineRepresentativeFragmentTestStateCreateInfoNV {
    type LowLevel =
        crate::native::extensions::nv_representative_fragment_test::PipelineRepresentativeFragmentTestStateCreateInfoNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate :: native :: extensions :: nv_representative_fragment_test :: PipelineRepresentativeFragmentTestStateCreateInfoNV { s_type : StructureType :: PipelineRepresentativeFragmentTestStateCreateInfoNv , p_next : std :: ptr :: null () , representative_fragment_test_enable : self . representative_fragment_test_enable . into_low_level (context , bump) }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineRepresentativeFragmentTestStateCreateInfoNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            representative_fragment_test_enable: crate::conv::FromLowLevel::from_low_level(
                context,
                value.representative_fragment_test_enable,
            ),
        }
    }
}
