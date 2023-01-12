[vkWriteAccelerationStructuresPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWriteAccelerationStructuresPropertiesKHR.html) - Query acceleration structure meta-data on the host

# C Specifications
To query acceleration structure size parameters on the host, call:
```c
// Provided by VK_KHR_acceleration_structure
VkResult vkWriteAccelerationStructuresPropertiesKHR(
    VkDevice                                    device,
    uint32_t                                    accelerationStructureCount,
    const VkAccelerationStructureKHR*           pAccelerationStructures,
    VkQueryType                                 queryType,
    size_t                                      dataSize,
    void*                                       pData,
    size_t                                      stride);
```

# Parameters
- [`device`] is the device which owns the acceleration structures in [`p_acceleration_structures`].
- [`acceleration_structure_count`] is the count of acceleration structures for which to query the property.
- [`p_acceleration_structures`] is a pointer to an array of existing previously built acceleration structures.
- [`query_type`] is a [`QueryType`] value specifying the property to be queried.
- [`data_size`] is the size in bytes of the buffer pointed to by [`p_data`].
- [`p_data`] is a pointer to a user-allocated buffer where the results will be written.
- [`stride`] is the stride in bytes between results for individual queries within [`p_data`].

# Description
This command fulfills the same task as
[`cmd_write_acceleration_structures_properties_khr`] but is executed by the
host.
## Valid Usage
-    All acceleration structures in [`p_acceleration_structures`] **must**  have been built prior to the execution of this command
-    All acceleration structures in [`p_acceleration_structures`] **must**  have been built with `VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR` if [`query_type`] is `VK_QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR`
-  [`query_type`] **must**  be `VK_QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR` or `VK_QUERY_TYPE_ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR`
-    If [`query_type`] is `VK_QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR`, then [`stride`] **must**  be a multiple of the size of [`DeviceSize`]
-    If [`query_type`] is `VK_QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR`, then `data` **must**  point to a [`DeviceSize`]
-    If [`query_type`] is `VK_QUERY_TYPE_ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR`, then [`stride`] **must**  be a multiple of the size of [`DeviceSize`]
-    If [`query_type`] is `VK_QUERY_TYPE_ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR`, then `data` **must**  point to a [`DeviceSize`]
-  [`data_size`] **must**  be greater than or equal to [`acceleration_structure_count`]*[`stride`]
-    The `buffer` used to create each acceleration structure in [`p_acceleration_structures`] **must**  be bound to host-visible device memory
-    The [[`PhysicalDeviceAccelerationStructureFeaturesKHR::acceleration_structure_host_commands`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-accelerationStructureHostCommands) feature  **must**  be enabled
-    The `buffer` used to create each acceleration structure in [`p_acceleration_structures`] **must**  be bound to memory that was not allocated with multiple instances

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_acceleration_structures`] **must**  be a valid pointer to an array of [`acceleration_structure_count`] valid [`AccelerationStructureKHR`] handles
-  [`query_type`] **must**  be a valid [`QueryType`] value
-  [`p_data`] **must**  be a valid pointer to an array of [`data_size`] bytes
-  [`acceleration_structure_count`] **must**  be greater than `0`
-  [`data_size`] **must**  be greater than `0`
-    Each element of [`p_acceleration_structures`] **must**  have been created, allocated, or retrieved from [`device`]

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`khr_acceleration_structure`]
- [`AccelerationStructureKHR`]
- [`Device`]
- [`QueryType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        