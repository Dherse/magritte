use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_EXT_EXTERNAL_MEMORY_DMA_BUF_SPEC_VERSION")]
pub const EXT_EXTERNAL_MEMORY_DMA_BUF_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_EXTERNAL_MEMORY_DMA_BUF_EXTENSION_NAME")]
pub const EXT_EXTERNAL_MEMORY_DMA_BUF_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_external_memory_dma_buf");
