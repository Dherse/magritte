[`residency_standard2_d_block_shape`] is [`TRUE`] if the physical
device will access all single-sample 2D sparse resources using the
standard sparse image block shapes (based on image format), as described
in the [Standard Sparse Image
Block Shapes (Single Sample)](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#sparsememory-sparseblockshapessingle) table.
If this property is not supported the value returned in the
`imageGranularity` member of the [`SparseImageFormatProperties`]
structure for single-sample 2D images is not  **required**  to match the
standard sparse image block dimensions listed in the table.