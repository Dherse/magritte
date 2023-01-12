[VkSwapchainDisplayNativeHdrCreateInfoAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSwapchainDisplayNativeHdrCreateInfoAMD.html) - Structure specifying display native HDR parameters of a newly created swapchain object

# C Specifications
If the [`p_next`] chain of [`SwapchainCreateInfoKHR`] includes a
[`SwapchainDisplayNativeHdrCreateInfoAMD`] structure, then that
structure includes additional swapchain creation parameters specific to
display native HDR support.The [`SwapchainDisplayNativeHdrCreateInfoAMD`] structure is defined as:
```c
// Provided by VK_AMD_display_native_hdr
typedef struct VkSwapchainDisplayNativeHdrCreateInfoAMD {
    VkStructureType    sType;
    const void*        pNext;
    VkBool32           localDimmingEnable;
} VkSwapchainDisplayNativeHdrCreateInfoAMD;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`local_dimming_enable`] specifies whether local dimming is enabled for the swapchain.

# Description
If the [`p_next`] chain of [`SwapchainCreateInfoKHR`] does not include
this structure, the default value for [`local_dimming_enable`] is
`VK_TRUE`, meaning local dimming is initially enabled for the swapchain.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD`

## Valid Usage
-    It is only valid to set [`local_dimming_enable`] to `VK_TRUE` if [`DisplayNativeHdrSurfaceCapabilitiesAMD::local_dimming_support`] is supported

# Related
- [`amd_display_native_hdr`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        