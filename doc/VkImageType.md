[VkImageType](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageType.html) - Specifies the type of an image object

# C Specifications
Possible values of [`ImageCreateInfo::image_type`], specifying the
basic dimensionality of an image, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkImageType {
    VK_IMAGE_TYPE_1D = 0,
    VK_IMAGE_TYPE_2D = 1,
    VK_IMAGE_TYPE_3D = 2,
} VkImageType;
```

# Description
- [`VK_IMAGE_TYPE`] specifies a one-dimensional image.
- [`VK_IMAGE_TYPE`] specifies a two-dimensional image.
- [`VK_IMAGE_TYPE`] specifies a three-dimensional image.

# Related
- [`crate::vulkan1_0`]
- [`ImageCreateInfo`]
- [`PhysicalDeviceImageFormatInfo2`]
- [`PhysicalDeviceSparseImageFormatInfo2`]
- [`get_physical_device_external_image_format_properties_nv`]
- [`get_physical_device_image_format_properties`]
- [`get_physical_device_sparse_image_format_properties`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        