use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomInfo {
    pub password: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "cmd")]
pub enum APMessage {
    RoomInfo(RoomInfo),
    Connect(Connect),
    Connected(Connected)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connected {
    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "class")]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub build: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct Connect {
    pub name: String,
    pub password: String,
    pub game: String,
    pub uuid: String,
    pub version: Version,
    pub items_handling: u32,
    pub tags: Vec<String>,
    pub slot_data: bool,
}

impl Default for Connect {
    fn default() -> Self {
        Self {
            name: Default::default(),
            password: Default::default(),
            game: Default::default(),
            uuid: Default::default(),
            version: Version {
                major: 5,
                minor: 0,
                build: 0,
            },
            items_handling: 0,
            tags: ["Tracker".to_owned()].to_vec(),
            slot_data: false,
        }
    }
}
