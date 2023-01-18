[VK_INTEL_performance_query](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_INTEL_performance_query.html) - device extension

# Description
This extension allows an application to capture performance data to be
interpreted by a external application or library.Such a library is available at : [https://github.com/intel/metrics-discovery](https://github.com/intel/metrics-discovery)Performance analysis tools such as
[Graphics
Performance Analyzers](https://software.intel.com/content/www/us/en/develop/tools/graphics-performance-analyzers.html) make use of this extension and the metrics-discovery
library to present the data in a human readable way.

# Registered extension number
211

# Revision
2

# Dependencies
- Requires Vulkan 1.0

# Contacts
- Lionel Landwerlin [llandwerlin](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_INTEL_performance_query] @llandwerlin%0A<<Here describe the issue or question you have about the VK_INTEL_performance_query extension>>)

# New object types
- [`PerformanceConfigurationINTEL`]

# New commands
- [`acquire_performance_configuration_intel`]
- [`cmd_set_performance_marker_intel`]
- [`cmd_set_performance_override_intel`]
- [`cmd_set_performance_stream_marker_intel`]
- [`get_performance_parameter_intel`]
- [`initialize_performance_api_intel`]
- [`queue_set_performance_configuration_intel`]
- [`release_performance_configuration_intel`]
- [`uninitialize_performance_api_intel`]

# New structures
- [`InitializePerformanceApiInfoINTEL`]
- [`PerformanceConfigurationAcquireInfoINTEL`]
- [`PerformanceMarkerInfoINTEL`]
- [`PerformanceOverrideInfoINTEL`]
- [`PerformanceStreamMarkerInfoINTEL`]
- [`PerformanceValueINTEL`]
- Extending [`QueryPoolCreateInfo`]:  - [`QueryPoolCreateInfoINTEL`]  - [`QueryPoolPerformanceQueryCreateInfoINTEL`]

# New unions
- [`PerformanceValueDataINTEL`]

# New enums
- [`PerformanceConfigurationTypeINTEL`]
- [`PerformanceOverrideTypeINTEL`]
- [`PerformanceParameterTypeINTEL`]
- [`PerformanceValueTypeINTEL`]
- [`QueryPoolSamplingModeINTEL`]

# New constants
- [`INTEL_PERFORMANCE_QUERY_EXTENSION_NAME`]
- [`INTEL_PERFORMANCE_QUERY_SPEC_VERSION`]
- Extending [`ObjectType`]:  - `VK_OBJECT_TYPE_PERFORMANCE_CONFIGURATION_INTEL` 
- Extending [`QueryType`]:  - `VK_QUERY_TYPE_PERFORMANCE_QUERY_INTEL` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_INITIALIZE_PERFORMANCE_API_INFO_INTEL`  - `VK_STRUCTURE_TYPE_PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL`  - `VK_STRUCTURE_TYPE_PERFORMANCE_MARKER_INFO_INTEL`  - `VK_STRUCTURE_TYPE_PERFORMANCE_OVERRIDE_INFO_INTEL`  - `VK_STRUCTURE_TYPE_PERFORMANCE_STREAM_MARKER_INFO_INTEL`  - `VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO_INTEL`  - `VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL`

# Version history
- Revision 2, 2020-03-06 (Lionel Landwerlin)  - Rename VkQueryPoolCreateInfoINTEL in VkQueryPoolPerformanceQueryCreateInfoINTEL 
- Revision 1, 2018-05-16 (Lionel Landwerlin)  - Initial revision

# Other information
* 2018-05-16
* No known IP claims.
*   - Lionel Landwerlin, Intel  - Piotr Maciejewski, Intel

# Related
- [`InitializePerformanceApiInfoINTEL`]
- [`PerformanceConfigurationAcquireInfoINTEL`]
- [`PerformanceConfigurationINTEL`]
- [`PerformanceConfigurationTypeINTEL`]
- [`PerformanceMarkerInfoINTEL`]
- [`PerformanceOverrideInfoINTEL`]
- [`PerformanceOverrideTypeINTEL`]
- [`PerformanceParameterTypeINTEL`]
- [`PerformanceStreamMarkerInfoINTEL`]
- [`PerformanceValueDataINTEL`]
- [`PerformanceValueINTEL`]
- [`PerformanceValueTypeINTEL`]
- [`QueryPoolCreateInfoINTEL`]
- [`QueryPoolPerformanceQueryCreateInfoINTEL`]
- [`QueryPoolSamplingModeINTEL`]
- [`acquire_performance_configuration_intel`]
- [`cmd_set_performance_marker_intel`]
- [`cmd_set_performance_override_intel`]
- [`cmd_set_performance_stream_marker_intel`]
- [`get_performance_parameter_intel`]
- [`initialize_performance_api_intel`]
- [`queue_set_performance_configuration_intel`]
- [`release_performance_configuration_intel`]
- [`uninitialize_performance_api_intel`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        