[VkDependencyFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDependencyFlagBits.html) - Bitmask specifying how execution and memory dependencies are formed

# C Specifications
Bits which  **can**  be set in [`cmd_pipeline_barrier`]`::dependencyFlags`,
specifying how execution and memory dependencies are formed, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkDependencyFlagBits {
    VK_DEPENDENCY_BY_REGION_BIT = 0x00000001,
  // Provided by VK_VERSION_1_1
    VK_DEPENDENCY_DEVICE_GROUP_BIT = 0x00000004,
  // Provided by VK_VERSION_1_1
    VK_DEPENDENCY_VIEW_LOCAL_BIT = 0x00000002,
  // Provided by VK_KHR_multiview
    VK_DEPENDENCY_VIEW_LOCAL_BIT_KHR = VK_DEPENDENCY_VIEW_LOCAL_BIT,
  // Provided by VK_KHR_device_group
    VK_DEPENDENCY_DEVICE_GROUP_BIT_KHR = VK_DEPENDENCY_DEVICE_GROUP_BIT,
} VkDependencyFlagBits;
```

# Description
- [`VK_DEPENDENCY_FLAG_BITS`] specifies that dependencies will be [framebuffer-local](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-framebuffer-regions).
- [`VIEW_LOCAL`] specifies that a [subpass has more than one view](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-barriers-subpass-self-dependencies).
- [`DEVICE_GROUP`] specifies that dependencies are [non-device-local](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-device-local-dependencies).

# Related
- [`crate::vulkan1_0`]
- [VkDependencyFlags]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        