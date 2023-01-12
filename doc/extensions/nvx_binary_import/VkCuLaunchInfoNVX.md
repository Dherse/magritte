[VkCuLaunchInfoNVX](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCuLaunchInfoNVX.html) - Stub description of VkCuLaunchInfoNVX

# C Specifications
There is currently no specification language written for this type.
This section acts only as placeholder and to avoid dead links in the
specification and reference pages.
```c
// Provided by VK_NVX_binary_import
typedef struct VkCuLaunchInfoNVX {
    VkStructureType        sType;
    const void*            pNext;
    VkCuFunctionNVX        function;
    uint32_t               gridDimX;
    uint32_t               gridDimY;
    uint32_t               gridDimZ;
    uint32_t               blockDimX;
    uint32_t               blockDimY;
    uint32_t               blockDimZ;
    uint32_t               sharedMemBytes;
    size_t                 paramCount;
    const void* const *    pParams;
    size_t                 extraCount;
    const void* const *    pExtras;
} VkCuLaunchInfoNVX;
```

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_CU_LAUNCH_INFO_NVX`
-  [`p_next`] **must**  be `NULL`
-  [`function`] **must**  be a valid [`CuFunctionNVX`] handle
-    If [`param_count`] is not `0`, [`params`] **must**  be a valid pointer to an array of [`param_count`] bytes
-    If [`extra_count`] is not `0`, [`extras`] **must**  be a valid pointer to an array of [`extra_count`] bytes

# Related
- [`nvx_binary_import`]
- [`CuFunctionNVX`]
- [`StructureType`]
- [`cmd_cu_launch_kernel_nvx`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        