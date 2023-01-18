[`TRANSIENT_ATTACHMENT`] specifies that
implementations  **may**  support using [memory allocations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory) with
the `VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT` to back an image with
this usage.
This bit  **can**  be set for any image that  **can**  be used to create a
[`ImageView`] suitable for use as a color, resolve, depth/stencil,
or input attachment.