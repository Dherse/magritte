[VK_EXT_provoking_vertex](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_provoking_vertex.html) - device extension

# Description
This extension allows changing the provoking vertex convention between
Vulkan’s default convention (first vertex) and OpenGL’s convention (last
vertex).This extension is intended for use by API-translation layers that implement
APIs like OpenGL on top of Vulkan, and need to match the source API’s
provoking vertex convention.
Applications using Vulkan directly should use Vulkan’s default convention.

# Registered extension number
255

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`khr_get_physical_device_properties2`]`

# Contacts
- Jesse Hall [jessehall](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_provoking_vertex] @jessehall%0A<<Here describe the issue or question you have about the VK_EXT_provoking_vertex extension>>)

# New structures
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceProvokingVertexFeaturesEXT`] 
- Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceProvokingVertexPropertiesEXT`] 
- Extending [`PipelineRasterizationStateCreateInfo`]:  - [`PipelineRasterizationProvokingVertexStateCreateInfoEXT`]

# New enums
- [`ProvokingVertexModeEXT`]

# New constants
- `VK_EXT_PROVOKING_VERTEX_EXTENSION_NAME`
- `VK_EXT_PROVOKING_VERTEX_SPEC_VERSION`
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT`  - `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT`

# Known issues & F.A.Q.
1) At what granularity should this state be set? **RESOLVED** : At pipeline bind, with an optional per-render pass restriction.The most natural place to put this state is in the graphics pipeline object.
Some implementations require it to be known when creating the pipeline, and
pipeline state is convenient for implementing OpenGL 3.2’s
glProvokingVertex, which can change the state between draw calls.
However, some implementations can only change it approximately render pass
granularity.
To accommodate both, provoking vertex will be pipeline state, but
implementations can require that only one mode is used within a render pass
instance; the render pass’s mode is chosen implicitly when the first
pipeline is bound.2) Does the provoking vertex mode affect the order that vertices are written
to transform feedback buffers? **RESOLVED** : Yes, to enable layered implementations of OpenGL and D3D.All of OpenGL, OpenGL ES, and Direct3D 11 require that vertices are written
to transform feedback buffers such that flat-shaded attributes have the same
value when drawing the contents of the transform feedback buffer as they did
in the original drawing when the transform feedback buffer was written
(assuming the provoking vertex mode has not changed, in APIs that support
more than one mode).

# Version history
- Revision 1, (1c) 2021-02-22 (Jesse Hall)  - Added VkPhysicalDeviceProvokingVertexPropertiesEXT::transformFeedbackPreservesTriangleFanProvokingVertex to accommodate implementations that cannot change the transform feedback vertex order for triangle fans. 
- Revision 1, (1b) 2020-06-14 (Jesse Hall)  - Added VkPhysicalDeviceProvokingVertexFeaturesEXT::transformFeedbackPreservesProvokingVertex and required that transform feedback write vertices so as to preserve the provoking vertex of each primitive. 
- Revision 1, (1a) 2019-10-23 (Jesse Hall)  - Initial draft, based on a proposal by Alexis Hétu

# Other information
* 2021-02-22
* No known IP claims.
*   - Alexis Hétu, Google  - Bill Licea-Kane, Qualcomm  - Daniel Koch, Nvidia  - Jamie Madill, Google  - Jan-Harald Fredriksen, Arm  - Jason Ekstrand, Intel  - Jeff Bolz, Nvidia  - Jeff Leger, Qualcomm  - Jesse Hall, Google  - Jörg Wagner, Arm  - Matthew Netsch, Qualcomm  - Mike Blumenkrantz, Valve  - Piers Daniell, Nvidia  - Tobias Hector, AMD

# Related
- [`PhysicalDeviceProvokingVertexFeaturesEXT`]
- [`PhysicalDeviceProvokingVertexPropertiesEXT`]
- [`PipelineRasterizationProvokingVertexStateCreateInfoEXT`]
- [`ProvokingVertexModeEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        