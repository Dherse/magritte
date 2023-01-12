[VK_NV_external_memory_win32](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_external_memory_win32.html) - device extension

# Description
Applications may wish to export memory to other Vulkan instances or other
APIs, or import memory from other Vulkan instances or other APIs to enable
Vulkan workloads to be split up across application module, process, or API
boundaries.
This extension enables win32 applications to export win32 handles from
Vulkan memory objects such that the underlying resources can be referenced
outside the Vulkan instance that created them, and import win32 handles
created in the Direct3D API to Vulkan memory objects.

# Registered extension number
58

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`nv_external_memory`]`

# Deprecation state
- *Deprecated* by `[`khr_external_memory_win32`]` extension

# Contacts
- James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_external_memory_win32] @cubanismo%0A<<Here describe the issue or question you have about the VK_NV_external_memory_win32 extension>>)

# New commands
- [`get_memory_win32_handle_nv`]

# New structures
- Extending [`MemoryAllocateInfo`]:  - [`ExportMemoryWin32HandleInfoNV`]  - [`ImportMemoryWin32HandleInfoNV`]

# New constants
- `VK_NV_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME`
- `VK_NV_EXTERNAL_MEMORY_WIN32_SPEC_VERSION`
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_NV`  - `VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_NV`

# Known issues & F.A.Q.
1) If memory objects are shared between processes and APIs, is this
considered aliasing according to the rules outlined in the
[Memory Aliasing](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-memory-aliasing) section? **RESOLVED** : Yes, but strict exceptions to the rules are added to allow some
forms of aliasing in these cases.
Further, other extensions may build upon these new aliasing rules to define
specific support usage within Vulkan for imported native memory objects, or
memory objects from other APIs.2) Are new image layouts or metadata required to specify image layouts and
layout transitions compatible with non-Vulkan APIs, or with other instances
of the same Vulkan driver? **RESOLVED** : No.
Separate instances of the same Vulkan driver running on the same GPU should
have identical internal layout semantics, so applictions have the tools they
need to ensure views of images are consistent between the two instances.
Other APIs will fall into two categories: Those that are Vulkan compatible
(a term to be defined by subsequent interopability extensions), or Vulkan
incompatible.
When sharing images with Vulkan incompatible APIs, the Vulkan image must be
transitioned to the `VK_IMAGE_LAYOUT_GENERAL` layout before handing it
off to the external API.Note this does not attempt to address cross-device transitions, nor
transitions to engines on the same device which are not visible within the
Vulkan API.
Both of these are beyond the scope of this extension.3) Do applications need to call `CloseHandle`() on the values returned
from [`get_memory_win32_handle_nv`] when `handleType` is
`VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_NV`? **RESOLVED** : Yes, unless it is passed back in to another driver instance to
import the object.
A successful get call transfers ownership of the handle to the application,
while an import transfers ownership to the associated driver.
Destroying the memory object will not destroy the handle or the handle’s
reference to the underlying memory resource.

# Version history
- Revision 1, 2016-08-11 (James Jones)  - Initial draft

# Other information
* 2016-08-19
* No known IP claims.
*   - James Jones, NVIDIA  - Carsten Rohde, NVIDIA

# Related
- [`ExportMemoryWin32HandleInfoNV`]
- [`ImportMemoryWin32HandleInfoNV`]
- [`get_memory_win32_handle_nv`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        