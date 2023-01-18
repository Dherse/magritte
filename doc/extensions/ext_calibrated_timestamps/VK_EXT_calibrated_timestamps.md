[VK_EXT_calibrated_timestamps](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_calibrated_timestamps.html) - device extension

# Description
This extension provides an interface to query calibrated timestamps obtained
quasi simultaneously from two time domains.

# Registered extension number
185

# Revision
2

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_get_physical_device_properties2`]`

# Contacts
- Daniel Rakos [drakos-amd](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_calibrated_timestamps] @drakos-amd%0A<<Here describe the issue or question you have about the VK_EXT_calibrated_timestamps extension>>)

# New commands
- [`get_calibrated_timestamps_ext`]
- [`get_physical_device_calibrateable_time_domains_ext`]

# New structures
- [`CalibratedTimestampInfoEXT`]

# New enums
- [`TimeDomainEXT`]

# New constants
- [`EXT_CALIBRATED_TIMESTAMPS_EXTENSION_NAME`]
- [`EXT_CALIBRATED_TIMESTAMPS_SPEC_VERSION`]
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_CALIBRATED_TIMESTAMP_INFO_EXT`

# Known issues & F.A.Q.
1) Is the device timestamp value returned in the same time domain as the
timestamp values written by [`cmd_write_timestamp`]? **RESOLVED** : Yes.2) What time domain is the host timestamp returned in? **RESOLVED** : A query is provided to determine the calibrateable time domains.
The expected host time domain used on Windows is that of
QueryPerformanceCounter, and on Linux that of CLOCK_MONOTONIC.3) Should we support other time domain combinations than just one host and
the device time domain? **RESOLVED** : Supporting that would need the application to query the set of
supported time domains, while supporting only one host and the device time
domain would only need a query for the host time domain type.
The proposed API chooses the general approach for the sake of extensibility.4) Should we use CLOCK_MONOTONIC_RAW instead of CLOCK_MONOTONIC? **RESOLVED** : CLOCK_MONOTONIC is usable in a wider set of situations, however,
it is subject to NTP adjustments so some use cases may prefer
CLOCK_MONOTONIC_RAW.
Thus this extension allows both to be exposed.5) How can the application extrapolate future device timestamp values from
the calibrated timestamp value? **RESOLVED** : [`PhysicalDeviceLimits::timestamp_period`] makes it
possible to calculate future device timestamps as follows:6) In what queue are timestamp values in time domain
`VK_TIME_DOMAIN_DEVICE_EXT` captured by
[`get_calibrated_timestamps_ext`]? **RESOLVED** : An implementation supporting this extension will have all its
VkQueue share the same time domain.
```c
futureTimestamp = calibratedTimestamp + deltaNanoseconds / timestampPeriod
```
6) Can the host and device timestamp values drift apart over longer periods
of time? **RESOLVED** : Yes, especially as some time domains by definition allow for
that to happen (e.g. CLOCK_MONOTONIC is subject to NTP adjustments).
Thus it is recommended that applications re-calibrate from time to time.7) Should we add a query for reporting the maximum deviation of the
timestamp values returned by calibrated timestamp queries? **RESOLVED** : A global query seems inappropriate and difficult to enforce.
However, it is possible to return the maximum deviation any single
calibrated timestamp query can have by sampling one of the time domains
twice as follows:
```c
timestampX = timestampX_before = SampleTimeDomain(X)
for each time domain Y != X
    timestampY = SampleTimeDomain(Y)
timestampX_after = SampleTimeDomain(X)
maxDeviation = timestampX_after - timestampX_before
```
8) Can the maximum deviation reported ever be zero? **RESOLVED** : Unless the tick of each clock corresponding to the set of time
domains coincides and all clocks can literally be sampled simutaneously,
there is not really a possibility for the maximum deviation to be zero, so
by convention the maximum deviation is always at least the maximum of the
length of the ticks of the set of time domains calibrated and thus can never
be zero.

# Version history
- Revision 2, 2021-03-16 (Lionel Landwerlin)  - Specify requirement on device timestamps 
- Revision 1, 2018-10-04 (Daniel Rakos)  - Internal revisions.

# Other information
* 2018-10-04
* No known IP claims.
*   - Matthaeus G. Chajdas, AMD  - Alan Harrison, AMD  - Derrick Owens, AMD  - Daniel Rakos, AMD  - Jason Ekstrand, Intel  - Keith Packard, Valve

# Related
- [`CalibratedTimestampInfoEXT`]
- [`TimeDomainEXT`]
- [`get_calibrated_timestamps_ext`]
- [`get_physical_device_calibrateable_time_domains_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        