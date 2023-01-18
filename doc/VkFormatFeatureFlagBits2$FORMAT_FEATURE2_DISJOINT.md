[`FORMAT_FEATURE2_DISJOINT`] specifies that a multi-planar
image  **can**  have the `VK_IMAGE_CREATE_DISJOINT_BIT` set during image
creation.
An implementation  **must**  not set [`FORMAT_FEATURE2_DISJOINT`]
for *single-plane formats*.