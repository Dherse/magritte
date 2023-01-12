[VkMemoryRequirements](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryRequirements.html) - Structure specifying memory requirements

# C Specifications
The [`MemoryRequirements`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkMemoryRequirements {
    VkDeviceSize    size;
    VkDeviceSize    alignment;
    uint32_t        memoryTypeBits;
} VkMemoryRequirements;
```

# Members
- [`size`] is the size, in bytes, of the memory allocation  **required**  for the resource.
- [`alignment`] is the alignment, in bytes, of the offset within the allocation  **required**  for the resource.
- [`memory_type_bits`] is a bitmask and contains one bit set for every supported memory type for the resource. Bit `i` is set if and only if the memory type `i` in the [`PhysicalDeviceMemoryProperties`] structure for the physical device is supported for the resource.

# Related
- [`crate::vulkan1_0`]
- [`DeviceSize`]
- [`MemoryRequirements2`]
- [`get_buffer_memory_requirements`]
- [`get_image_memory_requirements`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        