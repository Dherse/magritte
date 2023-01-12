[`full_draw_index_uint32`] specifies the
full 32-bit range of indices is supported for indexed draw calls when
using a [`IndexType`] of `VK_INDEX_TYPE_UINT32`.
`maxDrawIndexedIndexValue` is the maximum index value that  **may**  be
used (aside from the primitive restart index, which is always 2<sup>32</sup>-1
when the [`IndexType`] is `VK_INDEX_TYPE_UINT32`).
If this feature is supported, `maxDrawIndexedIndexValue` **must**  be
2<sup>32</sup>-1; otherwise it  **must**  be no smaller than 2<sup>24</sup>-1.
See [maxDrawIndexedIndexValue](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxDrawIndexedIndexValue).