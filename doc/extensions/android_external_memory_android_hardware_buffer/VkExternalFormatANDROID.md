[VkExternalFormatANDROID](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalFormatANDROID.html) - Structure containing an Android hardware buffer external format

# C Specifications
To create an image with an
[external
format](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-external-android-hardware-buffer-external-formats), add a [`ExternalFormatANDROID`] structure in the [`p_next`]
chain of [`ImageCreateInfo`].
[`ExternalFormatANDROID`] is defined as:
```c
// Provided by VK_ANDROID_external_memory_android_hardware_buffer
typedef struct VkExternalFormatANDROID {
    VkStructureType    sType;
    void*              pNext;
    uint64_t           externalFormat;
} VkExternalFormatANDROID;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`external_format`] is an implementation-defined identifier for the external format

# Description
If [`external_format`] is zero, the effect is as if the
[`ExternalFormatANDROID`] structure was not present.
Otherwise, the `image` will have the specified external format.
## Valid Usage
-  [`external_format`] **must**  be `0` or a value returned in the [`external_format`] member of [`AndroidHardwareBufferFormatPropertiesANDROID`] by an earlier call to [`get_android_hardware_buffer_properties_android`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_EXTERNAL_FORMAT_ANDROID`

# Related
- [`android_external_memory_android_hardware_buffer`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        