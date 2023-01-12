[`MUTABLE_FORMAT`] specifies that the
images of the swapchain  **can**  be used to create a [`ImageView`] with
a different format than what the swapchain was created with.
The list of allowed image view formats is specified by adding a
[`ImageFormatListCreateInfo`] structure to the `pNext` chain of
[`SwapchainCreateInfoKHR`].
In addition, this flag also specifies that the swapchain  **can**  be created
with usage flags that are not supported for the format the swapchain is
created with but are supported for at least one of the allowed image
view formats.