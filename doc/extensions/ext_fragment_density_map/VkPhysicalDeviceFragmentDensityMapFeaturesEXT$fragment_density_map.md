[`fragment_density_map`] specifies
whether the implementation supports render passes with a fragment
density map attachment.
If this feature is not enabled and the [`p_next`] chain of
[`RenderPassCreateInfo`] includes a
[`RenderPassFragmentDensityMapCreateInfoEXT`] structure,
`fragmentDensityMapAttachment` **must**  be `VK_ATTACHMENT_UNUSED`.