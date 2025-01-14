use super::image::error;
use std::convert::From;
use std::{fmt, io};

#[derive(Debug)]
pub enum ImagixError {
    FileIOError(String),
    UserInputError(String),
    ImageResizingError(String),
    FormatError(String),
}

// impl fmt::Display for ImagixError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             ImagixError::FileIOError(msg) => write!(f, "File operator failed: {}", msg),
//             ImagixError::UserInputError(msg) => write!(f, "User input error: {}", msg),
//             ImagixError::ImageResizingError(msg) => write!(f, "Image resize error: {}", msg),
//             ImagixError::ImageResizingError(msg) => write!(f, "Format error: {}", msg),
//         }
//     }
// }

// impl std::error::Error for ImagixError {}

// /////


impl From<io::Error> for ImagixError {
    fn from(_error: io::Error) -> Self {
        ImagixError::FileIOError("Error in File I/O".to_string())
    }
}

impl From<image::ImageError> for ImagixError {
    fn from(_error: error::ImageError) -> Self {
        ImagixError::ImageResizingError("Error in image processing".to_string())
    }
}

impl From<io::ErrorKind> for ImagixError {
    fn from(_error: io::ErrorKind) -> Self {
        ImagixError::UserInputError("Error in user input".to_string())
    }
}

impl fmt::Display for ImagixError {
    fn fmt(&self, out: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            out,
            "{}",
            ImagixError::FormatError("Error occurred".to_string())
        )
    }
}
