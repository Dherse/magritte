[VK_QUEUE_FAMILY_FOREIGN_EXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_QUEUE_FAMILY_FOREIGN_EXT.html) - Foreign queue family index sentinel

# C Specifications
The special queue family index [`QUEUE_FAMILY_FOREIGN_EXT`] represents
any queue external to the resource’s current Vulkan instance, regardless of
the queue’s underlying physical device or driver version.
This includes, for example, queues for fixed-function image processing
devices, media codec devices, and display devices, as well as all queues
that use the same underlying
device group or
physical device, and the same driver version as the resource’s
[`Device`].
```c
#define VK_QUEUE_FAMILY_FOREIGN_EXT       (~2U)
```

# Related
- [`VK_EXT_queue_family_foreign`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        