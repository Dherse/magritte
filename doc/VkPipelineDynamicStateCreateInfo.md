[VkPipelineDynamicStateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineDynamicStateCreateInfo.html) - Structure specifying parameters of a newly created pipeline dynamic state

# C Specifications
The [`PipelineDynamicStateCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkPipelineDynamicStateCreateInfo {
    VkStructureType                      sType;
    const void*                          pNext;
    VkPipelineDynamicStateCreateFlags    flags;
    uint32_t                             dynamicStateCount;
    const VkDynamicState*                pDynamicStates;
} VkPipelineDynamicStateCreateInfo;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is reserved for future use.
- [`dynamic_state_count`] is the number of elements in the [`dynamic_states`] array.
- [`dynamic_states`] is a pointer to an array of [`DynamicState`] values specifying which pieces of pipeline state will use the values from dynamic state commands rather than from pipeline state creation information.

# Description
## Valid Usage
-    Each element of [`dynamic_states`] **must**  be unique

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO`
-  [`p_next`] **must**  be `NULL`
-  [`flags`] **must**  be `0`
-    If [`dynamic_state_count`] is not `0`, [`dynamic_states`] **must**  be a valid pointer to an array of [`dynamic_state_count`] valid [`DynamicState`] values

# Related
- [`crate::vulkan1_0`]
- [`DynamicState`]
- [`GraphicsPipelineCreateInfo`]
- [`PipelineDynamicStateCreateFlags`]
- [`RayTracingPipelineCreateInfoKHR`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        