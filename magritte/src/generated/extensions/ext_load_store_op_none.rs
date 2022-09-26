//![VK_EXT_load_store_op_none](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_load_store_op_none.html) - device extension
//!# Description
//!This extension incorporates `VK_ATTACHMENT_STORE_OP_NONE_EXT` from
//!`[`qcom_render_pass_store_ops`]`, enabling applications to avoid
//!unnecessary synchronization when an attachment is not written during a
//!render pass.Additionally, `VK_ATTACHMENT_LOAD_OP_NONE_EXT` is introduced to avoid
//!unnecessary synchronization when an attachment is not used during a render
//!pass at all.
//!In combination with `VK_ATTACHMENT_STORE_OP_NONE_EXT`, this is useful as
//!an alternative to preserve attachments in applications that cannot decide if
//!an attachment will be used in a render pass until after the necessary
//!pipelines have been created.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Shahbaz Youssefi [syoussefi](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_load_store_op_none]
//!   @syoussefi%0A<<Here describe the issue or question you have about the
//!   VK_EXT_load_store_op_none extension>>)
//!# New constants
//! - [`EXT_LOAD_STORE_OP_NONE_EXTENSION_NAME`]
//! - [`EXT_LOAD_STORE_OP_NONE_SPEC_VERSION`]
//! - Extending [`AttachmentLoadOp`]:  - `VK_ATTACHMENT_LOAD_OP_NONE_EXT`
//! - Extending [`AttachmentStoreOp`]:  - `VK_ATTACHMENT_STORE_OP_NONE_EXT`
//!# Version history
//! - Revision 1, 2021-06-06 (Shahbaz Youssefi)  - Initial revision, based on
//!   VK_QCOM_render_pass_store_ops.  - Added VK_ATTACHMENT_LOAD_OP_NONE_EXT.
//!# Other information
//! * 2021-06-06
//! * - Shahbaz Youssefi, Google  - Bill Licea-Kane, Qualcomm Technologies, Inc.  - Tobias Hector,
//!   AMD
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_LOAD_STORE_OP_NONE_SPEC_VERSION")]
pub const EXT_LOAD_STORE_OP_NONE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_LOAD_STORE_OP_NONE_EXTENSION_NAME")]
pub const EXT_LOAD_STORE_OP_NONE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_load_store_op_none");
