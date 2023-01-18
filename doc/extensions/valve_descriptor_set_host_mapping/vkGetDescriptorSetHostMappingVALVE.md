[vkGetDescriptorSetHostMappingVALVE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetHostMappingVALVE.html) - Stub description of vkGetDescriptorSetHostMappingVALVE

# C Specifications
There is currently no specification language written for this command.
This section acts only as placeholder and to avoid dead links in the
specification and reference pages.
```c
// Provided by VK_VALVE_descriptor_set_host_mapping
void vkGetDescriptorSetHostMappingVALVE(
    VkDevice                                    device,
    VkDescriptorSet                             descriptorSet,
    void**                                      ppData);
```

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`descriptor_set`] **must**  be a valid [`DescriptorSet`] handle
-  [`pp_data`] **must**  be a valid pointer to a pointer value

# Related
- [`VK_VALVE_descriptor_set_host_mapping`]
- [`DescriptorSet`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        