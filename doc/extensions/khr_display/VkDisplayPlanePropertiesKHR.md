[VkDisplayPlanePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPlanePropertiesKHR.html) - Structure describing display plane properties

# C Specifications
The [`DisplayPlanePropertiesKHR`] structure is defined as:
```c
// Provided by VK_KHR_display
typedef struct VkDisplayPlanePropertiesKHR {
    VkDisplayKHR    currentDisplay;
    uint32_t        currentStackIndex;
} VkDisplayPlanePropertiesKHR;
```

# Members
- [`current_display`] is the handle of the display the plane is currently associated with. If the plane is not currently attached to any displays, this will be [`crate::Handle::null`].
- [`current_stack_index`] is the current z-order of the plane. This will be between 0 and the value returned by [`get_physical_device_display_plane_properties_khr`] in `pPropertyCount`.

# Related
- [`khr_display`]
- [`DisplayKHR`]
- [`DisplayPlaneProperties2KHR`]
- [`get_physical_device_display_plane_properties_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        