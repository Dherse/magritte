[VkPhysicalDeviceShaderAtomicFloatFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderAtomicFloatFeaturesEXT.html) - Structure describing features supported by VK_EXT_shader_atomic_float

# C Specifications
The [`PhysicalDeviceShaderAtomicFloatFeaturesEXT`] structure is defined
as:
```c
// Provided by VK_EXT_shader_atomic_float
typedef struct VkPhysicalDeviceShaderAtomicFloatFeaturesEXT {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           shaderBufferFloat32Atomics;
    VkBool32           shaderBufferFloat32AtomicAdd;
    VkBool32           shaderBufferFloat64Atomics;
    VkBool32           shaderBufferFloat64AtomicAdd;
    VkBool32           shaderSharedFloat32Atomics;
    VkBool32           shaderSharedFloat32AtomicAdd;
    VkBool32           shaderSharedFloat64Atomics;
    VkBool32           shaderSharedFloat64AtomicAdd;
    VkBool32           shaderImageFloat32Atomics;
    VkBool32           shaderImageFloat32AtomicAdd;
    VkBool32           sparseImageFloat32Atomics;
    VkBool32           sparseImageFloat32AtomicAdd;
} VkPhysicalDeviceShaderAtomicFloatFeaturesEXT;
```

# Members
This structure describes the following features:

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.

- [`shader_buffer_float32_atomics`] indicates whether shaders  **can**  perform 32-bit floating-point load, store and exchange atomic operations on storage buffers.
- [`shader_buffer_float32_atomic_add`] indicates whether shaders  **can**  perform 32-bit floating-point add atomic operations on storage buffers.
- [`shader_buffer_float64_atomics`] indicates whether shaders  **can**  perform 64-bit floating-point load, store and exchange atomic operations on storage buffers.
- [`shader_buffer_float64_atomic_add`] indicates whether shaders  **can**  perform 64-bit floating-point add atomic operations on storage buffers.
- [`shader_shared_float32_atomics`] indicates whether shaders  **can**  perform 32-bit floating-point load, store and exchange atomic operations on shared memory.
- [`shader_shared_float32_atomic_add`] indicates whether shaders  **can**  perform 32-bit floating-point add atomic operations on shared memory.
- [`shader_shared_float64_atomics`] indicates whether shaders  **can**  perform 64-bit floating-point load, store and exchange atomic operations on shared memory.
- [`shader_shared_float64_atomic_add`] indicates whether shaders  **can**  perform 64-bit floating-point add atomic operations on shared memory.
- [`shader_image_float32_atomics`] indicates whether shaders  **can**  perform 32-bit floating-point load, store and exchange atomic image operations.
- [`shader_image_float32_atomic_add`] indicates whether shaders  **can**  perform 32-bit floating-point add atomic image operations.
- [`sparse_image_float32_atomics`] indicates whether 32-bit floating-point load, store and exchange atomic operations  **can**  be used on sparse images.
- [`sparse_image_float32_atomic_add`] indicates whether 32-bit floating-point add atomic operations  **can**  be used on sparse images.
If the [`PhysicalDeviceShaderAtomicFloatFeaturesEXT`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDeviceShaderAtomicFloatFeaturesEXT`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT`

# Related
- [`VK_EXT_shader_atomic_float`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        