[VkDescriptorSetBindingReferenceVALVE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetBindingReferenceVALVE.html) - Stub description of VkDescriptorSetBindingReferenceVALVE

# C Specifications
There is currently no specification language written for this type.
This section acts only as placeholder and to avoid dead links in the
specification and reference pages.
```c
// Provided by VK_VALVE_descriptor_set_host_mapping
typedef struct VkDescriptorSetBindingReferenceVALVE {
    VkStructureType          sType;
    const void*              pNext;
    VkDescriptorSetLayout    descriptorSetLayout;
    uint32_t                 binding;
} VkDescriptorSetBindingReferenceVALVE;
```

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_BINDING_REFERENCE_VALVE`
-  [`p_next`] **must**  be `NULL`
-  [`descriptor_set_layout`] **must**  be a valid [`DescriptorSetLayout`] handle

# Related
- [`VK_VALVE_descriptor_set_host_mapping`]
- [`DescriptorSetLayout`]
- [`StructureType`]
- [`get_descriptor_set_layout_host_mapping_info_valve`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        