[VkSharingMode](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSharingMode.html) - Buffer and image sharing modes

# C Specifications
Buffer and image objects are created with a *sharing mode* controlling how
they  **can**  be accessed from queues.
The supported sharing modes are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkSharingMode {
    VK_SHARING_MODE_EXCLUSIVE = 0,
    VK_SHARING_MODE_CONCURRENT = 1,
} VkSharingMode;
```

# Description
- [`EXCLUSIVE`] specifies that access to any range or image subresource of the object will be exclusive to a single queue family at a time.
- [`CONCURRENT`] specifies that concurrent access to any range or image subresource of the object from multiple queue families is supported.
Ranges of buffers and image subresources of image objects created using
[`EXCLUSIVE`] **must**  only be accessed by queues in the
queue family that has *ownership* of the resource.
Upon creation, such resources are not owned by any queue family; ownership
is implicitly acquired upon first use within a queue.
Once a resource using [`EXCLUSIVE`] is owned by some queue
family, the application  **must**  perform a
[queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers) to make
the memory contents of a range or image subresource accessible to a
different queue family.A queue family  **can**  take ownership of an image subresource or buffer range
of a resource created with [`EXCLUSIVE`], without an
ownership transfer, in the same way as for a resource that was just created;
however, taking ownership in this way has the effect that the contents of
the image subresource or buffer range are undefined.Ranges of buffers and image subresources of image objects created using
[`CONCURRENT`] **must**  only be accessed by queues from the
queue families specified through the `queueFamilyIndexCount` and
`pQueueFamilyIndices` members of the corresponding create info
structures.

# Related
- [`crate::vulkan1_0`]
- [`BufferCreateInfo`]
- [`ImageCreateInfo`]
- [`PhysicalDeviceImageDrmFormatModifierInfoEXT`]
- [`SwapchainCreateInfoKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        