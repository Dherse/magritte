[`DONT_CARE`] specifies the contents within the
render area are not needed after rendering, and  **may**  be discarded; the
contents of the attachment will be undefined inside the render area.
For attachments with a depth/stencil format, this uses the access type
`VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT`.
For attachments with a color format, this uses the access type
`VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT`.