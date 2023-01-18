[VkCheckpointData2NV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCheckpointData2NV.html) - Return structure for command buffer checkpoint data

# C Specifications
The [`CheckpointData2NV`] structure is defined as:
```c
// Provided by VK_KHR_synchronization2 with VK_NV_device_diagnostic_checkpoints
typedef struct VkCheckpointData2NV {
    VkStructureType          sType;
    void*                    pNext;
    VkPipelineStageFlags2    stage;
    void*                    pCheckpointMarker;
} VkCheckpointData2NV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`stage`] indicates a single pipeline stage which the checkpoint marker data refers to.
- [`checkpoint_marker`] contains the value of the last checkpoint marker executed in the stage that [`stage`] refers to.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_CHECKPOINT_DATA_2_NV`
-  [`p_next`] **must**  be `NULL`
The stages at which a checkpoint marker  **can**  be executed are
implementation-defined and  **can**  be queried by calling
[`get_physical_device_queue_family_properties2`].

# Related
- [`VK_KHR_synchronization2`]
- [`VK_NV_device_diagnostic_checkpoints`]
- [`PipelineStageFlags2`]
- [`StructureType`]
- [`get_queue_checkpoint_data2_nv`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        