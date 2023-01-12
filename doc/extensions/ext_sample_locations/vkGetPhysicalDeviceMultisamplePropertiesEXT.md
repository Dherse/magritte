[vkGetPhysicalDeviceMultisamplePropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMultisamplePropertiesEXT.html) - Report sample count specific multisampling capabilities of a physical device

# C Specifications
To query additional multisampling capabilities which  **may**  be supported for a
specific sample count, beyond the minimum capabilities described for
[Limits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits) above, call:
```c
// Provided by VK_EXT_sample_locations
void vkGetPhysicalDeviceMultisamplePropertiesEXT(
    VkPhysicalDevice                            physicalDevice,
    VkSampleCountFlagBits                       samples,
    VkMultisamplePropertiesEXT*                 pMultisampleProperties);
```

# Parameters
- [`physical_device`] is the physical device from which to query the additional multisampling capabilities.
- [`samples`] is a [`SampleCountFlagBits`] value specifying the sample count to query capabilities for.
- [`p_multisample_properties`] is a pointer to a [`MultisamplePropertiesEXT`] structure in which information about additional multisampling capabilities specific to the sample count is returned.

# Description
## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`samples`] **must**  be a valid [`SampleCountFlagBits`] value
-  [`p_multisample_properties`] **must**  be a valid pointer to a [`MultisamplePropertiesEXT`] structure

# Related
- [`ext_sample_locations`]
- [`MultisamplePropertiesEXT`]
- [`PhysicalDevice`]
- [`SampleCountFlagBits`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        