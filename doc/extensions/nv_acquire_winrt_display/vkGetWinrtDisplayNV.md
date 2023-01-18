[vkGetWinrtDisplayNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetWinrtDisplayNV.html) - Query the VkDisplayKHR corresponding to a WinRT DisplayTarget

# C Specifications
When acquiring displays on Windows 10, an application may also wish to
enumerate and identify them using a native handle rather than a
[`DisplayKHR`] handle.To determine the [`DisplayKHR`] handle corresponding to a
[“winrt::Windows::Devices::Display::Core::DisplayTarget”](https://docs.microsoft.com/en-us/uwp/api/windows.devices.display.core.displaytarget),
call:
```c
// Provided by VK_NV_acquire_winrt_display
VkResult vkGetWinrtDisplayNV(
    VkPhysicalDevice                            physicalDevice,
    uint32_t                                    deviceRelativeId,
    VkDisplayKHR*                               pDisplay);
```

# Parameters
- [`physical_device`] The physical device on which to query the display handle.
- [`device_relative_id`] The value of the [“AdapterRelativeId”](https://docs.microsoft.com/en-us/uwp/api/windows.devices.display.core.displaytarget.adapterrelativeid) property of a [“DisplayTarget”](https://docs.microsoft.com/en-us/uwp/api/windows.devices.display.core.displaytarget) that is enumerated by a [“DisplayAdapter”](https://docs.microsoft.com/en-us/uwp/api/windows.devices.display.core.displayadapter) with an [“Id”](https://docs.microsoft.com/en-us/uwp/api/windows.devices.display.core.displayadapter.id) property matching the `deviceLUID` property of a [`PhysicalDeviceIdProperties`] for [`physical_device`].
- [`p_display`] The corresponding [`DisplayKHR`] handle will be returned here.

# Description
If there is no [`DisplayKHR`] corresponding to [`device_relative_id`] on
[`physical_device`], [`crate::Handle::null`] **must**  be returned in
[`p_display`].
## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`p_display`] **must**  be a valid pointer to a [`DisplayKHR`] handle

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_DEVICE_LOST`  - `VK_ERROR_INITIALIZATION_FAILED`

# Related
- [`VK_NV_acquire_winrt_display`]
- [`DisplayKHR`]
- [`PhysicalDevice`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        