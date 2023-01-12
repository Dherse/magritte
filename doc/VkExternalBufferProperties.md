[VkExternalBufferProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalBufferProperties.html) - Structure specifying supported external handle capabilities

# C Specifications
The [`ExternalBufferProperties`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkExternalBufferProperties {
    VkStructureType               sType;
    void*                         pNext;
    VkExternalMemoryProperties    externalMemoryProperties;
} VkExternalBufferProperties;
```
or the equivalent
```c
// Provided by VK_KHR_external_memory_capabilities
typedef VkExternalBufferProperties VkExternalBufferPropertiesKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`external_memory_properties`] is a [`ExternalMemoryProperties`] structure specifying various capabilities of the external handle type when used with the specified buffer creation parameters.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES`
-  [`p_next`] **must**  be `NULL`

# Related
- [`crate::vulkan1_1`]
- [`ExternalMemoryProperties`]
- [`StructureType`]
- [`get_physical_device_external_buffer_properties`]
- [`get_physical_device_external_buffer_properties_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        