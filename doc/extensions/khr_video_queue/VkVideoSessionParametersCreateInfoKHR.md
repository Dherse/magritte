[VkVideoSessionParametersCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoSessionParametersCreateInfoKHR.html) - Structure to set video session parameters

# C Specifications
The [`VideoSessionParametersCreateInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_video_queue
typedef struct VkVideoSessionParametersCreateInfoKHR {
    VkStructureType                sType;
    const void*                    pNext;
    VkVideoSessionParametersKHR    videoSessionParametersTemplate;
    VkVideoSessionKHR              videoSession;
} VkVideoSessionParametersCreateInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`video_session_parameters_template`] is [`crate::Handle::null`] or a valid handle to a [`VideoSessionParametersKHR`] object. If this parameter represents a valid handle, then the underlying Video Session Parameters object will be used as a template for constructing the new video session parameters object. All of the template object’s current parameters will be inherited by the new object in such a case. Optionally, some of the template’s parameters can be updated or new parameters added to the newly constructed object via the extension-specific parameters.
- [`video_session`] is the video session object against which the video session parameters object is going to be created.

# Description
## Valid Usage
-    If [`video_session_parameters_template`] represents a valid handle, it  **must**  have been created against [`video_session`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR`
-    Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**  be either `NULL` or a pointer to a valid instance of [`VideoDecodeH264SessionParametersCreateInfoEXT`], [`VideoDecodeH265SessionParametersCreateInfoEXT`], [`VideoEncodeH264SessionParametersCreateInfoEXT`], or [`VideoEncodeH265SessionParametersCreateInfoEXT`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-    If [`video_session_parameters_template`] is not [`crate::Handle::null`], [`video_session_parameters_template`] **must**  be a valid [`VideoSessionParametersKHR`] handle
-  [`video_session`] **must**  be a valid [`VideoSessionKHR`] handle
-    If [`video_session_parameters_template`] is a valid handle, it  **must**  have been created, allocated, or retrieved from [`video_session`]
-    Both of [`video_session`], and [`video_session_parameters_template`] that are valid handles of non-ignored parameters  **must**  have been created, allocated, or retrieved from the same [`Device`]

# Related
- [`VK_KHR_video_queue`]
- [`StructureType`]
- [`VideoSessionKHR`]
- [`VideoSessionParametersKHR`]
- [`create_video_session_parameters_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        