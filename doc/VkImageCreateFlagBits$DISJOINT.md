[`DISJOINT`] specifies that an image with a
[multi-planar format](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion) **must** 
have each plane separately bound to memory, rather than having a single
memory binding for the whole image; the presence of this bit
distinguishes a *disjoint image* from an image without this bit set.