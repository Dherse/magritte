[VkPhysicalDeviceVariablePointersFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVariablePointersFeatures.html) - Structure describing variable pointers features that can be supported by an implementation

# C Specifications
The [`PhysicalDeviceVariablePointersFeatures`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkPhysicalDeviceVariablePointersFeatures {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           variablePointersStorageBuffer;
    VkBool32           variablePointers;
} VkPhysicalDeviceVariablePointersFeatures;
```

```c
// Provided by VK_VERSION_1_1
typedef VkPhysicalDeviceVariablePointersFeatures VkPhysicalDeviceVariablePointerFeatures;
```
or the equivalent
```c
// Provided by VK_KHR_variable_pointers
typedef VkPhysicalDeviceVariablePointersFeatures VkPhysicalDeviceVariablePointersFeaturesKHR;
```

```c
// Provided by VK_KHR_variable_pointers
typedef VkPhysicalDeviceVariablePointersFeatures VkPhysicalDeviceVariablePointerFeaturesKHR;
```

# Members
This structure describes the following features:

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.

- [`variable_pointers_storage_buffer`] specifies whether the implementation supports the SPIR-V `VariablePointersStorageBuffer` capability. When this feature is not enabled, shader modules  **must**  not declare the `SPV_KHR_variable_pointers` extension or the `VariablePointersStorageBuffer` capability.
- [`variable_pointers`] specifies whether the implementation supports the SPIR-V `VariablePointers` capability. When this feature is not enabled, shader modules  **must**  not declare the `VariablePointers` capability.
If the [`PhysicalDeviceVariablePointersFeatures`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDeviceVariablePointersFeatures`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.
## Valid Usage
-    If [`variable_pointers`] is enabled then [`variable_pointers_storage_buffer`] **must**  also be enabled

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES`

# Related
- [`VK_KHR_variable_pointers`]
- [`crate::vulkan1_1`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        