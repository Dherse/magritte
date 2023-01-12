[VkPushConstantRange](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPushConstantRange.html) - Structure specifying a push constant range

# C Specifications
The [`PushConstantRange`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkPushConstantRange {
    VkShaderStageFlags    stageFlags;
    uint32_t              offset;
    uint32_t              size;
} VkPushConstantRange;
```

# Members
- [`stage_flags`] is a set of stage flags describing the shader stages that will access a range of push constants. If a particular stage is not included in the range, then accessing members of that range of push constants from the corresponding shader stage will return undefined values.
- [`offset`] and [`size`] are the start offset and size, respectively, consumed by the range. Both [`offset`] and [`size`] are in units of bytes and  **must**  be a multiple of 4. The layout of the push constant variables is specified in the shader.

# Description
## Valid Usage
-  [`offset`] **must**  be less than [`PhysicalDeviceLimits::max_push_constants_size`]
-  [`offset`] **must**  be a multiple of `4`
-  [`size`] **must**  be greater than `0`
-  [`size`] **must**  be a multiple of `4`
-  [`size`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_push_constants_size`] minus [`offset`]

## Valid Usage (Implicit)
-  [`stage_flags`] **must**  be a valid combination of [`ShaderStageFlagBits`] values
-  [`stage_flags`] **must**  not be `0`

# Related
- [`crate::vulkan1_0`]
- [`PipelineLayoutCreateInfo`]
- [VkShaderStageFlags]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        