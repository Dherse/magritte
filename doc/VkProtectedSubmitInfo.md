[VkProtectedSubmitInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkProtectedSubmitInfo.html) - Structure indicating whether the submission is protected

# C Specifications
If the [`p_next`] chain of [`SubmitInfo`] includes a
[`ProtectedSubmitInfo`] structure, then the structure indicates whether
the batch is protected.
The [`ProtectedSubmitInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkProtectedSubmitInfo {
    VkStructureType    sType;
    const void*        pNext;
    VkBool32           protectedSubmit;
} VkProtectedSubmitInfo;
```

# Members
- [`protected_submit`] specifies whether the batch is protected. If [`protected_submit`] is [`TRUE`], the batch is protected. If [`protected_submit`] is [`FALSE`], the batch is unprotected. If the [`SubmitInfo`]::[`p_next`] chain does not include this structure, the batch is unprotected.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PROTECTED_SUBMIT_INFO`

# Related
- [`crate::vulkan1_1`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        