[VK_SUBPASS_EXTERNAL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_SUBPASS_EXTERNAL.html) - Subpass index sentinel expanding synchronization scope outside a subpass

# C Specifications
[`SUBPASS_EXTERNAL`] is a special subpass index value expanding
synchronization scope outside a subpass.
It is described in more detail by [`SubpassDependency`].
```c
#define VK_SUBPASS_EXTERNAL               (~0U)
```

# Related
- [`crate::vulkan1_0`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        