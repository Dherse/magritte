[VkBindImageMemoryInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindImageMemoryInfo.html) - Structure specifying how to bind an image to memory

# C Specifications
[`BindImageMemoryInfo`] contains members corresponding to the parameters
of [`bind_image_memory`].The [`BindImageMemoryInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkBindImageMemoryInfo {
    VkStructureType    sType;
    const void*        pNext;
    VkImage            image;
    VkDeviceMemory     memory;
    VkDeviceSize       memoryOffset;
} VkBindImageMemoryInfo;
```
or the equivalent
```c
// Provided by VK_KHR_bind_memory2
typedef VkBindImageMemoryInfo VkBindImageMemoryInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`image`] is the image to be attached to memory.
- [`memory`] is a [`DeviceMemory`] object describing the device memory to attach.
- [`memory_offset`] is the start offset of the region of [`memory`] which is to be bound to the image. The number of bytes returned in the [`MemoryRequirements::size`] member in [`memory`], starting from [`memory_offset`] bytes, will be bound to the specified image.

# Description
## Valid Usage
-  [`image`] **must**  not already be backed by a memory object
-  [`image`] **must**  not have been created with any sparse memory binding flags
-  [`memory_offset`] **must**  be less than the size of [`memory`]
-    If [`image`] requires a dedicated allocation (as reported by [`get_image_memory_requirements2`] in [`MemoryDedicatedRequirements::requires_dedicated_allocation`] for [`image`]), [`memory`] **must**  have been created with [`MemoryDedicatedAllocateInfo`]::[`image`] equal to [`image`]
-    If the [dedicated allocation image aliasing](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-dedicatedAllocationImageAliasing) feature is not enabled, and the [`MemoryAllocateInfo`] provided when [`memory`] was allocated included a [`MemoryDedicatedAllocateInfo`] structure in its [`p_next`] chain, and [`MemoryDedicatedAllocateInfo`]::[`image`] was not [`crate::Handle::null`], then [`image`] **must**  equal [`MemoryDedicatedAllocateInfo`]::[`image`] and [`memory_offset`] **must**  be zero
-    If the [dedicated allocation image aliasing](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-dedicatedAllocationImageAliasing) feature is enabled, and the [`MemoryAllocateInfo`] provided when [`memory`] was allocated included a [`MemoryDedicatedAllocateInfo`] structure in its [`p_next`] chain, and [`MemoryDedicatedAllocateInfo`]::[`image`] was not [`crate::Handle::null`], then [`memory_offset`] **must**  be zero, and [`image`] **must**  be either equal to [`MemoryDedicatedAllocateInfo`]::[`image`] or an image that was created using the same parameters in [`ImageCreateInfo`], with the exception that `extent` and `arrayLayers` **may**  differ subject to the following restrictions: every dimension in the `extent` parameter of the image being bound  **must**  be equal to or smaller than the original image for which the allocation was created; and the `arrayLayers` parameter of the image being bound  **must**  be equal to or smaller than the original image for which the allocation was created
-    If image was created with the `VK_IMAGE_CREATE_PROTECTED_BIT` bit set, the image  **must**  be bound to a memory object allocated with a memory type that reports `VK_MEMORY_PROPERTY_PROTECTED_BIT`
-    If image was created with the `VK_IMAGE_CREATE_PROTECTED_BIT` bit not set, the image  **must**  not be bound to a memory object created with a memory type that reports `VK_MEMORY_PROPERTY_PROTECTED_BIT`
-    If [`image`] was created with [`DedicatedAllocationImageCreateInfoNV::dedicated_allocation`] equal to [`TRUE`], [`memory`] **must**  have been created with [`DedicatedAllocationMemoryAllocateInfoNV`]::[`image`] equal to an image handle created with identical creation parameters to [`image`] and [`memory_offset`] **must**  be zero
-    If the value of [`ExportMemoryAllocateInfo::handle_types`] used to allocate [`memory`] is not `0`, it  **must**  include at least one of the handles set in [`ExternalMemoryImageCreateInfo::handle_types`] when [`image`] was created
-    If [`memory`] was created by a memory import operation, that is not [`ImportAndroidHardwareBufferInfoANDROID`] with a non-`NULL``buffer` value, the external handle type of the imported memory  **must**  also have been set in [`ExternalMemoryImageCreateInfo::handle_types`] when [`image`] was created
-    If [`memory`] was created with the [`ImportAndroidHardwareBufferInfoANDROID`] memory import operation with a non-`NULL``buffer` value, `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID` **must**  also have been set in [`ExternalMemoryImageCreateInfo::handle_types`] when [`image`] was created
-    If the [`p_next`] chain does not include a [`BindImagePlaneMemoryInfo`] structure, [`memory`] **must**  have been allocated using one of the memory types allowed in the `memoryTypeBits` member of the [`MemoryRequirements`] structure returned from a call to [`get_image_memory_requirements2`] with [`image`]
-    If the [`p_next`] chain does not include a [`BindImagePlaneMemoryInfo`] structure, [`memory_offset`] **must**  be an integer multiple of the `alignment` member of the [`MemoryRequirements`] structure returned from a call to [`get_image_memory_requirements2`] with [`image`]
-    If the [`p_next`] chain does not include a [`BindImagePlaneMemoryInfo`] structure, the difference of the size of [`memory`] and [`memory_offset`] **must**  be greater than or equal to the `size` member of the [`MemoryRequirements`] structure returned from a call to [`get_image_memory_requirements2`] with the same [`image`]
-    If the [`p_next`] chain includes a [`BindImagePlaneMemoryInfo`] structure, [`image`] **must**  have been created with the `VK_IMAGE_CREATE_DISJOINT_BIT` bit set
-    If the [`p_next`] chain includes a [`BindImagePlaneMemoryInfo`] structure, [`memory`] **must**  have been allocated using one of the memory types allowed in the `memoryTypeBits` member of the [`MemoryRequirements`] structure returned from a call to [`get_image_memory_requirements2`] with [`image`] and where [`BindImagePlaneMemoryInfo::plane_aspect`] corresponds to the [`ImagePlaneMemoryRequirementsInfo::plane_aspect`] in the [`ImageMemoryRequirementsInfo2`] structure’s [`p_next`] chain
-    If the [`p_next`] chain includes a [`BindImagePlaneMemoryInfo`] structure, [`memory_offset`] **must**  be an integer multiple of the `alignment` member of the [`MemoryRequirements`] structure returned from a call to [`get_image_memory_requirements2`] with [`image`] and where [`BindImagePlaneMemoryInfo::plane_aspect`] corresponds to the [`ImagePlaneMemoryRequirementsInfo::plane_aspect`] in the [`ImageMemoryRequirementsInfo2`] structure’s [`p_next`] chain
-    If the [`p_next`] chain includes a [`BindImagePlaneMemoryInfo`] structure, the difference of the size of [`memory`] and [`memory_offset`] **must**  be greater than or equal to the `size` member of the [`MemoryRequirements`] structure returned from a call to [`get_image_memory_requirements2`] with the same [`image`] and where [`BindImagePlaneMemoryInfo::plane_aspect`] corresponds to the [`ImagePlaneMemoryRequirementsInfo::plane_aspect`] in the [`ImageMemoryRequirementsInfo2`] structure’s [`p_next`] chain
-    If the [`p_next`] chain includes a [`BindImageMemoryDeviceGroupInfo`] structure, all instances of [`memory`] specified by [`BindImageMemoryDeviceGroupInfo::device_indices`] **must**  have been allocated
-    If the [`p_next`] chain includes a [`BindImageMemoryDeviceGroupInfo`] structure, and [`BindImageMemoryDeviceGroupInfo::split_instance_bind_region_count`] is not zero, then [`image`] **must**  have been created with the `VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT` bit set
-    If the [`p_next`] chain includes a [`BindImageMemoryDeviceGroupInfo`] structure, all elements of [`BindImageMemoryDeviceGroupInfo::split_instance_bind_regions`] **must**  be valid rectangles contained within the dimensions of [`image`]
-    If the [`p_next`] chain includes a [`BindImageMemoryDeviceGroupInfo`] structure, the union of the areas of all elements of [`BindImageMemoryDeviceGroupInfo::split_instance_bind_regions`] that correspond to the same instance of [`image`] **must**  cover the entire image
-    If [`image`] was created with a valid swapchain handle in [`ImageSwapchainCreateInfoKHR::swapchain`], then the [`p_next`] chain  **must**  include a [`BindImageMemorySwapchainInfoKHR`] structure containing the same swapchain handle
-    If the [`p_next`] chain includes a [`BindImageMemorySwapchainInfoKHR`] structure, [`memory`] **must**  be [`crate::Handle::null`]
-    If the [`p_next`] chain does not include a [`BindImageMemorySwapchainInfoKHR`] structure, [`memory`] **must**  be a valid [`DeviceMemory`] handle

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO`
-    Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**  be either `NULL` or a pointer to a valid instance of [`BindImageMemoryDeviceGroupInfo`], [`BindImageMemorySwapchainInfoKHR`], or [`BindImagePlaneMemoryInfo`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`image`] **must**  be a valid [`Image`] handle
-    Both of [`image`], and [`memory`] that are valid handles of non-ignored parameters  **must**  have been created, allocated, or retrieved from the same [`Device`]

# Related
- [`crate::vulkan1_1`]
- [`DeviceMemory`]
- [`DeviceSize`]
- [`Image`]
- [`StructureType`]
- [`bind_image_memory2`]
- [`bind_image_memory2_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        