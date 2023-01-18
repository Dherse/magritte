[VkPhysicalDeviceMaintenance4Features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMaintenance4Features.html) - Structure describing whether the implementation supports maintenance4 functionality

# C Specifications
The [`PhysicalDeviceMaintenance4Features`] structure is defined as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkPhysicalDeviceMaintenance4Features {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           maintenance4;
} VkPhysicalDeviceMaintenance4Features;
```
or the equivalent
```c
// Provided by VK_KHR_maintenance4
typedef VkPhysicalDeviceMaintenance4Features VkPhysicalDeviceMaintenance4FeaturesKHR;
```

# Members
This structure describes the following features:

# Description
- [`maintenance4`] indicates that the implementation supports the following:  - The application  **may**  destroy a [`PipelineLayout`] object immediately after using it to create another object.  - `LocalSizeId` **can**  be used as an alternative to `LocalSize` to specify the local workgroup size with specialization constants.  - Images created with identical creation parameters will always have the same alignment requirements.  - The size memory requirement of a buffer or image is never greater than that of another buffer or image created with a greater or equal size.  - Push constants do not have to be initialized before they are dynamically accessed.  - The interface matching rules allow a larger output vector to match with a smaller input vector, with additional values being discarded. 
If the [`PhysicalDeviceMaintenance4Features`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDeviceMaintenance4Features`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES`

# Related
- [`VK_KHR_maintenance4`]
- [`crate::vulkan1_3`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        