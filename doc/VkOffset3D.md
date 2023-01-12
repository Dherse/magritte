[VkOffset3D](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkOffset3D.html) - Structure specifying a three-dimensional offset

# C Specifications
A three-dimensional offset is defined by the structure:
```c
// Provided by VK_VERSION_1_0
typedef struct VkOffset3D {
    int32_t    x;
    int32_t    y;
    int32_t    z;
} VkOffset3D;
```

# Members
- [`x`] is the x offset.
- [`y`] is the y offset.
- [`z`] is the z offset.

# Related
- [`crate::vulkan1_0`]
- [`BufferImageCopy`]
- [`BufferImageCopy2`]
- [`ImageBlit`]
- [`ImageBlit2`]
- [`ImageCopy`]
- [`ImageCopy2`]
- [`ImageResolve`]
- [`ImageResolve2`]
- [`SparseImageMemoryBind`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        