use crate::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_SHADER_IMAGE_ATOMIC_INT64_SPEC_VERSION")]
pub const EXT_SHADER_IMAGE_ATOMIC_INT64_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_SHADER_IMAGE_ATOMIC_INT64_EXTENSION_NAME")]
pub const EXT_SHADER_IMAGE_ATOMIC_INT64_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_EXT_shader_image_atomic_int64");
///[VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT.html) - Structure describing features supported by VK_EXT_shader_image_atomic_int64
///# C Specifications
///The [`PhysicalDeviceShaderImageAtomicInt64FeaturesEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_shader_image_atomic_int64
///typedef struct VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           shaderImageInt64Atomics;
///    VkBool32           sparseImageInt64Atomics;
///} VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`shader_image_int_64_atomics`] indicates whether shaders **can** support 64-bit unsigned and
///   signed integer atomic operations on images.
/// - [`sparse_image_int_64_atomics`] indicates whether 64-bit integer atomics **can** be used on
///   sparse images.
///If the `VkPhysicalDeviceShaderAtomicInt64FeaturesEXT` structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///`VkPhysicalDeviceShaderAtomicInt64FeaturesEXT`**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT`
///# Related
/// - [`VK_EXT_shader_image_atomic_int64`]
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
pub struct PhysicalDeviceShaderImageAtomicInt64FeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`shader_image_int_64_atomics`]
    ///indicates whether shaders **can** support 64-bit unsigned and signed
    ///integer atomic operations on images.
    shader_image_int_64_atomics: Bool32,
    ///[`sparse_image_int_64_atomics`]
    ///indicates whether 64-bit integer atomics **can** be used on sparse images.
    sparse_image_int_64_atomics: Bool32,
}
