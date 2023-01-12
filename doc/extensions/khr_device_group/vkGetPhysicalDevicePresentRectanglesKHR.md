[vkGetPhysicalDevicePresentRectanglesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDevicePresentRectanglesKHR.html) - Query present rectangles for a surface on a physical device

# C Specifications
When using `VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHR`,
the application  **may**  need to know which regions of the surface are used when
presenting locally on each physical device.
Presentation of swapchain images to this surface need only have valid
contents in the regions returned by this command.To query a set of rectangles used in presentation on the physical device,
call:
```c
// Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_surface
VkResult vkGetPhysicalDevicePresentRectanglesKHR(
    VkPhysicalDevice                            physicalDevice,
    VkSurfaceKHR                                surface,
    uint32_t*                                   pRectCount,
    VkRect2D*                                   pRects);
```

# Parameters
- [`physical_device`] is the physical device.
- [`surface`] is the surface.
- [`p_rect_count`] is a pointer to an integer related to the number of rectangles available or queried, as described below.
- [`p_rects`] is either `NULL` or a pointer to an array of [`Rect2D`] structures.

# Description
If [`p_rects`] is `NULL`, then the number of rectangles used when
presenting the given [`surface`] is returned in [`p_rect_count`].
Otherwise, [`p_rect_count`] **must**  point to a variable set by the user to the
number of elements in the [`p_rects`] array, and on return the variable is
overwritten with the number of structures actually written to [`p_rects`].
If the value of [`p_rect_count`] is less than the number of rectangles, at
most [`p_rect_count`] structures will be written, and `VK_INCOMPLETE`
will be returned instead of `VK_SUCCESS`, to indicate that not all the
available rectangles were returned.The values returned by this command are not invariant, and  **may**  change in
response to the surface being moved, resized, or occluded.The rectangles returned by this command  **must**  not overlap.
## Valid Usage
-  [`surface`] **must**  be a valid [`SurfaceKHR`] handle
-  [`surface`] **must**  be supported by [`physical_device`], as reported by [`get_physical_device_surface_support_khr`] or an equivalent platform-specific mechanism

## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`surface`] **must**  be a valid [`SurfaceKHR`] handle
-  [`p_rect_count`] **must**  be a valid pointer to a `uint32_t` value
-    If the value referenced by [`p_rect_count`] is not `0`, and [`p_rects`] is not `NULL`, [`p_rects`] **must**  be a valid pointer to an array of [`p_rect_count`][`Rect2D`] structures
-    Both of [`physical_device`], and [`surface`] **must**  have been created, allocated, or retrieved from the same [`Instance`]

## Host Synchronization
- Host access to [`surface`] **must**  be externally synchronized

## Return Codes
*   - `VK_SUCCESS`  - `VK_INCOMPLETE` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`khr_device_group`]
- [`khr_surface`]
- [`khr_swapchain`]
- [`crate::vulkan1_1`]
- [`PhysicalDevice`]
- [`Rect2D`]
- [`SurfaceKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        