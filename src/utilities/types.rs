use serde::Deserialize;

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

#[derive(Clone)]
pub struct User {
    pub id: u64,
    pub acquaint_fate: u64,
    pub intertwined_fate: u64,
    pub primogems: u64,
    pub standard_pity: u64,
    pub weapon_pity: u64,
    pub character_pity: u64,
}

#[derive(Deserialize, Clone)]
pub struct Item {
    pub image_url: String,
    pub id: u32,
}

#[derive(Deserialize)]
pub struct Items {
    pub items: Vec<Item>,
}

#[derive(Deserialize)]
pub struct WikiQuery(
    pub String,
    pub Vec<String>,
    pub Vec<String>,
    pub Vec<String>,
);
