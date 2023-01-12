[`command_buffer_device_masks`] is a pointer to an array of
[`command_buffer_count`] device masks indicating which physical devices
execute the command buffer in the corresponding element of
[`SubmitInfo`]::`pCommandBuffers`.
A physical device executes the command buffer if the corresponding bit
is set in the mask.