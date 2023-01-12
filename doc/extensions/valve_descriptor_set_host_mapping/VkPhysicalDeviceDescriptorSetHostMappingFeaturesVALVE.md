[VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE.html) - Stub description of VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE

# C Specifications
There is currently no specification language written for this type.
This section acts only as placeholder and to avoid dead links in the
specification and reference pages.
```c
// Provided by VK_VALVE_descriptor_set_host_mapping
typedef struct VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           descriptorSetHostMapping;
} VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE;
```

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE`

# Related
- [`valve_descriptor_set_host_mapping`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        