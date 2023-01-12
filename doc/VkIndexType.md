[VkIndexType](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndexType.html) - Type of index buffer indices

# C Specifications
Possible values of [`cmd_bind_index_buffer`]`::indexType`, specifying
the size of indices, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkIndexType {
    VK_INDEX_TYPE_UINT16 = 0,
    VK_INDEX_TYPE_UINT32 = 1,
  // Provided by VK_KHR_acceleration_structure
    VK_INDEX_TYPE_NONE_KHR = 1000165000,
  // Provided by VK_EXT_index_type_uint8
    VK_INDEX_TYPE_UINT8_EXT = 1000265000,
  // Provided by VK_NV_ray_tracing
    VK_INDEX_TYPE_NONE_NV = VK_INDEX_TYPE_NONE_KHR,
} VkIndexType;
```

# Description
- [`VK_INDEX_TYPE`] specifies that indices are 16-bit unsigned integer values.
- [`VK_INDEX_TYPE`] specifies that indices are 32-bit unsigned integer values.
- [`NONE_KHR`] specifies that no indices are provided.
- [`UINT8_EXT`] specifies that indices are 8-bit unsigned integer values.

# Related
- [`crate::vulkan1_0`]
- [`AccelerationStructureGeometryTrianglesDataKHR`]
- [`BindIndexBufferIndirectCommandNV`]
- [`GeometryTrianglesNV`]
- [`IndirectCommandsLayoutTokenNV`]
- [`cmd_bind_index_buffer`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        