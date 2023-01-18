[VK_KHR_external_semaphore_win32](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_semaphore_win32.html) - device extension

# Description
An application using external memory may wish to synchronize access to that
memory using semaphores.
This extension enables an application to export semaphore payload to and
import semaphore payload from Windows handles.

# Registered extension number
79

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_external_semaphore`]`

# Contacts
- James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_external_semaphore_win32] @cubanismo%0A<<Here describe the issue or question you have about the VK_KHR_external_semaphore_win32 extension>>)

# New commands
- [`get_semaphore_win32_handle_khr`]
- [`import_semaphore_win32_handle_khr`]

# New structures
- [`ImportSemaphoreWin32HandleInfoKHR`]
- [`SemaphoreGetWin32HandleInfoKHR`]
- Extending [`SemaphoreCreateInfo`]:  - [`ExportSemaphoreWin32HandleInfoKHR`] 
- Extending [`SubmitInfo`]:  - [`D3d12FenceSubmitInfoKHR`]

# New constants
- [`KHR_EXTERNAL_SEMAPHORE_WIN32_EXTENSION_NAME`]
- [`KHR_EXTERNAL_SEMAPHORE_WIN32_SPEC_VERSION`]
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_D3D12_FENCE_SUBMIT_INFO_KHR`  - `VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR`  - `VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR`  - `VK_STRUCTURE_TYPE_SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR`

# Known issues & F.A.Q.
1) Do applications need to call `CloseHandle`() on the values returned
from [`get_semaphore_win32_handle_khr`] when `handleType` is
`VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR`? **RESOLVED** : Yes, unless it is passed back in to another driver instance to
import the object.
A successful get call transfers ownership of the handle to the application.
Destroying the semaphore object will not destroy the handle or the handle’s
reference to the underlying semaphore resource.2) Should the language regarding KMT/Windows 7 handles be moved to a
separate extension so that it can be deprecated over time? **RESOLVED** : No.
Support for them can be deprecated by drivers if they choose, by no longer
returning them in the supported handle types of the instance level queries.3) Should applications be allowed to specify additional object attributes
for shared handles? **RESOLVED** : Yes.
Applications will be allowed to provide similar attributes to those they
would to any other handle creation API.4) How do applications communicate the desired fence values to use with
`D3D12_FENCE`-based Vulkan semaphores? **RESOLVED** : There are a couple of options.
The values for the signaled and reset states could be communicated up front
when creating the object and remain static for the life of the Vulkan
semaphore, or they could be specified using auxiliary structures when
submitting semaphore signal and wait operations, similar to what is done
with the keyed mutex extensions.
The latter is more flexible and consistent with the keyed mutex usage, but
the former is a much simpler API.Since Vulkan tends to favor flexibility and consistency over simplicity, a
new structure specifying D3D12 fence acquire and release values is added to
the [`queue_submit`] function.

# Version history
- Revision 1, 2016-10-21 (James Jones)  - Initial revision

# Other information
* 2016-10-21
* No known IP claims.
*   - James Jones, NVIDIA  - Jeff Juliano, NVIDIA  - Carsten Rohde, NVIDIA

# Related
- [`D3d12FenceSubmitInfoKHR`]
- [`ExportSemaphoreWin32HandleInfoKHR`]
- [`ImportSemaphoreWin32HandleInfoKHR`]
- [`SemaphoreGetWin32HandleInfoKHR`]
- [`get_semaphore_win32_handle_khr`]
- [`import_semaphore_win32_handle_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        