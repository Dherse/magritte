[`ANY`] specifies that the semaphore wait
condition is that at least one of the semaphores in
[`SemaphoreWaitInfo`]::`pSemaphores` has reached the value
specified by the corresponding element of
[`SemaphoreWaitInfo`]::`pValues`.
If [`ANY`] is not set, the semaphore wait
condition is that all of the semaphores in
[`SemaphoreWaitInfo`]::`pSemaphores` have reached the value
specified by the corresponding element of
[`SemaphoreWaitInfo`]::`pValues`.