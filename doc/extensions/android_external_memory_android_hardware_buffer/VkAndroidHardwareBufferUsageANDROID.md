[VkAndroidHardwareBufferUsageANDROID](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAndroidHardwareBufferUsageANDROID.html) - Struct containing Android hardware buffer usage flags

# C Specifications
To obtain optimal Android hardware buffer usage flags for specific image
creation parameters, add a [`AndroidHardwareBufferUsageANDROID`]
structure to the [`p_next`] chain of a [`ImageFormatProperties2`]
structure passed to [`get_physical_device_image_format_properties2`].
This structure is defined as:
```c
// Provided by VK_ANDROID_external_memory_android_hardware_buffer
typedef struct VkAndroidHardwareBufferUsageANDROID {
    VkStructureType    sType;
    void*              pNext;
    uint64_t           androidHardwareBufferUsage;
} VkAndroidHardwareBufferUsageANDROID;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`android_hardware_buffer_usage`] returns the Android hardware buffer usage flags.

# Description
The [`android_hardware_buffer_usage`] field  **must**  include Android hardware
buffer usage flags listed in the
[AHardwareBuffer Usage
Equivalence](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-external-android-hardware-buffer-usage) table when the corresponding Vulkan image usage or image
creation flags are included in the `usage` or `flags` fields of
[`PhysicalDeviceImageFormatInfo2`].
It  **must**  include at least one GPU usage flag
(`AHARDWAREBUFFER_USAGE_GPU_*`), even if none of the corresponding Vulkan
usages or flags are requested.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_USAGE_ANDROID`

# Related
- [`android_external_memory_android_hardware_buffer`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        