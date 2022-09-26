//![VK_AMD_draw_indirect_count](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_AMD_draw_indirect_count.html) - device extension
//!# Description
//!This extension allows an application to source the number of draws for
//!indirect drawing commands from a buffer.
//!This enables applications to generate an arbitrary number of drawing
//!commands and execute them without host intervention.
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Deprecation State
//! - *Promoted* to `[`khr_draw_indirect_count`]` extension  - Which in turn was *promoted* to [Vulkan 1.2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.2-promotions)
//!# Contacts
//! - Daniel Rakos [drakos-amd](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_AMD_draw_indirect_count]
//!   @drakos-amd%0A<<Here describe the issue or question you have about the
//!   VK_AMD_draw_indirect_count extension>>)
//!# New commands
//! - [`cmd_draw_indexed_indirect_count_amd`]
//! - [`cmd_draw_indirect_count_amd`]
//!# New constants
//! - [`AMD_DRAW_INDIRECT_COUNT_EXTENSION_NAME`]
//! - [`AMD_DRAW_INDIRECT_COUNT_SPEC_VERSION`]
//!# Version history
//! - Revision 2, 2016-08-23 (Dominik Witczak)  - Minor fixes
//! - Revision 1, 2016-07-21 (Matthaeus Chajdas)  - Initial draft
//!# Other information
//! * 2016-08-23
//! * - Promoted to `[`khr_draw_indirect_count`]`
//! * No known IP claims.
//! * - Matthaeus G. Chajdas, AMD  - Derrick Owens, AMD  - Graham Sellers, AMD  - Daniel Rakos, AMD
//!   - Dominik Witczak, AMD
//!# Related
//! - [`cmd_draw_indexed_indirect_count_amd`]
//! - [`cmd_draw_indirect_count_amd`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    vulkan1_0::Device,
    vulkan1_2::{FNCmdDrawIndexedIndirectCount, FNCmdDrawIndirectCount},
};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_DRAW_INDIRECT_COUNT_SPEC_VERSION")]
pub const AMD_DRAW_INDIRECT_COUNT_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_DRAW_INDIRECT_COUNT_EXTENSION_NAME")]
pub const AMD_DRAW_INDIRECT_COUNT_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_AMD_draw_indirect_count");
///The V-table of [`Device`] for functions from `VK_AMD_draw_indirect_count`
pub struct DeviceAmdDrawIndirectCountVTable {
    ///See [`FNCmdDrawIndirectCount`] for more information.
    pub cmd_draw_indirect_count_amd: FNCmdDrawIndirectCount,
    ///See [`FNCmdDrawIndexedIndirectCount`] for more information.
    pub cmd_draw_indexed_indirect_count_amd: FNCmdDrawIndexedIndirectCount,
}
impl DeviceAmdDrawIndirectCountVTable {
    ///Loads the VTable from the owner and the names
    #[track_caller]
    pub fn load(
        loader_fn: unsafe extern "system" fn(
            Device,
            *const std::os::raw::c_char,
        ) -> Option<unsafe extern "system" fn()>,
        loader: Device,
    ) -> Self {
        Self {
            cmd_draw_indirect_count_amd: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdDrawIndirectCountAMD").as_ptr()))
            },
            cmd_draw_indexed_indirect_count_amd: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCmdDrawIndexedIndirectCountAMD").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::cmd_draw_indirect_count_amd`]. See [`FNCmdDrawIndirectCount`] for more
    /// information.
    pub fn cmd_draw_indirect_count_amd(&self) -> FNCmdDrawIndirectCount {
        self.cmd_draw_indirect_count_amd
    }
    ///Gets [`Self::cmd_draw_indexed_indirect_count_amd`]. See [`FNCmdDrawIndexedIndirectCount`]
    /// for more information.
    pub fn cmd_draw_indexed_indirect_count_amd(&self) -> FNCmdDrawIndexedIndirectCount {
        self.cmd_draw_indexed_indirect_count_amd
    }
}
