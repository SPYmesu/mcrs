use core::error::Error;
use core::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum GuiAtlasError {
    Textures(String),
}

impl fmt::Display for GuiAtlasError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Textures(reason) => write!(formatter, "the GUI textures: {reason}"),
        }
    }
}

impl Error for GuiAtlasError {}
