[`shader_image_gather_extended`]
specifies whether the extended set of image gather instructions are
available in shader code.
If this feature is not enabled, the `OpImage*Gather` instructions do
not support the `Offset` and `ConstOffsets` operands.
This also specifies whether shader modules  **can**  declare the
`ImageGatherExtended` capability.