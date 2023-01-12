[vkGetPhysicalDeviceSparseImageFormatProperties2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties2.html) - Retrieve properties of an image format applied to sparse images

# C Specifications
[`get_physical_device_sparse_image_format_properties2`] returns an array of
[`SparseImageFormatProperties2`].
Each element will describe properties for one set of image aspects that are
bound simultaneously in the image.
This is usually one element for each aspect in the image, but for
interleaved depth/stencil images there is only one element describing the
combined aspects.
```c
// Provided by VK_VERSION_1_1
void vkGetPhysicalDeviceSparseImageFormatProperties2(
    VkPhysicalDevice                            physicalDevice,
    const VkPhysicalDeviceSparseImageFormatInfo2* pFormatInfo,
    uint32_t*                                   pPropertyCount,
    VkSparseImageFormatProperties2*             pProperties);
```
or the equivalent command
```c
// Provided by VK_KHR_get_physical_device_properties2
void vkGetPhysicalDeviceSparseImageFormatProperties2KHR(
    VkPhysicalDevice                            physicalDevice,
    const VkPhysicalDeviceSparseImageFormatInfo2* pFormatInfo,
    uint32_t*                                   pPropertyCount,
    VkSparseImageFormatProperties2*             pProperties);
```

# Parameters
- [`physical_device`] is the physical device from which to query the sparse image format properties.
- [`p_format_info`] is a pointer to a [`PhysicalDeviceSparseImageFormatInfo2`] structure containing input parameters to the command.
- [`p_property_count`] is a pointer to an integer related to the number of sparse format properties available or queried, as described below.
- [`p_properties`] is either `NULL` or a pointer to an array of [`SparseImageFormatProperties2`] structures.

# Description
[`get_physical_device_sparse_image_format_properties2`] behaves identically to
[`get_physical_device_sparse_image_format_properties`], with the ability to
return extended information by adding extending structures to the
`pNext` chain of its [`p_properties`] parameter.
## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`p_format_info`] **must**  be a valid pointer to a valid [`PhysicalDeviceSparseImageFormatInfo2`] structure
-  [`p_property_count`] **must**  be a valid pointer to a `uint32_t` value
-    If the value referenced by [`p_property_count`] is not `0`, and [`p_properties`] is not `NULL`, [`p_properties`] **must**  be a valid pointer to an array of [`p_property_count`][`SparseImageFormatProperties2`] structures

# Related
- [`crate::vulkan1_1`]
- [`PhysicalDevice`]
- [`PhysicalDeviceSparseImageFormatInfo2`]
- [`SparseImageFormatProperties2`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        