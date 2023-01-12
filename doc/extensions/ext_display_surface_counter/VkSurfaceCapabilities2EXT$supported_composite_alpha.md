[`supported_composite_alpha`] is a bitmask of
[`CompositeAlphaFlagBitsKHR`], representing the alpha compositing
modes supported by the presentation engine for the surface on the
specified device, and at least one bit will be set.
Opaque composition  **can**  be achieved in any alpha compositing mode by
either using an image format that has no alpha component, or by ensuring
that all pixels in the presentable images have an alpha value of 1.0.