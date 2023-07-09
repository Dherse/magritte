pub use crate::common::extensions::khr_external_fence_fd::{
    KHR_EXTERNAL_FENCE_FD_EXTENSION_NAME, KHR_EXTERNAL_FENCE_FD_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{Fence, StructureType},
    vulkan1_1::{ExternalFenceHandleTypeFlagBits, FenceImportFlags},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkImportFenceFdInfoKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ImportFenceFdInfoKHR {
    pub fence: Fence,
    pub flags: FenceImportFlags,
    #[doc(alias = "handleType")]
    pub handle_type: ExternalFenceHandleTypeFlagBits,
    pub fd: i32,
}
impl ImportFenceFdInfoKHR {
    ///Get a reference to the `fence` field.
    pub fn fence(&self) -> &Fence {
        &self.fence
    }
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> FenceImportFlags {
        self.flags
    }
    ///Get a reference to the `handle_type` field.
    pub fn handle_type(&self) -> ExternalFenceHandleTypeFlagBits {
        self.handle_type
    }
    ///Get a reference to the `fd` field.
    pub fn fd(&self) -> i32 {
        self.fd
    }
    ///Get a mutable reference to the `fence` field.
    pub fn fence_mut(&mut self) -> &mut Fence {
        &mut self.fence
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut FenceImportFlags {
        &mut self.flags
    }
    ///Get a mutable reference to the `handle_type` field.
    pub fn handle_type_mut(&mut self) -> &mut ExternalFenceHandleTypeFlagBits {
        &mut self.handle_type
    }
    ///Get a mutable reference to the `fd` field.
    pub fn fd_mut(&mut self) -> &mut i32 {
        &mut self.fd
    }
    ///Sets the `fence` field.
    pub fn set_fence(&mut self, fence: Fence) -> &mut Self {
        self.fence = fence;
        self
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: FenceImportFlags) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `handle_type` field.
    pub fn set_handle_type(&mut self, handle_type: ExternalFenceHandleTypeFlagBits) -> &mut Self {
        self.handle_type = handle_type;
        self
    }
    ///Sets the `fd` field.
    pub fn set_fd(&mut self, fd: i32) -> &mut Self {
        self.fd = fd;
        self
    }
    ///Sets the `fence` field in a builder way.
    pub fn with_fence(mut self, fence: Fence) -> Self {
        self.fence = fence;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: FenceImportFlags) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `handle_type` field in a builder way.
    pub fn with_handle_type(mut self, handle_type: ExternalFenceHandleTypeFlagBits) -> Self {
        self.handle_type = handle_type;
        self
    }
    ///Sets the `fd` field in a builder way.
    pub fn with_fd(mut self, fd: i32) -> Self {
        self.fd = fd;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ImportFenceFdInfoKHR {
    type LowLevel = crate::native::extensions::khr_external_fence_fd::ImportFenceFdInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_external_fence_fd::ImportFenceFdInfoKHR {
            s_type: StructureType::ImportFenceFdInfoKhr,
            p_next: std::ptr::null(),
            fence: self.fence.into_low_level(context, bump),
            flags: self.flags.into_low_level(context, bump),
            handle_type: self.handle_type.into_low_level(context, bump),
            fd: self.fd.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ImportFenceFdInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            fence: crate::conv::FromLowLevel::from_low_level(context, value.fence),
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            handle_type: crate::conv::FromLowLevel::from_low_level(context, value.handle_type),
            fd: crate::conv::FromLowLevel::from_low_level(context, value.fd),
        }
    }
}
#[doc(alias = "VkFenceGetFdInfoKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FenceGetFdInfoKHR {
    pub fence: Fence,
    #[doc(alias = "handleType")]
    pub handle_type: ExternalFenceHandleTypeFlagBits,
}
impl FenceGetFdInfoKHR {
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
unsafe impl crate::conv::IntoLowLevel for FenceGetFdInfoKHR {
    type LowLevel = crate::native::extensions::khr_external_fence_fd::FenceGetFdInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_external_fence_fd::FenceGetFdInfoKHR {
            s_type: StructureType::FenceGetFdInfoKhr,
            p_next: std::ptr::null(),
            fence: self.fence.into_low_level(context, bump),
            handle_type: self.handle_type.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for FenceGetFdInfoKHR {
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
