[`preprocess_buffer`] is the [`Buffer`] that is used for
preprocessing the input data for execution.
If this structure is used with [`cmd_execute_generated_commands_nv`]
with its `isPreprocessed` set to [`TRUE`], then the preprocessing
step is skipped and data is only read from this buffer.