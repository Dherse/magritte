[VkDisplayModePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayModePropertiesKHR.html) - Structure describing display mode properties

# C Specifications
The [`DisplayModePropertiesKHR`] structure is defined as:
```c
// Provided by VK_KHR_display
typedef struct VkDisplayModePropertiesKHR {
    VkDisplayModeKHR              displayMode;
    VkDisplayModeParametersKHR    parameters;
} VkDisplayModePropertiesKHR;
```

# Members
- [`display_mode`] is a handle to the display mode described in this structure. This handle will be valid for the lifetime of the Vulkan instance.
- [`parameters`] is a [`DisplayModeParametersKHR`] structure describing the display parameters associated with [`display_mode`].

# Related
- [`khr_display`]
- [`DisplayModeKHR`]
- [`DisplayModeParametersKHR`]
- [`DisplayModeProperties2KHR`]
- [`get_display_mode_properties_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        