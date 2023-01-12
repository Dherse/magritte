[`HOST_MAPPED_FOREIGN_MEMORY_EXT`]
specifies a host pointer to *host mapped foreign memory*.
It does not own a reference to the underlying memory resource, and will
therefore become invalid if the foreign memory is unmapped or otherwise
becomes no longer available.