use crate::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_RGBA10X6_FORMATS_SPEC_VERSION")]
pub const EXT_RGBA10X6_FORMATS_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_RGBA10X6_FORMATS_EXTENSION_NAME")]
pub const EXT_RGBA10X6_FORMATS_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_rgba10x6_formats");
///[VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT.html) - Structure describing whether rendering to VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16 formats can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceRgba10X6FormatsFeaturesEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_rgba10x6_formats
///typedef struct VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           formatRgba10x6WithoutYCbCrSampler;
///} VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT;
///```
///# Members
///The members of the [`PhysicalDeviceRgba10X6FormatsFeaturesEXT`]
///structure describe the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`format_rgba_10_x_6_without_y_cb_cr_sampler`] indicates that `VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16`**can** be used with a [`ImageView`] with `subresourceRange.aspectMask` equal to `VK_IMAGE_ASPECT_COLOR_BIT` without a [sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-YCbCr-conversion) enabled.
///If the [`PhysicalDeviceRgba10X6FormatsFeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceRgba10X6FormatsFeaturesEXT`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT`
///# Related
/// - [`VK_EXT_rgba10x6_formats`]
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
pub struct PhysicalDeviceRgba10X6FormatsFeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`format_rgba_10_x_6_without_y_cb_cr_sampler`] indicates that
    ///`VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16`**can** be used with a
    ///[`ImageView`] with `subresourceRange.aspectMask` equal to
    ///`VK_IMAGE_ASPECT_COLOR_BIT` without a [sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-YCbCr-conversion) enabled.
    format_rgba_10_x_6_without_y_cb_cr_sampler: Bool32,
}
