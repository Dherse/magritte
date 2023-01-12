[vkCreateShaderModule](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateShaderModule.html) - Creates a new shader module object

# C Specifications
To create a shader module, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkCreateShaderModule(
    VkDevice                                    device,
    const VkShaderModuleCreateInfo*             pCreateInfo,
    const VkAllocationCallbacks*                pAllocator,
    VkShaderModule*                             pShaderModule);
```

# Parameters
- [`device`] is the logical device that creates the shader module.
- [`p_create_info`] is a pointer to a [`ShaderModuleCreateInfo`] structure.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.
- [`p_shader_module`] is a pointer to a [`ShaderModule`] handle in which the resulting shader module object is returned.

# Description
Once a shader module has been created, any entry points it contains  **can**  be
used in pipeline shader stages as described in [Compute
Pipelines](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-compute) and [Graphics Pipelines](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-graphics).
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_create_info`] **must**  be a valid pointer to a valid [`ShaderModuleCreateInfo`] structure
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-  [`p_shader_module`] **must**  be a valid pointer to a [`ShaderModule`] handle

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_INVALID_SHADER_NV`

# Related
- [`crate::vulkan1_0`]
- [`AllocationCallbacks`]
- [`Device`]
- [`ShaderModule`]
- [`ShaderModuleCreateInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        