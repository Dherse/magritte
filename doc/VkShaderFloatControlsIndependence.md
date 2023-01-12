[VkShaderFloatControlsIndependence](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderFloatControlsIndependence.html) - Bitmask specifying whether, and how, shader float controls can be set separately

# C Specifications
Values which  **may**  be returned in the `denormBehaviorIndependence` and
`roundingModeIndependence` fields of
[`PhysicalDeviceFloatControlsProperties`] are:
```c
// Provided by VK_VERSION_1_2
typedef enum VkShaderFloatControlsIndependence {
    VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY = 0,
    VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL = 1,
    VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE = 2,
  // Provided by VK_KHR_shader_float_controls
    VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY_KHR = VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY,
  // Provided by VK_KHR_shader_float_controls
    VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL_KHR = VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL,
  // Provided by VK_KHR_shader_float_controls
    VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE_KHR = VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE,
} VkShaderFloatControlsIndependence;
```
or the equivalent
```c
// Provided by VK_KHR_shader_float_controls
typedef VkShaderFloatControlsIndependence VkShaderFloatControlsIndependenceKHR;
```

# Description
- [`VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE`] specifies that shader float controls for 32-bit floating point  **can**  be set independently; other bit widths  **must**  be set identically to each other.
- [`VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE`] specifies that shader float controls for all bit widths  **can**  be set independently.
- [`VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE`] specifies that shader float controls for all bit widths  **must**  be set identically.

# Related
- [`khr_shader_float_controls`]
- [`crate::vulkan1_2`]
- [`PhysicalDeviceFloatControlsProperties`]
- [`PhysicalDeviceVulkan12Properties`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        