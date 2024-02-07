-- modlog schema
CREATE TABLE IF NOT EXISTS mod_log (
    id BIGINT NOT NULL,
    guild_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    moderator_id BIGINT NOT NULL,
    action_type TEXT NOT NULL,
    action_duration INTEGER,
    reason TEXT NOT NULL DEFAULT "No reason provided",
    time_created TEXT NOT NULL,
    PRIMARY KEY (id)
    FOREIGN KEY (guild_id) REFERENCES guild_settings(guild_id) ON DELETE CASCADE
);