[VkStridedDeviceAddressRegionKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkStridedDeviceAddressRegionKHR.html) - Structure specifying a region of device addresses with a stride

# C Specifications
The [`StridedDeviceAddressRegionKHR`] structure is defined as:
```c
// Provided by VK_KHR_ray_tracing_pipeline
typedef struct VkStridedDeviceAddressRegionKHR {
    VkDeviceAddress    deviceAddress;
    VkDeviceSize       stride;
    VkDeviceSize       size;
} VkStridedDeviceAddressRegionKHR;
```

# Members
- [`device_address`] is the device address (as returned by the [`get_buffer_device_address`] command) at which the region starts, or zero if the region is unused.
- [`stride`] is the byte stride between consecutive elements.
- [`size`] is the size in bytes of the region starting at [`device_address`].

# Description
## Valid Usage
-    If [`size`] is not zero, all addresses between [`device_address`] and [`device_address`] +  [`size`] - 1 **must**  be in the buffer device address range of the same buffer
-    If [`size`] is not zero, [`stride`] **must**  be less than or equal to the size of the buffer from which [`device_address`] was queried

# Related
- [`khr_ray_tracing_pipeline`]
- [`DeviceAddress`]
- [`DeviceSize`]
- [`cmd_trace_rays_indirect_khr`]
- [`cmd_trace_rays_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        