use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_DEVICE_DIAGNOSTICS_CONFIG_SPEC_VERSION")]
pub const NV_DEVICE_DIAGNOSTICS_CONFIG_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_DEVICE_DIAGNOSTICS_CONFIG_EXTENSION_NAME")]
pub const NV_DEVICE_DIAGNOSTICS_CONFIG_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_device_diagnostics_config");
///[VkPhysicalDeviceDiagnosticsConfigFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDiagnosticsConfigFeaturesNV.html) - Structure describing the device-generated diagnostic configuration features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceDiagnosticsConfigFeaturesNV`] structure is defined
///as:
///```c
///// Provided by VK_NV_device_diagnostics_config
///typedef struct VkPhysicalDeviceDiagnosticsConfigFeaturesNV {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           diagnosticsConfig;
///} VkPhysicalDeviceDiagnosticsConfigFeaturesNV;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`diagnostics_config`] indicates whether the implementation supports the ability to configure
///   diagnostic tools.
///If the [`PhysicalDeviceDiagnosticsConfigFeaturesNV`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceDiagnosticsConfigFeaturesNV`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV`
///# Related
/// - [`VK_NV_device_diagnostics_config`]
/// - [`Bool32`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceDiagnosticsConfigFeaturesNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`diagnostics_config`] indicates whether
    ///the implementation supports the ability to configure diagnostic tools.
    diagnostics_config: Bool32,
}
///[VkDeviceDiagnosticsConfigCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceDiagnosticsConfigCreateInfoNV.html) - Specify diagnostics config for a Vulkan device
///# C Specifications
///When using the Nsight<sup>™</sup> Aftermath SDK, to configure how device crash
///dumps are created, add a [`DeviceDiagnosticsConfigCreateInfoNV`]
///structure to the [`p_next`] chain of the [`DeviceCreateInfo`]
///structure.
///```c
///// Provided by VK_NV_device_diagnostics_config
///typedef struct VkDeviceDiagnosticsConfigCreateInfoNV {
///    VkStructureType                     sType;
///    const void*                         pNext;
///    VkDeviceDiagnosticsConfigFlagsNV    flags;
///} VkDeviceDiagnosticsConfigCreateInfoNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is a bitmask of [`DeviceDiagnosticsConfigFlagBitsNV`] specifying addtional
///   parameters for configuring diagnostic tools.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV`
/// - [`flags`]**must** be a valid combination of [`DeviceDiagnosticsConfigFlagBitsNV`] values
///# Related
/// - [`VK_NV_device_diagnostics_config`]
/// - [`DeviceDiagnosticsConfigFlagsNV`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DeviceDiagnosticsConfigCreateInfoNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`DeviceDiagnosticsConfigFlagBitsNV`]
    ///specifying addtional parameters for configuring diagnostic tools.
    flags: DeviceDiagnosticsConfigFlagsNV,
}
