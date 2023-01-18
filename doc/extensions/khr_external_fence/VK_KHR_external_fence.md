[VK_KHR_external_fence](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_fence.html) - device extension

# Description
An application using external memory may wish to synchronize access to that
memory using fences.
This extension enables an application to create fences from which non-Vulkan
handles that reference the underlying synchronization primitive can be
exported.

# Registered extension number
114

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_external_fence_capabilities`]`

# Deprecation state
- *Promoted* to [Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions)

# Contacts
- Jesse Hall [critsec](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_external_fence] @critsec%0A<<Here describe the issue or question you have about the VK_KHR_external_fence extension>>)

# New structures
- Extending [`FenceCreateInfo`]:  - [`ExportFenceCreateInfoKHR`]

# New enums
- [`FenceImportFlagBitsKHR`]

# New bitmasks
- [`FenceImportFlagsKHR`]

# New constants
- [`KHR_EXTERNAL_FENCE_EXTENSION_NAME`]
- [`KHR_EXTERNAL_FENCE_SPEC_VERSION`]
- Extending [`FenceImportFlagBits`]:  - `VK_FENCE_IMPORT_TEMPORARY_BIT_KHR` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO_KHR`

# Known issues & F.A.Q.
This extension borrows concepts, semantics, and language from
`[`VK_KHR_external_semaphore`]`.
That extension’s issues apply equally to this extension.

# Version history
- Revision 1, 2017-05-08 (Jesse Hall)  - Initial revision

# Other information
* 2017-05-08
* No known IP claims.
*   - Promoted to Vulkan 1.1 Core 
*   - Jesse Hall, Google  - James Jones, NVIDIA  - Jeff Juliano, NVIDIA  - Cass Everitt, Oculus  - Contributors to `[`VK_KHR_external_semaphore`]`

# Related
- [`ExportFenceCreateInfoKHR`]
- [`FenceImportFlagBitsKHR`]
- [`FenceImportFlagsKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        