pub use crate::common::extensions::ext_debug_marker::{
    DebugReportObjectTypeEXT, EXT_DEBUG_MARKER_EXTENSION_NAME, EXT_DEBUG_MARKER_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkDebugMarkerObjectNameInfoEXT")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DebugMarkerObjectNameInfoEXT {
    #[doc(alias = "objectType")]
    pub object_type: DebugReportObjectTypeEXT,
    pub object: u64,
    #[doc(alias = "pObjectName")]
    pub object_name: String,
}
impl DebugMarkerObjectNameInfoEXT {
    ///Get a reference to the `object_type` field.
    pub fn object_type(&self) -> DebugReportObjectTypeEXT {
        self.object_type
    }
    ///Get a reference to the `object` field.
    pub fn object(&self) -> u64 {
        self.object
    }
    ///Get a reference to the `object_name` field.
    pub fn object_name(&self) -> &String {
        &self.object_name
    }
    ///Get a mutable reference to the `object_type` field.
    pub fn object_type_mut(&mut self) -> &mut DebugReportObjectTypeEXT {
        &mut self.object_type
    }
    ///Get a mutable reference to the `object` field.
    pub fn object_mut(&mut self) -> &mut u64 {
        &mut self.object
    }
    ///Get a mutable reference to the `object_name` field.
    pub fn object_name_mut(&mut self) -> &mut String {
        &mut self.object_name
    }
    ///Sets the `object_type` field.
    pub fn set_object_type(&mut self, object_type: DebugReportObjectTypeEXT) -> &mut Self {
        self.object_type = object_type;
        self
    }
    ///Sets the `object` field.
    pub fn set_object(&mut self, object: u64) -> &mut Self {
        self.object = object;
        self
    }
    ///Sets the `object_name` field.
    pub fn set_object_name(&mut self, object_name: String) -> &mut Self {
        self.object_name = object_name;
        self
    }
    ///Sets the `object_type` field in a builder way.
    pub fn with_object_type(mut self, object_type: DebugReportObjectTypeEXT) -> Self {
        self.object_type = object_type;
        self
    }
    ///Sets the `object` field in a builder way.
    pub fn with_object(mut self, object: u64) -> Self {
        self.object = object;
        self
    }
    ///Sets the `object_name` field in a builder way.
    pub fn with_object_name(mut self, object_name: String) -> Self {
        self.object_name = object_name;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DebugMarkerObjectNameInfoEXT {
    type LowLevel = crate::native::extensions::ext_debug_marker::DebugMarkerObjectNameInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_debug_marker::DebugMarkerObjectNameInfoEXT {
            s_type: StructureType::DebugMarkerObjectNameInfoExt,
            p_next: std::ptr::null(),
            object_type: self.object_type.into_low_level(context, bump),
            object: self.object.into_low_level(context, bump),
            object_name: self.object_name.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DebugMarkerObjectNameInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            object_type: crate::conv::FromLowLevel::from_low_level(context, value.object_type),
            object: crate::conv::FromLowLevel::from_low_level(context, value.object),
            object_name: crate::conv::FromLowLevel::from_low_level(context, value.object_name),
        }
    }
}
#[doc(alias = "VkDebugMarkerObjectTagInfoEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DebugMarkerObjectTagInfoEXT {
    #[doc(alias = "objectType")]
    pub object_type: DebugReportObjectTypeEXT,
    pub object: u64,
    #[doc(alias = "tagName")]
    pub tag_name: u64,
    #[doc(alias = "pTag")]
    pub tag: Vec<u8>,
}
impl DebugMarkerObjectTagInfoEXT {
    ///Get a reference to the `object_type` field.
    pub fn object_type(&self) -> DebugReportObjectTypeEXT {
        self.object_type
    }
    ///Get a reference to the `object` field.
    pub fn object(&self) -> u64 {
        self.object
    }
    ///Get a reference to the `tag_name` field.
    pub fn tag_name(&self) -> u64 {
        self.tag_name
    }
    ///Get a reference to the `tag` field.
    pub fn tag(&self) -> &Vec<u8> {
        &self.tag
    }
    ///Get a mutable reference to the `object_type` field.
    pub fn object_type_mut(&mut self) -> &mut DebugReportObjectTypeEXT {
        &mut self.object_type
    }
    ///Get a mutable reference to the `object` field.
    pub fn object_mut(&mut self) -> &mut u64 {
        &mut self.object
    }
    ///Get a mutable reference to the `tag_name` field.
    pub fn tag_name_mut(&mut self) -> &mut u64 {
        &mut self.tag_name
    }
    ///Get a mutable reference to the `tag` field.
    pub fn tag_mut(&mut self) -> &mut Vec<u8> {
        &mut self.tag
    }
    ///Sets the `object_type` field.
    pub fn set_object_type(&mut self, object_type: DebugReportObjectTypeEXT) -> &mut Self {
        self.object_type = object_type;
        self
    }
    ///Sets the `object` field.
    pub fn set_object(&mut self, object: u64) -> &mut Self {
        self.object = object;
        self
    }
    ///Sets the `tag_name` field.
    pub fn set_tag_name(&mut self, tag_name: u64) -> &mut Self {
        self.tag_name = tag_name;
        self
    }
    ///Sets the `tag` field.
    pub fn set_tag(&mut self, tag: Vec<u8>) -> &mut Self {
        self.tag = tag;
        self
    }
    ///Sets the `object_type` field in a builder way.
    pub fn with_object_type(mut self, object_type: DebugReportObjectTypeEXT) -> Self {
        self.object_type = object_type;
        self
    }
    ///Sets the `object` field in a builder way.
    pub fn with_object(mut self, object: u64) -> Self {
        self.object = object;
        self
    }
    ///Sets the `tag_name` field in a builder way.
    pub fn with_tag_name(mut self, tag_name: u64) -> Self {
        self.tag_name = tag_name;
        self
    }
    ///Sets the `tag` field in a builder way.
    pub fn with_tag(mut self, tag: Vec<u8>) -> Self {
        self.tag = tag;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DebugMarkerObjectTagInfoEXT {
    type LowLevel = crate::native::extensions::ext_debug_marker::DebugMarkerObjectTagInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_tag = self.tag.len() as usize;
        crate::native::extensions::ext_debug_marker::DebugMarkerObjectTagInfoEXT {
            s_type: StructureType::DebugMarkerObjectTagInfoExt,
            p_next: std::ptr::null(),
            object_type: self.object_type.into_low_level(context, bump),
            object: self.object.into_low_level(context, bump),
            tag_name: self.tag_name.into_low_level(context, bump),
            tag_size: len_tag,
            tag: self.tag.as_ptr().cast(),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DebugMarkerObjectTagInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let tag_len = value.tag_size;
        let mut tag = Vec::with_capacity(tag_len as usize);
        let tag_ptr = value.tag as *const u8;
        for i in 0..tag_len {
            tag.push(crate::conv::FromLowLevel::from_low_level(
                context,
                tag_ptr.add(i as usize).read(),
            ));
        }
        Self {
            object_type: crate::conv::FromLowLevel::from_low_level(context, value.object_type),
            object: crate::conv::FromLowLevel::from_low_level(context, value.object),
            tag_name: crate::conv::FromLowLevel::from_low_level(context, value.tag_name),
            tag: tag,
        }
    }
}
#[doc(alias = "VkDebugMarkerMarkerInfoEXT")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DebugMarkerMarkerInfoEXT {
    #[doc(alias = "pMarkerName")]
    pub marker_name: String,
    pub color: [f32; 4 as usize],
}
impl DebugMarkerMarkerInfoEXT {
    ///Get a reference to the `marker_name` field.
    pub fn marker_name(&self) -> &String {
        &self.marker_name
    }
    ///Get a reference to the `color` field.
    pub fn color(&self) -> [f32; 4 as usize] {
        self.color
    }
    ///Get a mutable reference to the `marker_name` field.
    pub fn marker_name_mut(&mut self) -> &mut String {
        &mut self.marker_name
    }
    ///Get a mutable reference to the `color` field.
    pub fn color_mut(&mut self) -> &mut [f32; 4 as usize] {
        &mut self.color
    }
    ///Sets the `marker_name` field.
    pub fn set_marker_name(&mut self, marker_name: String) -> &mut Self {
        self.marker_name = marker_name;
        self
    }
    ///Sets the `color` field.
    pub fn set_color(&mut self, color: [f32; 4 as usize]) -> &mut Self {
        self.color = color;
        self
    }
    ///Sets the `marker_name` field in a builder way.
    pub fn with_marker_name(mut self, marker_name: String) -> Self {
        self.marker_name = marker_name;
        self
    }
    ///Sets the `color` field in a builder way.
    pub fn with_color(mut self, color: [f32; 4 as usize]) -> Self {
        self.color = color;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DebugMarkerMarkerInfoEXT {
    type LowLevel = crate::native::extensions::ext_debug_marker::DebugMarkerMarkerInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_debug_marker::DebugMarkerMarkerInfoEXT {
            s_type: StructureType::DebugMarkerMarkerInfoExt,
            p_next: std::ptr::null(),
            marker_name: self.marker_name.into_low_level(context, bump),
            color: self.color.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DebugMarkerMarkerInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            marker_name: crate::conv::FromLowLevel::from_low_level(context, value.marker_name),
            color: crate::conv::FromLowLevel::from_low_level(context, value.color),
        }
    }
}
