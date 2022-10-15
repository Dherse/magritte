//! # Vulkan results
//! A wrapper around a [`VulkanResultCodes`]

use std::{
    convert::Infallible,
    hint::unreachable_unchecked,
    ops::{ControlFlow, FromResidual, Try},
};

use crate::vulkan1_0::VulkanResultCodes;

/// A result in the Vulkan sense, a success can have a code other than
/// [`VulkanResultCodes::Success`]. The exact success and error codes are defined for each function,
/// look at the function description for more information.
///
/// # Try
/// A `VulkanResult` supports the [`Try`] rust operator.
/// With this operator, you can seemlessly turn a [`VulkanResult`] into a normal [`Result`].
/// For more information on this operation, see the example below or the rust documentation of
/// [`Try`] and [`Result`].
///
/// ```
///     fn try_demo() -> Result<(), VulanErrorCodes> {
///         // create a `VulkanResult<&str>`
///         let vulkan_result = VulkanResult::Success(VulkanResultCodes::Success, "Hello world!");
///     
///         // try operation to get the `&str`
///         let message = vulkan_result?;
///     
///         // print them essage if it was successful
///         println!("{}", message);
///         
///         // return `Ok(())` if the result was a `VulkanResult::Success`
///         Ok(())
///     }
/// ```
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum VulkanResult<T> {
    /// The operation was a success
    Success(VulkanResultCodes, T),

    /// The operation failed
    Err(VulkanResultCodes),
}

impl<T> VulkanResult<T> {
    /// Turns `self` into a regular rust [`Result`]
    #[inline]
    #[track_caller]
    pub fn result(self) -> Result<T, VulkanResultCodes> {
        match self {
            VulkanResult::Success(_, t) => Ok(t),
            VulkanResult::Err(e) => Err(e),
        }
    }

    /// Unwraps `self`
    #[inline]
    #[track_caller]
    pub fn unwrap(self) -> T {
        match self {
            VulkanResult::Success(_, t) => t,
            VulkanResult::Err(e) => panic!("called `VulkanResult::unwrap()` on an `Err` value: {}", e),
        }
    }
}

impl<T> FromResidual<VulkanResult<Infallible>> for VulkanResult<T> {
    #[inline]
    #[track_caller]
    fn from_residual(residual: VulkanResult<Infallible>) -> Self {
        match residual {
            VulkanResult::Err(e) => VulkanResult::Err(e),
            VulkanResult::Success(_, _) => unreachable!(),
        }
    }
}

impl<T> FromResidual<Result<Infallible, VulkanResultCodes>> for VulkanResult<T> {
    #[inline]
    #[track_caller]
    fn from_residual(residual: Result<Infallible, VulkanResultCodes>) -> Self {
        match residual {
            Err(e) => Self::Err(From::from(e)),
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

impl<T> Try for VulkanResult<T> {
    type Output = (T, VulkanResultCodes);

    type Residual = Result<Infallible, VulkanResultCodes>;

    #[inline]
    fn from_output(output: Self::Output) -> Self {
        Self::Success(output.1, output.0)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            VulkanResult::Success(code, value) => ControlFlow::Continue((value, code)),
            VulkanResult::Err(err) => ControlFlow::Break(Err(err)),
        }
    }
}
