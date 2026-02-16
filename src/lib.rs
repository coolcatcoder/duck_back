#![feature(try_trait_v2)]
#![feature(try_as_dyn)]
use std::{
    any::{TypeId, try_as_dyn},
    convert::Infallible,
    fmt::{Debug, Display},
    ops::{ControlFlow, FromResidual, Try},
    panic::Location,
};

use bevy::log::{error, trace};

pub struct BevyResult<T, E, const ERROR: bool>(pub Result<T, E>);

impl<T, E, const ERROR: bool> Try for BevyResult<T, E, ERROR> {
    type Output = T;
    type Residual = BevyResult<Infallible, E, ERROR>;

    fn from_output(output: Self::Output) -> Self {
        BevyResult(Ok(output))
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
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

pub trait Else {
    type Output<const ERROR: bool>;
    fn else_error(self) -> Self::Output<true>;
    fn else_return(self) -> Self::Output<false>;
}

impl<T> Else for Option<T> {
    type Output<const ERROR: bool> = BevyResult<T, (), ERROR>;

    fn else_error(self) -> Self::Output<true> {
        BevyResult(self.ok_or(()))
    }
    fn else_return(self) -> Self::Output<false> {
        BevyResult(self.ok_or(()))
    }
}

impl<T, E> Else for Result<T, E> {
    type Output<const ERROR: bool> = BevyResult<T, E, ERROR>;

    fn else_error(self) -> Self::Output<true> {
        BevyResult(self)
    }
    fn else_return(self) -> Self::Output<false> {
        BevyResult(self)
    }
}
