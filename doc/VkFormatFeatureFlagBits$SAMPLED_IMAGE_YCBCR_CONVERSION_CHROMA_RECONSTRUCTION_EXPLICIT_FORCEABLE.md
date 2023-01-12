[`SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE`]
specifies that reconstruction  **can**  be forcibly made explicit by setting
[`SamplerYcbcrConversionCreateInfo`]::`forceExplicitReconstruction`
to `VK_TRUE`.
If the format being queried supports
[`SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT`]
it  **must**  also support
[`SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE`].