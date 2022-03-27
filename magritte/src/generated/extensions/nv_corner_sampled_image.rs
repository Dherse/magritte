use crate::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_CORNER_SAMPLED_IMAGE_SPEC_VERSION")]
pub const NV_CORNER_SAMPLED_IMAGE_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_CORNER_SAMPLED_IMAGE_EXTENSION_NAME")]
pub const NV_CORNER_SAMPLED_IMAGE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_corner_sampled_image");
///[VkPhysicalDeviceCornerSampledImageFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCornerSampledImageFeaturesNV.html) - Structure describing corner sampled image features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceCornerSampledImageFeaturesNV`] structure is defined
///as:
///```c
///// Provided by VK_NV_corner_sampled_image
///typedef struct VkPhysicalDeviceCornerSampledImageFeaturesNV {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           cornerSampledImage;
///} VkPhysicalDeviceCornerSampledImageFeaturesNV;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`corner_sampled_image`] specifies whether images can be created with a [`ImageCreateInfo::flags`] containing `VK_IMAGE_CREATE_CORNER_SAMPLED_BIT_NV`. See [Corner-Sampled Images](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-images-corner-sampled).
///If the [`PhysicalDeviceCornerSampledImageFeaturesNV`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceCornerSampledImageFeaturesNV`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV`
///# Related
/// - [`VK_NV_corner_sampled_image`]
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
pub struct PhysicalDeviceCornerSampledImageFeaturesNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`corner_sampled_image`] specifies
    ///whether images can be created with a
    ///[`ImageCreateInfo`]::`flags` containing
    ///`VK_IMAGE_CREATE_CORNER_SAMPLED_BIT_NV`.
    ///See [Corner-Sampled Images](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-images-corner-sampled).
    corner_sampled_image: Bool32,
}
