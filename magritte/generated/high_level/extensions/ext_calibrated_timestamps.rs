pub use crate::common::extensions::ext_calibrated_timestamps::{
    TimeDomainEXT, EXT_CALIBRATED_TIMESTAMPS_EXTENSION_NAME, EXT_CALIBRATED_TIMESTAMPS_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkCalibratedTimestampInfoEXT")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CalibratedTimestampInfoEXT {
    #[doc(alias = "timeDomain")]
    pub time_domain: TimeDomainEXT,
}
impl CalibratedTimestampInfoEXT {
    ///Get a reference to the `time_domain` field.
    pub fn time_domain(&self) -> TimeDomainEXT {
        self.time_domain
    }
    ///Get a mutable reference to the `time_domain` field.
    pub fn time_domain_mut(&mut self) -> &mut TimeDomainEXT {
        &mut self.time_domain
    }
    ///Sets the `time_domain` field.
    pub fn set_time_domain(&mut self, time_domain: TimeDomainEXT) -> &mut Self {
        self.time_domain = time_domain;
        self
    }
    ///Sets the `time_domain` field in a builder way.
    pub fn with_time_domain(mut self, time_domain: TimeDomainEXT) -> Self {
        self.time_domain = time_domain;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for CalibratedTimestampInfoEXT {
    type LowLevel = crate::native::extensions::ext_calibrated_timestamps::CalibratedTimestampInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_calibrated_timestamps::CalibratedTimestampInfoEXT {
            s_type: StructureType::CalibratedTimestampInfoExt,
            p_next: std::ptr::null(),
            time_domain: self.time_domain.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for CalibratedTimestampInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            time_domain: crate::conv::FromLowLevel::from_low_level(context, value.time_domain),
        }
    }
}
