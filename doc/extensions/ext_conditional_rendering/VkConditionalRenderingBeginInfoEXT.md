[VkConditionalRenderingBeginInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkConditionalRenderingBeginInfoEXT.html) - Structure specifying conditional rendering begin information

# C Specifications
The [`ConditionalRenderingBeginInfoEXT`] structure is defined as:
```c
// Provided by VK_EXT_conditional_rendering
typedef struct VkConditionalRenderingBeginInfoEXT {
    VkStructureType                   sType;
    const void*                       pNext;
    VkBuffer                          buffer;
    VkDeviceSize                      offset;
    VkConditionalRenderingFlagsEXT    flags;
} VkConditionalRenderingBeginInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`buffer`] is a buffer containing the predicate for conditional rendering.
- [`offset`] is the byte offset into [`buffer`] where the predicate is located.
- [`flags`] is a bitmask of [`ConditionalRenderingFlagsEXT`] specifying the behavior of conditional rendering.

# Description
If the 32-bit value at [`offset`] in [`buffer`] memory is zero, then the
rendering commands are discarded, otherwise they are executed as normal.
If the value of the predicate in buffer memory changes while conditional
rendering is active, the rendering commands  **may**  be discarded in an
implementation-dependent way.
Some implementations may latch the value of the predicate upon beginning
conditional rendering while others may read it before every rendering
command.
## Valid Usage
-    If [`buffer`] is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-  [`buffer`] **must**  have been created with the `VK_BUFFER_USAGE_CONDITIONAL_RENDERING_BIT_EXT` bit set
-  [`offset`] **must**  be less than the size of [`buffer`] by at least 32 bits
-  [`offset`] **must**  be a multiple of 4

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_CONDITIONAL_RENDERING_BEGIN_INFO_EXT`
-  [`p_next`] **must**  be `NULL`
-  [`buffer`] **must**  be a valid [`Buffer`] handle
-  [`flags`] **must**  be a valid combination of [`ConditionalRenderingFlagBitsEXT`] values

# Related
- [`VK_EXT_conditional_rendering`]
- [`Buffer`]
- [`ConditionalRenderingFlagsEXT`]
- [`DeviceSize`]
- [`StructureType`]
- [`cmd_begin_conditional_rendering_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        