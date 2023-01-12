[`VK_DESCRIPTOR_BINDING_FLAG_BITS`] indicates that if
descriptors in this binding are updated between when the descriptor set
is bound in a command buffer and when that command buffer is submitted
to a queue, then the submission will use the most recently set
descriptors for this binding and the updates do not invalidate the
command buffer.
Descriptor bindings created with this flag are also partially exempt
from the external synchronization requirement in
[`update_descriptor_set_with_template_khr`] and
[`update_descriptor_sets`].
Multiple descriptors with this flag set  **can**  be updated concurrently in
different threads, though the same descriptor  **must**  not be updated
concurrently by two threads.
Descriptors with this flag set  **can**  be updated concurrently with the set
being bound to a command buffer in another thread, but not concurrently
with the set being reset or freed.