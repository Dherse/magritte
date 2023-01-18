[VkExtensionProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExtensionProperties.html) - Structure specifying an extension properties

# C Specifications
The [`ExtensionProperties`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkExtensionProperties {
    char        extensionName[VK_MAX_EXTENSION_NAME_SIZE];
    uint32_t    specVersion;
} VkExtensionProperties;
```

# Members
- [`extension_name`] is an array of [`MAX_EXTENSION_NAME_SIZE`]`char` containing a null-terminated UTF-8 string which is the name of the extension.
- [`spec_version`] is the version of this extension. It is an integer, incremented with backward compatible changes.

# Related
- [`crate::vulkan1_0`]
- [`VideoCapabilitiesKHR`]
- [`VideoSessionCreateInfoKHR`]
- [`enumerate_device_extension_properties`]
- [`enumerate_instance_extension_properties`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        