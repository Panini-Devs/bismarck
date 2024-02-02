#[derive(Clone)]
pub struct GuildSettings {
    pub prefix: String,
    pub owner_id: u64,
    pub mute_type: String,
    pub mute_role: u64,
    pub default_mute_duration: u64,
}

#[derive(Clone)]
pub struct GuildStats {
    pub commands_ran: u64,
    pub songs_played: u64,
}
