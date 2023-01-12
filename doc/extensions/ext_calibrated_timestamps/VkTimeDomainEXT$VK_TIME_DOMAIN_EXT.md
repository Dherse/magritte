[`VK_TIME_DOMAIN_EXT`] specifies the device time domain.
Timestamp values in this time domain use the same units and are
comparable with device timestamp values captured using
[`cmd_write_timestamp`]
or [`cmd_write_timestamp2`]
and are defined to be incrementing according to the
[timestampPeriod](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-timestampPeriod) of the device.