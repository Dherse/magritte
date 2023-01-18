[VkMemoryPropertyFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryPropertyFlagBits.html) - Bitmask specifying properties for a memory type

# C Specifications
Bits which  **may**  be set in [`MemoryType::property_flags`],
indicating properties of a memory heap, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkMemoryPropertyFlagBits {
    VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT = 0x00000001,
    VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT = 0x00000002,
    VK_MEMORY_PROPERTY_HOST_COHERENT_BIT = 0x00000004,
    VK_MEMORY_PROPERTY_HOST_CACHED_BIT = 0x00000008,
    VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT = 0x00000010,
  // Provided by VK_VERSION_1_1
    VK_MEMORY_PROPERTY_PROTECTED_BIT = 0x00000020,
  // Provided by VK_AMD_device_coherent_memory
    VK_MEMORY_PROPERTY_DEVICE_COHERENT_BIT_AMD = 0x00000040,
  // Provided by VK_AMD_device_coherent_memory
    VK_MEMORY_PROPERTY_DEVICE_UNCACHED_BIT_AMD = 0x00000080,
  // Provided by VK_NV_external_memory_rdma
    VK_MEMORY_PROPERTY_RDMA_CAPABLE_BIT_NV = 0x00000100,
} VkMemoryPropertyFlagBits;
```

# Description
- [`DEVICE_LOCAL`] bit specifies that memory allocated with this type is the most efficient for device access. This property will be set if and only if the memory type belongs to a heap with the `VK_MEMORY_HEAP_DEVICE_LOCAL_BIT` set.
- [`HOST_VISIBLE`] bit specifies that memory allocated with this type  **can**  be mapped for host access using [`map_memory`].
- [`HOST_COHERENT`] bit specifies that the host cache management commands [`flush_mapped_memory_ranges`] and [`invalidate_mapped_memory_ranges`] are not needed to flush host writes to the device or make device writes visible to the host, respectively.
- [`HOST_CACHED`] bit specifies that memory allocated with this type is cached on the host. Host memory accesses to uncached memory are slower than to cached memory, however uncached memory is always host coherent.
- [`LAZILY_ALLOCATED`] bit specifies that the memory type only allows device access to the memory. Memory types  **must**  not have both [`LAZILY_ALLOCATED`] and [`HOST_VISIBLE`] set. Additionally, the object’s backing memory  **may**  be provided by the implementation lazily as specified in [Lazily Allocated Memory](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-device-lazy_allocation).
- [`PROTECTED`] bit specifies that the memory type only allows device access to the memory, and allows protected queue operations to access the memory. Memory types  **must**  not have [`PROTECTED`] set and any of [`HOST_VISIBLE`] set, or [`HOST_COHERENT`] set, or [`HOST_CACHED`] set.
- [`DEVICE_COHERENT_AMD`] bit specifies that device accesses to allocations of this memory type are automatically made available and visible.
- [`DEVICE_UNCACHED_AMD`] bit specifies that memory allocated with this type is not cached on the device. Uncached device memory is always device coherent.
- [`RDMA_CAPABLE_NV`] bit specifies that external devices can access this memory directly.
For any memory allocated with both the
[`HOST_COHERENT`] and the
[`DEVICE_COHERENT_AMD`], host or device accesses
also perform automatic memory domain transfer operations, such that writes
are always automatically available and visible to both host and device
memory domains.

# Related
- [`crate::vulkan1_0`]
- [`MemoryPropertyFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        