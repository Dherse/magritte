[VkCommandPoolResetFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandPoolResetFlagBits.html) - Bitmask controlling behavior of a command pool reset

# C Specifications
Bits which  **can**  be set in [`reset_command_pool`]`::flags`, controlling
the reset operation, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkCommandPoolResetFlagBits {
    VK_COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT = 0x00000001,
} VkCommandPoolResetFlagBits;
```

# Description
- [`RELEASE_RESOURCES`] specifies that resetting a command pool recycles all of the resources from the command pool back to the system.

# Related
- [`crate::vulkan1_0`]
- [`CommandPoolResetFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        