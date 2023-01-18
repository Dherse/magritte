[`SPARSE_ALIASED`] specifies that the buffer will
be backed using sparse memory binding with memory ranges that might also
simultaneously be backing another buffer (or another portion of the same
buffer).
Buffers created with this flag  **must**  also be created with the
[`SPARSE_BINDING`] flag.