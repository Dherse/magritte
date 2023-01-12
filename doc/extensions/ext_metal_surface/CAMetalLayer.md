[CAMetalLayer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/CAMetalLayer.html) - CoreAnimation native layer type for Metal

# C Specifications
To remove an unnecessary compile-time dependency, an incomplete type
definition of [`CaMetalLayer`] is provided in the Vulkan headers:
```c
// Provided by VK_EXT_metal_surface

#ifdef __OBJC__
@class CAMetalLayer;
#else
typedef void CAMetalLayer;
#endif
```

# Related
- [`ext_metal_surface`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        