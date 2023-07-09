pub use crate::common::extensions::valve_mutable_descriptor_type::{
    VALVE_MUTABLE_DESCRIPTOR_TYPE_EXTENSION_NAME, VALVE_MUTABLE_DESCRIPTOR_TYPE_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{DescriptorType, StructureType},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceMutableDescriptorTypeFeaturesVALVE {
    #[doc(alias = "mutableDescriptorType")]
    pub mutable_descriptor_type: bool,
}
impl PhysicalDeviceMutableDescriptorTypeFeaturesVALVE {
    ///Get a reference to the `mutable_descriptor_type` field.
    pub fn mutable_descriptor_type(&self) -> &bool {
        &self.mutable_descriptor_type
    }
    ///Get a mutable reference to the `mutable_descriptor_type` field.
    pub fn mutable_descriptor_type_mut(&mut self) -> &mut bool {
        &mut self.mutable_descriptor_type
    }
    ///Sets the `mutable_descriptor_type` field.
    pub fn set_mutable_descriptor_type(&mut self, mutable_descriptor_type: bool) -> &mut Self {
        self.mutable_descriptor_type = mutable_descriptor_type;
        self
    }
    ///Sets the `mutable_descriptor_type` field in a builder way.
    pub fn with_mutable_descriptor_type(mut self, mutable_descriptor_type: bool) -> Self {
        self.mutable_descriptor_type = mutable_descriptor_type;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceMutableDescriptorTypeFeaturesVALVE {
    type LowLevel =
        crate::native::extensions::valve_mutable_descriptor_type::PhysicalDeviceMutableDescriptorTypeFeaturesVALVE;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::valve_mutable_descriptor_type::PhysicalDeviceMutableDescriptorTypeFeaturesVALVE {
            s_type: StructureType::PhysicalDeviceMutableDescriptorTypeFeaturesValve,
            p_next: std::ptr::null_mut(),
            mutable_descriptor_type: self.mutable_descriptor_type.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceMutableDescriptorTypeFeaturesVALVE {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            mutable_descriptor_type: crate::conv::FromLowLevel::from_low_level(context, value.mutable_descriptor_type),
        }
    }
}
#[doc(alias = "VkMutableDescriptorTypeListVALVE")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MutableDescriptorTypeListVALVE {
    #[doc(alias = "pDescriptorTypes")]
    pub descriptor_types: SmallVec<[DescriptorType; 8]>,
}
impl MutableDescriptorTypeListVALVE {
    ///Get a reference to the `descriptor_types` field.
    pub fn descriptor_types(&self) -> &SmallVec<[DescriptorType; 8]> {
        &self.descriptor_types
    }
    ///Get a mutable reference to the `descriptor_types` field.
    pub fn descriptor_types_mut(&mut self) -> &mut SmallVec<[DescriptorType; 8]> {
        &mut self.descriptor_types
    }
    ///Sets the `descriptor_types` field.
    pub fn set_descriptor_types(&mut self, descriptor_types: SmallVec<[DescriptorType; 8]>) -> &mut Self {
        self.descriptor_types = descriptor_types;
        self
    }
    ///Sets the `descriptor_types` field in a builder way.
    pub fn with_descriptor_types(mut self, descriptor_types: SmallVec<[DescriptorType; 8]>) -> Self {
        self.descriptor_types = descriptor_types;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for MutableDescriptorTypeListVALVE {
    type LowLevel = crate::native::extensions::valve_mutable_descriptor_type::MutableDescriptorTypeListVALVE;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_descriptor_types = self.descriptor_types.len() as u32;
        let descriptor_types = bump
            .alloc_slice_fill_iter(self.descriptor_types.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::extensions::valve_mutable_descriptor_type::MutableDescriptorTypeListVALVE {
            descriptor_type_count: len_descriptor_types,
            descriptor_types: descriptor_types,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for MutableDescriptorTypeListVALVE {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let descriptor_types_len = value.descriptor_type_count;
        let mut descriptor_types = SmallVec::with_capacity(descriptor_types_len as usize);
        for i in 0..descriptor_types_len {
            descriptor_types.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.descriptor_types.add(i as usize).read(),
            ));
        }
        Self {
            descriptor_types: descriptor_types,
        }
    }
}
#[doc(alias = "VkMutableDescriptorTypeCreateInfoVALVE")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MutableDescriptorTypeCreateInfoVALVE {
    #[doc(alias = "pMutableDescriptorTypeLists")]
    pub mutable_descriptor_type_lists: SmallVec<[MutableDescriptorTypeListVALVE; 8]>,
}
impl MutableDescriptorTypeCreateInfoVALVE {
    ///Get a reference to the `mutable_descriptor_type_lists` field.
    pub fn mutable_descriptor_type_lists(&self) -> &SmallVec<[MutableDescriptorTypeListVALVE; 8]> {
        &self.mutable_descriptor_type_lists
    }
    ///Get a mutable reference to the `mutable_descriptor_type_lists` field.
    pub fn mutable_descriptor_type_lists_mut(&mut self) -> &mut SmallVec<[MutableDescriptorTypeListVALVE; 8]> {
        &mut self.mutable_descriptor_type_lists
    }
    ///Sets the `mutable_descriptor_type_lists` field.
    pub fn set_mutable_descriptor_type_lists(
        &mut self,
        mutable_descriptor_type_lists: SmallVec<[MutableDescriptorTypeListVALVE; 8]>,
    ) -> &mut Self {
        self.mutable_descriptor_type_lists = mutable_descriptor_type_lists;
        self
    }
    ///Sets the `mutable_descriptor_type_lists` field in a builder way.
    pub fn with_mutable_descriptor_type_lists(
        mut self,
        mutable_descriptor_type_lists: SmallVec<[MutableDescriptorTypeListVALVE; 8]>,
    ) -> Self {
        self.mutable_descriptor_type_lists = mutable_descriptor_type_lists;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for MutableDescriptorTypeCreateInfoVALVE {
    type LowLevel = crate::native::extensions::valve_mutable_descriptor_type::MutableDescriptorTypeCreateInfoVALVE;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_mutable_descriptor_type_lists = self.mutable_descriptor_type_lists.len() as u32;
        let mutable_descriptor_type_lists = bump
            .alloc_slice_fill_iter(
                self.mutable_descriptor_type_lists
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_ptr()
            .cast();
        crate::native::extensions::valve_mutable_descriptor_type::MutableDescriptorTypeCreateInfoVALVE {
            s_type: StructureType::MutableDescriptorTypeCreateInfoValve,
            p_next: std::ptr::null(),
            mutable_descriptor_type_list_count: len_mutable_descriptor_type_lists,
            mutable_descriptor_type_lists: mutable_descriptor_type_lists,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for MutableDescriptorTypeCreateInfoVALVE {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let mutable_descriptor_type_lists_len = value.mutable_descriptor_type_list_count;
        let mut mutable_descriptor_type_lists = SmallVec::with_capacity(mutable_descriptor_type_lists_len as usize);
        for i in 0..mutable_descriptor_type_lists_len {
            mutable_descriptor_type_lists.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.mutable_descriptor_type_lists.add(i as usize).read(),
            ));
        }
        Self {
            mutable_descriptor_type_lists: mutable_descriptor_type_lists,
        }
    }
}
