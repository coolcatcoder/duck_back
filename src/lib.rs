#![doc = include_str!("../README.md")]
#![feature(try_trait_v2)]
#![feature(try_as_dyn)]
#![warn(clippy::pedantic)]
#![warn(missing_docs)]
#![no_std]

extern crate alloc;
use alloc::{format, string::String};

use core::{
    any::{TypeId, try_as_dyn},
    convert::Infallible,
    fmt::{Debug, Display},
    ops::{ControlFlow, FromResidual, Try},
    panic::Location,
};

use tracing::{error, trace};

/// A wrapper around [`Result<T, E>`].\
/// The [`?`](core::ops::Try) operator can be used to reduce this into `()`.\
/// ERROR controls whether it raises an error when it gets reduced.
#[must_use]
pub struct BevyResult<T, E, const ERROR: bool>(pub Result<T, E>);

impl<T, E, const ERROR: bool> Try for BevyResult<T, E, ERROR> {
    type Output = T;
    type Residual = BevyResult<Infallible, E, ERROR>;

    fn from_output(output: Self::Output) -> Self {
        BevyResult(Ok(output))
    }

    fn branch(self) -> core::ops::ControlFlow<Self::Residual, Self::Output> {
        match self.0 {
            Ok(output) => ControlFlow::Continue(output),
            Err(error) => ControlFlow::Break(BevyResult(Err(error))),
        }
    }
}

impl<T, E, const ERROR: bool> FromResidual for BevyResult<T, E, ERROR> {
    fn from_residual(error: <Self as Try>::Residual) -> Self {
        let Err(error) = error.0;
        BevyResult(Err(error))
    }
}

// We might be able to remove the static bound if try_as_dyn ever stops requiring it.
impl<E: 'static, const ERROR: bool> FromResidual<BevyResult<Infallible, E, ERROR>> for () {
    #[track_caller]
    fn from_residual(result: BevyResult<Infallible, E, ERROR>) {
        let Err(error) = result.0;

        let end = if TypeId::of::<E>() == TypeId::of::<()>() {
            String::new()
        } else if let Some(error) = try_as_dyn::<_, dyn Display>(&error) {
            format!("\n{error}")
        } else if let Some(error) = try_as_dyn::<_, dyn Debug>(&error) {
            format!("\n{error:#?}")
        } else {
            String::new()
        };

        let message = format!("({})\nFailed to unwrap value.{end}", Location::caller());

        if ERROR {
            error!("{message}");
        } else {
            trace!("{message}");
        }
    }
}

/// An extension trait that allows converting options and results into [`BevyResult`].\
/// (It is actually implemented for any type that implements [`UnwrappedResidual`].)
pub trait Else {
    /// The [`BevyResult`] to be returned.
    type Output<const ERROR: bool>;
    /// Will convert self to a [`BevyResult`] that will raise an error.
    fn else_error(self) -> Self::Output<true>;
    /// Will convert self to a [`BevyResult`] that will not raise an error, but will still appear at the trace logging level.
    fn else_return(self) -> Self::Output<false>;
}

/// An unfortunate trait.\
/// The residual of `Result<T, E>` is `Result<!, E>`, and not `E`.\
/// This trait removes the type wrapping from the residual.
pub trait UnwrappedResidual: Try {
    /// The residual of the type without wrapping.
    type UnwrappedResidual;

    /// Go from the wrapped residual to the unwrapped residual.
    fn unwrap_residual(residual: Self::Residual) -> Self::UnwrappedResidual;
}

impl<T: UnwrappedResidual> Else for T {
    type Output<const ERROR: bool> =
        BevyResult<<Self as Try>::Output, <Self as UnwrappedResidual>::UnwrappedResidual, ERROR>;

    fn else_error(self) -> Self::Output<true> {
        BevyResult(self.branch().continue_ok().map_err(Self::unwrap_residual))
    }
    fn else_return(self) -> Self::Output<false> {
        BevyResult(self.branch().continue_ok().map_err(Self::unwrap_residual))
    }
}

impl<T> UnwrappedResidual for Option<T> {
    type UnwrappedResidual = ();

    fn unwrap_residual(residual: Self::Residual) -> Self::UnwrappedResidual {
        let None = residual;
    }
}

impl<T, E> UnwrappedResidual for Result<T, E> {
    type UnwrappedResidual = E;

    fn unwrap_residual(residual: Self::Residual) -> Self::UnwrappedResidual {
        let Err(error) = residual;
        error
    }
}
