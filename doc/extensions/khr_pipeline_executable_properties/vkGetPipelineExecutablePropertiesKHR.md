[vkGetPipelineExecutablePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutablePropertiesKHR.html) - Get the executables associated with a pipeline

# C Specifications
When a pipeline is created, its state and shaders are compiled into zero or
more device-specific executables, which are used when executing commands
against that pipeline.
To query the properties of these pipeline executables, call:
```c
// Provided by VK_KHR_pipeline_executable_properties
VkResult vkGetPipelineExecutablePropertiesKHR(
    VkDevice                                    device,
    const VkPipelineInfoKHR*                    pPipelineInfo,
    uint32_t*                                   pExecutableCount,
    VkPipelineExecutablePropertiesKHR*          pProperties);
```

# Parameters
- [`device`] is the device that created the pipeline.
- [`p_pipeline_info`] describes the pipeline being queried.
- [`p_executable_count`] is a pointer to an integer related to the number of pipeline executables available or queried, as described below.
- [`p_properties`] is either `NULL` or a pointer to an array of [`PipelineExecutablePropertiesKHR`] structures.

# Description
If [`p_properties`] is `NULL`, then the number of pipeline executables
associated with the pipeline is returned in [`p_executable_count`].
Otherwise, [`p_executable_count`] **must**  point to a variable set by the user
to the number of elements in the [`p_properties`] array, and on return the
variable is overwritten with the number of structures actually written to
[`p_properties`].
If [`p_executable_count`] is less than the number of pipeline executables
associated with the pipeline, at most [`p_executable_count`] structures will
be written, and `VK_INCOMPLETE` will be returned instead of
`VK_SUCCESS`, to indicate that not all the available properties were
returned.
## Valid Usage
-  [`pipelineExecutableInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-pipelineExecutableInfo) **must**  be enabled
-  `pipeline` member of [`p_pipeline_info`] **must**  have been created with [`device`]

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_pipeline_info`] **must**  be a valid pointer to a valid [`PipelineInfoKHR`] structure
-  [`p_executable_count`] **must**  be a valid pointer to a `uint32_t` value
-    If the value referenced by [`p_executable_count`] is not `0`, and [`p_properties`] is not `NULL`, [`p_properties`] **must**  be a valid pointer to an array of [`p_executable_count`][`PipelineExecutablePropertiesKHR`] structures

## Return Codes
*   - `VK_SUCCESS`  - `VK_INCOMPLETE` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`VK_KHR_pipeline_executable_properties`]
- [`Device`]
- [`PipelineExecutablePropertiesKHR`]
- [`PipelineInfoKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        