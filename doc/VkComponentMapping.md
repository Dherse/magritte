[VkComponentMapping](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkComponentMapping.html) - Structure specifying a color component mapping

# C Specifications
The [`ComponentMapping`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkComponentMapping {
    VkComponentSwizzle    r;
    VkComponentSwizzle    g;
    VkComponentSwizzle    b;
    VkComponentSwizzle    a;
} VkComponentMapping;
```

# Members
- [`r`] is a [`ComponentSwizzle`] specifying the component value placed in the R component of the output vector.
- [`g`] is a [`ComponentSwizzle`] specifying the component value placed in the G component of the output vector.
- [`b`] is a [`ComponentSwizzle`] specifying the component value placed in the B component of the output vector.
- [`a`] is a [`ComponentSwizzle`] specifying the component value placed in the A component of the output vector.

# Description
## Valid Usage (Implicit)
-  [`r`] **must**  be a valid [`ComponentSwizzle`] value
-  [`g`] **must**  be a valid [`ComponentSwizzle`] value
-  [`b`] **must**  be a valid [`ComponentSwizzle`] value
-  [`a`] **must**  be a valid [`ComponentSwizzle`] value

# Related
- [`crate::vulkan1_0`]
- [`AndroidHardwareBufferFormatProperties2ANDROID`]
- [`AndroidHardwareBufferFormatPropertiesANDROID`]
- [`BufferCollectionPropertiesFUCHSIA`]
- [`ComponentSwizzle`]
- [`ImageViewCreateInfo`]
- [`SamplerBorderColorComponentMappingCreateInfoEXT`]
- [`SamplerYcbcrConversionCreateInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        