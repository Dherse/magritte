use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, ComponentMapping, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_BORDER_COLOR_SWIZZLE_SPEC_VERSION")]
pub const EXT_BORDER_COLOR_SWIZZLE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_BORDER_COLOR_SWIZZLE_EXTENSION_NAME")]
pub const EXT_BORDER_COLOR_SWIZZLE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_border_color_swizzle");
///[VkSamplerBorderColorComponentMappingCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerBorderColorComponentMappingCreateInfoEXT.html) - Structure specifying the component mapping of the border color
///# C Specifications
///If the sampler is created with `VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK`,
///`VK_BORDER_COLOR_INT_OPAQUE_BLACK`,
///`VK_BORDER_COLOR_FLOAT_CUSTOM_EXT`, or
///`VK_BORDER_COLOR_INT_CUSTOM_EXT``borderColor`, and that sampler
///will be combined with an image view that does not have an
///[identity swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings), and
///[`PhysicalDeviceBorderColorSwizzleFeaturesEXT::border_color_swizzle_from_image`]
///is not enabled, then it is necessary to specify the component mapping of the
///border color, by including the
///[`SamplerBorderColorComponentMappingCreateInfoEXT`] structure in the
///[`SamplerCreateInfo`]::[`p_next`] chain, to get defined results.The [`SamplerBorderColorComponentMappingCreateInfoEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_border_color_swizzle
///typedef struct VkSamplerBorderColorComponentMappingCreateInfoEXT {
///    VkStructureType       sType;
///    const void*           pNext;
///    VkComponentMapping    components;
///    VkBool32              srgb;
///} VkSamplerBorderColorComponentMappingCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`components`] is a [`ComponentMapping`] structure specifying a remapping of the border color
///   components.
/// - [`srgb`] indicates that the sampler will be combined with an image view that has an image
///   format which is sRGB encoded.
///# Description
///The [`ComponentMapping`][`components`] member describes a remapping
///from components of the border color to components of the vector returned by
///shader image instructions when the border color is used.Valid Usage
/// - The [`borderColorSwizzle`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-borderColorSwizzle)
///   feature **must** be enabled.
///Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_SAMPLER_BORDER_COLOR_COMPONENT_MAPPING_CREATE_INFO_EXT`
/// - [`components`]**must** be a valid [`ComponentMapping`] structure
///# Related
/// - [`VK_EXT_border_color_swizzle`]
/// - [`Bool32`]
/// - [`ComponentMapping`]
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
pub struct SamplerBorderColorComponentMappingCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`components`] is a [`ComponentMapping`] structure specifying a
    ///remapping of the border color components.
    components: ComponentMapping,
    ///[`srgb`] indicates that the sampler will be combined with an image
    ///view that has an image format which is sRGB encoded.
    srgb: Bool32,
}
///[VkPhysicalDeviceBorderColorSwizzleFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceBorderColorSwizzleFeaturesEXT.html) - Structure describing whether samplers with custom border colors require the component swizzle specified in order to have defined behavior
///# C Specifications
///The [`PhysicalDeviceBorderColorSwizzleFeaturesEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_border_color_swizzle
///typedef struct VkPhysicalDeviceBorderColorSwizzleFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           borderColorSwizzle;
///    VkBool32           borderColorSwizzleFromImage;
///} VkPhysicalDeviceBorderColorSwizzleFeaturesEXT;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`border_color_swizzle`] indicates that defined values are returned by sampled image operations when used with a sampler that uses a `VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK`, `VK_BORDER_COLOR_INT_OPAQUE_BLACK`, `VK_BORDER_COLOR_FLOAT_CUSTOM_EXT`, or `VK_BORDER_COLOR_INT_CUSTOM_EXT``borderColor` and an image view that uses a non-[identity component mapping](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings), when either [`border_color_swizzle_from_image`] is enabled or the [`SamplerBorderColorComponentMappingCreateInfoEXT`] is specified.
/// - [`border_color_swizzle_from_image`] indicates that the implementation will return the correct border color values from sampled image operations under the conditions expressed above, without the application having to specify the border color component mapping when creating the sampler object. If this feature bit is not set, applications **can** chain a [`SamplerBorderColorComponentMappingCreateInfoEXT`] structure when creating samplers for use with image views that do not have an [identity swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings) and, when those samplers are combined with image views using the same component mapping, sampled image operations that use opaque black or custom border colors will return the correct border color values.
///If the [`PhysicalDeviceBorderColorSwizzleFeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceBorderColorSwizzleFeaturesEXT`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT`
///# Related
/// - [`VK_EXT_border_color_swizzle`]
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
pub struct PhysicalDeviceBorderColorSwizzleFeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`border_color_swizzle`] indicates that
    ///defined values are returned by sampled image operations when used with a
    ///sampler that uses a `VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK`,
    ///`VK_BORDER_COLOR_INT_OPAQUE_BLACK`,
    ///`VK_BORDER_COLOR_FLOAT_CUSTOM_EXT`, or
    ///`VK_BORDER_COLOR_INT_CUSTOM_EXT``borderColor` and an image view
    ///that uses a non-[identity
    ///component mapping](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings), when either [`border_color_swizzle_from_image`] is
    ///enabled or the [`SamplerBorderColorComponentMappingCreateInfoEXT`]
    ///is specified.
    border_color_swizzle: Bool32,
    ///[`border_color_swizzle_from_image`] indicates that the implementation will
    ///return the correct border color values from sampled image operations
    ///under the conditions expressed above, without the application having to
    ///specify the border color component mapping when creating the sampler
    ///object.
    ///If this feature bit is not set, applications **can** chain a
    ///[`SamplerBorderColorComponentMappingCreateInfoEXT`] structure when
    ///creating samplers for use with image views that do not have an
    ///[identity swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings) and, when
    ///those samplers are combined with image views using the same component
    ///mapping, sampled image operations that use opaque black or custom border
    ///colors will return the correct border color values.
    border_color_swizzle_from_image: Bool32,
}
