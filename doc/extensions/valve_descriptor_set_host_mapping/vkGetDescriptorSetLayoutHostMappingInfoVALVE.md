[vkGetDescriptorSetLayoutHostMappingInfoVALVE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutHostMappingInfoVALVE.html) - Stub description of vkGetDescriptorSetLayoutHostMappingInfoVALVE

# C Specifications
There is currently no specification language written for this command.
This section acts only as placeholder and to avoid dead links in the
specification and reference pages.
```c
// Provided by VK_VALVE_descriptor_set_host_mapping
void vkGetDescriptorSetLayoutHostMappingInfoVALVE(
    VkDevice                                    device,
    const VkDescriptorSetBindingReferenceVALVE* pBindingReference,
    VkDescriptorSetLayoutHostMappingInfoVALVE*  pHostMapping);
```

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_binding_reference`] **must**  be a valid pointer to a valid [`DescriptorSetBindingReferenceVALVE`] structure
-  [`p_host_mapping`] **must**  be a valid pointer to a [`DescriptorSetLayoutHostMappingInfoVALVE`] structure

# Related
- [`VK_VALVE_descriptor_set_host_mapping`]
- [`DescriptorSetBindingReferenceVALVE`]
- [`DescriptorSetLayoutHostMappingInfoVALVE`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        