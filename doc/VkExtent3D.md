[VkExtent3D](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExtent3D.html) - Structure specifying a three-dimensional extent

# C Specifications
A three-dimensional extent is defined by the structure:
```c
// Provided by VK_VERSION_1_0
typedef struct VkExtent3D {
    uint32_t    width;
    uint32_t    height;
    uint32_t    depth;
} VkExtent3D;
```

# Members
- [`width`] is the width of the extent.
- [`height`] is the height of the extent.
- [`depth`] is the depth of the extent.

# Related
- [`crate::vulkan1_0`]
- [`BufferImageCopy`]
- [`BufferImageCopy2`]
- [`ImageCopy`]
- [`ImageCopy2`]
- [`ImageCreateInfo`]
- [`ImageFormatProperties`]
- [`ImageResolve`]
- [`ImageResolve2`]
- [`QueueFamilyProperties`]
- [`SparseImageFormatProperties`]
- [`SparseImageMemoryBind`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        