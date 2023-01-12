[`residency_standard2_d_multisample_block_shape`] is `VK_TRUE` if the
physical device will access all multisample 2D sparse resources using
the standard sparse image block shapes (based on image format), as
described in the [Standard Sparse
Image Block Shapes (MSAA)](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#sparsememory-sparseblockshapesmsaa) table.
If this property is not supported, the value returned in the
`imageGranularity` member of the [`SparseImageFormatProperties`]
structure for multisample 2D images is not  **required**  to match the
standard sparse image block dimensions listed in the table.