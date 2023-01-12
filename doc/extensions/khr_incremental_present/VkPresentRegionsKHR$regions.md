[`regions`] is `NULL` or a pointer to an array of
[`PresentRegionKHR`] elements with [`swapchain_count`] entries.
If not `NULL`, each element of [`regions`] contains the region that
has changed since the last present to the swapchain in the corresponding
entry in the [`PresentInfoKHR`]::`pSwapchains` array.