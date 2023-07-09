pub use crate::common::extensions::google_display_timing::{
    PastPresentationTimingGOOGLE, PresentTimeGOOGLE, RefreshCycleDurationGOOGLE, GOOGLE_DISPLAY_TIMING_EXTENSION_NAME,
    GOOGLE_DISPLAY_TIMING_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
impl RefreshCycleDurationGOOGLE {
    ///Get a reference to the `refresh_duration` field.
    pub fn refresh_duration(&self) -> u64 {
        self.refresh_duration
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for RefreshCycleDurationGOOGLE {
    type LowLevel = crate::native::extensions::google_display_timing::RefreshCycleDurationGOOGLE;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::google_display_timing::RefreshCycleDurationGOOGLE {
            refresh_duration: self.refresh_duration.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for RefreshCycleDurationGOOGLE {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            refresh_duration: crate::conv::FromLowLevel::from_low_level(context, value.refresh_duration),
        }
    }
}
impl PastPresentationTimingGOOGLE {
    ///Get a reference to the `present_id` field.
    pub fn present_id(&self) -> u32 {
        self.present_id
    }
    ///Get a reference to the `desired_present_time` field.
    pub fn desired_present_time(&self) -> u64 {
        self.desired_present_time
    }
    ///Get a reference to the `actual_present_time` field.
    pub fn actual_present_time(&self) -> u64 {
        self.actual_present_time
    }
    ///Get a reference to the `earliest_present_time` field.
    pub fn earliest_present_time(&self) -> u64 {
        self.earliest_present_time
    }
    ///Get a reference to the `present_margin` field.
    pub fn present_margin(&self) -> u64 {
        self.present_margin
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PastPresentationTimingGOOGLE {
    type LowLevel = crate::native::extensions::google_display_timing::PastPresentationTimingGOOGLE;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::google_display_timing::PastPresentationTimingGOOGLE {
            present_id: self.present_id.into_low_level(context, bump),
            desired_present_time: self.desired_present_time.into_low_level(context, bump),
            actual_present_time: self.actual_present_time.into_low_level(context, bump),
            earliest_present_time: self.earliest_present_time.into_low_level(context, bump),
            present_margin: self.present_margin.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PastPresentationTimingGOOGLE {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            present_id: crate::conv::FromLowLevel::from_low_level(context, value.present_id),
            desired_present_time: crate::conv::FromLowLevel::from_low_level(context, value.desired_present_time),
            actual_present_time: crate::conv::FromLowLevel::from_low_level(context, value.actual_present_time),
            earliest_present_time: crate::conv::FromLowLevel::from_low_level(context, value.earliest_present_time),
            present_margin: crate::conv::FromLowLevel::from_low_level(context, value.present_margin),
        }
    }
}
#[doc(alias = "VkPresentTimesInfoGOOGLE")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PresentTimesInfoGOOGLE {
    #[doc(alias = "pTimes")]
    pub times: SmallVec<[PresentTimeGOOGLE; 8]>,
}
impl PresentTimesInfoGOOGLE {
    ///Get a reference to the `times` field.
    pub fn times(&self) -> &SmallVec<[PresentTimeGOOGLE; 8]> {
        &self.times
    }
    ///Get a mutable reference to the `times` field.
    pub fn times_mut(&mut self) -> &mut SmallVec<[PresentTimeGOOGLE; 8]> {
        &mut self.times
    }
    ///Sets the `times` field.
    pub fn set_times(&mut self, times: SmallVec<[PresentTimeGOOGLE; 8]>) -> &mut Self {
        self.times = times;
        self
    }
    ///Sets the `times` field in a builder way.
    pub fn with_times(mut self, times: SmallVec<[PresentTimeGOOGLE; 8]>) -> Self {
        self.times = times;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PresentTimesInfoGOOGLE {
    type LowLevel = crate::native::extensions::google_display_timing::PresentTimesInfoGOOGLE;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_times = self.times.len() as u32;
        let times = bump
            .alloc_slice_fill_iter(self.times.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::extensions::google_display_timing::PresentTimesInfoGOOGLE {
            s_type: StructureType::PresentTimesInfoGoogle,
            p_next: std::ptr::null(),
            swapchain_count: len_times,
            times: times,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PresentTimesInfoGOOGLE {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let times_len = value.swapchain_count;
        let mut times = SmallVec::with_capacity(times_len as usize);
        for i in 0..times_len {
            times.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.times.add(i as usize).read(),
            ));
        }
        Self { times: times }
    }
}
impl PresentTimeGOOGLE {
    ///Get a reference to the `present_id` field.
    pub fn present_id(&self) -> u32 {
        self.present_id
    }
    ///Get a reference to the `desired_present_time` field.
    pub fn desired_present_time(&self) -> u64 {
        self.desired_present_time
    }
    ///Get a mutable reference to the `present_id` field.
    pub fn present_id_mut(&mut self) -> &mut u32 {
        &mut self.present_id
    }
    ///Get a mutable reference to the `desired_present_time` field.
    pub fn desired_present_time_mut(&mut self) -> &mut u64 {
        &mut self.desired_present_time
    }
    ///Sets the `present_id` field.
    pub fn set_present_id(&mut self, present_id: u32) -> &mut Self {
        self.present_id = present_id;
        self
    }
    ///Sets the `desired_present_time` field.
    pub fn set_desired_present_time(&mut self, desired_present_time: u64) -> &mut Self {
        self.desired_present_time = desired_present_time;
        self
    }
    ///Sets the `present_id` field in a builder way.
    pub fn with_present_id(mut self, present_id: u32) -> Self {
        self.present_id = present_id;
        self
    }
    ///Sets the `desired_present_time` field in a builder way.
    pub fn with_desired_present_time(mut self, desired_present_time: u64) -> Self {
        self.desired_present_time = desired_present_time;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PresentTimeGOOGLE {
    type LowLevel = crate::native::extensions::google_display_timing::PresentTimeGOOGLE;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::google_display_timing::PresentTimeGOOGLE {
            present_id: self.present_id.into_low_level(context, bump),
            desired_present_time: self.desired_present_time.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PresentTimeGOOGLE {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            present_id: crate::conv::FromLowLevel::from_low_level(context, value.present_id),
            desired_present_time: crate::conv::FromLowLevel::from_low_level(context, value.desired_present_time),
        }
    }
}
