[`fragment_shading_rate_strict_multiply_combiner`] specifies whether
`VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MUL_KHR` accurately performs a
multiplication or not.
Implementations where this value is [`FALSE`] will instead combine
rates with an addition.
If [`fragment_shading_rate_non_trivial_combiner_ops`] is [`FALSE`],
implementations  **must**  report this as [`FALSE`].
If [`fragment_shading_rate_non_trivial_combiner_ops`] is [`TRUE`],
implementations  **should**  report this as [`TRUE`].