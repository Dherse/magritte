[`max_compute_shared_memory_size`]
is the maximum total storage size, in bytes, available for variables
declared with the `Workgroup` storage class in shader modules (or
with the `shared` storage qualifier in GLSL) in the compute shader
stage.
When variables declared with the `Workgroup` storage class are
explicitly laid out (hence they are also decorated with `Block`), the
amount of storage consumed is the size of the largest Block variable,
not counting any padding at the end.
The amount of storage consumed by the
non-Block
variables declared with the `Workgroup` storage class is
implementation-dependent.
However, the amount of storage consumed may not exceed the largest block
size that would be obtained if all active
non-Block
variables declared with `Workgroup` storage class were assigned
offsets in an arbitrary order by successively taking the smallest valid
offset according to the [Standard
Storage Buffer Layout](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#interfaces-resources-standard-layout) rules.
(This is equivalent to using the GLSL std430 layout rules.)