[vkGetPhysicalDeviceImageFormatProperties2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties2.html) - Lists physical device's image format capabilities

# C Specifications
To query additional capabilities specific to image types, call:
```c
// Provided by VK_VERSION_1_1
VkResult vkGetPhysicalDeviceImageFormatProperties2(
    VkPhysicalDevice                            physicalDevice,
    const VkPhysicalDeviceImageFormatInfo2*     pImageFormatInfo,
    VkImageFormatProperties2*                   pImageFormatProperties);
```
or the equivalent command
```c
// Provided by VK_KHR_get_physical_device_properties2
VkResult vkGetPhysicalDeviceImageFormatProperties2KHR(
    VkPhysicalDevice                            physicalDevice,
    const VkPhysicalDeviceImageFormatInfo2*     pImageFormatInfo,
    VkImageFormatProperties2*                   pImageFormatProperties);
```

# Parameters
- [`physical_device`] is the physical device from which to query the image capabilities.
- [`p_image_format_info`] is a pointer to a [`PhysicalDeviceImageFormatInfo2`] structure describing the parameters that would be consumed by [`create_image`].
- [`p_image_format_properties`] is a pointer to a [`ImageFormatProperties2`] structure in which capabilities are returned.

# Description
[`get_physical_device_image_format_properties2`] behaves similarly to
[`get_physical_device_image_format_properties`], with the ability to return
extended information in a `pNext` chain of output structures.
## Valid Usage
-    If the `pNext` chain of [`p_image_format_properties`] includes a [`AndroidHardwareBufferUsageANDROID`] structure, the `pNext` chain of [`p_image_format_info`] **must**  include a [`PhysicalDeviceExternalImageFormatInfo`] structure with `handleType` set to `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID`

## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`p_image_format_info`] **must**  be a valid pointer to a valid [`PhysicalDeviceImageFormatInfo2`] structure
-  [`p_image_format_properties`] **must**  be a valid pointer to a [`ImageFormatProperties2`] structure

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_FORMAT_NOT_SUPPORTED`

# Related
- [`crate::vulkan1_1`]
- [`ImageFormatProperties2`]
- [`PhysicalDevice`]
- [`PhysicalDeviceImageFormatInfo2`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        