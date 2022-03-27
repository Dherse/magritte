use crate::vulkan1_0::ImageFormatProperties;
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION")]
pub const NV_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME")]
pub const NV_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_NV_external_memory_capabilities");
///[VkExternalImageFormatPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalImageFormatPropertiesNV.html) - Structure specifying external image format properties
///# C Specifications
///The [`ExternalImageFormatPropertiesNV`] structure is defined as:
///```c
///// Provided by VK_NV_external_memory_capabilities
///typedef struct VkExternalImageFormatPropertiesNV {
///    VkImageFormatProperties              imageFormatProperties;
///    VkExternalMemoryFeatureFlagsNV       externalMemoryFeatures;
///    VkExternalMemoryHandleTypeFlagsNV    exportFromImportedHandleTypes;
///    VkExternalMemoryHandleTypeFlagsNV    compatibleHandleTypes;
///} VkExternalImageFormatPropertiesNV;
///```
///# Members
/// - [`image_format_properties`] will be filled in as when calling
///   [`GetPhysicalDeviceImageFormatProperties`], but the values returned **may** vary depending on
///   the external handle type requested.
/// - [`external_memory_features`] is a bitmask of [`ExternalMemoryFeatureFlagBitsNV`], indicating
///   properties of the external memory handle type
///   ([`GetPhysicalDeviceExternalImageFormatPropertiesNV`]`::externalHandleType`) being queried, or
///   0 if the external memory handle type is 0.
/// - [`export_from_imported_handle_types`] is a bitmask of [`ExternalMemoryHandleTypeFlagBitsNV`]
///   containing a bit set for every external handle type that **may** be used to create memory from
///   which the handles of the type specified in
///   [`GetPhysicalDeviceExternalImageFormatPropertiesNV`]`::externalHandleType`**can** be exported,
///   or 0 if the external memory handle type is 0.
/// - [`compatible_handle_types`] is a bitmask of [`ExternalMemoryHandleTypeFlagBitsNV`] containing
///   a bit set for every external handle type that **may** be specified simultaneously with the
///   handle type specified by
///   [`GetPhysicalDeviceExternalImageFormatPropertiesNV`]`::externalHandleType` when calling
///   [`AllocateMemory`], or 0 if the external memory handle type is 0. [`compatible_handle_types`]
///   will always contain [`GetPhysicalDeviceExternalImageFormatPropertiesNV`]`::externalHandleType`
///# Related
/// - [`VK_NV_external_memory_capabilities`]
/// - [`ExternalMemoryFeatureFlagsNV`]
/// - [`ExternalMemoryHandleTypeFlagsNV`]
/// - [`ImageFormatProperties`]
/// - [`GetPhysicalDeviceExternalImageFormatPropertiesNV`]
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
pub struct ExternalImageFormatPropertiesNV {
    ///[`image_format_properties`] will be filled in as when calling
    ///[`GetPhysicalDeviceImageFormatProperties`], but the values returned
    ///**may** vary depending on the external handle type requested.
    image_format_properties: ImageFormatProperties,
    ///[`external_memory_features`] is a bitmask of
    ///[`ExternalMemoryFeatureFlagBitsNV`], indicating properties of the
    ///external memory handle type
    ///([`GetPhysicalDeviceExternalImageFormatPropertiesNV`]::`externalHandleType`)
    ///being queried, or 0 if the external memory handle type is 0.
    external_memory_features: ExternalMemoryFeatureFlagsNV,
    ///[`export_from_imported_handle_types`] is a bitmask of
    ///[`ExternalMemoryHandleTypeFlagBitsNV`] containing a bit set for
    ///every external handle type that **may** be used to create memory from which
    ///the handles of the type specified in
    ///[`GetPhysicalDeviceExternalImageFormatPropertiesNV`]::`externalHandleType`**can** be exported, or 0 if the external memory handle type is 0.
    export_from_imported_handle_types: ExternalMemoryHandleTypeFlagsNV,
    ///[`compatible_handle_types`] is a bitmask of
    ///[`ExternalMemoryHandleTypeFlagBitsNV`] containing a bit set for
    ///every external handle type that **may** be specified simultaneously with
    ///the handle type specified by
    ///[`GetPhysicalDeviceExternalImageFormatPropertiesNV`]::`externalHandleType`
    ///when calling [`AllocateMemory`], or 0 if the external memory handle
    ///type is 0.
    ///[`compatible_handle_types`] will always contain
    ///[`GetPhysicalDeviceExternalImageFormatPropertiesNV`]::`externalHandleType`
    compatible_handle_types: ExternalMemoryHandleTypeFlagsNV,
}
