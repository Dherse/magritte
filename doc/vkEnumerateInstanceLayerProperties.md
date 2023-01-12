[vkEnumerateInstanceLayerProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceLayerProperties.html) - Returns up to requested number of global layer properties

# C Specifications
To query the available layers, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkEnumerateInstanceLayerProperties(
    uint32_t*                                   pPropertyCount,
    VkLayerProperties*                          pProperties);
```

# Parameters
- [`p_property_count`] is a pointer to an integer related to the number of layer properties available or queried, as described below.
- [`p_properties`] is either `NULL` or a pointer to an array of [`LayerProperties`] structures.

# Description
If [`p_properties`] is `NULL`, then the number of layer properties
available is returned in [`p_property_count`].
Otherwise, [`p_property_count`] **must**  point to a variable set by the user to
the number of elements in the [`p_properties`] array, and on return the
variable is overwritten with the number of structures actually written to
[`p_properties`].
If [`p_property_count`] is less than the number of layer properties
available, at most [`p_property_count`] structures will be written, and
`VK_INCOMPLETE` will be returned instead of `VK_SUCCESS`, to
indicate that not all the available properties were returned.The list of available layers may change at any time due to actions outside
of the Vulkan implementation, so two calls to
[`enumerate_instance_layer_properties`] with the same parameters  **may** 
return different results, or retrieve different [`p_property_count`] values
or [`p_properties`] contents.
Once an instance has been created, the layers enabled for that instance will
continue to be enabled and valid for the lifetime of that instance, even if
some of them become unavailable for future instances.
## Valid Usage (Implicit)
-  [`p_property_count`] **must**  be a valid pointer to a `uint32_t` value
-    If the value referenced by [`p_property_count`] is not `0`, and [`p_properties`] is not `NULL`, [`p_properties`] **must**  be a valid pointer to an array of [`p_property_count`][`LayerProperties`] structures

## Return Codes
*   - `VK_SUCCESS`  - `VK_INCOMPLETE` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`crate::vulkan1_0`]
- [`LayerProperties`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        