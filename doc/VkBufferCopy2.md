[VkBufferCopy2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCopy2.html) - Structure specifying a buffer copy operation

# C Specifications
The [`BufferCopy2`] structure is defined as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkBufferCopy2 {
    VkStructureType    sType;
    const void*        pNext;
    VkDeviceSize       srcOffset;
    VkDeviceSize       dstOffset;
    VkDeviceSize       size;
} VkBufferCopy2;
```
or the equivalent
```c
// Provided by VK_KHR_copy_commands2
typedef VkBufferCopy2 VkBufferCopy2KHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`src_offset`] is the starting offset in bytes from the start of `srcBuffer`.
- [`dst_offset`] is the starting offset in bytes from the start of `dstBuffer`.
- [`size`] is the number of bytes to copy.

# Description
## Valid Usage
-    The [`size`] **must**  be greater than `0`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BUFFER_COPY_2`
-  [`p_next`] **must**  be `NULL`

# Related
- [`VK_KHR_copy_commands2`]
- [`crate::vulkan1_3`]
- [`CopyBufferInfo2`]
- [`DeviceSize`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        