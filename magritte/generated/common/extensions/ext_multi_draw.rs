use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VkMultiDrawInfoEXT")]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct MultiDrawInfoEXT {
    #[doc(alias = "firstVertex")]
    pub first_vertex: u32,
    #[doc(alias = "vertexCount")]
    pub vertex_count: u32,
}
#[doc(alias = "VkMultiDrawIndexedInfoEXT")]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct MultiDrawIndexedInfoEXT {
    #[doc(alias = "firstIndex")]
    pub first_index: u32,
    #[doc(alias = "indexCount")]
    pub index_count: u32,
    #[doc(alias = "vertexOffset")]
    pub vertex_offset: i32,
}
#[doc(alias = "VK_EXT_MULTI_DRAW_SPEC_VERSION")]
pub const EXT_MULTI_DRAW_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_MULTI_DRAW_EXTENSION_NAME")]
pub const EXT_MULTI_DRAW_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_multi_draw");
