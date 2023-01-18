[`CLEAR`] specifies that the contents within the
render area will be cleared to a uniform value, which is specified when
a render pass instance is begun.
For attachments with a depth/stencil format, this uses the access type
`VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT`.
For attachments with a color format, this uses the access type
`VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT`.