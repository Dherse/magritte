[`virtual_buffer_size_in_ms`] is the leaky bucket model virtual buffer
size in milliseconds, with respect to peak bitrate.
Valid when rate control mode is not
`VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`.
For example, virtual buffer size is ([`virtual_buffer_size_in_ms`] *
[`max_bitrate`] / 1000).