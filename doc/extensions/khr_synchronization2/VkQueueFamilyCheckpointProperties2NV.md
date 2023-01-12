[VkQueueFamilyCheckpointProperties2NV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyCheckpointProperties2NV.html) - Return structure for queue family checkpoint information query

# C Specifications
The [`QueueFamilyCheckpointProperties2NV`] structure is defined as:
```c
// Provided by VK_KHR_synchronization2 with VK_NV_device_diagnostic_checkpoints
typedef struct VkQueueFamilyCheckpointProperties2NV {
    VkStructureType          sType;
    void*                    pNext;
    VkPipelineStageFlags2    checkpointExecutionStageMask;
} VkQueueFamilyCheckpointProperties2NV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`checkpoint_execution_stage_mask`] is a mask indicating which pipeline stages the implementation can execute checkpoint markers in.

# Description
Additional queue family information can be queried by setting
[`QueueFamilyProperties2`]::[`p_next`] to point to a
[`QueueFamilyCheckpointProperties2NV`] structure.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV`

# Related
- [`khr_synchronization2`]
- [`nv_device_diagnostic_checkpoints`]
- [`PipelineStageFlags2`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        