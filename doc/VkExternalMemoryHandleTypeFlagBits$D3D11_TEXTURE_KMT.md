[`D3D11_TEXTURE_KMT`] specifies a
global share handle returned by `IDXGIResource`::`GetSharedHandle`
referring to a Direct3D 10 or 11 texture resource.
It does not own a reference to the underlying Direct3D resource, and
will therefore become invalid when all Vulkan memory objects and
Direct3D resources associated with it are destroyed.