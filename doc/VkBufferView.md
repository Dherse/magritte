[VkBufferView](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferView.html) - Opaque handle to a buffer view object

# C Specifications
A *buffer view* represents a contiguous range of a buffer and a specific
format to be used to interpret the data.
Buffer views are used to enable shaders to access buffer contents
interpreted as formatted data.
In order to create a valid buffer view, the buffer  **must**  have been created
with at least one of the following usage flags:
- `VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT`
- `VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT`
Buffer views are represented by [`BufferView`] handles:
```c
// Provided by VK_VERSION_1_0
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkBufferView)
```

# Related
- [`crate::vulkan1_0`]
- [`WriteDescriptorSet`]
- [`create_buffer_view`]
- [`destroy_buffer_view`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        