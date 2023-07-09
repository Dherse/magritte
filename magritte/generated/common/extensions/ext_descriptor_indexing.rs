use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_EXT_DESCRIPTOR_INDEXING_SPEC_VERSION")]
pub const EXT_DESCRIPTOR_INDEXING_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_EXT_DESCRIPTOR_INDEXING_EXTENSION_NAME")]
pub const EXT_DESCRIPTOR_INDEXING_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_descriptor_indexing");
