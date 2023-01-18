[VK_KHR_pipeline_executable_properties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_pipeline_executable_properties.html) - device extension

# Description
When a pipeline is created, its state and shaders are compiled into zero or
more device-specific executables, which are used when executing commands
against that pipeline.
This extension adds a mechanism to query properties and statistics about the
different executables produced by the pipeline compilation process.
This is intended to be used by debugging and performance tools to allow them
to provide more detailed information to the user.
Certain compile-time shader statistics provided through this extension may
be useful to developers for debugging or performance analysis.

# Registered extension number
270

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_get_physical_device_properties2`]`

# Contacts
- Jason Ekstrand [jekstrand](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_pipeline_executable_properties] @jekstrand%0A<<Here describe the issue or question you have about the VK_KHR_pipeline_executable_properties extension>>)

# New commands
- [`get_pipeline_executable_internal_representations_khr`]
- [`get_pipeline_executable_properties_khr`]
- [`get_pipeline_executable_statistics_khr`]

# New structures
- [`PipelineExecutableInfoKHR`]
- [`PipelineExecutableInternalRepresentationKHR`]
- [`PipelineExecutablePropertiesKHR`]
- [`PipelineExecutableStatisticKHR`]
- [`PipelineInfoKHR`]
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDevicePipelineExecutablePropertiesFeaturesKHR`]

# New unions
- [`PipelineExecutableStatisticValueKHR`]

# New enums
- [`PipelineExecutableStatisticFormatKHR`]

# New constants
- [`KHR_PIPELINE_EXECUTABLE_PROPERTIES_EXTENSION_NAME`]
- [`KHR_PIPELINE_EXECUTABLE_PROPERTIES_SPEC_VERSION`]
- Extending [`PipelineCreateFlagBits`]:  - `VK_PIPELINE_CREATE_CAPTURE_INTERNAL_REPRESENTATIONS_BIT_KHR`  - `VK_PIPELINE_CREATE_CAPTURE_STATISTICS_BIT_KHR` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR`  - `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INFO_KHR`  - `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR`  - `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_PROPERTIES_KHR`  - `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_STATISTIC_KHR`  - `VK_STRUCTURE_TYPE_PIPELINE_INFO_KHR`

# Known issues & F.A.Q.
1) What should we call the pieces of the pipeline which are produced by the
compilation process and about which you can query properties and statistics? **RESOLVED** : Call them “executables”.
The name “binary” was used in early drafts of the extension but it was
determined that “pipeline binary” could have a fairly broad meaning (such
as a binary serialized form of an entire pipeline) and was too big of a
namespace for the very specific needs of this extension.

# Version history
- Revision 1, 2019-05-28 (Jason Ekstrand)  - Initial draft

# Other information
* 2019-05-28
* No known IP claims.
*   - Jason Ekstrand, Intel  - Ian Romanick, Intel  - Kenneth Graunke, Intel  - Baldur Karlsson, Valve  - Jesse Hall, Google  - Jeff Bolz, Nvidia  - Piers Daniel, Nvidia  - Tobias Hector, AMD  - Jan-Harald Fredriksen, ARM  - Tom Olson, ARM  - Daniel Koch, Nvidia  - Spencer Fricke, Samsung

# Related
- [`PhysicalDevicePipelineExecutablePropertiesFeaturesKHR`]
- [`PipelineExecutableInfoKHR`]
- [`PipelineExecutableInternalRepresentationKHR`]
- [`PipelineExecutablePropertiesKHR`]
- [`PipelineExecutableStatisticFormatKHR`]
- [`PipelineExecutableStatisticKHR`]
- [`PipelineExecutableStatisticValueKHR`]
- [`PipelineInfoKHR`]
- [`get_pipeline_executable_internal_representations_khr`]
- [`get_pipeline_executable_properties_khr`]
- [`get_pipeline_executable_statistics_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        