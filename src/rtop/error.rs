use std::error;
use std::fmt;
use std::io;
use std::sync::mpsc;

#[cfg(feature = "battery-monitor")]
use battery::errors::Error as batteryError;
#[cfg(feature = "gpu-monitor")]
use nvml_wrapper::error::NvmlError as nvmlError;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    ChannelRecvError(mpsc::RecvError),
    #[cfg(feature = "gpu-monitor")]
    NVMLError(nvmlError),
    #[cfg(feature = "battery-monitor")]
    BatteryMonitorError(batteryError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IoError(ref err) => write!(f, "Io error: {}", err),
            Error::ChannelRecvError(ref err) => write!(f, "Channel receive error: {}", err),
            #[cfg(feature = "gpu-monitor")]
            Error::NVMLError(ref err) => write!(f, "NVML error: {}", err),
            #[cfg(feature = "battery-monitor")]
            Error::BatteryMonitorError(ref err) => write!(f, "Battery Monitor error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::IoError(ref err) => Some(err),
            Error::ChannelRecvError(ref err) => Some(err),
            #[cfg(feature = "gpu-monitor")]
            Error::NVMLError(ref err) => Some(err),
            #[cfg(feature = "battery-monitor")]
            Error::BatteryMonitorError(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<mpsc::RecvError> for Error {
    fn from(err: mpsc::RecvError) -> Error {
        Error::ChannelRecvError(err)
    }
}

#[cfg(feature = "gpu-monitor")]
impl From<nvmlError> for Error {
    fn from(err: nvmlError) -> Error {
        Error::NVMLError(err)
    }
}

#[cfg(feature = "battery-monitor")]
impl From<batteryError> for Error {
    fn from(err: batteryError) -> Error {
        Error::BatteryMonitorError(err)
    }
}
