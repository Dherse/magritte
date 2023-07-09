pub use crate::common::extensions::nv_fragment_shading_rate_enums::{
    FragmentShadingRateNV, FragmentShadingRateTypeNV, NV_FRAGMENT_SHADING_RATE_ENUMS_EXTENSION_NAME,
    NV_FRAGMENT_SHADING_RATE_ENUMS_SPEC_VERSION,
};
use crate::{
    context::Context,
    extensions::khr_fragment_shading_rate::FragmentShadingRateCombinerOpKHR,
    vulkan1_0::{SampleCountFlagBits, StructureType},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceFragmentShadingRateEnumsFeaturesNV {
    #[doc(alias = "fragmentShadingRateEnums")]
    pub fragment_shading_rate_enums: bool,
    #[doc(alias = "supersampleFragmentShadingRates")]
    pub supersample_fragment_shading_rates: bool,
    #[doc(alias = "noInvocationFragmentShadingRates")]
    pub no_invocation_fragment_shading_rates: bool,
}
impl PhysicalDeviceFragmentShadingRateEnumsFeaturesNV {
    ///Get a reference to the `fragment_shading_rate_enums` field.
    pub fn fragment_shading_rate_enums(&self) -> &bool {
        &self.fragment_shading_rate_enums
    }
    ///Get a reference to the `supersample_fragment_shading_rates` field.
    pub fn supersample_fragment_shading_rates(&self) -> &bool {
        &self.supersample_fragment_shading_rates
    }
    ///Get a reference to the `no_invocation_fragment_shading_rates` field.
    pub fn no_invocation_fragment_shading_rates(&self) -> &bool {
        &self.no_invocation_fragment_shading_rates
    }
    ///Get a mutable reference to the `fragment_shading_rate_enums` field.
    pub fn fragment_shading_rate_enums_mut(&mut self) -> &mut bool {
        &mut self.fragment_shading_rate_enums
    }
    ///Get a mutable reference to the `supersample_fragment_shading_rates` field.
    pub fn supersample_fragment_shading_rates_mut(&mut self) -> &mut bool {
        &mut self.supersample_fragment_shading_rates
    }
    ///Get a mutable reference to the `no_invocation_fragment_shading_rates` field.
    pub fn no_invocation_fragment_shading_rates_mut(&mut self) -> &mut bool {
        &mut self.no_invocation_fragment_shading_rates
    }
    ///Sets the `fragment_shading_rate_enums` field.
    pub fn set_fragment_shading_rate_enums(&mut self, fragment_shading_rate_enums: bool) -> &mut Self {
        self.fragment_shading_rate_enums = fragment_shading_rate_enums;
        self
    }
    ///Sets the `supersample_fragment_shading_rates` field.
    pub fn set_supersample_fragment_shading_rates(&mut self, supersample_fragment_shading_rates: bool) -> &mut Self {
        self.supersample_fragment_shading_rates = supersample_fragment_shading_rates;
        self
    }
    ///Sets the `no_invocation_fragment_shading_rates` field.
    pub fn set_no_invocation_fragment_shading_rates(
        &mut self,
        no_invocation_fragment_shading_rates: bool,
    ) -> &mut Self {
        self.no_invocation_fragment_shading_rates = no_invocation_fragment_shading_rates;
        self
    }
    ///Sets the `fragment_shading_rate_enums` field in a builder way.
    pub fn with_fragment_shading_rate_enums(mut self, fragment_shading_rate_enums: bool) -> Self {
        self.fragment_shading_rate_enums = fragment_shading_rate_enums;
        self
    }
    ///Sets the `supersample_fragment_shading_rates` field in a builder way.
    pub fn with_supersample_fragment_shading_rates(mut self, supersample_fragment_shading_rates: bool) -> Self {
        self.supersample_fragment_shading_rates = supersample_fragment_shading_rates;
        self
    }
    ///Sets the `no_invocation_fragment_shading_rates` field in a builder way.
    pub fn with_no_invocation_fragment_shading_rates(mut self, no_invocation_fragment_shading_rates: bool) -> Self {
        self.no_invocation_fragment_shading_rates = no_invocation_fragment_shading_rates;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceFragmentShadingRateEnumsFeaturesNV {
    type LowLevel =
        crate::native::extensions::nv_fragment_shading_rate_enums::PhysicalDeviceFragmentShadingRateEnumsFeaturesNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_fragment_shading_rate_enums::PhysicalDeviceFragmentShadingRateEnumsFeaturesNV {
            s_type: StructureType::PhysicalDeviceFragmentShadingRateEnumsFeaturesNv,
            p_next: std::ptr::null_mut(),
            fragment_shading_rate_enums: self.fragment_shading_rate_enums.into_low_level(context, bump),
            supersample_fragment_shading_rates: self.supersample_fragment_shading_rates.into_low_level(context, bump),
            no_invocation_fragment_shading_rates: self
                .no_invocation_fragment_shading_rates
                .into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceFragmentShadingRateEnumsFeaturesNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            fragment_shading_rate_enums: crate::conv::FromLowLevel::from_low_level(
                context,
                value.fragment_shading_rate_enums,
            ),
            supersample_fragment_shading_rates: crate::conv::FromLowLevel::from_low_level(
                context,
                value.supersample_fragment_shading_rates,
            ),
            no_invocation_fragment_shading_rates: crate::conv::FromLowLevel::from_low_level(
                context,
                value.no_invocation_fragment_shading_rates,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceFragmentShadingRateEnumsPropertiesNV {
    #[doc(alias = "maxFragmentShadingRateInvocationCount")]
    pub max_fragment_shading_rate_invocation_count: SampleCountFlagBits,
}
impl PhysicalDeviceFragmentShadingRateEnumsPropertiesNV {
    ///Get a reference to the `max_fragment_shading_rate_invocation_count` field.
    pub fn max_fragment_shading_rate_invocation_count(&self) -> SampleCountFlagBits {
        self.max_fragment_shading_rate_invocation_count
    }
    ///Get a mutable reference to the `max_fragment_shading_rate_invocation_count` field.
    pub fn max_fragment_shading_rate_invocation_count_mut(&mut self) -> &mut SampleCountFlagBits {
        &mut self.max_fragment_shading_rate_invocation_count
    }
    ///Sets the `max_fragment_shading_rate_invocation_count` field.
    pub fn set_max_fragment_shading_rate_invocation_count(
        &mut self,
        max_fragment_shading_rate_invocation_count: SampleCountFlagBits,
    ) -> &mut Self {
        self.max_fragment_shading_rate_invocation_count = max_fragment_shading_rate_invocation_count;
        self
    }
    ///Sets the `max_fragment_shading_rate_invocation_count` field in a builder way.
    pub fn with_max_fragment_shading_rate_invocation_count(
        mut self,
        max_fragment_shading_rate_invocation_count: SampleCountFlagBits,
    ) -> Self {
        self.max_fragment_shading_rate_invocation_count = max_fragment_shading_rate_invocation_count;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceFragmentShadingRateEnumsPropertiesNV {
    type LowLevel =
        crate::native::extensions::nv_fragment_shading_rate_enums::PhysicalDeviceFragmentShadingRateEnumsPropertiesNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_fragment_shading_rate_enums::PhysicalDeviceFragmentShadingRateEnumsPropertiesNV {
            s_type: StructureType::PhysicalDeviceFragmentShadingRateEnumsPropertiesNv,
            p_next: std::ptr::null_mut(),
            max_fragment_shading_rate_invocation_count: self
                .max_fragment_shading_rate_invocation_count
                .into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceFragmentShadingRateEnumsPropertiesNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            max_fragment_shading_rate_invocation_count: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_fragment_shading_rate_invocation_count,
            ),
        }
    }
}
#[doc(alias = "VkPipelineFragmentShadingRateEnumStateCreateInfoNV")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineFragmentShadingRateEnumStateCreateInfoNV {
    #[doc(alias = "shadingRateType")]
    pub shading_rate_type: FragmentShadingRateTypeNV,
    #[doc(alias = "shadingRate")]
    pub shading_rate: FragmentShadingRateNV,
    #[doc(alias = "combinerOps")]
    pub combiner_ops: [FragmentShadingRateCombinerOpKHR; 2 as usize],
}
impl PipelineFragmentShadingRateEnumStateCreateInfoNV {
    ///Get a reference to the `shading_rate_type` field.
    pub fn shading_rate_type(&self) -> FragmentShadingRateTypeNV {
        self.shading_rate_type
    }
    ///Get a reference to the `shading_rate` field.
    pub fn shading_rate(&self) -> FragmentShadingRateNV {
        self.shading_rate
    }
    ///Get a reference to the `combiner_ops` field.
    pub fn combiner_ops(&self) -> [FragmentShadingRateCombinerOpKHR; 2 as usize] {
        self.combiner_ops
    }
    ///Get a mutable reference to the `shading_rate_type` field.
    pub fn shading_rate_type_mut(&mut self) -> &mut FragmentShadingRateTypeNV {
        &mut self.shading_rate_type
    }
    ///Get a mutable reference to the `shading_rate` field.
    pub fn shading_rate_mut(&mut self) -> &mut FragmentShadingRateNV {
        &mut self.shading_rate
    }
    ///Get a mutable reference to the `combiner_ops` field.
    pub fn combiner_ops_mut(&mut self) -> &mut [FragmentShadingRateCombinerOpKHR; 2 as usize] {
        &mut self.combiner_ops
    }
    ///Sets the `shading_rate_type` field.
    pub fn set_shading_rate_type(&mut self, shading_rate_type: FragmentShadingRateTypeNV) -> &mut Self {
        self.shading_rate_type = shading_rate_type;
        self
    }
    ///Sets the `shading_rate` field.
    pub fn set_shading_rate(&mut self, shading_rate: FragmentShadingRateNV) -> &mut Self {
        self.shading_rate = shading_rate;
        self
    }
    ///Sets the `combiner_ops` field.
    pub fn set_combiner_ops(&mut self, combiner_ops: [FragmentShadingRateCombinerOpKHR; 2 as usize]) -> &mut Self {
        self.combiner_ops = combiner_ops;
        self
    }
    ///Sets the `shading_rate_type` field in a builder way.
    pub fn with_shading_rate_type(mut self, shading_rate_type: FragmentShadingRateTypeNV) -> Self {
        self.shading_rate_type = shading_rate_type;
        self
    }
    ///Sets the `shading_rate` field in a builder way.
    pub fn with_shading_rate(mut self, shading_rate: FragmentShadingRateNV) -> Self {
        self.shading_rate = shading_rate;
        self
    }
    ///Sets the `combiner_ops` field in a builder way.
    pub fn with_combiner_ops(mut self, combiner_ops: [FragmentShadingRateCombinerOpKHR; 2 as usize]) -> Self {
        self.combiner_ops = combiner_ops;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineFragmentShadingRateEnumStateCreateInfoNV {
    type LowLevel =
        crate::native::extensions::nv_fragment_shading_rate_enums::PipelineFragmentShadingRateEnumStateCreateInfoNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_fragment_shading_rate_enums::PipelineFragmentShadingRateEnumStateCreateInfoNV {
            s_type: StructureType::PipelineFragmentShadingRateEnumStateCreateInfoNv,
            p_next: std::ptr::null(),
            shading_rate_type: self.shading_rate_type.into_low_level(context, bump),
            shading_rate: self.shading_rate.into_low_level(context, bump),
            combiner_ops: self.combiner_ops.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineFragmentShadingRateEnumStateCreateInfoNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            shading_rate_type: crate::conv::FromLowLevel::from_low_level(context, value.shading_rate_type),
            shading_rate: crate::conv::FromLowLevel::from_low_level(context, value.shading_rate),
            combiner_ops: crate::conv::FromLowLevel::from_low_level(context, value.combiner_ops),
        }
    }
}
