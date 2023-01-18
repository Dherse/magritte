[VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE.html) - Structure describing whether the mutable descriptor type is supported

# C Specifications
The [`PhysicalDeviceMutableDescriptorTypeFeaturesVALVE`] structure is
defined as:
```c
// Provided by VK_VALVE_mutable_descriptor_type
typedef struct VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           mutableDescriptorType;
} VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE;
```

# Members
This structure describes the following feature:

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`mutable_descriptor_type`] indicates that the implementation  **must**  support using the [`DescriptorType`] of `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE` with at least the following descriptor types, where any combination of the types  **must**  be supported:  - `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`  - `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`  - `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`  - `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`  - `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER`  - `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` 
- Additionally, [`mutable_descriptor_type`] indicates that:  - Non-uniform descriptor indexing  **must**  be supported if all descriptor types in a [`MutableDescriptorTypeListVALVE`] for `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE` have the corresponding non-uniform indexing features enabled in [`PhysicalDeviceDescriptorIndexingFeatures`].  - `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` with `descriptorType` of `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE` relaxes the list of required descriptor types to the descriptor types which have the corresponding update-after-bind feature enabled in [`PhysicalDeviceDescriptorIndexingFeatures`].  - Dynamically uniform descriptor indexing  **must**  be supported if all descriptor types in a [`MutableDescriptorTypeListVALVE`] for `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE` have the corresponding dynamic indexing features enabled.  - `VK_DESCRIPTOR_SET_LAYOUT_CREATE_HOST_ONLY_POOL_BIT_VALVE` **must**  be supported.  - `VK_DESCRIPTOR_POOL_CREATE_HOST_ONLY_BIT_VALVE` **must**  be supported. 
If the [`PhysicalDeviceMutableDescriptorTypeFeaturesVALVE`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDeviceMutableDescriptorTypeFeaturesVALVE`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_VALVE`

# Related
- [`VK_VALVE_mutable_descriptor_type`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        