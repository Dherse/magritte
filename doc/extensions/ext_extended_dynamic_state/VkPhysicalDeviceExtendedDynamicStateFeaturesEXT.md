[VkPhysicalDeviceExtendedDynamicStateFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExtendedDynamicStateFeaturesEXT.html) - Structure describing what extended dynamic state can be used

# C Specifications
The [`PhysicalDeviceExtendedDynamicStateFeaturesEXT`] structure is
defined as:
```c
// Provided by VK_EXT_extended_dynamic_state
typedef struct VkPhysicalDeviceExtendedDynamicStateFeaturesEXT {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           extendedDynamicState;
} VkPhysicalDeviceExtendedDynamicStateFeaturesEXT;
```

# Members
This structure describes the following feature:

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`extended_dynamic_state`] indicates that the implementation supports the following dynamic states:  - `VK_DYNAMIC_STATE_CULL_MODE`  - `VK_DYNAMIC_STATE_FRONT_FACE`  - `VK_DYNAMIC_STATE_PRIMITIVE_TOPOLOGY`  - `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT`  - `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT`  - `VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE`  - `VK_DYNAMIC_STATE_DEPTH_TEST_ENABLE`  - `VK_DYNAMIC_STATE_DEPTH_WRITE_ENABLE`  - `VK_DYNAMIC_STATE_DEPTH_COMPARE_OP`  - `VK_DYNAMIC_STATE_DEPTH_BOUNDS_TEST_ENABLE`  - `VK_DYNAMIC_STATE_STENCIL_TEST_ENABLE`  - `VK_DYNAMIC_STATE_STENCIL_OP` 
If the [`PhysicalDeviceExtendedDynamicStateFeaturesEXT`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDeviceExtendedDynamicStateFeaturesEXT`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT`

# Related
- [`VK_EXT_extended_dynamic_state`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        