[`timestamp_valid_bits`] is the unsigned integer count of meaningful
bits in the timestamps written via
[`cmd_write_timestamp2`] or
[`cmd_write_timestamp`].
The valid range for the count is 36..64 bits, or a value of 0,
indicating no support for timestamps.
Bits outside the valid range are guaranteed to be zeros.