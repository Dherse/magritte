[VkImageConstraintsInfoFlagBitsFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageConstraintsInfoFlagBitsFUCHSIA.html) - Bitmask specifying image constraints flags

# C Specifications
Bits which  **can**  be set in
[`ImageConstraintsInfoFlagBitsFUCHSIA`]`::flags` include:
```c
// Provided by VK_FUCHSIA_buffer_collection
typedef enum VkImageConstraintsInfoFlagBitsFUCHSIA {
    VK_IMAGE_CONSTRAINTS_INFO_CPU_READ_RARELY_FUCHSIA = 0x00000001,
    VK_IMAGE_CONSTRAINTS_INFO_CPU_READ_OFTEN_FUCHSIA = 0x00000002,
    VK_IMAGE_CONSTRAINTS_INFO_CPU_WRITE_RARELY_FUCHSIA = 0x00000004,
    VK_IMAGE_CONSTRAINTS_INFO_CPU_WRITE_OFTEN_FUCHSIA = 0x00000008,
    VK_IMAGE_CONSTRAINTS_INFO_PROTECTED_OPTIONAL_FUCHSIA = 0x00000010,
} VkImageConstraintsInfoFlagBitsFUCHSIA;
```

# Description
General hints about the type of memory that should be allocated by Sysmem
based on the expected usage of the images in the buffer collection include:
- [`CPU_READ_RARELY`]
- [`CPU_READ_OFTEN`]
- [`CPU_WRITE_RARELY`]
- [`CPU_WRITE_OFTEN`]
For protected memory:
- [`PROTECTED_OPTIONAL`] specifies that protected memory is optional for the buffer collection.
Note that if all participants in the buffer collection (Vulkan or otherwise)
specify that protected memory is optional, Sysmem will not allocate
protected memory.

# Related
- [`VK_FUCHSIA_buffer_collection`]
- [`ImageConstraintsInfoFlagsFUCHSIA`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        