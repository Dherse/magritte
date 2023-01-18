[vkSetDebugUtilsObjectNameEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetDebugUtilsObjectNameEXT.html) - Give a user-friendly name to an object

# C Specifications
```c
// Provided by VK_EXT_debug_utils
VkResult vkSetDebugUtilsObjectNameEXT(
    VkDevice                                    device,
    const VkDebugUtilsObjectNameInfoEXT*        pNameInfo);
```

# Parameters
- [`device`] is the device that created the object.
- [`p_name_info`] is a pointer to a [`DebugUtilsObjectNameInfoEXT`] structure specifying parameters of the name to set on the object.

# Description
## Valid Usage
-  `pNameInfo->objectType` **must**  not be `VK_OBJECT_TYPE_UNKNOWN`
-  `pNameInfo->objectHandle` **must**  not be [`crate::Handle::null`]

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_name_info`] **must**  be a valid pointer to a valid [`DebugUtilsObjectNameInfoEXT`] structure

## Host Synchronization
- Host access to `pNameInfo->objectHandle` **must**  be externally synchronized

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`VK_EXT_debug_utils`]
- [`DebugUtilsObjectNameInfoEXT`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        