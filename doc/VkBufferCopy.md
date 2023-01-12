[VkBufferCopy](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCopy.html) - Structure specifying a buffer copy operation

# C Specifications
The [`BufferCopy`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkBufferCopy {
    VkDeviceSize    srcOffset;
    VkDeviceSize    dstOffset;
    VkDeviceSize    size;
} VkBufferCopy;
```

# Members
- [`src_offset`] is the starting offset in bytes from the start of `srcBuffer`.
- [`dst_offset`] is the starting offset in bytes from the start of `dstBuffer`.
- [`size`] is the number of bytes to copy.

# Description
## Valid Usage
-    The [`size`] **must**  be greater than `0`

# Related
- [`crate::vulkan1_0`]
- [`DeviceSize`]
- [`cmd_copy_buffer`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        