[VkFullScreenExclusiveEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFullScreenExclusiveEXT.html) - Hint values an application can specify affecting full-screen transition behavior

# C Specifications
Possible values of
[`SurfaceFullScreenExclusiveInfoEXT::full_screen_exclusive`] are:
```c
// Provided by VK_EXT_full_screen_exclusive
typedef enum VkFullScreenExclusiveEXT {
    VK_FULL_SCREEN_EXCLUSIVE_DEFAULT_EXT = 0,
    VK_FULL_SCREEN_EXCLUSIVE_ALLOWED_EXT = 1,
    VK_FULL_SCREEN_EXCLUSIVE_DISALLOWED_EXT = 2,
    VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT = 3,
} VkFullScreenExclusiveEXT;
```

# Description
- [`VK_FULL_SCREEN_EXCLUSIVE_EXT`] indicates the implementation  **should**  determine the appropriate full-screen method by whatever means it deems appropriate.
- [`VK_FULL_SCREEN_EXCLUSIVE_EXT`] indicates the implementation  **may**  use full-screen exclusive mechanisms when available. Such mechanisms  **may**  result in better performance and/or the availability of different presentation capabilities, but  **may**  require a more disruptive transition during swapchain initialization, first presentation and/or destruction.
- [`VK_FULL_SCREEN_EXCLUSIVE_EXT`] indicates the implementation  **should**  avoid using full-screen mechanisms which rely on disruptive transitions.
- [`VK_FULL_SCREEN_EXCLUSIVE_EXT`] indicates the application will manage full-screen exclusive mode by using the [`acquire_full_screen_exclusive_mode_ext`] and [`release_full_screen_exclusive_mode_ext`] commands.

# Related
- [`ext_full_screen_exclusive`]
- [`SurfaceFullScreenExclusiveInfoEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        