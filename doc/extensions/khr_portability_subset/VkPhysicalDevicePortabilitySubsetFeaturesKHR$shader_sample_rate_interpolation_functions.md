[`shader_sample_rate_interpolation_functions`] indicates whether this
implementation supports fragment shaders which use the
[`InterpolationFunction`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#spirvenv-capabilities-table-InterpolationFunction) capability and the extended instructions
`InterpolateAtCentroid`, `InterpolateAtOffset`, and
`InterpolateAtSample` from the `GLSL.std.450` extended instruction set.
This member is only meaningful if the
[sampleRateShading](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-sampleRateShading) feature is supported.