[vkCreateInstance](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateInstance.html) - Create a new Vulkan instance

# C Specifications
To create an instance object, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkCreateInstance(
    const VkInstanceCreateInfo*                 pCreateInfo,
    const VkAllocationCallbacks*                pAllocator,
    VkInstance*                                 pInstance);
```

# Parameters
- [`p_create_info`] is a pointer to a [`InstanceCreateInfo`] structure controlling creation of the instance.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.
- [`p_instance`] points a [`Instance`] handle in which the resulting instance is returned.

# Description
[`create_instance`] verifies that the requested layers exist.
If not, [`create_instance`] will return `VK_ERROR_LAYER_NOT_PRESENT`.
Next [`create_instance`] verifies that the requested extensions are
supported (e.g. in the implementation or in any enabled instance layer) and
if any requested extension is not supported, [`create_instance`] **must** 
return `VK_ERROR_EXTENSION_NOT_PRESENT`.
After verifying and enabling the instance layers and extensions the
[`Instance`] object is created and returned to the application.
If a requested extension is only supported by a layer, both the layer and
the extension need to be specified at [`create_instance`] time for the
creation to succeed.
## Valid Usage
-    All [required extensions](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#extendingvulkan-extensions-extensiondependencies) for each extension in the [`InstanceCreateInfo::pp_enabled_extension_names`] list  **must**  also be present in that list

## Valid Usage (Implicit)
-  [`p_create_info`] **must**  be a valid pointer to a valid [`InstanceCreateInfo`] structure
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-  [`p_instance`] **must**  be a valid pointer to a [`Instance`] handle

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_INITIALIZATION_FAILED`  - `VK_ERROR_LAYER_NOT_PRESENT`  - `VK_ERROR_EXTENSION_NOT_PRESENT`  - `VK_ERROR_INCOMPATIBLE_DRIVER`

# Related
- [`crate::vulkan1_0`]
- [`AllocationCallbacks`]
- [`Instance`]
- [`InstanceCreateInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        