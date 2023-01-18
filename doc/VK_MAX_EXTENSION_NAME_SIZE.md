[VK_MAX_EXTENSION_NAME_SIZE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_EXTENSION_NAME_SIZE.html) - Maximum length of a layer of extension name string

# C Specifications
[`MAX_EXTENSION_NAME_SIZE`] is the length in `char` values of an
array containing a layer or extension name string, as returned in
[`LayerProperties`]::layerName,
[`ExtensionProperties`]::extensionName, and other queries.
```c
#define VK_MAX_EXTENSION_NAME_SIZE        256U
```
# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        