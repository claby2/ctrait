//! Internal errors.

use sdl2::{render::UpdateTextureError, video::WindowBuildError, IntegerOrSdlError};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// Type alias for a [`Result`] with [`CtraitError`] error type.
pub type CtraitResult<T> = Result<T, CtraitError>;

/// Enum representing potential error types.
#[allow(missing_docs, clippy::module_name_repetitions)]
#[derive(Debug)]
pub enum CtraitError {
    IntegerOr(IntegerOrSdlError),
    UpdateTexture(UpdateTextureError),
    WindowBuild(WindowBuildError),
    Other(String),
}

impl Error for CtraitError {}

impl Display for CtraitError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            CtraitError::IntegerOr(ref e) => e.fmt(f),
            CtraitError::UpdateTexture(ref e) => e.fmt(f),
            CtraitError::WindowBuild(ref e) => e.fmt(f),
            CtraitError::Other(ref e) => e.fmt(f),
        }
    }
}

impl From<IntegerOrSdlError> for CtraitError {
    fn from(err: IntegerOrSdlError) -> Self {
        Self::IntegerOr(err)
    }
}

impl From<UpdateTextureError> for CtraitError {
    fn from(err: UpdateTextureError) -> Self {
        Self::UpdateTexture(err)
    }
}

impl From<WindowBuildError> for CtraitError {
    fn from(err: WindowBuildError) -> Self {
        Self::WindowBuild(err)
    }
}

impl From<String> for CtraitError {
    fn from(err: String) -> Self {
        Self::Other(err)
    }
}

#[cfg(test)]
mod tests {
    use super::{CtraitError, IntegerOrSdlError, UpdateTextureError, WindowBuildError};

    macro_rules! assert_error_display {
        ($variant:ident, $error:expr) => {
            format!("{}", CtraitError::$variant($error))
        };
        ($variant:ident, $error:expr, $expected:expr) => {
            assert_eq!(assert_error_display!($variant, $error), $expected);
        };
    }

    #[test]
    fn error_display_integer_or() {
        assert_error_display!(IntegerOr, IntegerOrSdlError::IntegerOverflows("a", 1));
    }

    #[test]
    fn error_display_update_texture() {
        assert_error_display!(UpdateTexture, UpdateTextureError::PitchOverflows(1));
    }

    #[test]
    fn error_display_window_build() {
        assert_error_display!(WindowBuild, WindowBuildError::HeightOverflows(1));
    }

    #[test]
    fn error_display_other() {
        assert_error_display!(Other, String::from("error"), "error");
    }
}
