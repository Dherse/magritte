[VkMemoryGetAndroidHardwareBufferInfoANDROID](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryGetAndroidHardwareBufferInfoANDROID.html) - Structure describing an Android hardware buffer memory export operation

# C Specifications
The [`MemoryGetAndroidHardwareBufferInfoANDROID`] structure is defined
as:
```c
// Provided by VK_ANDROID_external_memory_android_hardware_buffer
typedef struct VkMemoryGetAndroidHardwareBufferInfoANDROID {
    VkStructureType    sType;
    const void*        pNext;
    VkDeviceMemory     memory;
} VkMemoryGetAndroidHardwareBufferInfoANDROID;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`memory`] is the memory object from which the Android hardware buffer will be exported.

# Description
## Valid Usage
-  `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID` **must**  have been included in [`ExportMemoryAllocateInfo::handle_types`] when [`memory`] was created
-    If the [`p_next`] chain of the [`MemoryAllocateInfo`] used to allocate [`memory`] included a [`MemoryDedicatedAllocateInfo`] with non-`NULL``image` member, then that `image` **must**  already be bound to [`memory`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID`
-  [`p_next`] **must**  be `NULL`
-  [`memory`] **must**  be a valid [`DeviceMemory`] handle

# Related
- [`android_external_memory_android_hardware_buffer`]
- [`DeviceMemory`]
- [`StructureType`]
- [`get_memory_android_hardware_buffer_android`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        