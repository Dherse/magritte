[VkPipelineViewportWScalingStateCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportWScalingStateCreateInfoNV.html) - Structure specifying parameters of a newly created pipeline viewport W scaling state

# C Specifications
The [`PipelineViewportWScalingStateCreateInfoNV`] structure is defined
as:
```c
// Provided by VK_NV_clip_space_w_scaling
typedef struct VkPipelineViewportWScalingStateCreateInfoNV {
    VkStructureType                sType;
    const void*                    pNext;
    VkBool32                       viewportWScalingEnable;
    uint32_t                       viewportCount;
    const VkViewportWScalingNV*    pViewportWScalings;
} VkPipelineViewportWScalingStateCreateInfoNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`viewport_w_scaling_enable`] controls whether viewport  **W**  scaling is enabled.
- [`viewport_count`] is the number of viewports used by  **W**  scaling, and  **must**  match the number of viewports in the pipeline if viewport  **W**  scaling is enabled.
- [`viewport_w_scalings`] is a pointer to an array of [`ViewportWScalingNV`] structures defining the  **W**  scaling parameters for the corresponding viewports. If the viewport  **W**  scaling state is dynamic, this member is ignored.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV`
-  [`viewport_count`] **must**  be greater than `0`

# Related
- [`VK_NV_clip_space_w_scaling`]
- [`Bool32`]
- [`StructureType`]
- [`ViewportWScalingNV`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        