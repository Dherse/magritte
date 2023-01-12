[VK_EXT_sample_locations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_sample_locations.html) - device extension

# Description
This extension allows an application to modify the locations of samples
within a pixel used in rasterization.
Additionally, it allows applications to specify different sample locations
for each pixel in a group of adjacent pixels, which  **can**  increase
antialiasing quality (particularly if a custom resolve shader is used that
takes advantage of these different locations).It is common for implementations to optimize the storage of depth values by
storing values that  **can**  be used to reconstruct depth at each sample
location, rather than storing separate depth values for each sample.
For example, the depth values from a single triangle  **may**  be represented
using plane equations.
When the depth value for a sample is needed, it is automatically evaluated
at the sample location.
Modifying the sample locations causes the reconstruction to no longer
evaluate the same depth values as when the samples were originally
generated, thus the depth aspect of a depth/stencil attachment  **must**  be
cleared before rendering to it using different sample locations.Some implementations  **may**  need to evaluate depth image values while
performing image layout transitions.
To accommodate this, instances of the [`SampleLocationsInfoEXT`]
structure  **can**  be specified for each situation where an explicit or
automatic layout transition has to take place.
[`SampleLocationsInfoEXT`] **can**  be chained from
[`ImageMemoryBarrier`] structures to provide sample locations for layout
transitions performed by [`cmd_wait_events`] and
[`cmd_pipeline_barrier`] calls, and
[`RenderPassSampleLocationsBeginInfoEXT`] **can**  be chained from
[`RenderPassBeginInfo`] to provide sample locations for layout
transitions performed implicitly by a render pass instance.

# Registered extension number
144

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`khr_get_physical_device_properties2`]`

# Contacts
- Daniel Rakos [drakos-amd](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_sample_locations] @drakos-amd%0A<<Here describe the issue or question you have about the VK_EXT_sample_locations extension>>)

# New commands
- [`cmd_set_sample_locations_ext`]
- [`get_physical_device_multisample_properties_ext`]

# New structures
- [`AttachmentSampleLocationsEXT`]
- [`MultisamplePropertiesEXT`]
- [`SampleLocationEXT`]
- [`SubpassSampleLocationsEXT`]
- Extending [`ImageMemoryBarrier`], [`ImageMemoryBarrier2`]:  - [`SampleLocationsInfoEXT`] 
- Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceSampleLocationsPropertiesEXT`] 
- Extending [`PipelineMultisampleStateCreateInfo`]:  - [`PipelineSampleLocationsStateCreateInfoEXT`] 
- Extending [`RenderPassBeginInfo`]:  - [`RenderPassSampleLocationsBeginInfoEXT`]

# New constants
- `VK_EXT_SAMPLE_LOCATIONS_EXTENSION_NAME`
- `VK_EXT_SAMPLE_LOCATIONS_SPEC_VERSION`
- Extending [`DynamicState`]:  - `VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_EXT` 
- Extending [`ImageCreateFlagBits`]:  - `VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_MULTISAMPLE_PROPERTIES_EXT`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT`  - `VK_STRUCTURE_TYPE_PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT`  - `VK_STRUCTURE_TYPE_RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT`  - `VK_STRUCTURE_TYPE_SAMPLE_LOCATIONS_INFO_EXT`

# Version history
- Revision 1, 2017-08-02 (Daniel Rakos)  - Internal revisions

# Other information
* 2017-08-02
*   - Mais Alnasser, AMD  - Matthaeus G. Chajdas, AMD  - Maciej Jesionowski, AMD  - Daniel Rakos, AMD  - Slawomir Grajewski, Intel  - Jeff Bolz, NVIDIA  - Bill Licea-Kane, Qualcomm

# Related
- [`AttachmentSampleLocationsEXT`]
- [`MultisamplePropertiesEXT`]
- [`PhysicalDeviceSampleLocationsPropertiesEXT`]
- [`PipelineSampleLocationsStateCreateInfoEXT`]
- [`RenderPassSampleLocationsBeginInfoEXT`]
- [`SampleLocationEXT`]
- [`SampleLocationsInfoEXT`]
- [`SubpassSampleLocationsEXT`]
- [`cmd_set_sample_locations_ext`]
- [`get_physical_device_multisample_properties_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        