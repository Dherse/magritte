//![VK_KHR_uniform_buffer_standard_layout](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_uniform_buffer_standard_layout.html) - device extension
//!# Description
//!This extension enables tighter array and struct packing to be used with
//!uniform buffers.It modifies the alignment rules for uniform buffers, allowing for tighter
//!packing of arrays and structures.
//!This allows, for example, the std430 layout, as defined in
//![GLSL](https://www.khronos.org/registry/OpenGL/specs/gl/GLSLangSpec.4.60.pdf)
//!to be supported in uniform buffers.
//!# Revision
//!1
//!# Dependencies
//! - *Promoted* to [Vulkan 1.2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.2-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_get_physical_device_properties2`]`
//!# Contacts
//! - Graeme Leese [gnl21](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_uniform_buffer_standard_layout]
//!   @gnl21%0A<<Here describe the issue or question you have about the
//!   VK_KHR_uniform_buffer_standard_layout extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceUniformBufferStandardLayoutFeaturesKHR`]
//!# New constants
//! - [`KHR_UNIFORM_BUFFER_STANDARD_LAYOUT_EXTENSION_NAME`]
//! - [`KHR_UNIFORM_BUFFER_STANDARD_LAYOUT_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES_KHR`
//!# Version History
//! - Revision 1, 2019-01-25 (Graeme Leese)  - Initial draft
//!# Other info
//! * 2019-01-25
//! * - Promoted to Vulkan 1.2 Core
//! * - Graeme Leese, Broadcom  - Jeff Bolz, NVIDIA  - Tobias Hector, AMD  - Jason Ekstrand, Intel
//!   - Neil Henning, AMD
//!# Related
//! - [`PhysicalDeviceUniformBufferStandardLayoutFeaturesKHR`]
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
#[doc(alias = "VK_KHR_UNIFORM_BUFFER_STANDARD_LAYOUT_SPEC_VERSION")]
pub const KHR_UNIFORM_BUFFER_STANDARD_LAYOUT_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_UNIFORM_BUFFER_STANDARD_LAYOUT_EXTENSION_NAME")]
pub const KHR_UNIFORM_BUFFER_STANDARD_LAYOUT_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_KHR_uniform_buffer_standard_layout");
