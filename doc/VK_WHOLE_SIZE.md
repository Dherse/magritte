[VK_WHOLE_SIZE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_WHOLE_SIZE.html) - Sentinel value to use entire remaining array length

# C Specifications
[`WHOLE_SIZE`] is a special value indicating that the entire remaining
length of a buffer following a given `offset` should be used.
It  **can**  be specified for [`BufferMemoryBarrier::size`] and other
structures.
```c
#define VK_WHOLE_SIZE                     (~0ULL)
```

# Related
- [`crate::vulkan1_0`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        