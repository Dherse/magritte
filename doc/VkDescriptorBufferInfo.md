[VkDescriptorBufferInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorBufferInfo.html) - Structure specifying descriptor buffer information

# C Specifications
The [`DescriptorBufferInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkDescriptorBufferInfo {
    VkBuffer        buffer;
    VkDeviceSize    offset;
    VkDeviceSize    range;
} VkDescriptorBufferInfo;
```

# Members
- [`buffer`] is [`crate::Handle::null`] or the buffer resource.
- [`offset`] is the offset in bytes from the start of [`buffer`]. Access to buffer memory via this descriptor uses addressing that is relative to this starting offset.
- [`range`] is the size in bytes that is used for this descriptor update, or [`WHOLE_SIZE`] to use the range from [`offset`] to the end of the buffer.

# Description
For `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC` and
`VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC` descriptor types,
[`offset`] is the base offset from which the dynamic offset is applied and
[`range`] is the static size used for all dynamic offsets.
## Valid Usage
-  [`offset`] **must**  be less than the size of [`buffer`]
-    If [`range`] is not equal to [`WHOLE_SIZE`], [`range`] **must**  be greater than `0`
-    If [`range`] is not equal to [`WHOLE_SIZE`], [`range`] **must**  be less than or equal to the size of [`buffer`] minus [`offset`]
-    If the [nullDescriptor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-nullDescriptor) feature is not enabled, [`buffer`] **must**  not be [`crate::Handle::null`]
-    If [`buffer`] is [`crate::Handle::null`], [`offset`] **must**  be zero and [`range`] **must**  be [`WHOLE_SIZE`]

## Valid Usage (Implicit)
-    If [`buffer`] is not [`crate::Handle::null`], [`buffer`] **must**  be a valid [`Buffer`] handle

# Related
- [`crate::vulkan1_0`]
- [`Buffer`]
- [`DeviceSize`]
- [`WriteDescriptorSet`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        