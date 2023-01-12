[vkGetValidationCacheDataEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetValidationCacheDataEXT.html) - Get the data store from a validation cache

# C Specifications
Data  **can**  be retrieved from a validation cache object using the command:
```c
// Provided by VK_EXT_validation_cache
VkResult vkGetValidationCacheDataEXT(
    VkDevice                                    device,
    VkValidationCacheEXT                        validationCache,
    size_t*                                     pDataSize,
    void*                                       pData);
```

# Parameters
- [`device`] is the logical device that owns the validation cache.
- [`validation_cache`] is the validation cache to retrieve data from.
- [`p_data_size`] is a pointer to a value related to the amount of data in the validation cache, as described below.
- [`p_data`] is either `NULL` or a pointer to a buffer.

# Description
If [`p_data`] is `NULL`, then the maximum size of the data that  **can**  be
retrieved from the validation cache, in bytes, is returned in
[`p_data_size`].
Otherwise, [`p_data_size`] **must**  point to a variable set by the user to the
size of the buffer, in bytes, pointed to by [`p_data`], and on return the
variable is overwritten with the amount of data actually written to
[`p_data`].
If [`p_data_size`] is less than the maximum size that  **can**  be retrieved by
the validation cache, at most [`p_data_size`] bytes will be written to
[`p_data`], and [`get_validation_cache_data_ext`] will return
`VK_INCOMPLETE` instead of `VK_SUCCESS`, to indicate that not all of
the validation cache was returned.Any data written to [`p_data`] is valid and  **can**  be provided as the
`pInitialData` member of the [`ValidationCacheCreateInfoEXT`]
structure passed to [`create_validation_cache_ext`].Two calls to [`get_validation_cache_data_ext`] with the same parameters
 **must**  retrieve the same data unless a command that modifies the contents of
the cache is called between them.Applications  **can**  store the data retrieved from the validation cache, and
use these data, possibly in a future run of the application, to populate new
validation cache objects.
The results of validation, however,  **may**  depend on the vendor ID, device ID,
driver version, and other details of the device.
To enable applications to detect when previously retrieved data is
incompatible with the device, the initial bytes written to [`p_data`] **must** 
be a header consisting of the following members:The first four bytes encode the length of the entire validation cache
header, in bytes.
This value includes all fields in the header including the validation cache
version field and the size of the length field.The next four bytes encode the validation cache version, as described for
[`ValidationCacheHeaderVersionEXT`].
A consumer of the validation cache  **should**  use the cache version to
interpret the remainder of the cache header.If [`p_data_size`] is less than what is necessary to store this header,
nothing will be written to [`p_data`] and zero will be written to
[`p_data_size`].
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`validation_cache`] **must**  be a valid [`ValidationCacheEXT`] handle
-  [`p_data_size`] **must**  be a valid pointer to a `size_t` value
-    If the value referenced by [`p_data_size`] is not `0`, and [`p_data`] is not `NULL`, [`p_data`] **must**  be a valid pointer to an array of [`p_data_size`] bytes
-  [`validation_cache`] **must**  have been created, allocated, or retrieved from [`device`]

## Return Codes
*   - `VK_SUCCESS`  - `VK_INCOMPLETE` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`ext_validation_cache`]
- [`Device`]
- [`ValidationCacheEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        