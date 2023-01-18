[vkGetDeviceGroupPeerMemoryFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupPeerMemoryFeatures.html) - Query supported peer memory features of a device

# C Specifications
*Peer memory* is memory that is allocated for a given physical device and
then bound to a resource and accessed by a different physical device, in a
logical device that represents multiple physical devices.
Some ways of reading and writing peer memory  **may**  not be supported by a
device.To determine how peer memory  **can**  be accessed, call:
```c
// Provided by VK_VERSION_1_1
void vkGetDeviceGroupPeerMemoryFeatures(
    VkDevice                                    device,
    uint32_t                                    heapIndex,
    uint32_t                                    localDeviceIndex,
    uint32_t                                    remoteDeviceIndex,
    VkPeerMemoryFeatureFlags*                   pPeerMemoryFeatures);
```
or the equivalent command
```c
// Provided by VK_KHR_device_group
void vkGetDeviceGroupPeerMemoryFeaturesKHR(
    VkDevice                                    device,
    uint32_t                                    heapIndex,
    uint32_t                                    localDeviceIndex,
    uint32_t                                    remoteDeviceIndex,
    VkPeerMemoryFeatureFlags*                   pPeerMemoryFeatures);
```

# Parameters
- [`device`] is the logical device that owns the memory.
- [`heap_index`] is the index of the memory heap from which the memory is allocated.
- [`local_device_index`] is the device index of the physical device that performs the memory access.
- [`remote_device_index`] is the device index of the physical device that the memory is allocated for.
- [`p_peer_memory_features`] is a pointer to a [`PeerMemoryFeatureFlags`] bitmask indicating which types of memory accesses are supported for the combination of heap, local, and remote devices.

# Description
## Valid Usage
-  [`heap_index`] **must**  be less than `memoryHeapCount`
-  [`local_device_index`] **must**  be a valid device index
-  [`remote_device_index`] **must**  be a valid device index
-  [`local_device_index`] **must**  not equal [`remote_device_index`]

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_peer_memory_features`] **must**  be a valid pointer to a [`PeerMemoryFeatureFlags`] value

# Related
- [`crate::vulkan1_1`]
- [`Device`]
- [`PeerMemoryFeatureFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        