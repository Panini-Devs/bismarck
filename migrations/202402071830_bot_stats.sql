-- bot stats schema
CREATE TABLE IF NOT EXISTS bot_stats (
    guild_id BIGINT NOT NULL DEFAULT 0,
    commands_ran BIGINT NOT NULL DEFAULT 0,
    songs_played BIGINT NOT NULL DEFAULT 0,
  	FOREIGN KEY (guild_id) REFERENCES guild_settings(guild_id) ON DELETE CASCADE
);