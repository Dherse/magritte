[VkAccessFlags2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccessFlags2.html) - 64-bit mask of access flags

# C Specifications
[`AccessFlags2`] is a bitmask type for setting a mask of zero or more
[`AccessFlagBits2`]:
```c
// Provided by VK_VERSION_1_3
typedef VkFlags64 VkAccessFlags2;
```
or the equivalent
```c
// Provided by VK_KHR_synchronization2
typedef VkAccessFlags2 VkAccessFlags2KHR;
```

# Related
- [`khr_synchronization2`]
- [`crate::vulkan1_3`]
- [`BufferMemoryBarrier2`]
- [`ImageMemoryBarrier2`]
- [`MemoryBarrier2`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        