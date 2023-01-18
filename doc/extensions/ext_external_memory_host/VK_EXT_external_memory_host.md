[VK_EXT_external_memory_host](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_external_memory_host.html) - device extension

# Description
This extension enables an application to import host allocations and host
mapped foreign device memory to Vulkan memory objects.

# Registered extension number
179

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_external_memory`]`

# Contacts
- Daniel Rakos [drakos-amd](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_external_memory_host] @drakos-amd%0A<<Here describe the issue or question you have about the VK_EXT_external_memory_host extension>>)

# New commands
- [`get_memory_host_pointer_properties_ext`]

# New structures
- [`MemoryHostPointerPropertiesEXT`]
- Extending [`MemoryAllocateInfo`]:  - [`ImportMemoryHostPointerInfoEXT`] 
- Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceExternalMemoryHostPropertiesEXT`]

# New constants
- [`EXT_EXTERNAL_MEMORY_HOST_EXTENSION_NAME`]
- [`EXT_EXTERNAL_MEMORY_HOST_SPEC_VERSION`]
- Extending [`ExternalMemoryHandleTypeFlagBits`]:  - `VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT`  - `VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_IMPORT_MEMORY_HOST_POINTER_INFO_EXT`  - `VK_STRUCTURE_TYPE_MEMORY_HOST_POINTER_PROPERTIES_EXT`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT`

# Known issues & F.A.Q.
1) What memory type has to be used to import host pointers? **RESOLVED** : Depends on the implementation.
Applications have to use the new [`get_memory_host_pointer_properties_ext`]
command to query the supported memory types for a particular host pointer.
The reported memory types may include memory types that come from a memory
heap that is otherwise not usable for regular memory object allocation and
thus such a heap’s size may be zero.2) Can the application still access the contents of the host allocation
after importing? **RESOLVED** : Yes.
However, usual synchronization requirements apply.3) Can the application free the host allocation? **RESOLVED** : No, it violates valid usage conditions.
Using the memory object imported from a host allocation that is already
freed thus results in undefined behavior.4) Is [`map_memory`] expected to return the same host address which was
specified when importing it to the memory object? **RESOLVED** : No.
Implementations are allowed to return the same address but it is not
required.
Some implementations might return a different virtual mapping of the
allocation, although the same physical pages will be used.5) Is there any limitation on the alignment of the host pointer and/or size? **RESOLVED** : Yes.
Both the address and the size have to be an integer multiple of
`minImportedHostPointerAlignment`.
In addition, some platforms and foreign devices may have additional
restrictions.6) Can the same host allocation be imported multiple times into a given
physical device? **RESOLVED** : No, at least not guaranteed by this extension.
Some platforms do not allow locking the same physical pages for device
access multiple times, so attempting to do it may result in undefined
behavior.7) Does this extension support exporting the new handle type? **RESOLVED** : No.8) Should we include the possibility to import host mapped foreign device
memory using this API? **RESOLVED** : Yes, through a separate handle type.
Implementations are still allowed to support only one of the handle types
introduced by this extension by not returning import support for a
particular handle type as returned in [`ExternalMemoryPropertiesKHR`].

# Version history
- Revision 1, 2017-11-10 (Daniel Rakos)  - Internal revisions

# Other information
* 2017-11-10
* No known IP claims.
*   - Jaakko Konttinen, AMD  - David Mao, AMD  - Daniel Rakos, AMD  - Tobias Hector, Imagination Technologies  - Jason Ekstrand, Intel  - James Jones, NVIDIA

# Related
- [`ImportMemoryHostPointerInfoEXT`]
- [`MemoryHostPointerPropertiesEXT`]
- [`PhysicalDeviceExternalMemoryHostPropertiesEXT`]
- [`get_memory_host_pointer_properties_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        