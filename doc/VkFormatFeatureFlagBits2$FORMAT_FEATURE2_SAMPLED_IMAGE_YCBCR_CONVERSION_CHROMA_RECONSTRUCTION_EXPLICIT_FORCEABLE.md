[`FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE`]
specifies that reconstruction  **can**  be forcibly made explicit by setting
[`SamplerYcbcrConversionCreateInfo`]::`forceExplicitReconstruction`
to [`TRUE`].
If the format being queried supports
[`FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT`]
it  **must**  also support
[`FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE`].