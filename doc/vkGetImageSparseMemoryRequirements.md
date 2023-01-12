[vkGetImageSparseMemoryRequirements](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageSparseMemoryRequirements.html) - Query the memory requirements for a sparse image

# C Specifications
To query sparse memory requirements for an image, call:
```c
// Provided by VK_VERSION_1_0
void vkGetImageSparseMemoryRequirements(
    VkDevice                                    device,
    VkImage                                     image,
    uint32_t*                                   pSparseMemoryRequirementCount,
    VkSparseImageMemoryRequirements*            pSparseMemoryRequirements);
```

# Parameters
- [`device`] is the logical device that owns the image.
- [`image`] is the [`Image`] object to get the memory requirements for.
- [`p_sparse_memory_requirement_count`] is a pointer to an integer related to the number of sparse memory requirements available or queried, as described below.
- [`p_sparse_memory_requirements`] is either `NULL` or a pointer to an array of [`SparseImageMemoryRequirements`] structures.

# Description
If [`p_sparse_memory_requirements`] is `NULL`, then the number of sparse
memory requirements available is returned in
[`p_sparse_memory_requirement_count`].
Otherwise, [`p_sparse_memory_requirement_count`] **must**  point to a variable set
by the user to the number of elements in the [`p_sparse_memory_requirements`]
array, and on return the variable is overwritten with the number of
structures actually written to [`p_sparse_memory_requirements`].
If [`p_sparse_memory_requirement_count`] is less than the number of sparse
memory requirements available, at most [`p_sparse_memory_requirement_count`]
structures will be written.If the image was not created with `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT`
then [`p_sparse_memory_requirement_count`] will be set to zero and
[`p_sparse_memory_requirements`] will not be written to.
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`image`] **must**  be a valid [`Image`] handle
-  [`p_sparse_memory_requirement_count`] **must**  be a valid pointer to a `uint32_t` value
-    If the value referenced by [`p_sparse_memory_requirement_count`] is not `0`, and [`p_sparse_memory_requirements`] is not `NULL`, [`p_sparse_memory_requirements`] **must**  be a valid pointer to an array of [`p_sparse_memory_requirement_count`][`SparseImageMemoryRequirements`] structures
-  [`image`] **must**  have been created, allocated, or retrieved from [`device`]

# Related
- [`crate::vulkan1_0`]
- [`Device`]
- [`Image`]
- [`SparseImageMemoryRequirements`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        