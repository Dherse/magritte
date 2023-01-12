[vkGetPipelineExecutableInternalRepresentationsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutableInternalRepresentationsKHR.html) - Get internal representations of the pipeline executable

# C Specifications
Each pipeline executable  **may**  have one or more text or binary internal
representations associated with it which are generated as part of the
compile process.
These  **may**  include the final shader assembly, a binary form of the compiled
shader, or the shader compiler’s internal representation at any number of
intermediate compile steps.
To query the internal representations associated with a pipeline executable,
call:
```c
// Provided by VK_KHR_pipeline_executable_properties
VkResult vkGetPipelineExecutableInternalRepresentationsKHR(
    VkDevice                                    device,
    const VkPipelineExecutableInfoKHR*          pExecutableInfo,
    uint32_t*                                   pInternalRepresentationCount,
    VkPipelineExecutableInternalRepresentationKHR* pInternalRepresentations);
```

# Parameters
- [`device`] is the device that created the pipeline.
- [`p_executable_info`] describes the pipeline executable being queried.
- [`p_internal_representation_count`] is a pointer to an integer related to the number of internal representations available or queried, as described below.
- [`p_internal_representations`] is either `NULL` or a pointer to an array of [`PipelineExecutableInternalRepresentationKHR`] structures.

# Description
If [`p_internal_representations`] is `NULL`, then the number of internal
representations associated with the pipeline executable is returned in
[`p_internal_representation_count`].
Otherwise, [`p_internal_representation_count`] **must**  point to a variable set
by the user to the number of elements in the [`p_internal_representations`]
array, and on return the variable is overwritten with the number of
structures actually written to [`p_internal_representations`].
If [`p_internal_representation_count`] is less than the number of internal
representations associated with the pipeline executable, at most
[`p_internal_representation_count`] structures will be written, and
`VK_INCOMPLETE` will be returned instead of `VK_SUCCESS`, to
indicate that not all the available representations were returned.While the details of the internal representations remain
implementation-dependent, the implementation  **should**  order the internal
representations in the order in which they occur in the compiled pipeline
with the final shader assembly (if any) last.
## Valid Usage
-  [`pipelineExecutableInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-pipelineExecutableInfo) **must**  be enabled
-  `pipeline` member of [`p_executable_info`] **must**  have been created with [`device`]
-  `pipeline` member of [`p_executable_info`] **must**  have been created with `VK_PIPELINE_CREATE_CAPTURE_INTERNAL_REPRESENTATIONS_BIT_KHR`

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_executable_info`] **must**  be a valid pointer to a valid [`PipelineExecutableInfoKHR`] structure
-  [`p_internal_representation_count`] **must**  be a valid pointer to a `uint32_t` value
-    If the value referenced by [`p_internal_representation_count`] is not `0`, and [`p_internal_representations`] is not `NULL`, [`p_internal_representations`] **must**  be a valid pointer to an array of [`p_internal_representation_count`][`PipelineExecutableInternalRepresentationKHR`] structures

## Return Codes
*   - `VK_SUCCESS`  - `VK_INCOMPLETE` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`khr_pipeline_executable_properties`]
- [`Device`]
- [`PipelineExecutableInfoKHR`]
- [`PipelineExecutableInternalRepresentationKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        