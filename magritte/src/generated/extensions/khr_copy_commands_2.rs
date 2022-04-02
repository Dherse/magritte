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
use crate::{
    vulkan1_0::Device,
    vulkan1_3::{
        FNCmdBlitImage2, FNCmdCopyBuffer2, FNCmdCopyBufferToImage2, FNCmdCopyImage2, FNCmdCopyImageToBuffer2,
        FNCmdResolveImage2,
    },
};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_COPY_COMMANDS_2_SPEC_VERSION")]
pub const KHR_COPY_COMMANDS_2_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_COPY_COMMANDS_2_EXTENSION_NAME")]
pub const KHR_COPY_COMMANDS_2_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_copy_commands2");
///The V-table of [`Device`] for functions from VK_KHR_copy_commands2
pub struct DeviceKhrCopyCommands2VTable {
    ///See [`FNCmdCopyBuffer2`] for more information.
    pub cmd_copy_buffer2: FNCmdCopyBuffer2,
    ///See [`FNCmdCopyImage2`] for more information.
    pub cmd_copy_image2: FNCmdCopyImage2,
    ///See [`FNCmdBlitImage2`] for more information.
    pub cmd_blit_image2: FNCmdBlitImage2,
    ///See [`FNCmdCopyBufferToImage2`] for more information.
    pub cmd_copy_buffer_to_image2: FNCmdCopyBufferToImage2,
    ///See [`FNCmdCopyImageToBuffer2`] for more information.
    pub cmd_copy_image_to_buffer2: FNCmdCopyImageToBuffer2,
    ///See [`FNCmdResolveImage2`] for more information.
    pub cmd_resolve_image2: FNCmdResolveImage2,
}
impl DeviceKhrCopyCommands2VTable {
    ///Loads the VTable from the owner and the names
    pub fn load<F>(loader_fn: F, loader: Device) -> Self
    where
        F: Fn(Device, &'static CStr) -> Option<extern "system" fn()>,
    {
        Self {
            cmd_copy_buffer2: unsafe { std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdCopyBuffer2KHR"))) },
            cmd_copy_image2: unsafe { std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdCopyImage2KHR"))) },
            cmd_blit_image2: unsafe { std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdBlitImage2KHR"))) },
            cmd_copy_buffer_to_image2: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdCopyBufferToImage2KHR")))
            },
            cmd_copy_image_to_buffer2: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdCopyImageToBuffer2KHR")))
            },
            cmd_resolve_image2: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdResolveImage2KHR")))
            },
        }
    }
    ///Gets [`Self::cmd_copy_buffer2`]. See [`FNCmdCopyBuffer2`] for more information.
    pub fn cmd_copy_buffer2(&self) -> FNCmdCopyBuffer2 {
        self.cmd_copy_buffer2
    }
    ///Gets [`Self::cmd_copy_image2`]. See [`FNCmdCopyImage2`] for more information.
    pub fn cmd_copy_image2(&self) -> FNCmdCopyImage2 {
        self.cmd_copy_image2
    }
    ///Gets [`Self::cmd_blit_image2`]. See [`FNCmdBlitImage2`] for more information.
    pub fn cmd_blit_image2(&self) -> FNCmdBlitImage2 {
        self.cmd_blit_image2
    }
    ///Gets [`Self::cmd_copy_buffer_to_image2`]. See [`FNCmdCopyBufferToImage2`] for more
    /// information.
    pub fn cmd_copy_buffer_to_image2(&self) -> FNCmdCopyBufferToImage2 {
        self.cmd_copy_buffer_to_image2
    }
    ///Gets [`Self::cmd_copy_image_to_buffer2`]. See [`FNCmdCopyImageToBuffer2`] for more
    /// information.
    pub fn cmd_copy_image_to_buffer2(&self) -> FNCmdCopyImageToBuffer2 {
        self.cmd_copy_image_to_buffer2
    }
    ///Gets [`Self::cmd_resolve_image2`]. See [`FNCmdResolveImage2`] for more information.
    pub fn cmd_resolve_image2(&self) -> FNCmdResolveImage2 {
        self.cmd_resolve_image2
    }
}
