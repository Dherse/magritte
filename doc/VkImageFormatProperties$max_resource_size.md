[`max_resource_size`] is an upper bound on the total image size in
bytes, inclusive of all image subresources.
Implementations  **may**  have an address space limit on total size of a
resource, which is advertised by this property.
[`max_resource_size`] **must**  be at least 2<sup>31</sup>.