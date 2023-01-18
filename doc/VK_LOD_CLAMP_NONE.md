[VK_LOD_CLAMP_NONE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_LOD_CLAMP_NONE.html) - Maximum level of detail unclamped access sentinel

# C Specifications
[`LOD_CLAMP_NONE`] is a special constant value used for
[`SamplerCreateInfo::max_lod`] to indicate that maximum LOD
clamping should not be performed.
```c
#define VK_LOD_CLAMP_NONE                 1000.0F
```

# Related
- [`crate::vulkan1_0`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        