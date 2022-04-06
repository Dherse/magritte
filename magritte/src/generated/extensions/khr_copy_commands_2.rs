//![VK_KHR_copy_commands2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_copy_commands2.html) - device extension
//!# Description
//!This extension provides extensible versions of the Vulkan buffer and image
//!copy commands.
//!The new commands are functionally identical to the core commands, except
//!that their copy parameters are specified using extensible structures that
//!can be used to pass extension-specific information.The following extensible copy commands are
//! introduced with this extension:
//![`cmd_copy_buffer2_khr`], [`cmd_copy_image2_khr`],
//![`cmd_copy_buffer_to_image2_khr`], [`cmd_copy_image_to_buffer2_khr`],
//![`cmd_blit_image2_khr`], and [`cmd_resolve_image2_khr`].
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
//! - [`cmd_blit_image2_khr`]
//! - [`cmd_copy_buffer2_khr`]
//! - [`cmd_copy_buffer_to_image2_khr`]
//! - [`cmd_copy_image2_khr`]
//! - [`cmd_copy_image_to_buffer2_khr`]
//! - [`cmd_resolve_image2_khr`]
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
//! - [`cmd_blit_image2_khr`]
//! - [`cmd_copy_buffer2_khr`]
//! - [`cmd_copy_buffer_to_image2_khr`]
//! - [`cmd_copy_image2_khr`]
//! - [`cmd_copy_image_to_buffer2_khr`]
//! - [`cmd_resolve_image2_khr`]
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
///The V-table of [`Device`] for functions from `VK_KHR_copy_commands2`
pub struct DeviceKhrCopyCommands2VTable {
    ///See [`FNCmdCopyBuffer2`] for more information.
    pub cmd_copy_buffer2_khr: FNCmdCopyBuffer2,
    ///See [`FNCmdCopyImage2`] for more information.
    pub cmd_copy_image2_khr: FNCmdCopyImage2,
    ///See [`FNCmdBlitImage2`] for more information.
    pub cmd_blit_image2_khr: FNCmdBlitImage2,
    ///See [`FNCmdCopyBufferToImage2`] for more information.
    pub cmd_copy_buffer_to_image2_khr: FNCmdCopyBufferToImage2,
    ///See [`FNCmdCopyImageToBuffer2`] for more information.
    pub cmd_copy_image_to_buffer2_khr: FNCmdCopyImageToBuffer2,
    ///See [`FNCmdResolveImage2`] for more information.
    pub cmd_resolve_image2_khr: FNCmdResolveImage2,
}
impl DeviceKhrCopyCommands2VTable {
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
            cmd_copy_buffer2_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdCopyBuffer2KHR").as_ptr()))
            },
            cmd_copy_image2_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdCopyImage2KHR").as_ptr()))
            },
            cmd_blit_image2_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdBlitImage2KHR").as_ptr()))
            },
            cmd_copy_buffer_to_image2_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdCopyBufferToImage2KHR").as_ptr()))
            },
            cmd_copy_image_to_buffer2_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdCopyImageToBuffer2KHR").as_ptr()))
            },
            cmd_resolve_image2_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdResolveImage2KHR").as_ptr()))
            },
        }
    }
    ///Gets [`Self::cmd_copy_buffer2_khr`]. See [`FNCmdCopyBuffer2`] for more information.
    pub fn cmd_copy_buffer2_khr(&self) -> FNCmdCopyBuffer2 {
        self.cmd_copy_buffer2_khr
    }
    ///Gets [`Self::cmd_copy_image2_khr`]. See [`FNCmdCopyImage2`] for more information.
    pub fn cmd_copy_image2_khr(&self) -> FNCmdCopyImage2 {
        self.cmd_copy_image2_khr
    }
    ///Gets [`Self::cmd_blit_image2_khr`]. See [`FNCmdBlitImage2`] for more information.
    pub fn cmd_blit_image2_khr(&self) -> FNCmdBlitImage2 {
        self.cmd_blit_image2_khr
    }
    ///Gets [`Self::cmd_copy_buffer_to_image2_khr`]. See [`FNCmdCopyBufferToImage2`] for more
    /// information.
    pub fn cmd_copy_buffer_to_image2_khr(&self) -> FNCmdCopyBufferToImage2 {
        self.cmd_copy_buffer_to_image2_khr
    }
    ///Gets [`Self::cmd_copy_image_to_buffer2_khr`]. See [`FNCmdCopyImageToBuffer2`] for more
    /// information.
    pub fn cmd_copy_image_to_buffer2_khr(&self) -> FNCmdCopyImageToBuffer2 {
        self.cmd_copy_image_to_buffer2_khr
    }
    ///Gets [`Self::cmd_resolve_image2_khr`]. See [`FNCmdResolveImage2`] for more information.
    pub fn cmd_resolve_image2_khr(&self) -> FNCmdResolveImage2 {
        self.cmd_resolve_image2_khr
    }
}
