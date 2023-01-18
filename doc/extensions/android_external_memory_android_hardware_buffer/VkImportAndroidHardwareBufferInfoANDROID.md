[VkImportAndroidHardwareBufferInfoANDROID](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportAndroidHardwareBufferInfoANDROID.html) - Import memory from an Android hardware buffer

# C Specifications
To import memory created outside of the current Vulkan instance from an
Android hardware buffer, add a
[`ImportAndroidHardwareBufferInfoANDROID`] structure to the [`p_next`]
chain of the [`MemoryAllocateInfo`] structure.
The [`ImportAndroidHardwareBufferInfoANDROID`] structure is defined as:
```c
// Provided by VK_ANDROID_external_memory_android_hardware_buffer
typedef struct VkImportAndroidHardwareBufferInfoANDROID {
    VkStructureType            sType;
    const void*                pNext;
    struct AHardwareBuffer*    buffer;
} VkImportAndroidHardwareBufferInfoANDROID;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`buffer`] is the Android hardware buffer to import.

# Description
If the [`allocate_memory`] command succeeds, the implementation  **must** 
acquire a reference to the imported hardware buffer, which it  **must**  release
when the device memory object is freed.
If the command fails, the implementation  **must**  not retain a reference.
## Valid Usage
-    If [`buffer`] is not `NULL`, Android hardware buffers  **must**  be supported for import, as reported by [`ExternalImageFormatProperties`] or [`ExternalBufferProperties`]
-    If [`buffer`] is not `NULL`, it  **must**  be a valid Android hardware buffer object with `AHardwareBuffer_Desc`::`usage` compatible with Vulkan as described in [Android Hardware Buffers](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-external-android-hardware-buffer)

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID`
-  [`buffer`] **must**  be a valid pointer to an [`AHardwareBuffer`] value

# Related
- [`VK_ANDROID_external_memory_android_hardware_buffer`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        