use crate::dfu_status::{State, Status};
use std::fmt;
pub enum Error {
    DeviceNotFound(String),
    Argument(String),
    InvalidControlResponse(String),
    InvalidState(Status, State),
    InvalidStatus(Status, u8),
    USB(String, std::io::Error),
    USBNix(String, nix::Error),
    FileIO(std::io::Error),
    UnknownCommandByte(u8),
}

impl From<std::io::Error> for Error {
    fn from(io: std::io::Error) -> Self {
        crate::Error::FileIO(io)
    }
}

impl From<Error> for i32 {
    fn from(err: Error) -> Self {
        use Error::*;
        match err {
            DeviceNotFound(_) => 64,
            Argument(_) => 65,
            USB(_, _) => 66,
            USBNix(_, _) => 67,
            InvalidControlResponse(_) => 68,
            InvalidState(_, _) => 69,
            InvalidStatus(_, _) => 70,
            FileIO(_) => 71,
            UnknownCommandByte(_) => 72,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            DeviceNotFound(d) => write!(f, "Device not found: {}", d),
            Argument(d) => write!(f, "Argument {}", d),
            USB(e, io) => write!(f, "USB {} failed cause {}", e, io),
            USBNix(e, io) => write!(f, "USB {} failed cause {}", e, io),
            InvalidControlResponse(w) => write!(f, "Invalid control response on {}", w),
            InvalidState(s, expect) => write!(
                f,
                "Invalid state Get status gave:\n{}\nExpected state: {}",
                s, expect
            ),
            InvalidStatus(s, expect) => write!(
                f,
                "Invalid state Get status gave:\n{}\nExpected status: {}",
                s, expect
            ),
            FileIO(io) => write!(f, "IO error {}", io),
            UnknownCommandByte(b) => write!(f, "Unknown command byte: 0x{:X}", b),
        }
    }
}