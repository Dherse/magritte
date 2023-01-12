[VK_KHR_external_memory_fd](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_memory_fd.html) - device extension

# Description
An application may wish to reference device memory in multiple Vulkan
logical devices or instances, in multiple processes, and/or in multiple
APIs.
This extension enables an application to export POSIX file descriptor
handles from Vulkan memory objects and to import Vulkan memory objects from
POSIX file descriptor handles exported from other Vulkan memory objects or
from similar resources in other APIs.

# Registered extension number
75

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`khr_external_memory`]`

# Contacts
- James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_external_memory_fd] @cubanismo%0A<<Here describe the issue or question you have about the VK_KHR_external_memory_fd extension>>)

# New commands
- [`get_memory_fd_khr`]
- [`get_memory_fd_properties_khr`]

# New structures
- [`MemoryFdPropertiesKHR`]
- [`MemoryGetFdInfoKHR`]
- Extending [`MemoryAllocateInfo`]:  - [`ImportMemoryFdInfoKHR`]

# New constants
- `VK_KHR_EXTERNAL_MEMORY_FD_EXTENSION_NAME`
- `VK_KHR_EXTERNAL_MEMORY_FD_SPEC_VERSION`
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_IMPORT_MEMORY_FD_INFO_KHR`  - `VK_STRUCTURE_TYPE_MEMORY_FD_PROPERTIES_KHR`  - `VK_STRUCTURE_TYPE_MEMORY_GET_FD_INFO_KHR`

# Known issues & F.A.Q.
1) Does the application need to close the file descriptor returned by
[`get_memory_fd_khr`]? **RESOLVED** : Yes, unless it is passed back in to a driver instance to import
the memory.
A successful get call transfers ownership of the file descriptor to the
application, and a successful import transfers it back to the driver.
Destroying the original memory object will not close the file descriptor or
remove its reference to the underlying memory resource associated with it.2) Do drivers ever need to expose multiple file descriptors per memory
object? **RESOLVED** : No.
This would indicate there are actually multiple memory objects, rather than
a single memory object.3) How should the valid size and memory type for POSIX file descriptor
memory handles created outside of Vulkan be specified? **RESOLVED** : The valid memory types are queried directly from the external
handle.
The size will be specified by future extensions that introduce such external
memory handle types.

# Version history
- Revision 1, 2016-10-21 (James Jones)  - Initial revision

# Other information
* 2016-10-21
* No known IP claims.
*   - James Jones, NVIDIA  - Jeff Juliano, NVIDIA

# Related
- [`ImportMemoryFdInfoKHR`]
- [`MemoryFdPropertiesKHR`]
- [`MemoryGetFdInfoKHR`]
- [`get_memory_fd_khr`]
- [`get_memory_fd_properties_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        