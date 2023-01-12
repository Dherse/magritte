[VkPeerMemoryFeatureFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPeerMemoryFeatureFlagBits.html) - Bitmask specifying supported peer memory features

# C Specifications
Bits which  **may**  be set in
[`get_device_group_peer_memory_features`]`::pPeerMemoryFeatures`,
indicating supported peer memory features, are:
```c
// Provided by VK_VERSION_1_1
typedef enum VkPeerMemoryFeatureFlagBits {
    VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT = 0x00000001,
    VK_PEER_MEMORY_FEATURE_COPY_DST_BIT = 0x00000002,
    VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT = 0x00000004,
    VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT = 0x00000008,
  // Provided by VK_KHR_device_group
    VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT_KHR = VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT,
  // Provided by VK_KHR_device_group
    VK_PEER_MEMORY_FEATURE_COPY_DST_BIT_KHR = VK_PEER_MEMORY_FEATURE_COPY_DST_BIT,
  // Provided by VK_KHR_device_group
    VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT_KHR = VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT,
  // Provided by VK_KHR_device_group
    VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT_KHR = VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT,
} VkPeerMemoryFeatureFlagBits;
```
or the equivalent
```c
// Provided by VK_KHR_device_group
typedef VkPeerMemoryFeatureFlagBits VkPeerMemoryFeatureFlagBitsKHR;
```

# Description
- [`VK_PEER_MEMORY_FEATURE_FLAG_BITS`] specifies that the memory  **can**  be accessed as the source of any `vkCmdCopy*` command.
- [`VK_PEER_MEMORY_FEATURE_FLAG_BITS`] specifies that the memory  **can**  be accessed as the destination of any `vkCmdCopy*` command.
- [`VK_PEER_MEMORY_FEATURE_FLAG_BITS`] specifies that the memory  **can**  be read as any memory access type.
- [`VK_PEER_MEMORY_FEATURE_FLAG_BITS`] specifies that the memory  **can**  be written as any memory access type. Shader atomics are considered to be writes.
[`VK_PEER_MEMORY_FEATURE_FLAG_BITS`] **must**  be supported for all host
local heaps and for at least one device-local memory heap.If a device does not support a peer memory feature, it is still valid to use
a resource that includes both local and peer memory bindings with the
corresponding access type as long as only the local bindings are actually
accessed.
For example, an application doing split-frame rendering would use
framebuffer attachments that include both local and peer memory bindings,
but would scissor the rendering to only update local memory.

# Related
- [`crate::vulkan1_1`]
- [VkPeerMemoryFeatureFlags]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        