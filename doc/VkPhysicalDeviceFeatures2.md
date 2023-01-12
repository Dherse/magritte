[VkPhysicalDeviceFeatures2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFeatures2.html) - Structure describing the fine-grained features that can be supported by an implementation

# C Specifications
The [`PhysicalDeviceFeatures2`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkPhysicalDeviceFeatures2 {
    VkStructureType             sType;
    void*                       pNext;
    VkPhysicalDeviceFeatures    features;
} VkPhysicalDeviceFeatures2;
```
or the equivalent
```c
// Provided by VK_KHR_get_physical_device_properties2
typedef VkPhysicalDeviceFeatures2 VkPhysicalDeviceFeatures2KHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`features`] is a [`PhysicalDeviceFeatures`] structure describing the fine-grained features of the Vulkan 1.0 API.

# Description
The [`p_next`] chain of this structure is used to extend the structure with
features defined by extensions.
This structure  **can**  be used in [`get_physical_device_features2`] or  **can**  be
included in the [`p_next`] chain of a [`DeviceCreateInfo`] structure,
in which case it controls which features are enabled in the device in lieu
of `pEnabledFeatures`.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2`

# Related
- [`crate::vulkan1_1`]
- [`PhysicalDeviceFeatures`]
- [`StructureType`]
- [`get_physical_device_features2`]
- [`get_physical_device_features2_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        