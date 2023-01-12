[`fragment_shading_rate_with_shader_depth_stencil_writes`] specifies whether
the implementation supports writing `FragDepth`
or `FragStencilRefEXT`
from a fragment shader for multi-pixel fragments.
If this value is `VK_FALSE`, writing to those built-ins will clamp
the fragment shading rate to (1,1).