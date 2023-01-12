[VkPhysicalDeviceExternalImageFormatInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalImageFormatInfo.html) - Structure specifying external image creation parameters

# C Specifications
To determine the image capabilities compatible with an external memory
handle type, add a [`PhysicalDeviceExternalImageFormatInfo`] structure
to the [`p_next`] chain of the [`PhysicalDeviceImageFormatInfo2`]
structure and a [`ExternalImageFormatProperties`] structure to the
[`p_next`] chain of the [`ImageFormatProperties2`] structure.The [`PhysicalDeviceExternalImageFormatInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkPhysicalDeviceExternalImageFormatInfo {
    VkStructureType                       sType;
    const void*                           pNext;
    VkExternalMemoryHandleTypeFlagBits    handleType;
} VkPhysicalDeviceExternalImageFormatInfo;
```
or the equivalent
```c
// Provided by VK_KHR_external_memory_capabilities
typedef VkPhysicalDeviceExternalImageFormatInfo VkPhysicalDeviceExternalImageFormatInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value specifying the memory handle type that will be used with the memory associated with the image.

# Description
If [`handle_type`] is 0, [`get_physical_device_image_format_properties2`]
will behave as if [`PhysicalDeviceExternalImageFormatInfo`] was not
present, and [`ExternalImageFormatProperties`] will be ignored.If [`handle_type`] is not compatible with the `format`, `type`,
`tiling`, `usage`, and `flags` specified in
[`PhysicalDeviceImageFormatInfo2`], then
[`get_physical_device_image_format_properties2`] returns
`VK_ERROR_FORMAT_NOT_SUPPORTED`.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO`
-    If [`handle_type`] is not `0`, [`handle_type`] **must**  be a valid [`ExternalMemoryHandleTypeFlagBits`] value

# Related
- [`crate::vulkan1_1`]
- [`ExternalMemoryHandleTypeFlagBits`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        