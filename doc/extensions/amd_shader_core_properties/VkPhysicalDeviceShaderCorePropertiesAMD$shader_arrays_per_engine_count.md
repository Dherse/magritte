[`shader_arrays_per_engine_count`]
is an unsigned integer value indicating the number of shader arrays
inside a shader engine.
Each shader array has its own scan converter, set of compute units, and
a render back end (color and depth attachments).
Shader arrays within a shader engine share shader processor input (wave
launcher) and shader export (export buffer) units.
Currently, a shader engine can have one or two shader arrays.