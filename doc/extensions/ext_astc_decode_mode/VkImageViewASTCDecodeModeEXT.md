[VkImageViewASTCDecodeModeEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageViewASTCDecodeModeEXT.html) - Structure describing the ASTC decode mode for an image view

# C Specifications
If the [`p_next`] chain includes a [`ImageViewAstcDecodeModeEXT`]
structure, then that structure includes a parameter specifying the decode
mode for image views using ASTC compressed formats.The [`ImageViewAstcDecodeModeEXT`] structure is defined as:
```c
// Provided by VK_EXT_astc_decode_mode
typedef struct VkImageViewASTCDecodeModeEXT {
    VkStructureType    sType;
    const void*        pNext;
    VkFormat           decodeMode;
} VkImageViewASTCDecodeModeEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`decode_mode`] is the intermediate format used to decode ASTC compressed formats.

# Description
## Valid Usage
-  [`decode_mode`] **must**  be one of `VK_FORMAT_R16G16B16A16_SFLOAT`, `VK_FORMAT_R8G8B8A8_UNORM`, or `VK_FORMAT_E5B9G9R9_UFLOAT_PACK32`
-    If the [`decodeModeSharedExponent`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-astc-decodeModeSharedExponent) feature is not enabled, [`decode_mode`] **must**  not be `VK_FORMAT_E5B9G9R9_UFLOAT_PACK32`
-    If [`decode_mode`] is `VK_FORMAT_R8G8B8A8_UNORM` the image view  **must**  not include blocks using any of the ASTC HDR modes
-  `format` of the image view  **must**  be one of the [ASTC Compressed Image Formats](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#appendix-compressedtex-astc)
If `format` uses sRGB encoding then the [`decode_mode`] has no effect.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_VIEW_ASTC_DECODE_MODE_EXT`
-  [`decode_mode`] **must**  be a valid [`Format`] value

# Related
- [`ext_astc_decode_mode`]
- [`Format`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        