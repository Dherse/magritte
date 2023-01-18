[VkPhysicalDeviceFloatControlsProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFloatControlsProperties.html) - Structure describing properties supported by VK_KHR_shader_float_controls

# C Specifications
The [`PhysicalDeviceFloatControlsProperties`] structure is defined as:
```c
// Provided by VK_VERSION_1_2
typedef struct VkPhysicalDeviceFloatControlsProperties {
    VkStructureType                      sType;
    void*                                pNext;
    VkShaderFloatControlsIndependence    denormBehaviorIndependence;
    VkShaderFloatControlsIndependence    roundingModeIndependence;
    VkBool32                             shaderSignedZeroInfNanPreserveFloat16;
    VkBool32                             shaderSignedZeroInfNanPreserveFloat32;
    VkBool32                             shaderSignedZeroInfNanPreserveFloat64;
    VkBool32                             shaderDenormPreserveFloat16;
    VkBool32                             shaderDenormPreserveFloat32;
    VkBool32                             shaderDenormPreserveFloat64;
    VkBool32                             shaderDenormFlushToZeroFloat16;
    VkBool32                             shaderDenormFlushToZeroFloat32;
    VkBool32                             shaderDenormFlushToZeroFloat64;
    VkBool32                             shaderRoundingModeRTEFloat16;
    VkBool32                             shaderRoundingModeRTEFloat32;
    VkBool32                             shaderRoundingModeRTEFloat64;
    VkBool32                             shaderRoundingModeRTZFloat16;
    VkBool32                             shaderRoundingModeRTZFloat32;
    VkBool32                             shaderRoundingModeRTZFloat64;
} VkPhysicalDeviceFloatControlsProperties;
```
or the equivalent
```c
// Provided by VK_KHR_shader_float_controls
typedef VkPhysicalDeviceFloatControlsProperties VkPhysicalDeviceFloatControlsPropertiesKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.

# Description
- [`denorm_behavior_independence`] is a [`ShaderFloatControlsIndependence`] value indicating whether, and how, denorm behavior can be set independently for different bit widths.
- [`rounding_mode_independence`] is a [`ShaderFloatControlsIndependence`] value indicating whether, and how, rounding modes can be set independently for different bit widths.
- [`shader_signed_zero_inf_nan_preserve_float16`] is a boolean value indicating whether sign of a zero, Nans and <span class="katex"><span class="katex-html" aria-hidden="true"><span class="base"><span class="strut" style="height:0.66666em;vertical-align:-0.08333em;"></span><span class="mord">±</span><span class="mord">∞</span></span></span></span> **can**  be preserved in 16-bit floating-point computations. It also indicates whether the `SignedZeroInfNanPreserve` execution mode  **can**  be used for 16-bit floating-point types.
- [`shader_signed_zero_inf_nan_preserve_float32`] is a boolean value indicating whether sign of a zero, Nans and <span class="katex"><span aria-hidden="true" class="katex-html"><span class="base"><span style="height:0.66666em;vertical-align:-0.08333em;" class="strut"></span><span class="mord">±</span><span class="mord">∞</span></span></span></span> **can**  be preserved in 32-bit floating-point computations. It also indicates whether the `SignedZeroInfNanPreserve` execution mode  **can**  be used for 32-bit floating-point types.
- [`shader_signed_zero_inf_nan_preserve_float64`] is a boolean value indicating whether sign of a zero, Nans and <span class="katex"><span aria-hidden="true" class="katex-html"><span class="base"><span class="strut" style="height:0.66666em;vertical-align:-0.08333em;"></span><span class="mord">±</span><span class="mord">∞</span></span></span></span> **can**  be preserved in 64-bit floating-point computations. It also indicates whether the `SignedZeroInfNanPreserve` execution mode  **can**  be used for 64-bit floating-point types.
- [`shader_denorm_preserve_float16`] is a boolean value indicating whether denormals  **can**  be preserved in 16-bit floating-point computations. It also indicates whether the `DenormPreserve` execution mode  **can**  be used for 16-bit floating-point types.
- [`shader_denorm_preserve_float32`] is a boolean value indicating whether denormals  **can**  be preserved in 32-bit floating-point computations. It also indicates whether the `DenormPreserve` execution mode  **can**  be used for 32-bit floating-point types.
- [`shader_denorm_preserve_float64`] is a boolean value indicating whether denormals  **can**  be preserved in 64-bit floating-point computations. It also indicates whether the `DenormPreserve` execution mode  **can**  be used for 64-bit floating-point types.
- [`shader_denorm_flush_to_zero_float16`] is a boolean value indicating whether denormals  **can**  be flushed to zero in 16-bit floating-point computations. It also indicates whether the `DenormFlushToZero` execution mode  **can**  be used for 16-bit floating-point types.
- [`shader_denorm_flush_to_zero_float32`] is a boolean value indicating whether denormals  **can**  be flushed to zero in 32-bit floating-point computations. It also indicates whether the `DenormFlushToZero` execution mode  **can**  be used for 32-bit floating-point types.
- [`shader_denorm_flush_to_zero_float64`] is a boolean value indicating whether denormals  **can**  be flushed to zero in 64-bit floating-point computations. It also indicates whether the `DenormFlushToZero` execution mode  **can**  be used for 64-bit floating-point types.
- [`shader_rounding_mode_rte_float16`] is a boolean value indicating whether an implementation supports the round-to-nearest-even rounding mode for 16-bit floating-point arithmetic and conversion instructions. It also indicates whether the `RoundingModeRTE` execution mode  **can**  be used for 16-bit floating-point types.
- [`shader_rounding_mode_rte_float32`] is a boolean value indicating whether an implementation supports the round-to-nearest-even rounding mode for 32-bit floating-point arithmetic and conversion instructions. It also indicates whether the `RoundingModeRTE` execution mode  **can**  be used for 32-bit floating-point types.
- [`shader_rounding_mode_rte_float64`] is a boolean value indicating whether an implementation supports the round-to-nearest-even rounding mode for 64-bit floating-point arithmetic and conversion instructions. It also indicates whether the `RoundingModeRTE` execution mode  **can**  be used for 64-bit floating-point types.
- [`shader_rounding_mode_rtz_float16`] is a boolean value indicating whether an implementation supports the round-towards-zero rounding mode for 16-bit floating-point arithmetic and conversion instructions. It also indicates whether the `RoundingModeRTZ` execution mode  **can**  be used for 16-bit floating-point types.
- [`shader_rounding_mode_rtz_float32`] is a boolean value indicating whether an implementation supports the round-towards-zero rounding mode for 32-bit floating-point arithmetic and conversion instructions. It also indicates whether the `RoundingModeRTZ` execution mode  **can**  be used for 32-bit floating-point types.
- [`shader_rounding_mode_rtz_float64`] is a boolean value indicating whether an implementation supports the round-towards-zero rounding mode for 64-bit floating-point arithmetic and conversion instructions. It also indicates whether the `RoundingModeRTZ` execution mode  **can**  be used for 64-bit floating-point types.
If the [`PhysicalDeviceFloatControlsProperties`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceProperties2`] structure passed to
[`get_physical_device_properties2`], it is filled in with each
corresponding implementation-dependent property.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES`

# Related
- [`VK_KHR_shader_float_controls`]
- [`crate::vulkan1_2`]
- [`Bool32`]
- [`ShaderFloatControlsIndependence`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        