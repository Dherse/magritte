[VkSparseImageMemoryBind](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSparseImageMemoryBind.html) - Structure specifying sparse image memory bind

# C Specifications
The [`SparseImageMemoryBind`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkSparseImageMemoryBind {
    VkImageSubresource         subresource;
    VkOffset3D                 offset;
    VkExtent3D                 extent;
    VkDeviceMemory             memory;
    VkDeviceSize               memoryOffset;
    VkSparseMemoryBindFlags    flags;
} VkSparseImageMemoryBind;
```

# Members
- [`subresource`] is the image *aspect* and region of interest in the image.
- [`offset`] are the coordinates of the first texel within the image subresource to bind.
- [`extent`] is the size in texels of the region within the image subresource to bind. The extent  **must**  be a multiple of the sparse image block dimensions, except when binding sparse image blocks along the edge of an image subresource it  **can**  instead be such that any coordinate of [`offset`] +  [`extent`] equals the corresponding dimensions of the image subresource.
- [`memory`] is the [`DeviceMemory`] object that the sparse image blocks of the image are bound to. If [`memory`] is [`crate::Handle::null`], the sparse image blocks are unbound.
- [`memory_offset`] is an offset into [`DeviceMemory`] object. If [`memory`] is [`crate::Handle::null`], this value is ignored.
- [`flags`] are sparse memory binding flags.

# Description
## Valid Usage
-    If the [sparse aliased residency](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-sparseResidencyAliased) feature is not enabled, and if any other resources are bound to ranges of [`memory`], the range of [`memory`] being bound  **must**  not overlap with those bound ranges
-  [`memory`] and [`memory_offset`] **must**  match the memory requirements of the calling command’s `image`, as described in section [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-association](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-association)
-  [`subresource`] **must**  be a valid image subresource for `image` (see [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views))
-  `offset.x` **must**  be a multiple of the sparse image block width ([`SparseImageFormatProperties`]::`imageGranularity.width`) of the image
-  `extent.width` **must**  either be a multiple of the sparse image block width of the image, or else (`extent.width` +  `offset.x`) **must**  equal the width of the image subresource
-  `offset.y` **must**  be a multiple of the sparse image block height ([`SparseImageFormatProperties`]::`imageGranularity.height`) of the image
-  `extent.height` **must**  either be a multiple of the sparse image block height of the image, or else (`extent.height` +  `offset.y`) **must**  equal the height of the image subresource
-  `offset.z` **must**  be a multiple of the sparse image block depth ([`SparseImageFormatProperties`]::`imageGranularity.depth`) of the image
-  `extent.depth` **must**  either be a multiple of the sparse image block depth of the image, or else (`extent.depth` +  `offset.z`) **must**  equal the depth of the image subresource
-    If [`memory`] was created with [`ExportMemoryAllocateInfo::handle_types`] not equal to `0`, at least one handle type it contained  **must**  also have been set in [`ExternalMemoryImageCreateInfo::handle_types`] when the image was created
-    If [`memory`] was created by a memory import operation, the external handle type of the imported memory  **must**  also have been set in [`ExternalMemoryImageCreateInfo::handle_types`] when `image` was created

## Valid Usage (Implicit)
-  [`subresource`] **must**  be a valid [`ImageSubresource`] structure
-    If [`memory`] is not [`crate::Handle::null`], [`memory`] **must**  be a valid [`DeviceMemory`] handle
-  [`flags`] **must**  be a valid combination of [`SparseMemoryBindFlagBits`] values

# Related
- [`crate::vulkan1_0`]
- [`DeviceMemory`]
- [`DeviceSize`]
- [`Extent3D`]
- [`ImageSubresource`]
- [`Offset3D`]
- [`SparseImageMemoryBindInfo`]
- [VkSparseMemoryBindFlags]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        