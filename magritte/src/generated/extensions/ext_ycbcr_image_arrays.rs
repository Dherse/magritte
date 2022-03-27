use crate::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_YCBCR_IMAGE_ARRAYS_SPEC_VERSION")]
pub const EXT_YCBCR_IMAGE_ARRAYS_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_YCBCR_IMAGE_ARRAYS_EXTENSION_NAME")]
pub const EXT_YCBCR_IMAGE_ARRAYS_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_ycbcr_image_arrays");
///[VkPhysicalDeviceYcbcrImageArraysFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceYcbcrImageArraysFeaturesEXT.html) - Structure describing extended Y′C<sub>B</sub>C<sub>R</sub> image creation features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceYcbcrImageArraysFeaturesEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_ycbcr_image_arrays
///typedef struct VkPhysicalDeviceYcbcrImageArraysFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           ycbcrImageArrays;
///} VkPhysicalDeviceYcbcrImageArraysFeaturesEXT;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`ycbcr_image_arrays`] indicates that the implementation supports creating images with a format that requires [Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion) and has multiple array layers.
///If the [`PhysicalDeviceYcbcrImageArraysFeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceYcbcrImageArraysFeaturesEXT`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT`
///# Related
/// - [`VK_EXT_ycbcr_image_arrays`]
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
pub struct PhysicalDeviceYcbcrImageArraysFeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`ycbcr_image_arrays`] indicates that the
    ///implementation supports creating images with a format that requires
    ///[Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion) and
    ///has multiple array layers.
    ycbcr_image_arrays: Bool32,
}
