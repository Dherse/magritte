[vkBindImageMemory](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindImageMemory.html) - Bind device memory to an image object

# C Specifications
To attach memory to a [`Image`] object created without the
`VK_IMAGE_CREATE_DISJOINT_BIT` set, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkBindImageMemory(
    VkDevice                                    device,
    VkImage                                     image,
    VkDeviceMemory                              memory,
    VkDeviceSize                                memoryOffset);
```

# Parameters
- [`device`] is the logical device that owns the image and memory.
- [`image`] is the image.
- [`memory`] is the [`DeviceMemory`] object describing the device memory to attach.
- [`memory_offset`] is the start offset of the region of [`memory`] which is to be bound to the image. The number of bytes returned in the [`MemoryRequirements::size`] member in [`memory`], starting from [`memory_offset`] bytes, will be bound to the specified image.

# Description
[`bind_image_memory`] is equivalent to passing the same parameters through
[`BindImageMemoryInfo`] to [`bind_image_memory2`].
## Valid Usage
-  [`image`] **must**  not already be backed by a memory object
-  [`image`] **must**  not have been created with any sparse memory binding flags
-  [`memory_offset`] **must**  be less than the size of [`memory`]
-    If [`image`] requires a dedicated allocation (as reported by [`get_image_memory_requirements2`] in [`MemoryDedicatedRequirements::requires_dedicated_allocation`] for [`image`]), [`memory`] **must**  have been created with [`MemoryDedicatedAllocateInfo`]::[`image`] equal to [`image`]
-    If the [dedicated allocation image aliasing](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-dedicatedAllocationImageAliasing) feature is not enabled, and the [`MemoryAllocateInfo`] provided when [`memory`] was allocated included a [`MemoryDedicatedAllocateInfo`] structure in its `pNext` chain, and [`MemoryDedicatedAllocateInfo`]::[`image`] was not [`crate::Handle::null`], then [`image`] **must**  equal [`MemoryDedicatedAllocateInfo`]::[`image`] and [`memory_offset`] **must**  be zero
-    If the [dedicated allocation image aliasing](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-dedicatedAllocationImageAliasing) feature is enabled, and the [`MemoryAllocateInfo`] provided when [`memory`] was allocated included a [`MemoryDedicatedAllocateInfo`] structure in its `pNext` chain, and [`MemoryDedicatedAllocateInfo`]::[`image`] was not [`crate::Handle::null`], then [`memory_offset`] **must**  be zero, and [`image`] **must**  be either equal to [`MemoryDedicatedAllocateInfo`]::[`image`] or an image that was created using the same parameters in [`ImageCreateInfo`], with the exception that `extent` and `arrayLayers` **may**  differ subject to the following restrictions: every dimension in the `extent` parameter of the image being bound  **must**  be equal to or smaller than the original image for which the allocation was created; and the `arrayLayers` parameter of the image being bound  **must**  be equal to or smaller than the original image for which the allocation was created
-    If image was created with the `VK_IMAGE_CREATE_PROTECTED_BIT` bit set, the image  **must**  be bound to a memory object allocated with a memory type that reports `VK_MEMORY_PROPERTY_PROTECTED_BIT`
-    If image was created with the `VK_IMAGE_CREATE_PROTECTED_BIT` bit not set, the image  **must**  not be bound to a memory object created with a memory type that reports `VK_MEMORY_PROPERTY_PROTECTED_BIT`
-    If [`image`] was created with [`DedicatedAllocationImageCreateInfoNV::dedicated_allocation`] equal to `VK_TRUE`, [`memory`] **must**  have been created with [`DedicatedAllocationMemoryAllocateInfoNV`]::[`image`] equal to an image handle created with identical creation parameters to [`image`] and [`memory_offset`] **must**  be zero
-    If the value of [`ExportMemoryAllocateInfo::handle_types`] used to allocate [`memory`] is not `0`, it  **must**  include at least one of the handles set in [`ExternalMemoryImageCreateInfo::handle_types`] when [`image`] was created
-    If [`memory`] was created by a memory import operation, that is not [`ImportAndroidHardwareBufferInfoANDROID`] with a non-`NULL``buffer` value, the external handle type of the imported memory  **must**  also have been set in [`ExternalMemoryImageCreateInfo::handle_types`] when [`image`] was created
-    If [`memory`] was created with the [`ImportAndroidHardwareBufferInfoANDROID`] memory import operation with a non-`NULL``buffer` value, `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID` **must**  also have been set in [`ExternalMemoryImageCreateInfo::handle_types`] when [`image`] was created
-  [`image`] **must**  not have been created with the `VK_IMAGE_CREATE_DISJOINT_BIT` set
-  [`memory`] **must**  have been allocated using one of the memory types allowed in the `memoryTypeBits` member of the [`MemoryRequirements`] structure returned from a call to [`get_image_memory_requirements`] with [`image`]
-  [`memory_offset`] **must**  be an integer multiple of the `alignment` member of the [`MemoryRequirements`] structure returned from a call to [`get_image_memory_requirements`] with [`image`]
-    The difference of the size of [`memory`] and [`memory_offset`] **must**  be greater than or equal to the `size` member of the [`MemoryRequirements`] structure returned from a call to [`get_image_memory_requirements`] with the same [`image`]
-    If [`image`] was created with [`BufferCollectionImageCreateInfoFUCHSIA`] chained to [`ImageCreateInfo::p_next`], [`memory`] **must**  be allocated with a [`ImportMemoryBufferCollectionFUCHSIA`] chained to [`MemoryAllocateInfo::p_next`]

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`image`] **must**  be a valid [`Image`] handle
-  [`memory`] **must**  be a valid [`DeviceMemory`] handle
-  [`image`] **must**  have been created, allocated, or retrieved from [`device`]
-  [`memory`] **must**  have been created, allocated, or retrieved from [`device`]

## Host Synchronization
- Host access to [`image`] **must**  be externally synchronized

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`crate::vulkan1_0`]
- [`Device`]
- [`DeviceMemory`]
- [`DeviceSize`]
- [`Image`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        