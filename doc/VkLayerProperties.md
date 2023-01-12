[VkLayerProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkLayerProperties.html) - Structure specifying layer properties

# C Specifications
The [`LayerProperties`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkLayerProperties {
    char        layerName[VK_MAX_EXTENSION_NAME_SIZE];
    uint32_t    specVersion;
    uint32_t    implementationVersion;
    char        description[VK_MAX_DESCRIPTION_SIZE];
} VkLayerProperties;
```

# Members
- [`layer_name`] is an array of `VK_MAX_EXTENSION_NAME_SIZE``char` containing a null-terminated UTF-8 string which is the name of the layer. Use this name in the `ppEnabledLayerNames` array passed in the [`InstanceCreateInfo`] structure to enable this layer for an instance.
- [`spec_version`] is the Vulkan version the layer was written to, encoded as described in [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#extendingvulkan-coreversions-versionnumbers](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#extendingvulkan-coreversions-versionnumbers).
- [`implementation_version`] is the version of this layer. It is an integer, increasing with backward compatible changes.
- [`description`] is an array of `VK_MAX_DESCRIPTION_SIZE``char` containing a null-terminated UTF-8 string which provides additional details that  **can**  be used by the application to identify the layer.

# Related
- [`crate::vulkan1_0`]
- [`enumerate_device_layer_properties`]
- [`enumerate_instance_layer_properties`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        