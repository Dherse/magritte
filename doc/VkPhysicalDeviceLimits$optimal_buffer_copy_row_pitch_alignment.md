[`optimal_buffer_copy_row_pitch_alignment`] is the optimal buffer row pitch
alignment in bytes for
[`cmd_copy_buffer_to_image2`], [`cmd_copy_buffer_to_image`],
[`cmd_copy_image_to_buffer2`], and [`cmd_copy_image_to_buffer`].
Row pitch is the number of bytes between texels with the same X
coordinate in adjacent rows (Y coordinates differ by one).
The per texel alignment requirements are enforced, but applications
 **should**  use the optimal alignment for optimal performance and power use.
The value  **must**  be a power of two.