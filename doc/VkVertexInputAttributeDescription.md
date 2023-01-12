[VkVertexInputAttributeDescription](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVertexInputAttributeDescription.html) - Structure specifying vertex input attribute description

# C Specifications
Each vertex input attribute is specified by the
[`VertexInputAttributeDescription`] structure, defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkVertexInputAttributeDescription {
    uint32_t    location;
    uint32_t    binding;
    VkFormat    format;
    uint32_t    offset;
} VkVertexInputAttributeDescription;
```

# Members
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
-    If the `[`khr_portability_subset`]` extension is enabled, and [`PhysicalDevicePortabilitySubsetFeaturesKHR::vertex_attribute_access_beyond_stride`] is `VK_FALSE`, the sum of [`offset`] plus the size of the vertex attribute data described by [`format`] **must**  not be greater than `stride` in the [`VertexInputBindingDescription`] referenced in [`binding`]

## Valid Usage (Implicit)
-  [`format`] **must**  be a valid [`Format`] value

# Related
- [`crate::vulkan1_0`]
- [`Format`]
- [`PipelineVertexInputStateCreateInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        