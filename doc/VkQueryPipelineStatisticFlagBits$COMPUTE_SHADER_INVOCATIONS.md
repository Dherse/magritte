[`COMPUTE_SHADER_INVOCATIONS`]
specifies that queries managed by the pool will count the number of
compute shader invocations.
The counter’s value is incremented every time the compute shader is
invoked.
Implementations  **may**  skip the execution of certain compute shader
invocations or execute additional compute shader invocations for
implementation-dependent reasons as long as the results of rendering
otherwise remain unchanged.