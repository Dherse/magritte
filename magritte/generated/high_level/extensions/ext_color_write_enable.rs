pub use crate::common::extensions::ext_color_write_enable::{
    EXT_COLOR_WRITE_ENABLE_EXTENSION_NAME, EXT_COLOR_WRITE_ENABLE_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceColorWriteEnableFeaturesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceColorWriteEnableFeaturesEXT {
    #[doc(alias = "colorWriteEnable")]
    pub color_write_enable: bool,
}
impl PhysicalDeviceColorWriteEnableFeaturesEXT {
    ///Get a reference to the `color_write_enable` field.
    pub fn color_write_enable(&self) -> &bool {
        &self.color_write_enable
    }
    ///Get a mutable reference to the `color_write_enable` field.
    pub fn color_write_enable_mut(&mut self) -> &mut bool {
        &mut self.color_write_enable
    }
    ///Sets the `color_write_enable` field.
    pub fn set_color_write_enable(&mut self, color_write_enable: bool) -> &mut Self {
        self.color_write_enable = color_write_enable;
        self
    }
    ///Sets the `color_write_enable` field in a builder way.
    pub fn with_color_write_enable(mut self, color_write_enable: bool) -> Self {
        self.color_write_enable = color_write_enable;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceColorWriteEnableFeaturesEXT {
    type LowLevel = crate::native::extensions::ext_color_write_enable::PhysicalDeviceColorWriteEnableFeaturesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_color_write_enable::PhysicalDeviceColorWriteEnableFeaturesEXT {
            s_type: StructureType::PhysicalDeviceColorWriteEnableFeaturesExt,
            p_next: std::ptr::null_mut(),
            color_write_enable: self.color_write_enable.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceColorWriteEnableFeaturesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            color_write_enable: crate::conv::FromLowLevel::from_low_level(context, value.color_write_enable),
        }
    }
}
#[doc(alias = "VkPipelineColorWriteCreateInfoEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineColorWriteCreateInfoEXT {
    #[doc(alias = "pColorWriteEnables")]
    pub color_write_enables: SmallVec<[bool; 8]>,
}
impl PipelineColorWriteCreateInfoEXT {
    ///Get a reference to the `color_write_enables` field.
    pub fn color_write_enables(&self) -> &SmallVec<[bool; 8]> {
        &self.color_write_enables
    }
    ///Get a mutable reference to the `color_write_enables` field.
    pub fn color_write_enables_mut(&mut self) -> &mut SmallVec<[bool; 8]> {
        &mut self.color_write_enables
    }
    ///Sets the `color_write_enables` field.
    pub fn set_color_write_enables(&mut self, color_write_enables: SmallVec<[bool; 8]>) -> &mut Self {
        self.color_write_enables = color_write_enables;
        self
    }
    ///Sets the `color_write_enables` field in a builder way.
    pub fn with_color_write_enables(mut self, color_write_enables: SmallVec<[bool; 8]>) -> Self {
        self.color_write_enables = color_write_enables;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineColorWriteCreateInfoEXT {
    type LowLevel = crate::native::extensions::ext_color_write_enable::PipelineColorWriteCreateInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_color_write_enables = self.color_write_enables.len() as u32;
        let color_write_enables = bump
            .alloc_slice_fill_iter(self.color_write_enables.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::extensions::ext_color_write_enable::PipelineColorWriteCreateInfoEXT {
            s_type: StructureType::PipelineColorWriteCreateInfoExt,
            p_next: std::ptr::null(),
            attachment_count: len_color_write_enables,
            color_write_enables: color_write_enables,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineColorWriteCreateInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let color_write_enables_len = value.attachment_count;
        let mut color_write_enables = SmallVec::with_capacity(color_write_enables_len as usize);
        for i in 0..color_write_enables_len {
            color_write_enables.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.color_write_enables.add(i as usize).read(),
            ));
        }
        Self {
            color_write_enables: color_write_enables,
        }
    }
}
