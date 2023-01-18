[`DISALLOWED`] specifies the
application is not allowed to allocate device memory beyond the heap
sizes reported by [`PhysicalDeviceMemoryProperties`].
Allocations that are not explicitly made by the application within the
scope of the Vulkan instance are not accounted for.