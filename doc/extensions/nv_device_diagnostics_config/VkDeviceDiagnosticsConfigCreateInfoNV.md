[VkDeviceDiagnosticsConfigCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceDiagnosticsConfigCreateInfoNV.html) - Specify diagnostics config for a Vulkan device

# C Specifications
When using the Nsight<sup>™</sup> Aftermath SDK, to configure how device crash
dumps are created, add a [`DeviceDiagnosticsConfigCreateInfoNV`]
structure to the [`p_next`] chain of the [`DeviceCreateInfo`]
structure.
```c
// Provided by VK_NV_device_diagnostics_config
typedef struct VkDeviceDiagnosticsConfigCreateInfoNV {
    VkStructureType                     sType;
    const void*                         pNext;
    VkDeviceDiagnosticsConfigFlagsNV    flags;
} VkDeviceDiagnosticsConfigCreateInfoNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is a bitmask of [`DeviceDiagnosticsConfigFlagBitsNV`] specifying addtional parameters for configuring diagnostic tools.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV`
-  [`flags`] **must**  be a valid combination of [`DeviceDiagnosticsConfigFlagBitsNV`] values

# Related
- [`nv_device_diagnostics_config`]
- [VkDeviceDiagnosticsConfigFlagsNV]()
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        