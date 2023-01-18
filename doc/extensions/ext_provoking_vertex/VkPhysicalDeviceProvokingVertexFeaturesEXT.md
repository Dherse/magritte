[VkPhysicalDeviceProvokingVertexFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProvokingVertexFeaturesEXT.html) - Structure describing the provoking vertex features that can be supported by an implementation

# C Specifications
The [`PhysicalDeviceProvokingVertexFeaturesEXT`] structure is defined
as:
```c
// Provided by VK_EXT_provoking_vertex
typedef struct VkPhysicalDeviceProvokingVertexFeaturesEXT {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           provokingVertexLast;
    VkBool32           transformFeedbackPreservesProvokingVertex;
} VkPhysicalDeviceProvokingVertexFeaturesEXT;
```

# Members
This structure describes the following feature:

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`provoking_vertex_last`] indicates whether the implementation supports the `VK_PROVOKING_VERTEX_MODE_LAST_VERTEX_EXT`[`ProvokingVertexModeEXT`] for flat shading.
- [`transform_feedback_preserves_provoking_vertex`] indicates that the order of vertices within each primitive written by transform feedback will preserve the provoking vertex. This does not apply to triangle fan primitives when [`transformFeedbackPreservesTriangleFanProvokingVertex`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-transformFeedbackPreservesTriangleFanProvokingVertex) is [`FALSE`]. [`transform_feedback_preserves_provoking_vertex`] **must**  be [`FALSE`] when the [`VK_EXT_transform_feedback`] extension is not supported.
If the [`PhysicalDeviceProvokingVertexFeaturesEXT`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDeviceProvokingVertexFeaturesEXT`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.When [`PhysicalDeviceProvokingVertexFeaturesEXT`] is in the [`p_next`]
chain of [`DeviceCreateInfo`] but the
[transform feedback feature](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-transformFeedback) is not enabled,
the value of [`transform_feedback_preserves_provoking_vertex`] is ignored.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT`

# Related
- [`VK_EXT_provoking_vertex`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        