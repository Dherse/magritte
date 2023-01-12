[`wait_all`] is the condition that  **must**  be satisfied to successfully
unblock the wait.
If [`wait_all`] is `VK_TRUE`, then the condition is that all fences
in [`p_fences`] are signaled.
Otherwise, the condition is that at least one fence in [`p_fences`] is
signaled.