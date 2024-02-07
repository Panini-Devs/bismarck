-- guild settings schema
CREATE TABLE IF NOT EXISTS guild_settings (
    guild_id BIGINT NOT NULL UNIQUE,
    owner_id BIGINT NOT NULL,
    automod_enabled INTEGER NOT NULL DEFAULT 0,
    message_log_channel_id BIGINT,
    message_log_enabled INTEGER NOT NULL DEFAULT 0,
    mod_log_channel_id BIGINT,
    mod_log_enabled INTEGER NOT NULL DEFAULT 0,
    welcome_channel_id BIGINT,
    welcome_enabled INTEGER NOT NULL DEFAULT 0,
    welcome_message TEXT,
    prefix TEXT NOT NULL DEFAULT "-",
    mute_style TEXT NOT NULL DEFAULT "timeout",
    mute_duration INTEGER NOT NULL DEFAULT 3600,
    mute_role_id BIGINT,
    boosts INTEGER NOT NULL DEFAULT 0,
    boost_rewards_enabled INTEGER NOT NULL DEFAULT 0, -- role ids (to account for lists of roles) will be stored in another table
	PRIMARY KEY(guild_id)
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