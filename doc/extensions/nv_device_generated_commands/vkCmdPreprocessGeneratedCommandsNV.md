[vkCmdPreprocessGeneratedCommandsNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPreprocessGeneratedCommandsNV.html) - Performs preprocessing for generated commands

# C Specifications
Commands  **can**  be preprocessed prior execution using the following command:
```c
// Provided by VK_NV_device_generated_commands
void vkCmdPreprocessGeneratedCommandsNV(
    VkCommandBuffer                             commandBuffer,
    const VkGeneratedCommandsInfoNV*            pGeneratedCommandsInfo);
```

# Parameters
- [`command_buffer`] is the command buffer which does the preprocessing.
- [`p_generated_commands_info`] is a pointer to a [`GeneratedCommandsInfoNV`] structure containing parameters affecting the preprocessing step.

# Description
## Valid Usage
-  [`command_buffer`] **must**  not be a protected command buffer
-  [`p_generated_commands_info`]`s `indirectCommandsLayout` **must**  have been created with the `VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EXPLICIT_PREPROCESS_BIT_NV` bit set
-    The [[`PhysicalDeviceDeviceGeneratedCommandsFeaturesNV::device_generated_commands`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-deviceGeneratedCommands) feature  **must**  be enabled

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`p_generated_commands_info`] **must**  be a valid pointer to a valid [`GeneratedCommandsInfoNV`] structure
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics, or compute operations
-    This command  **must**  only be called outside of a render pass instance

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`nv_device_generated_commands`]
- [`CommandBuffer`]
- [`GeneratedCommandsInfoNV`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        