[VkAndroidHardwareBufferPropertiesANDROID](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAndroidHardwareBufferPropertiesANDROID.html) - Properties of External Memory Android Hardware Buffers

# C Specifications
The [`AndroidHardwareBufferPropertiesANDROID`] structure returned is
defined as:
```c
// Provided by VK_ANDROID_external_memory_android_hardware_buffer
typedef struct VkAndroidHardwareBufferPropertiesANDROID {
    VkStructureType    sType;
    void*              pNext;
    VkDeviceSize       allocationSize;
    uint32_t           memoryTypeBits;
} VkAndroidHardwareBufferPropertiesANDROID;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`allocation_size`] is the size of the external memory
- [`memory_type_bits`] is a bitmask containing one bit set for every memory type which the specified Android hardware buffer  **can**  be imported as.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID`
-    Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**  be either `NULL` or a pointer to a valid instance of [`AndroidHardwareBufferFormatProperties2ANDROID`] or [`AndroidHardwareBufferFormatPropertiesANDROID`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique

# Related
- [`android_external_memory_android_hardware_buffer`]
- [`DeviceSize`]
- [`StructureType`]
- [`get_android_hardware_buffer_properties_android`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        