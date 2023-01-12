[`max_push_constants_size`] is the
maximum size, in bytes, of the pool of push constant memory.
For each of the push constant ranges indicated by the
`pPushConstantRanges` member of the [`PipelineLayoutCreateInfo`]
structure, (`offset` +  `size`) **must**  be less than or
equal to this limit.