[`fragment_shading_rate_strict_multiply_combiner`] specifies whether
`VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MUL_KHR` accurately performs a
multiplication or not.
Implementations where this value is `VK_FALSE` will instead combine
rates with an addition.
If [`fragment_shading_rate_non_trivial_combiner_ops`] is `VK_FALSE`,
implementations  **must**  report this as `VK_FALSE`.
If [`fragment_shading_rate_non_trivial_combiner_ops`] is `VK_TRUE`,
implementations  **should**  report this as `VK_TRUE`.