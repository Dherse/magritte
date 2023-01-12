[VkBufferCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCreateFlagBits.html) - Bitmask specifying additional parameters of a buffer

# C Specifications
Bits which  **can**  be set in [`BufferCreateInfo::flags`], specifying
additional parameters of a buffer, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkBufferCreateFlagBits {
    VK_BUFFER_CREATE_SPARSE_BINDING_BIT = 0x00000001,
    VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT = 0x00000002,
    VK_BUFFER_CREATE_SPARSE_ALIASED_BIT = 0x00000004,
  // Provided by VK_VERSION_1_1
    VK_BUFFER_CREATE_PROTECTED_BIT = 0x00000008,
  // Provided by VK_VERSION_1_2
    VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT = 0x00000010,
  // Provided by VK_EXT_buffer_device_address
    VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_EXT = VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT,
  // Provided by VK_KHR_buffer_device_address
    VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR = VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT,
} VkBufferCreateFlagBits;
```

# Description
- [`VK_BUFFER_CREATE_FLAG_BITS`] specifies that the buffer will be backed using sparse memory binding.
- [`VK_BUFFER_CREATE_FLAG_BITS`] specifies that the buffer  **can**  be partially backed using sparse memory binding. Buffers created with this flag  **must**  also be created with the [`VK_BUFFER_CREATE_FLAG_BITS`] flag.
- [`VK_BUFFER_CREATE_FLAG_BITS`] specifies that the buffer will be backed using sparse memory binding with memory ranges that might also simultaneously be backing another buffer (or another portion of the same buffer). Buffers created with this flag  **must**  also be created with the [`VK_BUFFER_CREATE_FLAG_BITS`] flag.
- [`PROTECTED`] specifies that the buffer is a protected buffer.
- [`DEVICE_ADDRESS_CAPTURE_REPLAY`] specifies that the buffer’s address  **can**  be saved and reused on a subsequent run (e.g. for trace capture and replay), see [`BufferOpaqueCaptureAddressCreateInfo`] for more detail.
See [Sparse Resource Features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#sparsememory-sparseresourcefeatures) and
[Physical Device Features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features) for details of the sparse memory
features supported on a device.

# Related
- [`crate::vulkan1_0`]
- [VkBufferCreateFlags]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        