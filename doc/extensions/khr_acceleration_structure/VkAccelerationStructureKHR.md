[VkAccelerationStructureKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureKHR.html) - Opaque handle to an acceleration structure object

# C Specifications
Acceleration structures are opaque data structures that are built by the
implementation to more efficiently perform spatial queries on the provided
geometric data.
For this extension, an acceleration structure is either a top-level
acceleration structure containing a set of bottom-level acceleration
structures or a bottom-level acceleration structure containing either a set
of axis-aligned bounding boxes for custom geometry or a set of triangles.Each instance in the top-level acceleration structure contains a reference
to a bottom-level acceleration structure as well as an instance transform
plus information required to index into the shader bindings.
The top-level acceleration structure is what is bound to the acceleration
descriptor, for example to trace inside the shader in the ray tracing
pipeline.Acceleration structures are represented by [`AccelerationStructureKHR`]
handles:
```c
// Provided by VK_KHR_acceleration_structure
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkAccelerationStructureKHR)
```

# Related
- [`VK_KHR_acceleration_structure`]
- [`AccelerationStructureBuildGeometryInfoKHR`]
- [`AccelerationStructureDeviceAddressInfoKHR`]
- [`CopyAccelerationStructureInfoKHR`]
- [`CopyAccelerationStructureToMemoryInfoKHR`]
- [`CopyMemoryToAccelerationStructureInfoKHR`]
- [`WriteDescriptorSetAccelerationStructureKHR`]
- [`cmd_write_acceleration_structures_properties_khr`]
- [`create_acceleration_structure_khr`]
- [`destroy_acceleration_structure_khr`]
- [`write_acceleration_structures_properties_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        