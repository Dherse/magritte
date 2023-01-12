[VkPointClippingBehavior](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPointClippingBehavior.html) - Enum specifying the point clipping behavior

# C Specifications
Possible values of
[`PhysicalDevicePointClippingProperties::point_clipping_behavior`],
specifying clipping behavior of a point primitive whose vertex lies outside
the clip volume, are:
```c
// Provided by VK_VERSION_1_1
typedef enum VkPointClippingBehavior {
    VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES = 0,
    VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY = 1,
  // Provided by VK_KHR_maintenance2
    VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES_KHR = VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES,
  // Provided by VK_KHR_maintenance2
    VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY_KHR = VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY,
} VkPointClippingBehavior;
```
or the equivalent
```c
// Provided by VK_KHR_maintenance2
typedef VkPointClippingBehavior VkPointClippingBehaviorKHR;
```

# Description
- [`VK_POINT_CLIPPING_BEHAVIOR`] specifies that the primitive is discarded if the vertex lies outside any clip plane, including the planes bounding the view volume.
- [`VK_POINT_CLIPPING_BEHAVIOR`] specifies that the primitive is discarded only if the vertex lies outside any user clip plane.

# Related
- [`crate::vulkan1_1`]
- [`PhysicalDevicePointClippingProperties`]
- [`PhysicalDeviceVulkan11Properties`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        