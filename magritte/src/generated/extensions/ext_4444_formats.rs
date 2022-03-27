use crate::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_4444_FORMATS_SPEC_VERSION")]
pub const EXT_4444_FORMATS_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_4444_FORMATS_EXTENSION_NAME")]
pub const EXT_4444_FORMATS_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_4444_formats");
///[VkPhysicalDevice4444FormatsFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevice4444FormatsFeaturesEXT.html) - Structure describing additional 4444 formats supported by an implementation
///# C Specifications
///The [`PhysicalDevice4444FormatsFeaturesEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_4444_formats
///typedef struct VkPhysicalDevice4444FormatsFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           formatA4R4G4B4;
///    VkBool32           formatA4B4G4R4;
///} VkPhysicalDevice4444FormatsFeaturesEXT;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`format_a_4_r_4_g_4_b_4`] indicates that the implementation **must** support using a
///   [`Format`] of `VK_FORMAT_A4R4G4B4_UNORM_PACK16_EXT` with at least the following
///   [`FormatFeatureFlagBits`]:  - `VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`  -
///   `VK_FORMAT_FEATURE_BLIT_SRC_BIT`  - `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
/// - [`format_a_4_b_4_g_4_r_4`] indicates that the implementation **must** support using a
///   [`Format`] of `VK_FORMAT_A4B4G4R4_UNORM_PACK16_EXT` with at least the following
///   [`FormatFeatureFlagBits`]:  - `VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`  -
///   `VK_FORMAT_FEATURE_BLIT_SRC_BIT`  - `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
///If the [`PhysicalDevice4444FormatsFeaturesEXT`] structure is included in the [`p_next`] chain of
/// the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDevice4444FormatsFeaturesEXT`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT`
///# Related
/// - [`VK_EXT_4444_formats`]
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
pub struct PhysicalDevice4444FormatsFeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`format_a_4_r_4_g_4_b_4`] indicates that the
    ///implementation **must** support using a [`Format`] of
    ///`VK_FORMAT_A4R4G4B4_UNORM_PACK16_EXT` with at least the following
    ///[`FormatFeatureFlagBits`]:
    /// - `VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`
    /// - `VK_FORMAT_FEATURE_BLIT_SRC_BIT`
    /// - `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
    format_a_4_r_4_g_4_b_4: Bool32,
    ///[`format_a_4_b_4_g_4_r_4`] indicates that the
    ///implementation **must** support using a [`Format`] of
    ///`VK_FORMAT_A4B4G4R4_UNORM_PACK16_EXT` with at least the following
    ///[`FormatFeatureFlagBits`]:
    /// - `VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`
    /// - `VK_FORMAT_FEATURE_BLIT_SRC_BIT`
    /// - `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
    format_a_4_b_4_g_4_r_4: Bool32,
}
