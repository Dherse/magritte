[`results`] is a pointer to an array of [`VulkanResultCodes`] typed elements
with [`swapchain_count`] entries.
Applications that do not need per-swapchain results  **can**  use `NULL` for
[`results`].
If non-`NULL`, each entry in [`results`] will be set to the
[`VulkanResultCodes`] for presenting the swapchain corresponding to the same
index in [`swapchains`].