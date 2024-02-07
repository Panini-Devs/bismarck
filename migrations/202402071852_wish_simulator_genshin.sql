-- Wish Simulator Schema
CREATE TABLE IF NOT EXISTS wish_simulator (
    user_id BIGINT NOT NULL,
    entertwined_wishes INTEGER NOT NULL DEFAULT 0,
    limited_wishes_placeholder INTEGER NOT NULL DEFAULT 0, -- I forgor the name
    primos INTEGER NOT NULL DEFAULT 3200,
    standard_wishes_count INTEGER NOT NULL DEFAULT 0,
    limited_wishes_count INTEGER NOT NULL DEFAULT 0,
    character_list TEXT NOT NULL DEFAULT '[]',
    character_count INTEGER NOT NULL DEFAULT 0,
    weapon_list TEXT NOT NULL DEFAULT '[]',
    weapon_count INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id)
);

-- music settings schema (TBD)
/*
CREATE TABLE IF NOT EXISTS music_settings (
    guild_id BIGINT NOT NULL,
    volume INTEGER NOT NULL DEFAULT 59, -- max volume is 100, also, add 10 to DEFAULT to get funny number
    loop_mode TEXT NOT NULL DEFAULT "off",
    autoplay INTEGER NOT NULL DEFAULT 0,
    DEFAULT_search TEXT NOT NULL DEFAULT "youtube",
    PRIMARY KEY (guild_id)
)


-- snipes schema
CREATE TABLE IF NOT EXISTS snipes (
    guild_id BIGINT NOT NULL,
    channel_id BIGINT NOT NULL,
    id INTEGER NOT NULL UNIQUE DEFAULT 0,
    message_content TEXT NOT NULL DEFAULT "No message found",
    message_attachment TEXT NOT NULL DEFAULT "No attachment found"
);

*/