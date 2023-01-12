[VkResult](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkResult.html) - Vulkan command return codes

# C Specifications
While the core Vulkan API is not designed to capture incorrect usage, some
circumstances still require return codes.
Commands in Vulkan return their status via return codes that are in one of
two categories:
- Successful completion codes are returned when a command needs to communicate success or status information. All successful completion codes are non-negative values.
- Run time error codes are returned when a command needs to communicate a failure that could only be detected at runtime. All runtime error codes are negative values.
All return codes in Vulkan are reported via [`VulkanResultCodes`] return values.
The possible codes are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkResult {
    VK_SUCCESS = 0,
    VK_NOT_READY = 1,
    VK_TIMEOUT = 2,
    VK_EVENT_SET = 3,
    VK_EVENT_RESET = 4,
    VK_INCOMPLETE = 5,
    VK_ERROR_OUT_OF_HOST_MEMORY = -1,
    VK_ERROR_OUT_OF_DEVICE_MEMORY = -2,
    VK_ERROR_INITIALIZATION_FAILED = -3,
    VK_ERROR_DEVICE_LOST = -4,
    VK_ERROR_MEMORY_MAP_FAILED = -5,
    VK_ERROR_LAYER_NOT_PRESENT = -6,
    VK_ERROR_EXTENSION_NOT_PRESENT = -7,
    VK_ERROR_FEATURE_NOT_PRESENT = -8,
    VK_ERROR_INCOMPATIBLE_DRIVER = -9,
    VK_ERROR_TOO_MANY_OBJECTS = -10,
    VK_ERROR_FORMAT_NOT_SUPPORTED = -11,
    VK_ERROR_FRAGMENTED_POOL = -12,
    VK_ERROR_UNKNOWN = -13,
  // Provided by VK_VERSION_1_1
    VK_ERROR_OUT_OF_POOL_MEMORY = -1000069000,
  // Provided by VK_VERSION_1_1
    VK_ERROR_INVALID_EXTERNAL_HANDLE = -1000072003,
  // Provided by VK_VERSION_1_2
    VK_ERROR_FRAGMENTATION = -1000161000,
  // Provided by VK_VERSION_1_2
    VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS = -1000257000,
  // Provided by VK_VERSION_1_3
    VK_PIPELINE_COMPILE_REQUIRED = 1000297000,
  // Provided by VK_KHR_surface
    VK_ERROR_SURFACE_LOST_KHR = -1000000000,
  // Provided by VK_KHR_surface
    VK_ERROR_NATIVE_WINDOW_IN_USE_KHR = -1000000001,
  // Provided by VK_KHR_swapchain
    VK_SUBOPTIMAL_KHR = 1000001003,
  // Provided by VK_KHR_swapchain
    VK_ERROR_OUT_OF_DATE_KHR = -1000001004,
  // Provided by VK_KHR_display_swapchain
    VK_ERROR_INCOMPATIBLE_DISPLAY_KHR = -1000003001,
  // Provided by VK_EXT_debug_report
    VK_ERROR_VALIDATION_FAILED_EXT = -1000011001,
  // Provided by VK_NV_glsl_shader
    VK_ERROR_INVALID_SHADER_NV = -1000012000,
  // Provided by VK_EXT_image_drm_format_modifier
    VK_ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT = -1000158000,
  // Provided by VK_KHR_global_priority
    VK_ERROR_NOT_PERMITTED_KHR = -1000174001,
  // Provided by VK_EXT_full_screen_exclusive
    VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT = -1000255000,
  // Provided by VK_KHR_deferred_host_operations
    VK_THREAD_IDLE_KHR = 1000268000,
  // Provided by VK_KHR_deferred_host_operations
    VK_THREAD_DONE_KHR = 1000268001,
  // Provided by VK_KHR_deferred_host_operations
    VK_OPERATION_DEFERRED_KHR = 1000268002,
  // Provided by VK_KHR_deferred_host_operations
    VK_OPERATION_NOT_DEFERRED_KHR = 1000268003,
  // Provided by VK_KHR_maintenance1
    VK_ERROR_OUT_OF_POOL_MEMORY_KHR = VK_ERROR_OUT_OF_POOL_MEMORY,
  // Provided by VK_KHR_external_memory
    VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR = VK_ERROR_INVALID_EXTERNAL_HANDLE,
  // Provided by VK_EXT_descriptor_indexing
    VK_ERROR_FRAGMENTATION_EXT = VK_ERROR_FRAGMENTATION,
  // Provided by VK_EXT_global_priority
    VK_ERROR_NOT_PERMITTED_EXT = VK_ERROR_NOT_PERMITTED_KHR,
  // Provided by VK_EXT_buffer_device_address
    VK_ERROR_INVALID_DEVICE_ADDRESS_EXT = VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS,
  // Provided by VK_KHR_buffer_device_address
    VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR = VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS,
  // Provided by VK_EXT_pipeline_creation_cache_control
    VK_PIPELINE_COMPILE_REQUIRED_EXT = VK_PIPELINE_COMPILE_REQUIRED,
  // Provided by VK_EXT_pipeline_creation_cache_control
    VK_ERROR_PIPELINE_COMPILE_REQUIRED_EXT = VK_PIPELINE_COMPILE_REQUIRED,
} VkResult;
```

# Description
- [`VK_RESULT`] Command successfully completed
- [`VK_RESULT`] A fence or query has not yet completed
- [`VK_RESULT`] A wait operation has not completed in the specified time
- [`VK_RESULT`] An event is signaled
- [`VK_RESULT`] An event is unsignaled
- [`VK_RESULT`] A return array was too small for the result
- [`SUBOPTIMAL_KHR`] A swapchain no longer matches the surface properties exactly, but  **can**  still be used to present to the surface successfully.
- [`THREAD_IDLE_KHR`] A deferred operation is not complete but there is currently no work for this thread to do at the time of this call.
- [`THREAD_DONE_KHR`] A deferred operation is not complete but there is no work remaining to assign to additional threads.
- [`OPERATION_DEFERRED_KHR`] A deferred operation was requested and at least some of the work was deferred.
- [`OPERATION_NOT_DEFERRED_KHR`] A deferred operation was requested and no operations were deferred.
- [`PIPELINE_COMPILE_REQUIRED`] A requested pipeline creation would have required compilation, but the application requested compilation to not be performed.

- [`VK_RESULT`] A host memory allocation has failed.
- [`VK_RESULT`] A device memory allocation has failed.
- [`VK_RESULT`] Initialization of an object could not be completed for implementation-specific reasons.
- [`VK_RESULT`] The logical or physical device has been lost. See [Lost Device](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-lost-device)
- [`VK_RESULT`] Mapping of a memory object has failed.
- [`VK_RESULT`] A requested layer is not present or could not be loaded.
- [`VK_RESULT`] A requested extension is not supported.
- [`VK_RESULT`] A requested feature is not supported.
- [`VK_RESULT`] The requested version of Vulkan is not supported by the driver or is otherwise incompatible for implementation-specific reasons.
- [`VK_RESULT`] Too many objects of the type have already been created.
- [`VK_RESULT`] A requested format is not supported on this device.
- [`VK_RESULT`] A pool allocation has failed due to fragmentation of the pool’s memory. This  **must**  only be returned if no attempt to allocate host or device memory was made to accommodate the new allocation. This  **should**  be returned in preference to [`ERROR_OUT_OF_POOL_MEMORY`], but only if the implementation is certain that the pool allocation failure was due to fragmentation.
- [`ERROR_SURFACE_LOST_KHR`] A surface is no longer available.
- [`ERROR_NATIVE_WINDOW_IN_USE_KHR`] The requested window is already in use by Vulkan or another API in a manner which prevents it from being used again.
- [`ERROR_OUT_OF_DATE_KHR`] A surface has changed in such a way that it is no longer compatible with the swapchain, and further presentation requests using the swapchain will fail. Applications  **must**  query the new surface properties and recreate their swapchain if they wish to continue presenting to the surface.
- [`ERROR_INCOMPATIBLE_DISPLAY_KHR`] The display used by a swapchain does not use the same presentable image layout, or is incompatible in a way that prevents sharing an image.
- [`ERROR_INVALID_SHADER_NV`] One or more shaders failed to compile or link. More details are reported back to the application via `[`ext_debug_report`]` if enabled.
- [`ERROR_OUT_OF_POOL_MEMORY`] A pool memory allocation has failed. This  **must**  only be returned if no attempt to allocate host or device memory was made to accommodate the new allocation. If the failure was definitely due to fragmentation of the pool, [`VK_RESULT`] **should**  be returned instead.
- [`ERROR_INVALID_EXTERNAL_HANDLE`] An external handle is not a valid handle of the specified type.
- [`ERROR_FRAGMENTATION`] A descriptor pool creation has failed due to fragmentation.
- [`ERROR_INVALID_DEVICE_ADDRESS_EXT`] A buffer creation failed because the requested address is not available.
- [`ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS`] A buffer creation or memory allocation failed because the requested address is not available. A shader group handle assignment failed because the requested shader group handle information is no longer valid.
- [`ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`] An operation on a swapchain created with `VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT` failed as it did not have exlusive full-screen access. This  **may**  occur due to implementation-dependent reasons, outside of the application’s control.
- [`VK_RESULT`] An unknown error has occurred; either the application has provided invalid input, or an implementation failure has occurred.
If a command returns a runtime error, unless otherwise specified any output
parameters will have undefined contents, except that if the output
parameter is a structure with `sType` and `pNext` fields, those
fields will be unmodified.
Any structures chained from `pNext` will also have undefined contents,
except that `sType` and `pNext` will be unmodified.`VK_ERROR_OUT_OF_*_MEMORY` errors do not modify any currently existing
Vulkan objects.
Objects that have already been successfully created  **can**  still be used by
the application.[`VK_RESULT`] will be returned by an implementation when an
unexpected error occurs that cannot be attributed to valid behavior of the
application and implementation.
Under these conditions, it  **may**  be returned from any command returning a
[`VulkanResultCodes`].Performance-critical commands generally do not have return codes.
If a runtime error occurs in such commands, the implementation will defer
reporting the error until a specified point.
For commands that record into command buffers (`vkCmd*`) runtime errors
are reported by [`end_command_buffer`].

# Related
- [`crate::vulkan1_0`]
- [`PresentInfoKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        