use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub enum ProtocolVersion {
    V1 = 1,
}
#[derive(Serialize, Deserialize)]
pub enum MessageType {
    Chat,
}