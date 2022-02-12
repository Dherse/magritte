//![VK_KHR_variable_pointers](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_variable_pointers.html) - device extension
//!# Description
//!The [`VK_KHR_variable_pointers`] extension allows implementations to indicate
//!their level of support for the `SPV_KHR_variable_pointers` SPIR-V extension.
//!The SPIR-V extension allows shader modules to use invocation-private
//!pointers into uniform and/or storage buffers, where the pointer values can
//!be dynamic and non-uniform.The `SPV_KHR_variable_pointers` extension introduces two
//! capabilities.
//!The first, `VariablePointersStorageBuffer`, **must** be supported by all
//!implementations of this extension.
//!The second, `VariablePointers`, is optional.
//!# Revision
//!1
//!# Dependencies
//! - *Promoted* to
//![Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//! - Requires `[`VK_KHR_storage_buffer_storage_class`]`
//!# Contacts
//! - Jesse Hall [critsec](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_variable_pointers]
//!   @critsec%0A<<Here describe the issue or question you have about the VK_KHR_variable_pointers
//!   extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceVariablePointerFeaturesKHR`]
//! - [`PhysicalDeviceVariablePointersFeaturesKHR`]
//!# New constants
//! - [`KHR_VARIABLE_POINTERS_EXTENSION_NAME`]
//! - [`KHR_VARIABLE_POINTERS_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR`
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES_KHR`
//!# Known issues & F.A.Q
//!1) Do we need an optional property for the SPIR-V
//!`VariablePointersStorageBuffer` capability or should it be mandatory when
//!this extension is advertised?**RESOLVED**: Add it as a distinct feature, but make support
//! mandatory.
//!Adding it as a feature makes the extension easier to include in a future
//!core API version.
//!In the extension, the feature is mandatory, so that presence of the
//!extension guarantees some functionality.
//!When included in a core API version, the feature would be optional.2) Can support for these
//! capabilities vary between shader stages?**RESOLVED**: No, if the capability is supported in any
//! stage it must be
//!supported in all stages.3) Should the capabilities be features or limits?**RESOLVED**: Features,
//! primarily for consistency with other similar
//!extensions.
//!# Version History
//! - Revision 1, 2017-03-14 (Jesse Hall and John Kessenich)
//! - Internal revisions
//!# Other info
//! * 2017-09-05
//! * No known IP claims.
//!*
//! - This extension requires
//![`SPV_KHR_variable_pointers`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_variable_pointers.html)
//! - Promoted to Vulkan 1.1 Core
//!
//!*
//! - John Kessenich, Google
//! - Neil Henning, Codeplay
//! - David Neto, Google
//! - Daniel Koch, Nvidia
//! - Graeme Leese, Broadcom
//! - Weifeng Zhang, Qualcomm
//! - Stephen Clarke, Imagination Technologies
//! - Jason Ekstrand, Intel
//! - Jesse Hall, Google
//!# Related
//! - [`PhysicalDeviceVariablePointerFeaturesKHR`]
//! - [`PhysicalDeviceVariablePointersFeaturesKHR`]
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
#[doc(alias = "VK_KHR_VARIABLE_POINTERS_SPEC_VERSION")]
pub const KHR_VARIABLE_POINTERS_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_VARIABLE_POINTERS_EXTENSION_NAME")]
pub const KHR_VARIABLE_POINTERS_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_variable_pointers");
