[VkPhysicalDeviceMemoryProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMemoryProperties.html) - Structure specifying physical device memory properties

# C Specifications
The [`PhysicalDeviceMemoryProperties`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkPhysicalDeviceMemoryProperties {
    uint32_t        memoryTypeCount;
    VkMemoryType    memoryTypes[VK_MAX_MEMORY_TYPES];
    uint32_t        memoryHeapCount;
    VkMemoryHeap    memoryHeaps[VK_MAX_MEMORY_HEAPS];
} VkPhysicalDeviceMemoryProperties;
```

# Members
- [`memory_type_count`] is the number of valid elements in the [`memory_types`] array.
- [`memory_types`] is an array of `VK_MAX_MEMORY_TYPES`[`MemoryType`] structures describing the *memory types* that  **can**  be used to access memory allocated from the heaps specified by [`memory_heaps`].
- [`memory_heap_count`] is the number of valid elements in the [`memory_heaps`] array.
- [`memory_heaps`] is an array of `VK_MAX_MEMORY_HEAPS`[`MemoryHeap`] structures describing the *memory heaps* from which memory  **can**  be allocated.

# Description
The [`PhysicalDeviceMemoryProperties`] structure describes a number of
*memory heaps* as well as a number of *memory types* that  **can**  be used to
access memory allocated in those heaps.
Each heap describes a memory resource of a particular size, and each memory
type describes a set of memory properties (e.g. host cached vs uncached)
that  **can**  be used with a given memory heap.
Allocations using a particular memory type will consume resources from the
heap indicated by that memory type’s heap index.
More than one memory type  **may**  share each heap, and the heaps and memory
types provide a mechanism to advertise an accurate size of the physical
memory resources while allowing the memory to be used with a variety of
different properties.The number of memory heaps is given by [`memory_heap_count`] and is less
than or equal to `VK_MAX_MEMORY_HEAPS`.
Each heap is described by an element of the [`memory_heaps`] array as a
[`MemoryHeap`] structure.
The number of memory types available across all memory heaps is given by
[`memory_type_count`] and is less than or equal to
`VK_MAX_MEMORY_TYPES`.
Each memory type is described by an element of the [`memory_types`] array
as a [`MemoryType`] structure.At least one heap  **must**  include `VK_MEMORY_HEAP_DEVICE_LOCAL_BIT` in
[`MemoryHeap::flags`].
If there are multiple heaps that all have similar performance
characteristics, they  **may**  all include
`VK_MEMORY_HEAP_DEVICE_LOCAL_BIT`.
In a unified memory architecture (UMA) system there is often only a single
memory heap which is considered to be equally “local” to the host and to
the device, and such an implementation  **must**  advertise the heap as
device-local.Each memory type returned by [`get_physical_device_memory_properties`] **must** 
have its `propertyFlags` set to one of the following values:
- 0
- `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT` | `VK_MEMORY_PROPERTY_HOST_COHERENT_BIT`
- `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT` | `VK_MEMORY_PROPERTY_HOST_CACHED_BIT`
- `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT` | `VK_MEMORY_PROPERTY_HOST_CACHED_BIT` | `VK_MEMORY_PROPERTY_HOST_COHERENT_BIT`
- `VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT`
- `VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT` | `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT` | `VK_MEMORY_PROPERTY_HOST_COHERENT_BIT`
- `VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT` | `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT` | `VK_MEMORY_PROPERTY_HOST_CACHED_BIT`
- `VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT` | `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT` | `VK_MEMORY_PROPERTY_HOST_CACHED_BIT` | `VK_MEMORY_PROPERTY_HOST_COHERENT_BIT`
- `VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT` | `VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT`
- `VK_MEMORY_PROPERTY_PROTECTED_BIT`
- `VK_MEMORY_PROPERTY_PROTECTED_BIT` | `VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT`
- `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT` | `VK_MEMORY_PROPERTY_HOST_COHERENT_BIT` | `VK_MEMORY_PROPERTY_DEVICE_COHERENT_BIT_AMD`
- `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT` | `VK_MEMORY_PROPERTY_HOST_CACHED_BIT` | `VK_MEMORY_PROPERTY_HOST_COHERENT_BIT` | `VK_MEMORY_PROPERTY_DEVICE_COHERENT_BIT_AMD`
- `VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT` | `VK_MEMORY_PROPERTY_DEVICE_COHERENT_BIT_AMD`
- `VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT` | `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT` | `VK_MEMORY_PROPERTY_HOST_COHERENT_BIT` | `VK_MEMORY_PROPERTY_DEVICE_COHERENT_BIT_AMD`
- `VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT` | `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT` | `VK_MEMORY_PROPERTY_HOST_CACHED_BIT` | `VK_MEMORY_PROPERTY_HOST_COHERENT_BIT` | `VK_MEMORY_PROPERTY_DEVICE_COHERENT_BIT_AMD`
- `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT` | `VK_MEMORY_PROPERTY_HOST_COHERENT_BIT` | `VK_MEMORY_PROPERTY_DEVICE_COHERENT_BIT_AMD` | `VK_MEMORY_PROPERTY_DEVICE_UNCACHED_BIT_AMD`
- `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT` | `VK_MEMORY_PROPERTY_HOST_CACHED_BIT` | `VK_MEMORY_PROPERTY_HOST_COHERENT_BIT` | `VK_MEMORY_PROPERTY_DEVICE_COHERENT_BIT_AMD` | `VK_MEMORY_PROPERTY_DEVICE_UNCACHED_BIT_AMD`
- `VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT` | `VK_MEMORY_PROPERTY_DEVICE_COHERENT_BIT_AMD` | `VK_MEMORY_PROPERTY_DEVICE_UNCACHED_BIT_AMD`
- `VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT` | `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT` | `VK_MEMORY_PROPERTY_HOST_COHERENT_BIT` | `VK_MEMORY_PROPERTY_DEVICE_COHERENT_BIT_AMD` | `VK_MEMORY_PROPERTY_DEVICE_UNCACHED_BIT_AMD`
- `VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT` | `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT` | `VK_MEMORY_PROPERTY_HOST_CACHED_BIT` | `VK_MEMORY_PROPERTY_HOST_COHERENT_BIT` | `VK_MEMORY_PROPERTY_DEVICE_COHERENT_BIT_AMD` | `VK_MEMORY_PROPERTY_DEVICE_UNCACHED_BIT_AMD`
- `VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT` | `VK_MEMORY_PROPERTY_RDMA_CAPABLE_BIT_NV`
There  **must**  be at least one memory type with both the
`VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT` and
`VK_MEMORY_PROPERTY_HOST_COHERENT_BIT` bits set in its
`propertyFlags`.
There  **must**  be at least one memory type with the
`VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT` bit set in its
`propertyFlags`.
If the [`deviceCoherentMemory`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-deviceCoherentMemory) feature
is enabled, there  **must**  be at least one memory type with the
`VK_MEMORY_PROPERTY_DEVICE_COHERENT_BIT_AMD` bit set in its
`propertyFlags`.For each pair of elements  **X**  and  **Y**  returned in [`memory_types`],  **X**  **must**  be placed at a lower index position than  **Y**  if:
- the set of bit flags returned in the `propertyFlags` member of  **X**  is a strict subset of the set of bit flags returned in the `propertyFlags` member of  **Y** ; or
- the `propertyFlags` members of  **X**  and  **Y**  are equal, and  **X**  belongs to a memory heap with greater performance (as determined in an implementation-specific manner) ; or
- the `propertyFlags` members of  **Y**  includes `VK_MEMORY_PROPERTY_DEVICE_COHERENT_BIT_AMD` or `VK_MEMORY_PROPERTY_DEVICE_UNCACHED_BIT_AMD` and  **X**  does not
This ordering requirement enables applications to use a simple search loop
to select the desired memory type along the lines of:
```c
// Find a memory in `memoryTypeBitsRequirement` that includes all of `requiredProperties`
int32_t findProperties(const VkPhysicalDeviceMemoryProperties* pMemoryProperties,
                       uint32_t memoryTypeBitsRequirement,
                       VkMemoryPropertyFlags requiredProperties) {
    const uint32_t memoryCount = pMemoryProperties->memoryTypeCount;
    for (uint32_t memoryIndex = 0; memoryIndex < memoryCount; ++memoryIndex) {
        const uint32_t memoryTypeBits = (1 << memoryIndex);
        const bool isRequiredMemoryType = memoryTypeBitsRequirement & memoryTypeBits;

        const VkMemoryPropertyFlags properties =
            pMemoryProperties->memoryTypes[memoryIndex].propertyFlags;
        const bool hasRequiredProperties =
            (properties & requiredProperties) == requiredProperties;

        if (isRequiredMemoryType && hasRequiredProperties)
            return static_cast<int32_t>(memoryIndex);
    }

    // failed to find memory type
    return -1;
}

// Try to find an optimal memory type, or if it does not exist try fallback memory type
// `device` is the VkDevice
// `image` is the VkImage that requires memory to be bound
// `memoryProperties` properties as returned by vkGetPhysicalDeviceMemoryProperties
// `requiredProperties` are the property flags that must be present
// `optimalProperties` are the property flags that are preferred by the application
VkMemoryRequirements memoryRequirements;
vkGetImageMemoryRequirements(device, image, &memoryRequirements);
int32_t memoryType =
    findProperties(&memoryProperties, memoryRequirements.memoryTypeBits, optimalProperties);
if (memoryType == -1) // not found; try fallback properties
    memoryType =
        findProperties(&memoryProperties, memoryRequirements.memoryTypeBits, requiredProperties);
```

# Related
- [`crate::vulkan1_0`]
- [`MemoryHeap`]
- [`MemoryType`]
- [`PhysicalDeviceMemoryProperties2`]
- [`get_physical_device_memory_properties`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        