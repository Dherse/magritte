[VK_KHR_external_memory_win32](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_memory_win32.html) - device extension

# Description
An application may wish to reference device memory in multiple Vulkan
logical devices or instances, in multiple processes, and/or in multiple
APIs.
This extension enables an application to export Windows handles from Vulkan
memory objects and to import Vulkan memory objects from Windows handles
exported from other Vulkan memory objects or from similar resources in other
APIs.

# Registered extension number
74

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_external_memory`]`

# Contacts
- James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_external_memory_win32] @cubanismo%0A<<Here describe the issue or question you have about the VK_KHR_external_memory_win32 extension>>)

# New commands
- [`get_memory_win32_handle_khr`]
- [`get_memory_win32_handle_properties_khr`]

# New structures
- [`MemoryGetWin32HandleInfoKHR`]
- [`MemoryWin32HandlePropertiesKHR`]
- Extending [`MemoryAllocateInfo`]:  - [`ExportMemoryWin32HandleInfoKHR`]  - [`ImportMemoryWin32HandleInfoKHR`]

# New constants
- [`KHR_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME`]
- [`KHR_EXTERNAL_MEMORY_WIN32_SPEC_VERSION`]
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR`  - `VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR`  - `VK_STRUCTURE_TYPE_MEMORY_GET_WIN32_HANDLE_INFO_KHR`  - `VK_STRUCTURE_TYPE_MEMORY_WIN32_HANDLE_PROPERTIES_KHR`

# Known issues & F.A.Q.
1) Do applications need to call `CloseHandle`() on the values returned
from [`get_memory_win32_handle_khr`] when `handleType` is
`VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR`? **RESOLVED** : Yes, unless it is passed back in to another driver instance to
import the object.
A successful get call transfers ownership of the handle to the application.
Destroying the memory object will not destroy the handle or the handle’s
reference to the underlying memory resource.2) Should the language regarding KMT/Windows 7 handles be moved to a
separate extension so that it can be deprecated over time? **RESOLVED** : No.
Support for them can be deprecated by drivers if they choose, by no longer
returning them in the supported handle types of the instance level queries.3) How should the valid size and memory type for windows memory handles
created outside of Vulkan be specified? **RESOLVED** : The valid memory types are queried directly from the external
handle.
The size is determined by the associated image or buffer memory requirements
for external handle types that require dedicated allocations, and by the
size specified when creating the object from which the handle was exported
for other external handle types.

# Version history
- Revision 1, 2016-10-21 (James Jones)  - Initial revision

# Other information
* 2016-10-21
* No known IP claims.
*   - James Jones, NVIDIA  - Jeff Juliano, NVIDIA  - Carsten Rohde, NVIDIA

# Related
- [`ExportMemoryWin32HandleInfoKHR`]
- [`ImportMemoryWin32HandleInfoKHR`]
- [`MemoryGetWin32HandleInfoKHR`]
- [`MemoryWin32HandlePropertiesKHR`]
- [`get_memory_win32_handle_khr`]
- [`get_memory_win32_handle_properties_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        