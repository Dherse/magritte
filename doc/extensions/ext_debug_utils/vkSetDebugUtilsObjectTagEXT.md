[vkSetDebugUtilsObjectTagEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetDebugUtilsObjectTagEXT.html) - Attach arbitrary data to an object

# C Specifications
```c
// Provided by VK_EXT_debug_utils
VkResult vkSetDebugUtilsObjectTagEXT(
    VkDevice                                    device,
    const VkDebugUtilsObjectTagInfoEXT*         pTagInfo);
```

# Parameters
- [`device`] is the device that created the object.
- [`p_tag_info`] is a pointer to a [`DebugUtilsObjectTagInfoEXT`] structure specifying parameters of the tag to attach to the object.

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_tag_info`] **must**  be a valid pointer to a valid [`DebugUtilsObjectTagInfoEXT`] structure

## Host Synchronization
- Host access to `pTagInfo->objectHandle` **must**  be externally synchronized

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`ext_debug_utils`]
- [`DebugUtilsObjectTagInfoEXT`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        