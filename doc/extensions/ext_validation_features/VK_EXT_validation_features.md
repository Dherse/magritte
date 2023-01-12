[VK_EXT_validation_features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_validation_features.html) - instance extension

# Description
This extension provides the [`ValidationFeaturesEXT`] struct that can be
included in the `pNext` chain of the [`InstanceCreateInfo`]
structure passed as the `pCreateInfo` parameter of
[`create_instance`].
The structure contains an array of [`ValidationFeatureEnableEXT`] enum
values that enable specific validation features that are disabled by
default.
The structure also contains an array of [`ValidationFeatureDisableEXT`]
enum values that disable specific validation layer features that are enabled
by default.

# Registered extension number
248

# Revision
5

# Dependencies
- Requires Vulkan 1.0

# Contacts
- Karl Schultz [karl-lunarg](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_validation_features] @karl-lunarg%0A<<Here describe the issue or question you have about the VK_EXT_validation_features extension>>)

# New structures
- Extending [`InstanceCreateInfo`]:  - [`ValidationFeaturesEXT`]

# New enums
- [`ValidationFeatureDisableEXT`]
- [`ValidationFeatureEnableEXT`]

# New constants
- `VK_EXT_VALIDATION_FEATURES_EXTENSION_NAME`
- `VK_EXT_VALIDATION_FEATURES_SPEC_VERSION`
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_VALIDATION_FEATURES_EXT`

# Version history
- Revision 1, 2018-11-14 (Karl Schultz)  - Initial revision 
- Revision 2, 2019-08-06 (Mark Lobodzinski)  - Add Best Practices enable 
- Revision 3, 2020-03-04 (Tony Barbour)  - Add Debug Printf enable 
- Revision 4, 2020-07-29 (John Zulauf)  - Add Synchronization Validation enable 
- Revision 5, 2021-05-18 (Tony Barbour)  - Add Shader Validation Cache disable

# Other information
* 2018-11-14
* No known IP claims.
*   - Karl Schultz, LunarG  - Dave Houlton, LunarG  - Mark Lobodzinski, LunarG  - Camden Stocker, LunarG  - Tony Barbour, LunarG  - John Zulauf, LunarG

# Related
- [`ValidationFeatureDisableEXT`]
- [`ValidationFeatureEnableEXT`]
- [`ValidationFeaturesEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        