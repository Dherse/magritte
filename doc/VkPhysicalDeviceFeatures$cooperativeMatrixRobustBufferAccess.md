The pointer was not formed by `OpImageTexelPointer` and the object
pointed to is not wholly contained within the bound range.
This includes accesses performed via *variable pointers* where the
buffer descriptor being accessed cannot be statically determined.
Uninitialized pointers and pointers equal to `OpConstantNull` are
treated as pointing to a zero-sized object, so all accesses through
such pointers are considered to be out of bounds.
Buffer accesses through buffer device addresses are not
bounds-checked.
If the
[`cooperativeMatrixRobustBufferAccess`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-cooperativeMatrixRobustBufferAccess)
feature is not enabled, then accesses using
`OpCooperativeMatrixLoadNV` and `OpCooperativeMatrixStoreNV` **may**  not be bounds-checked.