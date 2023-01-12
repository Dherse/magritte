[VkImagePipeSurfaceCreateInfoFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImagePipeSurfaceCreateInfoFUCHSIA.html) - Structure specifying parameters of a newly created ImagePipe surface object

# C Specifications
The [`ImagePipeSurfaceCreateInfoFUCHSIA`] structure is defined as:
```c
// Provided by VK_FUCHSIA_imagepipe_surface
typedef struct VkImagePipeSurfaceCreateInfoFUCHSIA {
    VkStructureType                         sType;
    const void*                             pNext;
    VkImagePipeSurfaceCreateFlagsFUCHSIA    flags;
    zx_handle_t                             imagePipeHandle;
} VkImagePipeSurfaceCreateInfoFUCHSIA;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is reserved for future use.
- [`image_pipe_handle`] is a `zx_handle_t` referring to the ImagePipe to associate with the surface.

# Description
## Valid Usage
-  [`image_pipe_handle`] **must**  be a valid `zx_handle_t`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA`
-  [`p_next`] **must**  be `NULL`
-  [`flags`] **must**  be `0`

# Related
- [`fuchsia_imagepipe_surface`]
- [`ImagePipeSurfaceCreateFlagsFUCHSIA`]
- [`StructureType`]
- [`create_image_pipe_surface_fuchsia`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        