[vkGetAndroidHardwareBufferPropertiesANDROID](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAndroidHardwareBufferPropertiesANDROID.html) - Get Properties of External Memory Android Hardware Buffers

# C Specifications
To determine the memory parameters to use when importing an Android hardware
buffer, call:
```c
// Provided by VK_ANDROID_external_memory_android_hardware_buffer
VkResult vkGetAndroidHardwareBufferPropertiesANDROID(
    VkDevice                                    device,
    const struct AHardwareBuffer*               buffer,
    VkAndroidHardwareBufferPropertiesANDROID*   pProperties);
```

# Parameters
- [`device`] is the logical device that will be importing [`buffer`].
- [`buffer`] is the Android hardware buffer which will be imported.
- [`p_properties`] is a pointer to a [`AndroidHardwareBufferPropertiesANDROID`] structure in which the properties of [`buffer`] are returned.

# Description
## Valid Usage
-  [`buffer`] **must**  be a valid Android hardware buffer object with at least one of the `AHARDWAREBUFFER_USAGE_GPU_*` flags in its `AHardwareBuffer_Desc`::`usage`

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`buffer`] **must**  be a valid pointer to a valid [`AHardwareBuffer`] value
-  [`p_properties`] **must**  be a valid pointer to a [`AndroidHardwareBufferPropertiesANDROID`] structure

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR`

# Related
- [`VK_ANDROID_external_memory_android_hardware_buffer`]
- [`AndroidHardwareBufferPropertiesANDROID`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        