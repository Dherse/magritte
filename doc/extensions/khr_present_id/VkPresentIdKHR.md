[VkPresentIdKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPresentIdKHR.html) - The list of presentation identifiers

# C Specifications
The [`PresentIdKHR`] structure is defined as:
```c
// Provided by VK_KHR_present_id
typedef struct VkPresentIdKHR {
    VkStructureType    sType;
    const void*        pNext;
    uint32_t           swapchainCount;
    const uint64_t*    pPresentIds;
} VkPresentIdKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`swapchain_count`] is the number of swapchains being presented to the [`queue_present_khr`] command.
- [`present_ids`] is `NULL` or a pointer to an array of uint64_t with [`swapchain_count`] entries. If not `NULL`, each non-zero value in [`present_ids`] specifies the present id to be associated with the presentation of the swapchain with the same index in the [`queue_present_khr`] call.

# Description
For applications to be able to reference specific presentation events queued
by a call to [`queue_present_khr`], an identifier needs to be associated
with them.
When the [`presentId`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-presentId) feature is enabled,
applications  **can**  include the [`PresentIdKHR`] structure in the
[`p_next`] chain of the [`PresentInfoKHR`] structure to supply
identifiers.Each [`SwapchainKHR`] has a presentId associated with it.
This value is initially set to zero when the [`SwapchainKHR`] is
created.When a [`PresentIdKHR`] structure with a non-NULL [`present_ids`] is
included in the [`p_next`] chain of a [`PresentInfoKHR`] structure,
each `pSwapchains` entry has a presentId associated in the
[`present_ids`] array at the same index as the swapchain in the
`pSwapchains` array.
If this presentId is non-zero, then the application  **can**  later use this
value to refer to that image presentation.
A value of zero indicates that this presentation has no associated
presentId.
A non-zero presentId  **must**  be greater than any non-zero presentId passed
previously by the application for the same swapchain.There is no requirement for any precise timing relationship between the
presentation of the image to the user and the update of the presentId value,
but implementations  **should**  make this as close as possible to the
presentation of the first pixel in the new image to the user.
## Valid Usage
-  [`swapchain_count`] **must**  be the same value as [`PresentInfoKHR`]::[`swapchain_count`], where this [`PresentIdKHR`] is in the [`p_next`] chain of the [`PresentInfoKHR`] structure
-    Each `presentIds` entry  **must**  be greater than any previous `presentIds` entry passed for the associated `pSwapchains` entry

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PRESENT_ID_KHR`
-    If [`present_ids`] is not `NULL`, [`present_ids`] **must**  be a valid pointer to an array of [`swapchain_count`]`uint64_t` values
-  [`swapchain_count`] **must**  be greater than `0`

# Related
- [`khr_present_id`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        