[VK_EXT_validation_flags](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_validation_flags.html) - instance extension

# Description
This extension provides the [`ValidationFlagsEXT`] struct that can be
included in the `pNext` chain of the [`InstanceCreateInfo`]
structure passed as the `pCreateInfo` parameter of
[`create_instance`].
The structure contains an array of [`ValidationCheckEXT`] values that
will be disabled by the validation layers.

# Registered extension number
62

# Revision
2

# Dependencies
- Requires Vulkan 1.0

# Deprecation state
- *Deprecated* by `[`ext_validation_features`]` extension

# Contacts
- Tobin Ehlis [tobine](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_validation_flags] @tobine%0A<<Here describe the issue or question you have about the VK_EXT_validation_flags extension>>)

# New structures
- Extending [`InstanceCreateInfo`]:  - [`ValidationFlagsEXT`]

# New enums
- [`ValidationCheckEXT`]

# New constants
- `VK_EXT_VALIDATION_FLAGS_EXTENSION_NAME`
- `VK_EXT_VALIDATION_FLAGS_SPEC_VERSION`
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_VALIDATION_FLAGS_EXT`

# Version history
- Revision 2, 2019-08-19 (Mark Lobodzinski)  - Marked as deprecated 
- Revision 1, 2016-08-26 (Courtney Goeltzenleuchter)  - Initial draft

# Other information
* 2019-08-19
* No known IP claims.
*   - Tobin Ehlis, Google  - Courtney Goeltzenleuchter, Google

# Related
- [`ValidationCheckEXT`]
- [`ValidationFlagsEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        