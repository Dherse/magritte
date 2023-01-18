[VkSemaphoreTypeCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreTypeCreateInfo.html) - Structure specifying the type of a newly created semaphore

# C Specifications
The [`SemaphoreTypeCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_2
typedef struct VkSemaphoreTypeCreateInfo {
    VkStructureType    sType;
    const void*        pNext;
    VkSemaphoreType    semaphoreType;
    uint64_t           initialValue;
} VkSemaphoreTypeCreateInfo;
```
or the equivalent
```c
// Provided by VK_KHR_timeline_semaphore
typedef VkSemaphoreTypeCreateInfo VkSemaphoreTypeCreateInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`semaphore_type`] is a [`SemaphoreType`] value specifying the type of the semaphore.
- [`initial_value`] is the initial payload value if [`semaphore_type`] is `VK_SEMAPHORE_TYPE_TIMELINE`.

# Description
To create a semaphore of a specific type, add a
[`SemaphoreTypeCreateInfo`] structure to the
[`SemaphoreCreateInfo`]::[`p_next`] chain.If no [`SemaphoreTypeCreateInfo`] structure is included in the
[`p_next`] chain of [`SemaphoreCreateInfo`], then the created semaphore
will have a default [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_BINARY`.
## Valid Usage
-    If the [`timelineSemaphore`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-timelineSemaphore) feature is not enabled, [`semaphore_type`] **must**  not equal `VK_SEMAPHORE_TYPE_TIMELINE`
-    If [`semaphore_type`] is `VK_SEMAPHORE_TYPE_BINARY`, [`initial_value`] **must**  be zero

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO`
-  [`semaphore_type`] **must**  be a valid [`SemaphoreType`] value

# Related
- [`VK_KHR_timeline_semaphore`]
- [`crate::vulkan1_2`]
- [`SemaphoreType`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        