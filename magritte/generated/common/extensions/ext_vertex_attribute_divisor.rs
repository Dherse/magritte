use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VkVertexInputBindingDivisorDescriptionEXT")]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VertexInputBindingDivisorDescriptionEXT {
    pub binding: u32,
    pub divisor: u32,
}
#[doc(alias = "VK_EXT_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION")]
pub const EXT_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION: u32 = 3;
#[doc(alias = "VK_EXT_VERTEX_ATTRIBUTE_DIVISOR_EXTENSION_NAME")]
pub const EXT_VERTEX_ATTRIBUTE_DIVISOR_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_vertex_attribute_divisor");
