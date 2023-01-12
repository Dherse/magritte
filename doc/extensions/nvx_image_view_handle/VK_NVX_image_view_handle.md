[VK_NVX_image_view_handle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NVX_image_view_handle.html) - device extension

# Description
This extension allows applications to query an opaque handle from an image
view for use as a sampled image or storage image.
This provides no direct functionality itself.

# Registered extension number
31

# Revision
2

# Dependencies
- Requires Vulkan 1.0

# Contacts
- Eric Werness [ewerness-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NVX_image_view_handle] @ewerness-nv%0A<<Here describe the issue or question you have about the VK_NVX_image_view_handle extension>>)

# New commands
- [`get_image_view_address_nvx`]
- [`get_image_view_handle_nvx`]

# New structures
- [`ImageViewAddressPropertiesNVX`]
- [`ImageViewHandleInfoNVX`]

# New constants
- `VK_NVX_IMAGE_VIEW_HANDLE_EXTENSION_NAME`
- `VK_NVX_IMAGE_VIEW_HANDLE_SPEC_VERSION`
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_IMAGE_VIEW_ADDRESS_PROPERTIES_NVX`  - `VK_STRUCTURE_TYPE_IMAGE_VIEW_HANDLE_INFO_NVX`

# Version history
- Revision 2, 2020-04-03 (Piers Daniell)  - Add [`get_image_view_address_nvx`] 
- Revision 1, 2018-12-07 (Eric Werness)  - Internal revisions

# Other information
* 2020-04-03
*   - Eric Werness, NVIDIA  - Jeff Bolz, NVIDIA  - Daniel Koch, NVIDIA

# Related
- [`ImageViewAddressPropertiesNVX`]
- [`ImageViewHandleInfoNVX`]
- [`get_image_view_address_nvx`]
- [`get_image_view_handle_nvx`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        