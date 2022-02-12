//![VK_KHR_push_descriptor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_push_descriptor.html) - device extension
//!# Description
//!This extension allows descriptors to be written into the command buffer,
//!while the implementation is responsible for managing their memory.
//!Push descriptors may enable easier porting from older APIs and in some cases
//!can be more efficient than writing descriptors into descriptor sets.
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_push_descriptor]
//!   @jeffbolznv%0A<<Here describe the issue or question you have about the VK_KHR_push_descriptor
//!   extension>>)
//!# New functions & commands
//! - [`CmdPushDescriptorSetKHR`]
//!If [`VK_KHR_descriptor_update_template`] is supported:
//! - [`CmdPushDescriptorSetWithTemplateKHR`]
//!If [Version 1.1]() is supported:
//! - [`CmdPushDescriptorSetWithTemplateKHR`]
//!# New structures
//! - Extending [`PhysicalDeviceProperties2`]:
//! - [`PhysicalDevicePushDescriptorPropertiesKHR`]
//!# New constants
//! - [`KHR_PUSH_DESCRIPTOR_EXTENSION_NAME`]
//! - [`KHR_PUSH_DESCRIPTOR_SPEC_VERSION`]
//! - Extending [`DescriptorSetLayoutCreateFlagBits`]:
//! - `VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR`
//!
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR`
//!
//!If [`VK_KHR_descriptor_update_template`] is supported:
//! - Extending [`DescriptorUpdateTemplateType`]:
//! - `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`
//!
//!If [Version 1.1]() is supported:
//! - Extending [`DescriptorUpdateTemplateType`]:
//! - `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`
//!# Version History
//! - Revision 1, 2016-10-15 (Jeff Bolz)
//! - Internal revisions
//!
//! - Revision 2, 2017-09-12 (Tobias Hector)
//! - Added interactions with Vulkan 1.1
//!# Other info
//! * 2017-09-12
//! * No known IP claims.
//!*
//! - Jeff Bolz, NVIDIA
//! - Michael Worcester, Imagination Technologies
//!# Related
//! - [`PhysicalDevicePushDescriptorPropertiesKHR`]
//! - [`CmdPushDescriptorSetKHR`]
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
#[doc(alias = "VK_KHR_PUSH_DESCRIPTOR_SPEC_VERSION")]
pub const KHR_PUSH_DESCRIPTOR_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_PUSH_DESCRIPTOR_EXTENSION_NAME")]
pub const KHR_PUSH_DESCRIPTOR_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_push_descriptor");
