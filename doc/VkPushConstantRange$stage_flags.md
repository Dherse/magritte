[`stage_flags`] is a set of stage flags describing the shader stages
that will access a range of push constants.
If a particular stage is not included in the range, then accessing
members of that range of push constants from the corresponding shader
stage will return undefined values.