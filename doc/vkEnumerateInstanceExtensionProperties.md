[vkEnumerateInstanceExtensionProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceExtensionProperties.html) - Returns up to requested number of global extension properties

# C Specifications
To query the available instance extensions, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkEnumerateInstanceExtensionProperties(
    const char*                                 pLayerName,
    uint32_t*                                   pPropertyCount,
    VkExtensionProperties*                      pProperties);
```

# Parameters
- [`p_layer_name`] is either `NULL` or a pointer to a null-terminated UTF-8 string naming the layer to retrieve extensions from.
- [`p_property_count`] is a pointer to an integer related to the number of extension properties available or queried, as described below.
- [`p_properties`] is either `NULL` or a pointer to an array of [`ExtensionProperties`] structures.

# Description
When [`p_layer_name`] parameter is `NULL`, only extensions provided by the
Vulkan implementation or by implicitly enabled layers are returned.
When [`p_layer_name`] is the name of a layer, the instance extensions
provided by that layer are returned.If [`p_properties`] is `NULL`, then the number of extensions properties
available is returned in [`p_property_count`].
Otherwise, [`p_property_count`] **must**  point to a variable set by the user to
the number of elements in the [`p_properties`] array, and on return the
variable is overwritten with the number of structures actually written to
[`p_properties`].
If [`p_property_count`] is less than the number of extension properties
available, at most [`p_property_count`] structures will be written, and
`VK_INCOMPLETE` will be returned instead of `VK_SUCCESS`, to
indicate that not all the available properties were returned.Because the list of available layers may change externally between calls to
[`enumerate_instance_extension_properties`], two calls may retrieve
different results if a [`p_layer_name`] is available in one call but not in
another.
The extensions supported by a layer may also change between two calls, e.g.
if the layer implementation is replaced by a different version between those
calls.Implementations  **must**  not advertise any pair of extensions that cannot be
enabled together due to behavioral differences, or any extension that cannot
be enabled against the advertised version.
## Valid Usage (Implicit)
-    If [`p_layer_name`] is not `NULL`, [`p_layer_name`] **must**  be a null-terminated UTF-8 string
-  [`p_property_count`] **must**  be a valid pointer to a `uint32_t` value
-    If the value referenced by [`p_property_count`] is not `0`, and [`p_properties`] is not `NULL`, [`p_properties`] **must**  be a valid pointer to an array of [`p_property_count`][`ExtensionProperties`] structures

## Return Codes
*   - `VK_SUCCESS`  - `VK_INCOMPLETE` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_LAYER_NOT_PRESENT`

# Related
- [`crate::vulkan1_0`]
- [`ExtensionProperties`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        