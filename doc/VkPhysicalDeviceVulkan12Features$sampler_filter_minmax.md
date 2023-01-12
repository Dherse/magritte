[`sampler_filter_minmax`] indicates
whether the implementation supports a minimum set of required formats
supporting min/max filtering as defined by the
[`filterMinmaxSingleComponentFormats`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-filterMinmaxSingleComponentFormats-minimum-requirements)
property minimum requirements.
If this feature is not enabled, then no [`SamplerCreateInfo`][`p_next`] chain can include a [`SamplerReductionModeCreateInfo`]
structure.