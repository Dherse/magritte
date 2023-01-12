[vkGetGeneratedCommandsMemoryRequirementsNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetGeneratedCommandsMemoryRequirementsNV.html) - Retrieve the buffer allocation requirements for generated commands

# C Specifications
The generation of commands on the device requires a `preprocess` buffer.
To retrieve the memory size and alignment requirements of a particular
execution state call:
```c
// Provided by VK_NV_device_generated_commands
void vkGetGeneratedCommandsMemoryRequirementsNV(
    VkDevice                                    device,
    const VkGeneratedCommandsMemoryRequirementsInfoNV* pInfo,
    VkMemoryRequirements2*                      pMemoryRequirements);
```

# Parameters
- [`device`] is the logical device that owns the buffer.
- [`p_info`] is a pointer to a [`GeneratedCommandsMemoryRequirementsInfoNV`] structure containing parameters required for the memory requirements query.
- [`p_memory_requirements`] is a pointer to a [`MemoryRequirements2`] structure in which the memory requirements of the buffer object are returned.

# Description
## Valid Usage
-    The [[`PhysicalDeviceDeviceGeneratedCommandsFeaturesNV::device_generated_commands`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-deviceGeneratedCommands) feature  **must**  be enabled

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_info`] **must**  be a valid pointer to a valid [`GeneratedCommandsMemoryRequirementsInfoNV`] structure
-  [`p_memory_requirements`] **must**  be a valid pointer to a [`MemoryRequirements2`] structure

# Related
- [`nv_device_generated_commands`]
- [`Device`]
- [`GeneratedCommandsMemoryRequirementsInfoNV`]
- [`MemoryRequirements2`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        