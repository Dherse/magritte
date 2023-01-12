[`ALIAS`] specifies that two images created with
    the same creation parameters and aliased to the same memory  **can** 
    interpret the contents of the memory consistently with each other,
    subject to the rules described in the [Memory
    Aliasing](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-memory-aliasing) section.
    This flag further specifies that each plane of a *disjoint* image  **can** 
    share an in-memory non-linear representation with single-plane images,
    and that a single-plane image  **can**  share an in-memory non-linear
    representation with a plane of a multi-planar disjoint image, according
    to the rules in [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-compatible-planes](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-compatible-planes).
    If the `pNext` chain includes a
[`ExternalMemoryImageCreateInfo`]
or
[`ExternalMemoryImageCreateInfoNV`]
    structure whose `handleTypes` member is not `0`, it is as if
    [`ALIAS`] is set.