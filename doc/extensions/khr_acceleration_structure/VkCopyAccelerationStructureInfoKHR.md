[VkCopyAccelerationStructureInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyAccelerationStructureInfoKHR.html) - Parameters for copying an acceleration structure

# C Specifications
The [`CopyAccelerationStructureInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_acceleration_structure
typedef struct VkCopyAccelerationStructureInfoKHR {
    VkStructureType                       sType;
    const void*                           pNext;
    VkAccelerationStructureKHR            src;
    VkAccelerationStructureKHR            dst;
    VkCopyAccelerationStructureModeKHR    mode;
} VkCopyAccelerationStructureInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`src`] is the source acceleration structure for the copy.
- [`dst`] is the target acceleration structure for the copy.
- [`mode`] is a [`CopyAccelerationStructureModeKHR`] value specifying additional operations to perform during the copy.

# Description
## Valid Usage
-  [`mode`] **must**  be `VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR` or `VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_KHR`
-    The source acceleration structure [`src`] **must**  have been constructed prior to the execution of this command
-    If [`mode`] is `VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR`, [`src`] **must**  have been constructed with `VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR` in the build
-    The `buffer` used to create [`src`] **must**  be bound to device memory
-    The `buffer` used to create [`dst`] **must**  be bound to device memory

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_COPY_ACCELERATION_STRUCTURE_INFO_KHR`
-  [`p_next`] **must**  be `NULL`
-  [`src`] **must**  be a valid [`AccelerationStructureKHR`] handle
-  [`dst`] **must**  be a valid [`AccelerationStructureKHR`] handle
-  [`mode`] **must**  be a valid [`CopyAccelerationStructureModeKHR`] value
-    Both of [`dst`], and [`src`] **must**  have been created, allocated, or retrieved from the same [`Device`]

# Related
- [`VK_KHR_acceleration_structure`]
- [`AccelerationStructureKHR`]
- [`CopyAccelerationStructureModeKHR`]
- [`StructureType`]
- [`cmd_copy_acceleration_structure_khr`]
- [`copy_acceleration_structure_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        