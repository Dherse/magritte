//![VK_KHR_shader_integer_dot_product](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_shader_integer_dot_product.html) - device extension
//!# Description
//!This extension adds support for the integer dot product SPIR-V instructions
//!defined in SPV_KHR_integer_dot_product.
//!These instructions are particularly useful for neural network inference and
//!training but find uses in other general purpose compute applications as
//!well.
# ! [doc = concat ! ("# " , "Revision")]
//!1
# ! [doc = concat ! ("# " , "Dependencies")]
//! - Requires Vulkan 1.0
//! - Requires `[`khr_get_physical_device_properties2`]`
# ! [doc = concat ! ("# " , "Deprecation State")]
//! - *Promoted* to [Vulkan 1.3](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.3-promotions)
# ! [doc = concat ! ("# " , "Contacts")]
//! - Kevin Petit [kevinpetit](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_shader_integer_dot_product]
//!   @kevinpetit%0A<<Here describe the issue or question you have about the
//!   VK_KHR_shader_integer_dot_product extension>>)
# ! [doc = concat ! ("# " , "New structures")]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceShaderIntegerDotProductFeaturesKHR`]
//! - Extending [`PhysicalDeviceProperties2`]:  -
//!   [`PhysicalDeviceShaderIntegerDotProductPropertiesKHR`]
# ! [doc = concat ! ("# " , "New constants")]
//! - [`KHR_SHADER_INTEGER_DOT_PRODUCT_EXTENSION_NAME`]
//! - [`KHR_SHADER_INTEGER_DOT_PRODUCT_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES_KHR`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES_KHR`
# ! [doc = concat ! ("# " , "Version history")]
//! - Revision 1, 2021-06-16 (Kévin Petit)  - Initial revision
//!# Other info
//! * 2021-06-16
//! * - Promoted to Vulkan 1.3 Core  - This extension requires [`SPV_KHR_integer_dot_product`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_integer_dot_product.html).
//!   - This extension interacts with `[`khr_shader_float16_int8`]`.
//! * No known IP claims.
//! * - Kévin Petit, Arm Ltd.  - Jeff Bolz, NVidia  - Spencer Fricke, Samsung  - Jesse Hall, Google
//!   - John Kessenich, Google  - Graeme Leese, Broadcom  - Einar Hov, Arm Ltd.  - Stuart Brady, Arm
//!   Ltd.  - Pablo Cascon, Arm Ltd.  - Tobias Hector, AMD  - Jeff Leger, Qualcomm  - Ruihao Zhang,
//!   Qualcomm  - Pierre Boudier, NVidia  - Jon Leech, The Khronos Group  - Tom Olson, Arm Ltd.
//!# Related
//! - [`PhysicalDeviceShaderIntegerDotProductFeaturesKHR`]
//! - [`PhysicalDeviceShaderIntegerDotProductPropertiesKHR`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SHADER_INTEGER_DOT_PRODUCT_SPEC_VERSION")]
pub const KHR_SHADER_INTEGER_DOT_PRODUCT_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SHADER_INTEGER_DOT_PRODUCT_EXTENSION_NAME")]
pub const KHR_SHADER_INTEGER_DOT_PRODUCT_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_KHR_shader_integer_dot_product");
