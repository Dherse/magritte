[VkTessellationDomainOrigin](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkTessellationDomainOrigin.html) - Enum describing tessellation domain origin

# C Specifications
The possible tessellation domain origins are specified by the
[`TessellationDomainOrigin`] enumeration:
```c
// Provided by VK_VERSION_1_1
typedef enum VkTessellationDomainOrigin {
    VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT = 0,
    VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT = 1,
  // Provided by VK_KHR_maintenance2
    VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT_KHR = VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT,
  // Provided by VK_KHR_maintenance2
    VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT_KHR = VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT,
} VkTessellationDomainOrigin;
```
or the equivalent
```c
// Provided by VK_KHR_maintenance2
typedef VkTessellationDomainOrigin VkTessellationDomainOriginKHR;
```

# Description
- [`VK_TESSELLATION_DOMAIN_ORIGIN`] specifies that the origin of the domain space is in the upper left corner, as shown in figure [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#img-tessellation-topology-ul](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#img-tessellation-topology-ul).
- [`VK_TESSELLATION_DOMAIN_ORIGIN`] specifies that the origin of the domain space is in the lower left corner, as shown in figure [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#img-tessellation-topology-ll](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#img-tessellation-topology-ll).
This enum affects how the `VertexOrderCw` and `VertexOrderCcw`
tessellation execution modes are interpreted, since the winding is defined
relative to the orientation of the domain.

# Related
- [`crate::vulkan1_1`]
- [`PipelineTessellationDomainOriginStateCreateInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        