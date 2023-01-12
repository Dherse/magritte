[VkQueueFamilyProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyProperties.html) - Structure providing information about a queue family

# C Specifications
The [`QueueFamilyProperties`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkQueueFamilyProperties {
    VkQueueFlags    queueFlags;
    uint32_t        queueCount;
    uint32_t        timestampValidBits;
    VkExtent3D      minImageTransferGranularity;
} VkQueueFamilyProperties;
```

# Members
- [`queue_flags`] is a bitmask of [`QueueFlagBits`] indicating capabilities of the queues in this queue family.
- [`queue_count`] is the unsigned integer count of queues in this queue family. Each queue family  **must**  support at least one queue.
- [`timestamp_valid_bits`] is the unsigned integer count of meaningful bits in the timestamps written via [`cmd_write_timestamp2`] or [`cmd_write_timestamp`]. The valid range for the count is 36..64 bits, or a value of 0, indicating no support for timestamps. Bits outside the valid range are guaranteed to be zeros.
- [`min_image_transfer_granularity`] is the minimum granularity supported for image transfer operations on the queues in this queue family.

# Description
The value returned in [`min_image_transfer_granularity`] has a unit of
compressed texel blocks for images having a block-compressed format, and a
unit of texels otherwise.Possible values of [`min_image_transfer_granularity`] are:
- (0,0,0) specifies that only whole mip levels  **must**  be transferred using the image transfer operations on the corresponding queues. In this case, the following restrictions apply to all offset and extent parameters of image transfer operations:  - The `x`, `y`, and `z` members of a [`Offset3D`] parameter  **must**  always be zero.  - The `width`, `height`, and `depth` members of a [`Extent3D`] parameter  **must**  always match the width, height, and depth of the image subresource corresponding to the parameter, respectively. 
- (A<sub>x</sub>, A<sub>y</sub>, A<sub>z</sub>) where A<sub>x</sub>, A<sub>y</sub>, and A<sub>z</sub> are all integer powers of two. In this case the following restrictions apply to all image transfer operations:  - `x`, `y`, and `z` of a [`Offset3D`] parameter  **must**  be integer multiples of A<sub>x</sub>, A<sub>y</sub>, and A<sub>z</sub>, respectively.  - `width` of a [`Extent3D`] parameter  **must**  be an integer multiple of A<sub>x</sub>, or else `x` +  `width` **must**  equal the width of the image subresource corresponding to the parameter.  - `height` of a [`Extent3D`] parameter  **must**  be an integer multiple of A<sub>y</sub>, or else `y` +  `height` **must**  equal the height of the image subresource corresponding to the parameter.  - `depth` of a [`Extent3D`] parameter  **must**  be an integer multiple of A<sub>z</sub>, or else `z` +  `depth` **must**  equal the depth of the image subresource corresponding to the parameter.  - If the format of the image corresponding to the parameters is one of the block-compressed formats then for the purposes of the above calculations the granularity  **must**  be scaled up by the compressed texel block dimensions. 
Queues supporting graphics and/or compute operations  **must**  report
(1,1,1) in [`min_image_transfer_granularity`], meaning that there are
no additional restrictions on the granularity of image transfer operations
for these queues.
Other queues supporting image transfer operations are only  **required**  to
support whole mip level transfers, thus [`min_image_transfer_granularity`]
for queues belonging to such queue families  **may**  be (0,0,0).The [Device Memory](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-device) section describes memory properties
queried from the physical device.For physical device feature queries see the [Features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features) chapter.

# Related
- [`crate::vulkan1_0`]
- [`Extent3D`]
- [`QueueFamilyProperties2`]
- [VkQueueFlags]()
- [`get_physical_device_queue_family_properties`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        