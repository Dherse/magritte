[VkAccelerationStructureVersionInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureVersionInfoKHR.html) - Acceleration structure version information

# C Specifications
The [`AccelerationStructureVersionInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_acceleration_structure
typedef struct VkAccelerationStructureVersionInfoKHR {
    VkStructureType    sType;
    const void*        pNext;
    const uint8_t*     pVersionData;
} VkAccelerationStructureVersionInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`version_data`] is a pointer to the version header of an acceleration structure as defined in [`cmd_copy_acceleration_structure_to_memory_khr`]

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_VERSION_INFO_KHR`
-  [`p_next`] **must**  be `NULL`
-  [`version_data`] **must**  be a valid pointer to an array of <span class="katex"><span aria-hidden="true" class="katex-html"><span class="base"><span style="height:0.72777em;vertical-align:-0.08333em;" class="strut"></span><span class="mord">2</span><span style="margin-right:0.2222222222222222em;" class="mspace"></span><span class="mbin">×</span><span style="margin-right:0.2222222222222222em;" class="mspace"></span></span><span class="base"><span style="height:0.70625em;vertical-align:-0.09514em;" class="strut"></span><span class="mord"><span class="mord mathtt">V</span><span class="mord mathtt">K</span><span class="mord mathtt">_</span><span class="mord mathtt">U</span><span class="mord mathtt">U</span><span class="mord mathtt">I</span><span class="mord mathtt">D</span><span class="mord mathtt">_</span><span class="mord mathtt">S</span><span class="mord mathtt">I</span><span class="mord mathtt">Z</span><span class="mord mathtt">E</span></span></span></span></span>`uint8_t` values

# Related
- [`VK_KHR_acceleration_structure`]
- [`StructureType`]
- [`get_device_acceleration_structure_compatibility_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        