[VkDescriptorSetLayoutCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutCreateFlagBits.html) - Bitmask specifying descriptor set layout properties

# C Specifications
Bits which  **can**  be set in
[`DescriptorSetLayoutCreateInfo::flags`], specifying options for
descriptor set layout, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkDescriptorSetLayoutCreateFlagBits {
  // Provided by VK_VERSION_1_2
    VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT = 0x00000002,
  // Provided by VK_KHR_push_descriptor
    VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR = 0x00000001,
  // Provided by VK_VALVE_mutable_descriptor_type
    VK_DESCRIPTOR_SET_LAYOUT_CREATE_HOST_ONLY_POOL_BIT_VALVE = 0x00000004,
  // Provided by VK_EXT_descriptor_indexing
    VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT_EXT = VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT,
} VkDescriptorSetLayoutCreateFlagBits;
```

# Description
- [`PUSH_DESCRIPTOR_KHR`] specifies that descriptor sets  **must**  not be allocated using this layout, and descriptors are instead pushed by [`cmd_push_descriptor_set_khr`].
- [`UPDATE_AFTER_BIND_POOL`] specifies that descriptor sets using this layout  **must**  be allocated from a descriptor pool created with the `VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT` bit set. Descriptor set layouts created with this bit set have alternate limits for the maximum number of descriptors per-stage and per-pipeline layout. The non-UpdateAfterBind limits only count descriptors in sets created without this flag. The UpdateAfterBind limits count all descriptors, but the limits  **may**  be higher than the non-UpdateAfterBind limits.
- [`HOST_ONLY_POOL_VALVE`] specifies that descriptor sets using this layout  **must**  be allocated from a descriptor pool created with the `VK_DESCRIPTOR_POOL_CREATE_HOST_ONLY_BIT_VALVE` bit set. Descriptor set layouts created with this bit have no expressable limit for maximum number of descriptors per-stage. Host descriptor sets are limited only by available host memory, but  **may**  be limited for implementation specific reasons. Implementations  **may**  limit the number of supported descriptors to UpdateAfterBind limits or non-UpdateAfterBind limits, whichever is larger.

# Related
- [`crate::vulkan1_0`]
- [VkDescriptorSetLayoutCreateFlags]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        