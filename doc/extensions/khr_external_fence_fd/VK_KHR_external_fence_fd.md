[VK_KHR_external_fence_fd](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_fence_fd.html) - device extension

# Description
An application using external memory may wish to synchronize access to that
memory using fences.
This extension enables an application to export fence payload to and import
fence payload from POSIX file descriptors.

# Registered extension number
116

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`khr_external_fence`]`

# Contacts
- Jesse Hall [critsec](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_external_fence_fd] @critsec%0A<<Here describe the issue or question you have about the VK_KHR_external_fence_fd extension>>)

# New commands
- [`get_fence_fd_khr`]
- [`import_fence_fd_khr`]

# New structures
- [`FenceGetFdInfoKHR`]
- [`ImportFenceFdInfoKHR`]

# New constants
- `VK_KHR_EXTERNAL_FENCE_FD_EXTENSION_NAME`
- `VK_KHR_EXTERNAL_FENCE_FD_SPEC_VERSION`
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_FENCE_GET_FD_INFO_KHR`  - `VK_STRUCTURE_TYPE_IMPORT_FENCE_FD_INFO_KHR`

# Known issues & F.A.Q.
This extension borrows concepts, semantics, and language from
`[`khr_external_semaphore_fd`]`.
That extension’s issues apply equally to this extension.

# Version history
- Revision 1, 2017-05-08 (Jesse Hall)  - Initial revision

# Other information
* 2017-05-08
* No known IP claims.
*   - Jesse Hall, Google  - James Jones, NVIDIA  - Jeff Juliano, NVIDIA  - Cass Everitt, Oculus  - Contributors to `[`khr_external_semaphore_fd`]`

# Related
- [`FenceGetFdInfoKHR`]
- [`ImportFenceFdInfoKHR`]
- [`get_fence_fd_khr`]
- [`import_fence_fd_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        