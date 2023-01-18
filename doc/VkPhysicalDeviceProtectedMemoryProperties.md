[VkPhysicalDeviceProtectedMemoryProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProtectedMemoryProperties.html) - Structure describing protected memory properties that can be supported by an implementation

# C Specifications
The [`PhysicalDeviceProtectedMemoryProperties`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkPhysicalDeviceProtectedMemoryProperties {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           protectedNoFault;
} VkPhysicalDeviceProtectedMemoryProperties;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.

# Description
- [`protected_no_fault`] specifies how an implementation behaves when an application attempts to write to unprotected memory in a protected queue operation, read from protected memory in an unprotected queue operation, or perform a query in a protected queue operation. If this limit is [`TRUE`], such writes will be discarded or have undefined values written, reads and queries will return undefined values. If this limit is [`FALSE`], applications  **must**  not perform these operations. See [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-protected-access-rules](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-protected-access-rules) for more information.
If the [`PhysicalDeviceProtectedMemoryProperties`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceProperties2`] structure passed to
[`get_physical_device_properties2`], it is filled in with each
corresponding implementation-dependent property.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES`

# Related
- [`crate::vulkan1_1`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        