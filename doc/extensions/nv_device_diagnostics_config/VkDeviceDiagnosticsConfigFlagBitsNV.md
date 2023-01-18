[VkDeviceDiagnosticsConfigFlagBitsNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceDiagnosticsConfigFlagBitsNV.html) - Bitmask specifying diagnostics flags

# C Specifications
Bits which  **can**  be set in
[`DeviceDiagnosticsConfigCreateInfoNV::flags`] include:
```c
// Provided by VK_NV_device_diagnostics_config
typedef enum VkDeviceDiagnosticsConfigFlagBitsNV {
    VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_SHADER_DEBUG_INFO_BIT_NV = 0x00000001,
    VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_RESOURCE_TRACKING_BIT_NV = 0x00000002,
    VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_AUTOMATIC_CHECKPOINTS_BIT_NV = 0x00000004,
} VkDeviceDiagnosticsConfigFlagBitsNV;
```

# Description
- [`ENABLE_SHADER_DEBUG_INFO`] enables the generation of debug information for shaders.
- [`ENABLE_RESOURCE_TRACKING`] enables driver side tracking of resources (images, buffers, etc.) used to augment the device fault information.
- [`ENABLE_AUTOMATIC_CHECKPOINTS`] enables automatic insertion of [diagnostic checkpoints](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#device-diagnostic-checkpoints) for draw calls, dispatches, trace rays, and copies. The CPU call stack at the time of the command will be associated as the marker data for the automatically inserted checkpoints.

# Related
- [`VK_NV_device_diagnostics_config`]
- [`DeviceDiagnosticsConfigFlagsNV`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        