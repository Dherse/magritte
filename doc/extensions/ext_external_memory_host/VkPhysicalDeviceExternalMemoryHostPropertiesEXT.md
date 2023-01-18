[VkPhysicalDeviceExternalMemoryHostPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalMemoryHostPropertiesEXT.html) - Structure describing external memory host pointer limits that can be supported by an implementation

# C Specifications
The [`PhysicalDeviceExternalMemoryHostPropertiesEXT`] structure is
defined as:
```c
// Provided by VK_EXT_external_memory_host
typedef struct VkPhysicalDeviceExternalMemoryHostPropertiesEXT {
    VkStructureType    sType;
    void*              pNext;
    VkDeviceSize       minImportedHostPointerAlignment;
} VkPhysicalDeviceExternalMemoryHostPropertiesEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`min_imported_host_pointer_alignment`] is the minimum  **required**  alignment, in bytes, for the base address and size of host pointers that  **can**  be imported to a Vulkan memory object. The value  **must**  be a power of two.

# Description
If the [`PhysicalDeviceExternalMemoryHostPropertiesEXT`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceProperties2`] structure passed to
[`get_physical_device_properties2`], it is filled in with each
corresponding implementation-dependent property.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT`

# Related
- [`VK_EXT_external_memory_host`]
- [`DeviceSize`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        