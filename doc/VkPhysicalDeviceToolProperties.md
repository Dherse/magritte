[VkPhysicalDeviceToolProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceToolProperties.html) - Structure providing information about an active tool

# C Specifications
The [`PhysicalDeviceToolProperties`] structure is defined as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkPhysicalDeviceToolProperties {
    VkStructureType       sType;
    void*                 pNext;
    char                  name[VK_MAX_EXTENSION_NAME_SIZE];
    char                  version[VK_MAX_EXTENSION_NAME_SIZE];
    VkToolPurposeFlags    purposes;
    char                  description[VK_MAX_DESCRIPTION_SIZE];
    char                  layer[VK_MAX_EXTENSION_NAME_SIZE];
} VkPhysicalDeviceToolProperties;
```
or the equivalent
```c
// Provided by VK_EXT_tooling_info
typedef VkPhysicalDeviceToolProperties VkPhysicalDeviceToolPropertiesEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`name`] is a null-terminated UTF-8 string containing the name of the tool.
- [`version`] is a null-terminated UTF-8 string containing the version of the tool.
- [`purposes`] is a bitmask of [`ToolPurposeFlagBits`] which is populated with purposes supported by the tool.
- [`description`] is a null-terminated UTF-8 string containing a description of the tool.
- [`layer`] is a null-terminated UTF-8 string containing the name of the layer implementing the tool, if the tool is implemented in a layer - otherwise it  **may**  be an empty string.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TOOL_PROPERTIES`
-  [`p_next`] **must**  be `NULL`

# Related
- [`ext_tooling_info`]
- [`crate::vulkan1_3`]
- [`StructureType`]
- [VkToolPurposeFlags]()
- [`get_physical_device_tool_properties`]
- [`get_physical_device_tool_properties_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        