//![VK_EXT_external_memory_dma_buf](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_external_memory_dma_buf.html) - device extension
//!# Description
//!A `dma_buf` is a type of file descriptor, defined by the Linux kernel,
//!that allows sharing memory across kernel device drivers and across
//!processes.
//!This extension enables applications to import a `dma_buf` as
//![`DeviceMemory`], to export [`DeviceMemory`] as a `dma_buf`, and
//!to create [`Buffer`] objects that  **can**  be bound to that memory.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_external_memory_fd`]`
//!# Contacts
//! - Chad Versace [chadversary](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_external_memory_dma_buf]
//!   @chadversary%0A<<Here describe the issue or question you have about the
//!   VK_EXT_external_memory_dma_buf extension>>)
//!# New constants
//! - [`EXT_EXTERNAL_MEMORY_DMA_BUF_EXTENSION_NAME`]
//! - [`EXT_EXTERNAL_MEMORY_DMA_BUF_SPEC_VERSION`]
//! - Extending [`ExternalMemoryHandleTypeFlagBits`]:  -
//!   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF_BIT_EXT`
//!# Known issues & F.A.Q
//!1) How does the application, when creating a [`Image`] that it intends
//!to bind to `dma_buf`[`DeviceMemory`] containing an externally
//!produced image, specify the memory layout (such as row pitch and DRM format
//!modifier) of the [`Image`]? In other words, how does the application
//!achieve behavior comparable to that provided by
//![`EGL_EXT_image_dma_buf_import`](https://www.khronos.org/registry/EGL/extensions/EXT/EGL_EXT_image_dma_buf_import.txt)
//!and
//![`EGL_EXT_image_dma_buf_import_modifiers`](https://www.khronos.org/registry/EGL/extensions/EXT/EGL_EXT_image_dma_buf_import_modifiers.txt)
//!? **RESOLVED** : Features comparable to those in
//![`EGL_EXT_image_dma_buf_import`](https://www.khronos.org/registry/EGL/extensions/EXT/EGL_EXT_image_dma_buf_import.txt)
//!and
//![`EGL_EXT_image_dma_buf_import_modifiers`](https://www.khronos.org/registry/EGL/extensions/EXT/EGL_EXT_image_dma_buf_import_modifiers.txt)
//!will be provided by an extension layered atop this one.2) Without the ability to specify the
//! memory layout of external `dma_buf`
//!images, how is this extension useful? **RESOLVED** : This extension provides exactly one new
//! feature: the ability to
//!import/export between `dma_buf` and [`DeviceMemory`].
//!This feature, together with features provided by
//!`[`khr_external_memory_fd`]`, is sufficient to bind a [`Buffer`]
//!to `dma_buf`.
//!# Version History
//! - Revision 1, 2017-10-10 (Chad Versace)  - Squashed internal revisions
//!# Other info
//! * 2017-10-10
//! * No known IP claims.
//! * - Chad Versace, Google  - James Jones, NVIDIA  - Jason Ekstrand, Intel
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
#[doc(alias = "VK_EXT_EXTERNAL_MEMORY_DMA_BUF_SPEC_VERSION")]
pub const EXT_EXTERNAL_MEMORY_DMA_BUF_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_EXTERNAL_MEMORY_DMA_BUF_EXTENSION_NAME")]
pub const EXT_EXTERNAL_MEMORY_DMA_BUF_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_external_memory_dma_buf");
