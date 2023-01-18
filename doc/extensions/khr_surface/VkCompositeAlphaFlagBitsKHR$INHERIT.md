[`INHERIT`]: The way in which the
presentation engine treats the alpha component in the images is unknown
to the Vulkan API.
Instead, the application is responsible for setting the composite alpha
blending mode using native window system commands.
If the application does not set the blending mode using native window
system commands, then a platform-specific default will be used.