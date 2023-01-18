[`PARTIALLY_BOUND`] indicates that
descriptors in this binding that are not *dynamically used* need not
contain valid descriptors at the time the descriptors are consumed.
A descriptor is dynamically used if any shader invocation executes an
instruction that performs any memory access using the descriptor.