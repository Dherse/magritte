[`NO_DUPLICATE_ANY_HIT_INVOCATION`] indicates that
the implementation  **must**  only call the any-hit shader a single time for
each primitive in this geometry.
If this bit is absent an implementation  **may**  invoke the any-hit shader
more than once for this geometry.