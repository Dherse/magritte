[vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI.html) - Query maximum supported subpass shading workgroup size for a give render pass

# C Specifications
A subpass shading pipeline’s workgroup size is a 2D vector with number of
power-of-two in width and height.
The maximum number of width and height is implementation dependent, and  **may** 
vary for different formats and sample counts of attachments in a render
pass.To query the maximum workgroup size, call:
```c
// Provided by VK_HUAWEI_subpass_shading
VkResult vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI(
    VkDevice                                    device,
    VkRenderPass                                renderpass,
    VkExtent2D*                                 pMaxWorkgroupSize);
```

# Parameters
- [`device`] is a handle to a local device object that was used to create the given render pass.
- `renderPass` is a handle to a render pass object describing the environment in which the pipeline will be used. The pipeline  **must**  only be used with a render pass instance compatible with the one provided. See [Render Pass Compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-compatibility) for more information.
- [`p_max_workgroup_size`] is a pointer to a [`Extent2D`] structure.

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`renderpass`] **must**  be a valid [`RenderPass`] handle
-  [`p_max_workgroup_size`] **must**  be a valid pointer to a [`Extent2D`] structure
-  [`renderpass`] **must**  have been created, allocated, or retrieved from [`device`]

## Return Codes
*   - `VK_SUCCESS`  - `VK_INCOMPLETE` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_SURFACE_LOST_KHR`

# Related
- [`huawei_subpass_shading`]
- [`Device`]
- [`Extent2D`]
- [`RenderPass`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        