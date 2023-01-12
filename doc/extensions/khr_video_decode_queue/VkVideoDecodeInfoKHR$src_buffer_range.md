[`src_buffer_range`] is the size of the srcBuffer with valid encoded
bitstream, starting from [`src_buffer_offset`].
It  **must**  meet the alignment requirement
`minBitstreamBufferSizeAlignment` within
[`VideoCapabilitiesKHR`] queried with the
[`get_physical_device_video_capabilities_khr`] function.