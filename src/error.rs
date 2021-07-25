use sdl2::{render::UpdateTextureError, IntegerOrSdlError};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

pub type CtraitResult<T> = Result<T, CtraitError>;

#[derive(Debug)]
pub enum CtraitError {
    IntegerOr(IntegerOrSdlError),
    UpdateTexture(UpdateTextureError),
    Other(String),
}

impl Error for CtraitError {}

impl Display for CtraitError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "{}", self)
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

impl From<String> for CtraitError {
    fn from(err: String) -> Self {
        Self::Other(err)
    }
}
