[VkVertexInputAttributeDescription2EXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVertexInputAttributeDescription2EXT.html) - Structure specifying the extended vertex input attribute description

# C Specifications
The [`VertexInputAttributeDescription2EXT`] structure is defined as:
```c
// Provided by VK_EXT_vertex_input_dynamic_state
typedef struct VkVertexInputAttributeDescription2EXT {
    VkStructureType    sType;
    void*              pNext;
    uint32_t           location;
    uint32_t           binding;
    VkFormat           format;
    uint32_t           offset;
} VkVertexInputAttributeDescription2EXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`location`] is the shader input location number for this attribute.
- [`binding`] is the binding number which this attribute takes its data from.
- [`format`] is the size and type of the vertex attribute data.
- [`offset`] is a byte offset of this attribute relative to the start of an element in the vertex input binding.

# Description
## Valid Usage
-  [`location`] **must**  be less than [`PhysicalDeviceLimits::max_vertex_input_attributes`]
-  [`binding`] **must**  be less than [`PhysicalDeviceLimits::max_vertex_input_bindings`]
-  [`offset`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_vertex_input_attribute_offset`]
-  [`format`] **must**  be allowed as a vertex buffer format, as specified by the `VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT` flag in [`FormatProperties::buffer_features`] returned by [`get_physical_device_format_properties`]
-    If the `[`VK_KHR_portability_subset`]` extension is enabled, and [`PhysicalDevicePortabilitySubsetFeaturesKHR::vertex_attribute_access_beyond_stride`] is [`FALSE`], the sum of [`offset`] plus the size of the vertex attribute data described by [`format`] **must**  not be greater than `stride` in the [`VertexInputBindingDescription2EXT`] referenced in [`binding`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT`
-  [`format`] **must**  be a valid [`Format`] value

# Related
- [`VK_EXT_vertex_input_dynamic_state`]
- [`Format`]
- [`StructureType`]
- [`cmd_set_vertex_input_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        