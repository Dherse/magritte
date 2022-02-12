//![VK_NV_device_diagnostic_checkpoints](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_device_diagnostic_checkpoints.html) - device extension
//!# Description
//!This extension allows applications to insert markers in the command stream
//!and associate them with custom data.If a device lost error occurs, the application **may** then
//! query the
//!implementation for the last markers to cross specific implementation-defined
//!pipeline stages, in order to narrow down which commands were executing at
//!the time and might have caused the failure.
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Nuno Subtil [nsubtil](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_device_diagnostic_checkpoints]
//!   @nsubtil%0A<<Here describe the issue or question you have about the
//!   VK_NV_device_diagnostic_checkpoints extension>>)
//!# New functions & commands
//! - [`CmdSetCheckpointNV`]
//! - [`GetQueueCheckpointDataNV`]
//!# New structures
//! - [`CheckpointDataNV`]
//! - Extending [`QueueFamilyProperties2`]:
//! - [`QueueFamilyCheckpointPropertiesNV`]
//!# New constants
//! - [`NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_EXTENSION_NAME`]
//! - [`NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_CHECKPOINT_DATA_NV`
//! - `VK_STRUCTURE_TYPE_QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV`
//!# Version History
//! - Revision 1, 2018-07-16 (Nuno Subtil)
//! - Internal revisions
//! - Revision 2, 2018-07-16 (Nuno Subtil)
//! - ???
//!# Other info
//! * 2018-07-16
//!*
//! - Oleg Kuznetsov, NVIDIA
//! - Alex Dunn, NVIDIA
//! - Jeff Bolz, NVIDIA
//! - Eric Werness, NVIDIA
//! - Daniel Koch, NVIDIA
//!# Related
//! - [`CheckpointDataNV`]
//! - [`QueueFamilyCheckpointPropertiesNV`]
//! - [`CmdSetCheckpointNV`]
//! - [`GetQueueCheckpointDataNV`]
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
#[doc(alias = "VK_NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_SPEC_VERSION")]
pub const NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_EXTENSION_NAME")]
pub const NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_NV_device_diagnostic_checkpoints");
