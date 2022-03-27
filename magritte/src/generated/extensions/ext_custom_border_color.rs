use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, ClearColorValue, Format, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_CUSTOM_BORDER_COLOR_SPEC_VERSION")]
pub const EXT_CUSTOM_BORDER_COLOR_SPEC_VERSION: u32 = 12;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_CUSTOM_BORDER_COLOR_EXTENSION_NAME")]
pub const EXT_CUSTOM_BORDER_COLOR_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_custom_border_color");
///[VkSamplerCustomBorderColorCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerCustomBorderColorCreateInfoEXT.html) - Structure specifying custom border color
///# C Specifications
///In addition to the predefined border color values, applications **can** provide
///a custom border color value by including the
///[`SamplerCustomBorderColorCreateInfoEXT`] structure in the
///[`SamplerCreateInfo`]::[`p_next`] chain.The [`SamplerCustomBorderColorCreateInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_custom_border_color
///typedef struct VkSamplerCustomBorderColorCreateInfoEXT {
///    VkStructureType      sType;
///    const void*          pNext;
///    VkClearColorValue    customBorderColor;
///    VkFormat             format;
///} VkSamplerCustomBorderColorCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`custom_border_color`] is a [`ClearColorValue`] representing the desired custom sampler
///   border color.
/// - [`format`] is a [`Format`] representing the format of the sampled image view(s). This field may be `VK_FORMAT_UNDEFINED` if the [customBorderColorWithoutFormat](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-customBorderColorWithoutFormat) feature is enabled.
///# Description
///Valid Usage
/// -    If provided [`format`] is not `VK_FORMAT_UNDEFINED` then the [`SamplerCreateInfo::border_color`] type **must** match the sampled type of the provided [`format`], as shown in the *SPIR-V Sampled Type* column of the [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-numericformat](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-numericformat) table
/// - If the [customBorderColorWithoutFormat](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-customBorderColorWithoutFormat)
///   feature is not enabled then [`format`]**must** not be `VK_FORMAT_UNDEFINED`
/// - If the sampler is used to sample an image view of `VK_FORMAT_B4G4R4A4_UNORM_PACK16`,
///   `VK_FORMAT_B5G6R5_UNORM_PACK16`, or `VK_FORMAT_B5G5R5A1_UNORM_PACK16` format then
///   [`format`]**must** not be `VK_FORMAT_UNDEFINED`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT`
/// - [`format`]**must** be a valid [`Format`] value
///# Related
/// - [`VK_EXT_custom_border_color`]
/// - [`ClearColorValue`]
/// - [`Format`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct SamplerCustomBorderColorCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`custom_border_color`] is a [`ClearColorValue`] representing the
    ///desired custom sampler border color.
    custom_border_color: ClearColorValue,
    ///[`format`] is a [`Format`] representing the format of the sampled
    ///image view(s).
    ///This field may be `VK_FORMAT_UNDEFINED` if the
    ///[customBorderColorWithoutFormat](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-customBorderColorWithoutFormat)
    ///feature is enabled.
    format: Format,
}
///[VkPhysicalDeviceCustomBorderColorPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCustomBorderColorPropertiesEXT.html) - Structure describing whether custom border colors can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceCustomBorderColorPropertiesEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_custom_border_color
///typedef struct VkPhysicalDeviceCustomBorderColorPropertiesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           maxCustomBorderColorSamplers;
///} VkPhysicalDeviceCustomBorderColorPropertiesEXT;
///```
///# Members
/// - [`max_custom_border_color_samplers`] indicates the maximum number of samplers with custom
///   border colors which **can** simultaneously exist on a device.
///# Description
///If the [`PhysicalDeviceCustomBorderColorPropertiesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT`
///# Related
/// - [`VK_EXT_custom_border_color`]
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
pub struct PhysicalDeviceCustomBorderColorPropertiesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///No documentation found
    s_type: StructureType,
    ///No documentation found
    p_next: *const BaseOutStructure<'lt>,
    ///[`max_custom_border_color_samplers`] indicates the maximum number of
    ///samplers with custom border colors which **can** simultaneously exist on a
    ///device.
    max_custom_border_color_samplers: u32,
}
///[VkPhysicalDeviceCustomBorderColorFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCustomBorderColorFeaturesEXT.html) - Structure describing whether custom border colors can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceCustomBorderColorFeaturesEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_custom_border_color
///typedef struct VkPhysicalDeviceCustomBorderColorFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           customBorderColors;
///    VkBool32           customBorderColorWithoutFormat;
///} VkPhysicalDeviceCustomBorderColorFeaturesEXT;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`custom_border_colors`] indicates that the implementation supports providing a `borderColor`
///   value with one of the following values at sampler creation time:  -
///   `VK_BORDER_COLOR_FLOAT_CUSTOM_EXT`  - `VK_BORDER_COLOR_INT_CUSTOM_EXT`
/// - [`custom_border_color_without_format`] indicates that explicit formats are not required for
///   custom border colors and the value of the `format` member of the
///   [`SamplerCustomBorderColorCreateInfoEXT`] structure **may** be `VK_FORMAT_UNDEFINED`. If this
///   feature bit is not set, applications **must** provide the [`Format`] of the image view(s)
///   being sampled by this sampler in the `format` member of the
///   [`SamplerCustomBorderColorCreateInfoEXT`] structure.
///If the [`PhysicalDeviceCustomBorderColorFeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceCustomBorderColorFeaturesEXT`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT`
///# Related
/// - [`VK_EXT_custom_border_color`]
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
pub struct PhysicalDeviceCustomBorderColorFeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`custom_border_colors`] indicates that
    ///the implementation supports providing a `borderColor` value with one
    ///of the following values at sampler creation time:
    /// - `VK_BORDER_COLOR_FLOAT_CUSTOM_EXT`
    /// - `VK_BORDER_COLOR_INT_CUSTOM_EXT`
    custom_border_colors: Bool32,
    ///[`custom_border_color_without_format`] indicates that explicit formats are
    ///not required for custom border colors and the value of the `format`
    ///member of the [`SamplerCustomBorderColorCreateInfoEXT`] structure
    ///**may** be `VK_FORMAT_UNDEFINED`.
    ///If this feature bit is not set, applications **must** provide the
    ///[`Format`] of the image view(s) being sampled by this sampler in the
    ///`format` member of the [`SamplerCustomBorderColorCreateInfoEXT`]
    ///structure.
    custom_border_color_without_format: Bool32,
}
