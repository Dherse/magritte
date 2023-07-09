pub use crate::common::extensions::khr_external_fence_win32::{
    KHR_EXTERNAL_FENCE_WIN32_EXTENSION_NAME, KHR_EXTERNAL_FENCE_WIN32_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{Fence, StructureType},
    vulkan1_1::ExternalFenceHandleTypeFlagBits,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkFenceGetWin32HandleInfoKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FenceGetWin32HandleInfoKHR {
    pub fence: Fence,
    #[doc(alias = "handleType")]
    pub handle_type: ExternalFenceHandleTypeFlagBits,
}
impl FenceGetWin32HandleInfoKHR {
    ///Get a reference to the `fence` field.
    pub fn fence(&self) -> &Fence {
        &self.fence
    }
    ///Get a reference to the `handle_type` field.
    pub fn handle_type(&self) -> ExternalFenceHandleTypeFlagBits {
        self.handle_type
    }
    ///Get a mutable reference to the `fence` field.
    pub fn fence_mut(&mut self) -> &mut Fence {
        &mut self.fence
    }
    ///Get a mutable reference to the `handle_type` field.
    pub fn handle_type_mut(&mut self) -> &mut ExternalFenceHandleTypeFlagBits {
        &mut self.handle_type
    }
    ///Sets the `fence` field.
    pub fn set_fence(&mut self, fence: Fence) -> &mut Self {
        self.fence = fence;
        self
    }
    ///Sets the `handle_type` field.
    pub fn set_handle_type(&mut self, handle_type: ExternalFenceHandleTypeFlagBits) -> &mut Self {
        self.handle_type = handle_type;
        self
    }
    ///Sets the `fence` field in a builder way.
    pub fn with_fence(mut self, fence: Fence) -> Self {
        self.fence = fence;
        self
    }
    ///Sets the `handle_type` field in a builder way.
    pub fn with_handle_type(mut self, handle_type: ExternalFenceHandleTypeFlagBits) -> Self {
        self.handle_type = handle_type;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for FenceGetWin32HandleInfoKHR {
    type LowLevel = crate::native::extensions::khr_external_fence_win32::FenceGetWin32HandleInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_external_fence_win32::FenceGetWin32HandleInfoKHR {
            s_type: StructureType::FenceGetWin32HandleInfoKhr,
            p_next: std::ptr::null(),
            fence: self.fence.into_low_level(context, bump),
            handle_type: self.handle_type.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for FenceGetWin32HandleInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            fence: crate::conv::FromLowLevel::from_low_level(context, value.fence),
            handle_type: crate::conv::FromLowLevel::from_low_level(context, value.handle_type),
        }
    }
}
