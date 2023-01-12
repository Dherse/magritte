[VkDisplaySurfaceCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplaySurfaceCreateInfoKHR.html) - Structure specifying parameters of a newly created display plane surface object

# C Specifications
The [`DisplaySurfaceCreateInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_display
typedef struct VkDisplaySurfaceCreateInfoKHR {
    VkStructureType                   sType;
    const void*                       pNext;
    VkDisplaySurfaceCreateFlagsKHR    flags;
    VkDisplayModeKHR                  displayMode;
    uint32_t                          planeIndex;
    uint32_t                          planeStackIndex;
    VkSurfaceTransformFlagBitsKHR     transform;
    float                             globalAlpha;
    VkDisplayPlaneAlphaFlagBitsKHR    alphaMode;
    VkExtent2D                        imageExtent;
} VkDisplaySurfaceCreateInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is reserved for future use, and  **must**  be zero.
- [`display_mode`] is a [`DisplayModeKHR`] handle specifying the mode to use when displaying this surface.
- [`plane_index`] is the plane on which this surface appears.
- [`plane_stack_index`] is the z-order of the plane.
- [`transform`] is a [`SurfaceTransformFlagBitsKHR`] value specifying the transformation to apply to images as part of the scanout operation.
- [`global_alpha`] is the global alpha value. This value is ignored if [`alpha_mode`] is not `VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR`.
- [`alpha_mode`] is a [`DisplayPlaneAlphaFlagBitsKHR`] value specifying the type of alpha blending to use.
- [`image_extent`] is the size of the presentable images to use with the surface.

# Description
## Valid Usage
-  [`plane_index`] **must**  be less than the number of display planes supported by the device as determined by calling [`get_physical_device_display_plane_properties_khr`]
-    If the `planeReorderPossible` member of the [`DisplayPropertiesKHR`] structure returned by [`get_physical_device_display_properties_khr`] for the display corresponding to [`display_mode`] is `VK_TRUE` then [`plane_stack_index`] **must**  be less than the number of display planes supported by the device as determined by calling [`get_physical_device_display_plane_properties_khr`]; otherwise [`plane_stack_index`] **must**  equal the `currentStackIndex` member of [`DisplayPlanePropertiesKHR`] returned by [`get_physical_device_display_plane_properties_khr`] for the display plane corresponding to [`display_mode`]
-    If [`alpha_mode`] is `VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR` then [`global_alpha`] **must**  be between `0` and `1`, inclusive
-  [`alpha_mode`] **must**  be one of the bits present in the `supportedAlpha` member of [`DisplayPlaneCapabilitiesKHR`] for the display plane corresponding to [`display_mode`]
-    The `width` and `height` members of [`image_extent`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_image_dimension2_d`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR`
-  [`p_next`] **must**  be `NULL`
-  [`flags`] **must**  be `0`
-  [`display_mode`] **must**  be a valid [`DisplayModeKHR`] handle
-  [`transform`] **must**  be a valid [`SurfaceTransformFlagBitsKHR`] value
-  [`alpha_mode`] **must**  be a valid [`DisplayPlaneAlphaFlagBitsKHR`] value

# Related
- [`khr_display`]
- [`DisplayModeKHR`]
- [`DisplayPlaneAlphaFlagBitsKHR`]
- [`DisplaySurfaceCreateFlagsKHR`]
- [`Extent2D`]
- [`StructureType`]
- [`SurfaceTransformFlagBitsKHR`]
- [`create_display_plane_surface_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        