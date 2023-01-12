[VkSpecializationInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSpecializationInfo.html) - Structure specifying specialization information

# C Specifications
The [`SpecializationInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkSpecializationInfo {
    uint32_t                           mapEntryCount;
    const VkSpecializationMapEntry*    pMapEntries;
    size_t                             dataSize;
    const void*                        pData;
} VkSpecializationInfo;
```

# Members
- [`map_entry_count`] is the number of entries in the [`map_entries`] array.
- [`map_entries`] is a pointer to an array of [`SpecializationMapEntry`] structures which map constant IDs to offsets in [`data`].
- [`data_size`] is the byte size of the [`data`] buffer.
- [`data`] contains the actual constant values to specialize with.

# Description
## Valid Usage
-    The `offset` member of each element of [`map_entries`] **must**  be less than [`data_size`]
-    The `size` member of each element of [`map_entries`] **must**  be less than or equal to [`data_size`] minus `offset`
-    The `constantID` value of each element of [`map_entries`] **must**  be unique within [`map_entries`]

## Valid Usage (Implicit)
-    If [`map_entry_count`] is not `0`, [`map_entries`] **must**  be a valid pointer to an array of [`map_entry_count`] valid [`SpecializationMapEntry`] structures
-    If [`data_size`] is not `0`, [`data`] **must**  be a valid pointer to an array of [`data_size`] bytes

# Related
- [`crate::vulkan1_0`]
- [`PipelineShaderStageCreateInfo`]
- [`SpecializationMapEntry`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        