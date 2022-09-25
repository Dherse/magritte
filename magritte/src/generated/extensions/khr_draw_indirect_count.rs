//![VK_KHR_draw_indirect_count](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_draw_indirect_count.html) - device extension
//!# Description
//!This extension is based off the `[`amd_draw_indirect_count`]`
//!extension.
//!This extension allows an application to source the number of draws for
//!indirect drawing calls from a buffer.Applications might want to do culling on the GPU via a
//! compute shader prior
//!to drawing.
//!This enables the application to generate an arbitrary number of drawing
//!commands and execute them without host intervention.
# ! [doc = concat ! ("# " , "Revision")]
//!1
# ! [doc = concat ! ("# " , "Dependencies")]
//! - Requires Vulkan 1.0
# ! [doc = concat ! ("# " , "Deprecation State")]
//! - *Promoted* to [Vulkan 1.2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.2-promotions)
# ! [doc = concat ! ("# " , "Contacts")]
//! - Piers Daniell [pdaniell-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_draw_indirect_count]
//!   @pdaniell-nv%0A<<Here describe the issue or question you have about the
//!   VK_KHR_draw_indirect_count extension>>)
# ! [doc = concat ! ("# " , "New commands")]
//! - [`cmd_draw_indexed_indirect_count_khr`]
//! - [`cmd_draw_indirect_count_khr`]
# ! [doc = concat ! ("# " , "New constants")]
//! - [`KHR_DRAW_INDIRECT_COUNT_EXTENSION_NAME`]
//! - [`KHR_DRAW_INDIRECT_COUNT_SPEC_VERSION`]
# ! [doc = concat ! ("# " , "Version history")]
//! - Revision 1, 2017-08-25 (Piers Daniell)  - Initial draft based off VK_AMD_draw_indirect_count
//!# Other info
//! * 2017-08-25
//! * - Promoted to Vulkan 1.2 Core
//! * No known IP claims.
//! * - Matthaeus G. Chajdas, AMD  - Derrick Owens, AMD  - Graham Sellers, AMD  - Daniel Rakos, AMD
//!   - Dominik Witczak, AMD  - Piers Daniell, NVIDIA
//!# Related
//! - [`cmd_draw_indexed_indirect_count_khr`]
//! - [`cmd_draw_indirect_count_khr`]
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
#[doc(alias = "VK_KHR_DRAW_INDIRECT_COUNT_SPEC_VERSION")]
pub const KHR_DRAW_INDIRECT_COUNT_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_DRAW_INDIRECT_COUNT_EXTENSION_NAME")]
pub const KHR_DRAW_INDIRECT_COUNT_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_draw_indirect_count");
///The V-table of [`Device`] for functions from `VK_KHR_draw_indirect_count`
pub struct DeviceKhrDrawIndirectCountVTable {
    ///See [`FNCmdDrawIndirectCount`] for more information.
    pub cmd_draw_indirect_count_khr: FNCmdDrawIndirectCount,
    ///See [`FNCmdDrawIndexedIndirectCount`] for more information.
    pub cmd_draw_indexed_indirect_count_khr: FNCmdDrawIndexedIndirectCount,
}
impl DeviceKhrDrawIndirectCountVTable {
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
            cmd_draw_indirect_count_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdDrawIndirectCountKHR").as_ptr()))
            },
            cmd_draw_indexed_indirect_count_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCmdDrawIndexedIndirectCountKHR").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::cmd_draw_indirect_count_khr`]. See [`FNCmdDrawIndirectCount`] for more
    /// information.
    pub fn cmd_draw_indirect_count_khr(&self) -> FNCmdDrawIndirectCount {
        self.cmd_draw_indirect_count_khr
    }
    ///Gets [`Self::cmd_draw_indexed_indirect_count_khr`]. See [`FNCmdDrawIndexedIndirectCount`]
    /// for more information.
    pub fn cmd_draw_indexed_indirect_count_khr(&self) -> FNCmdDrawIndexedIndirectCount {
        self.cmd_draw_indexed_indirect_count_khr
    }
}
