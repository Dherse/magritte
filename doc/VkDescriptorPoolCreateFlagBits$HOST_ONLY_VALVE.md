[`HOST_ONLY_VALVE`] specifies that this
descriptor pool and the descriptor sets allocated from it reside
entirely in host memory and cannot be bound.
Descriptor sets allocated from this pool are partially exempt from the
external synchronization requirement in
[`update_descriptor_set_with_template_khr`] and
[`update_descriptor_sets`].
Descriptor sets and their descriptors can be updated concurrently in
different threads, though the same descriptor  **must**  not be updated
concurrently by two threads.