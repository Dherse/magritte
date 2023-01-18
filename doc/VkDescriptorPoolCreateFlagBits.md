[VkDescriptorPoolCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorPoolCreateFlagBits.html) - Bitmask specifying certain supported operations on a descriptor pool

# C Specifications
Bits which  **can**  be set in [`DescriptorPoolCreateInfo::flags`],
enabling operations on a descriptor pool, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkDescriptorPoolCreateFlagBits {
    VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT = 0x00000001,
  // Provided by VK_VERSION_1_2
    VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT = 0x00000002,
  // Provided by VK_VALVE_mutable_descriptor_type
    VK_DESCRIPTOR_POOL_CREATE_HOST_ONLY_BIT_VALVE = 0x00000004,
  // Provided by VK_EXT_descriptor_indexing
    VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT_EXT = VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT,
} VkDescriptorPoolCreateFlagBits;
```

# Description
- [`FREE_DESCRIPTOR_SET`] specifies that descriptor sets  **can**  return their individual allocations to the pool, i.e. all of [`allocate_descriptor_sets`], [`free_descriptor_sets`], and [`reset_descriptor_pool`] are allowed. Otherwise, descriptor sets allocated from the pool  **must**  not be individually freed back to the pool, i.e. only [`allocate_descriptor_sets`] and [`reset_descriptor_pool`] are allowed.
- [`UPDATE_AFTER_BIND`] specifies that descriptor sets allocated from this pool  **can**  include bindings with the `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` bit set. It is valid to allocate descriptor sets that have bindings that do not set the `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` bit from a pool that has [`UPDATE_AFTER_BIND`] set.
- [`HOST_ONLY_VALVE`] specifies that this descriptor pool and the descriptor sets allocated from it reside entirely in host memory and cannot be bound. Descriptor sets allocated from this pool are partially exempt from the external synchronization requirement in [`update_descriptor_set_with_template_khr`] and [`update_descriptor_sets`]. Descriptor sets and their descriptors can be updated concurrently in different threads, though the same descriptor  **must**  not be updated concurrently by two threads.

# Related
- [`crate::vulkan1_0`]
- [`DescriptorPoolCreateFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        