[VkTextureLODGatherFormatPropertiesAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkTextureLODGatherFormatPropertiesAMD.html) - Structure informing whether or not texture gather bias/LOD functionality is supported for a given image format and a given physical device.

# C Specifications
To determine if texture gather functions that take explicit LOD and/or bias
argument values  **can**  be used with a given image format, add a
[`TextureLodGatherFormatPropertiesAMD`] structure to the [`p_next`]
chain of the [`ImageFormatProperties2`] structure in a call to
[`get_physical_device_image_format_properties2`].The [`TextureLodGatherFormatPropertiesAMD`] structure is defined as:
```c
// Provided by VK_AMD_texture_gather_bias_lod
typedef struct VkTextureLODGatherFormatPropertiesAMD {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           supportsTextureGatherLODBiasAMD;
} VkTextureLODGatherFormatPropertiesAMD;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`supports_texture_gather_lod_bias_amd`] tells if the image format can be used with texture gather bias/LOD functions, as introduced by the `[`VK_AMD_texture_gather_bias_lod`]` extension. This field is set by the implementation. User-specified value is ignored.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD`

# Related
- [`VK_AMD_texture_gather_bias_lod`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        