//![VK_QCOM_render_pass_store_ops](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_QCOM_render_pass_store_ops.html) - device extension
//!# Description
//!Renderpass attachments  **can**  be read-only for the duration of a render pass.Examples include
//! input attachments and depth attachments where depth tests
//!are enabled but depth writes are not enabled.In such cases, there  **can**  be no contents
//! generated for an attachment within
//!the render area.This extension adds a new
//! [`AttachmentStoreOp`]`VK_ATTACHMENT_STORE_OP_NONE_QCOM` specifying that the contents within
//!the render area  **may**  not be written to memory, but that the prior contents
//!of the attachment in memory are preserved.
//!However, if any contents were generated within the render area during
//!rendering, the contents of the attachment will be undefined inside the
//!render area.
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Bill Licea-Kane [wwlk](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_QCOM_render_pass_store_ops]
//!   @wwlk%0A<<Here describe the issue or question you have about the VK_QCOM_render_pass_store_ops
//!   extension>>)
//!# New constants
//! - [`QCOM_RENDER_PASS_STORE_OPS_EXTENSION_NAME`]
//! - [`QCOM_RENDER_PASS_STORE_OPS_SPEC_VERSION`]
//! - Extending [`AttachmentStoreOp`]:  - `VK_ATTACHMENT_STORE_OP_NONE_QCOM`
//!# Version History
//! - Revision 1, 2019-12-20 (wwlk)  - Initial version
//! - Revision 2, 2020-03-25 (wwlk)  - Minor renaming
//!# Other info
//! * 2020-03-25
//! * - Bill Licea-Kane, Qualcomm Technologies, Inc.
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
#[doc(alias = "VK_QCOM_RENDER_PASS_STORE_OPS_SPEC_VERSION")]
pub const QCOM_RENDER_PASS_STORE_OPS_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_QCOM_RENDER_PASS_STORE_OPS_EXTENSION_NAME")]
pub const QCOM_RENDER_PASS_STORE_OPS_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_QCOM_render_pass_store_ops");
