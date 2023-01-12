[`stage_flags`] member is a bitmask of [`ShaderStageFlagBits`]
specifying which pipeline shader stages  **can**  access a resource for this
binding.
`VK_SHADER_STAGE_ALL` is a shorthand specifying that all defined
shader stages, including any additional stages defined by extensions,
 **can**  access the resource.If a shader stage is not included in [`stage_flags`], then a resource  **must** 
not be accessed from that stage via this binding within any pipeline using
the set layout.
Other than input attachments which are limited to the fragment shader, there
are no limitations on what combinations of stages  **can**  use a descriptor
binding, and in particular a binding  **can**  be used by both graphics stages
and the compute stage.