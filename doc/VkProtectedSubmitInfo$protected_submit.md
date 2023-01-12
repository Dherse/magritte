[`protected_submit`] specifies whether the batch is protected.
If [`protected_submit`] is `VK_TRUE`, the batch is protected.
If [`protected_submit`] is `VK_FALSE`, the batch is unprotected.
If the [`SubmitInfo`]::[`p_next`] chain does not include this
structure, the batch is unprotected.