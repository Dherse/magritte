[VkDescriptorSetLayoutHostMappingInfoVALVE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutHostMappingInfoVALVE.html) - Stub description of VkDescriptorSetLayoutHostMappingInfoVALVE

# C Specifications
There is currently no specification language written for this type.
This section acts only as placeholder and to avoid dead links in the
specification and reference pages.
```c
// Provided by VK_VALVE_descriptor_set_host_mapping
typedef struct VkDescriptorSetLayoutHostMappingInfoVALVE {
    VkStructureType    sType;
    void*              pNext;
    size_t             descriptorOffset;
    uint32_t           descriptorSize;
} VkDescriptorSetLayoutHostMappingInfoVALVE;
```

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE`
-  [`p_next`] **must**  be `NULL`

# Related
- [`VK_VALVE_descriptor_set_host_mapping`]
- [`StructureType`]
- [`get_descriptor_set_layout_host_mapping_info_valve`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        