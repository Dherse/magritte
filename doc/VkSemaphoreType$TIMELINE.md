[`TIMELINE`] specifies a *timeline semaphore* type
that has a strictly increasing 64-bit unsigned integer payload
indicating whether the semaphore is signaled with respect to a
particular reference value.
When created, the semaphore payload has the value given by the
`initialValue` field of [`SemaphoreTypeCreateInfo`].