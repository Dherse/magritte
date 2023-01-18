[VkBufferConstraintsInfoFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferConstraintsInfoFUCHSIA.html) - Structure buffer-based buffer collection constraints

# C Specifications
The [`BufferConstraintsInfoFUCHSIA`] structure is defined as:
```c
// Provided by VK_FUCHSIA_buffer_collection
typedef struct VkBufferConstraintsInfoFUCHSIA {
    VkStructureType                             sType;
    const void*                                 pNext;
    VkBufferCreateInfo                          createInfo;
    VkFormatFeatureFlags                        requiredFormatFeatures;
    VkBufferCollectionConstraintsInfoFUCHSIA    bufferCollectionConstraints;
} VkBufferConstraintsInfoFUCHSIA;
```

# Members
- [`s_type`] is the type of this structure
- [`p_next`] is `NULL` or a pointer to a structure extending this structure
- `pBufferCreateInfo` a pointer to a [`BufferCreateInfo`] struct describing the buffer attributes for the buffer collection
- [`required_format_features`] bitmask of [`FormatFeatureFlagBits`] required features of the buffers in the buffer collection
- [`buffer_collection_constraints`] is used to supply parameters for the negotiation and allocation of the buffer collection

# Description
## Valid Usage
-    The [`required_format_features`] bitmask of [`FormatFeatureFlagBits`] **must**  be chosen from among the buffer compatible format features listed in [buffer compatible format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#buffer-compatible-format-features)

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BUFFER_CONSTRAINTS_INFO_FUCHSIA`
-  [`p_next`] **must**  be `NULL`
-  [`create_info`] **must**  be a valid [`BufferCreateInfo`] structure
-  [`required_format_features`] **must**  be a valid combination of [`FormatFeatureFlagBits`] values
-  [`buffer_collection_constraints`] **must**  be a valid [`BufferCollectionConstraintsInfoFUCHSIA`] structure

# Related
- [`VK_FUCHSIA_buffer_collection`]
- [`BufferCollectionConstraintsInfoFUCHSIA`]
- [`BufferCreateInfo`]
- [`FormatFeatureFlags`]
- [`StructureType`]
- [`set_buffer_collection_buffer_constraints_fuchsia`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        