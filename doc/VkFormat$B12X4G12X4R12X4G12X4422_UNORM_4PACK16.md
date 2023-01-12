[`B12X4G12X4R12X4G12X4422_UNORM_4PACK16`] specifies a
four-component, 64-bit format containing a pair of G components, an R
component, and a B component, collectively encoding a 2×1
rectangle of unsigned normalized RGB texel data.
One G value is present at each *i* coordinate, with the B and R values
shared across both G values and thus recorded at half the horizontal
resolution of the image.
This format has a 12-bit B component in the top 12 bits of the word in
bytes 0..1, a 12-bit G component for the even *i* coordinate in the top
12 bits of the word in bytes 2..3, a 12-bit R component in the top 12
bits of the word in bytes 4..5, and a 12-bit G component for the odd *i*
coordinate in the top 12 bits of the word in bytes 6..7, with the bottom
4 bits of each word unused.
This format only supports images with a width that is a multiple of two.
For the purposes of the constraints on copy extents, this format is
treated as a compressed format with a 2×1 compressed texel block.