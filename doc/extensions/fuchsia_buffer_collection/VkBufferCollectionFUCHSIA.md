[VkBufferCollectionFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCollectionFUCHSIA.html) - Opaque handle to a buffer collection object

# C Specifications
Fuchsia’s FIDL-based Sysmem service interoperates with Vulkan via the
`[`fuchsia_buffer_collection`]` extension.A buffer collection is a set of one or more buffers which were allocated
together as a group and which all have the same properties.
These properties describe the buffers' internal representation, such as its
dimensions and memory layout.
This ensures that all of the buffers can be used interchangeably by tasks
that require swapping among multiple buffers, such as double-buffered
graphics rendering.On Fuchsia, the Sysmem service uses buffer collections as a core construct
in its design.Buffer collections are represented by [`BufferCollectionFUCHSIA`]
handles:
```c
// Provided by VK_FUCHSIA_buffer_collection
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkBufferCollectionFUCHSIA)
```

# Related
- [`fuchsia_buffer_collection`]
- [`BufferCollectionBufferCreateInfoFUCHSIA`]
- [`BufferCollectionImageCreateInfoFUCHSIA`]
- [`ImportMemoryBufferCollectionFUCHSIA`]
- [`create_buffer_collection_fuchsia`]
- [`destroy_buffer_collection_fuchsia`]
- [`get_buffer_collection_properties_fuchsia`]
- [`set_buffer_collection_buffer_constraints_fuchsia`]
- [`set_buffer_collection_image_constraints_fuchsia`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        