[VkPipelineCompilerControlCreateInfoAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCompilerControlCreateInfoAMD.html) - Structure used to pass compilation control flags to a pipeline

# C Specifications
The compilation of a pipeline  **can**  be tuned by adding a
[`PipelineCompilerControlCreateInfoAMD`] structure to the [`p_next`]
chain of [`GraphicsPipelineCreateInfo`] or
[`ComputePipelineCreateInfo`].
```c
// Provided by VK_AMD_pipeline_compiler_control
typedef struct VkPipelineCompilerControlCreateInfoAMD {
    VkStructureType                      sType;
    const void*                          pNext;
    VkPipelineCompilerControlFlagsAMD    compilerControlFlags;
} VkPipelineCompilerControlCreateInfoAMD;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`compiler_control_flags`] is a bitmask of [`PipelineCompilerControlFlagBitsAMD`] affecting how the pipeline will be compiled.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD`
-  [`compiler_control_flags`] **must**  be `0`

# Related
- [`VK_AMD_pipeline_compiler_control`]
- [`PipelineCompilerControlFlagsAMD`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        