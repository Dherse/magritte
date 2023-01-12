[VK_NV_ray_tracing_motion_blur](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_ray_tracing_motion_blur.html) - device extension

# Description
Ray tracing support in the API provides an efficient mechanism to intersect
rays against static geometry, but rendering algorithms often want to support
motion, which is more efficiently supported with motion-specific algorithms.
This extension adds a set of mechanisms to support fast tracing of moving
geometry:
- A ray pipeline trace call which takes a time parameter
- Flags to enable motion support in an acceleration structure
- Support for time-varying vertex positions in a geometry
- Motion instances to move existing instances over time
The motion represented here is parameterized across a normalized timestep
between 0.0 and 1.0.
A motion trace using `OpTraceRayMotionNV` provides a time within that
normalized range to be used when intersecting that ray with geometry.
The geometry can be provided with motion by a combination of adding a second
vertex position for time of 1.0 using
[`AccelerationStructureGeometryMotionTrianglesDataNV`] and providing
multiple transforms in the instance using
[`AccelerationStructureMotionInstanceNV`].

# Registered extension number
328

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`khr_ray_tracing_pipeline`]`

# Contacts
- Eric Werness

# New structures
- [`AccelerationStructureMatrixMotionInstanceNV`]
- [`AccelerationStructureMotionInstanceNV`]
- [`AccelerationStructureSrtMotionInstanceNV`]
- [`SrtDataNV`]
- Extending [`AccelerationStructureCreateInfoKHR`]:  - [`AccelerationStructureMotionInfoNV`] 
- Extending [`AccelerationStructureGeometryTrianglesDataKHR`]:  - [`AccelerationStructureGeometryMotionTrianglesDataNV`] 
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceRayTracingMotionBlurFeaturesNV`]

# New unions
- [`AccelerationStructureMotionInstanceDataNV`]

# New enums
- [`AccelerationStructureMotionInstanceTypeNV`]

# New bitmasks
- [`AccelerationStructureMotionInfoFlagsNV`]
- [`AccelerationStructureMotionInstanceFlagsNV`]

# New constants
- `VK_NV_RAY_TRACING_MOTION_BLUR_EXTENSION_NAME`
- `VK_NV_RAY_TRACING_MOTION_BLUR_SPEC_VERSION`
- Extending [`AccelerationStructureCreateFlagBitsKHR`]:  - `VK_ACCELERATION_STRUCTURE_CREATE_MOTION_BIT_NV` 
- Extending [`BuildAccelerationStructureFlagBitsKHR`]:  - `VK_BUILD_ACCELERATION_STRUCTURE_MOTION_BIT_NV` 
- Extending [`PipelineCreateFlagBits`]:  - `VK_PIPELINE_CREATE_RAY_TRACING_ALLOW_MOTION_BIT_NV` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV`  - `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MOTION_INFO_NV`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV`

# Known issues & F.A.Q.
(1) What size is VkAccelerationStructureMotionInstanceNV?
- Added a note on the structure size and made the stride explicit in the language.
(2) Allow arrayOfPointers for motion TLAS?
- Yes, with a packed encoding to minimize the amount of data sent for metadata.

# Version history
- Revision 1, 2020-06-16 (Eric Werness, Ashwin Lele)  - Initial external release

# Other information
* 2021-06-16
*   - This extension requires [`SPV_NV_ray_tracing_motion_blur`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/NV/SPV_NV_ray_tracing_motion_blur.html)  - This extension provides API support for [`GL_NV_ray_tracing_motion_blur`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/nv/GLSL_NV_ray_tracing_motion_blur.txt) 
*   - Eric Werness, NVIDIA  - Ashwin Lele, NVIDIA

# Related
- [`AccelerationStructureGeometryMotionTrianglesDataNV`]
- [`AccelerationStructureMatrixMotionInstanceNV`]
- [`AccelerationStructureMotionInfoFlagsNV`]
- [`AccelerationStructureMotionInfoNV`]
- [`AccelerationStructureMotionInstanceDataNV`]
- [`AccelerationStructureMotionInstanceFlagsNV`]
- [`AccelerationStructureMotionInstanceNV`]
- [`AccelerationStructureMotionInstanceTypeNV`]
- [`AccelerationStructureSrtMotionInstanceNV`]
- [`PhysicalDeviceRayTracingMotionBlurFeaturesNV`]
- [`SrtDataNV`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        