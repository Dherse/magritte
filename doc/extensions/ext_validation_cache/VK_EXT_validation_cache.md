[VK_EXT_validation_cache](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_validation_cache.html) - device extension

# Description
This extension provides a mechanism for caching the results of potentially
expensive internal validation operations across multiple runs of a Vulkan
application.
At the core is the [`ValidationCacheEXT`] object type, which is managed
similarly to the existing [`PipelineCache`].The new struct [`ShaderModuleValidationCacheCreateInfoEXT`] can be
included in the `pNext` chain at [`create_shader_module`] time.
It contains a [`ValidationCacheEXT`] to use when validating the
[`ShaderModule`].

# Registered extension number
161

# Revision
1

# Dependencies
- Requires Vulkan 1.0

# Contacts
- Cort Stratton [cdwfs](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_validation_cache] @cdwfs%0A<<Here describe the issue or question you have about the VK_EXT_validation_cache extension>>)

# New object types
- [`ValidationCacheEXT`]

# New commands
- [`create_validation_cache_ext`]
- [`destroy_validation_cache_ext`]
- [`get_validation_cache_data_ext`]
- [`merge_validation_caches_ext`]

# New structures
- [`ValidationCacheCreateInfoEXT`]
- Extending [`ShaderModuleCreateInfo`]:  - [`ShaderModuleValidationCacheCreateInfoEXT`]

# New enums
- [`ValidationCacheHeaderVersionEXT`]

# New bitmasks
- [`ValidationCacheCreateFlagsEXT`]

# New constants
- [`EXT_VALIDATION_CACHE_EXTENSION_NAME`]
- [`EXT_VALIDATION_CACHE_SPEC_VERSION`]
- Extending [`ObjectType`]:  - `VK_OBJECT_TYPE_VALIDATION_CACHE_EXT` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT`  - `VK_STRUCTURE_TYPE_VALIDATION_CACHE_CREATE_INFO_EXT`

# Version history
- Revision 1, 2017-08-29 (Cort Stratton)  - Initial draft

# Other information
* 2017-08-29
* No known IP claims.
*   - Cort Stratton, Google  - Chris Forbes, Google

# Related
- [`ShaderModuleValidationCacheCreateInfoEXT`]
- [`ValidationCacheCreateFlagsEXT`]
- [`ValidationCacheCreateInfoEXT`]
- [`ValidationCacheEXT`]
- [`ValidationCacheHeaderVersionEXT`]
- [`create_validation_cache_ext`]
- [`destroy_validation_cache_ext`]
- [`get_validation_cache_data_ext`]
- [`merge_validation_caches_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        