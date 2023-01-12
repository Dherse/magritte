[VkPhysicalDeviceMultiviewFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMultiviewFeatures.html) - Structure describing multiview features that can be supported by an implementation

# C Specifications
The [`PhysicalDeviceMultiviewFeatures`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkPhysicalDeviceMultiviewFeatures {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           multiview;
    VkBool32           multiviewGeometryShader;
    VkBool32           multiviewTessellationShader;
} VkPhysicalDeviceMultiviewFeatures;
```
or the equivalent
```c
// Provided by VK_KHR_multiview
typedef VkPhysicalDeviceMultiviewFeatures VkPhysicalDeviceMultiviewFeaturesKHR;
```

# Members
This structure describes the following features:

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.

- [`multiview`] specifies whether the implementation supports multiview rendering within a render pass. If this feature is not enabled, the view mask of each subpass  **must**  always be zero.
- [`multiview_geometry_shader`] specifies whether the implementation supports multiview rendering within a render pass, with [geometry shaders](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#geometry). If this feature is not enabled, then a pipeline compiled against a subpass with a non-zero view mask  **must**  not include a geometry shader.
- [`multiview_tessellation_shader`] specifies whether the implementation supports multiview rendering within a render pass, with [tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#tessellation). If this feature is not enabled, then a pipeline compiled against a subpass with a non-zero view mask  **must**  not include any tessellation shaders.
If the [`PhysicalDeviceMultiviewFeatures`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDeviceMultiviewFeatures`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.
## Valid Usage
-    If [`multiview_geometry_shader`] is enabled then [`multiview`] **must**  also be enabled
-    If [`multiview_tessellation_shader`] is enabled then [`multiview`] **must**  also be enabled

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES`

# Related
- [`crate::vulkan1_1`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        