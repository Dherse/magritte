[`residency_non_resident_strict`] specifies whether the physical device
 **can**  consistently access non-resident regions of a resource.
If this property is `VK_TRUE`, access to non-resident regions of
resources will be guaranteed to return values as if the resource was
populated with 0; writes to non-resident regions will be discarded.