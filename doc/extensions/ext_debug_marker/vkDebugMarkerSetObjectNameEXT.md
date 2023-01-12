[vkDebugMarkerSetObjectNameEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDebugMarkerSetObjectNameEXT.html) - Give a user-friendly name to an object

# C Specifications
An object can be given a user-friendly name by calling:
```c
// Provided by VK_EXT_debug_marker
VkResult vkDebugMarkerSetObjectNameEXT(
    VkDevice                                    device,
    const VkDebugMarkerObjectNameInfoEXT*       pNameInfo);
```

# Parameters
- [`device`] is the device that created the object.
- [`p_name_info`] is a pointer to a [`DebugMarkerObjectNameInfoEXT`] structure specifying the parameters of the name to set on the object.

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_name_info`] **must**  be a valid pointer to a valid [`DebugMarkerObjectNameInfoEXT`] structure

## Host Synchronization
- Host access to `pNameInfo->object` **must**  be externally synchronized

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`ext_debug_marker`]
- [`DebugMarkerObjectNameInfoEXT`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        