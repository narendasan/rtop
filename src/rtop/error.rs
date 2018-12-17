use std::io;
use std::error;
use std::fmt;

#[cfg(feature = "gpu-monitor")] use nvml_wrapper::error::Error as nvmlError; 

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    #[cfg(feature = "gpu-monitor")]
    NVML(nvmlError),
} 

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => write!(f, "Io error: {}", err),
            #[cfg(feature = "gpu-monitor")]
            Error::NVML(ref err) => write!(f, "NVML error: {}", err),   
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(error::Error + 'static)> {
        match *self {
            Error::Io(ref err) => Some(err),
            #[cfg(feature = "gpu-monitor")]
            Error::NVML(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

#[cfg(feature = "gpu-monitor")]
impl From<nvmlError> for Error {
    fn from(err: nvmlError) -> Error {
        Error::NVML(err)
    }
}