#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_FRAGMENT_SHADING_RATE_SPEC_VERSION")]
pub const KHR_FRAGMENT_SHADING_RATE_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_FRAGMENT_SHADING_RATE_EXTENSION_NAME")]
pub const KHR_FRAGMENT_SHADING_RATE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_fragment_shading_rate");
///[VkFragmentShadingRateCombinerOpKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFragmentShadingRateCombinerOpKHR.html) - Control how fragment shading rates are combined
///# C Specifications
///The equation used for each combiner operation is defined by
///[`FragmentShadingRateCombinerOpKHR`]:
///```c
///// Provided by VK_KHR_fragment_shading_rate
///typedef enum VkFragmentShadingRateCombinerOpKHR {
///    VK_FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP_KHR = 0,
///    VK_FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE_KHR = 1,
///    VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MIN_KHR = 2,
///    VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MAX_KHR = 3,
///    VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MUL_KHR = 4,
///} VkFragmentShadingRateCombinerOpKHR;
///```
///# Description
/// - [`FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP`] specifies a combiner
///operation of combine(A<sub>xy</sub>,B<sub>xy</sub>) = A<sub>xy</sub>.
/// - [`FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE`] specifies a
///combiner operation of combine(A<sub>xy</sub>,B<sub>xy</sub>) = B<sub>xy</sub>.
/// - [`FRAGMENT_SHADING_RATE_COMBINER_OP_MIN`] specifies a combiner
///operation of combine(A<sub>xy</sub>,B<sub>xy</sub>) = min(A<sub>xy</sub>,B<sub>xy</sub>).
/// - [`FRAGMENT_SHADING_RATE_COMBINER_OP_MAX`] specifies a combiner
///operation of combine(A<sub>xy</sub>,B<sub>xy</sub>) = max(A<sub>xy</sub>,B<sub>xy</sub>).
/// - [`FRAGMENT_SHADING_RATE_COMBINER_OP_MUL`] specifies a combiner
///operation of combine(A<sub>xy</sub>,B<sub>xy</sub>) = A<sub>xy</sub>*B<sub>xy</sub>.where
/// combine(A<sub>xy</sub>,B<sub>xy</sub>) is the combine operation, and A<sub>xy</sub>
///and B<sub>xy</sub> are the inputs to the operation.If [`fragmentShadingRateStrictMultiplyCombiner`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-fragmentShadingRateStrictMultiplyCombiner) is [`FALSE`], using
///[`FRAGMENT_SHADING_RATE_COMBINER_OP_MUL`] with values of 1 for both
///A and B in the same dimension results in the value 2 being produced for that
///dimension.
///See the definition of [`fragmentShadingRateStrictMultiplyCombiner`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-fragmentShadingRateStrictMultiplyCombiner) for more information.These operations are performed in a component-wise fashion.
///# Related
/// - [`VK_KHR_fragment_shading_rate`]
/// - [`PipelineFragmentShadingRateEnumStateCreateInfoNV`]
/// - [`PipelineFragmentShadingRateStateCreateInfoKHR`]
/// - [`CmdSetFragmentShadingRateEnumNV`]
/// - [`CmdSetFragmentShadingRateKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkFragmentShadingRateCombinerOpKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct FragmentShadingRateCombinerOpKHR(i32);
impl const Default for FragmentShadingRateCombinerOpKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for FragmentShadingRateCombinerOpKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("FragmentShadingRateCombinerOpKHR")
            .field(match *self {
                Self::FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP => &"FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP",
                Self::FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE => &"FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE",
                Self::FRAGMENT_SHADING_RATE_COMBINER_OP_MIN => &"FRAGMENT_SHADING_RATE_COMBINER_OP_MIN",
                Self::FRAGMENT_SHADING_RATE_COMBINER_OP_MAX => &"FRAGMENT_SHADING_RATE_COMBINER_OP_MAX",
                Self::FRAGMENT_SHADING_RATE_COMBINER_OP_MUL => &"FRAGMENT_SHADING_RATE_COMBINER_OP_MUL",
                other => unreachable!("invalid value for `FragmentShadingRateCombinerOpKHR`: {:?}", other),
            })
            .finish()
    }
}
impl FragmentShadingRateCombinerOpKHR {
    ///[`FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP`] specifies a combiner
    ///operation of combine(A<sub>xy</sub>,B<sub>xy</sub>) = A<sub>xy</sub>.
    pub const FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP: Self = Self(0);
    ///[`FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE`] specifies a
    ///combiner operation of combine(A<sub>xy</sub>,B<sub>xy</sub>) = B<sub>xy</sub>.
    pub const FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE: Self = Self(1);
    ///[`FRAGMENT_SHADING_RATE_COMBINER_OP_MIN`] specifies a combiner
    ///operation of combine(A<sub>xy</sub>,B<sub>xy</sub>) = min(A<sub>xy</sub>,B<sub>xy</sub>).
    pub const FRAGMENT_SHADING_RATE_COMBINER_OP_MIN: Self = Self(2);
    ///[`FRAGMENT_SHADING_RATE_COMBINER_OP_MAX`] specifies a combiner
    ///operation of combine(A<sub>xy</sub>,B<sub>xy</sub>) = max(A<sub>xy</sub>,B<sub>xy</sub>).
    pub const FRAGMENT_SHADING_RATE_COMBINER_OP_MAX: Self = Self(3);
    ///[`FRAGMENT_SHADING_RATE_COMBINER_OP_MUL`] specifies a combiner
    ///operation of combine(A<sub>xy</sub>,B<sub>xy</sub>) = A<sub>xy</sub>*B<sub>xy</sub>.
    pub const FRAGMENT_SHADING_RATE_COMBINER_OP_MUL: Self = Self(4);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
}
