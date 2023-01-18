[VkSparseMemoryBind](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSparseMemoryBind.html) - Structure specifying a sparse memory bind operation

# C Specifications
The [`SparseMemoryBind`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkSparseMemoryBind {
    VkDeviceSize               resourceOffset;
    VkDeviceSize               size;
    VkDeviceMemory             memory;
    VkDeviceSize               memoryOffset;
    VkSparseMemoryBindFlags    flags;
} VkSparseMemoryBind;
```

# Members
- [`resource_offset`] is the offset into the resource.
- [`size`] is the size of the memory region to be bound.
- [`memory`] is the [`DeviceMemory`] object that the range of the resource is bound to. If [`memory`] is [`crate::Handle::null`], the range is unbound.
- [`memory_offset`] is the offset into the [`DeviceMemory`] object to bind the resource range to. If [`memory`] is [`crate::Handle::null`], this value is ignored.
- [`flags`] is a bitmask of [`SparseMemoryBindFlagBits`] specifying usage of the binding operation.

# Description
The *binding range*[[`resource_offset`], [`resource_offset`] + 
[`size`]) has different constraints based on [`flags`].
If [`flags`] contains `VK_SPARSE_MEMORY_BIND_METADATA_BIT`, the
binding range  **must**  be within the mip tail region of the metadata aspect.
This metadata region is defined by:
* metadataRegion = [base, base +  `imageMipTailSize`)
* base = `imageMipTailOffset` +  `imageMipTailStride` × n
and `imageMipTailOffset`, `imageMipTailSize`, and
`imageMipTailStride` values are from the
[`SparseImageMemoryRequirements`] corresponding to the metadata aspect
of the image, and n is a valid array layer index for the image,`imageMipTailStride` is considered to be zero for aspects where
[`SparseImageMemoryRequirements`]::`formatProperties.flags` contains
`VK_SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT`.If [`flags`] does not contain `VK_SPARSE_MEMORY_BIND_METADATA_BIT`,
the binding range  **must**  be within the range
[0,[`MemoryRequirements`]::[`size`]).
## Valid Usage
-    If [`memory`] is not [`crate::Handle::null`], [`memory`] and [`memory_offset`] **must**  match the memory requirements of the resource, as described in section [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-association](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-association)
-    If [`memory`] is not [`crate::Handle::null`], [`memory`] **must**  not have been created with a memory type that reports `VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT` bit set
-  [`size`] **must**  be greater than `0`
-  [`resource_offset`] **must**  be less than the size of the resource
-  [`size`] **must**  be less than or equal to the size of the resource minus [`resource_offset`]
-  [`memory_offset`] **must**  be less than the size of [`memory`]
-  [`size`] **must**  be less than or equal to the size of [`memory`] minus [`memory_offset`]
-    If [`memory`] was created with [`ExportMemoryAllocateInfo::handle_types`] not equal to `0`, at least one handle type it contained  **must**  also have been set in [`ExternalMemoryBufferCreateInfo::handle_types`] or [`ExternalMemoryImageCreateInfo::handle_types`] when the resource was created
-    If [`memory`] was created by a memory import operation, the external handle type of the imported memory  **must**  also have been set in [`ExternalMemoryBufferCreateInfo::handle_types`] or [`ExternalMemoryImageCreateInfo::handle_types`] when the resource was created

## Valid Usage (Implicit)
-    If [`memory`] is not [`crate::Handle::null`], [`memory`] **must**  be a valid [`DeviceMemory`] handle
-  [`flags`] **must**  be a valid combination of [`SparseMemoryBindFlagBits`] values

# Related
- [`crate::vulkan1_0`]
- [`DeviceMemory`]
- [`DeviceSize`]
- [`SparseBufferMemoryBindInfo`]
- [`SparseImageOpaqueMemoryBindInfo`]
- [`SparseMemoryBindFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        