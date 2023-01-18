[VkVertexInputRate](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVertexInputRate.html) - Specify rate at which vertex attributes are pulled from buffers

# C Specifications
Possible values of [`VertexInputBindingDescription::input_rate`],
specifying the rate at which vertex attributes are pulled from buffers, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkVertexInputRate {
    VK_VERTEX_INPUT_RATE_VERTEX = 0,
    VK_VERTEX_INPUT_RATE_INSTANCE = 1,
} VkVertexInputRate;
```

# Description
- [`VERTEX`] specifies that vertex attribute addressing is a function of the vertex index.
- [`INSTANCE`] specifies that vertex attribute addressing is a function of the instance index.

# Related
- [`crate::vulkan1_0`]
- [`VertexInputBindingDescription`]
- [`VertexInputBindingDescription2EXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        