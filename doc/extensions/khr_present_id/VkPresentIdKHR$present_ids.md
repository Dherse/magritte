[`present_ids`] is `NULL` or a pointer to an array of uint64_t with
[`swapchain_count`] entries.
If not `NULL`, each non-zero value in [`present_ids`] specifies the
present id to be associated with the presentation of the swapchain with
the same index in the [`queue_present_khr`] call.