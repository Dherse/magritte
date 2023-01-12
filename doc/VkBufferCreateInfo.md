[VkBufferCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCreateInfo.html) - Structure specifying the parameters of a newly created buffer object

# C Specifications
The [`BufferCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkBufferCreateInfo {
    VkStructureType        sType;
    const void*            pNext;
    VkBufferCreateFlags    flags;
    VkDeviceSize           size;
    VkBufferUsageFlags     usage;
    VkSharingMode          sharingMode;
    uint32_t               queueFamilyIndexCount;
    const uint32_t*        pQueueFamilyIndices;
} VkBufferCreateInfo;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is a bitmask of [`BufferCreateFlagBits`] specifying additional parameters of the buffer.
- [`size`] is the size in bytes of the buffer to be created.
- [`usage`] is a bitmask of [`BufferUsageFlagBits`] specifying allowed usages of the buffer.
- [`sharing_mode`] is a [`SharingMode`] value specifying the sharing mode of the buffer when it will be accessed by multiple queue families.
- [`queue_family_index_count`] is the number of entries in the [`queue_family_indices`] array.
- [`queue_family_indices`] is a pointer to an array of queue families that will access this buffer. It is ignored if [`sharing_mode`] is not `VK_SHARING_MODE_CONCURRENT`.

# Description
## Valid Usage
-  [`size`] **must**  be greater than `0`
-    If [`sharing_mode`] is `VK_SHARING_MODE_CONCURRENT`, [`queue_family_indices`] **must**  be a valid pointer to an array of [`queue_family_index_count`]`uint32_t` values
-    If [`sharing_mode`] is `VK_SHARING_MODE_CONCURRENT`, [`queue_family_index_count`] **must**  be greater than `1`
-    If [`sharing_mode`] is `VK_SHARING_MODE_CONCURRENT`, each element of [`queue_family_indices`] **must**  be unique and  **must**  be less than `pQueueFamilyPropertyCount` returned by either [`get_physical_device_queue_family_properties`] or [`get_physical_device_queue_family_properties2`] for the `physicalDevice` that was used to create `device`
-    If the [sparse bindings](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-sparseBinding) feature is not enabled, [`flags`] **must**  not contain `VK_BUFFER_CREATE_SPARSE_BINDING_BIT`
-    If the [sparse buffer residency](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-sparseResidencyBuffer) feature is not enabled, [`flags`] **must**  not contain `VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT`
-    If the [sparse aliased residency](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-sparseResidencyAliased) feature is not enabled, [`flags`] **must**  not contain `VK_BUFFER_CREATE_SPARSE_ALIASED_BIT`
-    If [`flags`] contains `VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT` or `VK_BUFFER_CREATE_SPARSE_ALIASED_BIT`, it  **must**  also contain `VK_BUFFER_CREATE_SPARSE_BINDING_BIT`
-    If the [`p_next`] chain includes a [`ExternalMemoryBufferCreateInfo`] structure, its `handleTypes` member  **must**  only contain bits that are also in [`ExternalBufferProperties`]::`externalMemoryProperties.compatibleHandleTypes`, as returned by [`get_physical_device_external_buffer_properties`] with `pExternalBufferInfo->handleType` equal to any one of the handle types specified in [`ExternalMemoryBufferCreateInfo::handle_types`]
-    If the protected memory feature is not enabled, [`flags`] **must**  not contain `VK_BUFFER_CREATE_PROTECTED_BIT`
-    If any of the bits `VK_BUFFER_CREATE_SPARSE_BINDING_BIT`, `VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT`, or `VK_BUFFER_CREATE_SPARSE_ALIASED_BIT` are set, `VK_BUFFER_CREATE_PROTECTED_BIT` **must**  not also be set
-    If the [`p_next`] chain includes a [`DedicatedAllocationBufferCreateInfoNV`] structure, and the `dedicatedAllocation` member of the chained structure is `VK_TRUE`, then [`flags`] **must**  not include `VK_BUFFER_CREATE_SPARSE_BINDING_BIT`, `VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT`, or `VK_BUFFER_CREATE_SPARSE_ALIASED_BIT`
-    If [`BufferDeviceAddressCreateInfoEXT::device_address`] is not zero, [`flags`] **must**  include `VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT`
-    If [`BufferOpaqueCaptureAddressCreateInfo::opaque_capture_address`] is not zero, [`flags`] **must**  include `VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT`
-    If [`flags`] includes `VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT`, the [bufferDeviceAddressCaptureReplay](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-bufferDeviceAddressCaptureReplay) or [[`PhysicalDeviceBufferDeviceAddressFeaturesEXT::buffer_device_address_capture_replay`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-bufferDeviceAddressCaptureReplayEXT) feature  **must**  be enabled
-    If [`usage`] includes `VK_BUFFER_USAGE_VIDEO_DECODE_SRC_BIT_KHR`, `VK_BUFFER_USAGE_VIDEO_DECODE_DST_BIT_KHR`, then the [`p_next`] chain  **must**  include a valid [`VideoProfilesKHR`] structure which includes at least one [`VideoProfileKHR`] with a decode codec-operation
-    If [`usage`] includes `VK_BUFFER_USAGE_VIDEO_ENCODE_SRC_BIT_KHR`, `VK_BUFFER_USAGE_VIDEO_ENCODE_DST_BIT_KHR`, then the [`p_next`] chain  **must**  include a valid [`VideoProfilesKHR`] structure which includes at least one [`VideoProfileKHR`] with a encode codec-operation
-  [`size`] **must**  be less than or equal to [`PhysicalDeviceMaintenance4Properties::max_buffer_size`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO`
-    Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**  be either `NULL` or a pointer to a valid instance of [`BufferCollectionBufferCreateInfoFUCHSIA`], [`BufferDeviceAddressCreateInfoEXT`], [`BufferOpaqueCaptureAddressCreateInfo`], [`DedicatedAllocationBufferCreateInfoNV`], [`ExternalMemoryBufferCreateInfo`], [`VideoDecodeH264ProfileEXT`], [`VideoDecodeH265ProfileEXT`], [`VideoEncodeH264ProfileEXT`], [`VideoEncodeH265ProfileEXT`], [`VideoProfileKHR`], or [`VideoProfilesKHR`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`flags`] **must**  be a valid combination of [`BufferCreateFlagBits`] values
-  [`usage`] **must**  be a valid combination of [`BufferUsageFlagBits`] values
-  [`usage`] **must**  not be `0`
-  [`sharing_mode`] **must**  be a valid [`SharingMode`] value

# Related
- [`crate::vulkan1_0`]
- [`BufferConstraintsInfoFUCHSIA`]
- [VkBufferCreateFlags]()
- [VkBufferUsageFlags]()
- [`DeviceBufferMemoryRequirements`]
- [`DeviceSize`]
- [`SharingMode`]
- [`StructureType`]
- [`create_buffer`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        