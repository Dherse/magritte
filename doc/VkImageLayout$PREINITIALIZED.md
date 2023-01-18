[`PREINITIALIZED`] specifies that an image’s memory is
in a defined layout and  **can**  be populated by data, but that it has not
yet been initialized by the driver.
Image memory  **cannot**  be transitioned into this layout.
This layout  **can**  be used as the `initialLayout` member of
[`ImageCreateInfo`].
This layout is intended to be used as the initial layout for an image
whose contents are written by the host, and hence the data  **can**  be
written to memory immediately, without first executing a layout
transition.
Currently, [`PREINITIALIZED`] is only useful with
[linear](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-linear-resource) images because there is not a
standard layout defined for `VK_IMAGE_TILING_OPTIMAL` images.