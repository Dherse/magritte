//![VK_KHR_device_group_creation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_device_group_creation.html) - instance extension
//!# Description
//!This extension provides instance-level commands to enumerate groups of
//!physical devices, and to create a logical device from a subset of one of
//!those groups.
//!Such a logical device can then be used with new features in the
//!`[`VK_KHR_device_group`]` extension.
//!# Revision
//!1
//!# Dependencies
//! - *Promoted* to [Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_device_group_creation]
//!   @jeffbolznv%0A<<Here describe the issue or question you have about the
//!   VK_KHR_device_group_creation extension>>)
//!# New functions & commands
//! - [`EnumeratePhysicalDeviceGroupsKHR`]
//!# New structures
//! - [`PhysicalDeviceGroupPropertiesKHR`]
//! - Extending [`DeviceCreateInfo`]:  - [`DeviceGroupDeviceCreateInfoKHR`]
//!# New constants
//! - [`KHR_DEVICE_GROUP_CREATION_EXTENSION_NAME`]
//! - [`KHR_DEVICE_GROUP_CREATION_SPEC_VERSION`]
//! - [`MAX_DEVICE_GROUP_SIZE_KHR`]
//! - Extending [`MemoryHeapFlagBits`]:  - `VK_MEMORY_HEAP_MULTI_INSTANCE_BIT_KHR`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES_KHR`
//!# Version History
//! - Revision 1, 2016-10-19 (Jeff Bolz)  - Internal revisions
//!# Other info
//! * 2016-10-19
//! * No known IP claims.
//! * - Promoted to Vulkan 1.1 Core
//! * - Jeff Bolz, NVIDIA
//!# Related
//! - [`MAX_DEVICE_GROUP_SIZE_KHR`]
//! - [`DeviceGroupDeviceCreateInfoKHR`]
//! - [`PhysicalDeviceGroupPropertiesKHR`]
//! - [`EnumeratePhysicalDeviceGroupsKHR`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{vulkan1_0::Instance, vulkan1_1::FNEnumeratePhysicalDeviceGroups};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_DEVICE_GROUP_CREATION_SPEC_VERSION")]
pub const KHR_DEVICE_GROUP_CREATION_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_DEVICE_GROUP_CREATION_EXTENSION_NAME")]
pub const KHR_DEVICE_GROUP_CREATION_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_device_group_creation");
///The V-table of [`Instance`] for functions from VK_KHR_device_group_creation
pub struct InstanceKhrDeviceGroupCreationVTable {
    ///See [`FNEnumeratePhysicalDeviceGroups`] for more information.
    pub enumerate_physical_device_groups: FNEnumeratePhysicalDeviceGroups,
}
impl InstanceKhrDeviceGroupCreationVTable {
    ///Loads the VTable from the owner and the names
    pub fn load<F>(loader_fn: F, loader: Instance) -> Self
    where
        F: Fn(Instance, &'static CStr) -> Option<extern "system" fn()>,
    {
        Self {
            enumerate_physical_device_groups: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkEnumeratePhysicalDeviceGroupsKHR")))
            },
        }
    }
    ///Gets [`Self::enumerate_physical_device_groups`]. See [`FNEnumeratePhysicalDeviceGroups`] for
    /// more information.
    pub fn enumerate_physical_device_groups(&self) -> FNEnumeratePhysicalDeviceGroups {
        self.enumerate_physical_device_groups
    }
}
