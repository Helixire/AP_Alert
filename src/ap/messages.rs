use serde::{
    de::{Expected, Visitor},
    Deserialize, Serialize,
};

// Server Message

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "cmd")]
pub enum APServerMessage {
    RoomInfo(RoomInfo),
    ConnectionRefused(ConnectionRefused),
    Connected(Connected),
    ReceivedItems(()),
    LocationInfo(()),
    RoomUpdate(RoomUpdate),
    PrintJSON(PrintJSON),
    DataPackage(()),
    Bounced(()),
    InvalidPacket(()),
    Retrieved(()),
    SetReply(()),
}

#[derive(Debug, Clone, Deserialize)]
pub struct RoomInfo {
    pub password: bool,
    pub hint_cost: u32,
    pub location_check_points: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConnectionRefused {
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Connected {
    pub team: u32,
    pub slot: u32,
    pub players: Vec<NetworkPlayer>,
    pub missing_locations: Vec<u32>,
    pub checked_locations: Vec<u32>,
    pub hint_points: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NetworkPlayer {
    team: u32,
    slot: u32,
    alias: String,
    name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RoomUpdate {
    // See https://github.com/ArchipelagoMW/Archipelago/blob/main/docs/network%20protocol.md#roomupdate
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
#[serde()]
pub enum PrintJSON {
    Text {
        data: Vec<JSONMessagePart>,
    },
    ItemSend {
        data: Vec<JSONMessagePart>,
        receiving: u32,
        item: NetworkItem,
    },
    ItemCheat {
        data: Vec<JSONMessagePart>,
        receiving: u32,
        item: NetworkItem,
        team: u32,
    },
    Hint {
        data: Vec<JSONMessagePart>,
        receiving: u32,
        item: NetworkItem,
        found: bool,
    },
    Join {
        data: Vec<JSONMessagePart>,
        team: u32,
        slot: u32,
        tags: Vec<String>,
    },
    Part {
        data: Vec<JSONMessagePart>,
        team: u32,
        slot: u32,
    },
    Chat {
        data: Vec<JSONMessagePart>,
        team: u32,
        slot: u32,
        message: String,
    },
    ServerChat {
        data: Vec<JSONMessagePart>,
        message: String,
    },
    Tutorial {
        data: Vec<JSONMessagePart>,
    },
    TagsChanged {
        data: Vec<JSONMessagePart>,
        team: u32,
        slot: u32,
        tags: Vec<String>,
    },
    CommandResult {
        data: Vec<JSONMessagePart>,
    },
    AdminCommandResult {
        data: Vec<JSONMessagePart>,
    },
    Goal {
        data: Vec<JSONMessagePart>,
        team: u32,
        slot: u32,
    },
    Release {
        data: Vec<JSONMessagePart>,
        team: u32,
        slot: u32,
    },
    Collect {
        data: Vec<JSONMessagePart>,
        team: u32,
        slot: u32,
    },
    Countdown {
        data: Vec<JSONMessagePart>,
        countdown: u32,
    },
}

#[derive(Debug, Clone, Deserialize)]
pub struct JSONMessagePart {
    pub r#type: Option<String>,
    pub text: Option<String>,
    pub color: Option<String>, // only available if type is a color
    pub flags: Option<u32>,    // only available if type is an item_id or item_name
    pub player: Option<u32>,   // only available if type is either item or location
}

#[derive(Debug, Clone, Deserialize)]
pub struct NetworkItem {
    item: u32,
    location: u32,
    player: u32,
    flags: ItemType,
}

// See https://github.com/ArchipelagoMW/Archipelago/blob/main/docs/network%20protocol.md#networkitem
#[derive(Debug, Clone)]

pub enum ItemType {
    Normal,
    Logical,
    Important,
    Trap,
}

impl<'de> Deserialize<'de> for ItemType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_u64(ItemTypeVisitor)
    }
}

struct ItemTypeVisitor;

impl<'de> Visitor<'de> for ItemTypeVisitor {
    type Value = ItemType;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an integer between 0 and 4")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            0b000 => Ok(ItemType::Normal),
            0b001 => Ok(ItemType::Logical),
            0b010 => Ok(ItemType::Important),
            0b100 => Ok(ItemType::Trap),
            _ => Err(E::invalid_value(serde::de::Unexpected::Unsigned(v), &self)),
        }
    }
}

// Client Message

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "cmd")]
pub enum APClientMessage {
    Connect(Connect),
}

#[derive(Debug, Clone, Serialize)]

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

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "class")]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub build: u32,
}
