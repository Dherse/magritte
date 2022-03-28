//![VK_KHR_copy_commands2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_copy_commands2.html) - device extension
//!# Description
//!This extension provides extensible versions of the Vulkan buffer and image
//!copy commands.
//!The new commands are functionally identical to the core commands, except
//!that their copy parameters are specified using extensible structures that
//!can be used to pass extension-specific information.The following extensible copy commands are
//! introduced with this extension:
//![`CmdCopyBuffer2KHR`], [`CmdCopyImage2KHR`],
//![`CmdCopyBufferToImage2KHR`], [`CmdCopyImageToBuffer2KHR`],
//![`CmdBlitImage2KHR`], and [`CmdResolveImage2KHR`].
//!Each command contains an `*Info2KHR` structure parameter that includes
//!`sType`/`pNext` members.
//!Lower level structures describing each region to be copied are also extended
//!with `sType`/`pNext` members.
//!# Revision
//!1
//!# Dependencies
//! - *Promoted* to [Vulkan 1.3](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.3-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Jeff Leger [jackohound](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_copy_commands2]
//!   @jackohound%0A<<Here describe the issue or question you have about the VK_KHR_copy_commands2
//!   extension>>)
//!# New functions & commands
//! - [`CmdBlitImage2KHR`]
//! - [`CmdCopyBuffer2KHR`]
//! - [`CmdCopyBufferToImage2KHR`]
//! - [`CmdCopyImage2KHR`]
//! - [`CmdCopyImageToBuffer2KHR`]
//! - [`CmdResolveImage2KHR`]
//!# New structures
//! - [`BlitImageInfo2KHR`]
//! - [`BufferCopy2KHR`]
//! - [`BufferImageCopy2KHR`]
//! - [`CopyBufferInfo2KHR`]
//! - [`CopyBufferToImageInfo2KHR`]
//! - [`CopyImageInfo2KHR`]
//! - [`CopyImageToBufferInfo2KHR`]
//! - [`ImageBlit2KHR`]
//! - [`ImageCopy2KHR`]
//! - [`ImageResolve2KHR`]
//! - [`ResolveImageInfo2KHR`]
//!# New constants
//! - [`KHR_COPY_COMMANDS_2_EXTENSION_NAME`]
//! - [`KHR_COPY_COMMANDS_2_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_BLIT_IMAGE_INFO_2_KHR`  -
//!   `VK_STRUCTURE_TYPE_BUFFER_COPY_2_KHR`  - `VK_STRUCTURE_TYPE_BUFFER_IMAGE_COPY_2_KHR`  -
//!   `VK_STRUCTURE_TYPE_COPY_BUFFER_INFO_2_KHR`  -
//!   `VK_STRUCTURE_TYPE_COPY_BUFFER_TO_IMAGE_INFO_2_KHR`  -
//!   `VK_STRUCTURE_TYPE_COPY_IMAGE_INFO_2_KHR`  -
//!   `VK_STRUCTURE_TYPE_COPY_IMAGE_TO_BUFFER_INFO_2_KHR`  - `VK_STRUCTURE_TYPE_IMAGE_BLIT_2_KHR`  -
//!   `VK_STRUCTURE_TYPE_IMAGE_COPY_2_KHR`  - `VK_STRUCTURE_TYPE_IMAGE_RESOLVE_2_KHR`  -
//!   `VK_STRUCTURE_TYPE_RESOLVE_IMAGE_INFO_2_KHR`
//!# Version History
//! - Revision 1, 2020-07-06 (Jeff Leger)  - Internal revisions
//!# Other info
//! * 2020-07-06
//! * - Promoted to Vulkan 1.3 Core
//! * - None
//! * - Jeff Leger, Qualcomm  - Tobias Hector, AMD  - Jan-Harald Fredriksen, ARM  - Tom Olson, ARM
//!# Related
//! - [`BlitImageInfo2KHR`]
//! - [`BufferCopy2KHR`]
//! - [`BufferImageCopy2KHR`]
//! - [`CopyBufferInfo2KHR`]
//! - [`CopyBufferToImageInfo2KHR`]
//! - [`CopyImageInfo2KHR`]
//! - [`CopyImageToBufferInfo2KHR`]
//! - [`ImageBlit2KHR`]
//! - [`ImageCopy2KHR`]
//! - [`ImageResolve2KHR`]
//! - [`ResolveImageInfo2KHR`]
//! - [`CmdBlitImage2KHR`]
//! - [`CmdCopyBuffer2KHR`]
//! - [`CmdCopyBufferToImage2KHR`]
//! - [`CmdCopyImage2KHR`]
//! - [`CmdCopyImageToBuffer2KHR`]
//! - [`CmdResolveImage2KHR`]
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
#[doc(alias = "VK_KHR_COPY_COMMANDS_2_SPEC_VERSION")]
pub const KHR_COPY_COMMANDS_2_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_COPY_COMMANDS_2_EXTENSION_NAME")]
pub const KHR_COPY_COMMANDS_2_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_copy_commands2");
