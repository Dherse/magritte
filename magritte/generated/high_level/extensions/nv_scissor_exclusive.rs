pub use crate::common::extensions::nv_scissor_exclusive::{
    NV_SCISSOR_EXCLUSIVE_EXTENSION_NAME, NV_SCISSOR_EXCLUSIVE_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{Rect2D, StructureType},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceExclusiveScissorFeaturesNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceExclusiveScissorFeaturesNV {
    #[doc(alias = "exclusiveScissor")]
    pub exclusive_scissor: bool,
}
impl PhysicalDeviceExclusiveScissorFeaturesNV {
    ///Get a reference to the `exclusive_scissor` field.
    pub fn exclusive_scissor(&self) -> &bool {
        &self.exclusive_scissor
    }
    ///Get a mutable reference to the `exclusive_scissor` field.
    pub fn exclusive_scissor_mut(&mut self) -> &mut bool {
        &mut self.exclusive_scissor
    }
    ///Sets the `exclusive_scissor` field.
    pub fn set_exclusive_scissor(&mut self, exclusive_scissor: bool) -> &mut Self {
        self.exclusive_scissor = exclusive_scissor;
        self
    }
    ///Sets the `exclusive_scissor` field in a builder way.
    pub fn with_exclusive_scissor(mut self, exclusive_scissor: bool) -> Self {
        self.exclusive_scissor = exclusive_scissor;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceExclusiveScissorFeaturesNV {
    type LowLevel = crate::native::extensions::nv_scissor_exclusive::PhysicalDeviceExclusiveScissorFeaturesNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_scissor_exclusive::PhysicalDeviceExclusiveScissorFeaturesNV {
            s_type: StructureType::PhysicalDeviceExclusiveScissorFeaturesNv,
            p_next: std::ptr::null_mut(),
            exclusive_scissor: self.exclusive_scissor.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceExclusiveScissorFeaturesNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            exclusive_scissor: crate::conv::FromLowLevel::from_low_level(context, value.exclusive_scissor),
        }
    }
}
#[doc(alias = "VkPipelineViewportExclusiveScissorStateCreateInfoNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineViewportExclusiveScissorStateCreateInfoNV {
    #[doc(alias = "pExclusiveScissors")]
    pub exclusive_scissors: SmallVec<[Rect2D; 8]>,
}
impl PipelineViewportExclusiveScissorStateCreateInfoNV {
    ///Get a reference to the `exclusive_scissors` field.
    pub fn exclusive_scissors(&self) -> &SmallVec<[Rect2D; 8]> {
        &self.exclusive_scissors
    }
    ///Get a mutable reference to the `exclusive_scissors` field.
    pub fn exclusive_scissors_mut(&mut self) -> &mut SmallVec<[Rect2D; 8]> {
        &mut self.exclusive_scissors
    }
    ///Sets the `exclusive_scissors` field.
    pub fn set_exclusive_scissors(&mut self, exclusive_scissors: SmallVec<[Rect2D; 8]>) -> &mut Self {
        self.exclusive_scissors = exclusive_scissors;
        self
    }
    ///Sets the `exclusive_scissors` field in a builder way.
    pub fn with_exclusive_scissors(mut self, exclusive_scissors: SmallVec<[Rect2D; 8]>) -> Self {
        self.exclusive_scissors = exclusive_scissors;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineViewportExclusiveScissorStateCreateInfoNV {
    type LowLevel = crate::native::extensions::nv_scissor_exclusive::PipelineViewportExclusiveScissorStateCreateInfoNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_exclusive_scissors = self.exclusive_scissors.len() as u32;
        let exclusive_scissors = bump
            .alloc_slice_fill_iter(self.exclusive_scissors.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::extensions::nv_scissor_exclusive::PipelineViewportExclusiveScissorStateCreateInfoNV {
            s_type: StructureType::PipelineViewportExclusiveScissorStateCreateInfoNv,
            p_next: std::ptr::null(),
            exclusive_scissor_count: len_exclusive_scissors,
            exclusive_scissors: exclusive_scissors,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineViewportExclusiveScissorStateCreateInfoNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let exclusive_scissors_len = value.exclusive_scissor_count;
        let mut exclusive_scissors = SmallVec::with_capacity(exclusive_scissors_len as usize);
        for i in 0..exclusive_scissors_len {
            exclusive_scissors.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.exclusive_scissors.add(i as usize).read(),
            ));
        }
        Self {
            exclusive_scissors: exclusive_scissors,
        }
    }
}
