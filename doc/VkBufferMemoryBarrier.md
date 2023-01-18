[VkBufferMemoryBarrier](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferMemoryBarrier.html) - Structure specifying a buffer memory barrier

# C Specifications
The [`BufferMemoryBarrier`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkBufferMemoryBarrier {
    VkStructureType    sType;
    const void*        pNext;
    VkAccessFlags      srcAccessMask;
    VkAccessFlags      dstAccessMask;
    uint32_t           srcQueueFamilyIndex;
    uint32_t           dstQueueFamilyIndex;
    VkBuffer           buffer;
    VkDeviceSize       offset;
    VkDeviceSize       size;
} VkBufferMemoryBarrier;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`src_access_mask`] is a bitmask of [`AccessFlagBits`] specifying a [source access mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-masks).
- [`dst_access_mask`] is a bitmask of [`AccessFlagBits`] specifying a [destination access mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-masks).
- [`src_queue_family_index`] is the source queue family for a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers).
- [`dst_queue_family_index`] is the destination queue family for a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers).
- [`buffer`] is a handle to the buffer whose backing memory is affected by the barrier.
- [`offset`] is an offset in bytes into the backing memory for [`buffer`]; this is relative to the base offset as bound to the buffer (see [`bind_buffer_memory`]).
- [`size`] is a size in bytes of the affected area of backing memory for [`buffer`], or [`WHOLE_SIZE`] to use the range from [`offset`] to the end of the buffer.

# Description
The first [access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) is
limited to access to memory through the specified buffer range, via access
types in the [source access mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-masks) specified
by [`src_access_mask`].
If [`src_access_mask`] includes `VK_ACCESS_HOST_WRITE_BIT`, memory
writes performed by that access type are also made visible, as that access
type is not performed through a resource.The second [access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) is
limited to access to memory through the specified buffer range, via access
types in the [destination access mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-masks)
specified by [`dst_access_mask`].
If [`dst_access_mask`] includes `VK_ACCESS_HOST_WRITE_BIT` or
`VK_ACCESS_HOST_READ_BIT`, available memory writes are also made visible
to accesses of those types, as those access types are not performed through
a resource.If [`src_queue_family_index`] is not equal to [`dst_queue_family_index`], and
[`src_queue_family_index`] is equal to the current queue family, then the
memory barrier defines a [queue
family release operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers-release) for the specified buffer range, and the second
access scope includes no access, as if [`dst_access_mask`] was `0`.If [`dst_queue_family_index`] is not equal to [`src_queue_family_index`], and
[`dst_queue_family_index`] is equal to the current queue family, then the
memory barrier defines a [queue
family acquire operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers-acquire) for the specified buffer range, and the first
access scope includes no access, as if [`src_access_mask`] was `0`.
## Valid Usage
-  [`offset`] **must**  be less than the size of [`buffer`]
-    If [`size`] is not equal to [`WHOLE_SIZE`], [`size`] **must**  be greater than `0`
-    If [`size`] is not equal to [`WHOLE_SIZE`], [`size`] **must**  be less than or equal to than the size of [`buffer`] minus [`offset`]
-    If [`buffer`] is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-    If [`src_queue_family_index`] is not equal to [`dst_queue_family_index`], at least one  **must**  not be a special queue family reserved for external memory ownership transfers, as described in [[synchronization-queue-transfers]](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers)
-    If [`buffer`] was created with a sharing mode of `VK_SHARING_MODE_CONCURRENT`, [`src_queue_family_index`] and [`dst_queue_family_index`] are not equal, and one of [`src_queue_family_index`] and [`dst_queue_family_index`] is one of the special queue family values reserved for external memory transfers, the other  **must**  be [`QUEUE_FAMILY_IGNORED`]
-    If [`buffer`] was created with a sharing mode of `VK_SHARING_MODE_EXCLUSIVE`, and [`src_queue_family_index`] and [`dst_queue_family_index`] are not equal, [`src_queue_family_index`] and [`dst_queue_family_index`] **must**  both be valid queue families, or one of the special queue family values reserved for external memory transfers, as described in [[synchronization-queue-transfers]](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers)
-    If the [`synchronization2` feature](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-synchronization2) is not enabled, and [`buffer`] was created with a sharing mode of `VK_SHARING_MODE_CONCURRENT`, at least one of [`src_queue_family_index`] and [`dst_queue_family_index`] **must**  be [`QUEUE_FAMILY_IGNORED`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER`
-  [`p_next`] **must**  be `NULL`
-  [`buffer`] **must**  be a valid [`Buffer`] handle

# Related
- [`crate::vulkan1_0`]
- [`AccessFlags`]
- [`Buffer`]
- [`DeviceSize`]
- [`StructureType`]
- [`cmd_pipeline_barrier`]
- [`cmd_wait_events`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        