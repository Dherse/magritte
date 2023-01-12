[`shader_output_layer`] indicates whether
the implementation supports the `ShaderLayer` SPIR-V capability
enabling variables decorated with the `Layer` built-in to be exported
from vertex or tessellation evaluation shaders.
If this feature is not enabled, the `Layer` built-in decoration  **must** 
not be used on outputs in vertex or tessellation evaluation shaders.