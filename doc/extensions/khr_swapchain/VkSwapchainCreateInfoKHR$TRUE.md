If set to [`TRUE`], the presentable images associated with the
swapchain  **may**  not own all of their pixels.
Pixels in the presentable images that correspond to regions of the
target surface obscured by another window on the desktop, or subject to
some other clipping mechanism will have undefined content when read
back.
Fragment shaders  **may**  not execute for these pixels, and thus any side
effects they would have had will not occur.
Setting [`TRUE`] does not guarantee any clipping will occur, but
allows more efficient presentation methods to be used on some
platforms.