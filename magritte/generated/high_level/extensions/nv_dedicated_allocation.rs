pub use crate::common::extensions::nv_dedicated_allocation::{
    NV_DEDICATED_ALLOCATION_EXTENSION_NAME, NV_DEDICATED_ALLOCATION_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{Buffer, Image, StructureType},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkDedicatedAllocationImageCreateInfoNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DedicatedAllocationImageCreateInfoNV {
    #[doc(alias = "dedicatedAllocation")]
    pub dedicated_allocation: bool,
}
impl DedicatedAllocationImageCreateInfoNV {
    ///Get a reference to the `dedicated_allocation` field.
    pub fn dedicated_allocation(&self) -> &bool {
        &self.dedicated_allocation
    }
    ///Get a mutable reference to the `dedicated_allocation` field.
    pub fn dedicated_allocation_mut(&mut self) -> &mut bool {
        &mut self.dedicated_allocation
    }
    ///Sets the `dedicated_allocation` field.
    pub fn set_dedicated_allocation(&mut self, dedicated_allocation: bool) -> &mut Self {
        self.dedicated_allocation = dedicated_allocation;
        self
    }
    ///Sets the `dedicated_allocation` field in a builder way.
    pub fn with_dedicated_allocation(mut self, dedicated_allocation: bool) -> Self {
        self.dedicated_allocation = dedicated_allocation;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DedicatedAllocationImageCreateInfoNV {
    type LowLevel = crate::native::extensions::nv_dedicated_allocation::DedicatedAllocationImageCreateInfoNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_dedicated_allocation::DedicatedAllocationImageCreateInfoNV {
            s_type: StructureType::DedicatedAllocationImageCreateInfoNv,
            p_next: std::ptr::null(),
            dedicated_allocation: self.dedicated_allocation.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DedicatedAllocationImageCreateInfoNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            dedicated_allocation: crate::conv::FromLowLevel::from_low_level(context, value.dedicated_allocation),
        }
    }
}
#[doc(alias = "VkDedicatedAllocationBufferCreateInfoNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DedicatedAllocationBufferCreateInfoNV {
    #[doc(alias = "dedicatedAllocation")]
    pub dedicated_allocation: bool,
}
impl DedicatedAllocationBufferCreateInfoNV {
    ///Get a reference to the `dedicated_allocation` field.
    pub fn dedicated_allocation(&self) -> &bool {
        &self.dedicated_allocation
    }
    ///Get a mutable reference to the `dedicated_allocation` field.
    pub fn dedicated_allocation_mut(&mut self) -> &mut bool {
        &mut self.dedicated_allocation
    }
    ///Sets the `dedicated_allocation` field.
    pub fn set_dedicated_allocation(&mut self, dedicated_allocation: bool) -> &mut Self {
        self.dedicated_allocation = dedicated_allocation;
        self
    }
    ///Sets the `dedicated_allocation` field in a builder way.
    pub fn with_dedicated_allocation(mut self, dedicated_allocation: bool) -> Self {
        self.dedicated_allocation = dedicated_allocation;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DedicatedAllocationBufferCreateInfoNV {
    type LowLevel = crate::native::extensions::nv_dedicated_allocation::DedicatedAllocationBufferCreateInfoNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_dedicated_allocation::DedicatedAllocationBufferCreateInfoNV {
            s_type: StructureType::DedicatedAllocationBufferCreateInfoNv,
            p_next: std::ptr::null(),
            dedicated_allocation: self.dedicated_allocation.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DedicatedAllocationBufferCreateInfoNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            dedicated_allocation: crate::conv::FromLowLevel::from_low_level(context, value.dedicated_allocation),
        }
    }
}
#[doc(alias = "VkDedicatedAllocationMemoryAllocateInfoNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DedicatedAllocationMemoryAllocateInfoNV {
    pub image: Option<Image>,
    pub buffer: Option<Buffer>,
}
impl DedicatedAllocationMemoryAllocateInfoNV {
    ///Get a reference to the `image` field.
    pub fn image(&self) -> &Option<Image> {
        &self.image
    }
    ///Get a reference to the `buffer` field.
    pub fn buffer(&self) -> &Option<Buffer> {
        &self.buffer
    }
    ///Get a mutable reference to the `image` field.
    pub fn image_mut(&mut self) -> &mut Option<Image> {
        &mut self.image
    }
    ///Get a mutable reference to the `buffer` field.
    pub fn buffer_mut(&mut self) -> &mut Option<Buffer> {
        &mut self.buffer
    }
    ///Sets the `image` field.
    pub fn set_image(&mut self, image: Option<Image>) -> &mut Self {
        self.image = image;
        self
    }
    ///Sets the `buffer` field.
    pub fn set_buffer(&mut self, buffer: Option<Buffer>) -> &mut Self {
        self.buffer = buffer;
        self
    }
    ///Sets the `image` field in a builder way.
    pub fn with_image(mut self, image: Option<Image>) -> Self {
        self.image = image;
        self
    }
    ///Sets the `buffer` field in a builder way.
    pub fn with_buffer(mut self, buffer: Option<Buffer>) -> Self {
        self.buffer = buffer;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DedicatedAllocationMemoryAllocateInfoNV {
    type LowLevel = crate::native::extensions::nv_dedicated_allocation::DedicatedAllocationMemoryAllocateInfoNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_dedicated_allocation::DedicatedAllocationMemoryAllocateInfoNV {
            s_type: StructureType::DedicatedAllocationMemoryAllocateInfoNv,
            p_next: std::ptr::null(),
            image: self
                .image
                .as_ref()
                .map(|v| v.into_low_level(context, bump))
                .unwrap_or_else(Default::default),
            buffer: self
                .buffer
                .as_ref()
                .map(|v| v.into_low_level(context, bump))
                .unwrap_or_else(Default::default),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DedicatedAllocationMemoryAllocateInfoNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            image: if value.image == crate::native::vulkan1_0::Image::null() {
                None
            } else {
                Some(crate::conv::FromLowLevel::from_low_level(context, value.image))
            },
            buffer: if value.buffer == crate::native::vulkan1_0::Buffer::null() {
                None
            } else {
                Some(crate::conv::FromLowLevel::from_low_level(context, value.buffer))
            },
        }
    }
}
