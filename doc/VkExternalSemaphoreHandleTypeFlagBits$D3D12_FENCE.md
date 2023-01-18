[`D3D12_FENCE`] specifies an NT
handle returned by `ID3D12Device`::`CreateSharedHandle` referring
to a Direct3D 12 fence, or `ID3D11Device5`::`CreateFence`
referring to a Direct3D 11 fence.
It owns a reference to the underlying synchronization primitive
associated with the Direct3D fence.