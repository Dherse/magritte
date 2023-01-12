[vkGetMemoryAndroidHardwareBufferANDROID](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryAndroidHardwareBufferANDROID.html) - Get an Android hardware buffer for a memory object

# C Specifications
To export an Android hardware buffer referencing the payload of a Vulkan
device memory object, call:
```c
// Provided by VK_ANDROID_external_memory_android_hardware_buffer
VkResult vkGetMemoryAndroidHardwareBufferANDROID(
    VkDevice                                    device,
    const VkMemoryGetAndroidHardwareBufferInfoANDROID* pInfo,
    struct AHardwareBuffer**                    pBuffer);
```

# Parameters
- [`device`] is the logical device that created the device memory being exported.
- [`p_info`] is a pointer to a [`MemoryGetAndroidHardwareBufferInfoANDROID`] structure containing parameters of the export operation.
- [`p_buffer`] will return an Android hardware buffer referencing the payload of the device memory object.

# Description
Each call to [`get_memory_android_hardware_buffer_android`] **must**  return an
Android hardware buffer with a new reference acquired in addition to the
reference held by the [`DeviceMemory`].
To avoid leaking resources, the application  **must**  release the reference by
calling `AHardwareBuffer_release` when it is no longer needed.
When called with the same handle in
[`MemoryGetAndroidHardwareBufferInfoANDROID::memory`],
[`get_memory_android_hardware_buffer_android`] **must**  return the same Android
hardware buffer object.
If the device memory was created by importing an Android hardware buffer,
[`get_memory_android_hardware_buffer_android`] **must**  return that same Android
hardware buffer object.
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_info`] **must**  be a valid pointer to a valid [`MemoryGetAndroidHardwareBufferInfoANDROID`] structure
-  [`p_buffer`] **must**  be a valid pointer to a valid pointer to an [`AHardwareBuffer`] value

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`

# Related
- [`android_external_memory_android_hardware_buffer`]
- [`Device`]
- [`MemoryGetAndroidHardwareBufferInfoANDROID`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        