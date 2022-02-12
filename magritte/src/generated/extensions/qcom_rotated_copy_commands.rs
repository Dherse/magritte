//![VK_QCOM_rotated_copy_commands](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_QCOM_rotated_copy_commands.html) - device extension
//!# Description
//!This extension extends adds an optional rotation transform to copy commands
//![`CmdBlitImage2KHR`], [`CmdCopyImageToBuffer2KHR`] and
//![`CmdCopyBufferToImage2KHR`].
//!When copying between two resources, where one resource contains rotated
//!content and the other does not, a rotated copy may be desired.
//!This extension may be used in combination with VK_QCOM_render_pass_transform
//!which adds rotated render passes.This extension adds an extension structure to the following
//! commands:
//!vkCmdBlitImage2KHR, vkCmdCopyImageToBuffer2KHR and
//!vkCmdCopyBufferToImage2KHR
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_swapchain`]`
//! - Requires `[`VK_KHR_copy_commands2`]`
//!# Contacts
//! - Jeff Leger [jackohound](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_QCOM_rotated_copy_commands]
//!   @jackohound%0A<<Here describe the issue or question you have about the
//!   VK_QCOM_rotated_copy_commands extension>>)
//!# New structures
//! - Extending [`BufferImageCopy2KHR`], [`ImageBlit2KHR`]:
//! - [`CopyCommandTransformInfoQCOM`]
//!# New constants
//! - [`QCOM_ROTATED_COPY_COMMANDS_EXTENSION_NAME`]
//! - [`QCOM_ROTATED_COPY_COMMANDS_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_COPY_COMMAND_TRANSFORM_INFO_QCOM`
//!# Known issues & F.A.Q
//!1) What is an appropriate name for the added extension structure? The style
//!guide says “Structures which extend other structures through the
//!`pNext` chain should reflect the name of the base structure they
//!extend.”, but in this case a single extension structure is used to extend
//!three base structures (vkCmdBlitImage2KHR, vkCmdCopyImageToBuffer2KHR and
//!vkCmdCopyBufferToImage2KHR).
//!Creating three identical structures with unique names seemed undesirable.**RESOLVED**: Deviate
//! from the style guide for extension structure naming.2) Should this extension add a rotation
//! capability to vkCmdCopyImage2KHR?**RESOLVED**: No.
//!Use of rotated vkCmdBlitImage2KHR can fully address this use-case.3) Should this extension add a
//! rotation capability to vkCmdResolveImage2KHR?**RESOLVED** No.
//!Use of vkCmdResolveImage2KHR is very slow and extremely bandwidth intensive
//!on Qualcomm’s GPU architecture and use of pResolveAttachments in
//!vkRenderPass is the strongly preferred approach.
//!Therefore, we choose not to introduce a rotation capability to
//!vkCmdResolveImage2KHR.
//!# Version History
//! - Revision 1, 2020-09-19 (Jeff Leger)
//!# Other info
//! * 2020-09-18
//!*
//! - None
//!*
//! - Jeff Leger, Qualcomm Technologies, Inc.
//!# Related
//! - [`CopyCommandTransformInfoQCOM`]
//!
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
#[doc(alias = "VK_QCOM_ROTATED_COPY_COMMANDS_SPEC_VERSION")]
pub const QCOM_ROTATED_COPY_COMMANDS_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_QCOM_ROTATED_COPY_COMMANDS_EXTENSION_NAME")]
pub const QCOM_ROTATED_COPY_COMMANDS_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_QCOM_rotated_copy_commands");
