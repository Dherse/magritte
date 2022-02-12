//![VK_VALVE_mutable_descriptor_type](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VALVE_mutable_descriptor_type.html) - device extension
//!# Description
//!This extension allows applications to reduce descriptor memory footprint by
//!allowing a descriptor to be able to mutate to a given list of descriptor
//!types depending on which descriptor types are written into, or copied into a
//!descriptor set.The main use case this extension intends to address is descriptor indexing
//!with `VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT` where the
//!descriptor types are completely generic, as this means applications can
//!allocate one large descriptor set, rather than having one large descriptor
//!set per descriptor type, which significantly bloats descriptor memory usage
//!and causes performance issues.This extension also adds a mechanism to declare that a descriptor
//! pool, and
//!therefore the descriptor sets that are allocated from it, reside only in
//!host memory; as such these descriptors can only be updated/copied, but not
//!bound.These features together allow much more efficient emulation of the raw D3D12
//!binding model.
//!This extension is primarily intended to be useful for API layering efforts.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_maintenance3`]`
//!# Contacts
//! - Joshua Ashton [Joshua-Ashton](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_VALVE_mutable_descriptor_type]
//!   @Joshua-Ashton%0A<<Here describe the issue or question you have about the
//!   VK_VALVE_mutable_descriptor_type extension>>)
//! - Hans-Kristian Arntzen [HansKristian-Work](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_VALVE_mutable_descriptor_type]
//!   @HansKristian-Work%0A<<Here describe the issue or question you have about the
//!   VK_VALVE_mutable_descriptor_type extension>>)
//!# New structures
//! - [`MutableDescriptorTypeListVALVE`]
//! - Extending [`DescriptorSetLayoutCreateInfo`], [`DescriptorPoolCreateInfo`]:
//! - [`MutableDescriptorTypeCreateInfoVALVE`]
//!
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceMutableDescriptorTypeFeaturesVALVE`]
//!# New constants
//! - [`VALVE_MUTABLE_DESCRIPTOR_TYPE_EXTENSION_NAME`]
//! - [`VALVE_MUTABLE_DESCRIPTOR_TYPE_SPEC_VERSION`]
//! - Extending [`DescriptorPoolCreateFlagBits`]:
//! - `VK_DESCRIPTOR_POOL_CREATE_HOST_ONLY_BIT_VALVE`
//!
//! - Extending [`DescriptorSetLayoutCreateFlagBits`]:
//! - `VK_DESCRIPTOR_SET_LAYOUT_CREATE_HOST_ONLY_POOL_BIT_VALVE`
//!
//! - Extending [`DescriptorType`]:
//! - `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE`
//!
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_VALVE`
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_VALVE`
//!# Version History
//! - Revision 1, 2020-12-01 (Joshua Ashton, Hans-Kristian Arntzen)
//! - Initial specification, squashed from public draft.
//!# Other info
//! * 2020-12-02
//! * No known IP claims.
//!*
//! - Joshua Ashton, Valve
//! - Hans-Kristian Arntzen, Valve
//!# Related
//! - [`MutableDescriptorTypeCreateInfoVALVE`]
//! - [`MutableDescriptorTypeListVALVE`]
//! - [`PhysicalDeviceMutableDescriptorTypeFeaturesVALVE`]
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
#[doc(alias = "VK_VALVE_MUTABLE_DESCRIPTOR_TYPE_SPEC_VERSION")]
pub const VALVE_MUTABLE_DESCRIPTOR_TYPE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_VALVE_MUTABLE_DESCRIPTOR_TYPE_EXTENSION_NAME")]
pub const VALVE_MUTABLE_DESCRIPTOR_TYPE_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_VALVE_mutable_descriptor_type");
