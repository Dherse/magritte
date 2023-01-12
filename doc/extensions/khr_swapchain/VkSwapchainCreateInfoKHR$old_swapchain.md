[`old_swapchain`] is [`crate::Handle::null`], or the existing non-retired
swapchain currently associated with [`surface`].
Providing a valid [`old_swapchain`] **may**  aid in the resource reuse, and
also allows the application to still present any images that are already
acquired from it.