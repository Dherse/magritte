[vkBindBufferMemory](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindBufferMemory.html) - Bind device memory to a buffer object

# C Specifications
To attach memory to a buffer object, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkBindBufferMemory(
    VkDevice                                    device,
    VkBuffer                                    buffer,
    VkDeviceMemory                              memory,
    VkDeviceSize                                memoryOffset);
```

# Parameters
- [`device`] is the logical device that owns the buffer and memory.
- [`buffer`] is the buffer to be attached to memory.
- [`memory`] is a [`DeviceMemory`] object describing the device memory to attach.
- [`memory_offset`] is the start offset of the region of [`memory`] which is to be bound to the buffer. The number of bytes returned in the [`MemoryRequirements::size`] member in [`memory`], starting from [`memory_offset`] bytes, will be bound to the specified buffer.

# Description
[`bind_buffer_memory`] is equivalent to passing the same parameters
through [`BindBufferMemoryInfo`] to [`bind_buffer_memory2`].
## Valid Usage
-  [`buffer`] **must**  not already be backed by a memory object
-  [`buffer`] **must**  not have been created with any sparse memory binding flags
-  [`memory_offset`] **must**  be less than the size of [`memory`]
-  [`memory`] **must**  have been allocated using one of the memory types allowed in the `memoryTypeBits` member of the [`MemoryRequirements`] structure returned from a call to [`get_buffer_memory_requirements`] with [`buffer`]
-  [`memory_offset`] **must**  be an integer multiple of the `alignment` member of the [`MemoryRequirements`] structure returned from a call to [`get_buffer_memory_requirements`] with [`buffer`]
-    The `size` member of the [`MemoryRequirements`] structure returned from a call to [`get_buffer_memory_requirements`] with [`buffer`] **must**  be less than or equal to the size of [`memory`] minus [`memory_offset`]
-    If [`buffer`] requires a dedicated allocation (as reported by [`get_buffer_memory_requirements2`] in [`MemoryDedicatedRequirements::requires_dedicated_allocation`] for [`buffer`]), [`memory`] **must**  have been allocated with [`MemoryDedicatedAllocateInfo`]::[`buffer`] equal to [`buffer`]
-    If the [`MemoryAllocateInfo`] provided when [`memory`] was allocated included a [`MemoryDedicatedAllocateInfo`] structure in its `pNext` chain, and [`MemoryDedicatedAllocateInfo`]::[`buffer`] was not [`crate::Handle::null`], then [`buffer`] **must**  equal [`MemoryDedicatedAllocateInfo`]::[`buffer`], and [`memory_offset`] **must**  be zero
-    If [`buffer`] was created with the `VK_BUFFER_CREATE_PROTECTED_BIT` bit set, the buffer  **must**  be bound to a memory object allocated with a memory type that reports `VK_MEMORY_PROPERTY_PROTECTED_BIT`
-    If [`buffer`] was created with the `VK_BUFFER_CREATE_PROTECTED_BIT` bit not set, the buffer  **must**  not be bound to a memory object allocated with a memory type that reports `VK_MEMORY_PROPERTY_PROTECTED_BIT`
-    If [`buffer`] was created with [`DedicatedAllocationBufferCreateInfoNV::dedicated_allocation`] equal to [`TRUE`], [`memory`] **must**  have been allocated with [`DedicatedAllocationMemoryAllocateInfoNV`]::[`buffer`] equal to a buffer handle created with identical creation parameters to [`buffer`] and [`memory_offset`] **must**  be zero
-    If the value of [`ExportMemoryAllocateInfo::handle_types`] used to allocate [`memory`] is not `0`, it  **must**  include at least one of the handles set in [`ExternalMemoryBufferCreateInfo::handle_types`] when [`buffer`] was created
-    If [`memory`] was allocated by a memory import operation, that is not [`ImportAndroidHardwareBufferInfoANDROID`] with a non-`NULL`[`buffer`] value, the external handle type of the imported memory  **must**  also have been set in [`ExternalMemoryBufferCreateInfo::handle_types`] when [`buffer`] was created
-    If [`memory`] was allocated with the [`ImportAndroidHardwareBufferInfoANDROID`] memory import operation with a non-`NULL`[`buffer`] value, `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID` **must**  also have been set in [`ExternalMemoryBufferCreateInfo::handle_types`] when [`buffer`] was created
-    If the [`PhysicalDeviceBufferDeviceAddressFeatures::buffer_device_address`] feature is enabled and [`buffer`] was created with the `VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT` bit set, [`memory`] **must**  have been allocated with the `VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT` bit set
-    If [`buffer`] was created with [`BufferCollectionBufferCreateInfoFUCHSIA`] chained to [`BufferCreateInfo::p_next`], [`memory`] **must**  be allocated with a [`ImportMemoryBufferCollectionFUCHSIA`] chained to [`MemoryAllocateInfo::p_next`]

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`buffer`] **must**  be a valid [`Buffer`] handle
-  [`memory`] **must**  be a valid [`DeviceMemory`] handle
-  [`buffer`] **must**  have been created, allocated, or retrieved from [`device`]
-  [`memory`] **must**  have been created, allocated, or retrieved from [`device`]

## Host Synchronization
- Host access to [`buffer`] **must**  be externally synchronized

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`

# Related
- [`crate::vulkan1_0`]
- [`Buffer`]
- [`Device`]
- [`DeviceMemory`]
- [`DeviceSize`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        