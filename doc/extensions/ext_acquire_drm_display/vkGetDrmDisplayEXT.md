[vkGetDrmDisplayEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDrmDisplayEXT.html) - Query the VkDisplayKHR corresponding to a DRM connector ID

# C Specifications
Before acquiring a display from the DRM interface, the caller may want to
select a specific [`DisplayKHR`] handle by identifying it using a
[`connector_id`].
To do so, call:
```c
// Provided by VK_EXT_acquire_drm_display
VkResult vkGetDrmDisplayEXT(
    VkPhysicalDevice                            physicalDevice,
    int32_t                                     drmFd,
    uint32_t                                    connectorId,
    VkDisplayKHR*                               display);
```

# Parameters
- [`physical_device`] The physical device to query the display from.
- [`drm_fd`] DRM primary file descriptor.
- [`connector_id`] Identifier of the specified DRM connector.
- [`display`] The corresponding [`DisplayKHR`] handle will be returned here.

# Description
If there is no [`DisplayKHR`] corresponding to the [`connector_id`] on
the [`physical_device`], the returning [`display`] must be set to
[`crate::Handle::null`].
The provided [`drm_fd`] must correspond to the one owned by the
[`physical_device`].
If not, the error code `VK_ERROR_UNKNOWN` must be returned.
Master permissions are not required, because the file descriptor is just
used for information gathering purposes.
The given [`connector_id`] must be a resource owned by the provided
[`drm_fd`].
If not, the error code `VK_ERROR_UNKNOWN` must be returned.
If any error is encountered during the identification of the display, the
call must return the error code `VK_ERROR_INITIALIZATION_FAILED`.
## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`display`] **must**  be a valid pointer to a [`DisplayKHR`] handle

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_INITIALIZATION_FAILED`  - `VK_ERROR_OUT_OF_HOST_MEMORY`

# Related
- [`ext_acquire_drm_display`]
- [`DisplayKHR`]
- [`PhysicalDevice`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        