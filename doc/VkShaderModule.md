[VkShaderModule](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderModule.html) - Opaque handle to a shader module object

# C Specifications
*Shader modules* contain *shader code* and one or more entry points.
Shaders are selected from a shader module by specifying an entry point as
part of [pipeline](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines) creation.
The stages of a pipeline  **can**  use shaders that come from different modules.
The shader code defining a shader module  **must**  be in the SPIR-V format, as
described by the [Vulkan Environment for SPIR-V](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#spirvenv) appendix.Shader modules are represented by [`ShaderModule`] handles:
```c
// Provided by VK_VERSION_1_0
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkShaderModule)
```

# Related
- [`crate::vulkan1_0`]
- [`PipelineShaderStageCreateInfo`]
- [`create_shader_module`]
- [`destroy_shader_module`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        