[VkMemoryAllocateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryAllocateFlagBits.html) - Bitmask specifying flags for a device memory allocation

# C Specifications
Bits which  **can**  be set in [`MemoryAllocateFlagsInfo::flags`],
controlling device memory allocation, are:
```c
// Provided by VK_VERSION_1_1
typedef enum VkMemoryAllocateFlagBits {
    VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT = 0x00000001,
  // Provided by VK_VERSION_1_2
    VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT = 0x00000002,
  // Provided by VK_VERSION_1_2
    VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT = 0x00000004,
  // Provided by VK_KHR_device_group
    VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT_KHR = VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT,
  // Provided by VK_KHR_buffer_device_address
    VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT_KHR = VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT,
  // Provided by VK_KHR_buffer_device_address
    VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR = VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT,
} VkMemoryAllocateFlagBits;
```
or the equivalent
```c
// Provided by VK_KHR_device_group
typedef VkMemoryAllocateFlagBits VkMemoryAllocateFlagBitsKHR;
```

# Description
- [`VK_MEMORY_ALLOCATE_FLAG_BITS`] specifies that memory will be allocated for the devices in [`MemoryAllocateFlagsInfo::device_mask`].
- [`DEVICE_ADDRESS`] specifies that the memory  **can**  be attached to a buffer object created with the `VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT` bit set in `usage`, and that the memory handle  **can**  be used to retrieve an opaque address via [`get_device_memory_opaque_capture_address`].
- [`DEVICE_ADDRESS_CAPTURE_REPLAY`] specifies that the memory’s address  **can**  be saved and reused on a subsequent run (e.g. for trace capture and replay), see [`BufferOpaqueCaptureAddressCreateInfo`] for more detail.

# Related
- [`crate::vulkan1_1`]
- [VkMemoryAllocateFlags]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        