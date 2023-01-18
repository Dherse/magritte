[VkPhysicalDeviceExtendedDynamicState2FeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExtendedDynamicState2FeaturesEXT.html) - Structure describing what extended dynamic state can be used

# C Specifications
The [`PhysicalDeviceExtendedDynamicState2FeaturesEXT`] structure is
defined as:
```c
// Provided by VK_EXT_extended_dynamic_state2
typedef struct VkPhysicalDeviceExtendedDynamicState2FeaturesEXT {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           extendedDynamicState2;
    VkBool32           extendedDynamicState2LogicOp;
    VkBool32           extendedDynamicState2PatchControlPoints;
} VkPhysicalDeviceExtendedDynamicState2FeaturesEXT;
```

# Members
This structure describes the following features:

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`extended_dynamic_state2`] indicates that the implementation supports the following dynamic states:  - `VK_DYNAMIC_STATE_DEPTH_BIAS_ENABLE`  - `VK_DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE`  - `VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE` 
- [`extended_dynamic_state2_logic_op`] indicates that the implementation supports the following dynamic state:  - `VK_DYNAMIC_STATE_LOGIC_OP_EXT` 
- [`extended_dynamic_state2_patch_control_points`] indicates that the implementation supports the following dynamic state:  - `VK_DYNAMIC_STATE_PATCH_CONTROL_POINTS_EXT` 
If the [`PhysicalDeviceExtendedDynamicState2FeaturesEXT`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDeviceExtendedDynamicState2FeaturesEXT`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT`

# Related
- [`VK_EXT_extended_dynamic_state2`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        