pub use crate::common::extensions::khr_present_id::{KHR_PRESENT_ID_EXTENSION_NAME, KHR_PRESENT_ID_SPEC_VERSION};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
#[doc(alias = "VkPhysicalDevicePresentIdFeaturesKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDevicePresentIdFeaturesKHR {
    #[doc(alias = "presentId")]
    pub present_id: bool,
}
impl PhysicalDevicePresentIdFeaturesKHR {
    ///Get a reference to the `present_id` field.
    pub fn present_id(&self) -> &bool {
        &self.present_id
    }
    ///Get a mutable reference to the `present_id` field.
    pub fn present_id_mut(&mut self) -> &mut bool {
        &mut self.present_id
    }
    ///Sets the `present_id` field.
    pub fn set_present_id(&mut self, present_id: bool) -> &mut Self {
        self.present_id = present_id;
        self
    }
    ///Sets the `present_id` field in a builder way.
    pub fn with_present_id(mut self, present_id: bool) -> Self {
        self.present_id = present_id;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDevicePresentIdFeaturesKHR {
    type LowLevel = crate::native::extensions::khr_present_id::PhysicalDevicePresentIdFeaturesKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_present_id::PhysicalDevicePresentIdFeaturesKHR {
            s_type: StructureType::PhysicalDevicePresentIdFeaturesKhr,
            p_next: std::ptr::null_mut(),
            present_id: self.present_id.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDevicePresentIdFeaturesKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            present_id: crate::conv::FromLowLevel::from_low_level(context, value.present_id),
        }
    }
}
#[doc(alias = "VkPresentIdKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PresentIdKHR {
    #[doc(alias = "pPresentIds")]
    pub present_ids: SmallVec<[u64; 8]>,
}
impl PresentIdKHR {
    ///Get a reference to the `present_ids` field.
    pub fn present_ids(&self) -> &SmallVec<[u64; 8]> {
        &self.present_ids
    }
    ///Get a mutable reference to the `present_ids` field.
    pub fn present_ids_mut(&mut self) -> &mut SmallVec<[u64; 8]> {
        &mut self.present_ids
    }
    ///Sets the `present_ids` field.
    pub fn set_present_ids(&mut self, present_ids: SmallVec<[u64; 8]>) -> &mut Self {
        self.present_ids = present_ids;
        self
    }
    ///Sets the `present_ids` field in a builder way.
    pub fn with_present_ids(mut self, present_ids: SmallVec<[u64; 8]>) -> Self {
        self.present_ids = present_ids;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PresentIdKHR {
    type LowLevel = crate::native::extensions::khr_present_id::PresentIdKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_present_ids = self.present_ids.len() as u32;
        let present_ids = bump
            .alloc_slice_fill_iter(self.present_ids.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::extensions::khr_present_id::PresentIdKHR {
            s_type: StructureType::PresentIdKhr,
            p_next: std::ptr::null(),
            swapchain_count: len_present_ids,
            present_ids: present_ids,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PresentIdKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let present_ids_len = value.swapchain_count;
        let mut present_ids = SmallVec::with_capacity(present_ids_len as usize);
        for i in 0..present_ids_len {
            present_ids.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.present_ids.add(i as usize).read(),
            ));
        }
        Self {
            present_ids: present_ids,
        }
    }
}
