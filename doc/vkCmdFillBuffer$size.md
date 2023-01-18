[`size`] is the number of bytes to fill, and  **must**  be either a
multiple of 4, or [`WHOLE_SIZE`] to fill the range from
`offset` to the end of the buffer.
If [`WHOLE_SIZE`] is used and the remaining size of the buffer is
not a multiple of 4, then the nearest smaller multiple is used.