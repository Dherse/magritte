pub use crate::common::extensions::amd_draw_indirect_count::{
    AMD_DRAW_INDIRECT_COUNT_EXTENSION_NAME, AMD_DRAW_INDIRECT_COUNT_SPEC_VERSION,
};
use crate::native::vulkan1_2::{FNCmdDrawIndexedIndirectCount, FNCmdDrawIndirectCount};
///See [`cmd_draw_indirect_count`]
#[doc(alias = "vkCmdDrawIndirectCountAMD")]
pub type FNCmdDrawIndirectCountAmd = FNCmdDrawIndirectCount;
///See [`cmd_draw_indexed_indirect_count`]
#[doc(alias = "vkCmdDrawIndexedIndirectCountAMD")]
pub type FNCmdDrawIndexedIndirectCountAmd = FNCmdDrawIndexedIndirectCount;
