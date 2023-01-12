[VkDescriptorPoolSize](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorPoolSize.html) - Structure specifying descriptor pool size

# C Specifications
The [`DescriptorPoolSize`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkDescriptorPoolSize {
    VkDescriptorType    type;
    uint32_t            descriptorCount;
} VkDescriptorPoolSize;
```

# Members
- [`type_`] is the type of descriptor.
- [`descriptor_count`] is the number of descriptors of that type to allocate. If [`type_`] is `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` then [`descriptor_count`] is the number of bytes to allocate for descriptors of this type.

# Description
## Valid Usage
-  [`descriptor_count`] **must**  be greater than `0`
-    If [`type_`] is `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` then [`descriptor_count`] **must**  be a multiple of `4`

## Valid Usage (Implicit)
-  [`type_`] **must**  be a valid [`DescriptorType`] value

# Related
- [`crate::vulkan1_0`]
- [`DescriptorPoolCreateInfo`]
- [`DescriptorType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        