[VkCalibratedTimestampInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCalibratedTimestampInfoEXT.html) - Structure specifying the input parameters of a calibrated timestamp query

# C Specifications
The [`CalibratedTimestampInfoEXT`] structure is defined as:
```c
// Provided by VK_EXT_calibrated_timestamps
typedef struct VkCalibratedTimestampInfoEXT {
    VkStructureType    sType;
    const void*        pNext;
    VkTimeDomainEXT    timeDomain;
} VkCalibratedTimestampInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`time_domain`] is a [`TimeDomainEXT`] value specifying the time domain from which the calibrated timestamp value should be returned.

# Description
## Valid Usage
-  [`time_domain`] **must**  be one of the [`TimeDomainEXT`] values returned by [`get_physical_device_calibrateable_time_domains_ext`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_CALIBRATED_TIMESTAMP_INFO_EXT`
-  [`p_next`] **must**  be `NULL`
-  [`time_domain`] **must**  be a valid [`TimeDomainEXT`] value

# Related
- [`VK_EXT_calibrated_timestamps`]
- [`StructureType`]
- [`TimeDomainEXT`]
- [`get_calibrated_timestamps_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        