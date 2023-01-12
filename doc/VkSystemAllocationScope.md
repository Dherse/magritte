[VkSystemAllocationScope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSystemAllocationScope.html) - Allocation scope

# C Specifications
Each allocation has an *allocation scope* defining its lifetime and which
object it is associated with.
Possible values passed to the `allocationScope` parameter of the
callback functions specified by [`AllocationCallbacks`], indicating the
allocation scope, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkSystemAllocationScope {
    VK_SYSTEM_ALLOCATION_SCOPE_COMMAND = 0,
    VK_SYSTEM_ALLOCATION_SCOPE_OBJECT = 1,
    VK_SYSTEM_ALLOCATION_SCOPE_CACHE = 2,
    VK_SYSTEM_ALLOCATION_SCOPE_DEVICE = 3,
    VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE = 4,
} VkSystemAllocationScope;
```

# Description
- [`VK_SYSTEM_ALLOCATION_SCOPE`] specifies that the allocation is scoped to the duration of the Vulkan command.
- [`VK_SYSTEM_ALLOCATION_SCOPE`] specifies that the allocation is scoped to the lifetime of the Vulkan object that is being created or used.
- [`VK_SYSTEM_ALLOCATION_SCOPE`] specifies that the allocation is scoped to the lifetime of a [`PipelineCache`] or [`ValidationCacheEXT`] object.
- [`VK_SYSTEM_ALLOCATION_SCOPE`] specifies that the allocation is scoped to the lifetime of the Vulkan device.
- [`VK_SYSTEM_ALLOCATION_SCOPE`] specifies that the allocation is scoped to the lifetime of the Vulkan instance.
Most Vulkan commands operate on a single object, or there is a sole object
that is being created or manipulated.
When an allocation uses an allocation scope of
[`VK_SYSTEM_ALLOCATION_SCOPE`] or
[`VK_SYSTEM_ALLOCATION_SCOPE`], the allocation is scoped to the
object being created or manipulated.When an implementation requires host memory, it will make callbacks to the
application using the most specific allocator and allocation scope
available:
- If an allocation is scoped to the duration of a command, the allocator will use the [`VK_SYSTEM_ALLOCATION_SCOPE`] allocation scope. The most specific allocator available is used: if the object being created or manipulated has an allocator, that object’s allocator will be used, else if the parent [`Device`] has an allocator it will be used, else if the parent [`Instance`] has an allocator it will be used. Else,
- If an allocation is associated with a [`ValidationCacheEXT`] or [`PipelineCache`] object, the allocator will use the [`VK_SYSTEM_ALLOCATION_SCOPE`] allocation scope. The most specific allocator available is used (cache, else device, else instance). Else,
- If an allocation is scoped to the lifetime of an object, that object is being created or manipulated by the command, and that object’s type is not [`Device`] or [`Instance`], the allocator will use an allocation scope of [`VK_SYSTEM_ALLOCATION_SCOPE`]. The most specific allocator available is used (object, else device, else instance). Else,
- If an allocation is scoped to the lifetime of a device, the allocator will use an allocation scope of [`VK_SYSTEM_ALLOCATION_SCOPE`]. The most specific allocator available is used (device, else instance). Else,
- If the allocation is scoped to the lifetime of an instance and the instance has an allocator, its allocator will be used with an allocation scope of [`VK_SYSTEM_ALLOCATION_SCOPE`].
- Otherwise an implementation will allocate memory through an alternative mechanism that is unspecified.

# Related
- [`crate::vulkan1_0`]
- [`AllocationCallbacks`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        