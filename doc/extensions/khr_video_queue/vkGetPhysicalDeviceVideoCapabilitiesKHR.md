[vkGetPhysicalDeviceVideoCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceVideoCapabilitiesKHR.html) - Query video decode or encode capabilities

# C Specifications
To query video decode or encode capabilities for a specific codec, call:
```c
// Provided by VK_KHR_video_queue
VkResult vkGetPhysicalDeviceVideoCapabilitiesKHR(
    VkPhysicalDevice                            physicalDevice,
    const VkVideoProfileKHR*                    pVideoProfile,
    VkVideoCapabilitiesKHR*                     pCapabilities);
```

# Parameters
- [`physical_device`] is the physical device whose video decode or encode capabilities will be queried.
- [`p_video_profile`] is a pointer to a [`VideoProfileKHR`] structure with a chained codec-operation specific video profile structure.
- [`p_capabilities`] is a pointer to a [`VideoCapabilitiesKHR`] structure in which the capabilities are returned.

# Description
## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`p_video_profile`] **must**  be a valid pointer to a valid [`VideoProfileKHR`] structure
-  [`p_capabilities`] **must**  be a valid pointer to a [`VideoCapabilitiesKHR`] structure

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_EXTENSION_NOT_PRESENT`  - `VK_ERROR_INITIALIZATION_FAILED`  - `VK_ERROR_FEATURE_NOT_PRESENT`  - `VK_ERROR_FORMAT_NOT_SUPPORTED`

# Related
- [`khr_video_queue`]
- [`PhysicalDevice`]
- [`VideoCapabilitiesKHR`]
- [`VideoProfileKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        