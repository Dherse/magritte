[`VK_IMAGE_LAYOUT`] specifies that the layout is unknown.
Image memory  **cannot**  be transitioned into this layout.
This layout  **can**  be used as the `initialLayout` member of
[`ImageCreateInfo`].
This layout  **can**  be used in place of the current image layout in a
layout transition, but doing so will cause the contents of the image’s
memory to be undefined.