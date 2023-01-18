use crate::{
    cstr,
    vulkan1_2::{FNCmdDrawIndexedIndirectCount, FNCmdDrawIndirectCount},
};
use std::ffi::CStr;
#[doc(alias = "VK_AMD_DRAW_INDIRECT_COUNT_SPEC_VERSION")]
pub const AMD_DRAW_INDIRECT_COUNT_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_AMD_DRAW_INDIRECT_COUNT_EXTENSION_NAME")]
pub const AMD_DRAW_INDIRECT_COUNT_EXTENSION_NAME: &'static CStr = cstr!("VK_AMD_draw_indirect_count");
///See [`cmd_draw_indirect_count`]
#[doc(alias = "vkCmdDrawIndirectCountAMD")]
pub type FNCmdDrawIndirectCountAmd = FNCmdDrawIndirectCount;
///See [`cmd_draw_indexed_indirect_count`]
#[doc(alias = "vkCmdDrawIndexedIndirectCountAMD")]
pub type FNCmdDrawIndexedIndirectCountAmd = FNCmdDrawIndexedIndirectCount;
