use crate::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_SHADER_ATOMIC_FLOAT_SPEC_VERSION")]
pub const EXT_SHADER_ATOMIC_FLOAT_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_SHADER_ATOMIC_FLOAT_EXTENSION_NAME")]
pub const EXT_SHADER_ATOMIC_FLOAT_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_shader_atomic_float");
///[VkPhysicalDeviceShaderAtomicFloatFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderAtomicFloatFeaturesEXT.html) - Structure describing features supported by VK_EXT_shader_atomic_float
///# C Specifications
///The [`PhysicalDeviceShaderAtomicFloatFeaturesEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_shader_atomic_float
///typedef struct VkPhysicalDeviceShaderAtomicFloatFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           shaderBufferFloat32Atomics;
///    VkBool32           shaderBufferFloat32AtomicAdd;
///    VkBool32           shaderBufferFloat64Atomics;
///    VkBool32           shaderBufferFloat64AtomicAdd;
///    VkBool32           shaderSharedFloat32Atomics;
///    VkBool32           shaderSharedFloat32AtomicAdd;
///    VkBool32           shaderSharedFloat64Atomics;
///    VkBool32           shaderSharedFloat64AtomicAdd;
///    VkBool32           shaderImageFloat32Atomics;
///    VkBool32           shaderImageFloat32AtomicAdd;
///    VkBool32           sparseImageFloat32Atomics;
///    VkBool32           sparseImageFloat32AtomicAdd;
///} VkPhysicalDeviceShaderAtomicFloatFeaturesEXT;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///
/// - [`shader_buffer_float_32_atomics`] indicates whether shaders **can** perform 32-bit
///   floating-point load, store and exchange atomic operations on storage buffers.
/// - [`shader_buffer_float_32_atomic_add`] indicates whether shaders **can** perform 32-bit
///   floating-point add atomic operations on storage buffers.
/// - [`shader_buffer_float_64_atomics`] indicates whether shaders **can** perform 64-bit
///   floating-point load, store and exchange atomic operations on storage buffers.
/// - [`shader_buffer_float_64_atomic_add`] indicates whether shaders **can** perform 64-bit
///   floating-point add atomic operations on storage buffers.
/// - [`shader_shared_float_32_atomics`] indicates whether shaders **can** perform 32-bit
///   floating-point load, store and exchange atomic operations on shared memory.
/// - [`shader_shared_float_32_atomic_add`] indicates whether shaders **can** perform 32-bit
///   floating-point add atomic operations on shared memory.
/// - [`shader_shared_float_64_atomics`] indicates whether shaders **can** perform 64-bit
///   floating-point load, store and exchange atomic operations on shared memory.
/// - [`shader_shared_float_64_atomic_add`] indicates whether shaders **can** perform 64-bit
///   floating-point add atomic operations on shared memory.
/// - [`shader_image_float_32_atomics`] indicates whether shaders **can** perform 32-bit
///   floating-point load, store and exchange atomic image operations.
/// - [`shader_image_float_32_atomic_add`] indicates whether shaders **can** perform 32-bit
///   floating-point add atomic image operations.
/// - [`sparse_image_float_32_atomics`] indicates whether 32-bit floating-point load, store and
///   exchange atomic operations **can** be used on sparse images.
/// - [`sparse_image_float_32_atomic_add`] indicates whether 32-bit floating-point add atomic
///   operations **can** be used on sparse images.
///If the [`PhysicalDeviceShaderAtomicFloatFeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceShaderAtomicFloatFeaturesEXT`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT`
///# Related
/// - [`VK_EXT_shader_atomic_float`]
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
pub struct PhysicalDeviceShaderAtomicFloatFeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`shader_buffer_float_32_atomics`]
    ///indicates whether shaders **can** perform 32-bit floating-point load, store
    ///and exchange atomic operations on storage buffers.
    shader_buffer_float_32_atomics: Bool32,
    ///[`shader_buffer_float_32_atomic_add`] indicates whether shaders **can**
    ///perform 32-bit floating-point add atomic operations on storage buffers.
    shader_buffer_float_32_atomic_add: Bool32,
    ///[`shader_buffer_float_64_atomics`]
    ///indicates whether shaders **can** perform 64-bit floating-point load, store
    ///and exchange atomic operations on storage buffers.
    shader_buffer_float_64_atomics: Bool32,
    ///[`shader_buffer_float_64_atomic_add`] indicates whether shaders **can**
    ///perform 64-bit floating-point add atomic operations on storage buffers.
    shader_buffer_float_64_atomic_add: Bool32,
    ///[`shader_shared_float_32_atomics`]
    ///indicates whether shaders **can** perform 32-bit floating-point load, store
    ///and exchange atomic operations on shared memory.
    shader_shared_float_32_atomics: Bool32,
    ///[`shader_shared_float_32_atomic_add`] indicates whether shaders **can**
    ///perform 32-bit floating-point add atomic operations on shared memory.
    shader_shared_float_32_atomic_add: Bool32,
    ///[`shader_shared_float_64_atomics`]
    ///indicates whether shaders **can** perform 64-bit floating-point load, store
    ///and exchange atomic operations on shared memory.
    shader_shared_float_64_atomics: Bool32,
    ///[`shader_shared_float_64_atomic_add`] indicates whether shaders **can**
    ///perform 64-bit floating-point add atomic operations on shared memory.
    shader_shared_float_64_atomic_add: Bool32,
    ///[`shader_image_float_32_atomics`]
    ///indicates whether shaders **can** perform 32-bit floating-point load, store
    ///and exchange atomic image operations.
    shader_image_float_32_atomics: Bool32,
    ///[`shader_image_float_32_atomic_add`] indicates whether shaders **can** perform
    ///32-bit floating-point add atomic image operations.
    shader_image_float_32_atomic_add: Bool32,
    ///[`sparse_image_float_32_atomics`]
    ///indicates whether 32-bit floating-point load, store and exchange atomic
    ///operations **can** be used on sparse images.
    sparse_image_float_32_atomics: Bool32,
    ///[`sparse_image_float_32_atomic_add`] indicates whether 32-bit
    ///floating-point add atomic operations **can** be used on sparse images.
    sparse_image_float_32_atomic_add: Bool32,
}
