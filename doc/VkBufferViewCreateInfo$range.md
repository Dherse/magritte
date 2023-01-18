[`range`] is a size in bytes of the buffer view.
If [`range`] is equal to [`WHOLE_SIZE`], the range from
[`offset`] to the end of the buffer is used.
If [`WHOLE_SIZE`] is used and the remaining size of the buffer is
not a multiple of the [texel block size](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#texel-block-size) of
[`format`], the nearest smaller multiple is used.