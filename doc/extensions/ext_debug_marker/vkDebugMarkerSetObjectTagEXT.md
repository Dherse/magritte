[vkDebugMarkerSetObjectTagEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDebugMarkerSetObjectTagEXT.html) - Attach arbitrary data to an object

# C Specifications
In addition to setting a name for an object, debugging and validation layers
may have uses for additional binary data on a per-object basis that has no
other place in the Vulkan API.
For example, a [`ShaderModule`] could have additional debugging data
attached to it to aid in offline shader tracing.
To attach data to an object, call:
```c
// Provided by VK_EXT_debug_marker
VkResult vkDebugMarkerSetObjectTagEXT(
    VkDevice                                    device,
    const VkDebugMarkerObjectTagInfoEXT*        pTagInfo);
```

# Parameters
- [`device`] is the device that created the object.
- [`p_tag_info`] is a pointer to a [`DebugMarkerObjectTagInfoEXT`] structure specifying the parameters of the tag to attach to the object.

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_tag_info`] **must**  be a valid pointer to a valid [`DebugMarkerObjectTagInfoEXT`] structure

## Host Synchronization
- Host access to `pTagInfo->object` **must**  be externally synchronized

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`ext_debug_marker`]
- [`DebugMarkerObjectTagInfoEXT`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        