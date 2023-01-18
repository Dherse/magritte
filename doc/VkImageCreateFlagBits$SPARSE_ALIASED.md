[`SPARSE_ALIASED`] specifies that the image will
be backed using sparse memory binding with memory ranges that might also
simultaneously be backing another image (or another portion of the same
image).
Images created with this flag  **must**  also be created with the
[`SPARSE_BINDING`] flag.