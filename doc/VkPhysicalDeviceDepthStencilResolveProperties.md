[VkPhysicalDeviceDepthStencilResolveProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDepthStencilResolveProperties.html) - Structure describing depth/stencil resolve properties that can be supported by an implementation

# C Specifications
The [`PhysicalDeviceDepthStencilResolveProperties`] structure is defined
as:
```c
// Provided by VK_VERSION_1_2
typedef struct VkPhysicalDeviceDepthStencilResolveProperties {
    VkStructureType       sType;
    void*                 pNext;
    VkResolveModeFlags    supportedDepthResolveModes;
    VkResolveModeFlags    supportedStencilResolveModes;
    VkBool32              independentResolveNone;
    VkBool32              independentResolve;
} VkPhysicalDeviceDepthStencilResolveProperties;
```
or the equivalent
```c
// Provided by VK_KHR_depth_stencil_resolve
typedef VkPhysicalDeviceDepthStencilResolveProperties VkPhysicalDeviceDepthStencilResolvePropertiesKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.

# Description
- [`supported_depth_resolve_modes`] is a bitmask of [`ResolveModeFlagBits`] indicating the set of supported depth resolve modes. `VK_RESOLVE_MODE_SAMPLE_ZERO_BIT` **must**  be included in the set but implementations  **may**  support additional modes.
- [`supported_stencil_resolve_modes`] is a bitmask of [`ResolveModeFlagBits`] indicating the set of supported stencil resolve modes. `VK_RESOLVE_MODE_SAMPLE_ZERO_BIT` **must**  be included in the set but implementations  **may**  support additional modes. `VK_RESOLVE_MODE_AVERAGE_BIT` **must**  not be included in the set.
- [`independent_resolve_none`] is [`TRUE`] if the implementation supports setting the depth and stencil resolve modes to different values when one of those modes is `VK_RESOLVE_MODE_NONE`. Otherwise the implementation only supports setting both modes to the same value.
- [`independent_resolve`] is [`TRUE`] if the implementation supports all combinations of the supported depth and stencil resolve modes, including setting either depth or stencil resolve mode to `VK_RESOLVE_MODE_NONE`. An implementation that supports [`independent_resolve`] **must**  also support [`independent_resolve_none`].
If the [`PhysicalDeviceDepthStencilResolveProperties`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceProperties2`] structure passed to
[`get_physical_device_properties2`], it is filled in with each
corresponding implementation-dependent property.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES`

# Related
- [`VK_KHR_depth_stencil_resolve`]
- [`crate::vulkan1_2`]
- [`Bool32`]
- [`ResolveModeFlags`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        