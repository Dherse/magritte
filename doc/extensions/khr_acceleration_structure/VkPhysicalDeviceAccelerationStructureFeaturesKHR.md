[VkPhysicalDeviceAccelerationStructureFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceAccelerationStructureFeaturesKHR.html) - Structure describing the acceleration structure features that can be supported by an implementation

# C Specifications
The [`PhysicalDeviceAccelerationStructureFeaturesKHR`] structure is
defined as:
```c
// Provided by VK_KHR_acceleration_structure
typedef struct VkPhysicalDeviceAccelerationStructureFeaturesKHR {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           accelerationStructure;
    VkBool32           accelerationStructureCaptureReplay;
    VkBool32           accelerationStructureIndirectBuild;
    VkBool32           accelerationStructureHostCommands;
    VkBool32           descriptorBindingAccelerationStructureUpdateAfterBind;
} VkPhysicalDeviceAccelerationStructureFeaturesKHR;
```

# Members
This structure describes the following features:

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`acceleration_structure`] indicates whether the implementation supports the acceleration structure functionality. See [Acceleration Structures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#acceleration-structure).
- [`acceleration_structure_capture_replay`] indicates whether the implementation supports saving and reusing acceleration structure device addresses, e.g. for trace capture and replay.
- [`acceleration_structure_indirect_build`] indicates whether the implementation supports indirect acceleration structure build commands, e.g. [`cmd_build_acceleration_structures_indirect_khr`].
- [`acceleration_structure_host_commands`] indicates whether the implementation supports host side acceleration structure commands, e.g. [`build_acceleration_structures_khr`], [`copy_acceleration_structure_khr`], [`copy_acceleration_structure_to_memory_khr`], [`copy_memory_to_acceleration_structure_khr`], [`write_acceleration_structures_properties_khr`].
- [`descriptor_binding_acceleration_structure_update_after_bind`] indicates whether the implementation supports updating acceleration structure descriptors after a set is bound. If this feature is not enabled, `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be used with `VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR`.
If the [`PhysicalDeviceAccelerationStructureFeaturesKHR`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDeviceAccelerationStructureFeaturesKHR`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR`

# Related
- [`VK_KHR_acceleration_structure`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        