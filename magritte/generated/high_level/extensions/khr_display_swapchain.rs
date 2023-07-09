pub use crate::common::extensions::khr_display_swapchain::{
    KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME, KHR_DISPLAY_SWAPCHAIN_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{Rect2D, StructureType},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkDisplayPresentInfoKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DisplayPresentInfoKHR {
    #[doc(alias = "srcRect")]
    pub src_rect: Rect2D,
    #[doc(alias = "dstRect")]
    pub dst_rect: Rect2D,
    pub persistent: bool,
}
impl DisplayPresentInfoKHR {
    ///Get a reference to the `src_rect` field.
    pub fn src_rect(&self) -> Rect2D {
        self.src_rect
    }
    ///Get a reference to the `dst_rect` field.
    pub fn dst_rect(&self) -> Rect2D {
        self.dst_rect
    }
    ///Get a reference to the `persistent` field.
    pub fn persistent(&self) -> &bool {
        &self.persistent
    }
    ///Get a mutable reference to the `src_rect` field.
    pub fn src_rect_mut(&mut self) -> &mut Rect2D {
        &mut self.src_rect
    }
    ///Get a mutable reference to the `dst_rect` field.
    pub fn dst_rect_mut(&mut self) -> &mut Rect2D {
        &mut self.dst_rect
    }
    ///Get a mutable reference to the `persistent` field.
    pub fn persistent_mut(&mut self) -> &mut bool {
        &mut self.persistent
    }
    ///Sets the `src_rect` field.
    pub fn set_src_rect(&mut self, src_rect: Rect2D) -> &mut Self {
        self.src_rect = src_rect;
        self
    }
    ///Sets the `dst_rect` field.
    pub fn set_dst_rect(&mut self, dst_rect: Rect2D) -> &mut Self {
        self.dst_rect = dst_rect;
        self
    }
    ///Sets the `persistent` field.
    pub fn set_persistent(&mut self, persistent: bool) -> &mut Self {
        self.persistent = persistent;
        self
    }
    ///Sets the `src_rect` field in a builder way.
    pub fn with_src_rect(mut self, src_rect: Rect2D) -> Self {
        self.src_rect = src_rect;
        self
    }
    ///Sets the `dst_rect` field in a builder way.
    pub fn with_dst_rect(mut self, dst_rect: Rect2D) -> Self {
        self.dst_rect = dst_rect;
        self
    }
    ///Sets the `persistent` field in a builder way.
    pub fn with_persistent(mut self, persistent: bool) -> Self {
        self.persistent = persistent;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DisplayPresentInfoKHR {
    type LowLevel = crate::native::extensions::khr_display_swapchain::DisplayPresentInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_display_swapchain::DisplayPresentInfoKHR {
            s_type: StructureType::DisplayPresentInfoKhr,
            p_next: std::ptr::null(),
            src_rect: self.src_rect.into_low_level(context, bump),
            dst_rect: self.dst_rect.into_low_level(context, bump),
            persistent: self.persistent.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DisplayPresentInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            src_rect: crate::conv::FromLowLevel::from_low_level(context, value.src_rect),
            dst_rect: crate::conv::FromLowLevel::from_low_level(context, value.dst_rect),
            persistent: crate::conv::FromLowLevel::from_low_level(context, value.persistent),
        }
    }
}
