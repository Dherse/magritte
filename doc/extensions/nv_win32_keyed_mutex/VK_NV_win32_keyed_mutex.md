[VK_NV_win32_keyed_mutex](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_win32_keyed_mutex.html) - device extension

# Description
Applications that wish to import Direct3D 11 memory objects into the Vulkan
API may wish to use the native keyed mutex mechanism to synchronize access
to the memory between Vulkan and Direct3D.
This extension provides a way for an application to access the keyed mutex
associated with an imported Vulkan memory object when submitting command
buffers to a queue.

# Registered extension number
59

# Revision
2

# Dependencies
- Requires Vulkan 1.0
- Requires `[`nv_external_memory_win32`]`

# Deprecation state
- *Promoted* to `[`khr_win32_keyed_mutex`]` extension

# Contacts
- Carsten Rohde [crohde](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_win32_keyed_mutex] @crohde%0A<<Here describe the issue or question you have about the VK_NV_win32_keyed_mutex extension>>)

# New structures
- Extending [`SubmitInfo`], [`SubmitInfo2`]:  - [`Win32KeyedMutexAcquireReleaseInfoNV`]

# New constants
- `VK_NV_WIN32_KEYED_MUTEX_EXTENSION_NAME`
- `VK_NV_WIN32_KEYED_MUTEX_SPEC_VERSION`
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV`

# Version history
- Revision 2, 2016-08-11 (James Jones)  - Updated sample code based on the NV external memory extensions.  - Renamed from NVX to NV extension.  - Added Overview and Description sections.  - Updated sample code to use the NV external memory extensions. 
- Revision 1, 2016-06-14 (Carsten Rohde)  - Initial draft.

# Other information
* 2016-08-19
* No known IP claims.
*   - James Jones, NVIDIA  - Carsten Rohde, NVIDIA

# Related
- [`Win32KeyedMutexAcquireReleaseInfoNV`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        