[`SAMPLE_LOCATIONS_EXT`] specifies that the
`sampleLocationsInfo` state in
[`PipelineSampleLocationsStateCreateInfoEXT`] will be ignored and
 **must**  be set dynamically with [`cmd_set_sample_locations_ext`] before
any draw or clear commands.
Enabling custom sample locations is still indicated by the
`sampleLocationsEnable` member of
[`PipelineSampleLocationsStateCreateInfoEXT`].