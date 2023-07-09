pub use crate::common::extensions::ext_multi_draw::{
    MultiDrawIndexedInfoEXT, MultiDrawInfoEXT, EXT_MULTI_DRAW_EXTENSION_NAME, EXT_MULTI_DRAW_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
impl MultiDrawInfoEXT {
    ///Get a reference to the `first_vertex` field.
    pub fn first_vertex(&self) -> u32 {
        self.first_vertex
    }
    ///Get a reference to the `vertex_count` field.
    pub fn vertex_count(&self) -> u32 {
        self.vertex_count
    }
    ///Get a mutable reference to the `first_vertex` field.
    pub fn first_vertex_mut(&mut self) -> &mut u32 {
        &mut self.first_vertex
    }
    ///Get a mutable reference to the `vertex_count` field.
    pub fn vertex_count_mut(&mut self) -> &mut u32 {
        &mut self.vertex_count
    }
    ///Sets the `first_vertex` field.
    pub fn set_first_vertex(&mut self, first_vertex: u32) -> &mut Self {
        self.first_vertex = first_vertex;
        self
    }
    ///Sets the `vertex_count` field.
    pub fn set_vertex_count(&mut self, vertex_count: u32) -> &mut Self {
        self.vertex_count = vertex_count;
        self
    }
    ///Sets the `first_vertex` field in a builder way.
    pub fn with_first_vertex(mut self, first_vertex: u32) -> Self {
        self.first_vertex = first_vertex;
        self
    }
    ///Sets the `vertex_count` field in a builder way.
    pub fn with_vertex_count(mut self, vertex_count: u32) -> Self {
        self.vertex_count = vertex_count;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for MultiDrawInfoEXT {
    type LowLevel = crate::native::extensions::ext_multi_draw::MultiDrawInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_multi_draw::MultiDrawInfoEXT {
            first_vertex: self.first_vertex.into_low_level(context, bump),
            vertex_count: self.vertex_count.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for MultiDrawInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            first_vertex: crate::conv::FromLowLevel::from_low_level(context, value.first_vertex),
            vertex_count: crate::conv::FromLowLevel::from_low_level(context, value.vertex_count),
        }
    }
}
impl MultiDrawIndexedInfoEXT {
    ///Get a reference to the `first_index` field.
    pub fn first_index(&self) -> u32 {
        self.first_index
    }
    ///Get a reference to the `index_count` field.
    pub fn index_count(&self) -> u32 {
        self.index_count
    }
    ///Get a reference to the `vertex_offset` field.
    pub fn vertex_offset(&self) -> i32 {
        self.vertex_offset
    }
    ///Get a mutable reference to the `first_index` field.
    pub fn first_index_mut(&mut self) -> &mut u32 {
        &mut self.first_index
    }
    ///Get a mutable reference to the `index_count` field.
    pub fn index_count_mut(&mut self) -> &mut u32 {
        &mut self.index_count
    }
    ///Get a mutable reference to the `vertex_offset` field.
    pub fn vertex_offset_mut(&mut self) -> &mut i32 {
        &mut self.vertex_offset
    }
    ///Sets the `first_index` field.
    pub fn set_first_index(&mut self, first_index: u32) -> &mut Self {
        self.first_index = first_index;
        self
    }
    ///Sets the `index_count` field.
    pub fn set_index_count(&mut self, index_count: u32) -> &mut Self {
        self.index_count = index_count;
        self
    }
    ///Sets the `vertex_offset` field.
    pub fn set_vertex_offset(&mut self, vertex_offset: i32) -> &mut Self {
        self.vertex_offset = vertex_offset;
        self
    }
    ///Sets the `first_index` field in a builder way.
    pub fn with_first_index(mut self, first_index: u32) -> Self {
        self.first_index = first_index;
        self
    }
    ///Sets the `index_count` field in a builder way.
    pub fn with_index_count(mut self, index_count: u32) -> Self {
        self.index_count = index_count;
        self
    }
    ///Sets the `vertex_offset` field in a builder way.
    pub fn with_vertex_offset(mut self, vertex_offset: i32) -> Self {
        self.vertex_offset = vertex_offset;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for MultiDrawIndexedInfoEXT {
    type LowLevel = crate::native::extensions::ext_multi_draw::MultiDrawIndexedInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_multi_draw::MultiDrawIndexedInfoEXT {
            first_index: self.first_index.into_low_level(context, bump),
            index_count: self.index_count.into_low_level(context, bump),
            vertex_offset: self.vertex_offset.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for MultiDrawIndexedInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            first_index: crate::conv::FromLowLevel::from_low_level(context, value.first_index),
            index_count: crate::conv::FromLowLevel::from_low_level(context, value.index_count),
            vertex_offset: crate::conv::FromLowLevel::from_low_level(context, value.vertex_offset),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceMultiDrawPropertiesEXT")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceMultiDrawPropertiesEXT {
    #[doc(alias = "maxMultiDrawCount")]
    pub max_multi_draw_count: u32,
}
impl PhysicalDeviceMultiDrawPropertiesEXT {
    ///Get a reference to the `max_multi_draw_count` field.
    pub fn max_multi_draw_count(&self) -> u32 {
        self.max_multi_draw_count
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceMultiDrawPropertiesEXT {
    type LowLevel = crate::native::extensions::ext_multi_draw::PhysicalDeviceMultiDrawPropertiesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_multi_draw::PhysicalDeviceMultiDrawPropertiesEXT {
            s_type: StructureType::PhysicalDeviceMultiDrawPropertiesExt,
            p_next: std::ptr::null_mut(),
            max_multi_draw_count: self.max_multi_draw_count.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceMultiDrawPropertiesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            max_multi_draw_count: crate::conv::FromLowLevel::from_low_level(context, value.max_multi_draw_count),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceMultiDrawFeaturesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceMultiDrawFeaturesEXT {
    #[doc(alias = "multiDraw")]
    pub multi_draw: bool,
}
impl PhysicalDeviceMultiDrawFeaturesEXT {
    ///Get a reference to the `multi_draw` field.
    pub fn multi_draw(&self) -> &bool {
        &self.multi_draw
    }
    ///Get a mutable reference to the `multi_draw` field.
    pub fn multi_draw_mut(&mut self) -> &mut bool {
        &mut self.multi_draw
    }
    ///Sets the `multi_draw` field.
    pub fn set_multi_draw(&mut self, multi_draw: bool) -> &mut Self {
        self.multi_draw = multi_draw;
        self
    }
    ///Sets the `multi_draw` field in a builder way.
    pub fn with_multi_draw(mut self, multi_draw: bool) -> Self {
        self.multi_draw = multi_draw;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceMultiDrawFeaturesEXT {
    type LowLevel = crate::native::extensions::ext_multi_draw::PhysicalDeviceMultiDrawFeaturesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_multi_draw::PhysicalDeviceMultiDrawFeaturesEXT {
            s_type: StructureType::PhysicalDeviceMultiDrawFeaturesExt,
            p_next: std::ptr::null_mut(),
            multi_draw: self.multi_draw.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceMultiDrawFeaturesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            multi_draw: crate::conv::FromLowLevel::from_low_level(context, value.multi_draw),
        }
    }
}
