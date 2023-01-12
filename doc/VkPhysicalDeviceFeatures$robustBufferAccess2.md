If
[`robustBufferAccess2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-robustBufferAccess2) is not
enabled and
any buffer access is determined to be out of bounds, then any other
access of the same type (load, store, or atomic) to the same buffer
that accesses an address less than 16 bytes away from the out of
bounds address  **may**  also be considered out of bounds.