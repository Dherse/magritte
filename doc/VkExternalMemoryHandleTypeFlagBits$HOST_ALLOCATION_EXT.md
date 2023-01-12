[`HOST_ALLOCATION_EXT`] specifies a
host pointer returned by a host memory allocation command.
It does not own a reference to the underlying memory resource, and will
therefore become invalid if the host memory is freed.