use crate::protocol::enums::ProtocolVersion;
use std::sync::OnceLock;

pub static USERNAME: OnceLock<String> = OnceLock::new();
pub const CURRENT_VERSION: ProtocolVersion = ProtocolVersion::V1;
pub const MAX_PACKET_SIZE: usize = 64 * 1024; // 64 KB 