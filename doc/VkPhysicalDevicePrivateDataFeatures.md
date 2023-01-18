[VkPhysicalDevicePrivateDataFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePrivateDataFeatures.html) - Structure specifying physical device support

# C Specifications
The [`PhysicalDevicePrivateDataFeatures`] structure is defined as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkPhysicalDevicePrivateDataFeatures {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           privateData;
} VkPhysicalDevicePrivateDataFeatures;
```
or the equivalent
```c
// Provided by VK_EXT_private_data
typedef VkPhysicalDevicePrivateDataFeatures VkPhysicalDevicePrivateDataFeaturesEXT;
```

# Members
This structure describes the following feature:

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.

- [`private_data`] indicates whether the implementation supports private data. See [Private Data](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#private-data).
If the [`PhysicalDevicePrivateDataFeatures`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDevicePrivateDataFeatures`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES`

# Related
- [`VK_EXT_private_data`]
- [`crate::vulkan1_3`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        