//![VK_EXT_tooling_info](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_tooling_info.html) - device extension
//!# Description
//!When an error occurs during application development, a common question is
//!"What tools are actually running right now?" This extension adds the ability
//!to query that information directly from the Vulkan implementation.Outdated versions of one tool
//! might not play nicely with another, or perhaps
//!a tool is not actually running when it should have been.
//!Trying to figure that out can cause headaches as it is necessary to consult
//!each known tool to figure out what is going on — in some cases the tool
//!might not even be known.Typically, the expectation is that developers will simply print out this
//!information for visual inspection when an issue occurs, however a small
//!amount of semantic information about what the tool is doing is provided to
//!help identify it programmatically.
//!For example, if the advertised limits or features of an implementation are
//!unexpected, is there a tool active which modifies these limits? Or if an
//!application is providing debug markers, but the implementation is not
//!actually doing anything with that information, this can quickly point that
//!out.
//!# Revision
//!1
//!# Dependencies
//! - *Promoted* to [Vulkan 1.3](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.3-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Tobias Hector [tobski](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_tooling_info]
//!   @tobski%0A<<Here describe the issue or question you have about the VK_EXT_tooling_info
//!   extension>>)
//!# New functions & commands
//! - [`GetPhysicalDeviceToolPropertiesEXT`]
//!# New structures
//! - [`PhysicalDeviceToolPropertiesEXT`]
//!# New enums
//! - [`ToolPurposeFlagBitsEXT`]
//!# New bitmasks
//! - [`ToolPurposeFlagsEXT`]
//!# New constants
//! - [`EXT_TOOLING_INFO_EXTENSION_NAME`]
//! - [`EXT_TOOLING_INFO_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TOOL_PROPERTIES_EXT`
//!If [`VK_EXT_debug_marker`] is supported:
//! - Extending [`ToolPurposeFlagBits`]:  - `VK_TOOL_PURPOSE_DEBUG_MARKERS_BIT_EXT`
//!If [`VK_EXT_debug_report`] is supported:
//! - Extending [`ToolPurposeFlagBits`]:  - `VK_TOOL_PURPOSE_DEBUG_REPORTING_BIT_EXT`
//!If [`VK_EXT_debug_utils`] is supported:
//! - Extending [`ToolPurposeFlagBits`]:  - `VK_TOOL_PURPOSE_DEBUG_MARKERS_BIT_EXT`  -
//!   `VK_TOOL_PURPOSE_DEBUG_REPORTING_BIT_EXT`
//!# Known issues & F.A.Q
//!1) Why is this information separate from the layer mechanism?Some tooling may be built into a
//! driver, or be part of the Vulkan loader
//!etc.
//!Tying this information directly to layers would have been awkward at best.
//!# Version History
//! - Revision 1, 2018-11-05 (Tobias Hector)  - Initial draft
//!# Other info
//! * 2018-11-05
//! * - Promoted to Vulkan 1.3 Core
//! * - Rolando Caloca  - Matthaeus Chajdas  - Baldur Karlsson  - Daniel Rakos
//!# Related
//! - [`PhysicalDeviceToolPropertiesEXT`]
//! - [`ToolPurposeFlagBitsEXT`]
//! - [`ToolPurposeFlagsEXT`]
//! - [`GetPhysicalDeviceToolPropertiesEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{vulkan1_0::Instance, vulkan1_3::FNGetPhysicalDeviceToolProperties};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_TOOLING_INFO_SPEC_VERSION")]
pub const EXT_TOOLING_INFO_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_TOOLING_INFO_EXTENSION_NAME")]
pub const EXT_TOOLING_INFO_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_tooling_info");
///The V-table of [`Instance`] for functions from VK_EXT_tooling_info
pub struct InstanceExtToolingInfoVTable {
    ///See [`FNGetPhysicalDeviceToolProperties`] for more information.
    pub get_physical_device_tool_properties: FNGetPhysicalDeviceToolProperties,
}
impl InstanceExtToolingInfoVTable {
    ///Loads the VTable from the owner and the names
    pub fn load<F>(loader_fn: F, loader: Instance) -> Self
    where
        F: Fn(Instance, &'static CStr) -> Option<extern "system" fn()>,
    {
        Self {
            get_physical_device_tool_properties: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkGetPhysicalDeviceToolPropertiesEXT")))
            },
        }
    }
    ///Gets [`Self::get_physical_device_tool_properties`]. See
    /// [`FNGetPhysicalDeviceToolProperties`] for more information.
    pub fn get_physical_device_tool_properties(&self) -> FNGetPhysicalDeviceToolProperties {
        self.get_physical_device_tool_properties
    }
}
