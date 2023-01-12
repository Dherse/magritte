[VkVertexInputBindingDescription](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVertexInputBindingDescription.html) - Structure specifying vertex input binding description

# C Specifications
Each vertex input binding is specified by the
[`VertexInputBindingDescription`] structure, defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkVertexInputBindingDescription {
    uint32_t             binding;
    uint32_t             stride;
    VkVertexInputRate    inputRate;
} VkVertexInputBindingDescription;
```

# Members
- [`binding`] is the binding number that this structure describes.
- [`stride`] is the byte stride between consecutive elements within the buffer.
- [`input_rate`] is a [`VertexInputRate`] value specifying whether vertex attribute addressing is a function of the vertex index or of the instance index.

# Description
## Valid Usage
-  [`binding`] **must**  be less than [`PhysicalDeviceLimits::max_vertex_input_bindings`]
-  [`stride`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_vertex_input_binding_stride`]
-    If the `[`khr_portability_subset`]` extension is enabled, [`stride`] **must**  be a multiple of, and at least as large as, [`PhysicalDevicePortabilitySubsetPropertiesKHR::min_vertex_input_binding_stride_alignment`]

## Valid Usage (Implicit)
-  [`input_rate`] **must**  be a valid [`VertexInputRate`] value

# Related
- [`crate::vulkan1_0`]
- [`PipelineVertexInputStateCreateInfo`]
- [`VertexInputRate`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        