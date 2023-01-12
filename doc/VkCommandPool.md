[VkCommandPool](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandPool.html) - Opaque handle to a command pool object

# C Specifications
Command pools are opaque objects that command buffer memory is allocated
from, and which allow the implementation to amortize the cost of resource
creation across multiple command buffers.
Command pools are externally synchronized, meaning that a command pool  **must** 
not be used concurrently in multiple threads.
That includes use via recording commands on any command buffers allocated
from the pool, as well as operations that allocate, free, and reset command
buffers or the pool itself.Command pools are represented by [`CommandPool`] handles:
```c
// Provided by VK_VERSION_1_0
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkCommandPool)
```

# Related
- [`crate::vulkan1_0`]
- [`CommandBufferAllocateInfo`]
- [`create_command_pool`]
- [`destroy_command_pool`]
- [`free_command_buffers`]
- [`reset_command_pool`]
- [`trim_command_pool`]
- [`trim_command_pool_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        