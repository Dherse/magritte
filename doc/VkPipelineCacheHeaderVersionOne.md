[VkPipelineCacheHeaderVersionOne](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheHeaderVersionOne.html) - Structure describing the layout of the pipeline cache header

# C Specifications
Version one of the pipeline cache header is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkPipelineCacheHeaderVersionOne {
    uint32_t                        headerSize;
    VkPipelineCacheHeaderVersion    headerVersion;
    uint32_t                        vendorID;
    uint32_t                        deviceID;
    uint8_t                         pipelineCacheUUID[VK_UUID_SIZE];
} VkPipelineCacheHeaderVersionOne;
```

# Members
- [`header_size`] is the length in bytes of the pipeline cache header.
- [`header_version`] is a [`PipelineCacheHeaderVersion`] enum value specifying the version of the header. A consumer of the pipeline cache  **should**  use the cache version to interpret the remainder of the cache header.
- [`vendor_id`] is the [`PhysicalDeviceProperties`]::[`vendor_id`] of the implementation.
- [`device_id`] is the [`PhysicalDeviceProperties`]::[`device_id`] of the implementation.
- [`pipeline_cache_uuid`] is the [`PhysicalDeviceProperties`]::[`pipeline_cache_uuid`] of the implementation.

# Description
Unlike most structures declared by the Vulkan API, all fields of this
structure are written with the least significant byte first, regardless of
host byte-order.The C language specification does not define the packing of structure
members.
This layout assumes tight structure member packing, with members laid out in
the order listed in the structure, and the intended size of the structure is
32 bytes.
If a compiler produces code that diverges from that pattern, applications
 **must**  employ another method to set values at the correct offsets.
## Valid Usage
-  [`header_size`] **must**  be 32
-  [`header_version`] **must**  be `VK_PIPELINE_CACHE_HEADER_VERSION_ONE`

## Valid Usage (Implicit)
-  [`header_version`] **must**  be a valid [`PipelineCacheHeaderVersion`] value

# Related
- [`crate::vulkan1_0`]
- [`PipelineCacheHeaderVersion`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        