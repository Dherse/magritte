pub use crate::common::extensions::khr_draw_indirect_count::{
    KHR_DRAW_INDIRECT_COUNT_EXTENSION_NAME, KHR_DRAW_INDIRECT_COUNT_SPEC_VERSION,
};
use crate::native::vulkan1_2::{FNCmdDrawIndexedIndirectCount, FNCmdDrawIndirectCount};
///See [`cmd_draw_indirect_count`]
#[doc(alias = "vkCmdDrawIndirectCountKHR")]
pub type FNCmdDrawIndirectCountKhr = FNCmdDrawIndirectCount;
///See [`cmd_draw_indexed_indirect_count`]
#[doc(alias = "vkCmdDrawIndexedIndirectCountKHR")]
pub type FNCmdDrawIndexedIndirectCountKhr = FNCmdDrawIndexedIndirectCount;
