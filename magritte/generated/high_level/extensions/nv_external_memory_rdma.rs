pub use crate::common::extensions::nv_external_memory_rdma::{
    RemoteAddressNV, NV_EXTERNAL_MEMORY_RDMA_EXTENSION_NAME, NV_EXTERNAL_MEMORY_RDMA_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{DeviceMemory, StructureType},
    vulkan1_1::ExternalMemoryHandleTypeFlagBits,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceExternalMemoryRDMAFeaturesNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceExternalMemoryRdmaFeaturesNV {
    #[doc(alias = "externalMemoryRDMA")]
    pub external_memory_rdma: bool,
}
impl PhysicalDeviceExternalMemoryRdmaFeaturesNV {
    ///Get a reference to the `external_memory_rdma` field.
    pub fn external_memory_rdma(&self) -> &bool {
        &self.external_memory_rdma
    }
    ///Get a mutable reference to the `external_memory_rdma` field.
    pub fn external_memory_rdma_mut(&mut self) -> &mut bool {
        &mut self.external_memory_rdma
    }
    ///Sets the `external_memory_rdma` field.
    pub fn set_external_memory_rdma(&mut self, external_memory_rdma: bool) -> &mut Self {
        self.external_memory_rdma = external_memory_rdma;
        self
    }
    ///Sets the `external_memory_rdma` field in a builder way.
    pub fn with_external_memory_rdma(mut self, external_memory_rdma: bool) -> Self {
        self.external_memory_rdma = external_memory_rdma;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceExternalMemoryRdmaFeaturesNV {
    type LowLevel = crate::native::extensions::nv_external_memory_rdma::PhysicalDeviceExternalMemoryRdmaFeaturesNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_external_memory_rdma::PhysicalDeviceExternalMemoryRdmaFeaturesNV {
            s_type: StructureType::PhysicalDeviceExternalMemoryRdmaFeaturesNv,
            p_next: std::ptr::null_mut(),
            external_memory_rdma: self.external_memory_rdma.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceExternalMemoryRdmaFeaturesNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            external_memory_rdma: crate::conv::FromLowLevel::from_low_level(context, value.external_memory_rdma),
        }
    }
}
#[doc(alias = "VkMemoryGetRemoteAddressInfoNV")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MemoryGetRemoteAddressInfoNV {
    pub memory: DeviceMemory,
    #[doc(alias = "handleType")]
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
}
impl MemoryGetRemoteAddressInfoNV {
    ///Get a reference to the `memory` field.
    pub fn memory(&self) -> &DeviceMemory {
        &self.memory
    }
    ///Get a reference to the `handle_type` field.
    pub fn handle_type(&self) -> ExternalMemoryHandleTypeFlagBits {
        self.handle_type
    }
    ///Get a mutable reference to the `memory` field.
    pub fn memory_mut(&mut self) -> &mut DeviceMemory {
        &mut self.memory
    }
    ///Get a mutable reference to the `handle_type` field.
    pub fn handle_type_mut(&mut self) -> &mut ExternalMemoryHandleTypeFlagBits {
        &mut self.handle_type
    }
    ///Sets the `memory` field.
    pub fn set_memory(&mut self, memory: DeviceMemory) -> &mut Self {
        self.memory = memory;
        self
    }
    ///Sets the `handle_type` field.
    pub fn set_handle_type(&mut self, handle_type: ExternalMemoryHandleTypeFlagBits) -> &mut Self {
        self.handle_type = handle_type;
        self
    }
    ///Sets the `memory` field in a builder way.
    pub fn with_memory(mut self, memory: DeviceMemory) -> Self {
        self.memory = memory;
        self
    }
    ///Sets the `handle_type` field in a builder way.
    pub fn with_handle_type(mut self, handle_type: ExternalMemoryHandleTypeFlagBits) -> Self {
        self.handle_type = handle_type;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for MemoryGetRemoteAddressInfoNV {
    type LowLevel = crate::native::extensions::nv_external_memory_rdma::MemoryGetRemoteAddressInfoNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_external_memory_rdma::MemoryGetRemoteAddressInfoNV {
            s_type: StructureType::MemoryGetRemoteAddressInfoNv,
            p_next: std::ptr::null(),
            memory: self.memory.into_low_level(context, bump),
            handle_type: self.handle_type.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for MemoryGetRemoteAddressInfoNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            memory: crate::conv::FromLowLevel::from_low_level(context, value.memory),
            handle_type: crate::conv::FromLowLevel::from_low_level(context, value.handle_type),
        }
    }
}
