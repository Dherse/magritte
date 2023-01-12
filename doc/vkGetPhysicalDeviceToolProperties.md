[vkGetPhysicalDeviceToolProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceToolProperties.html) - Reports properties of tools active on the specified physical device

# C Specifications
Information about tools providing debugging, profiling, or similar services,
active for a given physical device, can be obtained by calling:
```c
// Provided by VK_VERSION_1_3
VkResult vkGetPhysicalDeviceToolProperties(
    VkPhysicalDevice                            physicalDevice,
    uint32_t*                                   pToolCount,
    VkPhysicalDeviceToolProperties*             pToolProperties);
```
or the equivalent command
```c
// Provided by VK_EXT_tooling_info
VkResult vkGetPhysicalDeviceToolPropertiesEXT(
    VkPhysicalDevice                            physicalDevice,
    uint32_t*                                   pToolCount,
    VkPhysicalDeviceToolProperties*             pToolProperties);
```

# Parameters
- [`physical_device`] is the handle to the physical device to query for active tools.
- [`p_tool_count`] is a pointer to an integer describing the number of tools active on [`physical_device`].
- [`p_tool_properties`] is either `NULL` or a pointer to an array of [`PhysicalDeviceToolProperties`] structures.

# Description
If [`p_tool_properties`] is `NULL`, then the number of tools currently
active on [`physical_device`] is returned in [`p_tool_count`].
Otherwise, [`p_tool_count`] **must**  point to a variable set by the user to the
number of elements in the [`p_tool_properties`] array, and on return the
variable is overwritten with the number of structures actually written to
[`p_tool_properties`].
If [`p_tool_count`] is less than the number of currently active tools, at
most [`p_tool_count`] structures will be written.The count and properties of active tools  **may**  change in response to events
outside the scope of the specification.
An application  **should**  assume these properties might change at any given
time.
## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`p_tool_count`] **must**  be a valid pointer to a `uint32_t` value
-    If the value referenced by [`p_tool_count`] is not `0`, and [`p_tool_properties`] is not `NULL`, [`p_tool_properties`] **must**  be a valid pointer to an array of [`p_tool_count`][`PhysicalDeviceToolProperties`] structures

## Return Codes
*   - `VK_SUCCESS`  - `VK_INCOMPLETE` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`

# Related
- [`ext_tooling_info`]
- [`crate::vulkan1_3`]
- [`PhysicalDevice`]
- [`PhysicalDeviceToolProperties`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        