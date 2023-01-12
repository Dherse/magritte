[VkDeviceEventTypeEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceEventTypeEXT.html) - Events that can occur on a device object

# C Specifications
Possible values of [`DeviceEventInfoEXT`]`::device`, specifying when
a fence will be signaled, are:
```c
// Provided by VK_EXT_display_control
typedef enum VkDeviceEventTypeEXT {
    VK_DEVICE_EVENT_TYPE_DISPLAY_HOTPLUG_EXT = 0,
} VkDeviceEventTypeEXT;
```

# Description
- [`VK_DEVICE_EVENT_TYPE_EXT`] specifies that the fence is signaled when a display is plugged into or unplugged from the specified device. Applications  **can**  use this notification to determine when they need to re-enumerate the available displays on a device.

# Related
- [`ext_display_control`]
- [`DeviceEventInfoEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        