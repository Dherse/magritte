[`clear_values`] is a pointer to an array of [`clear_value_count`][`ClearValue`] structures containing clear values for each
attachment, if the attachment uses a `loadOp` value of
`VK_ATTACHMENT_LOAD_OP_CLEAR` or if the attachment has a
depth/stencil format and uses a `stencilLoadOp` value of
`VK_ATTACHMENT_LOAD_OP_CLEAR`.
The array is indexed by attachment number.
Only elements corresponding to cleared attachments are used.
Other elements of [`clear_values`] are ignored.