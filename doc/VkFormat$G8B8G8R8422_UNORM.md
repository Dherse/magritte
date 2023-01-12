[`G8B8G8R8422_UNORM`] specifies a four-component, 32-bit
format containing a pair of G components, an R component, and a B
component, collectively encoding a 2×1 rectangle of unsigned
normalized RGB texel data.
One G value is present at each *i* coordinate, with the B and R values
shared across both G values and thus recorded at half the horizontal
resolution of the image.
This format has an 8-bit G component for the even *i* coordinate in byte
0, an 8-bit B component in byte 1, an 8-bit G component for the odd *i*
coordinate in byte 2, and an 8-bit R component in byte 3.
This format only supports images with a width that is a multiple of two.
For the purposes of the constraints on copy extents, this format is
treated as a compressed format with a 2×1 compressed texel block.