//![VK_KHR_create_renderpass2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_create_renderpass2.html) - device extension
//!# Description
//!This extension provides a new entry point to create render passes in a way
//!that can be easily extended by other extensions through the substructures of
//!render pass creation.
//!The Vulkan 1.0 render pass creation sub-structures do not include
//!`sType`/`pNext` members.
//!Additionally, the render pass begin/next/end commands have been augmented
//!with new extensible structures for passing additional subpass information.The
//! [`RenderPassMultiviewCreateInfo`] and
//![`InputAttachmentAspectReference`] structures that extended the original
//![`RenderPassCreateInfo`] are not accepted into the new creation
//!functions, and instead their parameters are folded into this extension as
//!follows:
//! - Elements of [`RenderPassMultiviewCreateInfo::view_masks`] are now specified in
//!   [`SubpassDescription2KHR::view_mask`].
//! - Elements of [`RenderPassMultiviewCreateInfo::view_offsets`] are now specified in
//!   [`SubpassDependency2KHR::view_offset`].
//! - [`RenderPassMultiviewCreateInfo::correlation_mask_count`] and
//!   [`RenderPassMultiviewCreateInfo::correlation_masks`] are directly specified in
//!   [`RenderPassCreateInfo2KHR`].
//! - [`InputAttachmentAspectReference::aspect_mask`] is now specified in the relevant input
//!   attachment description in [`AttachmentDescription2KHR`]`::aspectMask`
//!The details of these mappings are explained fully in the new structures.
//!# Revision
//!1
//!# Dependencies
//! - *Promoted* to [Vulkan 1.2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.2-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_multiview`]`
//! - Requires `[`VK_KHR_maintenance2`]`
//!# Contacts
//! - Tobias Hector [tobias](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_create_renderpass2]
//!   @tobias%0A<<Here describe the issue or question you have about the VK_KHR_create_renderpass2
//!   extension>>)
//!# New functions & commands
//! - [`CmdBeginRenderPass2KHR`]
//! - [`CmdEndRenderPass2KHR`]
//! - [`CmdNextSubpass2KHR`]
//! - [`CreateRenderPass2KHR`]
//!# New structures
//! - [`AttachmentDescription2KHR`]
//! - [`AttachmentReference2KHR`]
//! - [`RenderPassCreateInfo2KHR`]
//! - [`SubpassBeginInfoKHR`]
//! - [`SubpassDependency2KHR`]
//! - [`SubpassDescription2KHR`]
//! - [`SubpassEndInfoKHR`]
//!# New constants
//! - [`KHR_CREATE_RENDERPASS_2_EXTENSION_NAME`]
//! - [`KHR_CREATE_RENDERPASS_2_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_2_KHR`  -
//!   `VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_2_KHR`  -
//!   `VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO_2_KHR`  -
//!   `VK_STRUCTURE_TYPE_SUBPASS_BEGIN_INFO_KHR`  - `VK_STRUCTURE_TYPE_SUBPASS_DEPENDENCY_2_KHR`  -
//!   `VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_2_KHR`  - `VK_STRUCTURE_TYPE_SUBPASS_END_INFO_KHR`
//!# Version History
//! - Revision 1, 2018-02-07 (Tobias Hector)  - Internal revisions
//!# Other info
//! * 2018-02-07
//! * - Promoted to Vulkan 1.2 Core
//! * - Tobias Hector  - Jeff Bolz
//!# Related
//! - [`AttachmentDescription2KHR`]
//! - [`AttachmentReference2KHR`]
//! - [`RenderPassCreateInfo2KHR`]
//! - [`SubpassBeginInfoKHR`]
//! - [`SubpassDependency2KHR`]
//! - [`SubpassDescription2KHR`]
//! - [`SubpassEndInfoKHR`]
//! - [`CmdBeginRenderPass2KHR`]
//! - [`CmdEndRenderPass2KHR`]
//! - [`CmdNextSubpass2KHR`]
//! - [`CreateRenderPass2KHR`]
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
    vulkan1_2::{FNCmdBeginRenderPass2, FNCmdEndRenderPass2, FNCmdNextSubpass2, FNCreateRenderPass2},
};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_CREATE_RENDERPASS_2_SPEC_VERSION")]
pub const KHR_CREATE_RENDERPASS_2_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_CREATE_RENDERPASS_2_EXTENSION_NAME")]
pub const KHR_CREATE_RENDERPASS_2_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_create_renderpass2");
///The V-table of [`Device`] for functions from VK_KHR_create_renderpass2
pub struct DeviceKhrCreateRenderpass2VTable {
    ///See [`FNCreateRenderPass2`] for more information.
    pub create_render_pass2: FNCreateRenderPass2,
    ///See [`FNCmdBeginRenderPass2`] for more information.
    pub cmd_begin_render_pass2: FNCmdBeginRenderPass2,
    ///See [`FNCmdNextSubpass2`] for more information.
    pub cmd_next_subpass2: FNCmdNextSubpass2,
    ///See [`FNCmdEndRenderPass2`] for more information.
    pub cmd_end_render_pass2: FNCmdEndRenderPass2,
}
impl DeviceKhrCreateRenderpass2VTable {
    ///Loads the VTable from the owner and the names
    pub fn load<F>(loader_fn: F, loader: Device) -> Self
    where
        F: Fn(Device, &'static CStr) -> Option<extern "system" fn()>,
    {
        Self {
            create_render_pass2: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCreateRenderPass2KHR")))
            },
            cmd_begin_render_pass2: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdBeginRenderPass2KHR")))
            },
            cmd_next_subpass2: unsafe { std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdNextSubpass2KHR"))) },
            cmd_end_render_pass2: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdEndRenderPass2KHR")))
            },
        }
    }
    ///Gets [`Self::create_render_pass2`]. See [`FNCreateRenderPass2`] for more information.
    pub fn create_render_pass2(&self) -> FNCreateRenderPass2 {
        self.create_render_pass2
    }
    ///Gets [`Self::cmd_begin_render_pass2`]. See [`FNCmdBeginRenderPass2`] for more information.
    pub fn cmd_begin_render_pass2(&self) -> FNCmdBeginRenderPass2 {
        self.cmd_begin_render_pass2
    }
    ///Gets [`Self::cmd_next_subpass2`]. See [`FNCmdNextSubpass2`] for more information.
    pub fn cmd_next_subpass2(&self) -> FNCmdNextSubpass2 {
        self.cmd_next_subpass2
    }
    ///Gets [`Self::cmd_end_render_pass2`]. See [`FNCmdEndRenderPass2`] for more information.
    pub fn cmd_end_render_pass2(&self) -> FNCmdEndRenderPass2 {
        self.cmd_end_render_pass2
    }
}
