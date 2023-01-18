[VkComponentSwizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkComponentSwizzle.html) - Specify how a component is swizzled

# C Specifications
Possible values of the members of [`ComponentMapping`], specifying the
component values placed in each component of the output vector, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkComponentSwizzle {
    VK_COMPONENT_SWIZZLE_IDENTITY = 0,
    VK_COMPONENT_SWIZZLE_ZERO = 1,
    VK_COMPONENT_SWIZZLE_ONE = 2,
    VK_COMPONENT_SWIZZLE_R = 3,
    VK_COMPONENT_SWIZZLE_G = 4,
    VK_COMPONENT_SWIZZLE_B = 5,
    VK_COMPONENT_SWIZZLE_A = 6,
} VkComponentSwizzle;
```

# Description
- [`IDENTITY`] specifies that the component is set to the identity swizzle.
- [`ZERO`] specifies that the component is set to zero.
- [`ONE`] specifies that the component is set to either 1 or 1.0, depending on whether the type of the image view format is integer or floating-point respectively, as determined by the [Format Definition](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-definition) section for each [`Format`].
- [`R`] specifies that the component is set to the value of the R component of the image.
- [`G`] specifies that the component is set to the value of the G component of the image.
- [`B`] specifies that the component is set to the value of the B component of the image.
- [`A`] specifies that the component is set to the value of the A component of the image.
Setting the identity swizzle on a component is equivalent to setting the
identity mapping on that component.
That is:

# Related
- [`crate::vulkan1_0`]
- [`ComponentMapping`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        