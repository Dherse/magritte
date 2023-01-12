[`image_mip_tail_size`] is the memory size (in bytes) of the mip tail
region.
If `formatProperties.flags` contains
`VK_SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT`, this is the size of the
whole mip tail, otherwise this is the size of the mip tail of a single
array layer.
This value is guaranteed to be a multiple of the sparse block size in
bytes.