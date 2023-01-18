[VkIndirectCommandsLayoutNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsLayoutNV.html) - Opaque handle to an indirect commands layout object

# C Specifications
The device-side command generation happens through an iterative processing
of an atomic sequence comprised of command tokens, which are represented by:
```c
// Provided by VK_NV_device_generated_commands
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkIndirectCommandsLayoutNV)
```

# Related
- [`VK_NV_device_generated_commands`]
- [`GeneratedCommandsInfoNV`]
- [`GeneratedCommandsMemoryRequirementsInfoNV`]
- [`create_indirect_commands_layout_nv`]
- [`destroy_indirect_commands_layout_nv`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        