[VkCopyBufferInfo2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyBufferInfo2.html) - Structure specifying parameters of a buffer copy command

# C Specifications
The [`CopyBufferInfo2`] structure is defined as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkCopyBufferInfo2 {
    VkStructureType         sType;
    const void*             pNext;
    VkBuffer                srcBuffer;
    VkBuffer                dstBuffer;
    uint32_t                regionCount;
    const VkBufferCopy2*    pRegions;
} VkCopyBufferInfo2;
```
or the equivalent
```c
// Provided by VK_KHR_copy_commands2
typedef VkCopyBufferInfo2 VkCopyBufferInfo2KHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`src_buffer`] is the source buffer.
- [`dst_buffer`] is the destination buffer.
- [`region_count`] is the number of regions to copy.
- [`regions`] is a pointer to an array of [`BufferCopy2`] structures specifying the regions to copy.

# Description
Members defined by this structure with the same name as parameters in
[`cmd_copy_buffer`] have the identical effect to those parameters; the
child structure [`BufferCopy2`] is a variant of [`BufferCopy`] which
includes [`s_type`] and [`p_next`] parameters, allowing it to be extended.
## Valid Usage
-    The `srcOffset` member of each element of [`regions`] **must**  be less than the size of [`src_buffer`]
-    The `dstOffset` member of each element of [`regions`] **must**  be less than the size of [`dst_buffer`]
-    The `size` member of each element of [`regions`] **must**  be less than or equal to the size of [`src_buffer`] minus `srcOffset`
-    The `size` member of each element of [`regions`] **must**  be less than or equal to the size of [`dst_buffer`] minus `dstOffset`
-    The union of the source regions, and the union of the destination regions, specified by the elements of [`regions`],  **must**  not overlap in memory
-  [`src_buffer`] **must**  have been created with `VK_BUFFER_USAGE_TRANSFER_SRC_BIT` usage flag
-    If [`src_buffer`] is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-  [`dst_buffer`] **must**  have been created with `VK_BUFFER_USAGE_TRANSFER_DST_BIT` usage flag
-    If [`dst_buffer`] is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_COPY_BUFFER_INFO_2`
-  [`p_next`] **must**  be `NULL`
-  [`src_buffer`] **must**  be a valid [`Buffer`] handle
-  [`dst_buffer`] **must**  be a valid [`Buffer`] handle
-  [`regions`] **must**  be a valid pointer to an array of [`region_count`] valid [`BufferCopy2`] structures
-  [`region_count`] **must**  be greater than `0`
-    Both of [`dst_buffer`], and [`src_buffer`] **must**  have been created, allocated, or retrieved from the same [`Device`]

# Related
- [`VK_KHR_copy_commands2`]
- [`crate::vulkan1_3`]
- [`Buffer`]
- [`BufferCopy2`]
- [`StructureType`]
- [`cmd_copy_buffer2`]
- [`cmd_copy_buffer2_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        