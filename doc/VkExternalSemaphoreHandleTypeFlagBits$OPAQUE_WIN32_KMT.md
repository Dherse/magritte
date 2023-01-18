[`OPAQUE_WIN32_KMT`] specifies a
global share handle that has only limited valid usage outside of Vulkan
and other compatible APIs.
It is not compatible with any native APIs.
It does not own a reference to the underlying synchronization primitive
represented by its Vulkan semaphore object, and will therefore become
invalid when all Vulkan semaphore objects associated with it are
destroyed.