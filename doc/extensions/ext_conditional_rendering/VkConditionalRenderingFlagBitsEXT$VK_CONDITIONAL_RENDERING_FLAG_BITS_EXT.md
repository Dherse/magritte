[`VK_CONDITIONAL_RENDERING_FLAG_BITS_EXT`] specifies the condition
used to determine whether to discard rendering commands or not.
That is, if the 32-bit predicate read from `buffer` memory at
`offset` is zero, the rendering commands are not discarded, and if
non zero, then they are discarded.