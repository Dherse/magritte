[VK_KHR_external_semaphore_fd](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_semaphore_fd.html) - device extension

# Description
An application using external memory may wish to synchronize access to that
memory using semaphores.
This extension enables an application to export semaphore payload to and
import semaphore payload from POSIX file descriptors.

# Registered extension number
80

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`khr_external_semaphore`]`

# Contacts
- James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_external_semaphore_fd] @cubanismo%0A<<Here describe the issue or question you have about the VK_KHR_external_semaphore_fd extension>>)

# New commands
- [`get_semaphore_fd_khr`]
- [`import_semaphore_fd_khr`]

# New structures
- [`ImportSemaphoreFdInfoKHR`]
- [`SemaphoreGetFdInfoKHR`]

# New constants
- `VK_KHR_EXTERNAL_SEMAPHORE_FD_EXTENSION_NAME`
- `VK_KHR_EXTERNAL_SEMAPHORE_FD_SPEC_VERSION`
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_FD_INFO_KHR`  - `VK_STRUCTURE_TYPE_SEMAPHORE_GET_FD_INFO_KHR`

# Known issues & F.A.Q.
1) Does the application need to close the file descriptor returned by
[`get_semaphore_fd_khr`]? **RESOLVED** : Yes, unless it is passed back in to a driver instance to import
the semaphore.
A successful get call transfers ownership of the file descriptor to the
application, and a successful import transfers it back to the driver.
Destroying the original semaphore object will not close the file descriptor
or remove its reference to the underlying semaphore resource associated with
it.

# Version history
- Revision 1, 2016-10-21 (Jesse Hall)  - Initial revision

# Other information
* 2016-10-21
* No known IP claims.
*   - Jesse Hall, Google  - James Jones, NVIDIA  - Jeff Juliano, NVIDIA  - Carsten Rohde, NVIDIA

# Related
- [`ImportSemaphoreFdInfoKHR`]
- [`SemaphoreGetFdInfoKHR`]
- [`get_semaphore_fd_khr`]
- [`import_semaphore_fd_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        