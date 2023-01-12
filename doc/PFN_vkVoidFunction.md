[PFN_vkVoidFunction](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/PFN_vkVoidFunction.html) - Placeholder function pointer type returned by queries

# C Specifications
The definition of [`PFNVoidFunction`] is:
```c
// Provided by VK_VERSION_1_0
typedef void (VKAPI_PTR *PFN_vkVoidFunction)(void);
```

# Parameters
This type is returned from command function pointer queries, and  **must**  be

# Description
cast to an actual command function pointer before use.

# Related
- [`crate::vulkan1_0`]
- [`get_device_proc_addr`]
- [`get_instance_proc_addr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        