//!# [VK_KHR_draw_indirect_count](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_draw_indirect_count.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_draw_indirect_count/VK_KHR_draw_indirect_count.md")]
use crate::{
    cstr,
    vulkan1_2::{FNCmdDrawIndexedIndirectCount, FNCmdDrawIndirectCount},
};
use std::ffi::CStr;
#[doc(alias = "VK_KHR_DRAW_INDIRECT_COUNT_SPEC_VERSION")]
pub const KHR_DRAW_INDIRECT_COUNT_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_DRAW_INDIRECT_COUNT_EXTENSION_NAME")]
pub const KHR_DRAW_INDIRECT_COUNT_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_draw_indirect_count");
///See [`cmd_draw_indirect_count`]
#[doc(alias = "vkCmdDrawIndirectCountKHR")]
pub type FNCmdDrawIndirectCountKhr = FNCmdDrawIndirectCount;
///See [`cmd_draw_indexed_indirect_count`]
#[doc(alias = "vkCmdDrawIndexedIndirectCountKHR")]
pub type FNCmdDrawIndexedIndirectCountKhr = FNCmdDrawIndexedIndirectCount;