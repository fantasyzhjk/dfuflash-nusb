use crate::core::*;
use crate::error::Error;
use std::fmt;
use nusb;
use nusb::transfer::{ControlIn, ControlType, Recipient};

#[derive(Debug, Clone, PartialEq)]
pub enum State {
    AppIdle,
    AppDetach,
    DfuIdle,
    DfuDownloadSync,
    DfuDownloadBusy,
    DfuDownloadIdle,
    DfuManifestSync,
    DfuManifest,
    DfuManifestWaitReset,
    DfuUploadIdle,
    DfuError,
    Unknown,
}

impl From<&State> for u8 {
    fn from(state: &State) -> u8 {
        use crate::State::*;
        match state {
            AppIdle => 0,
            AppDetach => 1,
            DfuIdle => 2,
            DfuDownloadSync => 3,
            DfuDownloadBusy => 4,
            DfuDownloadIdle => 5,
            DfuManifestSync => 6,
            DfuManifest => 7,
            DfuManifestWaitReset => 8,
            DfuUploadIdle => 9,
            DfuError => 10,
            Unknown => 255,
        }
    }
}

impl From<u8> for State {
    fn from(state: u8) -> State {
        use crate::State::*;
        match state {
            0 => AppIdle,
            1 => AppDetach,
            2 => DfuIdle,
            3 => DfuDownloadSync,
            4 => DfuDownloadBusy,
            5 => DfuDownloadIdle,
            6 => DfuManifestSync,
            7 => DfuManifest,
            8 => DfuManifestWaitReset,
            9 => DfuUploadIdle,
            10 => DfuError,
            _ => Unknown,
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::State::*;
        match self {
            AppIdle => write!(f, "App Idle"),
            AppDetach => write!(f, "App detach"),
            DfuIdle => write!(f, "Dfu Idle"),
            DfuDownloadSync => write!(f, "Dfu download sync"),
            DfuDownloadBusy => write!(f, "Dfu download busy"),
            DfuDownloadIdle => write!(f, "Dfu download idle"),
            DfuManifestSync => write!(f, "Dfu manifest sync"),
            DfuManifest => write!(f, "Dfu manifest"),
            DfuManifestWaitReset => write!(f, "Dfu manifest wait reset"),
            DfuUploadIdle => write!(f, "Dfu Upload idle"),
            DfuError => write!(f, "Dfu error"),
            Unknown => write!(f, "Unknown state"),
        }
    }
}
#[derive(Debug, Default)]
pub struct Status {
    pub status: u8,
    pub poll_timeout: usize,
    pub state: u8,
    pub string_index: u8,
}
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = writeln!(f, "Status: {}", self.status).is_ok();
        let _ = writeln!(f, "poll_timeout: {}", self.poll_timeout).is_ok();
        let _ = writeln!(f, "State: {}", State::from(self.state)).is_ok();
        write!(f, "string_index: {}", self.string_index)
    }
}

impl Status {
    pub async fn get(interface: &nusb::Interface) -> Result<Self, Error> {
        let mut s = Self::default();
        let data: Vec<u8> = interface.control_in(ControlIn {
            control_type: ControlType::Class,
            recipient: Recipient::Interface,
            request: DFU_GET_STATUS,
            value: 0,
            index: 0,
            length: 6,
        }).await.into_result().map_err(|e| Error::USB("Control transfer: DFU_GET_STATUS".into(), e.into()))?;

        let mut data = data.iter();
        if data.len() != 6 {
            return Err(Error::InvalidControlResponse(format!(
                "Status length was {}",
                data.len()
            )));
        }
        s.status = *(data.next().unwrap_or(&0_u8));
        s.poll_timeout = ((*(data.next().unwrap_or(&0_u8)) as usize) << 16) as usize;
        s.poll_timeout |= ((*(data.next().unwrap_or(&0_u8)) as usize) << 8) as usize;
        s.poll_timeout |= (*(data.next().unwrap_or(&0_u8))) as usize;
        s.state = *(data.next().unwrap_or(&0_u8));
        s.string_index = *(data.next().unwrap_or(&0_u8));

        Ok(s)
    }
}
