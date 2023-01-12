[`COSITED_CHROMA_SAMPLES`] specifies that an
application  **can**  define a [sampler Y′C<sub>B</sub>C<sub>R</sub>
conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-YCbCr-conversion) using this format as a source, and that an image of this
format  **can**  be used with a [`SamplerYcbcrConversionCreateInfo`]`xChromaOffset` and/or `yChromaOffset` of
`VK_CHROMA_LOCATION_COSITED_EVEN`.
Otherwise both `xChromaOffset` and `yChromaOffset` **must**  be
`VK_CHROMA_LOCATION_MIDPOINT`.
If neither [`COSITED_CHROMA_SAMPLES`] nor
[`MIDPOINT_CHROMA_SAMPLES`] is set, the
application  **must**  not define a [sampler
Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-YCbCr-conversion) using this format as a source.