use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct SantaConfig {
    pub pool_max_lifetime : u64,
    pub pool_lifetime_check_freq : u64,
    pub message_lifetime : u64,
    pub max_messages_in_room_count : u64,
    pub old_messages_check_freq : u64
}