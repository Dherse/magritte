[vkEnumerateDeviceExtensionProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumerateDeviceExtensionProperties.html) - Returns properties of available physical device extensions

# C Specifications
To query the extensions available to a given physical device, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkEnumerateDeviceExtensionProperties(
    VkPhysicalDevice                            physicalDevice,
    const char*                                 pLayerName,
    uint32_t*                                   pPropertyCount,
    VkExtensionProperties*                      pProperties);
```

# Parameters
- [`physical_device`] is the physical device that will be queried.
- [`p_layer_name`] is either `NULL` or a pointer to a null-terminated UTF-8 string naming the layer to retrieve extensions from.
- [`p_property_count`] is a pointer to an integer related to the number of extension properties available or queried, and is treated in the same fashion as the [`enumerate_instance_extension_properties`]::[`p_property_count`] parameter.
- [`p_properties`] is either `NULL` or a pointer to an array of [`ExtensionProperties`] structures.

# Description
When [`p_layer_name`] parameter is `NULL`, only extensions provided by the
Vulkan implementation or by implicitly enabled layers are returned.
When [`p_layer_name`] is the name of a layer, the device extensions provided
by that layer are returned.Implementations  **must**  not advertise any pair of extensions that cannot be
enabled together due to behavioral differences, or any extension that cannot
be enabled against the advertised version.Implementations claiming support for the [Roadmap 2022](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#roadmap-2022)
profile  **must**  advertise the [`VK_KHR_global_priority`] extension in
[`p_properties`].
## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-    If [`p_layer_name`] is not `NULL`, [`p_layer_name`] **must**  be a null-terminated UTF-8 string
-  [`p_property_count`] **must**  be a valid pointer to a `uint32_t` value
-    If the value referenced by [`p_property_count`] is not `0`, and [`p_properties`] is not `NULL`, [`p_properties`] **must**  be a valid pointer to an array of [`p_property_count`][`ExtensionProperties`] structures

## Return Codes
*   - `VK_SUCCESS`  - `VK_INCOMPLETE` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_LAYER_NOT_PRESENT`

# Related
- [`crate::vulkan1_0`]
- [`ExtensionProperties`]
- [`PhysicalDevice`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        