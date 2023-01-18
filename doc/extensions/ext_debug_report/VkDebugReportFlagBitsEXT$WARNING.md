[`WARNING`] specifies use of Vulkan that  **may** 
expose an app bug.
Such cases may not be immediately harmful, such as a fragment shader
outputting to a location with no attachment.
Other cases  **may**  point to behavior that is almost certainly bad when
unintended such as using an image whose memory has not been filled.
In general if you see a warning but you know that the behavior is
intended/desired, then simply ignore the warning.