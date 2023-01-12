[VkBindImageMemoryDeviceGroupInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindImageMemoryDeviceGroupInfo.html) - Structure specifying device within a group to bind to

# C Specifications
The [`BindImageMemoryDeviceGroupInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkBindImageMemoryDeviceGroupInfo {
    VkStructureType    sType;
    const void*        pNext;
    uint32_t           deviceIndexCount;
    const uint32_t*    pDeviceIndices;
    uint32_t           splitInstanceBindRegionCount;
    const VkRect2D*    pSplitInstanceBindRegions;
} VkBindImageMemoryDeviceGroupInfo;
```
or the equivalent
```c
// Provided by VK_KHR_bind_memory2 with VK_KHR_device_group
typedef VkBindImageMemoryDeviceGroupInfo VkBindImageMemoryDeviceGroupInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`device_index_count`] is the number of elements in [`device_indices`].
- [`device_indices`] is a pointer to an array of device indices.
- [`split_instance_bind_region_count`] is the number of elements in [`split_instance_bind_regions`].
- [`split_instance_bind_regions`] is a pointer to an array of [`Rect2D`] structures describing which regions of the image are attached to each instance of memory.

# Description
If the [`p_next`] chain of [`BindImageMemoryInfo`] includes a
[`BindImageMemoryDeviceGroupInfo`] structure, then that structure
determines how memory is bound to images across multiple devices in a device
group.If [`device_index_count`] is greater than zero, then on device index i`image` is attached to the instance of the memory on the physical device
with device index pDeviceIndices[i].Let N be the number of physical devices in the logical device.
If [`split_instance_bind_region_count`] is greater than zero, then
[`split_instance_bind_regions`] is a pointer to an array of N<sup>2</sup>
rectangles, where the image region specified by the rectangle at element
i*N+j in resource instance i is bound to the memory instance
j.
The blocks of the memory that are bound to each sparse image block region
use an offset in memory, relative to `memoryOffset`, computed as if the
whole image was being bound to a contiguous range of memory.
In other words, horizontally adjacent image blocks use consecutive blocks of
memory, vertically adjacent image blocks are separated by the number of
bytes per block multiplied by the width in blocks of `image`, and the
block at (0,0) corresponds to memory starting at `memoryOffset`.If [`split_instance_bind_region_count`] and [`device_index_count`] are zero
and the memory comes from a memory heap with the
`VK_MEMORY_HEAP_MULTI_INSTANCE_BIT` bit set, then it is as if
[`device_indices`] contains consecutive indices from zero to the number of
physical devices in the logical device, minus one.
In other words, by default each physical device attaches to its own instance
of the memory.If [`split_instance_bind_region_count`] and [`device_index_count`] are zero
and the memory comes from a memory heap without the
`VK_MEMORY_HEAP_MULTI_INSTANCE_BIT` bit set, then it is as if
[`device_indices`] contains an array of zeros.
In other words, by default each physical device attaches to instance zero.
## Valid Usage
-    At least one of [`device_index_count`] and [`split_instance_bind_region_count`] **must**  be zero
-  [`device_index_count`] **must**  either be zero or equal to the number of physical devices in the logical device
-    All elements of [`device_indices`] **must**  be valid device indices
-  [`split_instance_bind_region_count`] **must**  either be zero or equal to the number of physical devices in the logical device squared
-    Elements of [`split_instance_bind_regions`] that correspond to the same instance of an image  **must**  not overlap
-    The `offset.x` member of any element of [`split_instance_bind_regions`] **must**  be a multiple of the sparse image block width ([`SparseImageFormatProperties`]::`imageGranularity.width`) of all non-metadata aspects of the image
-    The `offset.y` member of any element of [`split_instance_bind_regions`] **must**  be a multiple of the sparse image block height ([`SparseImageFormatProperties`]::`imageGranularity.height`) of all non-metadata aspects of the image
-    The `extent.width` member of any element of [`split_instance_bind_regions`] **must**  either be a multiple of the sparse image block width of all non-metadata aspects of the image, or else `extent.width` +  `offset.x` **must**  equal the width of the image subresource
-    The `extent.height` member of any element of [`split_instance_bind_regions`] **must**  either be a multiple of the sparse image block height of all non-metadata aspects of the image, or else `extent.height` +  `offset.y` **must**  equal the height of the image subresource

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO`
-    If [`device_index_count`] is not `0`, [`device_indices`] **must**  be a valid pointer to an array of [`device_index_count`]`uint32_t` values
-    If [`split_instance_bind_region_count`] is not `0`, [`split_instance_bind_regions`] **must**  be a valid pointer to an array of [`split_instance_bind_region_count`][`Rect2D`] structures

# Related
- [`crate::vulkan1_1`]
- [`Rect2D`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        