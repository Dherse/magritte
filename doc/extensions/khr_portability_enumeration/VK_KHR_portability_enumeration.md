[VK_KHR_portability_enumeration](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_portability_enumeration.html) - instance extension

# Description
This extension allows applications to control whether devices that expose
the `[`VK_KHR_portability_subset`]` extension are included in the results
of physical device enumeration.
Since devices which support the `[`VK_KHR_portability_subset`]` extension
are not fully conformant Vulkan implementations, the Vulkan loader does not
report those devices unless the application explicitly asks for them.
This prevents applications which may not be aware of non-conformant devices
from accidentally using them, as any device which supports the
`[`VK_KHR_portability_subset`]` extension mandates that the extension
must be enabled if that device is used.This extension is implemented in the loader.

# Registered extension number
395

# Revision
1

# Dependencies
- Requires Vulkan 1.0

# Contacts
- Charles Giessen [charles-lunarg](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_portability_enumeration] @charles-lunarg%0A<<Here describe the issue or question you have about the VK_KHR_portability_enumeration extension>>)

# New constants
- [`KHR_PORTABILITY_ENUMERATION_EXTENSION_NAME`]
- [`KHR_PORTABILITY_ENUMERATION_SPEC_VERSION`]
- Extending [`InstanceCreateFlagBits`]:  - `VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR`

# Version history
- Revision 1, 2021-06-02 (Lenny Komow)  - Initial version

# Other information
* 2021-06-02
* No known IP claims.
*   - Interacts with `[`VK_KHR_portability_subset`]` 
*   - Lenny Komow, LunarG  - Charles Giessen, LunarG
# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        