-- user profile schema
CREATE TABLE IF NOT EXISTS user_profile (
    user_id BIGINT NOT NULL,
    guild_id BIGINT NOT NULL DEFAULT 0,
    first_joined_at TEXT NOT NULL,
    latest_joined_at TEXT NOT NULL,
    commands_ran INTEGER NOT NULL DEFAULT 0,
    infractions INTEGER NOT NULL DEFAULT 0,
  	PRIMARY KEY(user_id),
  	FOREIGN KEY (guild_id) REFERENCES guild_settings(guild_id) ON DELETE CASCADE
);