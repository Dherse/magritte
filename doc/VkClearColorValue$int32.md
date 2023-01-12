[`int32`] are the color clear values when the format of the image or
attachment is signed integer (`SINT`).
Signed integer values are converted to the format of the image by
casting to the smaller type (with negative 32-bit values mapping to
negative values in the smaller type).
If the integer clear value is not representable in the target type (e.g.
would overflow in conversion to that type), the clear value is
undefined.