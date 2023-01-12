[`pp_data`] is a pointer to a `void *` variable in which is returned a
host-accessible pointer to the beginning of the mapped range.
This pointer minus [`offset`] **must**  be aligned to at least
[`PhysicalDeviceLimits`]::`minMemoryMapAlignment`.