[`residency_aligned_mip_size`] is `VK_TRUE` if images with mip level
dimensions that are not integer multiples of the corresponding
dimensions of the sparse image block  **may**  be placed in the mip tail.
If this property is not reported, only mip levels with dimensions
smaller than the `imageGranularity` member of the
[`SparseImageFormatProperties`] structure will be placed in the mip
tail.
If this property is reported the implementation is allowed to return
`VK_SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT` in the `flags`
member of [`SparseImageFormatProperties`], indicating that mip level
dimensions that are not integer multiples of the corresponding
dimensions of the sparse image block will be placed in the mip tail.